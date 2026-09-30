#!/usr/bin/env python3
"""Stamp/verify Finder version from validated build metadata (pre-sign only)."""
import argparse
import os
from pathlib import Path
import plistlib
import re
import stat
import uuid
import build_identity as bi


def version(bundle: Path) -> str:
    value = bi.payload_metadata(bundle)['version']
    if not re.fullmatch(r'(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)', value):
        raise ValueError('Bundle requires a three-part numeric version')
    return value


def verify(bundle: Path) -> str:
    expected = version(bundle)
    info = plistlib.loads(bi.safe_file(bundle, 'Contents/Info.plist').read_bytes())
    for key in ('CFBundleShortVersionString', 'CFBundleVersion'):
        if info.get(key) != expected:
            raise ValueError(f'{key} does not match BuildIdentity version')
    return expected


def stamp(bundle: Path) -> str:
    if (bundle / 'Contents/_CodeSignature').exists():
        raise ValueError('Refuse to mutate an already signed bundle')
    expected = version(bundle)
    path = bi.safe_file(bundle, 'Contents/Info.plist')
    info = plistlib.loads(path.read_bytes())
    info.update(CFBundleShortVersionString=expected, CFBundleVersion=expected)
    temp = path.with_name('.version-' + uuid.uuid4().hex)
    try:
        with temp.open('xb') as handle:
            handle.write(plistlib.dumps(info, sort_keys=False))
            handle.flush()
            os.fsync(handle.fileno())
        temp.chmod(stat.S_IMODE(path.stat().st_mode))
        os.replace(temp, path)
    finally:
        temp.unlink(missing_ok=True)
    return verify(bundle)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action', choices=('stamp', 'verify'))
    parser.add_argument('bundle', type=Path)
    args = parser.parse_args()
    print('Bundle version: ' + (stamp if args.action == 'stamp' else verify)(args.bundle))
