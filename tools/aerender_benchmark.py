#!/usr/bin/env python3
"""Controlled aerender benchmark harness. Never edits projects, preferences or caches."""
from __future__ import annotations
import argparse
import hashlib
import json
import math
from pathlib import Path, PurePosixPath
import platform
import re
import statistics
import subprocess
import sys
import time
import uuid

import build_identity as bi
from install_candidate import checked_path, signature

SCHEMA = 1
MIN_SAMPLES = 5
MAX_SAMPLES = 20
MAX_OUTPUT_FILES = 10000
MAX_OUTPUT_BYTES = 8 * 1024 * 1024 * 1024
BUILD_MARKER = re.compile(r'ElasticGridBuildID=(EGFX-[0-9a-f]{24})')
TIME_LINE = re.compile(r'^\s*([0-9.]+)\s+real\s+([0-9.]+)\s+user\s+([0-9.]+)\s+sys\s*$', re.M)
RSS_LINE = re.compile(r'^\s*(\d+)\s+maximum resident set size\s*$', re.M)


def file_digest(path: Path) -> str:
    h = hashlib.sha256()
    total = 0
    with path.open('rb') as handle:
        while True:
            block = handle.read(1024 * 1024)
            if not block:
                break
            total += len(block)
            if total > MAX_OUTPUT_BYTES:
                raise ValueError('output file exceeds benchmark size limit')
            h.update(block)
    return h.hexdigest()


def relative_under(root: Path, path: Path) -> Path:
    root = checked_path(root, directory=True)
    path = checked_path(path)
    try:
        rel = path.relative_to(root)
    except ValueError as error:
        raise ValueError('benchmark project/output must stay inside the controlled workspace') from error
    if not rel.parts or '..' in rel.parts:
        raise ValueError('invalid controlled workspace path')
    return rel


def safe_output_pattern(value: str) -> str:
    rel = PurePosixPath(value)
    if rel.is_absolute() or len(rel.parts) != 1 or rel.as_posix() != value or '\\' in value:
        raise ValueError('output pattern must be one relative filename')
    if any(ord(c) < 32 for c in value) or '#' not in value:
        raise ValueError('output pattern must contain frame-number # placeholders')
    return value


def load_fixture(workspace: Path, fixture_path: Path) -> dict:
    workspace = checked_path(workspace, directory=True)
    fixture_path = checked_path(fixture_path)
    relative_under(workspace, fixture_path)
    data = json.loads(fixture_path.read_text())
    common = {'schema','fixture_id','project','project_sha256','composition','rqindex',
              'width','height','fps','frame_start','frame_end','bit_depth',
              'quality','output_format','output_pattern'}
    schema = data.get('schema')
    if schema == 1:
        if set(data) != common:
            raise ValueError('invalid legacy performance fixture schema')
        data = dict(data, geometry='grid', color_management='legacy-unspecified')
    elif schema == 2:
        if set(data) != common | {'geometry','color_management'}:
            raise ValueError('invalid performance fixture schema v2')
        if data['geometry'] not in ('grid','four_corners','native_3d'):
            raise ValueError('invalid fixture geometry')
        if data['color_management'] not in ('none-linearize-off',):
            raise ValueError('invalid fixture color-management contract')
    else:
        raise ValueError('unsupported performance fixture schema')
    if not re.fullmatch(r'[A-Za-z0-9_.-]{1,80}', data['fixture_id']):
        raise ValueError('invalid fixture identifier')
    project_rel = PurePosixPath(data['project'])
    if project_rel.is_absolute() or '..' in project_rel.parts or project_rel.as_posix() != data['project']:
        raise ValueError('unsafe fixture project path')
    project = checked_path(workspace / Path(data['project']))
    relative_under(workspace, project)
    if project.suffix.lower() != '.aep' or not project.is_file() or project.stat().st_size <= 0:
        raise ValueError('fixture project must be a nonempty .aep file')
    if file_digest(project) != data['project_sha256']:
        raise ValueError('fixture project changed')
    if not isinstance(data['composition'], str) or not data['composition'] or len(data['composition']) > 160:
        raise ValueError('invalid fixture composition')
    if not isinstance(data['rqindex'], int) or data['rqindex'] < 1:
        raise ValueError('invalid render queue index')
    for key in ('width','height','frame_start','frame_end','bit_depth'):
        if not isinstance(data[key], int):
            raise ValueError('invalid integer fixture field: '+key)
    if data['width'] < 1 or data['height'] < 1 or data['frame_end'] < data['frame_start']:
        raise ValueError('invalid fixture dimensions/frame range')
    if data['bit_depth'] not in (8,16,32) or data['quality'] != 'Final Bicubic':
        raise ValueError('benchmark fixture must use Final Bicubic and 8/16/32 bpc')
    if not isinstance(data['fps'], (int,float)) or not math.isfinite(data['fps']) or data['fps'] <= 0:
        raise ValueError('invalid fixture fps')
    if data['output_format'] not in ('PNG sequence','EXR sequence'):
        raise ValueError('benchmark output must be a deterministic image sequence')
    safe_output_pattern(data['output_pattern'])
    return dict(data, project_path=project)


