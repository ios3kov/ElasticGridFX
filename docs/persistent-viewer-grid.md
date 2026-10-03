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
