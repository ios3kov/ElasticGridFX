#!/usr/bin/env python3
import pathlib
import sys

if len(sys.argv) != 3:
    raise SystemExit("usage: verify_smartfx_contract.py LIB_RS UI_RS")

lib = pathlib.Path(sys.argv[1]).read_text()
ui = pathlib.Path(sys.argv[2]).read_text()

required = [
    "fn checkout_smart_render_state(",
    "params.checkout(Params::GridState)",
    "params.checkout(Params::MinSpacing)",
    "params.checkout(Params::StretchEasing)",
    "params.checkout(Params::EasingDistance)",
    "params.checkout(Params::WaveAmplitude)",
    "params.checkout(Params::WaveFrequency)",
    "params.checkout(Params::WavePhase)",
    "params.checkout(Params::WaveSpeed)",
    "params.checkout(Params::WaveAxis)",
    "params.checkout(Params::EnableVisualization)",
    "params.checkout(Params::ColumnStrokeColor)",
    "params.checkout(Params::RowStrokeColor)",
    "params.checkout(Params::StrokeWidth)",
    "params.checkout(Params::Opacity)",
    "params.checkout(Params::EdgeMode)",
    "params.checkout(Params::Quality)",
]
for item in required:
    if item not in lib:
        raise SystemExit(f"missing SmartFX/original contract evidence: {item}")

for item in [
    "request.rect = full_source.into()",
    "request.preserve_rgb_of_zero_alpha = 1",
    "right: in_data.width().max(1)",
    "bottom: in_data.height().max(1)",
    "max_rect.union(&full_source)",
]:
    if item not in lib:
        raise SystemExit(f"missing original SmartPreRender contract: {item}")

if "rendered_canvas(in_data)" in lib[lib.find("ae::Command::SmartPreRender"):lib.find("ae::Command::SmartRender { extra } =>")]:
    raise SystemExit("SmartPreRender still uses downsampled canvas instead of full PF_InData dimensions")

if "ae::EventOutFlags::ALWAYS_UPDATE" in ui:
    raise SystemExit("drag path still sets ALWAYS_UPDATE; original returns event flags 0x9")
if "invalidate_rect(event.context_handle()" in ui:
    raise SystemExit("drag path still uses extra App-suite invalidation absent from original")
if "ae::EventOutFlags::HANDLED_EVENT | ae::EventOutFlags::UPDATE_NOW" not in ui:
    raise SystemExit("missing exact original drag event flags")

if "checkout_smart_render_dependencies" in lib:
    raise SystemExit("obsolete dropped SmartPreRender checkout helper still present")

smart_start = lib.find("ae::Command::SmartRender { extra } =>")
gpu_setup = lib.find("ae::Command::GpuDeviceSetup", smart_start)
if smart_start < 0 or gpu_setup < 0:
    raise SystemExit("cannot isolate SmartRender body")
smart = lib[smart_start:gpu_setup]

for forbidden in ["grid_snapshot(params)", "evaluated_params(params"]:
    if forbidden in smart:
        raise SystemExit(f"SmartRender illegally reads ordinary params: {forbidden}")
if "checkout_smart_render_state(params)" not in smart:
    raise SystemExit("SmartRender does not use render-time checked state")

print("SmartFX contract: PASS render-time checkouts + full-source pre-render + original drag flags")
