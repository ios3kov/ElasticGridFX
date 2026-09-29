#!/usr/bin/env python3
"""Read-only target-Mac identity diagnostic. No installation, JSX or project edits."""
from __future__ import annotations
import argparse
import ctypes
import os
import json
from pathlib import Path
import platform
import plistlib
import re
import stat
import struct
import subprocess
import sys
import time
import uuid
import zipfile

import build_identity as bi
from install_candidate import checked_path, discover, signature

ROOT = Path(__file__).resolve().parents[1]
BASELINE = ROOT / 'diagnostics' / 'candidate-6d3b846.json'
LIMIT = 16 * 1024 * 1024


class Blocked(ValueError):
    """Missing trustworthy observation, not a successful runtime test."""


def macho_uuids(data: bytes) -> set[str]:
    """Read LC_UUID from bounded 64-bit Mach-O slices; accept FAT32/FAT64 too."""
    if len(data) < 4 or len(data) > LIMIT:
        raise Blocked('Invalid Mach-O size')
    magic = data[:4]
    if magic in (b'\xca\xfe\xba\xbe', b'\xbe\xba\xfe\xca', b'\xca\xfe\xba\xbf', b'\xbf\xba\xfe\xca'):
        endian = '>' if magic[0] == 0xca else '<'
        fat64 = magic in (b'\xca\xfe\xba\xbf', b'\xbf\xba\xfe\xca')
        if len(data) < 8:
            raise Blocked('Truncated FAT header')
        count = struct.unpack_from(endian+'I', data, 4)[0]
        entry_size = 32 if fat64 else 20
        table_end = 8 + count * entry_size
        if not 1 <= count <= 16 or table_end > len(data):
            raise Blocked('Invalid FAT architecture table')
        result: set[str] = set()
        ranges = []
        for index in range(count):
            offset = 8 + index * entry_size
            fmt = endian + ('IIQQII' if fat64 else 'IIIII')
            record = struct.unpack_from(fmt, data, offset)
            begin, size = record[2:4]
            end = begin + size
            if begin < table_end or size < 32 or end > len(data) or any(begin < b and end > a for a, b in ranges):
                raise Blocked('Invalid/overlapping FAT slice')
            ranges.append((begin, end))
            child = data[begin:end]
            if child[:4] not in (b'\xcf\xfa\xed\xfe', b'\xfe\xed\xfa\xcf'):
                raise Blocked('Unsupported FAT slice')
            found = macho_uuids(child)
            if result & found:
                raise Blocked('Duplicate Mach-O UUID')
            result |= found
        return result
    if magic not in (b'\xcf\xfa\xed\xfe', b'\xfe\xed\xfa\xcf') or len(data) < 32:
        raise Blocked('Require 64-bit Mach-O')
    endian = '<' if magic[0] == 0xcf else '>'
    header = struct.unpack_from(endian+'8I', data)
    count, size = header[4:6]
    if not 1 <= count <= 4096 or 32+size > len(data):
        raise Blocked('Invalid Mach-O commands')
    cursor = 32
    found = []
    for _ in range(count):
        if cursor+8 > 32+size:
            raise Blocked('Truncated load command')
        kind, length = struct.unpack_from(endian+'II', data, cursor)
        if length < 8 or length % 8 or cursor+length > 32+size:
            raise Blocked('Invalid load command length')
        if kind == 0x1b:  # LC_UUID in Apple's mach-o/loader.h
            if length != 24:
                raise Blocked('Invalid LC_UUID')
            value = uuid.UUID(bytes=data[cursor+8:cursor+24])
            if value.int == 0:
                raise Blocked('Zero LC_UUID')
            found.append(str(value).upper())
        cursor += length
    if cursor != 32+size or len(found) != 1:
        raise Blocked('Missing/ambiguous LC_UUID')
    return set(found)


