# Stage 9 research — 2D/3D perspective deformation plane

Date: 2026-09-29
Current production stage: 7 of 10. This document is research only.
No perspective-plane production code is enabled before Stage 7 functional
acceptance and Stage 8 baseline profiling.

## Product requirement

The deformation must operate in a plane that can be:
1. positioned by four 2D corner points;
2. attached to a 3D AE layer so position/scale/rotation/parenting and active
   camera perspective affect the grid and deformation consistently.

The same transform must drive:
- rendered pixel sampling;
- visible grid lines;
- handles;
- pointer hit-testing / dragging.

It is not acceptable to distort only the final raster while controls remain in
untransformed screen space.

## API research

Reference implementation source:
https://github.com/virtualritz/after-effects

### Effect context / camera

`AEGP_PFInterfaceSuite1` exposes:
- effect layer handle;
- effect-time -> composition-time conversion;
- active effect camera;
- `AEGP_GetEffectCameraMatrix`, returning camera matrix, distance to image plane,
  and image-plane dimensions.

The wrapper notes:
- the camera matrix convention is row-based;
- the model-view is the inverse of the returned camera matrix;
- non-square composition/layer pixels require explicit PAR correction.

### Layer transforms

`AEGP_LayerSuite7` exposes:
- `AEGP_GetLayerToWorldXform`;
- `AEGP_GetLayerToWorldXformFromView`;
- layer parent, stable layer ID and 2D/3D state.

This is preferable to re-implementing AE parenting/rotation order from individual
transform properties.

### Camera geometry

`AEGP_CameraSuite2` exposes camera type and film size. The reference docs state
that camera position/orientation spaces and projection conventions must be
handled carefully. Perspective and orthographic cameras are distinct cases.

### Existing effect example

The reference Resizer example dynamically sets:
- `OutFlags2::IUse3DCamera`;
- `OutFlags2::IUse3DLights` when its 3D mode is active;

and reads camera/layer data only in that mode. ElasticGrid only needs camera
dependency for the requested plane feature; lights are not part of the current
requirement.

## Dependency/caching risk

The PFInterface documentation explicitly warns that external data dependencies
can change without AE automatically knowing that an effect depends on them.

Therefore Stage 9 cannot simply call AEGP layer/camera APIs during render and
assume cache invalidation works.

Required prototype checks:
- moving/rotating/scaling the effect layer invalidates the correct frames;
- changing a parent invalidates;
- moving/changing active camera invalidates;
- camera switch/no-camera state invalidates;
- MFR frames do not share stale transform state.

Use the documented 3D-camera out flag where applicable, but treat the flag as a
dependency hint to be verified, not proof of correctness.

## Chosen mathematical architecture

Keep ElasticGrid's existing deformation in **plane-local normalized coordinates**.

For a screen/output point q:
1. inverse-project q into plane coordinates with `H^-1`;
2. apply ElasticGrid inverse deformation `W^-1` in plane space;
3. project the resulting source coordinate through `H`/the equivalent
   layer-camera transform;
4. sample the source image once.

Conceptually:
`source = H(W^-1(H^-1(q)))`.

For 2D four-corner mode, H is a projective homography.
For 3D mode, H is derived from AE's layer-to-world and camera projection.

The old nonprojected X/Y-separable renderer stays an explicit fast path. A
general projected warp is not separable in screen X/Y, so the current two 1D
sampling LUTs cannot represent it without approximation.

## 2D Stage 9A prototype

New state must be additive; existing saved parameter IDs/types/ordinals remain
unchanged.

Proposed controls:
- Plane Mode: Existing / Four Corner / 3D Layer;
- four point controls for 2D mode;
- Reset Plane.

Before choosing exact parameter IDs/order, create a migration/schema test.

Required CPU prototype:
- stable double-precision homography solve on control path;
- float/double precomputed inverse matrix passed to render;
- singular/near-singular determinant detection;
- no per-pixel matrix inversion;
- inverse mapping only (no holes from forward rasterization);
- Bicubic sampling quality unchanged.

Required behavior to decide/test:
- crossed quadrilateral;
- degenerate/near-zero area;
- corners outside canvas;
- plane partly outside frame;
- adjustment-layer pixels outside plane.

Recommended initial visual policy for invalid/near-singular planes:
- do not return black;
- fail safe to pass-through/original outside a valid transform domain;
- expose an interactive warning only outside noninteractive render callbacks.

This policy is a proposal until accepted/verified.

## 3D Stage 9B prototype

At render time for the exact frame:
1. convert effect time to comp time;
2. get the effect layer handle;
3. get layer-to-world transform from AE;
4. get active effect camera/camera matrix;
5. compose local-plane -> world -> camera/image-plane -> render-pixel transform;
6. include PAR/downsample/origin corrections;
7. derive the 2D projective mapping used by the same projected sampler.

The grid UI/hit-test path must derive the same plane transform for the active
composition view. Render-view and arbitrary editor view are not assumed
identical; multi-view/custom-view behavior needs a separate defined policy.

## Singular 3D cases

“Any rotation” cannot mathematically include an exactly edge-on plane as a
finite invertible homography.

Define and test:
- edge-on/near-edge-on threshold;
- plane behind camera;
- camera crossing the plane;
- zero scale;
- negative scale / winding flip;
- parent with collapsed transform;
- orthographic camera;
- no active camera;
- non-square pixels;
- downsampled preview.

These cases must produce a documented stable result, never division-by-zero,
NaN propagation or an unexplained black frame.

## Performance constraints

Projection must not destroy the current fast path.

Implementation targets:
- zero projected-path overhead when Plane Mode = Existing;
- per-frame matrix/homography preparation only;
- no per-pixel heap allocation;
- no intermediate full-frame raster solely for projection;
- one final source sample per output pixel;
- SIMD/GPU work considered only after measured CPU baseline;
- Stage 8 Before -> Change -> After protocol applies.

## Test matrix before enabling

Math/unit:
- identity homography;
- affine rectangle;
- known perspective quadrilateral;
- inverse roundtrip;
- random convex quads;
- near-singular rejection;
- finite-value guarantees.

Pixel:
- 8/16/32 bpc;
- alpha/transparent borders;
- negative/>1 float values;
- Bicubic quality;
- sparse/full equivalence.

AE integration:
- four-corner drag agrees with pointer;
- Undo/Redo/save/reopen;
- layer 3D toggle;
- XYZ rotations and orientation;
- scale including negative;
- parenting;
- camera position/orientation/zoom;
- no camera / default view;
- RAM Preview/cache invalidation;
- Render Queue/aerender;
- MFR determinism.

## Decision at this research stage

The requested feature is technically feasible with AE's native layer/camera
matrix APIs. No custom 3D engine is needed.

However implementation must wait until:
1. Stage 7 confirms the current SmartFX/Corner Pin fix in the real target AE;
2. Stage 8 records a trustworthy performance baseline.

This prevents a new projection architecture from masking an unresolved host bug
or making performance attribution impossible.
