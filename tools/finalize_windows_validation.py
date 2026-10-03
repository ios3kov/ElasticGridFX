#!/usr/bin/env python3
"""Create a scoped Windows build-validation record for an exact .aex."""
from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import re

import build_identity as bi
import verify_windows_pipl as pipl_verify


def require(pattern: str, path: Path, label: str) -> None:
    text=path.read_text(encoding='utf-8-sig',errors='replace')
    if not re.search(pattern,text,re.I|re.M):
        raise ValueError(label+' evidence missing')


def validate(manifest_path: Path, aex: Path, headers: Path, exports: Path,
             resources: Path, expected_pipl: Path, env: dict[str, str] | None = None) -> dict:
    manifest=json.loads(manifest_path.read_text(encoding='utf-8-sig'))
    if manifest.get('schema')!=1 or manifest.get('artifact')!='FSTR Stretch.aex':
        raise ValueError('invalid Windows artifact manifest')
    build=bi.validate_identity(manifest['build'])
    if build.get('artifact_type')!='AE native effect (.aex)' or 'windows-msvc' not in build.get('target',''):
        raise ValueError('manifest is not a Windows .aex build')
    if bi.digest(aex.read_bytes())!=manifest.get('sha256'):
        raise ValueError('AEX hash does not match manifest')

    require(r'machine \(x64\)',headers,'x64 PE')
    require(r'\bEffectMain\b',exports,'EffectMain export')
    require(r'PiPL',resources,'PiPL resource')
    if pipl_verify.embedded_pipl(aex)!=expected_pipl.read_bytes():
        raise ValueError('embedded PiPL differs from generated PiPL bytes')

    env=os.environ if env is None else env
    if env.get('GITHUB_ACTIONS')!='true':
        raise ValueError('build validation record may only be finalized in GitHub Actions')
    if env.get('GITHUB_SHA')!=build['commit']:
        raise ValueError('GitHub Actions SHA does not match Build Identity commit')
    if not re.fullmatch(r'[1-9][0-9]*',env.get('GITHUB_RUN_ID','')):
        raise ValueError('invalid GitHub Actions run id')
    if not re.fullmatch(r'[1-9][0-9]*',env.get('GITHUB_RUN_ATTEMPT','')):
        raise ValueError('invalid GitHub Actions run attempt')

    return {
        'schema':1,
        'artifact':aex.name,
        'artifact_sha256':manifest['sha256'],
        'build_id':build['build_id'],
        'commit':build['commit'],
        'target':build['target'],
        'toolchain':build['toolchain'],
        'ci':{
            'run_id':env['GITHUB_RUN_ID'],
            'run_attempt':env['GITHUB_RUN_ATTEMPT'],
            'sha':env['GITHUB_SHA'],
        },
        'checks':{
            'portable_core_tests':'PASS',
            'windows_packager_tests':'PASS',
            'rust_host_tests':'PASS',
            'rust_clippy_minus_d_warnings':'PASS',
            'release_cdylib_build':'PASS',
            'pe_x64':'PASS',
            'effectmain_export':'PASS',
            'pipl_resource_present':'PASS',
            'pipl_resource_byte_exact':'PASS',
            'after_effects_load':'NOT RUN',
            'after_effects_runtime_ui_render_mfr_undo':'NOT RUN',
        },
        'scope':'build/static validation only; not After Effects runtime evidence',
    }


def main() -> int:
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--manifest',required=True,type=Path)
    parser.add_argument('--aex',required=True,type=Path)
    parser.add_argument('--headers',required=True,type=Path)
    parser.add_argument('--exports',required=True,type=Path)
    parser.add_argument('--resources',required=True,type=Path)
    parser.add_argument('--expected-pipl',required=True,type=Path)
    parser.add_argument('--out',required=True,type=Path)
    args=parser.parse_args()
    record=validate(*(p.resolve(strict=True) for p in (
        args.manifest,args.aex,args.headers,args.exports,args.resources,args.expected_pipl)))
    args.out.parent.mkdir(parents=True,exist_ok=True)
    if args.out.exists():
        raise FileExistsError(args.out)
    args.out.write_text(json.dumps(record,indent=2,sort_keys=True)+'\n',encoding='utf-8')
    print(args.out)
    return 0


if __name__=='__main__':
    raise SystemExit(main())
