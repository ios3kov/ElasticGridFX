# Corner loupe — 2026-10-05

## Current source decision — 2026-10-08

Latest explicit user instruction and supplied photo supersede the earlier L3
exclusion of FSTR: magnify the current evaluated composition. A transparent
area must remain transparent (shown with a checkerboard); a visible layer must
appear. Include all evaluated visible layers and effects, including FSTR, from
the fixed frame at press time. Do not show original-layer cells where the current
composition is transparent. Earlier upstream-only Dev187 is a diagnostic source
experiment, not a shipping implementation of this decision.

Ordinary Dev188 uses the existing asynchronous composition-item route, with
explicit full-resolution U8 ARGB straight-alpha request and owned premultiplied
copy for lens compositing. Layer viewer retains its evaluated layer view. One
immutable frame per gesture, overlay-only corner movement, target/cursor policy,
Undo/keys and release rendering remain required on Mac and Windows. No scratch
composition, effect flag mutation, per-movement render or diagnostic logger.

Current acceptance boundary: ordinary188 source0cba0dd is installed; exact
canonical loaded AE25.6 identity, source134/Clippy/host17 and artifact checks PASS.
Tiny multilayer Perspective setup and actual native Point readback/key retention
PASS. Evaluated-composition lens contents, alpha/background, freeze, cursor and
current188 Undo remain NOT_RUN. Automated short inputs omit pre-press hover
required by corner_ownership and cannot prove the custom frozen route; no lens
was observed. Do not label launcher exit0 or native Point readback as loupe PASS.
One timed manual gesture in a fresh owned scene remains the next acceptance step.

Original-idle-cache r2 reached the fixed256MiB growth bound before input; same
artifact tiny idle-off r3/r5 trials did not hit a limit during45s. This isolates
an idle-render confound, not the original hang root cause. All changes to the
single idle-cache key are temporary test conditions, restored with the full
original25.6 preference hash and exact9 excluded component tree hashes verified.
Heavy-scene/RAM Preview remains NOT_RUN. Current Windows MSVC/AE NOT_RUN; no new
source pushed. Private evidence: outputs/loupe-dev188/native-composition*/ and
STATUS.md. Earlier Dev187 original-cell/no-hand/no-alert user PASS remains scoped
to its upstream-only diagnostic source and does not close current L3.

Historical source decisions below are retained as dated evidence; this section
and L3 govern the current source scope.

2026-10-07 source prerequisite prepared locally: nondefault
`loupe-upstream-probe` Dev186 requests the affected layer before FSTR through
SDK LayerRenderOptionsSuite2; layer-space sampling uses the same native/view
projection as corner placement. No project flag mutation, no additional request
per mouse movement, unchanged ordinary composition source. This experiment
intentionally omits lower-layer composition and is NOT the accepted product fix.
Probe/default Rust133 each, probe strict Clippy and host source17 PASS.
First compile used incorrect Fixed conversion names and failed; corrected against
the installed wrapper before successful checks. Native source/coordinate coverage
Surface + Perspective (2D/3D, nonidentity layer transform) NOT_RUN. No installation
or publication. Next: bounded single-layer upstream pixel proof, then a separate
full-composition exclusion route and multilayer acceptance; no inferred PASS.
Evidence: outputs/loupe-upstream-{tests,default-tests,clippy,contracts}.log.

2026-10-07 confirmed source scope: retain the affected layer's original cells
without FSTR distortion, together with visible background/lower layers. User
explicitly chose retaining original cells in the single-layer fixture. Do not
omit the affected layer or reduce the accepted composition to layer-only pixels.
Full-composition exclusion remains implementation OPEN, not a pending question.

SDK25.6 source audit: RenderOptionsSuite4 (AE_GeneralPlug.h:5128-5240)
provides time, ROI, resolution, matte/channel order, guide layers and quality;
no documented lower-layer range/excluded-layer selector found there.
LayerRenderOptionsSuite2 NewFromUpstreamOfEffect (:5260-5264) excludes this
and later effects but requests that layer, not the composited lower stack.
CanvasSuite RenderLayerPlus uses an Artisan render context; ordinary effect UI
has no such context. Guide authority:
https://ae-plugins.docsforadobe.dev/aegps/overview/
https://ae-plugins.docsforadobe.dev/aegps/aegp-suites/
Do not substitute an upstream-only frame for accepted composition/background
scope. Do not temporarily toggle user effect/layer flags during an async request:
that mutates the displayed project and has unresolved request snapshot lifetime.
Next research must preserve project state, native composition blending/mattes,
frame-request bounds and Mac/Windows parity. No new host experiment dispatched
for this unresolved source design. Cursor source fix e5eec9e is independently
verified by Rust133/Clippy/source17, not yet installed/native accepted.

2026-10-07 current user acceptance on ordinary Dev184: colored lens, held picture
and Undo PASS for the owned color fixture. New accepted source requirement:
exclude FSTR rendering in the lens while moving corners; retain the original
affected-layer image and visible lower/background layers. Do not replace this
composited scope with layer-only pixels. New cursor acceptance:
no hand may flash inside the lens throughout any corner gesture. Source correction
preserves the active transparent cursor instead of setting hand on every DRAG;
Mac/Windows use the same shared UI route, native recheck pending.

