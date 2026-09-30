#!/usr/bin/env python3
"""Prepare a fresh smoke workspace; execute only in explicitly selected test AE."""
from __future__ import annotations
import argparse
import json
from pathlib import Path
import platform
import plistlib
import re
import subprocess
import sys
import uuid

import build_identity as bi
from install_candidate import checked_path, signature
from live_identity import running_ae
from smoke_pixels import FRAMES, pattern, validate_frames

ROOT = Path(__file__).resolve().parents[1]

GUARDS = frozenset((
    'CLEAN', 'DIRTY_UNAVAILABLE', 'DIRTY', 'DIRTY_INVALID', 'DIRTY_READ_ERROR',
    'SAVED', 'FILE_READ_ERROR', 'OCCUPIED', 'ITEMS_READ_ERROR', 'NO_PROJECT',
    'PROJECT_READ_ERROR', 'NOT_CHECKED',
))


class AEPhaseError(ValueError):
    """A bounded, shareable AE phase failure rather than a transport failure."""

    def __init__(self, message: str, record: dict | None = None):
        super().__init__(message)
        self.record = record


def _phase_record(folder: Path, name: str, run_id: str, fields: tuple[str, ...]) -> dict:
    path = bi.safe_file(folder, name)
    if path.stat().st_size > 16384:
        raise ValueError('oversized AE phase record: '+name)
    data = json.loads(path.read_text(encoding='utf-8-sig'))
    if data.get('run_id') != run_id or data.get('status') not in ('ARMED', 'CLEAN', 'CAPTURED', 'FAIL'):
        raise ValueError('missing, stale or invalid AE phase record: '+name)
    record = dict(run_id=run_id, status=data['status'])
    stage = data.get('stage')
    if not isinstance(stage, str) or not re.fullmatch(r'[a-z_]{1,64}', stage):
        raise ValueError('invalid AE phase stage: '+name)
    record['stage'] = stage
    for field in fields:
        value = data.get(field)
        if field in ('guard', 'fresh_guard'):
            if value not in GUARDS:
                raise ValueError('invalid AE project guard record: '+name)
        elif field in ('project_revision', 'fresh_project_revision'):
            if not isinstance(value, str) or not (value in ('INVALID', 'READ_ERROR', 'UNAVAILABLE', 'NOT_CHECKED') or
                                                  re.fullmatch(r'[1-9][0-9]{0,9}', value)):
                raise ValueError('invalid AE project revision record: '+name)
        elif field == 'ae_version':
            if not isinstance(value, str) or not re.fullmatch(r'[0-9A-Za-z._ -]{1,64}', value):
                raise ValueError('invalid AE version record: '+name)
        record[field] = value
    return record


def _proves_test_project_ownership(record: dict, guard_field: str, revision_field: str) -> bool:
    return record[guard_field] == 'CLEAN' or (
        record[guard_field] == 'DIRTY_UNAVAILABLE' and record[revision_field] == '1'
    )


def prepare(parent: Path, build: dict) -> tuple[Path, dict]:
    checked_path(parent)
    parent.mkdir(parents=True, exist_ok=True)
    run_id = uuid.uuid4().hex
    folder = parent / ('EGFX-smoke-' + run_id)
    folder.mkdir(mode=0o700)
    pattern(folder / 'pattern.png')
    source = (ROOT / 'tests/ae_runtime_smoke.jsx').read_text()
    config = dict(run_id=run_id, folder=str(folder.resolve()))
    (folder / 'arm.jsx').write_text(source + '\nelasticGridSmokeArm(' + json.dumps(config) + ');\n')
    (folder / 'disarm.jsx').write_text(source + '\nelasticGridSmokeDisarm(' + json.dumps(config) + ');\n')
    (folder / 'run.jsx').write_text(source + '\nelasticGridSmoke(' + json.dumps(config) + ');\n')
    metadata = dict(schema=1, run_id=run_id, expected_build=build, status='NOT RUN',
                    loaded_build_id=None, actual_ae_execution=False,
                    runner_sha256=bi.digest(Path(__file__).read_bytes()),
                    jsx_sha256=bi.digest(source.encode()),
                    comparator_sha256=bi.digest((ROOT/'tools/smoke_pixels.py').read_bytes()),
                    fixture_sha256=bi.digest((folder/'pattern.png').read_bytes()))
    bi.dump(folder/'run.json', metadata)
    return folder, metadata


