#!/usr/bin/env python3
"""Stage 9 target-Mac gate for de31498: loaded identity + plane/3D/camera pixels."""
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
from install_candidate import checked_path
from plane_smoke_pixels import FRAMES, validate_plane_frames
from smoke_pixels import pattern

ROOT = Path(__file__).resolve().parents[1]
TARGET = Path.home()/'Library/Application Support/Adobe/Common/Plug-ins/7.0/MediaCore/FSTR FX/ElasticGrid.plugin'
AE2025 = Path('/Applications/Adobe After Effects 2025/Adobe After Effects 2025.app')
CANDIDATE = ROOT/'work/plane-candidate-de31498'
MANIFEST = CANDIDATE/'ElasticGrid.artifact.json'
PACKAGE = CANDIDATE/'ElasticGrid.plugin.zip'
EXPECTED_COMMIT = 'de314981005606741bc75c517d8bb33798b46a1d'
EXPECTED_BUILD = 'EGFX-0fa68430a170b3612e8d00f7'


def load_candidate(manifest_path: Path, package_path: Path) -> dict:
    checked_path(manifest_path)
    checked_path(package_path)
    if not manifest_path.is_file() or not package_path.is_file():
        raise ValueError('de31498 candidate manifest/package not found')
    manifest = json.loads(manifest_path.read_text())
    build = bi.validate_identity(manifest['build'])
    if build.get('commit') != EXPECTED_COMMIT or build.get('build_id') != EXPECTED_BUILD:
        raise ValueError('Stage 9 runner requires exact de31498 candidate identity')
    if manifest.get('package_sha256') != bi.digest(package_path.read_bytes()):
        raise ValueError('de31498 candidate package hash mismatch')
    return manifest


def prepare(parent: Path, build: dict) -> tuple[Path, dict]:
    checked_path(parent)
    parent.mkdir(parents=True, exist_ok=True, mode=0o700)
    run_id = uuid.uuid4().hex
    folder = parent/('EGFX-plane-'+run_id)
    folder.mkdir(mode=0o700)
    pattern(folder/'pattern.png', 128, 96)
    source = (ROOT/'tests/ae_plane_smoke.jsx').read_text()
    config = dict(run_id=run_id, folder=str(folder.absolute()))
    (folder/'run.jsx').write_text(source+'\nelasticGridPlaneSmoke('+json.dumps(config)+');\n')
    (folder/'cleanup.jsx').write_text(source+'\nelasticGridPlaneSmokeCleanup('+json.dumps(config)+');\n')
    metadata = dict(
        schema=1, run_id=run_id, expected_build=build, status='NOT RUN',
        actual_ae_execution=False, runner_sha256=bi.digest(Path(__file__).read_bytes()),
        jsx_sha256=bi.digest(source.encode()),
        comparator_sha256=bi.digest((ROOT/'tools/plane_smoke_pixels.py').read_bytes()),
        fixture_sha256=bi.digest((folder/'pattern.png').read_bytes()),
    )
    bi.dump(folder/'run.json', metadata)
    return folder, metadata


def phase_record(folder: Path, name: str, run_id: str, expected_status: str) -> dict:
    path = bi.safe_file(folder, name)
    if path.stat().st_size > 32768:
        raise ValueError('oversized Stage 9 AE phase record')
    data = json.loads(path.read_text(encoding='utf-8-sig'))
    if data.get('run_id') != run_id or data.get('status') != expected_status:
        raise ValueError('missing, stale or unsuccessful Stage 9 AE phase: '+name)
    return data


def write_report(run: Path, result: dict, fixture: Path) -> Path:
    public = dict(result)
    if 'identity' in public:
        public['identity'] = live_identity.public_record(public['identity'])
    bi.dump(run/'plane-acceptance.json', public)
    (run/'plane-acceptance.txt').write_text(
        ('STAGE 9 PLANE TEST PASS\n' if result['functional_status'] == 'PASS'
         else 'STAGE 9 PLANE TEST NOT PASSED\n') +
        'Release status: BLOCKED\n\n' +
        json.dumps(public, ensure_ascii=False, indent=2) + '\n'
    )
    output = run.parent/(run.name+'.zip')
    with zipfile.ZipFile(output, 'x', compression=zipfile.ZIP_DEFLATED) as archive:
        for name in ('plane-acceptance.json','plane-acceptance.txt'):
            archive.write(run/name, name)
        for name in ('plane-smoke.json','plane-cleanup.json'):
            file = fixture/name
            if file.is_file() and not file.is_symlink():
                archive.write(file, name)
        for name in FRAMES:
            frame = fixture/(name+'.png')
            if frame.is_file() and not frame.is_symlink():
                archive.write(frame, 'frames/'+frame.name)
    output.chmod(0o600)
    return output


