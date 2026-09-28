# Architecture

## Implemented boundary

host-rust uses pinned after-effects/pipl bindings for AE ABI, parameters, UI and
SmartFX snapshots. src/bridge is the validated C ABI boundary; src/core owns
portable grid evaluation, inverse mapping and CPU sampling; src/gpu contains
Metal code. The current host uses eg_render_frame and full-source checkout.
Its SmartPreRender currently disables GPU dispatch. GPU code and advertised
capabilities alone do not establish a working AE Metal path.

## Existing fast path and quality

The grid is separable: vertical guides map X, horizontal guides map Y. Evaluated
monotonic axes feed inverse LUTs and prepared Bilinear/Catmull-Rom Bicubic taps.
Exact uniform identity copies rows rather than interpolating. Final remains
Bicubic. CPU channel operations support 8/16/32 bpc; 16-bpc uses AE's 32768 range.
macOS scheduling uses GCD; independent frame threads own reusable plan/row caches.
Actual AE MFR, cancellation and throughput still require host acceptance.

## Staged sparse CPU path (not host-connected)

eg_render_frame_sparse requires explicit positive logical canvas dimensions.
The caller must establish that pixels absent from the returned compact world
are zero, rather than merely unrequested. Build taps on that full coordinate
system, map them into storage, and substitute zero for missing/outside taps.
No padded full-canvas image is allocated. Empty source clears output without
reading input; padding is preserved and cancellation is propagated.

The internal CPU-only plan flag chooses dense or transparent-aware template
kernels once per render. Transparent taps use -1 internally; the row cache uses
-2 as its unused key. Existing eg_prepare_gpu_plan keeps nonnegative indices and
unchanged layout; these sentinels MUST NOT be sent to current Metal kernels.
The frozen 160-byte EgRenderParams and saved AE parameter schema are unchanged.
See sparse-render-stage-plan.md and sparse-render-results-2026-09-28.md.

## Planned perspective plane

Four-corner placement and camera/layer-driven 3D perspective are approved product
requirements, not active code. Proposed composition maps output to plane-local
coordinates, applies inverse grid deformation, maps back and samples once.
Overlay and hit testing must use that same transform. The general projected
mapping is not screen-separable; retain the existing separable fast path when
projection is off. See perspective-plane-plan.md for decisions and acceptance.

## Acceptance

Quality and equal-condition profiling are governed by performance-quality-contract.md.
Portable tests, native compilation and install receipts have separate scopes;
none replaces actual loaded-identity, AE effect-chain, render/preview, hardware
Metal and project-lifecycle evidence. No current source build is release approval.
