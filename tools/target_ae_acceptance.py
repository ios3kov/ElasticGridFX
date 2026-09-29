#!/usr/bin/env python3
"""One target-Mac functional gate: live candidate identity + deterministic AE pixels."""
from __future__ import annotations
import argparse
import json
from pathlib import Path
import platform
import subprocess
import sys
import uuid
import zipfile

import ae_smoke_runner
import build_identity as bi
import live_identity
from install_candidate import checked_path, signature
from smoke_pixels import FRAMES

ROOT = Path(__file__).resolve().parents[1]
TARGET = Path.home()/'Library/Application Support/Adobe/Common/Plug-ins/7.0/MediaCore/FSTR FX/ElasticGrid.plugin'
AE2025 = Path('/Applications/Adobe After Effects 2025/Adobe After Effects 2025.app')
MANIFEST = ROOT/'diagnostics/candidate-fd69988.json'
PACKAGE = ROOT/'payload/ElasticGrid.plugin.zip'


def load_candidate(manifest_path: Path = MANIFEST, package_path: Path = PACKAGE) -> dict:
    checked_path(manifest_path)
    checked_path(package_path)
    manifest = json.loads(manifest_path.read_text())
    meta = bi.validate_identity(manifest['build'])
    if meta['commit'] != 'fd69988c10b25268eb8cad6ee6ced7f6a28bee9d':
        raise ValueError('acceptance package is pinned to the wrong plugin commit')
    if bi.digest(package_path.read_bytes()) != manifest['package_sha256']:
        raise ValueError('candidate package hash mismatch')
    return manifest


def verify_installed(bundle: Path, package: Path, manifest_path: Path) -> None:
    checked_path(bundle, directory=True)
    checked_path(manifest_path)
    bi.verify(bundle, package, manifest_path)
    signature(bundle)


def write_report(run: Path, result: dict, smoke_folder: Path | None) -> Path:
    public = dict(result)
    if 'identity' in public:
        public['identity'] = live_identity.public_record(public['identity'])
    bi.dump(run/'acceptance.json', public)
    (run/'acceptance.txt').write_text(
        ('FUNCTIONAL TEST PASS\n' if result['functional_status'] == 'PASS' else
         'FUNCTIONAL TEST NOT PASSED\n') +
        'Release status: BLOCKED\n\n' +
        json.dumps(public, ensure_ascii=False, indent=2) + '\n'
    )
    output = run.parent/(run.name+'.zip')
    with zipfile.ZipFile(output, 'x', compression=zipfile.ZIP_DEFLATED) as archive:
        archive.write(run/'acceptance.json', 'acceptance.json')
        archive.write(run/'acceptance.txt', 'acceptance.txt')
        if smoke_folder is not None:
            for name in FRAMES:
                frame = smoke_folder/(name+'.png')
                if frame.is_file() and not frame.is_symlink():
                    archive.write(frame, 'frames/'+frame.name)
            capture = smoke_folder/'capture.json'
            if capture.is_file() and not capture.is_symlink():
                archive.write(capture, 'capture.json')
    output.chmod(0o600)
    return output


