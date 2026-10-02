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

## Sparse CPU path

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
The host uses this path when no plane snapshot is available; its immutable
snapshot carries logical-canvas dimensions/origins and handles empty checkouts.
See sparse-render-stage-plan.md and sparse-render-results-2026-09-28.md.

## Implemented perspective plane

`host-rust/src/plane.rs` renders immutable evaluated axes through the separate
152-byte `EgPlaneFrame` ABI. Normal bounded planes use `eg_render_plane_region`;
native comp-space text uses `eg_render_plane_layer` with end-cell extrapolation
so fractional perimeter coverage deforms. Mapping composes the plane transform
with inverse deformation and samples the original image once. Overlay and hit
testing share the plane geometry. Four Corners remains a 2D bounded region;
native 3D uses Layer Plane. See perspective-plane-plan.md and the exact 0.9.3
acceptance record for scope and limitations.

The resumed performance branch caches the original mapping and sampling taps
per output axis only when both transform matrices have exactly zero cross-axis
and perspective coefficients, the warp does not project a separate source, and
raster surface scale is one. There is no approximate geometry classification.
Four call-local horizontal rows reuse Bicubic intermediate results without
changing per-channel arithmetic/tap order. Cache keys refer to logical source
rows and cannot evict any row still required by the current output row.
Non-unit scales, rotated/perspective/projected geometry and failed intermediate
projections retain the general sampler. C++ reference tests can disable this
optimization through `PlaneCanvasRegion`; it is not an AE parameter or C ABI field.

These caches are local to one render call. MFR frames do not share mutable
mapping, rows or source pointers. Cancellation is polled on the calling thread,
before preparation and at the same per-row frequency as the original sampler.

## Optional callback observation

Cargo `render-diagnostics` observes the existing Render/SmartPreRender/SmartRender
branches without changing render decisions or introducing an Adobe API. The
feature is absent from default builds. Phase endpoints use monotonic time and
process-relative callback starts; per-process logs contain no project/source
names or pixels. Private exclusive files are bounded at 4096 records and stop on
write failure. The optional writer serializes after observed endpoints; its
execution still perturbs the host and is unsuitable as acceptance timing.

`tools/render_observation.py` pins Build ID and schema, refuses malformed or
incomplete records, and labels partial/capped/empty observations explicitly.
Callbacks are not unique frames; missing observations do not prove cache hits.
Overlapping sampling intervals do not establish concurrent CPU execution.
Active Cargo features participate in Build Identity. A diagnostic and default
build cannot share a Build ID for the same source/toolchain/settings.
The existing PiPL effect version separates Develop build 2 (ordinary) and 3
(observer) from accepted build 1, so validation variants use distinct effect
cache identities without global purge. Product/saved-state versions are unchanged.
See [observation design](performance-observation-design.md). Actual AE 25.6
headless execution observed 60 successful 32-bpc legacy callbacks, with exact
loaded-image identity. That workload does not exercise plane acceleration.
New performance fixtures explicitly read back plane mode/corners in schema 3;
the observation reader checks route, dimensions, depth and complete rational
frame coverage against the hashed fixture. Historical schema 1/2 is retained.
Actual plane execution and uninstrumented target timing remain pending.

## Acceptance

Quality and equal-condition profiling are governed by performance-quality-contract.md.
Portable tests, native compilation and install receipts have separate scopes;
none replaces actual loaded-identity, AE effect-chain, render/preview, hardware
Metal and project-lifecycle evidence. No current source build is release approval.

Performance fixture preparation uses two distinct guarded AE script turns. The
first creates only an empty-project-owned scene; the second verifies UUID,
source file, geometry, one layer/effect and already-installed error-free hidden
binding streams before deformation/save. The existing native deferred binding
and its pending-render refusal are unchanged. Actual AE proves complete
plane_region coverage for all 60 schema-3 frames; observer timing stays separate
from ordinary Render/RAM Preview acceptance.