def verify_candidate(bundle: Path, package: Path, manifest_path: Path) -> dict:
    bundle = checked_path(bundle, directory=True)
    package = checked_path(package)
    manifest_path = checked_path(manifest_path)
    bi.verify(bundle, package, manifest_path)
    signature(bundle)
    manifest = json.loads(manifest_path.read_text())
    return bi.validate_identity(manifest['build'])


def parse_time_metrics(stderr: str) -> dict:
    times = TIME_LINE.findall(stderr)
    rss = RSS_LINE.findall(stderr)
    if len(times) != 1 or len(rss) != 1:
        raise ValueError('missing/ambiguous macOS time -l metrics')
    real, user, sys_time = map(float, times[0])
    peak = int(rss[0])
    if min(real,user,sys_time) < 0 or peak <= 0:
        raise ValueError('invalid process metrics')
    return dict(time_real_seconds=real, user_seconds=user, sys_seconds=sys_time,
                peak_rss_bytes=peak)


def runtime_build_id(stdout: str, stderr: str, expected: str) -> str:
    found = set(BUILD_MARKER.findall(stdout + '\n' + stderr))
    if found != {expected}:
        raise ValueError('runtime Build ID was missing, ambiguous or different')
    return expected


def output_manifest(folder: Path) -> tuple[list[dict], str]:
    checked_path(folder, directory=True)
    files = []
    total = 0
    for path in sorted(folder.rglob('*')):
        checked_path(path)
        if path.is_dir():
            continue
        if not path.is_file():
            raise ValueError('unsupported benchmark output entry')
        total += path.stat().st_size
        if len(files) >= MAX_OUTPUT_FILES or total > MAX_OUTPUT_BYTES:
            raise ValueError('benchmark output exceeds retained evidence limits')
        rel = path.relative_to(folder).as_posix()
        files.append(dict(path=rel, size=path.stat().st_size, sha256=file_digest(path)))
    if not files:
        raise ValueError('aerender produced no benchmark output')
    return files, bi.digest(bi.encoded(files))


def percentile(values: list[float], p: float) -> float:
    if not values:
        raise ValueError('empty percentile')
    ordered = sorted(values)
    rank = max(0, min(len(ordered)-1, math.ceil(p * len(ordered)) - 1))
    return ordered[rank]


def summarize(samples: list[dict], frame_count: int) -> dict:
    if len(samples) < MIN_SAMPLES:
        raise ValueError('at least five measured samples are required')
    if frame_count < 1:
        raise ValueError('invalid benchmark frame count')
    walls = [s['wall_seconds'] for s in samples]
    rss = [s['metrics']['peak_rss_bytes'] for s in samples]
    digests = sorted(set(s['output_digest'] for s in samples))
    if len(digests) != 1:
        raise ValueError('measured runs produced different encoded output digests')
    median = statistics.median(walls)
    p95 = percentile(walls,0.95)
    return dict(samples=len(samples), frame_count=frame_count,
                wall_median_seconds=median, wall_p95_seconds=p95,
                wall_min_seconds=min(walls), wall_max_seconds=max(walls),
                ms_per_frame_median=median*1000.0/frame_count,
                ms_per_frame_p95=p95*1000.0/frame_count,
                peak_rss_max_bytes=max(rss), output_digest=digests[0],
                encoded_output_repeatability='PASS')


def run_once(index: int, kind: str, workspace: Path, aerender: Path, fixture: dict,
             expected_build_id: str, mfr: str, timeout: int) -> dict:
    run = workspace / 'runs' / f'{kind}-{index:02d}-{uuid.uuid4().hex}'
    output = run / 'output'
    run.mkdir(parents=True, mode=0o700)
    output.mkdir(mode=0o700)
    pattern = safe_output_pattern(fixture['output_pattern'])
    output_path = output / pattern
    cmd = ['/usr/bin/time','-l',str(aerender),'-project',str(fixture['project_path']),
           '-rqindex',str(fixture['rqindex']),'-output',str(output_path),
           '-v','ERRORS_AND_PROGRESS']
    if mfr != 'inherit':
        cmd += ['-mfr', mfr.upper(), '100']
    started = time.perf_counter()
    result = subprocess.run(cmd, capture_output=True, text=True, timeout=timeout)
    wall = time.perf_counter() - started
    (run/'stdout.log').write_text(result.stdout)
    (run/'stderr.log').write_text(result.stderr)
    if result.returncode != 0:
        raise ValueError(f'aerender failed in {kind} sample {index}; retained logs in controlled workspace')
    observed = runtime_build_id(result.stdout, result.stderr, expected_build_id)
    metrics = parse_time_metrics(result.stderr)
    outputs, digest = output_manifest(output)
    record = dict(index=index,kind=kind,wall_seconds=wall,metrics=metrics,
                  runtime_build_id=observed,output_digest=digest,
                  output_files=len(outputs),output_bytes=sum(x['size'] for x in outputs))
    bi.dump(run/'sample.json', record)
    return record