def inspect_capture(folder: Path, metadata: dict) -> dict:
    capture = bi.safe_file(folder, 'capture.json')
    if capture.stat().st_size > 16384:
        raise ValueError('oversized capture result')
    data = json.loads(capture.read_text(encoding='utf-8-sig'))
    if data.get('run_id') != metadata['run_id'] or data.get('status') != 'CAPTURED':
        raise ValueError('missing, stale or unsuccessful capture')
    if data.get('project_bpc') != 32 or not isinstance(data.get('ae_version'), str) or not data['ae_version']:
        raise ValueError('capture is missing host information')
    if bi.digest(bi.safe_file(folder, 'pattern.png').read_bytes()) != metadata['fixture_sha256']:
        raise ValueError('input fixture changed during capture')
    result = validate_frames(folder)
    result['frames_sha256'] = {n: bi.digest(bi.safe_file(folder, n+'.png').read_bytes()) for n in FRAMES}
    result['ae_version'] = data['ae_version']
    return result


def _transport_context(metadata: dict, ae_app: Path, installed: Path, package: Path, manifest: Path):
    checked_path(ae_app, directory=True)
    checked_path(installed, directory=True)
    bi.verify(installed, package, manifest)
    signature(installed)
    if bi.payload_metadata(installed) != metadata['expected_build']:
        raise ValueError('candidate changed after smoke preparation')
    plist = bi.safe_file(ae_app, 'Contents/Info.plist')
    app_info = plistlib.loads(plist.read_bytes())
    identifier = app_info['CFBundleIdentifier']
    if not re.fullmatch(r'com\.adobe\.[A-Za-z0-9_.-]+', identifier, re.I) or 'aftereffects' not in identifier.lower():
        raise ValueError('selected application is not identified as After Effects')
    expected_executable = ae_app / 'Contents/MacOS' / app_info['CFBundleExecutable']
    hosts = running_ae()
    if len(hosts) != 1 or Path(hosts[0]['executable']) != expected_executable:
        raise ValueError('require exactly one running AE process at the selected app path')
    return identifier, hosts[0]['pid']


def _run_jsx(folder: Path, script_name: str, identifier: str, timeout_seconds: int = 125) -> int:
    apple = '''on run argv
set jsxText to read POSIX file (item 1 of argv) as «class utf8»
if not (running of application id "IDENTIFIER") then error "Selected test AE is not running"
with timeout of 120 seconds
    tell application id "IDENTIFIER"
        set resultCode to DoScript jsxText
    end tell
end timeout
return resultCode
end run
'''.replace('IDENTIFIER', identifier)
    result = subprocess.run(['/usr/bin/osascript', '-', str(folder/script_name)], input=apple,
                            capture_output=True, text=True, timeout=timeout_seconds)
    (folder/(script_name+'.transport.log')).write_text(result.stdout + '\n' + result.stderr)
    if result.returncode:
        raise ValueError('AE transport failed for '+script_name+'; see retained transport log')
    output = result.stdout.strip()
    if not re.fullmatch(r'-?[0-9]+', output):
        raise ValueError('AE transport returned no numeric script status for '+script_name+'; see retained transport log')
    return int(output)


def arm(folder: Path, metadata: dict, ae_app: Path, installed: Path, package: Path, manifest: Path) -> dict:
    identifier, pid = _transport_context(metadata, ae_app, installed, package, manifest)
    exit_code = _run_jsx(folder, 'arm.jsx', identifier)
    record = _phase_record(folder, 'arm.json', metadata['run_id'], ('ae_version', 'guard', 'project_revision'))
    if exit_code != 0:
        if record['status'] == 'FAIL' and record['stage'] == 'guard':
            raise AEPhaseError('AE arm guard refused: '+record['guard'], record)
        raise AEPhaseError('AE arm script exited '+str(exit_code), record)
    if (record['status'] != 'ARMED' or record['stage'] != 'armed' or
            not _proves_test_project_ownership(record, 'guard', 'project_revision')):
        raise AEPhaseError('AE arm phase record did not prove test-project ownership', record)
    metadata['target_pid'] = pid
    metadata['arm_status'] = 'ARMED'
    metadata['arm'] = record
    return metadata


