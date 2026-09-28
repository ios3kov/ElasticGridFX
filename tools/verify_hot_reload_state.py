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


def extract_function(text: str, signature: str) -> str:
    start = text.find(signature)
    if start < 0:
        raise SystemExit(f"missing function {signature}")
    opened = text.find("{", start)
    if opened < 0:
        raise SystemExit(f"missing function body {signature}")
    depth = 0
    in_string = False
    quote = ""
    escaped = False
    line_comment = False
    block_comment = 0
    i = opened
    while i < len(text):
        c = text[i]
        n = text[i + 1] if i + 1 < len(text) else ""
        if line_comment:
            if c == "\n":
                line_comment = False
            i += 1
            continue
        if block_comment:
            if c == "*" and n == "/":
                block_comment -= 1
                i += 2
                continue
            if c == "/" and n == "*":
                block_comment += 1
                i += 2
                continue
            i += 1
            continue
        if in_string:
            if escaped:
                escaped = False
            elif c == "\\":
                escaped = True
            elif c == quote:
                in_string = False
            i += 1
            continue
        if c == "/" and n == "/":
            line_comment = True
            i += 2
            continue
        if c == "/" and n == "*":
            block_comment = 1
            i += 2
            continue
        if c in ('"', "'"):
            in_string = True
            quote = c
            i += 1
            continue
        if c == "{":
            depth += 1
        elif c == "}":
            depth -= 1
            if depth == 0:
                return text[start:i + 1]
        i += 1
    raise SystemExit(f"unbalanced function body {signature}")


def normalized_code(text: str) -> str:
    text = re.sub(r'/\*.*?\*/', '', text, flags=re.DOTALL)
    text = re.sub(r'//[^\n]*', '', text)
    return re.sub(r'\s+', '', text)


def fnv1a64(text: str) -> int:
    value = 1469598103934665603
    for byte in text.encode("utf-8"):
        value ^= byte
        value = (value * 1099511628211) & 0xFFFFFFFFFFFFFFFF
    return value


EXPECTED_PARAMS_SETUP_FNV64 = 0x200ad28b1add9ec0
params_setup_hash = fnv1a64(normalized_code(extract_function(lib, "fn params_setup(")))
if params_setup_hash != EXPECTED_PARAMS_SETUP_FNV64:
    raise SystemExit(
        "params_setup host contract changed; this cannot be live-reloaded safely. "
        "Bump StateABI, rebuild/install the shell, and update the verifier intentionally: "
        f"expected=0x{EXPECTED_PARAMS_SETUP_FNV64:016x} actual=0x{params_setup_hash:016x}"
    )

impl_protocol = int(require(
    r'AEHotLoader_ImplementationABI\(\)\s*->\s*u32\s*\{\s*(\d+)\s*\}',
    lib,
    "implementation protocol ABI",
))
shell_protocol = int(require(
    r'kImplementationAbi\s*=\s*(\d+)\s*;',
    shell,
    "shell implementation ABI",
))
if impl_protocol != 2 or shell_protocol != 2 or impl_protocol != shell_protocol:
    raise SystemExit(
        f"Protocol ABI drift: implementation={impl_protocol} shell={shell_protocol}; expected=2"
    )

impl_abi = int(require(r'const\s+HOT_RELOAD_STATE_ABI:\s*u64\s*=\s*(\d+)\s*;', lib, "implementation StateABI"))
shell_abi = int(require(r'kImplementationStateAbi\s*=\s*(\d+)\s*;', shell, "shell StateABI"))
if impl_abi != shell_abi:
    raise SystemExit(f"StateABI drift: implementation={impl_abi} shell={shell_abi}")

if impl_abi != 4:
    raise SystemExit(f"unexpected ElasticGrid StateABI {impl_abi}; update verifier intentionally when schema changes")

if "ae::define_effect!(Plugin, (), Params);" not in lib:
    raise SystemExit("effect global/sequence contract changed; review StateABI")

wire_version = int(require(r'GRID_WIRE_VERSION:\s*u16\s*=\s*(\d+)\s*;', lib, "GRID_WIRE_VERSION"))
if wire_version != 1:
    raise SystemExit("GridArb wire version changed; review/bump hot-reload StateABI")