def system_path(pid: int, address: int | None = None) -> str:
    """Read path via Apple's libproc; permission denial blocks, never escalates."""
    if not 0 < pid < 2**31 or (address is not None and not 0 <= address < 2**64):
        raise Blocked('Invalid native process/address')
    # Exported signatures from Apple's xnu/libsyscall/wrappers/libproc/libproc.h.
    # These private interfaces may change; the native fixture gates support.
    try:
        library = ctypes.CDLL('/usr/lib/libproc.dylib', use_errno=True)
        function = getattr(library, 'proc_pidpath' if address is None else 'proc_regionfilename')
    except (OSError, AttributeError) as error:
        raise Blocked('Native path interface unavailable; no permissions changed') from error
    buffer = ctypes.create_string_buffer(4096)
    if address is None:
        function.argtypes = [ctypes.c_int, ctypes.c_void_p, ctypes.c_uint32]
        function.restype = ctypes.c_int
        size = function(pid, buffer, len(buffer))
    else:
        function.argtypes = [ctypes.c_int, ctypes.c_uint64, ctypes.c_void_p, ctypes.c_uint32]
        function.restype = ctypes.c_int
        size = function(pid, address, buffer, len(buffer))
    if not 0 < size < len(buffer):
        raise Blocked('Native process/image path unavailable; no permissions changed')
    value = os.fsdecode(buffer.raw[:size].rstrip(b'\0'))
    if not value.startswith('/') or '*' in value or any(ord(c) < 32 for c in value):
        raise Blocked('Invalid native path observation')
    return value


def path_alias(value: str) -> str:
    """Normalize only documented/observed macOS filesystem aliases, not arbitrary symlinks."""
    if value == '/System/Volumes/Data':
        value = '/'
    elif value.startswith('/System/Volumes/Data/'):
        value = value[len('/System/Volumes/Data'):]
    # Apple's tools may spell these VFS aliases either with or without /private.
    for short in ('/var/', '/tmp/'):
        if value.startswith(short):
            value = '/private' + value
    return value


def path_text_consistent(reported: str, observed: str) -> bool:
    """Text-path check only; live identity still requires UUID and on-disk payload checks."""
    reported, observed = path_alias(reported), path_alias(observed)
    if '*' not in reported:
        return reported == observed
    if reported.count('*') != 1 or '*' in observed:
        return False
    prefix, suffix = reported.split('*')
    return (prefix.startswith('/') and prefix.endswith('/') and suffix.startswith('/')
            and observed.startswith(prefix) and observed.endswith(suffix)
            and len(observed) >= len(prefix) + len(suffix))


def masked_path_consistent(reported: str, observed: str) -> bool:
    """Backward-compatible name for the privacy-mask constraint."""
    return '*' in reported and path_text_consistent(reported, observed)


def same_underlying_path(a: Path, b: Path) -> bool:
    """Compare inode/device after refusing ordinary symlinked paths."""
    try:
        a = checked_path(a)
        b = checked_path(b)
        sa, sb = a.stat(), b.stat()
        return (sa.st_dev, sa.st_ino) == (sb.st_dev, sb.st_ino)
    except (OSError, ValueError):
        return False


def redundant_symlink_root(root: Path, roots: list[Path]) -> Path | None:
    """Accept a symlink scan root only when it aliases another independently listed root."""
    if not root.is_symlink():
        return None
    try:
        target = root.resolve(strict=True)
        if not target.is_dir():
            raise ValueError('symlinked scan root target is not a directory')
        target_stat = target.stat()
        for peer in roots:
            if peer == root or peer.is_symlink():
                continue
            try:
                peer_stat = peer.stat()
            except FileNotFoundError:
                continue
            if (peer_stat.st_dev, peer_stat.st_ino) == (target_stat.st_dev, target_stat.st_ino):
                return peer
    except (OSError, RuntimeError) as error:
        raise ValueError('unresolvable symlinked scan root') from error
    raise ValueError('symlinked scan root is not a redundant known Adobe root')


