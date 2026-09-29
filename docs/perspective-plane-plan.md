# Perspective plane and effect-chain contract

Confirmed by the user on 2026-09-28. Source baseline: a871596 (native plugin
installed for testing: 6d3b846 / EGFX-603e9d3e4025d271e0488201).

## Required product behavior (not implemented/verified yet)

Reconfirmed 2026-09-29: the built-in four-corner control places the deformation
plane itself. In 3D-layer mode the plane follows layer transforms and camera;
moving a guide along a wall deforms along that wall, not screen-horizontal.
Overlay, handles, pointer inversion and pixel sampling share plane coordinates.
Stage 9 started after the user explicitly skipped further Stage 8 acceleration
work on 2026-09-29. Existing baseline retained; this is not a completed feature.

## First implementation slice: shared geometry

P9-1: one validated forward/inverse homography for local unit-square coordinates
and four ordered surface corners. Corner order: top-left, top-right, bottom-right,
bottom-left by local identity, including mirrored transforms. Strict convex quads
only; reject nonfinite, crossed, concave, collapsed and numerically edge-on inputs.
This is an internal validity policy, not yet the visual fallback policy in AE.
Coordinate translation/scale normalization keeps tolerance independent of pixels.
Tests cover identity, analytic trapezoid center, corner correspondence, mirrored
winding, large translation, dense roundtrips and horizon/nonfinite rejection.

P9-2a implemented in standalone core: PlaneWarp composes the existing inverse
grid calculation with H^-1 and H and returns the source coordinate before any
image sampling. It owns validated evaluated guide vectors (strictly increasing,
finite, endpoint-anchored), with no mutable per-frame shared state. Exact uniform
grids return original destination coordinates without roundtrip drift. Outside
points and invalid projections are distinct results without usable source values;
the caller must explicitly choose a visual policy rather than silently sample (0,0).
Tests use an analytic slanted projective plane: moving a column changes both
screen coordinates, and local guide .75 samples the old .5 guide. Dense identity,
existing easing equivalence, snapshot ownership, invalid grids and horizon checks
are included. This does not yet sample pixels or integrate AE.

