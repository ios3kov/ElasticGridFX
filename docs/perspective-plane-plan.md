# Perspective plane and effect-chain contract

Confirmed by the user on 2026-09-28. Source baseline: a871596 (native plugin
installed for testing: 6d3b846 / EGFX-603e9d3e4025d271e0488201).

## Required product behavior (not implemented/verified yet)

Reconfirmed 2026-09-29: the built-in four-corner control places the deformation
plane itself. In 3D-layer mode the plane follows layer transforms and camera;
moving a guide along a wall deforms along that wall, not screen-horizontal.
Overlay, handles, pointer inversion and pixel sampling share plane coordinates.
This remains Stage 9 work after the Stage 8 baseline, not a completed feature.

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
The desired singular/outside-plane visual policy is still a design decision;
do not silently claim it was specified by the user.

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
