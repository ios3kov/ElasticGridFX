# Persistent viewer grid — feasibility gate

Original research: 2026-09-29, target AE 25.6 arm64. Resumed 2026-10-03 under
the next-update request, now covering macOS Apple Silicon and Windows x64.
Status: FEASIBILITY OPEN; not implemented and not PASS. Historical deferral
applied to Stage 9/10 only. See feature-backlog.md U7 for the active requirement.

Preserve the current requirements below. The supplied
GridWarp.aex (SHA-256 d26054ae75b0561a4d391a3d8866212f922239754d9cc7d658a4df4e727db91e)
contains Visualization / Enable Visualization / stroke color, width and opacity
strings. A sibling macOS bundle contains render symbols accepting colors and
floats. These are leads, not proof of its drawing mechanism or preview/export
exclusion. Revisit behavior before asserting a separate module is necessary;
do not copy proprietary code or licensing routines.

## Confirmed requirements

- PV-1: per-effect "Show grid without selection" switch, default off, preserving
  existing saved parameter IDs and behavior.
- PV-2: when on and stopped, show the same thin transformed guide grid even when
  its effect/layer is deselected. When off, retain ordinary selected-effect UI.
- PV-3: with Show Grid enabled, remain visible during RAM Preview playback.
  This is the user's latest explicit answer on 2026-10-03, replacing the earlier
  stopped-preview-only answer. Never draw into output or cached pixels
  (including nested comps, render queue and aerender). When off, no persistent
  playback overlay is requested.
- PV-4: passive grid never consumes pointer events or changes the selected-tool
  cursor. Keep the existing interactive overlay only for the selected effect.
- PV-5: handle multiple instances, comp/view changes, deletion, undo, reopen and
  render cancellation without stale overlays or modifying unrelated project data.

## Evidence and decision

The current code uses PF_Cmd_EVENT/PF_Event_DRAW and the drawing context supplied
by the host. Adobe's Effect UI documentation ties Custom Comp UI to the selected
effect; a checkbox in draw_viewer cannot create callbacks after deselection.
Do not retain a DRAWBOT context beyond its callback or call it from Render/idle.

Pinned after-effects-sys 0.4.0 bindings were inspected:
- AEGP_ItemViewSuite1.GetItemViewPlaybackTime provides preview state, but no drawing
  surface; polling this does not solve persistent drawing by itself.
- AEGP_RegisterSuite5.RegisterInteractiveArtisan and QueryXForm draw callbacks
  exist, but belong to a separate interactive renderer integration, not an additive
  per-effect overlay hook. Compatibility/coexistence on AE 25.6 is NOT_RUN.
- New GuideSuite is documented for AE 26.0+ (v2 26.2+), outside this target. Guides
  are axis-oriented project data and do not provide this custom transformed grid.
- BlitHook/Mercury Transmit targets external video output, not the requested viewer
  overlay; it is not an established solution here.

Burning the grid into pixels, auto-selecting the layer, adding guide layers, using
private AE view internals or silently changing composition renderer would change
the agreed behavior. None is implemented or authorized by this research result.

Next work: isolated public-API feasibility research/prototype within the authorized
update. Prove deselected-view callbacks and playback lifecycle before exposing
the switch. Current headers have not established such a hook; selected-effect
draw callbacks alone do not satisfy PV-2/PV-3. Do not expose a nonfunctional switch.
This is not a claim that all possible implementations are impossible.

## Sources inspected

- https://ae-plugins.docsforadobe.dev/effect-ui-events/effect-ui-events/
- https://ae-plugins.docsforadobe.dev/effect-ui-events/PF_EventExtra/
- https://ae-plugins.docsforadobe.dev/aegps/aegp-suites/ (Item Views, Guides, Register)
- https://ae-plugins.docsforadobe.dev/artisans/artisan-data-types/
- https://ae-plugins.docsforadobe.dev/intro/what-can-i-do/

Research/source review PASS within this narrow scope. Runtime feasibility and all
PV acceptance tests NOT_RUN. User's decision to perform visual QA does not make
these technical acceptance conditions PASS. No release claim.


## Isolated PREVIEW callback probe — 2026-10-03