def parse_sample(text: str, pid: int, executable: str, *, path_lookup=None) -> list[dict]:
    """Use live sample header/image table, never scrape arbitrary stack text."""
    if len(text) > LIMIT or text.count('Binary Images:') != 1:
        raise Blocked('Missing/ambiguous live image table')
    header, table = text.split('Binary Images:', 1)
    pids = re.findall(r'^Process:\s+.*\[(\d+)\]\s*$', header, re.M)
    paths = re.findall(r'^Path:\s+(.+?)\s*$', header, re.M)
    if pids != [str(pid)] or len(paths) != 1:
        raise Blocked('Sample belongs to a different process')
    header_path = paths[0]
    if '*' in header_path:
        if path_lookup is None:
            raise Blocked('Masked process path requires independent native observation')
        native_process = path_lookup(None)
        if (not masked_path_consistent(header_path, native_process)
                or not path_text_consistent(native_process, executable)):
            raise Blocked('Sample belongs to a different process')
    elif not path_text_consistent(header_path, executable):
        if path_lookup is None or not path_text_consistent(path_lookup(None), executable):
            raise Blocked('Sample belongs to a different process')
    result = []
    for line in table.splitlines():
        match = re.fullmatch(r'\s*(0x[\da-fA-F]+)\s*-\s*(0x[\da-fA-F]+)\s+(.+?)\s+<([\da-fA-F-]+)>\s+(/[^\r\n]+)\s*', line)
        if not match:
            if 'elasticgrid' in line.lower() or 'com.elasticgrid.fx' in line.lower():
                raise Blocked('Unrecognized candidate image record')
            continue
        low, high, name, value, path = match.groups()
        try:
            image_id = str(uuid.UUID(value)).upper()
        except ValueError as error:
            raise Blocked('Malformed image UUID') from error
        if int(high, 16) <= int(low, 16):
            raise Blocked('Invalid image address range')
        reported_path = path.rstrip()
        observed_path = reported_path
        source = 'sample'
        if '*' in reported_path and ('elasticgrid' in reported_path.lower() or 'com.elasticgrid.fx' in name.lower()):
            if path_lookup is not None:
                observed_path = path_lookup(int(low, 16))
                if not masked_path_consistent(reported_path, observed_path):
                    raise Blocked('Native path contradicts reported image')
                source = 'libproc region at sampled load address'
        result.append(dict(uuid=image_id, path=observed_path, label=name,
                           reported_path=reported_path, path_source=source))
    if not result:
        raise Blocked('Empty live image table')
    return result


def process_key(pid: int) -> str:
    if pid <= 0:
        raise Blocked('Invalid PID')
    value = subprocess.check_output(['/bin/ps', '-p', str(pid), '-o', 'lstart=,comm='], text=True, timeout=5).strip()
    if not value or '\n' in value:
        raise Blocked('Missing/ambiguous process')
    return value


def capture(pid: int, executable: Path, folder: Path) -> tuple[list[dict], dict]:
    """Bounded Apple sampler; no sudo, debug injection, host kill or retry."""
    before = process_key(pid)
    if not before.endswith(' '+str(executable)):
        raise Blocked('Process executable changed')
    output = folder / 'sample-private.txt'
    if output.exists() or output.is_symlink():
        raise Blocked('Refusing stale sample path')
    started = time.time_ns()
    result = subprocess.run(['/usr/bin/sample', str(pid), '1', '10', '-fullPaths', '-file', str(output)],
                            stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=15)
    # Never include all stdout/stacks in the shareable report.
    if result.returncode:
        raise Blocked('Apple sample refused or failed; no permissions changed')
    if process_key(pid) != before:
        raise Blocked('Target process changed during capture')
    checked_path(output)
    info = output.stat()
    if not stat.S_ISREG(info.st_mode) or info.st_size > LIMIT or info.st_size == 0 or info.st_mtime_ns < started - 2_000_000_000:
        raise Blocked('Invalid/stale sample file')
    text = output.read_text(encoding='utf-8-sig')
    images = parse_sample(text, pid, str(executable), path_lookup=lambda address: system_path(pid, address))
    observation = dict(pid=pid, process_key_sha256=bi.digest(before.encode()),
                       private_sample_sha256=bi.digest(output.read_bytes()),
                       method='Apple sample image UUID + libproc for redacted paths + Mach-O LC_UUID',
                       duration_seconds=1, interval_milliseconds=10)
    return images, observation


