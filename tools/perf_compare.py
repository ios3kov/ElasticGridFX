#!/usr/bin/env python3
"""Compare controlled aerender reports with output-integrity and timing gates."""
from __future__ import annotations
import argparse
import json
from pathlib import Path
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
                              'fps','frame_start','frame_end','bit_depth','quality','output_format','output_pattern')} | {
        'geometry': f.get('geometry','grid'),
        'color_management': f.get('color_management','legacy-unspecified'),
    }


def compare(a:dict,b:dict)->dict:
    if signature(a)!=signature(b) or a['settings']['mfr']!=b['settings']['mfr']:
        raise ValueError('reports are not comparable: fixture/MFR settings differ')
    if a.get('test_case_id') != b.get('test_case_id'):
        raise ValueError('reports are not comparable: Test Case IDs differ')
    ma=a['summary']['wall_median_seconds']; mb=b['summary']['wall_median_seconds']
    p95a=a['summary']['wall_p95_seconds']; p95b=b['summary']['wall_p95_seconds']
    if ma<=0 or p95a<=0:
        raise ValueError('invalid baseline timing')
    digest_a=a['summary'].get('output_digest')
    digest_b=b['summary'].get('output_digest')
    if not digest_a or not digest_b:
        raise ValueError('reports lack stable encoded-output digest evidence')
    exact_output = digest_a == digest_b
    delta=(mb-ma)/ma*100.0
    delta95=(p95b-p95a)/p95a*100.0
    return dict(schema=1,status='COMPARED' if exact_output else 'FAIL',release='BLOCKED',
                test_case_id=a.get('test_case_id'),
                fixture=signature(a),mfr=a['settings']['mfr'],
                a=dict(build_id=a['candidate']['build_id'],median=ma,p95=p95a,
                       peak_rss=a['summary']['peak_rss_max_bytes'],output_digest=digest_a),
                b=dict(build_id=b['candidate']['build_id'],median=mb,p95=p95b,
                       peak_rss=b['summary']['peak_rss_max_bytes'],output_digest=digest_b),
                delta_percent=dict(median=delta,p95=delta95),
                meaningful_timing_change=abs(delta)>5.0,
                investigation_gate=abs(delta)>5.0,
                slowdown_investigation_gate=delta>5.0,
                encoded_output_exact_match=exact_output,
                encoded_output_equivalence='EXACT_MATCH' if exact_output else 'FAIL_MISMATCH',
                quality_equivalence='NOT RUN',
                quality_scope='Encoded timing outputs only; not a 32f/HDR or universal pixel-quality oracle.',
                performance_claim_allowed=False,
                note='Timing comparison alone never approves an optimization; all applicable quality/runtime gates must also pass.')


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
    return 0 if result['status']=='COMPARED' else 2

if __name__=='__main__':sys.exit(main())
