# Stage 9 user acceptance corrections

User request: record and implement five corrections after visual acceptance.
Stage 9 OPEN; immutable installed de31498 unchanged until a new candidate passes.
Historical 34-frame/26-check PASS verifies that matrix only, not these requirements.

## Requirements and acceptance

1. Rename Existing Grid to Layer Plane (same numeric mode). Hide native corner
   targets in the viewer and disable corner/reset controls outside Four Corners.
2. Four Corners maps the complete layer image into the destination quadrilateral,
   even with uniform guides and zero wave. It must not merely transform a region
   in which an otherwise stationary image is distorted. Source rectangle and
   destination plane are distinct. Outside the valid destination quad is
   transparent; invalid-quad fallback remains original image until specified otherwise.
3. On a 3D layer the overlay, corner handles and inverse pointer mapping coincide
   with the actual transformed layer plane and active camera, including parenting.
   Validate positions numerically and visually, not just changed rendered frames.
4. Render Quality is the last visible parameter.
5. Logical panel order: plane mode/corners, grid, elastic deformation, wave,
   edge handling, render quality. Preserve saved parameter IDs, value semantics,
   keyframes and arbitrary grid data when reordering/renaming controls.

## Tasks

- [x] Source implementation: Four Corners source-rectangle sampling; regression
  for zero-wave nonidentity perspective, known markers and transparent outside.
  New artifact and real-AE verification remain pending.
- [ ] Implement/test actual 3D layer-to-camera projection and inverse hit testing.
- [ ] Mode-dependent native corner visibility and control availability.
- [ ] Friendly labels/order with explicit persistent IDs and legacy AEP roundtrip.
- [ ] New clean candidate, exact loaded identity and strengthened real-AE matrix.
- [ ] Final native interaction acceptance; Stage 10 remains NOT STARTED.

Root cause found for requirement 2: PlaneWarp computes H(W^-1(H^-1(q))) and
explicitly returns q for uniform guides. That models deformation within a plane,
not projection of the original source rectangle into that plane. Existing tests
asserted this obsolete behavior. Keep any historical regional-warp primitive
explicit, while the Four Corners bridge must use the source-rectangle mapping.

3D screenshot confirms viewer mismatch; precise host API cause is under investigation.
Do not change render camera transforms merely to align the overlay: rendering in
layer coordinates and viewer projection are separate responsibilities.

## First implementation checkpoint

Four Corners FFI now uses prepareProjected with the source canvas last-pixel
extent, not the destination quadrilateral, for source sampling. Uniform guides
still project the image. Pixels outside the valid destination quad or at its
projection horizon are transparent. Invalid quad retains original-image fallback.
The regional-warp primitive remains explicit for historical coverage; no double
resampling, bit-depth clamp changes, GPU changes or installed artifact replacement.

Local evidence: 199/199 Python tests PASS, 18/18 CTest PASS, 22/22 Rust host tests PASS, projected-source
bridge ASan/UBSan PASS. Independent sampling-oracle coverage retained for the
regional warp and extended to the projected FFI at 8/16/32 bpc and all quality/
edge combinations; sparse checkout equivalence remains checked. Static scanner
exit 1: existing unchanged workflow supply-chain and test heuristic findings,
not a release PASS. No new AE acceptance claim for these source changes.

Remaining high-risk work: 3D viewer projection/inverse pointer coordinates and
native point-control visibility; friendly panel ordering requires preserving disk
IDs rather than relying on source registration order. No UI changes yet.