P9-2b: user approved outside-plane exact pass-through and invalid-plane original
image plus diagnostics. Initial standalone dense float CPU implementation now
performs one Catmull-Rom sample of the original input at the composed coordinate.
Null/invalid plane returns an invalid_plane flag; per-pixel projection failures
pass through with a counter. Outside-plane pixels and unchanged mapped coordinates
copy all four channels bit-for-bit. No alert/dialog in rendering. Source/destination
must be distinct nonoverlapping same-size dense canvases with positive valid strides.
Sampling clamps at canvas edges; alpha and HDR/negative float channels are not clamped.
Cancellation is checked per row and can leave partial output, which callers must
discard on RenderCancelled. The old separable renderer is unchanged.
Tests: Final equivalence on a flat plane, fractional negative/HDR values, alpha,
outside/invalid/identity copying, padding preservation, alias rejection and cancellation.
Sparse-canvas slice now implemented in standalone float renderer: source/output
origins are independent of storage dimensions; cubic taps clamp against the logical
canvas and absent checkout pixels read transparent zero. No allocation of a full
zero-filled canvas is needed. A 0x0 source is valid; stored rectangles must lie
inside a positive canvas, with nonempty output and valid strides/nonoverlap.
Expanded rectangles are supported in the standalone core: negative source/output
origins and stored pixels beyond the logical canvas are allowed. Destination
pixels outside that canvas are transparent zero; source storage outside it is
ignored. This matches the existing sparse bridge's canvas policy, without claiming
that the new plane path has been wired into AE. Dense API delegates to the same region path.
Tests require bit-exact compact/full-zero-filled equivalence for deformed,
identity and invalid-plane states, tiled-output equivalence, padding preservation,
empty input and malformed-region rejection.
8/16-bit region entry points now share the float path's coordinate/tap calculation.
Accumulate both cubic axes in float and round once; sampled channels clamp to
255 or AE's 32768, never 65535 for 16-bpc. Identity/outside/invalid-plane samples
retain input bits (including unusual values), matching the previous copy policy.
Acceptance: padded strides, sparse/full bit-exact equality, empty input, bad stride,
outside/invalid copying and flat-plane Final comparison within one integer level.
These are standalone core tests, not AE bit-depth acceptance.
Raster/surface basis is now explicit: PlaneCanvasRegion carries positive finite
surface_units_x/y. Multiply global raster coordinates before plane mapping, then
divide the returned source coordinate before cubic sampling. Source/output origins
remain raster pixel indices. Identity skips the multiply/divide roundtrip for exact
copies. Reject zero/negative/nonfinite/overflowing scale metadata before output.
This supports a caller-defined downsample/PAR basis; it does NOT derive AE matrices,
apply camera projection, infer pixel-center offsets or confirm AE PAR behavior.
Tests compare scaled corners against unit-scale coordinates at (2,2), (2.4,3),
(.75,1.25), including exact identity and invalid metadata rejection.
Expanded-canvas tests now cover 8/16/float, moved/identity/invalid planes, bit-exact
interior equality to a dense reference, outside zero, stride padding, cancellation,
and INT_MIN/INT_MAX origins. Coordinate sums/differences use int64 before bounds
checks to prevent overflow. ASan/UBSan and the 16-target Release suite pass.
Next: native host wiring; verify real AE pixel-center,
downsample and PAR conventions before presenting this as supported host behavior.
P9-2c: additive native bridge now compiled by Cargo, with a real Rust-to-C++ test.
EgPlaneFrame borrows an immutable, already evaluated guide snapshot (including
any wave evaluation by the caller); corners and surface scales use explicit
double coordinates. EgPlaneImage byte strides are validated and converted to
element strides once. Existing EgRenderParams and saved parameter IDs are unchanged.
Limits: 2..52 guides per axis, aligned positive strides, nonoverlapping views;
invalid grids/views return error before rendering, invalid corners pass through
with a report flag. Exceptions never cross the C ABI; abort returns 5 and the
caller must discard partial output. Reports reset on entry and publish on success.
Current bridge is explicitly Final Catmull-Rom with logical-canvas Clamp; native
dispatch must not silently route Draft/Wrap/Mirror through it. Before enabling
plane controls, complete those modes or explicitly agree a narrower mode contract.
17 CTest targets, 20 Rust tests, Clippy (existing macro exceptions), bridge
ASan/UBSan PASS. Tests include all depths, direct-core equality, ABI layouts,
invalid-grid/depth/view handling, invalid-plane copying, empty input and abort.
Production render dispatch is NOT connected yet; native UI and AE acceptance
remain NOT_RUN. No installed artifact changes in this slice.
P9-3: append compatible parameters and connect overlay/pointer inversion.
P9-4: integrate AE layer/camera with context-correct transforms and real-host
acceptance. Existing host-rust/src/ui.rs already uses layer_to_comp/comp_to_layer;
do not apply camera projection twice to an ordinary transformed layer.
SDK-derived references checked on 2026-09-29:
https://ae-plugins.docsforadobe.dev/effect-ui-events/ui-callbacks/
https://ae-plugins.docsforadobe.dev/aegps/aegp-suites/
They describe UI coordinate callbacks and PFInterface camera access; target
thread/context applicability and exact matrix conventions still require validation.
No new dependency, native installation or camera-support claim in P9-1.

The user confirmed that “управляющий слой” means an Adjustment Layer.
1. Fix the reported black output when Adobe Corner Pin follows ElasticGrid FX
   on the same adjustment layer. The user reports Fast Blur works in comparison.
   This is a compatibility defect, not a reason to require a built-in replacement.
2. Add a four-corner 2D placement mode for the plane in which grid deformation
   operates: align the grid/handles/warp to a wall, screen or tabletop. Do not
   simply distort the finished output independently of the interaction grid.
