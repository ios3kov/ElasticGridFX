#!/usr/bin/env python3
"""Prepare a synthetic pinned aerender fixture in an explicitly empty test AE project."""
from __future__ import annotations
import argparse
import json
from pathlib import Path
import platform
import plistlib
import re
import subprocess
import sys
import uuid

import aerender_benchmark as ab
import build_identity as bi
from install_candidate import checked_path, signature
from live_identity import running_ae
from smoke_pixels import pattern

ROOT=Path(__file__).resolve().parents[1]


def prepare(root:Path,width:int,height:int,bit_depth:int,mode:str,output_precision:int=16)->tuple[Path,dict]:
    root=checked_path(root)
    root.mkdir(parents=True,exist_ok=True)
    if not ((width,height) in ((1920,1080),(3840,2160))) or bit_depth not in (8,16,32) or mode not in ('static','animated') or output_precision not in (8,16):
        raise ValueError('unsupported performance fixture configuration')
    run_id=uuid.uuid4().hex
    folder=root/('EGFX-perf-'+run_id)
    folder.mkdir(mode=0o700)
    pattern(folder/'pattern.png',width,height)
    source=(ROOT/'tests/perf_fixture.jsx').read_text()
    config=dict(run_id=run_id,folder=str(folder.resolve()),width=width,height=height,
                bit_depth=bit_depth,fps=30,duration=2,mode=mode,output_precision=output_precision)
    (folder/'run.jsx').write_text(source+'\nelasticGridPerfFixture('+json.dumps(config)+');\n')
    meta=dict(schema=1,run_id=run_id,status='NOT RUN',actual_ae_execution=False,
              config={k:v for k,v in config.items() if k!='folder'},
              jsx_sha256=bi.digest(source.encode()),pattern_sha256=ab.file_digest(folder/'pattern.png'))
    bi.dump(folder/'prepare.json',meta)
    return folder,meta


def inspect(folder:Path,meta:dict)->dict:
    capture=bi.safe_file(folder,'capture.json')
    if capture.stat().st_size>16384: raise ValueError('oversized fixture capture')
    data=json.loads(capture.read_text(encoding='utf-8-sig'))
    if data.get('run_id')!=meta['run_id'] or data.get('status')!='PREPARED' or data.get('stage')!='prepared' or data.get('saved') is not True:
        raise ValueError('fixture preparation did not complete')
    cfg=meta['config']
    for key in ('width','height','bit_depth','mode'):
        if data.get(key)!=cfg[key]: raise ValueError('fixture capture configuration mismatch')
    if data.get('fps')!=30 or data.get('duration')!=2 or data.get('composition')!='EGFX_PERF' or data.get('rqindex')!=1:
        raise ValueError('fixture render configuration mismatch')
    if data.get('render_template')!='Best Settings' or data.get('output_template') not in ('PNG Sequence','png','_HIDDEN X-Factor 16') or data.get('output_format')!='PNG Sequence':
        raise ValueError('required render/output templates were not applied')
    precision=cfg['output_precision']
    if data.get('output_precision')!=precision or data.get('working_space') not in ('','None') or data.get('linearize') is not False:
        raise ValueError('output precision or controlled color context was not recorded')
    if precision==16 and (data.get('output_depth')!='Trillions of Colors+' or data.get('output_channels')!='RGB + Alpha' or data.get('output_color')!='Straight (Unmatted)'):
        raise ValueError('required straight RGBA16 output was not applied')
    if precision==8 and (data.get('output_depth')!='Millions of Colors' or data.get('output_channels')!='RGB'):
        raise ValueError('legacy RGB8 output was not applied')
    if data.get('output_pattern')!='frame_[#####].png':
        raise ValueError('unexpected fixture output pattern')
    project=bi.safe_file(folder,'EGFX_PERF.aep')
    fixture=dict(schema=2,fixture_id=f"egfx-perf-{cfg['width']}x{cfg['height']}-{cfg['bit_depth']}bpc-{cfg['mode']}-{precision}out",
                 project='EGFX_PERF.aep',project_sha256=ab.file_digest(project),
                 composition='EGFX_PERF',rqindex=1,width=cfg['width'],height=cfg['height'],
                 fps=30.0,frame_start=0,frame_end=59,bit_depth=cfg['bit_depth'],
                 quality='Final Bicubic',output_format='PNG sequence',
                 output_pattern='frame_[#####].png',output_bit_depth=precision,
                 output_channels='RGBA' if precision==16 else 'RGB',
                 output_color=data['output_color'],working_space=data['working_space'],linearize=False)
    bi.dump(folder/'fixture.json',fixture)
    return fixture


