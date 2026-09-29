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


def prepare(parent: Path, build: dict) -> tuple[Path, dict]:
    checked_path(parent)
    parent.mkdir(parents=True, exist_ok=True)
    run_id = uuid.uuid4().hex
    folder = parent / ('EGFX-smoke-' + run_id)
    folder.mkdir(mode=0o700)
    pattern(folder / 'pattern.png')
    source = (ROOT / 'tests/ae_runtime_smoke.jsx').read_text()
    config = dict(run_id=run_id, folder=str(folder.resolve()))
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


def execute(folder: Path, metadata: dict, ae_app: Path, installed: Path,
            package: Path, manifest: Path) -> dict:
    # Exact on-disk payload/signature checks, not a claim of a loaded Build ID.
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
    # Bundle IDs may be shared across installed AE versions. Require one exact
    # running executable, rather than letting LaunchServices pick another app.
    expected_executable = ae_app / 'Contents/MacOS' / app_info['CFBundleExecutable']
    hosts = running_ae()
    if len(hosts) != 1 or Path(hosts[0]['executable']) != expected_executable:
        raise ValueError('require exactly one running AE process at the selected app path')
    metadata['target_pid'] = hosts[0]['pid']
    # argv carries the path, not interpolated AppleScript or shell code.
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
    metadata['ae_execution_attempted'] = True
    result = subprocess.run(['/usr/bin/osascript', '-', str(folder/'run.jsx')], input=apple,
                            capture_output=True, text=True, timeout=125)
    # Keep output private in the per-run folder; no global log scraping.
    (folder/'transport.log').write_text(result.stdout + '\n' + result.stderr)
    if result.returncode or result.stdout.strip() != '0':
        raise ValueError('AE transport/capture failed; see retained transport log')
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