def select_image(images: list[dict], binary: Path, expected_uuids: set[str]) -> dict:
    relevant = [image for image in images if path_text_consistent(image['path'], str(binary)) or
                'elasticgrid' in image['path'].lower() or 'com.elasticgrid.fx' in image['label'].lower()]
    if len(relevant) != 1:
        raise Blocked('Candidate image missing or multiple copies loaded')
    image = relevant[0]
    if '*' in image['path']:
        raise Blocked('Masked image path requires independent native observation')
    if (not path_text_consistent(image['path'], str(binary))
            and not same_underlying_path(Path(image['path']), binary)):
        raise Blocked('Loaded image path differs from the candidate on disk')
    if image['uuid'] not in expected_uuids:
        raise Blocked('Loaded image UUID differs from the candidate on disk')
    return image


def verify_disk(bundle: Path, manifest: dict) -> dict:
    checked_path(bundle, directory=True)
    if bi.payload_files(bundle) != manifest['files']:
        raise Blocked('Installed payload does not match pinned candidate')
    meta = bi.payload_metadata(bundle)
    if meta != manifest['build']:
        raise Blocked('Installed metadata does not match pinned candidate')
    return meta


def installed_roots(app: Path | None, additional: list[Path]) -> list[Path]:
    roots = [Path('/Library/Application Support/Adobe/Common/Plug-ins/7.0/MediaCore'),
             Path.home()/'Library/Application Support/Adobe/Common/Plug-ins/7.0/MediaCore']
    if app is not None:
        roots += [app/'Contents/Plug-ins', app/'Plug-ins', app.parent/'Plug-ins']
    return list(dict.fromkeys(roots+additional))


def running_ae() -> list[dict]:
    text = subprocess.check_output(['/bin/ps', '-axo', 'pid=,comm='], text=True, timeout=5)
    result = []
    for line in text.splitlines():
        match = re.fullmatch(r'\s*(\d+)\s+(.+\.app)/Contents/MacOS/([^/]+)', line)
        if match and match.group(3).lower() in ('after effects', 'adobe after effects'):
            result.append(dict(pid=int(match.group(1)), app=match.group(2), executable=match.group(2)+'/Contents/MacOS/'+match.group(3)))
    return result


def disk_summary(bundle: Path) -> dict:
    """Legacy copies may lack metadata. Report honestly without executing code."""
    result = {'path': str(bundle), 'build_id': None}
    plist = bundle/'Contents/Info.plist'
    if plist.is_file():
        checked_path(plist)
        data = plistlib.loads(plist.read_bytes())
        result['version'] = data.get('CFBundleShortVersionString')
        result['identifier'] = data.get('CFBundleIdentifier')
    binary = bundle/'Contents/MacOS/ElasticGrid'
    if binary.is_file():
        checked_path(binary)
        if binary.stat().st_size > LIMIT:
            raise Blocked('Oversized candidate binary')
        data = binary.read_bytes()
        result['binary_sha256'] = bi.digest(data)
        result['macho_uuids'] = sorted(macho_uuids(data))
    identity = bundle/'Contents/Resources/BuildIdentity.json'
    if identity.is_file():
        checked_path(identity)
        if identity.stat().st_size > 16384:
            raise Blocked('Oversized identity metadata')
        meta = bi.validate_identity(json.loads(identity.read_text()))
        result['build_id'] = meta['build_id']  # This field is DISK metadata only.
    return result


