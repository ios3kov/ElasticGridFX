# Persistent viewer grid — feasibility gate

2026-09-29. Stage 9. Target: AE 25.6 arm64. Status: BLOCKED pending architecture
choice; no parameter, runtime code or installed payload changed for this request.

## Confirmed requirements

- PV-1: per-effect "Show grid without selection" switch, default off, preserving
  existing saved parameter IDs and behavior.
- PV-2: when on and stopped, show the same thin transformed guide grid even when
  its effect/layer is deselected. When off, retain ordinary selected-effect UI.
- PV-3: hide during RAM Preview playback, restore on stop. Never draw into render
  output or cached pixels (including nested comps, render queue and aerender).
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

Next decision: authorize a separately isolated interactive-overlay feasibility
prototype (no install or compositor replacement), or defer persistence and retain
the already accepted selected-effect overlay. Do not expose a nonfunctional switch.
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
