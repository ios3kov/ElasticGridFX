"""Bounded macOS owned-AE memory observer. No UI input or render request.

Observe by default. Explicit --terminate-on-limit is for a saved disposable
fixture only, under the user's AE-close authority. A PID/path/birth mismatch
never signals any process. Does not inspect or stop other applications.
When armed, the saved disposable AE process is also closed at the deadline.
"""
from __future__ import annotations
import argparse
import ctypes
import json
import os
from pathlib import Path
import signal
import subprocess
import time

HOST = '/Applications/Adobe After Effects 2025/Adobe After Effects 2025.app/Contents/MacOS/After Effects'
MIB = 1024 * 1024
MAX_BASELINE = 1024 * MIB
MAX_FOOTPRINT = 1536 * MIB
MAX_GROWTH = 256 * MIB

class Usage(ctypes.Structure):
    # Exact RUSAGE_INFO_V2 from target macOS sys/resource.h (160 bytes).
    _fields_ = [('uuid', ctypes.c_uint8 * 16)] + [
        (name, ctypes.c_uint64) for name in (
            'user_time', 'system_time', 'pkg_idle_wkups', 'interrupt_wkups',
            'pageins', 'wired_size', 'resident_size', 'phys_footprint',
            'proc_start_abstime', 'proc_exit_abstime', 'child_user_time',
            'child_system_time', 'child_pkg_idle_wkups', 'child_interrupt_wkups',
            'child_pageins', 'child_elapsed_abstime', 'diskio_bytesread',
            'diskio_byteswritten')]

class Process:
    def __init__(self, pid: int):
        self.pid = pid
        self.lib = ctypes.CDLL('/usr/lib/libproc.dylib', use_errno=True)
        self.lib.proc_pid_rusage.argtypes = [ctypes.c_int, ctypes.c_int, ctypes.POINTER(Usage)]
        self.lib.proc_pid_rusage.restype = ctypes.c_int
        self.lib.proc_pidpath.argtypes = [ctypes.c_int, ctypes.c_void_p, ctypes.c_uint32]
        self.lib.proc_pidpath.restype = ctypes.c_int
    def read(self):
        path = ctypes.create_string_buffer(4096)
        if self.lib.proc_pidpath(self.pid, path, len(path)) <= 0:
            raise ProcessLookupError(self.pid)
        usage = Usage()
        if self.lib.proc_pid_rusage(self.pid, 2, ctypes.byref(usage)) != 0:
            raise OSError(ctypes.get_errno(), 'proc_pid_rusage')
        return path.value.decode(), usage.proc_start_abstime, usage.phys_footprint

def limit_reason(baseline: int, footprint: int):
    if footprint > MAX_FOOTPRINT:
        return 'absolute-footprint'
    if footprint - baseline > MAX_GROWTH:
        return 'growth-from-baseline'
    return None

def identity_matches(path, birth, expected_birth):
    return path == HOST and birth == expected_birth and birth > 0

def signal_owned(process, expected_birth, signum, send=os.kill):
    path, birth, _ = process.read()
    if not identity_matches(path, birth, expected_birth):
        raise RuntimeError('PID/path/birth changed; no signal sent')
    send(process.pid, signum)

def stop_on_limit(process, birth, destination, record, sample=subprocess.run, send=os.kill):
    # A failed sample/log write must not leave an owned AE suspended.
    signal_owned(process, birth, signal.SIGSTOP, send)
    try:
        record({'event': 'STOPPED_ON_LIMIT'})
        result = sample(['/usr/bin/sample', str(process.pid), '1', '10', '-file',
            str(destination / 'limit-sample.txt')], capture_output=True, text=True, timeout=4)
        record({'event': 'STOPPED_SAMPLE', 'exit': result.returncode})
    finally:
        signal_owned(process, birth, signal.SIGKILL, send)
        record({'event': 'OWNED_AE_KILL_SENT'})

def monitor(pid: int, destination: Path, seconds: float, terminate: bool):
    assert ctypes.sizeof(Usage) == 160
    if not 1 <= seconds <= 45:
        raise ValueError('Observation limited to 1–45 seconds')
    destination.mkdir(mode=0o700)  # fresh; never overwrite previous evidence
    process = Process(pid)
    path, birth, baseline = process.read()
    if not identity_matches(path, birth, birth) or baseline > MAX_BASELINE:
        raise RuntimeError('AE identity or <=1GiB idle baseline gate failed; no gesture')
    fd = os.open(destination / 'memory.jsonl', os.O_CREAT | os.O_EXCL | os.O_WRONLY, 0o600)
    with os.fdopen(fd, 'w') as log:
        def record(data):
            data['unix_ms'] = time.time_ns() // 1_000_000
            log.write(json.dumps(data) + '\n'); log.flush()
        record({'event': 'READY', 'pid': pid, 'birth': birth, 'baseline': baseline,
                'max_footprint': MAX_FOOTPRINT, 'max_growth': MAX_GROWTH,
                'terminate_on_limit_or_deadline': terminate, 'deadline_seconds': seconds})
        print('READY', flush=True)
        start = time.monotonic()
        while time.monotonic() - start < seconds:
            try:
                path, observed_birth, footprint = process.read()
            except (ProcessLookupError, OSError) as error:
                record({'event': 'UNAVAILABLE', 'type': type(error).__name__}); return
            if not identity_matches(path, observed_birth, birth):
                record({'event': 'IDENTITY_CHANGED', 'signal': 'NONE'}); return
            reason = limit_reason(baseline, footprint)
            record({'seconds': time.monotonic() - start, 'footprint': footprint, 'limit': reason})
            if reason:
                if not terminate:
                    record({'event': 'LIMIT', 'signal': 'NONE'}); return
                # Stop allocation before a one-second diagnostic sample. Recheck
                # identity before each signal; never touch an unrelated PID.
                stop_on_limit(process, birth, destination, record)
                return
            time.sleep(0.2)
        if terminate:
            signal_owned(process, birth, signal.SIGKILL)
            record({'event': 'DEADLINE_OWNED_AE_KILL_SENT'})
        else:
            record({'event': 'TIME_LIMIT', 'signal': 'NONE'})

if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--pid', required=True, type=int)
    parser.add_argument('--out', required=True, type=Path)
    parser.add_argument('--seconds', type=float, default=30)
    parser.add_argument('--terminate-on-limit', action='store_true',
                        help='Close saved disposable AE on a limit or observation deadline')
    args = parser.parse_args()
    monitor(args.pid, args.out, args.seconds, args.terminate_on_limit)
