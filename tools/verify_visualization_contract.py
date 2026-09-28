#!/usr/bin/env python3
import pathlib
import sys

if len(sys.argv) != 4:
    raise SystemExit("usage: verify_visualization_contract.py LIB_RS FFI_H FFI_CPP")

lib = pathlib.Path(sys.argv[1]).read_text()
hdr = pathlib.Path(sys.argv[2]).read_text()
cpp = pathlib.Path(sys.argv[3]).read_text()

for item in [
    "visualization_enabled",
    "column_stroke_argb",
    "row_stroke_argb",
    "visualization_stroke_width",
    "visualization_opacity",
    "visualization_canvas_width",
    "visualization_canvas_height",
    "visualization_origin_x",
    "visualization_origin_y",
]:
    if item not in lib or item not in hdr:
        raise SystemExit(f"missing visualization ABI field: {item}")

for item in [
    "composite_visualization(",
    "coverage * opacity * color.a",
    "src_a + dst_a * (1.0f - src_a)",
    "guide * static_cast<float>(canvas_width)",
    "guide * static_cast<float>(canvas_height)",
    "p.visualization_origin_x",
    "p.visualization_origin_y",
]:
    if item not in cpp:
        raise SystemExit(f"missing recovered visualization behavior: {item}")

for item in [
    "params.checkout(Params::EnableVisualization)",
    "params.checkout(Params::ColumnStrokeColor)",
    "params.checkout(Params::RowStrokeColor)",
    "params.checkout(Params::StrokeWidth)",
    "params.checkout(Params::Opacity)",
]:
    if item not in lib:
        raise SystemExit(f"missing SmartRender visualization checkout: {item}")

print("visualization contract: PASS")
