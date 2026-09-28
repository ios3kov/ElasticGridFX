#!/usr/bin/env python3
"""Package ONLY the clean, versioned diagnostic; never include a plugin binary."""
import argparse
import json
from pathlib import Path
import zipfile

import build_identity as bi

ROOT=Path(__file__).resolve().parents[1]
FILES=('CHECK_ELASTICGRID_MAC.command', 'tools/live_identity.py',
       'tools/build_identity.py', 'tools/install_candidate.py',
       'diagnostics/candidate-6d3b846.json', 'diagnostics/README_RU.txt')


def package(output: Path) -> dict:
    record=bi.source_record(ROOT)
    if record['source_state']!='clean':
        raise ValueError('Only clean committed diagnostics may be shared')
    meta=dict(schema=1, type='non-installing Mac diagnostic', version='1.0',
              commit=record['commit'], source_state='clean',
              source_sha256=record['source_sha256'], files=bi.hashes(ROOT,FILES),
              candidate_commit='6d3b846463410f37198fda4b625e56e4cea44c22')
    prefix='ElasticGridFX-Diagnostics/'
    with zipfile.ZipFile(output,'x',compression=zipfile.ZIP_DEFLATED) as archive:
        for name in (*FILES,'diagnostics/DiagnosticIdentity.json'):
            data=json.dumps(meta,sort_keys=True,indent=2).encode()+b'\n' if name.endswith('DiagnosticIdentity.json') else (ROOT/name).read_bytes()
            item=zipfile.ZipInfo(prefix+name, date_time=(2026,9,28,0,0,0))
            item.create_system=3
            item.compress_type=zipfile.ZIP_DEFLATED
            item.external_attr=(0o100755 if name.endswith('.command') else 0o100644)<<16
            archive.writestr(item,data)
    with zipfile.ZipFile(output) as archive:
        if set(archive.namelist())!={prefix+n for n in (*FILES,'diagnostics/DiagnosticIdentity.json')}:
            raise ValueError('Unexpected diagnostic package contents')
        for name in FILES:
            if bi.digest(archive.read(prefix+name))!=meta['files'][name]['sha256']:
                raise ValueError('Diagnostic byte mismatch')
    return dict(commit=record['commit'],package_sha256=bi.digest(output.read_bytes()),path=str(output),
                scope='diagnostic only; no plugin; AE runtime result NOT RUN')


if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('output',type=Path)
    args=parser.parse_args()
    print(json.dumps(package(args.output),indent=2))
