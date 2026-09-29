#!/usr/bin/env python3
"""Explicit, reversible replacement of the ONE user-authorized ElasticGrid copy."""
from __future__ import annotations
import argparse
import ctypes
import json
import os
from pathlib import Path
import platform
import plistlib
import pwd
import stat
import subprocess
import sys
import uuid
import zipfile

import build_identity as bi
from xattr_reader import attribute_hashes
from install_candidate import checked_path, adobe_hosts_stopped, signature, install_lock

ROOT = Path(__file__).resolve().parents[1]
TARGET = Path('Library/Application Support/Adobe/Common/Plug-ins/7.0/MediaCore/FSTR FX/ElasticGrid.plugin')
BACKUPS = Path('Library/Application Support/ElasticGridFX/Test Backups')
OLD_SHA = '4958d73bff702cbf21fa46d1aee670fa524cc99c57f2423c62a7d0cae32ebae7'
PACKAGE_SHA = '68c135f1a2a9390a0032a31d9073c8111bff6dd43691b9222ec50fad3913982e'
PREVIOUS_MANIFEST = 'candidate-6d3b846.json'
CANDIDATE_MANIFEST = 'candidate-fd69988.json'
MAX_BYTES = 32 * 1024 * 1024


def native_swap(a: Path, b: Path) -> None:
    """No copy/delete fallback: unsupported filesystem/permission means no swap."""
    if platform.system() != 'Darwin':
        raise ValueError('Atomic replacement requires macOS')
    checked_path(a, directory=True); checked_path(b, directory=True)
    lib = ctypes.CDLL('/usr/lib/libSystem.B.dylib', use_errno=True)
    fn = lib.renamex_np
    fn.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_uint]
    fn.restype = ctypes.c_int
    if fn(os.fsencode(a), os.fsencode(b), 0x02 | 0x10) != 0:
        error = ctypes.get_errno()
        raise OSError(error, os.strerror(error))


def snapshot(root: Path) -> dict:
    """Record identity/content/modes/xattrs; rename changes ctime, not this state."""
    checked_path(root, directory=True)
    paths = [root, *sorted(root.rglob('*'))]
    if len(paths) > 4096:
        raise ValueError('Unexpectedly large plugin tree')
    out, total = {}, 0
    for path in paths:
        info = path.lstat()
        if not (stat.S_ISREG(info.st_mode) or stat.S_ISDIR(info.st_mode)):
            raise ValueError('Unsupported/symlinked plugin entry')
        item = dict(mode=info.st_mode, uid=info.st_uid, gid=info.st_gid,
                    inode=info.st_ino, device=info.st_dev,
                    flags=getattr(info, 'st_flags', 0), mtime_ns=info.st_mtime_ns)
        item['xattrs'] = attribute_hashes(path)
        if stat.S_ISREG(info.st_mode):
            total += info.st_size
            if total > MAX_BYTES:
                raise ValueError('Unexpectedly large plugin payload')
            item['sha256'] = bi.digest(path.read_bytes())
        out['.' if path == root else path.relative_to(root).as_posix()] = item
    return out


def save_receipt(path: Path, record: dict) -> None:
    temp = path.parent / ('receipt-write-' + uuid.uuid4().hex + '.json')
    with temp.open('x', encoding='utf-8') as handle:
        json.dump(record, handle, indent=2, sort_keys=True)
        handle.write('\n'); handle.flush(); os.fsync(handle.fileno())
    os.replace(temp, path)  # Only this transaction's own receipt is replaced.
    fd = os.open(path.parent, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW)
    try:
        os.fsync(fd)
    finally:
        os.close(fd)