def environment_summary(aerender: Path) -> dict:
    version = subprocess.run([str(aerender),'-version'],capture_output=True,text=True,timeout=30)
    text = (version.stdout+'\n'+version.stderr).strip()
    if version.returncode != 0 or not text:
        raise ValueError('aerender version query failed')
    return dict(os=platform.system(),os_version=platform.mac_ver()[0],
                architecture=platform.machine(),aerender_version=text[:1024])


def benchmark(workspace: Path, fixture_path: Path, aerender: Path, bundle: Path,
              package: Path, manifest: Path, warmups: int, samples: int,
              mfr: str, timeout: int, test_case_id: str) -> tuple[dict, Path]:
    if platform.system() != 'Darwin' or platform.machine() != 'arm64':
        raise ValueError('benchmark requires the target Apple Silicon macOS environment')
    if not 0 <= warmups <= 5 or not MIN_SAMPLES <= samples <= MAX_SAMPLES:
        raise ValueError('invalid warmup/sample count')
    if mfr not in ('inherit','on','off') or timeout < 30:
        raise ValueError('invalid benchmark settings')
    if not re.fullmatch(r'PERF094-[A-Z0-9-]{3,64}', test_case_id):
        raise ValueError('invalid 0.9.4 performance Test Case ID')
    workspace = checked_path(workspace, directory=True)
    fixture = load_fixture(workspace, fixture_path)
    aerender = checked_path(aerender)
    if aerender.name != 'aerender' or not aerender.is_file():
        raise ValueError('explicit aerender executable required')
    build = verify_candidate(bundle, package, manifest)
    env = environment_summary(aerender)
    runs = workspace/'runs'
    checked_path(runs)
    if runs.exists():
        if any(runs.iterdir()):
            raise ValueError('benchmark runs directory must be fresh/empty')
    else:
        runs.mkdir(mode=0o700)
    warm = [run_once(i,'warmup',workspace,aerender,fixture,build['build_id'],mfr,timeout)
            for i in range(1,warmups+1)]
    measured = [run_once(i,'sample',workspace,aerender,fixture,build['build_id'],mfr,timeout)
                for i in range(1,samples+1)]
    frame_count = fixture['frame_end'] - fixture['frame_start'] + 1
    report = dict(schema=SCHEMA,status='MEASURED',release='BLOCKED',
                  test_case_id=test_case_id,test_run_id=uuid.uuid4().hex,
                  scope='aerender timing/memory + runtime Build ID; not RAM Preview or HDR pixel equivalence',
                  candidate=build,fixture={k:v for k,v in fixture.items() if k!='project_path'},
                  environment=env,settings=dict(warmups=warmups,samples=samples,mfr=mfr,
                                                timeout_seconds=timeout,
                                                cache_state='warm-after-explicit-warmups' if warmups else 'uncontrolled'),
                  warmups=warm,samples_data=measured,summary=summarize(measured,frame_count),
                  quality_equivalence='NOT RUN',
                  encoded_output_repeatability='PASS')
    output = workspace/'aerender-benchmark.json'
    if output.exists() or output.is_symlink():
        raise ValueError('refusing stale benchmark report')
    bi.dump(output,report)
    return report,output


def main() -> int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--workspace',type=Path,required=True)
    p.add_argument('--fixture',type=Path,required=True)
    p.add_argument('--aerender',type=Path,required=True)
    p.add_argument('--installed-bundle',type=Path,required=True)
    p.add_argument('--package',type=Path,required=True)
    p.add_argument('--manifest',type=Path,required=True)
    p.add_argument('--warmups',type=int,default=2)
    p.add_argument('--samples',type=int,default=5)
    p.add_argument('--mfr',choices=('inherit','on','off'),default='inherit')
    p.add_argument('--timeout',type=int,default=1800)
    p.add_argument('--test-case-id',required=True,
                   help='Versioned Test Case ID, e.g. PERF094-RQ-001')
    a=p.parse_args()
    try:
        report,path=benchmark(a.workspace,a.fixture,a.aerender,a.installed_bundle,a.package,a.manifest,
                              a.warmups,a.samples,a.mfr,a.timeout,a.test_case_id)
    except (OSError,ValueError,KeyError,subprocess.SubprocessError) as e:
        print('BLOCKED: '+str(e),file=sys.stderr); return 3
    print(json.dumps(dict(status=report['status'],report=str(path),release='BLOCKED'),indent=2))
    return 0

if __name__=='__main__':
    sys.exit(main())