grid_body = require(r'struct\s+GridArb\s*\{(.*?)\}', lib, "GridArb")
grid_fields = [
    (name, typ.strip())
    for name, typ in re.findall(
        r'(?:pub\(crate\)\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*:\s*([^,]+),',
        grid_body,
    )
]
expected_grid_fields = [
    ("columns", "u16"),
    ("rows", "u16"),
    ("column_lines", "Vec<f32>"),
    ("row_lines", "Vec<f32>"),
    ("column_pins", "Vec<u8>"),
    ("row_pins", "Vec<u8>"),
]
if grid_fields != expected_grid_fields:
    raise SystemExit(
        "GridArb persistent schema changed; bump StateABI/wire version and update verifier intentionally:\n"
        f"  expected={expected_grid_fields}\n  actual={grid_fields}"
    )

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
expected_gpu = {
    "state": "*mut c_void",
    "generation": "u64",
    "destroy_fn": "MetalDestroyFn",
}
if gpu_fields != expected_gpu:
    raise SystemExit(
        "ElasticGrid GPU state schema changed; bump StateABI and update verifier intentionally:\n"
        f"  expected={expected_gpu}\n  actual={gpu_fields}"
    )


pre_render_body = require(
    r'struct\s+HotReloadPreRenderState\s*\{(.*?)\}',
    lib,
    "HotReloadPreRenderState",
)
pre_render_fields = {
    name: typ.strip()
    for name, typ in re.findall(
        r'([A-Za-z_][A-Za-z0-9_]*)\s*:\s*([^,]+),',
        pre_render_body,
    )
}
expected_pre_render = {"generation": "u64"}
if pre_render_fields != expected_pre_render:
    raise SystemExit(
        "ElasticGrid SmartFX pre-render schema changed; bump StateABI and update verifier intentionally:\n"
        f"  expected={expected_pre_render}\n  actual={pre_render_fields}"
    )

for evidence in [
    "size_of::<MetalGpuData>()",
    "offset_of!(MetalGpuData, generation)",
    "offset_of!(MetalGpuData, destroy_fn)",
    "size_of::<HotReloadPreRenderState>()",
    "offset_of!(HotReloadPreRenderState, generation)",
    "AEHotLoader_ImplementationRuntimeABI",
    "AEHotLoader_SetGeneration",
]:
    if evidence not in lib:
        raise SystemExit(f"missing hot-reload state/layout evidence: {evidence}")


# build.rs is one directory above src/lib.rs.
build_rs = pathlib.Path(sys.argv[1]).parent.parent / "build.rs"
build_text = build_rs.read_text()
global_flags = require(
    r'Property::AE_Effect_Global_OutFlags\((.*?)\),\s*Property::AE_Effect_Global_OutFlags_2',
    build_text,
    "PiPL Global OutFlags",
)
normalized_global = re.sub(r'\s+', '', global_flags)
expected_global = "OutFlags::UseOutputExtent|OutFlags::NonParamVary|OutFlags::DeepColorAware|OutFlags::CustomUI"
if normalized_global != expected_global:
    raise SystemExit(
        f"PiPL Global OutFlags changed; reinstall/restart required: expected={expected_global} actual={normalized_global}"
    )

# Inspect the complete host registration region.
region_match = re.search(
    r'let\s+mut\s+out_flags2\s*=\s*(.*?);\s*if\s+target_os\s*==\s*"macos"\s*\{(.*?)\}\s*pipl::plugin_build',
    build_text,
    re.DOTALL,
)
if not region_match:
    raise SystemExit("missing PiPL OutFlags2 setup")
flags2_region_text = region_match.group(1) + region_match.group(2)
actual_out2 = re.findall(r'OutFlags2::([A-Za-z0-9_]+)', flags2_region_text)
expected_out2 = ["SupportsSmartRender","SupportsQueryDynamicFlags","FloatColorAware","SupportsThreadedRendering","SupportsGetFlattenedSequenceData","SupportsGpuRenderF32"]
if actual_out2 != expected_out2:
    raise SystemExit(
        f"PiPL OutFlags2 changed; reinstall/restart required: expected={expected_out2} actual={actual_out2}"
    )

print(
    f"hot-reload state: PASS StateABI={impl_abi} "
    f"params={len(params)} grid_wire={wire_version} gpu_generation=present"
)
