#!/usr/bin/env python3
"""Package the unchanged pinned candidate plus clean, tested update/rollback tools."""
import argparse
import json
from pathlib import Path
import zipfile
import build_identity as bi
from authorized_update import PACKAGE_SHA, CANDIDATE_MANIFEST, PREVIOUS_MANIFEST

ROOT=Path(__file__).resolve().parents[1]
FILES=('UPDATE_ELASTICGRID_MAC.command','ROLLBACK_ELASTICGRID_MAC.command',
       'tools/authorized_update.py','tools/xattr_reader.py','tools/build_identity.py','tools/install_candidate.py',
       'diagnostics/candidate-6d3b846.json','diagnostics/candidate-fd69988.json',
       'diagnostics/UPDATE_FD69988_README_RU.txt')


def package(candidate: Path, output: Path) -> dict:
    source=bi.source_record(ROOT)
    if source['source_state']!='clean':
        raise ValueError('Refuse to distribute dirty installer sources')
    payload=candidate.read_bytes()
    if bi.digest(payload)!=PACKAGE_SHA:
        raise ValueError('Wrong pinned candidate package')
    files=bi.hashes(ROOT,FILES)
    files['payload/ElasticGrid.plugin.zip']=dict(sha256=PACKAGE_SHA,executable=False)
    identity=dict(schema=1, artifact_type='authorized single-copy test update/rollback',
                  source_state='clean',commit=source['commit'],source_sha256=source['source_sha256'],
                  files=files,candidate_commit='fd69988c10b25268eb8cad6ee6ced7f6a28bee9d',
                  previous_commit='6d3b846463410f37198fda4b625e56e4cea44c22')
    content={name:(ROOT/name).read_bytes() for name in FILES}
    content['payload/ElasticGrid.plugin.zip']=payload
    content['InstallToolIdentity.json']=json.dumps(identity,indent=2,sort_keys=True).encode()+b'\n'
    prefix='ElasticGridFX-Test-Update-fd69988/'
    with zipfile.ZipFile(output,'x',compression=zipfile.ZIP_DEFLATED) as z:
        for name,data in content.items():
            info=zipfile.ZipInfo(prefix+name,date_time=(2026,9,28,0,0,0))
            info.create_system=3; info.compress_type=zipfile.ZIP_DEFLATED
            info.external_attr=(0o100755 if name.endswith('.command') else 0o100644)<<16
            z.writestr(info,data)
    with zipfile.ZipFile(output) as z:
        if set(z.namelist())!={prefix+name for name in content}:
            raise ValueError('Unexpected output members')
        for name,data in content.items():
            if z.read(prefix+name)!=data:
                raise ValueError('Output bytes mismatch')
    return dict(commit=source['commit'], package_sha256=bi.digest(output.read_bytes()),
                candidate_package_sha256=PACKAGE_SHA, scope='authorized test replacement only; actual user execution NOT RUN')


if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('candidate',type=Path); parser.add_argument('output',type=Path)
    args=parser.parse_args()
    print(json.dumps(package(args.candidate,args.output),indent=2))
