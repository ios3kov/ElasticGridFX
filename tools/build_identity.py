#!/usr/bin/env python3
"""Deterministic source/build identity and signed-payload verification. No AE claims."""
from __future__ import annotations
import argparse
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import re
import shlex
import shutil
import subprocess
import sys
import zipfile

SETTINGS = ('RUSTFLAGS', 'CARGO_ENCODED_RUSTFLAGS', 'CFLAGS', 'CXXFLAGS',
            'CPPFLAGS', 'CC', 'CXX', 'MACOSX_DEPLOYMENT_TARGET')
GENERATED = {'.git', 'dist', 'build', '.preflight-macos', '.pytest_cache'}


def build_settings():
    # Cargo features change executable behavior even with identical source and
    # release flags. Include active features, not unrelated environment values.
    keys=set(SETTINGS) | {key for key in os.environ if re.fullmatch(r'CARGO_FEATURE_[A-Z0-9_]+',key)}
    return {key:os.environ.get(key,'') for key in sorted(keys)}


def encoded(value):
    return json.dumps(value, sort_keys=True, separators=(',', ':'), ensure_ascii=True).encode()


def digest(data):
    return hashlib.sha256(data).hexdigest()


def run(args, cwd=None):
    return subprocess.check_output(args, cwd=cwd, stderr=subprocess.PIPE, timeout=30).decode().strip()


def cxx_version():
    default = 'cl.exe' if os.name == 'nt' else 'clang++'
    command = shlex.split(os.environ.get('CXX', default))
    if os.name == 'nt' and Path(command[0]).name.lower() in ('cl', 'cl.exe'):
        completed = subprocess.run(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                                   timeout=30, check=False, text=True)
        lines = (completed.stderr + '\n' + completed.stdout).strip().splitlines()
        if not lines:
            raise RuntimeError('MSVC compiler banner unavailable')
        return lines[0]
    return run([*command, '--version']).splitlines()[0]


def git(root, *args):
    return run(['git', '-C', str(root), *args])


def safe_file(root, name):
    rel = PurePosixPath(name)
    if any(ord(c) < 32 for c in name):
        raise ValueError('control character in identity path')
    if rel.is_absolute() or not rel.parts or rel.as_posix() != name or any(p in ('..', '.') for p in rel.parts) or '\\' in name:
        raise ValueError('unsafe manifest path')
    path = root
    for part in rel.parts:
        path = path / part
        if path.is_symlink():
            raise ValueError('symlinks are not allowed in source/payload identity')
    if not path.is_file():
        raise ValueError('missing identity file: ' + name)
    return path


def hashes(root, names):
    return {name: {'sha256': digest(safe_file(root, name).read_bytes()),
                   'executable': bool(safe_file(root, name).stat().st_mode & 0o111)}
            for name in sorted(names)}


def source_record(root, snapshot=None):
    root = root.resolve()
    if snapshot:
        record = json.loads(Path(snapshot).read_text())
        if record.get('schema') != 1 or record.get('source_state') != 'clean':
            raise ValueError('snapshot must describe clean committed source')
        if not re.fullmatch('[0-9a-f]{40}', record.get('commit', '')):
            raise ValueError('invalid snapshot commit')
        expected = record['files']
        actual = hashes(root, expected)
        if actual != expected or digest(encoded(actual)) != record.get('source_sha256'):
            raise ValueError('snapshot source was modified')
        # Detect additional code, not only modified/deleted listed files.
        for path in root.rglob('*'):
            rel = path.relative_to(root)
            if rel.parts[0] in GENERATED or rel.parts[0].startswith('build-') or '__pycache__' in rel.parts:
                continue
            if rel.parts[:2] == ('host-rust', 'target'):
                continue
            if path.is_symlink() or (path.is_file() and rel.as_posix() not in expected):
                raise ValueError('unexpected snapshot file: ' + rel.as_posix())
        if (root / '.git').exists() and source_record(root) != record:
            raise ValueError('snapshot does not match current Git state')
        return record
    if Path(git(root, 'rev-parse', '--show-toplevel')).resolve() != root:
        raise ValueError('source root is not the Git root; use a verified snapshot')
    commit = git(root, 'rev-parse', 'HEAD')
    names = git(root, 'ls-files', '-z', '--cached', '--others', '--exclude-standard').split('\0')
    names = sorted(set(filter(None, names)))
    files = hashes(root, names)
    cargo = (root / 'host-rust/Cargo.toml').read_text()
    version = re.search(r'^version\s*=\s*"([0-9]+\.[0-9]+\.[0-9]+[A-Za-z0-9.+-]*)"', cargo, re.M)
    if not version:
        raise ValueError('missing package version')
    return dict(schema=1, version=version.group(1), commit=commit,
                source_state='dirty' if git(root, 'status', '--porcelain', '--untracked-files=all') else 'clean',
                source_sha256=digest(encoded(files)), files=files)


