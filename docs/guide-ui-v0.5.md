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

### Stage 9 viewer affordances — 2026-09-29

Current request supersedes the square-marker design: thin guides with elongated
grips, hover-only hand and visibility on light/dark/middle-gray backgrounds.

- UI-1: horizontal guides have a centered 48-frame-unit grip; vertical guides
  have two 24-unit grips flanking a 24-unit central gap, like the supplied reference.
  Geometry follows transformed endpoints; short guides under 80 units and grids
  over 34 lines suppress grips. Pinned guides have no draggable grips.
- UI-2: hover uses the same 9-unit line pick tolerance as clicks (including the
  vertical gap); outside a draggable guide AdjustCursor is ignored, allowing AE
  to choose its selected-tool cursor. Captured drag retains the closed hand.
- UI-3: opaque white strokes over black outlines: lines 1/3 units, grips 3/5.
  This is dual-tone contrast, not framebuffer-dependent inversion. The pinned
  DRAWBOT Surface API exposes no destination-invert operation; inversion alone
  also loses contrast at middle gray. No framebuffer reads or render changes.

Baseline: 7c421ab installed in AE 25.6 arm64; user reports preliminary success.
That feedback does not accept the new overlay. New candidate acceptance requires
white/black/50%-gray/detail backgrounds, 25/100/200% zoom, rotated layer,
hover off/on line with Selection/Text/Pen tools, drag/release/undo, deselection,
exit/re-entry, pinned/dense guides and absence of invalid-cursor dialogs.
Source unit tests cover split/gap geometry, rotated grip length, short/dense/
degenerate guides and distance picking. Real AE gate remains NOT_RUN for this change.

On macOS a main-thread-only AppKit shim selects openHandCursor/closedHandCursor;
AE is told CUSTOM only using AdjustCursor, never via App suite or the click/drag
union. No push/pop stack, dependencies or render changes.
Release, drag errors, invalidated topology, deactivation, context close and exit
clear dragging. Non-Mac uses native Hand/Pan as a fallback, not a verified exact
closed-hand appearance. AE owns delivery of active effect UI events.

Acceptance still requires live AE: selected/deselected effect, hover/click/drag/
release, exit/re-entry, cancellation, multiple effects, pinned guides, zoom
25/100/200%, and drag undo. Source tests/build cannot close this visual gate.
References: Apple NSCursor openHand/closedHand and
https://ae-plugins.docsforadobe.dev/effect-ui-events/PF_EventUnion/ (ignore
AdjustCursor to leave cursor ownership to AE). PF_SetCursor(NONE) caused a real
AE 25.6 modal error; it is not used. Installation remains separately gated.

`eg_render_frame` now accepts optional host-supplied guide arrays. When present, those guides are validated,
loaded into the same `AxisGrid` model used by the drag logic, then evaluated for Wave and converted into
inverse LUTs. This closes the previous v0.4 gap where elasticity controls existed in UI but could not affect
rendered output.

## Test gate

C++ core/bridge release tests and ASan/UBSan pass in the development container. The macOS preflight now also
runs `cargo test --release` for the AE host before the `.plugin` bundle is produced.

A real AE/macOS runtime pass is still required before declaring the first user test binary ready.