SDK25.6 AE_EffectUI.h:79/89 declares PF_CustomEFlag_PREVIEW=8 and
PF_Window_PREVIEW=3. Exact after-effects0.4.0 CustomEventFlags includes PREVIEW,
but WindowType omits PREVIEW/NONE; its conversion would panic for these contexts.
An opt-in preview-overlay-probe build registers the public flag and records raw
callback/window/time observations only. It guards null contexts, avoids wrapper
conversion for unknown windows, and never retains/draws into a preview context.
At this historical checkpoint ordinary builds did not register PREVIEW or compile this writer.
The later Dev8 gesture correction below supersedes registration only. Logs use an
exclusive private per-process folder, bounded4096 records and Build ID. Full log
or missing log cannot prove absence of callbacks. Probe version0.9.4 Dev3.
No Show Grid checkbox or production drawing hook is claimed. Selected, deselected
and playback phases need actual host observations; compilation is not feasibility
PASS. Negative observations cover only this public flag in the tested host.


### Runtime result — tested public flag only

35a9f3c / EGFX-7962a70953672d95f7328d1a, 0.9.4 Dev3 probe, AE25.6x101 arm64:
loaded UUID/path identity PASS. Actual stopped selected/unselected and RAM
Preview selected/unselected phases observed through Cua.1711 bounded records,
no PREVIEW window callbacks; after deselection zero comp DRAW events, seven
Layer DRAW events from the separate Layer view. Layer callbacks are not an
additive Composition overlay. The raw log is retained/hash-bound in
outputs/update-094-preview-probe-mac/feasibility-result.json. This rejects this
flag as a demonstrated PV-2/PV-3 route in this host; it does not prove universal
impossibility or transfer a Windows result. UI/output drawing was unchanged.
After the experiment the ordinary new0.9.4 Dev1 build
EGFX-2b54838b4bf29f6a45243d07 was installed from35a9f3c; loaded identity and exact
current .2-frame parity PASS. No old released plugin was restored. Show Grid
remains feasibility OPEN, no fake checkbox/baked guide pixels are shipped.


## Later ordinary gesture registration correction — Dev8

An isolated comparison found that AE25.6 delivered selected-effect Comp clicks
and drags when PREVIEW registration was added; the same bounded logger without
that flag saw cursor/draw/idle, but no click/drag and unchanged pixels. Ordinary
Dev8 now registers PREVIEW and rejects unsupported/null/PREVIEW contexts before
wrapper conversion. Its actual native guide drag/Undo/Redo/save-reopen passed
without a diagnostic writer. [Exact candidate](live-influence-mac-dev8-2026-10-03.json).
This does not establish an unselected or playback drawing surface: no guide pixels
are rendered, and Show Grid remains OPEN. The earlier negative overlay observation
retains its original source scope; do not reinterpret input delivery as overlay proof.

## SDK route recheck after Dev12 native range acceptance

2026-10-03: rechecked the Adobe SDK guide Effect UI & Events and AEGP Suites
against local SDK25.6 AE_EffectUI.h and AE_GeneralPlug.h. Custom Comp UI remains
specified for a selected effect. ItemViewSuite1 exposes playback time, but no
additive drawing surface or current view transform. Current online ItemViewSuite2
adds only guide visible/snap/locked controls, documented for AE26.0+; this does
not supply the missing custom drawing route or expand the target25.6 headers.
The previously tested public PREVIEW flag route remains insufficient for PV-2/3.
This is a bounded route assessment, not a proof that every implementation is
impossible. The complete Show Grid behavior is still a functional blocker; do
not convert successful selected Dev12 range feedback into Show Grid acceptance.

