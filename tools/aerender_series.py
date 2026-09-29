#!/usr/bin/env python3
"""Serial target render baseline: mapped-file identity, no stack sampler or cache purge."""
import argparse
import json
import re
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


def schedule(paired):
    modes = ('OFF', 'ON') if paired else ('OFF',)
    return [(mode, 'warmup') for mode in modes] + [
        (mode, 'measured') for _ in range(5) for mode in modes]


def progress_record(text, frame_count):
    """Retain coarse host reports, not precision per-frame latency claims."""
    frames = [(int(n), int(seconds)) for n, seconds in re.findall(
        r'^PROGRESS:\s+\d+:\d+:\d+:\d+ \((\d+)\): (\d+) Seconds\s*$', text, re.M)]
    totals = re.findall(r'^PROGRESS:\s+Total Time Elapsed: (\d+) Seconds\s*$', text, re.M)
    if sorted(n for n, _ in frames) != list(range(1, frame_count + 1)) or len(totals) != 1:
        raise ValueError('incomplete or duplicate host timing records')
    return dict(frame_reports=[dict(frame=n, reported_seconds=s) for n, s in frames],
                total_reported_seconds=int(totals[0]), resolution='whole seconds; host-reported, not precise latency')


def measure(root, fixture, args, index, manifest, mfr):
    run=root/('run-'+str(index));run.mkdir()
    output=run/'output';output.mkdir()
    target_output=output/fixture['output_pattern']
    cmd=[str(args.aerender),'-project',str(fixture['project_path']),'-rqindex',str(fixture['rqindex']),
         '-output',str(target_output),'-mfr',mfr,'100','-v','ERRORS_AND_PROGRESS']
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
    record=dict(returncode=process.returncode,wall_seconds=elapsed,target_pid=target,mfr_requested=mfr,
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
    record.update(output_files=len(files),output_digest=digest,
                  host_progress=progress_record((run/'stdout.log').read_text(),len(files)))
    bi.dump(run/'observation.json',record)
    return record


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    for name in ('workspace','aerender','render-executable','bundle','package','manifest'):
        parser.add_argument('--'+name,type=Path,required=True)
    parser.add_argument('--paired-mfr',action='store_true',help='Warm up both modes, then five serial OFF/ON pairs')
    args=parser.parse_args()
    args.workspace=args.workspace.resolve()
    fixture=ab.load_fixture(args.workspace,args.workspace/'fixture.json')
    manifest=json.loads(args.manifest.read_text())
    root=args.workspace/('series-'+uuid.uuid4().hex);root.mkdir()
    report=dict(status='INCOMPLETE',scope='pinned animated Final fixture; process-start runs; cache state uncontrolled; not RAM Preview',
                fixture={k:v for k,v in fixture.items() if k!='project_path'},candidate=manifest['build'],
                runner_sha256=bi.digest(Path(__file__).read_bytes()),mfr='OFF/ON' if args.paired_mfr else 'OFF',samples=[])
    try:
        for index,(mode,kind) in enumerate(schedule(args.paired_mfr)):
            record=measure(root,fixture,args,index,manifest,mode)
            record['kind']=kind
            report['samples'].append(record)
            bi.dump(root/'series.json',report)
            print(json.dumps(dict(run=index,**{k:v for k,v in record.items() if k!='host_progress'})),flush=True)
        measured=[r for r in report['samples'] if r['kind']=='measured']
        identical=len({r['output_digest'] for r in report['samples']})==1
        statistics_by_mode={}
        for mode in sorted({r['mfr_requested'] for r in measured}):
            group=[r for r in measured if r['mfr_requested']==mode]
            times=[r['wall_seconds'] for r in group]
            statistics_by_mode[mode]=dict(count=len(times),median_seconds=statistics.median(times),
                p95_seconds=ab.percentile(times,0.95),min_seconds=min(times),max_seconds=max(times),
                sampled_peak_rss_bytes=max(r['sampled_peak_rss_bytes'] for r in group))
        report.update(status='BASELINE_MEASURED' if identical else 'OUTPUT_MISMATCH',
                      byte_identical_outputs=identical,statistics_by_mode=statistics_by_mode,
                      limitations=['startup and PNG encoding included','polling overhead included','not cold-cache proof',
                                   'mapped path plus exact disk payload; runtime UUID established in separate preflight',
                                   'output equality is repeatability, not HDR fidelity',
                                   'MFR requested by CLI; actual concurrent frame execution not instrumented',
                                   'RSS covers observed aerendercore only, excludes child processes'])
    finally:
        bi.dump(root/'series.json',report)
        print('Report: '+str(root/'series.json'),flush=True)
    if report['status'] != 'BASELINE_MEASURED':
        raise SystemExit(1)

if __name__=='__main__':main()