def disarm(folder: Path, metadata: dict, ae_app: Path, installed: Path, package: Path, manifest: Path) -> dict:
    identifier, pid = _transport_context(metadata, ae_app, installed, package, manifest)
    if metadata.get('target_pid') != pid:
        raise ValueError('AE process changed before arm cleanup')
    exit_code = _run_jsx(folder, 'disarm.jsx', identifier)
    record = _phase_record(folder, 'disarm.json', metadata['run_id'], ('fresh_guard', 'fresh_project_revision'))
    if exit_code != 0:
        raise AEPhaseError('AE arm cleanup script exited '+str(exit_code), record)
    if (record['status'] != 'CLEAN' or record['stage'] != 'clean' or
            not _proves_test_project_ownership(record, 'fresh_guard', 'fresh_project_revision')):
        raise AEPhaseError('AE arm cleanup record did not prove a fresh test project', record)
    metadata['disarm_status'] = 'CLEAN'
    metadata['disarm'] = record
    return metadata


def execute(folder: Path, metadata: dict, ae_app: Path, installed: Path,
            package: Path, manifest: Path) -> dict:
    identifier, pid = _transport_context(metadata, ae_app, installed, package, manifest)
    if metadata.get('target_pid') is not None and metadata.get('target_pid') != pid:
        raise ValueError('AE process changed before pixel capture')
    metadata['target_pid'] = pid
    metadata['ae_execution_attempted'] = True
    exit_code = _run_jsx(folder, 'run.jsx', identifier)
    if exit_code != 0:
        raise ValueError('AE smoke script exited '+str(exit_code)+'; see retained transport log')
    pixels = inspect_capture(folder, metadata)
    metadata['actual_ae_execution'] = True
    metadata['pixels'] = pixels
    metadata['status'] = 'FAIL' if pixels['status'] == 'FAIL' else 'BLOCKED'
    metadata['remaining'] = ['actual loaded Build ID observation', 'guide drag/Undo/Redo',
                             'host compatibility and real Metal validation']
    # A successful PNG smoke MUST NOT approve an unidentified loaded binary.
    return metadata


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--run-root', type=Path, default=ROOT/'dist/ae-tests')
    parser.add_argument('--manifest', type=Path, default=ROOT/'dist/mac/ElasticGrid.artifact.json')
    parser.add_argument('--package', type=Path, default=ROOT/'dist/mac/ElasticGrid.plugin.zip')
    parser.add_argument('--installed-bundle', type=Path)
    parser.add_argument('--ae-app', type=Path, help='exact test AE .app; never select a latest version implicitly')
    parser.add_argument('--execute-in-test-ae', action='store_true')
    args = parser.parse_args()
    if args.execute_in_test_ae and (platform.system() != 'Darwin' or not args.ae_app or not args.installed_bundle):
        parser.error('execution requires macOS, --ae-app and --installed-bundle')
    checked_path(args.manifest)
    build = bi.validate_identity(json.loads(args.manifest.read_text())['build'])
    folder, metadata = prepare(args.run_root, build)
    try:
        if args.execute_in_test_ae:
            metadata = execute(folder, metadata, args.ae_app, args.installed_bundle, args.package, args.manifest)
    except subprocess.TimeoutExpired:
        metadata.update(status='BLOCKED', reason='Timeout: AE may still be executing. No host was killed; do not reuse this workspace.')
    except (ValueError, OSError, KeyError, subprocess.SubprocessError) as error:
        metadata.update(status='FAIL', reason=str(error))
    finally:
        bi.dump(folder/'run.json', metadata)
    print(json.dumps(dict(status=metadata['status'], report=str(folder/'run.json'),
                          loaded_build_id=None, pixels=metadata.get('pixels', {}).get('status','NOT RUN')), indent=2))
    if not args.execute_in_test_ae:
        return 0  # Preparation succeeded; run.json explicitly remains NOT RUN.
    return 1 if metadata['status'] == 'FAIL' else 3


if __name__ == '__main__':
    try:
        sys.exit(main())
    except (ValueError, OSError, KeyError) as error:
        print('ERROR: '+str(error), file=sys.stderr)
        sys.exit(1)
