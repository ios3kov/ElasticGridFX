#!/usr/bin/env python3
"""Read bounded diagnostic callback CSV; never certify host identity or speed."""
from __future__ import annotations
import argparse
import csv
import io
import json
from pathlib import Path
import re
import statistics
from fractions import Fraction

HEADER=('sequence','selector','time_numerator','time_scale','width','height','depth','path',
        'has_output','completed','parameters_ns','input_ns','output_ns','sampling_ns','checkin_ns','total_ns','start_ns')
PHASES=HEADER[10:15]
MAX_RECORDS=4096
MAX_BYTES=2*1024*1024

def inspect(path:Path,expected_build_id:str)->dict:
    if not re.fullmatch(r'EGFX-[0-9a-f]{24}',expected_build_id):raise ValueError('invalid expected build ID')
    if path.is_symlink() or not path.is_file() or path.stat().st_size>MAX_BYTES:raise ValueError('invalid diagnostic CSV')
    raw=path.read_bytes()
    if not raw.endswith(b'\n'):raise ValueError('incomplete diagnostic CSV')
    source=raw.decode('ascii');marker,body=source.split('\n',1)
    if marker!='# schema=1,build_id='+expected_build_id:raise ValueError('diagnostic build marker mismatch')
    reader=csv.DictReader(io.StringIO(body))
    if tuple(reader.fieldnames or ())!=HEADER:raise ValueError('diagnostic schema mismatch')
    records=[]
    for row in reader:
        if set(row)!=set(HEADER) or None in row or any(value is None for value in row.values()):raise ValueError('malformed diagnostic row')
        if len(records)>=MAX_RECORDS:raise ValueError('oversized diagnostic record list')
        parsed={}
        for key,value in row.items():
            if key in ('selector','path'):parsed[key]=value;continue
            if key in PHASES and value=='':parsed[key]=None;continue
            if not re.fullmatch(r'-?[0-9]{1,20}',value):raise ValueError('invalid diagnostic integer')
            parsed[key]=int(value)
        if parsed['sequence']!=len(records)+1:raise ValueError('missing/duplicate diagnostic sequence')
        if parsed['selector'] not in ('render','smart_pre','smart') or parsed['path'] not in ('unknown','legacy_cpu','plane_layer','plane_region'):
            raise ValueError('unknown diagnostic path/selector')
        if parsed['has_output'] not in (0,1) or parsed['completed'] not in (0,1):raise ValueError('invalid diagnostic completion flag')
        if parsed['total_ns']<0 or parsed['start_ns']<0 or parsed['time_scale']<0 or any(parsed[k]<0 for k in ('width','height','depth')):
            raise ValueError('invalid diagnostic dimensions/time')
        points=[parsed[k] for k in PHASES if parsed[k] is not None]+[parsed['total_ns']]
        if any(n<0 for n in points) or points!=sorted(points):raise ValueError('nonmonotonic diagnostic endpoints')
        if parsed['completed']:
            if parsed['path']=='unknown' or min(parsed['width'],parsed['height'],parsed['time_scale'])<=0:
                raise ValueError('completed diagnostic lacks context')
            required=('parameters_ns','input_ns') if parsed['selector']=='smart_pre' else ('parameters_ns',)
            if parsed['has_output']:
                if parsed['selector']=='smart_pre' or parsed['depth'] not in (8,16,32):raise ValueError('invalid rendered-output context')
                required+=('sampling_ns',)
                if parsed['selector']=='smart':required+=('input_ns','output_ns','checkin_ns')
            if any(parsed[k] is None for k in required):raise ValueError('completed diagnostic lacks required phases')
        records.append(parsed)
    partial=any(not r['completed'] for r in records)
    status='TRUNCATED' if len(records)==MAX_RECORDS else 'PARTIAL' if partial else 'DECODED' if records else 'NO_RECORDS'
    groups={};intervals=[]
    for r in records:
        if not r['completed'] or not r['has_output']:continue
        start=r['output_ns'] if r['selector']=='smart' else r['parameters_ns']
        duration=(r['sampling_ns']-start)/1e6
        if r['sampling_ns']>start:
            intervals.extend(((r['start_ns']+start,1),(r['start_ns']+r['sampling_ns'],-1)))
        groups.setdefault(r['path'],[]).append(duration)
    summary={path:dict(observed_render_callbacks=len(values),median_sampling_ms=statistics.median(values),
        p95_sampling_ms=sorted(values)[max(0,(95*len(values)+99)//100-1)],min_sampling_ms=min(values),max_sampling_ms=max(values))
        for path,values in groups.items()}
    active=peak=0
    for _,change in sorted(intervals):
        active+=change;peak=max(peak,active)
    return dict(schema=1,status=status,build_id=expected_build_id,observed_peak_sampling_interval_overlap=peak,
        scope='decoded instrumented callback observations; not runtime identity, target performance acceptance or cached playback proof',
        records=records,summary=summary,limitations=[
            'phase endpoints include observer overhead; synchronous logging happens afterward and perturbs host execution',
            'callbacks are not unique frames; MFR may finish out of order',
            'overlapping wall intervals do not alone prove concurrent CPU execution',
            'missing/no callbacks cannot alone prove cache hits or preview completion',
            'capped, partial or invalid observations cannot establish performance PASS'])

def validate_fixture(record:dict,fixture:dict)->dict:
    """Require the declared workload to have actually produced every frame.

    Use only a hash-checked schema-3 fixture from aerender_benchmark.load_fixture.
    This proves callback coverage, not onscreen playback or uninstrumented speed.
    """
    if record['status']!='DECODED' or fixture.get('schema')!=3:
        raise ValueError('complete observations and explicit schema-3 workload required')
    outputs=[r for r in record['records'] if r['selector'] in ('smart','render') and r['has_output'] and r['completed']]
    frames=set();fps=Fraction(str(fixture['fps']))
    for r in outputs:
        if (r['path'],r['width'],r['height'],r['depth'])!=(fixture['expected_render_path'],fixture['width'],fixture['height'],fixture['bit_depth']):
            raise ValueError('actual output callback route/dimensions/depth differs from fixture')
        frame=Fraction(r['time_numerator'],r['time_scale'])*fps
        if frame.denominator!=1:raise ValueError('unexpected fractional callback frame')
        frames.add(int(frame))
    count=fixture['frame_end']-fixture['frame_start']+1
    if not 0<count<=MAX_RECORDS:raise ValueError('bounded diagnostic frame range required')
    expected=set(range(fixture['frame_start'],fixture['frame_end']+1))
    if frames!=expected:raise ValueError('actual render frame coverage differs from fixture')
    return dict(status='ROUTE_AND_FRAME_COVERAGE_PASS',expected_render_path=fixture['expected_render_path'],
        unique_render_frames=len(frames),observed_render_callbacks=len(outputs),
        scope='instrumented completed-output callbacks; not cached playback or uninstrumented speed')

def main()->int:
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('csv',type=Path);p.add_argument('--build-id',required=True)
    p.add_argument('--fixture',type=Path,help='Pinned schema-3 fixture; reject the wrong render workload')
    a=p.parse_args()
    try:
        record=inspect(a.csv,a.build_id)
        if a.fixture:
            import aerender_benchmark as ab
            fixture=ab.load_fixture(a.fixture.absolute().parent,a.fixture.absolute())
            record['fixture_observation']=validate_fixture(record,fixture)
    except (ValueError,OSError,UnicodeError) as error:
        print(json.dumps(dict(status='REJECTED',reason=str(error))));return 3
    print(json.dumps(record,indent=2));return 0 if record['status']=='DECODED' else 3

if __name__=='__main__':raise SystemExit(main())