def identity(record, target, profile, toolchain, settings):
    if not re.fullmatch('[A-Za-z0-9_.-]{1,80}', target) or profile not in ('debug', 'release'):
        raise ValueError('invalid target/profile')
    result = {k: record[k] for k in ('version', 'commit', 'source_state', 'source_sha256')}
    result.update(schema=1, artifact_type='AE native effect (.plugin)', target=target, profile=profile, toolchain=toolchain,
                  settings_sha256=digest(encoded(settings)))
    result['build_id'] = 'EGFX-' + digest(encoded(result))[:24]
    return result


def validate_identity(meta):
    data = dict(meta)
    build_id = data.pop('build_id')
    if build_id != 'EGFX-' + digest(encoded(data))[:24] or data.get('schema') != 1:
        raise ValueError('invalid build identity')
    if data.get('source_state') != 'clean':
        raise ValueError('dirty builds cannot be packaged or delivered')
    return meta


def dump(path, value):
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(json.dumps(value, indent=2, sort_keys=True).encode() + b'\n')


def about_message(version):
    # PF_OutData.return_msg is a legacy A_char buffer, not a UTF-8 string.
    # AE 25.6 on macOS renders a single 0xA9 byte as the copyright symbol.
    # Writing UTF-8 C2 A9 makes the host display an extra leading character.
    return (b'FSTR Stretch\rVersion ' + version.encode('ascii') +
            b'\r\rProfessional mesh deformation for Adobe After Effects' +
            b'\r\r\xa9 2026 FSTR.tech. All rights reserved')


def generate(root, out, target, profile):
    snapshot = os.environ.get('ELASTICGRID_SOURCE_RECORD')
    record = source_record(root, snapshot)
    toolchain = dict(rustc=run([os.environ.get('RUSTC', 'rustc'), '--version']),
                     cxx=cxx_version())
    settings=build_settings()
    meta = identity(record, target, profile, toolchain, settings)
    out.mkdir(parents=True, exist_ok=True)
    dump(out / 'BuildIdentity.json', meta)
    marker = 'ElasticGridBuildID=' + meta['build_id']
    about = about_message(meta['version'])
    diagnostic = ('FSTR Stretch v' + meta['version'] + ' / ' + target + '\r' + marker +
                  '\rCommit: ' + meta['commit'] +
                  '\rSource: ' + meta['source_state'] + ' / ' + meta['source_sha256'][:16])
    if len(about) >= 256:
        raise ValueError('About message exceeds the AE ABI buffer')
    # Keep the product-facing About bytes exact. Provenance remains UTF-8/ASCII
    # in BuildIdentity and the noninteractive diagnostic string.
    rust = ('pub const ABOUT_BYTES: &[u8] = &[' +
            ', '.join(f'0x{byte:02x}' for byte in about) + '];\n' +
            '#[cfg(feature = "render-diagnostics")]\npub const BUILD_ID: &str = '+json.dumps(meta['build_id'])+';\n'+
            'pub const DIAGNOSTIC: &str = ' + json.dumps(diagnostic) + ';\n')
    (out / 'build_identity.rs').write_text(rust, encoding='ascii')
    for name in record['files']:
        print('cargo:rerun-if-changed=' + str(root / name))
    for name in ('src', 'host-rust/src', 'tools', 'tests', 'docs', '.github'):
        print('cargo:rerun-if-changed=' + str(root / name))
    if snapshot:
        print('cargo:rerun-if-changed=' + str(Path(snapshot).resolve()))
    else:
        for name in ('HEAD', 'index', 'packed-refs'):
            path = (root / git(root, 'rev-parse', '--git-path', name)).resolve()
            if path.exists():
                print('cargo:rerun-if-changed=' + str(path))
        ref = git(root, 'symbolic-ref', '-q', 'HEAD') if git(root, 'rev-parse', '--abbrev-ref', 'HEAD') != 'HEAD' else None
        if ref:
            path = (root / git(root, 'rev-parse', '--git-path', ref)).resolve()
            if path.exists():
                print('cargo:rerun-if-changed=' + str(path))
    for name in (*settings, 'ELASTICGRID_SOURCE_RECORD'):
        print('cargo:rerun-if-env-changed=' + name)


def payload_metadata(bundle):
    meta = validate_identity(json.loads(safe_file(bundle, 'Contents/Resources/BuildIdentity.json').read_text()))
    marker = ('ElasticGridBuildID=' + meta['build_id']).encode()
    if marker not in safe_file(bundle, 'Contents/MacOS/ElasticGrid').read_bytes():
        raise ValueError('binary does not match metadata Build ID')
    return meta