3. Add a layer-driven 3D plane mode, following layer position, scale, rotation,
   parenting and active camera perspective. Grid drawing, hit-testing and image
   sampling must use the same plane transform. Turning on the 3D switch alone
   is not an implementation or proof of this behavior.
4. Render and RAM Preview speed is a release requirement with NO hidden quality
   downgrade. See performance-quality-contract.md; absolute targets are pending
   the measured target-Mac baseline, not invented universal FPS promises.

## Sequence and current evidence

Stage A: correct existing strips, cropped-input/transparent-boundary behavior,
chain/bounds handling and clipped custom-control text. Stage B: four-corner
plane math and interaction. Stage C: AE 3D/camera integration. Profile each
performance-sensitive stage; optimize measured bottlenecks without weakening
pixel acceptance. Do not combine a new projection architecture with a bug fix.

The enabled/disabled screenshots establish visible streaks only with the effect
active in that state. They do not establish input-buffer origins, bit depth,
exact source of the defect or complete loaded Build ID. The installed-candidate
receipt was received and reviewed separately. Corner Pin black output is user
reported; no corresponding project/callback trace has been captured here.

## Proposed architecture, subject to research/prototype

Keep the separable grid warp in local plane coordinates. For a projected point
q, compose inverse screen-to-plane H^-1, local inverse grid warp W^-1, then H:
source = H(W^-1(H^-1(q))). The sampler reads the original image ONCE after
coordinate composition, not after multiple intermediate raster passes. The
existing nonprojected separable path remains an explicit fast path.

Four-corner H and camera/layer-derived H must share one validated transform
representation with overlay and hit-testing. A projected mapping is generally
not separable in screen coordinates, so existing X/Y-only sampling plans cannot
be reused unchanged. This is a proposal, not shipped functionality.

Decide and test before enabling the modes: plane-local domain; behavior outside
it (proposed pass-through for adjustment layers); degenerate/crossed corners;
edge-on planes, near-camera clipping/behind-camera cases; winding/back face;
negative/zero scales; pixel aspect ratio; downsampling; collapsed transforms;
which camera drives rendering versus nonactive views. “Any rotation” requires
stable defined handling at singular views, not division by zero or black output.
The user subsequently confirmed the singular/outside-plane pass-through policy
above. Camera near-plane and occlusion semantics still need explicit host validation.

Keep existing saved parameter IDs, types, popup ordinals and wire state. New
controls require explicit schema/migration tests; do not reinterpret old keys.
No depth/occlusion or true volumetric deformation is promised by a planar warp.

## Acceptance matrix

- Raster/text/precomp/adjustment input; identity and moved guides; reset.
- ElasticGrid -> Corner Pin, Corner Pin -> ElasticGrid, ElasticGrid -> Fast Blur
  comparator; chain toggles, cache invalidation, render queue and RAM Preview.
- Sparse/full input equivalence on the SAME logical canvas, transparent/opaque
  borders, alpha/HDR/negative values, 8/16/32 bpc, cropped/expanded output, stride.
- Four-corner identity matches existing output; known projective examples;
  drag follows pointer in the transformed plane; Undo/Redo/save/reopen.
- 3D camera motion and all layer transforms; singular cases; MFR determinism.
- Equal-quality before/after timings, memory and transfers; no speed claim from
  cached playback alone. Real AE and physical Metal remain required evidence.

## Sources checked / implementation implications

Adobe SDK-derived SmartFX reference:
https://ae-plugins.docsforadobe.dev/smartfx/smartfx/
Result rectangles must match requested output, max bounds must not depend on the
request, empty requests may ask only for bounds. Missing source pixels cannot
be replaced with stretched cropped-edge pixels.
https://ae-plugins.docsforadobe.dev/effect-basics/PF_EffectWorld/
World dimensions and rowbytes describe checked-out storage, not guaranteed full
layer dimensions; writing outside the output world is forbidden.
https://ae-plugins.docsforadobe.dev/effect-details/tips-tricks/
Output buffers need not start cleared; text-layer origins differ from usual
raster-layer origins. These are candidate investigation areas, not proof that
any single issue caused the user's Corner Pin failure.
