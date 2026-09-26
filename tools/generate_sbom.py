#!/usr/bin/env python3
from __future__ import annotations
import argparse, datetime as dt, json
from pathlib import Path

def purl(pkg): return f"pkg:cargo/{pkg['name']}@{pkg['version']}"
def licenses(expr): return [{"expression": expr}] if expr else [{"license":{"name":"NOASSERTION"}}]
def component(pkg):
    c={"type":"library","bom-ref":pkg["id"],"name":pkg["name"],"version":pkg["version"],"purl":purl(pkg),"licenses":licenses(pkg.get("license"))}
    refs=[]
    if pkg.get("repository"): refs.append({"type":"vcs","url":pkg["repository"]})
    if pkg.get("homepage"): refs.append({"type":"website","url":pkg["homepage"]})
    if refs: c["externalReferences"]=refs
    return c

def main():
    ap=argparse.ArgumentParser(); ap.add_argument('metadata',type=Path); ap.add_argument('output_dir',type=Path); a=ap.parse_args()
    meta=json.loads(a.metadata.read_text()); pkgs=sorted(meta.get('packages',[]),key=lambda p:(p['name'],p['version'],p['id'])); by={p['id']:p for p in pkgs}
    resolve=meta.get('resolve') or {}; root=resolve.get('root')
    deps=[]
    for n in resolve.get('nodes',[]):
        refs=sorted({d.get('pkg') for d in n.get('deps',[]) if d.get('pkg')})
        deps.append({"ref":n['id'],"dependsOn":refs})
    deps.sort(key=lambda x:x['ref'])
    md={"timestamp":dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat().replace('+00:00','Z'),"tools":{"components":[{"type":"application","name":"ElasticGrid SBOM generator","version":"1.0"}]}}
    if root in by:
        rc=component(by[root]); rc['type']='application'; md['component']=rc
    bom={"bomFormat":"CycloneDX","specVersion":"1.5","serialNumber":"urn:uuid:00000000-0000-0000-0000-000000000000","version":1,"metadata":md,"components":[component(p) for p in pkgs if p['id']!=root],"dependencies":deps}
    a.output_dir.mkdir(parents=True,exist_ok=True)
    (a.output_dir/'sbom.cdx.json').write_text(json.dumps(bom,indent=2,sort_keys=True)+'\n')
    lines=['name\tversion\tlicense\tsource\trepository']
    for p in pkgs: lines.append('\t'.join([p['name'],p['version'],p.get('license') or 'NOASSERTION',p.get('source') or 'path/local',p.get('repository') or '']))
    (a.output_dir/'dependencies.tsv').write_text('\n'.join(lines)+'\n')
    groups={}
    for p in pkgs: groups.setdefault(p.get('license') or 'NOASSERTION',[]).append(f"{p['name']} {p['version']}")
    with (a.output_dir/'licenses-summary.txt').open('w') as f:
        for k in sorted(groups):
            f.write(f'[{k}]\n'); [f.write(f'  {x}\n') for x in sorted(groups[k])]; f.write('\n')
    print(f"SBOM: {a.output_dir/'sbom.cdx.json'}"); print(f"Dependencies: {len(pkgs)} resolved Cargo packages")
if __name__=='__main__': main()