2026-10-06 incident: ordinary Dev152 stationary-press repair does not close drag
acceptance. User reports jerky dragging and then an AE-only hang. Exact native
sample/incident remain local in outputs/corner-clean-dev152/drag-profile;
[current hold](STATUS.md) records evidence and unknown attribution. Diagnostic
Dev156 is a nondefault source-disabled probe: no warm/gesture frame request,
no file logger, unchanged ordinary rendering/keys/UI state machine. The lens
intentionally has no pixels in this probe; it is not the requested product or
a workaround to ship. Planned comparison is one short corner gesture with Undo,
no RAM Preview, immediate stop/owned-process shutdown if responsiveness fails.
Do not repeat the15-second drag. First compile of the isolated C++ sanitizer
matrix omitted CpuRenderer.cpp (RenderCancelled definitions) and failed to link;
corrected source list compiled and existing depth/stride/edge/ROI/cancellation/
concurrency matrix passed under ASan/UBSan.116 probe Rust tests, strict probe
Clippy and16 host source contracts PASS; native hypothesis test NOT RUN.

Scoped addition to the four-mode update under rules8.0.0. Preserve prior mode,
Demo, licensing and installer obligations. User confirmed: the loupe itself is
centered on the corner, with a target at its center. Applies to Mac and Windows.

- L1: Only while dragging an editable corner in Surface mode or Perspective;
  no loupe on grid-line drags, Comp mode or Layer mode.
- L2: Circular 129 UI-unit lens, 3x current viewer scale, center follows the
  evaluated corner. Black/white target arms leave the exact center unobscured.
  Lens remains centered even beside a viewer edge; viewport clipping is accepted.
- L3: Magnify the current evaluated composition in Comp viewer, including FSTR,
  visible layers and effects with native transforms/compositing. In Layer viewer,
  magnify its current evaluated layer view. Freeze that source at press time;
  transparent pixels use a checkerboard rather than original-layer cells. No
  screen capture, OS permissions or native floating window.
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

Dev132 closes a demonstrated multi-owner hole in the Dev124 warm policy. A fixed
32-owner history prevents alternating effect rows from rearming playback requests.
Completion is remembered independently of the one retained frame copy. No idle
request after capacity; gestures remain allowed. Original-symptom host and
after-scrub hidden-pane acceptance still pending; Demo OFF and keys unchanged.

Dev132 short non-playback user check PASS for Undo, release, and after-scrub
hidden-pane image. Stationary press before any Point motion is still missing.
Dev136 adds a foreground UI pointer-qualified, hit-tested hover anchor scoped
to effect/view/time. It starts the lens on a held corner press without changing
Point values; existing changed-corner fallback retained. No idle frame-request
policy expansion. Stationary-press host acceptance NOT RUN.

Dev136 repeated press acceptance FAIL: only first press works, severe slowdown
reported. Dev140 caps async completion/refresh to one fixed frame per gesture;
pending polls keep original request identity, terminal errors finish it too.
Lens samples the gesture snapshot at the moving target; render pixels unchanged.
Release clears active work before new hover arming; same-corner supervision
cannot rearm work. Tests cover repeat, completion/error and changing host stamps.
Native acceptance NOT RUN; original Mac-hang attribution remains UNCONFIRMED.

Dev140 repeated stationary press still FAIL in native user check. Dev144 includes
a FAIL-before-fix test invoking the actual shared hover-start path three times
without new hover events or coordinate changes. Native release now preserves
the hit-tested scope/pointer anchor, while lifecycle cancellation still clears
it. Active single-corner movement follows the pointer anchor for later repress.
Gesture frame completion/budget unchanged. Native acceptance remains NOT RUN.

## Native press redraw, Dev152 / diagnostic155

Probe151 exact sourcee5ddc05/BID EGFX-f909f058d6c78b4fe8902137 captured
134 bounded local records. Initial press: start27, DRAW28 after5ms, lens success29.
Repress: start40 with scoped hover accepted (predicate63), active42, no viewer
DRAW until native movement supervision43; DRAW44 after841ms. Thus the missing
transition is not loss of the hover/active state in this capture. Raw CLICK/DRAG
are consumed by AE native Point controls. Native acceptance still FAIL.

SDK25.6 authority: AE_EffectUI.h:510 says UPDATE_NOW updates the view when using
PF_InvalidateRect; AE_EffectSuites.h:590-595 permits invalidation only during
a non-draw event, with the current context and optional null rect for whole view.
AppSuite wrapper0.4.0 matches the signature. Previous AdjustCursor start branch
returned UPDATE_NOW without invalidating the view. Dev152 calls invalidation
once when hover starts a gesture, before flags; repeated active cursor callbacks
return without invalidation. Context is callback-local, no pointer retained, no
timer, OS event injection, parameter/key write, renderer change, global refresh
or idle frame-request expansion. The original whole-Mac RAM Preview incident
remains OPEN/UNCONFIRMED; this trace does not establish its cause.

The source placement guard FAIL before fix; native before/after acceptance is
pending. Diagnostic155 adds transition509 recording invalidation success, with
the same bounded gesture log; it does not enable PREVIEW callbacks. Private
raw trace and project remain only under outputs/corner-press-probe151; public
source docs contain aggregate transition findings only.

2026-10-08 r7 update: ordinary188 manual Perspective composition/alpha/background,
frozen image and no hand/no alert USER_REPORTED_PASS. Memory acceptance remains
NOT_PASSED: guard terminated ownedPID2968 at44.5249s on272859208-byte growth;
no spontaneous crash established. Cleanup PASS. Current Undo readback NOT_RUN.
Next discriminating check is passive resource census on the same source: counts
of async polls/receipts/checkins and retained loupe/snapshot storage, with unchanged
45s/1.5GiB/256MiB limits. No shipping cache-setting change or guessed leak fix.
