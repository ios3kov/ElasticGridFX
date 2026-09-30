#!/usr/bin/env python3
"""Create the user-facing FSTR Stretch bundle without changing signed payload bytes.

The legacy build archive remains an internal input, never an additional install.
Output is create-only. This tool neither installs nor claims AE verification.
"""
from __future__ import annotations

import argparse
import json
from pathlib import Path
import plistlib
import shutil
import stat
import sys
import zipfile

import build_identity as bi
from install_candidate import checked_path

BUNDLE_NAME = 'FSTR Stretch.plugin'
PACKAGE_NAME = BUNDLE_NAME + '.zip'
MANIFEST_NAME = 'FSTR Stretch.artifact.json'
ROOT = Path(__file__).resolve().parents[1]


def package(bundle: Path, archive: Path, manifest: Path, output: Path) -> dict:
    """Verify the internal package, copy unchanged files, then seal the new name."""
    bundle = checked_path(bundle, directory=True)
    archive, manifest, output = (checked_path(p) for p in (archive, manifest, output))
    checked_path(output.parent, directory=True)
    if bundle.name != 'ElasticGrid.plugin' or output == bundle or bundle in output.parents:
        raise ValueError('Unexpected input bundle or nested output directory')
    bi.verify(bundle, archive, manifest)
    before = bi.payload_files(bundle)
    entries = list(bundle.rglob('*'))
    if len(entries) > 128 or any(not (stat.S_ISDIR(p.lstat().st_mode) or
                                    stat.S_ISREG(p.lstat().st_mode)) for p in entries):
        raise ValueError('Unsupported payload entry')
    if sum(bi.safe_file(bundle, n).stat().st_size for n in before) > 32 * 1024 * 1024:
        raise ValueError('Payload exceeds packaging limit')
    plist = plistlib.loads(bi.safe_file(bundle, 'Contents/Info.plist').read_bytes())
    if (plist.get('CFBundleIdentifier') != 'com.elasticgrid.fx' or
            plist.get('CFBundleExecutable') != 'ElasticGrid'):
        raise ValueError('Unexpected plugin identity or executable')

    # Never reuse/delete an old output or rewrite the input archive/manifest.
    output.mkdir(mode=0o700)
    branded = output / BUNDLE_NAME
    for name in before:
        target = branded / name
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(bi.safe_file(bundle, name), target)
    if bi.payload_files(branded) != before:
        raise ValueError('Copy changed payload bytes or execute permissions')
    bi.verify(bundle, archive, manifest)  # Also detect changes to the input during packaging.
    destination, record_path = output / PACKAGE_NAME, output / MANIFEST_NAME
    bi.seal(branded, destination, record_path)
    bi.verify(branded, destination, record_path)
    record = json.loads(record_path.read_text())
    record.update(bundle_name=BUNDLE_NAME, package_name=PACKAGE_NAME,
                  input_package_sha256=bi.digest(archive.read_bytes()),
                  payload_unchanged=True, signature_verification='NOT RUN')
    bi.dump(record_path, record)
    bi.verify(branded, destination, record_path)
    return record


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ('bundle', 'package', 'manifest', 'output'):
        parser.add_argument('--' + name, required=True, type=Path)
    args = parser.parse_args()
    source = bi.source_record(ROOT)
    if source['source_state'] != 'clean':
        raise ValueError('Packaging requires clean committed tooling')
    result = package(args.bundle, args.package, args.manifest, args.output)
    result['packaging_commit'] = source['commit']
    result['packaging_source_sha256'] = source['source_sha256']
    bi.dump(args.output / MANIFEST_NAME, result)
    print(json.dumps(result, indent=2, sort_keys=True))


if __name__ == '__main__':
    try:
        main()
    except (OSError, ValueError, KeyError, zipfile.BadZipFile) as error:
        print('Packaging stopped: ' + str(error), file=sys.stderr)
        sys.exit(1)
