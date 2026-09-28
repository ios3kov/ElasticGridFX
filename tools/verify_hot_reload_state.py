#!/usr/bin/env python3
import pathlib
import re
import sys

if len(sys.argv) != 3:
    raise SystemExit("usage: verify_hot_reload_state.py LIB_RS SHELL_CPP")

lib = pathlib.Path(sys.argv[1]).read_text()
shell = pathlib.Path(sys.argv[2]).read_text()

def require(pattern: str, text: str, label: str) -> str:
    m = re.search(pattern, text, re.MULTILINE | re.DOTALL)
    if not m:
        raise SystemExit(f"missing {label}")
    return m.group(1)

impl_abi = int(require(r'const\s+HOT_RELOAD_STATE_ABI:\s*u64\s*=\s*(\d+)\s*;', lib, "implementation StateABI"))
shell_abi = int(require(r'kImplementationStateAbi\s*=\s*(\d+)\s*;', shell, "shell StateABI"))
if impl_abi != shell_abi:
    raise SystemExit(f"StateABI drift: implementation={impl_abi} shell={shell_abi}")

if impl_abi != 2:
    raise SystemExit(f"unexpected ElasticGrid StateABI {impl_abi}; update verifier intentionally when schema changes")

if "ae::define_effect!(Plugin, (), Params);" not in lib:
    raise SystemExit("effect global/sequence contract changed; review StateABI")

wire_version = int(require(r'GRID_WIRE_VERSION:\s*u16\s*=\s*(\d+)\s*;', lib, "GRID_WIRE_VERSION"))
if wire_version != 1:
    raise SystemExit("GridArb wire version changed; review/bump hot-reload StateABI")

params_body = require(r'enum\s+Params\s*\{(.*?)\}', lib, "Params enum")
params_body = re.sub(r'//.*', '', params_body)
params = []
for item in params_body.split(','):
    item = item.strip()
    if not item:
        continue
    m = re.match(r'([A-Za-z_][A-Za-z0-9_]*)', item)
    if m:
        params.append(m.group(1))

expected_params = [
    "Columns", "Rows", "GridState", "TensionRadius", "Falloff",
    "ElasticityStrength", "MinSpacing", "StretchEasing", "EasingDistance",
    "WaveEnabled", "WaveAmplitude", "WaveFrequency", "WavePhase",
    "WaveSpeed", "WaveAxis", "EdgeMode", "Quality",
]
if params != expected_params:
    raise SystemExit(
        "ElasticGrid parameter schema changed; bump StateABI and update verifier intentionally:\n"
        f"  expected={expected_params}\n  actual={params}"
    )

gpu_body = require(r'struct\s+MetalGpuData\s*\{(.*?)\}', lib, "MetalGpuData")
gpu_fields = {
    name: typ.strip()
    for name, typ in re.findall(r'([A-Za-z_][A-Za-z0-9_]*)\s*:\s*([^,]+),', gpu_body)
}
expected_gpu = {"state": "*mut c_void", "generation": "u64"}
if gpu_fields != expected_gpu:
    raise SystemExit(
        "ElasticGrid GPU state schema changed; bump StateABI and update verifier intentionally:\n"
        f"  expected={expected_gpu}\n  actual={gpu_fields}"
    )

for evidence in [
    "size_of::<MetalGpuData>()",
    "offset_of!(MetalGpuData, generation)",
    "AEHotLoader_ImplementationRuntimeABI",
]:
    if evidence not in lib:
        raise SystemExit(f"missing hot-reload state/layout evidence: {evidence}")

print(
    f"hot-reload state: PASS StateABI={impl_abi} "
    f"params={len(params)} grid_wire={wire_version} gpu_generation=present"
)