def diagnose(folder: Path, manifest: dict, additional: list[Path]) -> dict:
    result = dict(schema=1, run_id=folder.name, status='BLOCKED',
                  expected_build=manifest['build'], installed=[], scan_errors=[],
                  loaded_image_status='NOT RUN', direct_runtime_build_id=None,
                  observed_build_id=None, functional_test='NOT RUN', release='BLOCKED',
                  environment=dict(os=platform.system(), os_version=platform.mac_ver()[0], architecture=platform.machine()))
    if platform.system() != 'Darwin':
        result['reason'] = 'Requires target macOS; nothing installed or executed in AE'
        return result
    try:
        hosts = running_ae()
        if len(hosts) != 1:
            raise Blocked('Open exactly one idle After Effects instance; diagnostic will not launch/close it')
        host = hosts[0]
        app = checked_path(Path(host['app']), directory=True)
        plist = bi.safe_file(app, 'Contents/Info.plist')
        app_meta = plistlib.loads(plist.read_bytes())
        if 'aftereffects' not in str(app_meta.get('CFBundleIdentifier', '')).lower():
            raise Blocked('Running app identity is not After Effects')
        result['ae'] = dict(pid=host['pid'], path=str(app), version=app_meta.get('CFBundleShortVersionString'))
        roots = installed_roots(app, additional)
        result['scan_roots'] = list(map(str, roots))
        result['scan_aliases'] = []
        found: set[Path] = set()
        for root in roots:
            try:
                alias = redundant_symlink_root(root, roots)
                if alias is not None:
                    result['scan_aliases'].append({'root': str(root), 'same_as': str(alias)})
                    continue
                found.update(discover([root]))
            except (OSError, ValueError) as error:
                result['scan_errors'].append({'root': str(root), 'kind': type(error).__name__})
        for candidate in sorted(found):
            try:
                result['installed'].append(disk_summary(candidate))
            except (OSError, ValueError, KeyError):
                result['installed'].append({'path':str(candidate), 'error':'Unreadable/unsupported identity'})
        matching_before = set()
        for candidate in found:
            try:
                verify_disk(candidate, manifest)
                signature(candidate)
                matching_before.add(candidate)
            except (OSError, ValueError, KeyError, subprocess.SubprocessError):
                pass  # Still capture old/mismatched copies for a useful diagnosis.
        # Capture even for old/no-metadata copies: that is useful target evidence.
        images, observation = capture(host['pid'], Path(host['executable']), folder)
        result['observation'] = observation
        result['loaded_images'] = [image for image in images if 'elasticgrid' in image['path'].lower() or 'com.elasticgrid.fx' in image['label'].lower()]
        if len(result['loaded_images']) != 1:
            raise Blocked('Candidate image absent or multiple copies loaded')
        observed_binary = checked_path(Path(result['loaded_images'][0]['path']))
        if observed_binary.name != 'ElasticGrid' or observed_binary.parent.name != 'MacOS' or observed_binary.parent.parent.name != 'Contents':
            raise Blocked('Unexpected loaded plugin layout')
        matches = [candidate for candidate in matching_before
                   if same_underlying_path(candidate/'Contents/MacOS/ElasticGrid', observed_binary)
                   or path_text_consistent(str(candidate/'Contents/MacOS/ElasticGrid'), str(observed_binary))]
        if len(matches) != 1:
            raise Blocked('Loaded image does not map uniquely to the pinned installed candidate')
        bundle = matches[0]
        binary = checked_path(bundle/'Contents/MacOS/ElasticGrid')
        meta = verify_disk(bundle, manifest)
        signature(bundle)
        observed = select_image(images, binary, macho_uuids(binary.read_bytes()))
        # Do not promote an unknown scan scope or a distinct second copy to PASS.
        distinct = [candidate for candidate in found
                    if not same_underlying_path(candidate/'Contents/MacOS/ElasticGrid', binary)
                    and not path_text_consistent(str(candidate/'Contents/MacOS/ElasticGrid'), str(binary))]
        if result['scan_errors'] or distinct:
            raise Blocked('Incomplete scan or conflicting installed copies')
        if bi.digest(process_key(host['pid']).encode()) != observation['process_key_sha256']:
            raise Blocked('AE restarted after sampling')
        verify_disk(bundle, manifest)
        result.update(status='PASS', loaded_image_status='PASS', observed_image_uuid=observed['uuid'],
                      observed_build_id=meta['build_id'],
                      reason='Pinned payload matches the live image UUID/path; functional tests are still NOT RUN')
    except (OSError, ValueError, KeyError, subprocess.SubprocessError) as error:
        result['reason'] = str(error) if isinstance(error, Blocked) else type(error).__name__ + ': diagnostic unavailable; no automatic retry'
        if isinstance(error, subprocess.TimeoutExpired):
            result['reason'] = 'Sampler timeout; no AE process was killed or restarted'
    return result