def unpack(package: Path, staged: Path, manifest: dict) -> None:
    checked_path(package)
    if package.stat().st_size > MAX_BYTES or bi.digest(package.read_bytes()) != manifest['package_sha256']:
        raise ValueError('Candidate ZIP checksum mismatch')
    with zipfile.ZipFile(package) as z:
        expected = {'ElasticGrid.plugin/' + k: v for k, v in manifest['files'].items()}
        members = z.infolist()
        if len(members) != len(expected) or {i.filename for i in members} != set(expected):
            raise ValueError('Unexpected archive entries')
        if sum(i.file_size for i in members) > MAX_BYTES:
            raise ValueError('Candidate archive exceeds size limit')
        staged.mkdir()
        for i in members:
            mode = i.external_attr >> 16
            if not stat.S_ISREG(mode) or i.flag_bits & 1:
                raise ValueError('Unsupported archive member')
            name = i.filename.removeprefix('ElasticGrid.plugin/')
            rel = Path(name)
            if rel.is_absolute() or '..' in rel.parts or rel.as_posix() != name or '\\' in name:
                raise ValueError('Unsafe archive path')
            data = z.read(i)
            if bi.digest(data) != expected[i.filename]['sha256']:
                raise ValueError('Candidate member checksum mismatch')
            if bool(mode & 0o111) != expected[i.filename]['executable']:
                raise ValueError('Candidate member permission mismatch')
            dest = staged / rel
            dest.parent.mkdir(parents=True, exist_ok=True)
            with dest.open('xb') as f:
                f.write(data); f.flush(); os.fsync(f.fileno())
            dest.chmod(0o755 if expected[i.filename]['executable'] else 0o644)
    verify_new(staged, manifest)


def verify_new(bundle: Path, manifest: dict) -> None:
    if bi.payload_files(bundle) != manifest['files'] or bi.payload_metadata(bundle) != manifest['build']:
        raise ValueError('Candidate payload mismatch')


def validate_old(target: Path, old_sha: str) -> None:
    checked_path(target, directory=True)
    if plistlib.loads(bi.safe_file(target, 'Contents/Info.plist').read_bytes()).get('CFBundleIdentifier') != 'com.elasticgrid.fx':
        raise ValueError('Destination is not the authorized ElasticGrid plugin')
    binary = bi.safe_file(target, 'Contents/MacOS/ElasticGrid')
    if binary.stat().st_size > MAX_BYTES or bi.digest(binary.read_bytes()) != old_sha:
        raise ValueError('Existing plugin changed since authorization; left untouched')


def get_paths(home: Path) -> tuple[Path, Path]:
    home = checked_path(home, directory=True)
    target, backups = home / TARGET, home / BACKUPS
    checked_path(target, directory=True)
    checked_path(backups)
    # No cross-volume fallback; the original inode must survive in the backup.
    backups.mkdir(parents=True, exist_ok=True, mode=0o700)
    if backups.stat().st_uid != os.getuid() or backups.stat().st_mode & 0o022:
        raise ValueError('Backup folder must belong to this user and not be shared-writable')
    if target.stat().st_dev != backups.stat().st_dev:
        raise ValueError('Backup and plugin must be on the same filesystem')
    return target, backups


def receipt_candidates(backups: Path, home: Path, manifest: dict, old_sha: str) -> list[tuple[Path, dict]]:
    found = []
    for file in sorted(backups.glob('EGFX-update-*/receipt.json')):
        checked_path(file)
        if file.stat().st_size > 2 * 1024 * 1024:
            raise ValueError('Oversized update receipt')
        data = json.loads(file.read_text())
        if data.get('schema') != 1 or data.get('target_relative') != TARGET.as_posix():
            raise ValueError('Unexpected receipt scope')
        if data.get('candidate') != manifest or data.get('old_binary_sha256') != old_sha:
            continue
        if data.get('state') in ('PREPARED', 'INSTALLED_FOR_TEST', 'ROLLBACK_BLOCKED'):
            found.append((file, data))
    return found


