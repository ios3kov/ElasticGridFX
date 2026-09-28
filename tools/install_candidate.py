#!/usr/bin/env python3
"""Inspect by default; create-only installation into an explicitly authorized test scope."""
from __future__ import annotations
import argparse
import contextlib
import fcntl
import json
import os
from pathlib import Path
import platform
import plistlib
import shutil
import stat
import subprocess
import sys
import tempfile
import uuid

import build_identity as bi

MEDIA_CORE = Path('/Library/Application Support/Adobe/Common/Plug-ins/7.0/MediaCore')


def checked_path(path: Path, directory: bool = False) -> Path:
    """Do not resolve away a symlink before checking it."""
    path = path.absolute()
    for component in [*reversed(path.parents), path]:
        try:
            info = component.lstat()
        except FileNotFoundError:
            continue
        if stat.S_ISLNK(info.st_mode):
            raise ValueError('symlinked install/scan path refused: ' + str(component))
    if directory and not path.is_dir():
        raise ValueError('an existing directory is required: ' + str(path))
    return path


def default_roots() -> list[Path]:
    roots = [MEDIA_CORE, Path.home() / 'Library/Application Support/Adobe/Common/Plug-ins/7.0/MediaCore']
    apps = Path('/Applications')
    if apps.is_dir():
        for app in apps.iterdir():
            if app.name.startswith('Adobe After Effects'):
                roots.extend([app / 'Plug-ins', app / 'Contents/Plug-ins'])
    return roots


def discover(roots: list[Path]) -> list[Path]:
    """Read-only; scan failure is never interpreted as no conflicts."""
    found: set[Path] = set()
    def walk(folder: Path, depth: int) -> None:
        checked_path(folder, directory=True)
        if depth > 12:
            raise ValueError('plugin search depth exceeded: ' + str(folder))
        for path in folder.iterdir():
            checked_path(path)
            if not path.is_dir():
                continue
            if path.suffix.lower() == '.plugin':
                candidate = 'elasticgrid' in path.name.lower()
                plist = path / 'Contents/Info.plist'
                if plist.exists():
                    checked_path(plist)
                    try:
                        identifier = plistlib.loads(plist.read_bytes()).get('CFBundleIdentifier', '')
                    except (plistlib.InvalidFileException, ValueError):
                        # Unknown/corrupt metadata is not safe evidence of absence.
                        raise ValueError('unreadable plugin identity: ' + str(path)) from None
                    candidate |= identifier == 'com.elasticgrid.fx'
                if candidate:
                    found.add(path)
                continue
            # Adobe documents parenthesized folders as non-loading directories.
            if path.name.startswith('(') and path.name.endswith(')'):
                continue
            walk(path, depth + 1)
    for root in sorted(set(roots)):
        checked_path(root)
        try:
            root.stat()
        except FileNotFoundError:
            continue
        walk(root, 0)
    return sorted(found)


def adobe_hosts_stopped() -> None:
    text = subprocess.check_output(['/bin/ps', '-axo', 'comm='], text=True, timeout=10)
    needles = ('after effects', 'aerender', 'premiere pro', 'adobe media encoder', 'dynamiclinkmanager')
    if any(any(needle in line.lower() for needle in needles) for line in text.splitlines()):
        raise ValueError('Adobe host/render process is running; nothing installed or terminated')


def signature(bundle: Path) -> None:
    subprocess.run(['/usr/bin/codesign', '--verify', '--deep', '--strict', str(bundle)],
                   check=True, capture_output=True, text=True, timeout=30)


def inspect(bundle: Path, package: Path, manifest: Path, roots: list[Path]) -> dict:
    checked_path(bundle, directory=True)
    for path in (package, manifest):
        checked_path(path)
    bi.verify(bundle, package, manifest)
    return dict(build=bi.payload_metadata(bundle), conflicts=[str(p) for p in discover(roots)],
                scope='explicit roots only; custom AE search paths must be supplied',
                runtime_verification='NOT RUN')


@contextlib.contextmanager
def install_lock(scope: Path):
    path = scope / '.elasticgrid-install.lock'
    fd = os.open(path, os.O_CREAT | os.O_RDWR | os.O_NOFOLLOW, 0o600)
    try:
        if not stat.S_ISREG(os.fstat(fd).st_mode):
            raise ValueError('install lock is not a regular file')
        fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
        yield
    finally:
        os.close(fd)


