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
