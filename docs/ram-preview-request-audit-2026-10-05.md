# Frame-request and redraw audit — 2026-10-05

Scope: first-party product host-rust/src and src native bridges/renderer, source
674c984 (Dev124 mitigation). Includes SDK render request entry points, invalidation,
UI update flags, snapshots, overlays, expression bindings and retained caches.
Static inventory: outputs/ram-preview-hang-dev120/render-request-inventory.txt.
This is source evidence, not a live AE request count or hang root cause.

## Independent host frame requests

Only corner_loupe.rs uses RenderOptions/LayerRenderOptions and SDK
checkout_or_render_*_frame_async_manager. Surface/Perspective Effect-pane DRAW
previously reached this without a gesture and refreshed all windows on completion.
Dev124 bounds warm requests to one owner/time and makes completion refresh
gesture-only. Hidden-pane after-scrub loupe regression remains NOT RUN. No other
product parameter, Reset, range visualization, licensing or Demo code initiates
independent host frame render requests.

SmartPreRender checkout_layer at lib.rs1359 declares one full-source dependency;
SmartRender checkout_layer_pixels at1392 consumes it and checkin at1430 releases
it. These are separate phases of the same SDK render route, not two independent
plugin-requested renders. Upstream frames may need AE evaluation when unavailable
in host cache. Full input is deliberate for arbitrary warps and bicubic neighbor
taps; reducing it without ROI proof risks clipping/seams.

## Additional avoidable work to investigate

- lib.rs Columns/Rows UserChangedParam unconditionally ForceRerender despite
  reflow preserving deformation. With Show Grid ON density changes output pixels
  and rerender is needed. With Show Grid OFF/Demo OFF it is a likely avoidable
  image invalidation. Native redraw behavior must be preserved and verified.
- ui.rs corner DRAG writes Point/CHANGED_VALUE and UPDATE_NOW/ALWAYS_UPDATE even
  if its computed position equals the current value. Unlike guide drag, there is
  no before/after guard. Candidate for no-op suppression; not a playback loop.
- plane.rs update_ui sends7 UpdateParamUI calls (mode, edge,4 corners, Fit Layer)
  each callback without checking whether presentation changed. Additional calls
  occur on supervised selector changes. This updates controls, not independently
  rendering image frames; no callback loop demonstrated.
- smart_render_snapshot and plane/binding helpers may checkout some parameters
  more than once (mode, research kind, conditional neutral-grid proof). Those are
  values/expressions rather than image frame requests. Four hidden point bindings
  each evaluate sourceRectAtTime/toComp/fromComp; this may repeat geometry work,
  not an explicit async frame render. No unverified cache/migration added.

## Necessary or already guarded work

Guide drag compares grid before writing/updating. Reset/Fit Layer only force
rerender on real changes. Other ForceRerender calls occur on user changes to mode,
edge, radius. QueryDynamicFlags clears NonParamVary for inactive Wave animation;
it does not start a timer or request frames. Show Grid draws on the existing
output only when enabled. Demo OFF exits before dependencies or raster work.
Thread-local CPU preparation caches keep reusable geometry/LUT/row buffers, not
a sequence of host frames; CPU fan-out is bounded to4 by default/16 explicit.
No additional full-frame sequence cache or independent render-request loop found.

## Candidate and incident status

Dev124 clean source674c984, BID EGFX-b6c190318a974348695d86e0:103 Rust tests,
strict Clippy, Mac build/signature/archive/manifest PASS. Installer ZIP SHA256
e1c47e3e79936dfb0aa4656f723188c78bda189c458fc5a97c511a8b029e8aaa.
Candidate NOT INSTALLED; disk plugin remains Dev120. Runtime loupe/playback
regression NOT RUN, root cause UNCONFIRMED, Windows candidate NOT BUILT.
Current post-reboot swap measured7.69GiB at19:59+0200. No repeated RAM Preview
or live heavy rendering attempted. Do not close the incident or promote release.
