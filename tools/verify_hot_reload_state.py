#!/usr/bin/env python3
import pathlib,re,sys
lib=pathlib.Path(sys.argv[1]).read_text(); shell=pathlib.Path(sys.argv[2]).read_text()
def req(p,t,l):
 m=re.search(p,t,re.S|re.M)
 if not m: raise SystemExit("missing "+l)
 return m.group(1)
if int(req(r'HOT_RELOAD_STATE_ABI:\s*u64\s*=\s*(\d+)',lib,'StateABI'))!=5: raise SystemExit('StateABI != 5')
if int(req(r'kImplementationStateAbi\s*=\s*(\d+)',shell,'shell StateABI'))!=5: raise SystemExit('shell StateABI != 5')
if int(req(r'GRID_WIRE_VERSION:\s*u16\s*=\s*(\d+)',lib,'grid wire'))!=3: raise SystemExit('grid wire != 3')
for s in ['MAX_GUIDES: usize = 50','"Num Columns"','"Num Rows"','"Falloff Profile"','"Smoothstep", "Gaussian", "Linear"','"Wave Animation"','"Visualization"','"Rendering"','"Reset Grid"','checkout_smart_render_state(params)','extra.set_gpu_render_possible(false)','HotReloadPreRenderState','AEHotLoader_ImplementationRuntimeABI','AEHotLoader_SetGeneration','offset_of!(MetalGpuData,generation)','offset_of!(MetalGpuData,destroy_fn)']:
 if s not in lib: raise SystemExit('missing parity evidence: '+s)
print('hot-reload parity state: PASS StateABI=5 grid_wire=3 original-derived surface=present')