Sources rechecked: [Effect UI & Events](https://ae-plugins.docsforadobe.dev/effect-ui-events/effect-ui-events/)
and [Item Views](https://ae-plugins.docsforadobe.dev/aegps/aegp-suites/#item-views).
No new unsupported host route, OS overlay, auto-selection or renderer change
was implemented. No package/installer work resumed while this feature is open.

## Additional route boundary — 2026-10-04

The user-confirmed live selected slider feedback does not satisfy PV-2/3.
SDK25.6 AE_GeneralPlug.h3773 exposes GetPlatformWindowRef, viewport scale and
translation only with PR_RenderContextH under interactive artisan information.
QueryDrawProcs/PrepareForLineDrawing at4061+ require PR_QueryContextH and are
artisan-provided camera/light handle drawing functions; they are not a general
PF-effect registration for an existing viewer. PR_Public.h explicitly defines
artisans as plugin renderers and requires render_func. GeneralPlugPanels.h
provides a native view for an owned custom panel, not the existing Composition
view/transform. Merely acquiring these suites supplies none of those contexts.

[Adobe UI Callbacks](https://ae-plugins.docsforadobe.dev/effect-ui-events/ui-callbacks/)
requires current PF_ContextH for source/frame transforms, including zoom. Those
borrowed callbacks are not a transform query after the host stops delivering the
effect's draws. [Artisan documentation](https://ae-plugins.docsforadobe.dev/artisans/artisan-data-types/)
describes the renderer-owned draw context. A cached prior transform or guessed
native window location cannot meet pan/zoom/playback/multiple-view acceptance.

This additional static assessment excludes these APIs as an established additive
route for the agreed target; no new runtime feasibility result is claimed.
The user explicitly chose on2026-10-04 to retain Show Grid in this update and
continue research. PV-1–PV-5 remain required; no deferral is pending. No checkbox,
renderer swap or OS-specific overlay is shipped as a substitute.


## Retained scope and new route evidence — 2026-10-04

[Hash-bound research record](show-grid-research-2026-10-04.json) distinguishes a
new native read-only query from reanalysis of the historical Dev3 callback log.
The guarded Dev16 fixture returned one Composition view zoom1.26049881944922
through `app.activeViewer.views[0].options.zoom`. This proves that query in the
owned Mac fixture only. The documented View/Viewer/ViewOptions inventory does
not establish image bounds or pan offset; a cached/guessed transform remains
insufficient. Current Cua AX observation exposes window/menu only. No OS helper,
new permission, UI selection or preference change was used for this query.

Between the historical deselect and select script timestamps, the final bounded
log has9 records:7 Layer DRAW, one Comp NEW_CONTEXT and one Comp ACTIVATE;
zero Comp DRAW and zero Comp IDLE. The interval includes transitions. This is
a scoped negative observation, not proof for every AE session. It supplies no
Comp idle context for a safe redraw experiment. SDK25.6 AppSuite's
PF_InvalidateRect requires a current effect context during a **non-draw** event.
Calling it with a retained context or treating a Layer DRAW as a Comp context
would violate that contract. No such experiment was built.

Async Manager is also not a new drawing registration: exact SDK25.6
AE_GeneralPlug.h5449 describes its PF_Event_DRAW-specific manager, acquired from
the effect custom UI context. It manages frame requests for an existing UI draw.

Next research gate: establish both an additive drawing registration and a
current image-to-screen transform after deselection/during playback, before
production implementation. A stopped selected grid or zoom query alone cannot
pass this gate. Require pan/zoom, multi-view, lifecycle and export/cache exclusion
proof on Mac and Windows. Show Grid remains required and unimplemented.

Sources: [Redrawing and Drawbot](https://ae-plugins.docsforadobe.dev/effect-ui-events/custom-ui-and-drawbot/),
[ViewOptions](https://ae-scripting.docsforadobe.dev/other/viewoptions/),
[View](https://ae-scripting.docsforadobe.dev/other/view/),
[Viewer](https://ae-scripting.docsforadobe.dev/other/viewer/); exact target headers
and native outputs are hashed in the research record.


## GridWarp1.0.0 feature reference — 2026-10-04

The newly supplied Archive.zip contains the same Windows binary as the earlier
reference and a universal Mac bundle1.0.0. The already-installed Mac executable
matches the supplied archive SHA-256; no install/replacement was needed. Exact
loaded-memory UUID verification is NOT_RUN, so no stronger loaded-artifact
identity claim is made. [Reference record](gridwarp-visualization-reference-2026-10-04.json).

A separate owned319x241 solid in AE25.6x101 confirms Enable Visualization and
color/width/opacity controls. With it enabled, blue grid lines remain after
deselecting the layer/effect and during actual Preview playback (Cua red
playhead/Playing indicator). Trial diagonal watermark remains; no activation or
licensing bypass was performed.

The grid also appears in an actual PNG produced by saveFrameToPng. Comparing the
same.2second frame with only Enable Visualization0→1 gives6516changed RGBA
pixels; blue-threshold pixels rise10→4025. The enabled exported PNG visibly
contains the blue vertical/horizontal grid. This directly fails PV-3's
never-in-output-pixels condition for this tested export path. Render Queue,
aerender and Windows behavior are NOT_RUN, not inferred from this test.

Internal visibility implementation is UNKNOWN. Render symbols accepting colors
and floats are only a lead; no recovered proprietary routines are copied. The
reference demonstrates persistent visibility, but has not established the
additive export-free drawing route needed by our contract. Retain Show Grid in
this update; no automatic scope relaxation follows from this finding.