def run_acceptance(report_root: Path, ae_app: Path, installed: Path,
                   manifest_path: Path = MANIFEST, package_path: Path = PACKAGE) -> tuple[dict, Path]:
    if platform.system() != 'Darwin' or platform.machine() != 'arm64':
        raise ValueError('target acceptance requires Apple Silicon macOS')
    manifest = load_candidate(manifest_path, package_path)
    checked_path(ae_app, directory=True)
    verify_installed(installed, package_path, manifest_path)

    checked_path(report_root)
    report_root.mkdir(parents=True, exist_ok=True, mode=0o700)
    run = report_root/('EGFX-AE-'+uuid.uuid4().hex)
    run.mkdir(mode=0o700)
    identity_dir = run/'identity-private'
    identity_dir.mkdir(mode=0o700)

    result = dict(schema=1, run_id=run.name, candidate=manifest['build'],
                  identity_status='NOT RUN', pixel_status='NOT RUN',
                  functional_status='BLOCKED', release='BLOCKED',
                  scope='real AE functional smoke only; not release/performance/Metal acceptance')
    smoke_folder, smoke_meta = ae_smoke_runner.prepare(run/'smoke', manifest['build'])
    try:
        smoke_meta = ae_smoke_runner.arm(
            smoke_folder, smoke_meta, ae_app, installed, package_path, manifest_path
        )
    except (ValueError, OSError, KeyError, subprocess.SubprocessError) as error:
        result['reason'] = 'AE arm phase blocked: ' + str(error)
        return result, write_report(run, result, smoke_folder)

    identity = live_identity.diagnose(identity_dir, manifest, [])
    result['identity'] = identity
    result['identity_status'] = identity.get('loaded_image_status', 'NOT RUN')
    identity_ok = (identity.get('status') == 'PASS'
                   and identity.get('loaded_image_status') == 'PASS'
                   and identity.get('observed_build_id') == manifest['build']['build_id']
                   and Path(identity.get('ae',{}).get('path','')) == ae_app
                   and identity.get('ae',{}).get('pid') == smoke_meta.get('target_pid'))
    try:
        smoke_meta = ae_smoke_runner.disarm(
            smoke_folder, smoke_meta, ae_app, installed, package_path, manifest_path
        )
    except (ValueError, OSError, KeyError, subprocess.SubprocessError) as error:
        result['reason'] = 'AE arm cleanup blocked: ' + str(error)
        return result, write_report(run, result, smoke_folder)
    if not identity_ok:
        result['reason'] = 'loaded candidate identity was not proven after controlled effect instantiation'
        return result, write_report(run, result, smoke_folder)

    try:
        smoke_result = ae_smoke_runner.execute(
            smoke_folder, smoke_meta, ae_app, installed, package_path, manifest_path
        )
    except subprocess.TimeoutExpired:
        result['reason'] = 'AE smoke timeout; no process was killed and no retry was attempted'
        return result, write_report(run, result, smoke_folder)
    except (ValueError, OSError, KeyError, subprocess.SubprocessError) as error:
        result['reason'] = 'AE smoke blocked: ' + str(error)
        smoke_meta['status'] = 'BLOCKED'
        smoke_meta['reason'] = str(error)
        bi.dump(smoke_folder/'run.json', smoke_meta)
        return result, write_report(run, result, smoke_folder)

    bi.dump(smoke_folder/'run.json', smoke_result)
    result['smoke'] = smoke_result
    result['pixel_status'] = smoke_result.get('pixels', {}).get('status', 'NOT RUN')
    if smoke_result.get('target_pid') != identity['ae']['pid']:
        result['reason'] = 'AE process changed between identity and pixel test'
        return result, write_report(run, result, smoke_folder)
    if not smoke_result.get('actual_ae_execution'):
        result['reason'] = 'AE script did not complete'
        return result, write_report(run, result, smoke_folder)
    if result['pixel_status'] != 'PASS':
        result['functional_status'] = 'FAIL'
        result['reason'] = 'one or more deterministic pixel checks failed'
        return result, write_report(run, result, smoke_folder)

    verify_installed(installed, package_path, manifest_path)
    result['functional_status'] = 'PASS'
    result['reason'] = 'loaded fd69988 and all deterministic AE pixel checks passed in one process'
    return result, write_report(run, result, smoke_folder)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--report-root', type=Path, default=Path.home()/'Desktop/ElasticGridFX-AE-Acceptance')
    parser.add_argument('--ae-app', type=Path, default=AE2025)
    parser.add_argument('--installed-bundle', type=Path, default=TARGET)
    args = parser.parse_args()
    try:
        result, archive = run_acceptance(args.report_root, args.ae_app, args.installed_bundle)
    except (ValueError, OSError, KeyError, subprocess.SubprocessError) as error:
        print('BLOCKED: '+str(error), file=sys.stderr)
        return 3
    print('Functional status: '+result['functional_status'])
    print('Release status: BLOCKED')
    print('Report: '+str(archive))
    return 0 if result['functional_status'] == 'PASS' else 3


if __name__ == '__main__':
    sys.exit(main())