def execute(folder:Path,meta:dict,ae_app:Path,installed:Path,package:Path,manifest:Path)->dict:
    if platform.system()!='Darwin' or platform.machine()!='arm64':
        raise ValueError('fixture execution requires Apple Silicon macOS')
    checked_path(ae_app,directory=True);checked_path(installed,directory=True)
    bi.verify(installed,package,manifest);signature(installed)
    app_info=plistlib.loads(bi.safe_file(ae_app,'Contents/Info.plist').read_bytes())
    identifier=app_info['CFBundleIdentifier']
    if not re.fullmatch(r'com\.adobe\.[A-Za-z0-9_.-]+',identifier,re.I) or 'aftereffects' not in identifier.lower():
        raise ValueError('selected app is not After Effects')
    expected=ae_app/'Contents/MacOS'/app_info['CFBundleExecutable']
    hosts=running_ae()
    if len(hosts)!=1 or Path(hosts[0]['executable'])!=expected:
        raise ValueError('require exactly one running target AE process')
    meta['target_pid']=hosts[0]['pid'];meta['ae_execution_attempted']=True
    apple='''on run argv
set jsxText to read POSIX file (item 1 of argv) as «class utf8»
if not (running of application id "IDENTIFIER") then error "Selected test AE is not running"
with timeout of 180 seconds
tell application id "IDENTIFIER"
set resultCode to DoScript jsxText
end tell
end timeout
return resultCode
end run
'''.replace('IDENTIFIER',identifier)
    result=subprocess.run(['/usr/bin/osascript','-',str(folder/'run.jsx')],input=apple,capture_output=True,text=True,timeout=185)
    (folder/'transport.log').write_text(result.stdout+'\n'+result.stderr)
    if result.returncode or result.stdout.strip()!='0':
        raise ValueError('AE fixture preparation failed; retained private transport log')
    fixture=inspect(folder,meta)
    meta.update(status='PREPARED',actual_ae_execution=True,fixture=fixture)
    return meta


def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--root',type=Path,required=True)
    p.add_argument('--width',type=int,default=1920);p.add_argument('--height',type=int,default=1080)
    p.add_argument('--bit-depth',type=int,default=32);p.add_argument('--mode',choices=('static','animated'),default='animated')
    p.add_argument('--output-precision',type=int,choices=(8,16),default=16,help='16: verified straight RGBA; 8: legacy dithering research only')
    p.add_argument('--ae-app',type=Path);p.add_argument('--installed-bundle',type=Path)
    p.add_argument('--package',type=Path);p.add_argument('--manifest',type=Path)
    p.add_argument('--execute-in-test-ae',action='store_true')
    a=p.parse_args()
    try:
        folder,meta=prepare(a.root,a.width,a.height,a.bit_depth,a.mode,a.output_precision)
        if a.execute_in_test_ae:
            if not all((a.ae_app,a.installed_bundle,a.package,a.manifest)):
                raise ValueError('execution requires AE app, installed bundle, package and manifest')
            meta=execute(folder,meta,a.ae_app,a.installed_bundle,a.package,a.manifest)
        bi.dump(folder/'prepare.json',meta)
    except (OSError,ValueError,KeyError,subprocess.SubprocessError) as e:
        print('BLOCKED: '+str(e),file=sys.stderr);return 3
    print(json.dumps(dict(status=meta['status'],workspace=str(folder),fixture=str(folder/'fixture.json') if meta['status']=='PREPARED' else None),indent=2))
    return 0 if (not a.execute_in_test_ae or meta['status']=='PREPARED') else 3

if __name__=='__main__':sys.exit(main())
