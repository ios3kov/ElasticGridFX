# Corner loupe — 2026-10-05

Scoped addition to the four-mode update under rules8.0.0. Preserve prior mode,
Demo, licensing and installer obligations. User confirmed: the loupe itself is
centered on the corner, with a target at its center. Applies to Mac and Windows.

- L1: Only while dragging an editable corner in Flat mode or Perspective;
  no loupe on grid-line drags, Comp mode or Layer mode.
- L2: Circular 129 UI-unit lens, 3x current viewer scale, center follows the
  evaluated corner. Black/white target arms leave the exact center unobscured.
  Lens remains centered even beside a viewer edge; viewport clipping is accepted.
- L3: Magnify the composited frame in Comp viewer, processed layer in Layer
  viewer, including content below the corner for alignment. Transparent pixels
  use a checkerboard. No screen capture, OS permissions or native floating window.
- L4: UI-only DRAWBOT image; no effect pixels, saved values, extra keyframes,
  watermark, Preview/export or renderer branch. Release, failed drag,
  deactivation and context closure clear the active gesture.
- L5: AE13.5+ AsyncManager supplies frames without synchronous UI rendering.
  Same purpose ID lets AE cancel superseded requests and request DRAW when ready.
  Pending/unavailable pixels show checkerboard/target without blocking editing.
  No retained context, pixel world, receipt or render options across callbacks;
  only an opaque effect-address identity plus window/corner index tracks the
  single UI gesture. A different instance/window never draws this lens.

SDK authority: AE25.6 AE_GeneralPlug.h:5448–5475, EffectCustomUISuite2 and
HistoGrid_UI_Handler.cpp:178–206; PF_OutFlag2_CUSTOM_UI_ASYNC_MANAGER in PiPL
and GlobalSetup. No synchronous render or custom callback lifetime management.

Verification: bounded raster sampling and target/alpha tests; Rust/Clippy;
Mac Flat/Perspective native drag, tracking/release/Undo, visual pixel orientation;
Windows MSVC build and host handoff. Until recorded, native visual acceptance is
NOT RUN. Existing four-mode Dev60 Mac pixel/animation/migration evidence remains
valid only for that exact candidate, not for the new loupe artifact.

Source verification: 92 Rust tests and Clippy all-targets with warnings denied
PASS; 22 host/package/version Python checks PASS. Static audit completed with
one unchanged fixture false positive (test_target_ae_acceptance.py:54). No
static result certifies UI acceptance. Logs retained in outputs/corner-loupe.