def run_acceptance(report_root: Path, ae_app: Path, installed: Path,
                   manifest_path: Path, package_path: Path) -> tuple[dict, Path]:
    if platform.system() != 'Darwin' or platform.machine() != 'arm64':
        raise ValueError('Stage 9 acceptance requires Apple Silicon macOS')
    manifest = load_candidate(manifest_path, package_path)
    identifier, pid = ae_smoke_runner._transport_context(
        dict(expected_build=manifest['build']), ae_app, installed, package_path, manifest_path
    )

    checked_path(report_root)
    report_root.mkdir(parents=True, exist_ok=True, mode=0o700)
    run = report_root/('EGFX-PLANE-'+uuid.uuid4().hex)
    run.mkdir(mode=0o700)
    fixture, meta = prepare(run/'fixture', manifest['build'])
    result = dict(
        schema=1, run_id=run.name, candidate=manifest['build'],
        identity_status='NOT RUN', pixel_status='NOT RUN', cleanup_status='NOT RUN',
        functional_status='BLOCKED', release='BLOCKED',
        scope=('de31498 Stage 9 real-AE plane pixels, AEP roundtrip, 3D layer/camera/parenting; '
               'native guide drag/Undo/Redo remains separate'),
    )

    captured = False
    try:
        code = ae_smoke_runner._run_jsx(fixture, 'run.jsx', identifier)
        record = phase_record(fixture, 'plane-smoke.json', meta['run_id'], 'CAPTURED')
        result['capture'] = record
        # CAPTURED means the script intentionally left its exact saved fixture
        # open for identity sampling; cleanup is mandatory even if the matrix is
        # malformed or the transport returned an unexpected code.
        captured = True
        if code != 0 or record.get('stage') != 'complete' or tuple(record.get('frames',())) != FRAMES:
            result['reason'] = 'Stage 9 AE fixture did not complete the exact frame matrix'
        else:
            meta['actual_ae_execution'] = True

            identity_dir = run/'identity-private'
            identity_dir.mkdir(mode=0o700)
            identity = live_identity.diagnose(identity_dir, manifest, [])
            result['identity'] = identity
            result['identity_status'] = identity.get('loaded_image_status','NOT RUN')
            identity_ok = (
                identity.get('status') == 'PASS' and
                identity.get('loaded_image_status') == 'PASS' and
                identity.get('observed_build_id') == EXPECTED_BUILD and
                identity.get('ae',{}).get('pid') == pid and
                Path(identity.get('ae',{}).get('path','')) == ae_app
            )
            if not identity_ok:
                result['reason'] = 'loaded de31498 identity was not proven in the fixture AE process'
            else:
                pixels = validate_plane_frames(fixture)
                result['pixels'] = pixels
                result['pixel_status'] = pixels['status']
                if pixels['status'] != 'PASS':
                    result['functional_status'] = 'FAIL'
                    result['reason'] = 'one or more Stage 9 plane/3D/camera pixel checks failed'
                else:
                    # Re-check immutable installed bytes after the live render work.
                    current_identifier, current_pid = ae_smoke_runner._transport_context(
                        dict(expected_build=manifest['build']), ae_app, installed, package_path, manifest_path
                    )
                    if current_identifier != identifier or current_pid != pid:
                        raise ValueError('AE process changed during Stage 9 acceptance')
                    result['functional_status'] = 'PASS'
                    result['reason'] = 'de31498 identity and automated Stage 9 plane/3D/camera pixel matrix passed'
    except subprocess.TimeoutExpired:
        result['reason'] = 'AE Stage 9 timeout; no host process was killed or retried'
    except (ValueError, OSError, KeyError, TypeError, AttributeError, subprocess.SubprocessError) as error:
        result['reason'] = type(error).__name__ + ': ' + str(error)
    finally:
        if captured:
            try:
                current_identifier, current_pid = ae_smoke_runner._transport_context(
                    dict(expected_build=manifest['build']), ae_app, installed, package_path, manifest_path
                )
                if current_identifier != identifier or current_pid != pid:
                    raise ValueError('AE process changed before Stage 9 cleanup')
                cleanup_code = ae_smoke_runner._run_jsx(fixture, 'cleanup.jsx', identifier)
                cleanup = phase_record(fixture, 'plane-cleanup.json', meta['run_id'], 'CLEAN')
                if cleanup_code != 0 or cleanup.get('stage') != 'complete':
                    raise ValueError('Stage 9 cleanup did not prove a fresh empty project')
                result['cleanup_status'] = 'CLEAN'
                result['cleanup'] = cleanup
            except (ValueError, OSError, KeyError, subprocess.SubprocessError) as error:
                result['cleanup_status'] = 'BLOCKED'
                result['cleanup_reason'] = str(error)
                if result.get('functional_status') == 'PASS':
                    result['functional_status'] = 'BLOCKED'
                    result['reason'] = 'pixel/identity checks passed but safe cleanup was not proven'

    return result, write_report(run, result, fixture)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--report-root', type=Path, default=Path.home()/'Desktop/ElasticGridFX-Plane-Acceptance')
    parser.add_argument('--ae-app', type=Path, default=AE2025)
    parser.add_argument('--installed-bundle', type=Path, default=TARGET)
    parser.add_argument('--manifest', type=Path, default=MANIFEST)
    parser.add_argument('--package', type=Path, default=PACKAGE)
    args = parser.parse_args()
    try:
        result, archive = run_acceptance(
            args.report_root, args.ae_app, args.installed_bundle, args.manifest, args.package
        )
    except (ValueError, OSError, KeyError, subprocess.SubprocessError) as error:
        print('BLOCKED: '+str(error), file=sys.stderr)
        return 3
    print('Stage 9 functional status: '+result['functional_status'])
    print('Release status: BLOCKED')
    print('Report: '+str(archive))
    return 0 if result['functional_status'] == 'PASS' and result.get('cleanup_status') == 'CLEAN' else 3


if __name__ == '__main__':
    sys.exit(main())
