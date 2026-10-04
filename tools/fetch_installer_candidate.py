#!/usr/bin/env python3
"""CI-only download of the exact Windows Dev56 candidate accepted by the user.

No arbitrary URL/repository/payload selection; never log the token or signed URL.
"""
import argparse
import hashlib
import io
import json
import os
from pathlib import Path
import time
import urllib.error
import urllib.request
import zipfile
import generate_installer_payload as payload

ARTIFACT=11298456203
COMMIT='cbb7468b36e9b94b4c575b3de1c27863ebe8caf2'
BUILD='EGFX-376f97bdd493cd2fbce61edd'
SHA='669436b62a06baef0eba895e8e72b50f89e448ff396c24c43e16a2f93c15bd6c'
API='https://api.github.com/repos/ios3kov/ElasticGridFX/actions/artifacts/'+str(ARTIFACT)


class NoRedirect(urllib.request.HTTPRedirectHandler):
    def redirect_request(self,*args):return None


def fetch(out):
    token=os.environ.get('GITHUB_TOKEN','')
    if not token:raise ValueError('GitHub Actions read token required')
    headers={'Authorization':'Bearer '+token,'Accept':'application/vnd.github+json','X-GitHub-Api-Version':'2022-11-28'}
    with urllib.request.urlopen(urllib.request.Request(API,headers=headers),timeout=30) as r:m=json.load(r)
    if m['expired'] or m['workflow_run']['head_sha']!=COMMIT:raise ValueError('Wrong or expired candidate artifact')
    request=urllib.request.Request(API+'/zip?fresh='+str(time.time_ns()),headers=headers)
    try:
        with urllib.request.build_opener(NoRedirect).open(request,timeout=30) as r:data=r.read()
    except urllib.error.HTTPError as e:
        if e.code!=302:raise
        url=e.headers['Location']
        if not url.startswith('https://'):raise ValueError('Non-HTTPS artifact redirect')
        with urllib.request.urlopen(url,timeout=30) as r:data=r.read()
    if out.exists():raise FileExistsError(out)
    out.mkdir(parents=True)
    names=['dist/windows/FSTR Stretch.aex','dist/windows/FSTR Stretch.windows-artifact.json','dist/windows/FSTR Stretch.windows-build-validation.json']
    with zipfile.ZipFile(io.BytesIO(data)) as z:
        if z.testzip() is not None or any(z.namelist().count(n)!=1 for n in names):raise ValueError('Invalid artifact archive')
        for name in names:(out/Path(name).name).write_bytes(z.read(name))
    aex=out/'FSTR Stretch.aex';manifest=out/'FSTR Stretch.windows-artifact.json'
    b,_,_=payload.collect(aex,manifest,BUILD)
    validation=json.loads((out/'FSTR Stretch.windows-build-validation.json').read_text(encoding='utf-8-sig'))
    if b['commit']!=COMMIT or hashlib.sha256(aex.read_bytes()).hexdigest()!=SHA or validation['artifact_sha256']!=SHA:raise ValueError('Candidate identity mismatch')
    if any(v!='PASS' for k,v in validation['checks'].items() if not k.startswith('after_effects')):raise ValueError('Candidate build gate not passed')
    return {'build_id':BUILD,'commit':COMMIT,'sha256':SHA}


if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--out',type=Path,required=True);a=p.parse_args();print(json.dumps(fetch(a.out.resolve()),indent=2))
