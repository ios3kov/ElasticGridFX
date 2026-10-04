#!/usr/bin/env python3
"""Package clean target-AE runner with exact immutable fd69988 payload for verification only."""
import argparse, json
from pathlib import Path
import zipfile
import build_identity as bi

ROOT=Path(__file__).resolve().parents[1]
FILES=('RUN_ELASTICGRID_AE_TEST_MAC.command',
       'tools/target_ae_acceptance.py','tools/ae_smoke_runner.py','tools/windows_ae_validation.py','tools/live_identity.py',
       'tools/smoke_pixels.py','tools/build_identity.py','tools/install_candidate.py',
       'tests/ae_runtime_smoke.jsx','tests/ae_runtime_smoke_phased.jsx','diagnostics/candidate-fd69988.json',
       'diagnostics/TARGET_AE_ACCEPTANCE_README_RU.txt')
CANDIDATE_SHA='68c135f1a2a9390a0032a31d9073c8111bff6dd43691b9222ec50fad3913982e'

def archive_mode(name:str, files:dict)->int:
    if name=='AcceptanceToolIdentity.json':
        return 0o100644
    if name not in files:
        raise ValueError('acceptance member missing from identity: '+name)
    return 0o100755 if files[name]['executable'] else 0o100644

def write_archive(output:Path, prefix:str, content:dict, files:dict):
    with zipfile.ZipFile(output,'x',compression=zipfile.ZIP_DEFLATED) as z:
        for name,data in content.items():
            info=zipfile.ZipInfo(prefix+name,date_time=(2026,9,29,0,0,0))
            info.create_system=3; info.compress_type=zipfile.ZIP_DEFLATED
            info.external_attr=archive_mode(name,files)<<16
            z.writestr(info,data)
    with zipfile.ZipFile(output) as z:
        if set(z.namelist())!={prefix+n for n in content}: raise ValueError('unexpected members')
        for name,data in content.items():
            member=prefix+name
            if z.read(member)!=data: raise ValueError('package byte mismatch')
            actual=bool((z.getinfo(member).external_attr>>16)&0o111)
            expected=bool(archive_mode(name,files)&0o111)
            if actual!=expected: raise ValueError('package execute permission mismatch: '+name)

def package(candidate:Path, output:Path):
    src=bi.source_record(ROOT)
    if src['source_state']!='clean':
        raise ValueError('refuse dirty acceptance sources')
    payload=candidate.read_bytes()
    if bi.digest(payload)!=CANDIDATE_SHA:
        raise ValueError('wrong immutable fd69988 candidate')
    files=bi.hashes(ROOT,FILES)
    files['payload/ElasticGrid.plugin.zip']=dict(sha256=CANDIDATE_SHA,executable=False)
    identity=dict(schema=1,artifact_type='target AE functional acceptance runner',
                  source_state='clean',commit=src['commit'],source_sha256=src['source_sha256'],
                  files=files,candidate_commit='fd69988c10b25268eb8cad6ee6ced7f6a28bee9d',
                  release_status='BLOCKED')
    content={name:(ROOT/name).read_bytes() for name in FILES}
    content['payload/ElasticGrid.plugin.zip']=payload
    content['AcceptanceToolIdentity.json']=json.dumps(identity,indent=2,sort_keys=True).encode()+b'\n'
    prefix='ElasticGridFX-AE-Acceptance-fd69988/'
    write_archive(output,prefix,content,files)
    return dict(commit=src['commit'],package_sha256=bi.digest(output.read_bytes()),
                candidate_package_sha256=CANDIDATE_SHA,runtime='NOT RUN')

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('candidate',type=Path);p.add_argument('output',type=Path)
    a=p.parse_args();print(json.dumps(package(a.candidate,a.output),indent=2))