def rollback_one(target: Path, file: Path, record: dict, *, swap, check_hosts) -> dict:
    previous = file.parent / 'previous.plugin'
    if snapshot(target) == record['old_snapshot']:
        record['state'] = 'ROLLED_BACK'
        save_receipt(file, record)
        return record
    # Do not overwrite unknown edits to either retained version.
    verify_new(target, record['candidate'])
    if snapshot(previous) != record['old_snapshot']:
        raise ValueError('Backup changed; rollback refused without touching either version')
    check_hosts()
    swap(target, previous)
    if snapshot(target) != record['old_snapshot']:
        raise ValueError('Restored original failed verification')
    record['state'] = 'ROLLED_BACK'
    save_receipt(file, record)
    return record


def replace(home: Path, package: Path, manifest: dict, *, old_sha=OLD_SHA,
            swap=native_swap, verify_signature=signature, check_hosts=adobe_hosts_stopped) -> tuple[Path, dict]:
    """Injectable OS operations are for isolated unit tests, not CLI overrides."""
    check_hosts()
    target, backups = get_paths(home)
    with install_lock(backups):
        active = receipt_candidates(backups, home, manifest, old_sha)
        if active:
            if len(active) != 1:
                raise ValueError('Ambiguous pending operation; no replacement performed')
            file, record = active[0]
            verify_new(target, manifest)
            if snapshot(file.parent/'previous.plugin') != record['old_snapshot']:
                raise ValueError('Pending operation/backup does not match; no replacement performed')
            verify_signature(target)
            record['state'] = 'INSTALLED_FOR_TEST'; save_receipt(file, record)
            return file, record
        validate_old(target, old_sha)
        old = snapshot(target)
        run = backups / ('EGFX-update-' + uuid.uuid4().hex)
        run.mkdir(mode=0o700)
        file, previous = run/'receipt.json', run/'previous.plugin'
        record = dict(schema=1, state='PREPARING', run_id=run.name,
                      target_relative=TARGET.as_posix(), old_binary_sha256=old_sha,
                      old_snapshot=old, candidate=manifest,
                      loaded_identity='NOT RUN', functional_ae='NOT RUN', release='BLOCKED')
        save_receipt(file, record)
        swapped = False
        try:
            unpack(package, previous, manifest)
            verify_signature(previous)
            check_hosts()
            if snapshot(target) != old:
                raise ValueError('Original changed during preparation; no replacement performed')
            record['state'] = 'PREPARED'; save_receipt(file, record)
            # Atomic swap retains the ORIGINAL directory, ACLs, forks and xattrs.
            swap(target, previous); swapped = True
            if snapshot(previous) != old:
                raise ValueError('Original backup identity mismatch')
            verify_new(target, manifest)
            verify_signature(target)
            record['state'] = 'INSTALLED_FOR_TEST'; save_receipt(file, record)
        except BaseException as error:
            record['error'] = type(error).__name__ + ': ' + str(error)
            # An interrupt can arrive after the syscall but before Python records
            # its return. Inspect actual locations instead of trusting that flag.
            try:
                swapped = swapped or snapshot(previous) == old
            except (OSError, ValueError):
                pass
            if swapped:
                try:
                    rollback_one(target, file, record, swap=swap, check_hosts=check_hosts)
                except BaseException as restore_error:
                    record['state'] = 'ROLLBACK_BLOCKED'
                    record['rollback_error'] = type(restore_error).__name__ + ': ' + str(restore_error)
            else:
                record['state'] = 'FAILED_BEFORE_SWAP'
            save_receipt(file, record)
        return file, record


def rollback(home: Path, manifest: dict, *, old_sha=OLD_SHA,
             swap=native_swap, check_hosts=adobe_hosts_stopped) -> tuple[Path, dict]:
    check_hosts()
    target, backups = get_paths(home)
    with install_lock(backups):
        active = receipt_candidates(backups, home, manifest, old_sha)
        if len(active) != 1:
            raise ValueError('Exactly one pending update is required for rollback')
        file, record = active[0]
        return file, rollback_one(target, file, record, swap=swap, check_hosts=check_hosts)


