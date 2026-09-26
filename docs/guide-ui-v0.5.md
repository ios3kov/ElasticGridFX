# Interactive guide state — v0.5

## What changed

The After Effects host now owns a keyframeable arbitrary-data parameter named **Grid Positions**.
It stores the clean-room guide topology (normalized column/row line positions plus pin bits) and is
serialized/interpolated by the host binding.

The Comp/Layer custom UI now:

- draws the layer border and all internal column/row guides;
- transforms guide endpoints through AE's Layer/Comp/Frame coordinate callbacks;
- hit-tests internal guides in screen space;
- drags a selected guide in layer space;
- calls the C++ `AxisGrid::dragElastic` implementation through the C ABI;
- applies Tension Radius, Falloff, Elasticity Strength and Min Line Spacing during the drag;
- writes the resulting Grid Positions parameter with `CHANGED_VALUE`, so AE owns undo/keyframe state;
- requests immediate viewer redraw/rerender during dragging.

Changing Columns/Rows resamples the current grid shape instead of throwing it away. If topology changes
between arbitrary-data keyframes, interpolation deliberately steps at 50%; same-topology keys interpolate
all guide positions continuously.

## Rendering path

`eg_render_frame` now accepts optional host-supplied guide arrays. When present, those guides are validated,
loaded into the same `AxisGrid` model used by the drag logic, then evaluated for Wave and converted into
inverse LUTs. This closes the previous v0.4 gap where elasticity controls existed in UI but could not affect
rendered output.

## Test gate

C++ core/bridge release tests and ASan/UBSan pass in the development container. The macOS preflight now also
runs `cargo test --release` for the AE host before the `.plugin` bundle is produced.

A real AE/macOS runtime pass is still required before declaring the first user test binary ready.
