#!/usr/bin/env python3
"""Instrumented render-process identity preflight; timings are NOT benchmarks."""
import argparse
import json
import re
from pathlib import Path
import subprocess
import time
import uuid

import aerender_benchmark as ab
import build_identity as bi
import live_identity as li

def render_children(parent, executable):
    text=subprocess.check_output(['/bin/ps','-axww','-o','pid=,ppid=,comm='],text=True)
    rows=[line.strip().split(None,2) for line in text.splitlines() if line.strip()]
    descendants={parent}
    for _ in range(8):
        expanded=descendants | {int(pid) for pid,ppid,path in rows if int(ppid) in descendants}
        if expanded==descendants: break
        descendants=expanded
    return [dict(pid=int(pid),executable=path) for pid,ppid,path in rows
            if int(pid)!=parent and int(pid) in descendants and path==str(executable)]

def detached_renderers(parent, executable, output_path):
    text=subprocess.check_output(['/bin/ps','-axww','-o','pid=,comm='],text=True)
    found=[]
    for line in text.splitlines():
        pair=line.strip().split(None,1)
        if len(pair)!=2 or pair[1]!=str(executable): continue
        pid=int(pair[0])
        args=subprocess.run(['/bin/ps','-ww','-p',str(pid),'-o','args='],capture_output=True,text=True)
        if (args.returncode==0 and re.search(r' -aerenderpid '+str(parent)+r'(?:\s|$)',args.stdout)
                and ' -output '+str(output_path)+' -mfr ' in args.stdout):
            found.append(dict(pid=pid,executable=pair[1]))
    return found


def probe(workspace, aerender, bundle, package, manifest_path, render_executable):
    fixture = ab.load_fixture(workspace, workspace/'fixture.json')
    build = ab.verify_candidate(bundle, package, manifest_path)
    manifest = json.loads(manifest_path.read_text())
    run = workspace/('identity-probe-'+uuid.uuid4().hex)
    run.mkdir(mode=0o700)
    output = run/'output'; output.mkdir()
    record = dict(status='BLOCKED', scope='instrumented preflight; not timing baseline',
                  build=build, observations=[], sampled_peak_rss_bytes=0)
    cmd = [str(aerender), '-project', str(fixture['project_path']), '-rqindex','1',
           '-output', str(output/fixture['output_pattern']), '-mfr','OFF','100',
           '-v','ERRORS_AND_PROGRESS']
    start = time.monotonic()
    with (run/'stdout.log').open('x') as stdout, (run/'stderr.log').open('x') as stderr:
        process = subprocess.Popen(cmd, stdout=stdout, stderr=stderr)
        identity = None; target = None; attempt = 0; last_capture = 0
        while process.poll() is None and time.monotonic()-start < 180:
            hosts = detached_renderers(process.pid,render_executable,output/fixture['output_pattern'])
            if len(hosts) == 1:
                host = hosts[0]
                if target is not None and target != host['pid']:
                    record['reason']='render process changed'; break
                target = host['pid']
                rss = subprocess.run(['/bin/ps','-p',str(target),'-o','rss='],capture_output=True,text=True)
                if rss.returncode == 0 and rss.stdout.strip().isdigit():
                    record['sampled_peak_rss_bytes']=max(record['sampled_peak_rss_bytes'],int(rss.stdout)*1024)
                if identity is None and time.monotonic()-last_capture > 2:
                    last_capture=time.monotonic(); attempt+=1
                    capture=run/('capture-'+str(attempt)); capture.mkdir()
                    try:
                        images, observation=li.capture(target,Path(host['executable']),capture)
                        image=li.select_image(images,bundle/'Contents/MacOS/ElasticGrid',
                            li.macho_uuids((bundle/'Contents/MacOS/ElasticGrid').read_bytes()))
                        li.verify_disk(bundle,manifest)
                        identity=dict(pid=target,uuid=image['uuid'],observation=observation)
                    except (ValueError,OSError,subprocess.SubprocessError) as error:
                        record['observations'].append(type(error).__name__)
            elif len(hosts)>1:
                record['reason']='ambiguous new AE process'; break
            time.sleep(0.25)
        record['returncode']=process.poll()
        record['elapsed_instrumented_seconds']=time.monotonic()-start
    if record['returncode']==0 and identity is not None:
        ab.verify_candidate(bundle,package,manifest_path)
        files,digest=ab.output_manifest(output)
        record.update(identity=identity,output_files=len(files),output_digest=digest)
        if len(files)==fixture['frame_end']-fixture['frame_start']+1:
            record['status']='IDENTITY_AND_FRAME_COUNT_PASS'
    if record['returncode'] is None:
        record['reason']='probe stopped; render processes were not killed'
    bi.dump(run/'probe.json',record)
    print(json.dumps(dict(report=str(run/'probe.json'),**record),indent=2))


if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    for name in ('workspace','aerender','bundle','package','manifest','render-executable'):
        parser.add_argument('--'+name,type=Path,required=True)
    args=parser.parse_args()
    probe(args.workspace.resolve(),args.aerender,args.bundle,args.package,args.manifest,args.render_executable)
