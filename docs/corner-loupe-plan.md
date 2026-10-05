# Corner loupe — 2026-10-05

Scoped addition to the four-mode update under rules8.0.0. Preserve prior mode,
Demo, licensing and installer obligations. User confirmed: the loupe itself is
centered on the corner, with a target at its center. Applies to Mac and Windows.

- L1: Only while dragging an editable corner in Surface mode or Perspective;
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

Candidate checkpoint: implementation source000ff91; ordinary Dev64. Mac exact
package/signature and loaded image PASS (EGFX-38c123bf765bbee9c1bc241a).
Windows CI37291232120 PASS and downloaded AEX identity/checksum verified
(EGFX-448748c343eede5cd2db801a). Native visual acceptance remains NOT RUN;
manual check requested in owned loupe.aep. Mac source CI37291232084 PASS.
Local release build succeeded but objcopy debug-info stripping warned about a
missing libLLVM.dylib in Rust toolchain; recorded, not hidden or counted as UI
failure. No release-ready claim. No new installer payload adoption yet.

Native Point path correction: AE may consume native Point gestures before custom
CLICK/DRAG. Append SUPERVISE to the same four existing Point parameters; observe
UserChangedParam without setters or key writes. Track that owning instance/corner
only while the left button is down in the foreground AE process. Read-only AppKit
(main-thread) / Win32 button queries; no OS capture, input synthesis, permission
prompt, render call or stored project data. Native release clears at next DRAW.
Dev64 source/native CI remains PASS, but native Point loupe coverage is incomplete;
replacement candidate Dev68 is under verification, not yet installed or accepted.

Dev68 observes same-frame changes to exactly one corner during a foreground
left-button gesture, covering native live previews before supervision. Scrubbing
and multi-corner plane motion do not start a lens. Frame sampling uses three
affine viewer callbacks per DRAW instead of one callback per lens pixel.
93 Rust tests and all-targets Clippy PASS; 15 host-contract Python checks PASS.
Evidence: outputs/corner-loupe-dev68. Native acceptance still NOT RUN.

Dev68 native visual check USER-REPORTED FAIL: no loupe while dragging.
Loaded exact Dev68 identity PASS does not imply feature acceptance. Add bounded
probe-only DRAW stages (editable corner, active gesture, drawing result), without
reading the DRAW event union as mouse input. Diagnose before another ordinary
replacement; native acceptance remains FAIL until reproduced and fixed.

Probe70b native evidence: button flags7 (UI thread, AE active, left button
down), same viewer/time, repeated single-corner changes; PF effect_ref owner
frequently differs between successive DRAW callbacks. A stale supervised
owner blocks observation from restarting a gesture. Dev72 fix uses the unique
ID of an existing effect parameter stream via AEGP_GetUniqueStreamID; dispose
stream/effect refs in each callback, retain only the ID. Probe output is bounded
and now excludes idle/cursor polling. Ordinary render math remains untouched.

Probe74 USER-REPORTED FAIL: lens appeared, showed no useful pixels, then AE
reported PF_GetContextAsyncManager: no active async manager in PF_Context.
SDK25.6 AE_EffectUI.h:419 explicitly describes reserved_job_manageP as Effect
pane custom UI. Dev76 removes manager acquisition from Comp/Layer DRAW.
Effect Controls DRAW obtains a current composited/processed frame asynchronously,
checks it in immediately and retains only an owned U8 pixel copy (max64MiB),
keyed by stable stream ID, viewer, time and project timestamp. Viewer DRAW
samples that copy; pending pixels do not masquerade as a successful checker lens.
Only Flat/Perspective prefetch; renderer unchanged. Closed contexts release cache.
Hidden Effect Controls before any prefetch remains an unresolved coverage case;
do not claim full loupe acceptance or silently drop it.

Probe78 installed 2026-10-05, source8831b2a, build
EGFX-8d3772b6e6492719146cd85c. Package/signature, atomic replacement,
95 Rust tests and Clippy PASS. Exact loaded-image UUID/path PASS in
AE25.6 PID23997. Saved owned Probe74 scene reopened; Effect Controls
shown with F3, no initial error. Native drag/image/release acceptance pending.
Probe74 generated repeated verification dialogs; saved scene and closed
project before verified PID21494 SIGTERM and safe replacement. No force-kill
or deletion of user scenes. Evidence in outputs/corner-loupe-probe78.

Probe78 Perspective drag USER-REPORTED PASS: image under the corner is
visible, lens disappears on release, no repeated verification error. User
requested hiding the hand cursor while the lens is visible. Dev80 implements
transparent local cursor images on Mac/Windows without global visibility
counters; restore on release/cancel/deactivate/context close and failed lens.
Rendering and saved parameters remain unchanged. Native cursor acceptance
and Flat/hidden-panel coverage remain pending.

Dev80 Windows CI37300965301 FAIL at portable native fixture compilation:
explicit LoadCursorW received the ANSI IDC_ARROW macro without UNICODE.
Fix uses MAKEINTRESOURCEW(32512), independent of project character-set
flags. Mac installed artifact remains bound to source2c2c931; Windows
replacement CI is separately verified.

Dev80 native loupe/cursor check USER-REPORTED PASS (requested Flat and
Perspective, user answered all okay). Cursor no longer interferes, target visible,
release restores cursor. The user then reported Comp/Layer mode both confined
to layer bounds and requested logical menu order; these are new active issues,
not loupe acceptance failure. Hidden-panel coverage still not established.

2026-10-05 caption update: Surface mode is the former Flat mode. Internal ordinal2
and loupe behavior unchanged; historical Flat evidence remains artifact-specific.

## Dev124 incident mitigation candidate

A user-reported whole-Mac RAM Preview hang blocks promotion. Inactive Effect-pane
DRAW must no longer request a fresh loupe frame for each playback time or globally
refresh windows on warm completion. The candidate permits one initial owner/time
warm request and active corner gestures. Unit policy coverage PASS; native request
lifecycle, playback stability and after-scrub hidden-pane loupe coverage NOT RUN.
One initial warm request still exists; do not claim zero background host work or
a proven root cause. Earlier hidden-pane acceptance does not cover this policy.
