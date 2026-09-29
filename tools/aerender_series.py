#!/usr/bin/env python3
"""Serial target render baseline: mapped-file identity, no stack sampler or cache purge."""
import argparse
import json
from pathlib import Path
import statistics
import struct
import subprocess
import time
import uuid

import aerender_benchmark as ab
import aerender_identity_probe as probe
import build_identity as bi
import live_identity as li


def mapped_candidate(text, pid, binary):
    if 'p'+str(pid) not in text.splitlines():
        raise ValueError('mapped files belong to another process')
    paths=[line[1:] for line in text.splitlines() if line.startswith('n')]
    candidates={p for p in paths if 'elasticgrid' in p.lower()}
    return len(candidates)==1 and li.path_text_consistent(next(iter(candidates)),str(binary))


def measure(root, fixture, args, index, manifest):
    run=root/('run-'+str(index));run.mkdir()
    output=run/'output';output.mkdir()
    target_output=output/fixture['output_pattern']
    cmd=[str(args.aerender),'-project',str(fixture['project_path']),'-rqindex',str(fixture['rqindex']),
         '-output',str(target_output),'-mfr','OFF','100','-v','ERRORS_AND_PROGRESS']
    ab.verify_candidate(args.bundle,args.package,args.manifest)
    start=time.monotonic();rss_peak=0;target=None;key=None;mapped=False;last_map=0;polls=0
    with (run/'stdout.log').open('x') as stdout,(run/'stderr.log').open('x') as stderr:
        process=subprocess.Popen(cmd,stdout=stdout,stderr=stderr)
        while process.poll() is None:
            if time.monotonic()-start>180:
                raise RuntimeError('render timeout; test processes retained, no automatic retry')
            hosts=probe.detached_renderers(process.pid,args.render_executable,target_output)
            if len(hosts)>1: raise RuntimeError('ambiguous render process')
            if hosts:
                pid=hosts[0]['pid'];current_key=li.process_key(pid)
                if target is not None and (pid!=target or current_key!=key):
                    raise RuntimeError('render process changed')
                target=pid;key=current_key
                result=subprocess.run(['/bin/ps','-p',str(pid),'-o','rss='],capture_output=True,text=True,timeout=5)
                if result.returncode==0 and result.stdout.strip().isdigit():rss_peak=max(rss_peak,int(result.stdout)*1024)
                if not mapped and time.monotonic()-last_map>2:
                    last_map=time.monotonic()
                    result=subprocess.run(['/usr/sbin/lsof','-a','-p',str(pid),'-d','txt','-Fpn'],capture_output=True,text=True,timeout=5)
                    if result.returncode==0:
                        mapped=mapped_candidate(result.stdout,pid,args.bundle/'Contents/MacOS/ElasticGrid')
                        if mapped: li.verify_disk(args.bundle,manifest)
                polls+=1
            time.sleep(0.5)
        elapsed=time.monotonic()-start
    record=dict(returncode=process.returncode,wall_seconds=elapsed,target_pid=target,
                mapped_candidate=mapped,sampled_peak_rss_bytes=rss_peak,polls=polls)
    bi.dump(run/'observation.json',record)
    if process.returncode!=0 or not mapped: raise ValueError('render exit or mapped identity failed; retained observation')
    ab.verify_candidate(args.bundle,args.package,args.manifest)
    files,digest=ab.output_manifest(output)
    if len(files)!=fixture['frame_end']-fixture['frame_start']+1: raise ValueError('incorrect frame count')
    for item in files:
        with (output/item['path']).open('rb') as handle: header=handle.read(24)
        if header[:8]!=b'\x89PNG\r\n\x1a\n' or struct.unpack('>II',header[16:24])!=(fixture['width'],fixture['height']):
            raise ValueError('invalid PNG geometry')
    record.update(output_files=len(files),output_digest=digest)
    bi.dump(run/'observation.json',record)
    return record


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    for name in ('workspace','aerender','render-executable','bundle','package','manifest'):
        parser.add_argument('--'+name,type=Path,required=True)
    args=parser.parse_args()
    args.workspace=args.workspace.resolve()
    fixture=ab.load_fixture(args.workspace,args.workspace/'fixture.json')
    manifest=json.loads(args.manifest.read_text())
    root=args.workspace/('series-'+uuid.uuid4().hex);root.mkdir()
    report=dict(status='INCOMPLETE',scope='1080p animated 32bpc Final; process-start runs; cache state uncontrolled; not RAM Preview',
                fixture={k:v for k,v in fixture.items() if k!='project_path'},candidate=manifest['build'],
                runner_sha256=bi.digest(Path(__file__).read_bytes()),mfr='OFF',samples=[])
    try:
        for index in range(6):
            record=measure(root,fixture,args,index,manifest)
            record['kind']='warmup' if index==0 else 'measured'
            report['samples'].append(record)
            bi.dump(root/'series.json',report)
            print(json.dumps(dict(run=index,**record)),flush=True)
        measured=report['samples'][1:];times=[r['wall_seconds'] for r in measured]
        identical=len({r['output_digest'] for r in report['samples']})==1
        report.update(status='BASELINE_MEASURED' if identical else 'OUTPUT_MISMATCH',
                      byte_identical_outputs=identical,median_seconds=statistics.median(times),
                      p95_seconds=ab.percentile(times,0.95),min_seconds=min(times),max_seconds=max(times),
                      sampled_peak_rss_bytes=max(r['sampled_peak_rss_bytes'] for r in measured),
                      limitations=['startup and PNG encoding included','polling overhead included','not cold-cache proof',
                                   'mapped path plus exact disk payload; runtime UUID established in separate preflight',
                                   'output equality is repeatability, not HDR fidelity'])
    finally:
        bi.dump(root/'series.json',report)
        print('Report: '+str(root/'series.json'),flush=True)

if __name__=='__main__':main()