def report(home: Path, receipt: Path, record: dict) -> Path:
    folder = home/'Desktop/ElasticGridFX-Install'
    checked_path(folder); folder.mkdir(parents=True, exist_ok=True)
    output = folder/(record['run_id']+'-'+uuid.uuid4().hex[:8]+'.zip')
    public = dict(run_id=record['run_id'], state=record['state'],
                  target='~/'+TARGET.as_posix(), backup='~/'+receipt.parent.relative_to(home).as_posix()+'/previous.plugin',
                  build_id=record['candidate']['build']['build_id'],
                  package_sha256=record['candidate']['package_sha256'],
                  old_binary_sha256=record['old_binary_sha256'],
                  old_snapshot_sha256=bi.digest(bi.encoded(record['old_snapshot'])),
                  error=record.get('error'), rollback_error=record.get('rollback_error'),
                  loaded_identity='NOT RUN', functional_ae='NOT RUN', release='BLOCKED')
    text = json.dumps(public, ensure_ascii=False, indent=2).replace(str(home), '~')+'\n'
    with zipfile.ZipFile(output, 'x', compression=zipfile.ZIP_DEFLATED) as z:
        z.writestr('report.json', text)
        z.writestr('report.txt', text)
    return output


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--apply-authorized-replacement', action='store_true')
    parser.add_argument('--rollback', action='store_true')
    args = parser.parse_args()
    if args.apply_authorized_replacement == args.rollback:
        parser.error('Choose --apply-authorized-replacement or --rollback')
    if platform.system() != 'Darwin' or os.geteuid() == 0:
        raise ValueError('Run on your Mac as your normal user, not sudo')
    if platform.machine() != 'arm64':
        raise ValueError('This authorized candidate is Apple Silicon only')
    identity_file = ROOT/'InstallToolIdentity.json'
    tool = json.loads(identity_file.read_text())
    if tool.get('source_state') != 'clean' or tool.get('files') != bi.hashes(ROOT, tool['files']):
        raise ValueError('Installer files changed; download the verified package again')
    home = Path(pwd.getpwuid(os.getuid()).pw_dir)
    previous = json.loads((ROOT/'diagnostics'/PREVIOUS_MANIFEST).read_text())
    manifest = json.loads((ROOT/'diagnostics'/CANDIDATE_MANIFEST).read_text())
    bi.validate_identity(previous['build'])
    bi.validate_identity(manifest['build'])
    if previous['files']['Contents/MacOS/ElasticGrid']['sha256'] != OLD_SHA:
        raise ValueError('Wrong authorized previous candidate')
    if manifest['package_sha256'] != PACKAGE_SHA:
        raise ValueError('Wrong authorized candidate')
    target = checked_path(home / TARGET, directory=True)
    if bi.payload_files(target) != previous['files'] or bi.payload_metadata(target) != previous['build']:
        raise ValueError('Installed plugin is not the authorized 6d3b846 candidate; left untouched')
    signature(target)
    if args.rollback:
        file, record = rollback(home, manifest)
    else:
        file, record = replace(home, ROOT/'payload/ElasticGrid.plugin.zip', manifest)
    output = report(home, file, record)
    print('Статус: '+record['state']+'\nОтчёт: '+str(output))
    if record['state'] == 'INSTALLED_FOR_TEST':
        print('Оригинал сохранён в резервной папке. AE не запускался.')
    elif record['state'] == 'ROLLED_BACK':
        print('Оригинал восстановлен. AE не запускался.')
    else:
        print('Операция не завершена. Сохрани отчёт и не удаляй файлы вручную.')
    return 0 if record['state'] in ('INSTALLED_FOR_TEST', 'ROLLED_BACK') and not record.get('error') else 3


if __name__ == '__main__':
    try:
        sys.exit(main())
    except (OSError, ValueError, KeyError, subprocess.SubprocessError, zipfile.BadZipFile) as error:
        print('Остановка: '+str(error)+'\nНе удаляй плагины вручную. Пришли это сообщение.', file=sys.stderr)
        sys.exit(2)