def install(bundle: Path, package: Path, manifest: Path, scope: Path, roots: list[Path],
            *, verify_signature=signature, check_hosts=adobe_hosts_stopped) -> dict:
    """Dependencies are injectable ONLY for unit fixtures, never via CLI flags."""
    scope = checked_path(scope, directory=True)
    bundle = checked_path(bundle, directory=True)
    if bundle.name != 'ElasticGrid.plugin' or scope == bundle or bundle in scope.parents:
        raise ValueError('invalid source or install scope')
    roots = [*roots, scope]
    destination = scope / bundle.name
    record = inspect(bundle, package, manifest, roots)
    verify_signature(bundle)
    check_hosts()
    # One coordinator per scope; publication also reserves the target with mkdir.
    with install_lock(scope):
        conflicts = discover(roots)
        if conflicts:
            if conflicts == [destination] and bi.payload_files(destination) == bi.payload_files(bundle):
                verify_signature(destination)
                return dict(status='ALREADY_PRESENT', build=record['build'], runtime_verification='NOT RUN')
            raise ValueError('existing/conflicting plugin preserved: ' + ', '.join(map(str, conflicts)))
        checked_path(destination)
        if destination.exists():
            raise ValueError('destination already exists; refusing overwrite')
        if sorted(p.name for p in bundle.iterdir()) != ['Contents']:
            raise ValueError('unexpected bundle layout')
        run_id = uuid.uuid4().hex
        stage = Path(tempfile.mkdtemp(prefix='(ElasticGridFX-stage-', suffix=')', dir=scope))
        staged = stage / bundle.name
        receipt = scope / ('ElasticGridFX-install-' + run_id + '.json')
        report = dict(run_id=run_id, status='NOT RUN', build=record['build'],
                      destination=str(destination), staging=str(stage), runtime_verification='NOT RUN')
        try:
            shutil.copytree(bundle, staged, symlinks=True)
            bi.verify(staged, package, manifest)
            verify_signature(staged)
            check_hosts()
            if discover(roots):
                raise ValueError('a conflicting copy appeared during staging; nothing replaced')
            checked_path(scope, directory=True)
            # mkdir is exclusive even for an existing empty directory/symlink.
            destination.mkdir(mode=0o755)
            os.rename(staged / 'Contents', destination / 'Contents')
            bi.verify(destination, package, manifest)
            verify_signature(destination)
            report['status'] = 'INSTALLED_FOR_TEST'
            staged.rmdir()
            stage.rmdir()
            return report
        except Exception as error:
            report.update(status='FAIL', error=str(error))
            # Never recursively delete on an error; retain staging/partial result.
            raise
        finally:
            with receipt.open('x', encoding='utf-8') as handle:
                json.dump(report, handle, indent=2)
                handle.write('\n')


def main() -> int:
    root = Path(__file__).resolve().parents[1]
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--bundle', type=Path, default=root / 'dist/mac/ElasticGrid.plugin')
    parser.add_argument('--package', type=Path, default=root / 'dist/mac/ElasticGrid.plugin.zip')
    parser.add_argument('--manifest', type=Path, default=root / 'dist/mac/ElasticGrid.artifact.json')
    parser.add_argument('--scan-root', action='append', type=Path, default=[], help='additional custom plugin root')
    parser.add_argument('--test-scope', type=Path, help='existing authorized test plugin directory')
    parser.add_argument('--apply-test-install', action='store_true', help='explicit create-only test installation')
    args = parser.parse_args()
    roots = [*default_roots(), *args.scan_root]
    if args.test_scope:
        roots.append(args.test_scope)
    if args.apply_test_install:
        if platform.system() != 'Darwin' or not args.test_scope:
            parser.error('test installation requires macOS and --test-scope')
        record = inspect(args.bundle, args.package, args.manifest, roots)
        target = {'arm64': 'aarch64-apple-darwin', 'x86_64': 'x86_64-apple-darwin'}.get(platform.machine())
        if record['build']['target'] != target:
            raise ValueError('candidate target does not match the native host')
        result = install(args.bundle, args.package, args.manifest, args.test_scope, roots)
    else:
        result = inspect(args.bundle, args.package, args.manifest, roots)
        result['status'] = 'INSPECTED_ONLY'
    print(json.dumps(result, indent=2))
    return 0


if __name__ == '__main__':
    try:
        sys.exit(main())
    except (OSError, ValueError, KeyError, subprocess.SubprocessError) as error:
        print('ERROR: ' + str(error), file=sys.stderr)
        sys.exit(1)