def public_record(result: dict) -> dict:
    # No arbitrary process stack, app project data or username in uploaded report.
    home = str(Path.home())
    encoded = json.dumps(result, ensure_ascii=False)
    encoded = encoded.replace(home, '~')
    encoded = re.sub(r'/Users/[^/"\\\s]+', '/Users/<user>', encoded)
    return json.loads(encoded)


def write_report(folder: Path, result: dict) -> Path:
    safe = public_record(result)
    bi.dump(folder/'report.json', safe)
    ready = result['loaded_image_status'] == 'PASS'
    message = ('Загруженная сборка совпадает с проверенной. Тесты изображения в AE ещё не выполнены.' if ready else
               'Проверка версии остановлена. Причина записана в report.json. Ничего не устанавливалось.')
    (folder/'report.txt').write_text(message+'\n\n'+json.dumps(safe, ensure_ascii=False, indent=2)+'\n')
    output = folder.parent/(folder.name+'.zip')
    with zipfile.ZipFile(output, 'x', compression=zipfile.ZIP_DEFLATED) as archive:
        for name in ('report.json', 'report.txt'):
            archive.write(folder/name, name)
    output.chmod(0o600)
    return output


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--report-root', type=Path, default=Path.home()/'Desktop/ElasticGridFX-Diagnostics')
    parser.add_argument('--scan-root', type=Path, action='append', default=[])
    parser.add_argument('--manifest', type=Path, default=BASELINE)
    args = parser.parse_args()
    checked_path(args.manifest)
    manifest = json.loads(args.manifest.read_text())
    bi.validate_identity(manifest['build'])
    checked_path(args.report_root)
    args.report_root.mkdir(parents=True, exist_ok=True, mode=0o700)
    folder = args.report_root/('EGFX-check-'+uuid.uuid4().hex)
    folder.mkdir(mode=0o700)
    result = diagnose(folder, manifest, args.scan_root)
    # Package metadata is generated from clean Git when the diagnostic is exported.
    metadata = ROOT/'diagnostics/DiagnosticIdentity.json'
    if metadata.is_file():
        result['diagnostic'] = json.loads(metadata.read_text())
    else:
        result['diagnostic'] = {'source_state':'internal source execution', 'tool_sha256':bi.digest(Path(__file__).read_bytes())}
    output = write_report(folder, result)
    print(('Версия подтверждена.' if result['status']=='PASS' else 'Проверка завершена с ограничением.')+'\nОтчёт: '+str(output))
    print('Проекты, настройки и установленные плагины не изменялись.\nПришли полученный ZIP в чат.')
    return 0 if result['status']=='PASS' else 3


if __name__ == '__main__':
    try:
        sys.exit(main())
    except (OSError, ValueError, KeyError) as error:
        print('ERROR: '+str(error), file=sys.stderr)
        sys.exit(1)