def stamp(root, bundle, cargo_log):
    candidates = []
    for line in cargo_log.read_text().splitlines():
        try:
            item = json.loads(line)
        except json.JSONDecodeError:
            continue
        if item.get('reason') == 'build-script-executed':
            path = Path(item['out_dir']) / 'BuildIdentity.json'
            if path.is_file():
                candidates.append(path)
    if len(candidates) != 1:
        raise ValueError('Cargo must identify exactly one build identity output')
    meta = validate_identity(json.loads(candidates[0].read_text()))
    record = source_record(root, os.environ.get('ELASTICGRID_SOURCE_RECORD'))
    if any(record[k] != meta[k] for k in ('commit', 'source_state', 'source_sha256')):
        raise ValueError('source changed after compilation or Cargo metadata is stale')
    destination = bundle / 'Contents/Resources/BuildIdentity.json'
    destination.parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(candidates[0], destination)
    payload_metadata(bundle)


def payload_files(bundle):
    if bundle.is_symlink():
        raise ValueError('bundle cannot be a symlink')
    paths = list(bundle.rglob('*'))
    if any(p.is_symlink() for p in paths):
        raise ValueError('symlink in payload')
    return hashes(bundle, [p.relative_to(bundle).as_posix() for p in paths if p.is_file()])


def verify_package(bundle, package, files):
    with zipfile.ZipFile(package) as archive:
        members = [i for i in archive.infolist() if not i.is_dir()]
        expected = {bundle.name + '/' + k: v for k, v in files.items()}
        if len(members) != len(expected) or {i.filename for i in members} != set(expected):
            raise ValueError('archive contents differ from signed payload')
        for item in members:
            record = expected[item.filename]
            if bool((item.external_attr >> 16) & 0o111) != record['executable']:
                raise ValueError('archive execute permission changed')
            if digest(archive.read(item)) != record['sha256']:
                raise ValueError('archive payload bytes changed')


def seal(bundle, package, output):
    base = bundle.resolve()
    if package.resolve() == output.resolve() or any(base == p.resolve() or base in p.resolve().parents for p in (package, output)):
        raise ValueError('package/manifest must be separate files outside the signed bundle')
    meta = payload_metadata(bundle)
    files = payload_files(bundle)
    # Build the final ZIP from the already-signed payload; never rebuild the binary.
    with zipfile.ZipFile(package, 'w', compression=zipfile.ZIP_DEFLATED) as archive:
        for name in files:
            archive.write(bundle / name, bundle.name + '/' + name)
    verify_package(bundle, package, files)
    dump(output, dict(schema=1, build=meta, files=files,
                      package_sha256=digest(package.read_bytes()),
                      runtime_verification='NOT RUN'))


def verify(bundle, package, manifest):
    record = json.loads(manifest.read_text())
    if record.get('schema') != 1 or record['build'] != payload_metadata(bundle):
        raise ValueError('manifest identity mismatch')
    if record['files'] != payload_files(bundle):
        raise ValueError('signed payload files were changed')
    if record['package_sha256'] != digest(package.read_bytes()):
        raise ValueError('package hash mismatch')
    verify_package(bundle, package, record['files'])


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action', choices=('snapshot', 'generate', 'stamp', 'seal', 'verify'))
    for key in ('root', 'out', 'bundle', 'cargo-log', 'package', 'manifest'):
        parser.add_argument('--' + key, type=Path)
    parser.add_argument('--target')
    parser.add_argument('--profile', default='release')
    args = parser.parse_args()
    required = {'snapshot': ('root', 'out'), 'generate': ('root', 'out', 'target'),
                'stamp': ('root', 'bundle', 'cargo_log'), 'seal': ('bundle', 'package', 'out'),
                'verify': ('bundle', 'package', 'manifest')}
    if any(getattr(args, k) is None for k in required[args.action]):
        parser.error('missing arguments for ' + args.action)
    if args.action == 'snapshot':
        record = source_record(args.root)
        if record['source_state'] != 'clean':
            raise ValueError('reproducible source snapshot requires clean Git')
        dump(args.out, record)
    elif args.action == 'generate':
        generate(args.root.resolve(), args.out, args.target, args.profile)
    elif args.action == 'stamp':
        stamp(args.root, args.bundle, args.cargo_log)
    elif args.action == 'seal':
        seal(args.bundle, args.package, args.out)
    else:
        verify(args.bundle, args.package, args.manifest)


if __name__ == '__main__':
    try:
        main()
    except (ValueError, KeyError, OSError, subprocess.SubprocessError, zipfile.BadZipFile) as error:
        print('ERROR: ' + str(error), file=sys.stderr)
        sys.exit(1)
