#!/usr/bin/env python3
"""Compare two controlled aerender benchmark reports without inventing a winner."""
from __future__ import annotations
import argparse
import json
from pathlib import Path
import statistics
import sys

MIN_SAMPLES=5


def load(path:Path)->dict:
    data=json.loads(path.read_text())
    if data.get('schema')!=1 or data.get('status')!='MEASURED':
        raise ValueError('benchmark report is not a completed measurement')
    if len(data.get('samples_data',[]))<MIN_SAMPLES:
        raise ValueError('benchmark report has too few samples')
    return data


def signature(report:dict)->dict:
    f=report['fixture']
    return {k:f[k] for k in ('fixture_id','project_sha256','composition','rqindex','width','height',
                              'fps','frame_start','frame_end','bit_depth','quality','output_format','output_pattern')}


def compare(a:dict,b:dict)->dict:
    if signature(a)!=signature(b) or a['settings']['mfr']!=b['settings']['mfr']:
        raise ValueError('reports are not comparable: fixture/MFR settings differ')
    ma=a['summary']['wall_median_seconds']; mb=b['summary']['wall_median_seconds']
    p95a=a['summary']['wall_p95_seconds']; p95b=b['summary']['wall_p95_seconds']
    if ma<=0 or p95a<=0: raise ValueError('invalid baseline timing')
    delta=(mb-ma)/ma*100.0
    delta95=(p95b-p95a)/p95a*100.0
    return dict(schema=1,status='COMPARED',release='BLOCKED',
                fixture=signature(a),mfr=a['settings']['mfr'],
                a=dict(build_id=a['candidate']['build_id'],median=ma,p95=p95a,
                       peak_rss=a['summary']['peak_rss_max_bytes']),
                b=dict(build_id=b['candidate']['build_id'],median=mb,p95=p95b,
                       peak_rss=b['summary']['peak_rss_max_bytes']),
                delta_percent=dict(median=delta,p95=delta95),
                investigation_gate=abs(delta)>5.0,
                quality_equivalence='NOT RUN',
                note='Timing comparison only. No optimization approval until pixel/quality equivalence is proven separately.')


def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('a',type=Path);p.add_argument('b',type=Path);p.add_argument('--out',type=Path)
    args=p.parse_args()
    try: result=compare(load(args.a),load(args.b))
    except (OSError,ValueError,KeyError,json.JSONDecodeError) as e:
        print('BLOCKED: '+str(e),file=sys.stderr);return 3
    text=json.dumps(result,indent=2,sort_keys=True)+'\n'
    if args.out:
        if args.out.exists(): print('BLOCKED: refusing stale output',file=sys.stderr);return 3
        args.out.write_text(text)
    else: print(text,end='')
    return 0

if __name__=='__main__':sys.exit(main())
