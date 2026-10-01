# FSTR Stretch 0.9.4 — Performance Release Plan

Status: **PLANNED — implementation not started**  
Date: 2026-10-01  
Baseline product: accepted FSTR Stretch 0.9.3 development candidate.

## 1. Goal

The next development cycle is dedicated to making FSTR Stretch render and build
RAM Preview **extremely fast while preserving the accepted image quality and
behavior of 0.9.3**.

This is not permission to trade quality for speed. The work must remove real
bottlenecks in the current implementation: cache invalidation, CPU/MFR scheduling,
SmartFX source checkout, disabled Metal execution and the serial/scalar projective
plane path.

Primary target environment remains macOS Apple Silicon / After Effects 2025 25.6
until a broader compatibility matrix is separately verified.

## 2. Immutable baseline

Performance and quality comparisons start from the exact accepted 0.9.3 runtime:

- source commit: `2ccc5f674b8b94b534d4c3ad6abdec3524e3624f`;
- Build ID: `EGFX-bd19dee13315abc0b7e6090e`;
- ZIP SHA-256: `1448a5231fc561e6b10491d1b9d1839de22f21a11266a43e2990244a25176ebc`;
- binary SHA-256: `7dc57622442eaddcc1f061e044e77db3796a2db44e4061c807eee6aa60c15beb`.

0.9.3 functional acceptance remains historical evidence. It must not be relabeled
as evidence for a later 0.9.4 binary.

The historical Stage 8 status for 0.9.3 remains **SKIPPED BY USER**. 0.9.4 opens a
new, explicit performance cycle rather than rewriting that historical status.

## 3. Non-negotiable quality contract

Performance work is rejected if it obtains speed by changing the product result.

Forbidden shortcuts:

- no Final Bicubic → Bilinear substitution;
- no hidden lower render resolution;
- no proxy or reduced-quality path presented as Final;
- no `fast-math`;
- no deliberate precision reduction;
- no different Catmull-Rom reconstruction kernel;
- no clipping of valid 32-bpc negative or >1.0 values;
- no alpha degradation;
- no changed Clamp / Wrap / Mirror semantics;
- no simplified Four Corners / 3D projective geometry;
- no disabling MFR, cancellation, Undo/Redo, save/reopen or project compatibility
  merely to improve a benchmark.

Existing quality gates remain the minimum contract:

- exact identity remains bit-for-bit where the current renderer guarantees it;
- 8/16-bpc output remains within the existing independent-reference limits;
- 32-bpc CPU/Metal parity must not weaken the existing tolerance
  (`max_abs <= 2.5e-5`, `RMS <= 3e-6`) unless a stricter contract replaces it;
- Final Catmull-Rom, HDR/negative-float behavior and edge modes remain tested.

Any optimization that violates this contract is reverted, regardless of speedup.

## 4. Performance objectives

These are engineering targets, not release claims until measured in real AE.

Primary targets:

- ordinary 4K Final Layer Plane / grid path: aim for **3× or better** real-AE
  render/RAM-build throughput versus the frozen 0.9.3 baseline where the optimized
  path is eligible;
- Four Corners / native 3D projective path: aim for **5× or better** where Metal
  removes the current serial/scalar bottleneck;
- UI-only guide-density changes should avoid pixel-cache invalidation entirely;
- warm/static geometry should avoid redundant plan construction;
- cancellation must remain responsive and deterministic;
- memory growth must remain bounded under long RAM Preview and MFR rendering.

If AE, upstream effects, encoding or memory bandwidth becomes the dominant cost,
the project must report that measured limit instead of weakening quality to chase
an arbitrary multiplier.

## 5. Measurement protocol before optimization

No major renderer rewrite starts before a trustworthy target-AE baseline exists.

Create one controlled synthetic project/fixture and record at minimum:

### Geometry / mode matrix

- ordinary Layer Plane;
- Four Corners;
- automatic native 3D Layer Plane;
- static deformation;
- animated Grid Positions;
- wave animation.

### Resolution / depth matrix

- 1920×1080;
- 3840×2160;
- 7680×4320 where practical;
- 8 / 16 / 32 bpc;
- Final Bicubic first, Preview Bilinear as a separate measurement.

### Host matrix

- Render Queue / aerender;
- MFR ON / OFF;
- RAM Preview cold build;
- RAM Preview warm replay;
- rebuild after a real render-affecting parameter edit;
- fresh-frame latency during interactive deformation.

Record:

- exact Build ID/package identity;
- AE version/build;
- macOS/hardware;
- wall time and per-frame timing where available;
- at least five measured runs after warm-up for automated Render Queue/aerender;
- p50/p95/spread;
- peak resident memory;
- CPU utilization;
- GPU utilization for GPU-eligible paths;
- output hashes / decoded-pixel comparison;
- cache state classification.

Cached playback FPS is not renderer throughput and must be reported separately.

## 6. Wave 1 — cache and invalidation wins

### 6.1. Columns / Rows must be UI-only

Current accepted architecture already makes Columns/Rows display density independent
from retained Grid Positions and renderer state.

Investigate and replace the current render invalidation with a **viewer/UI redraw
only** when Columns or Rows change.

Acceptance:

- Grid Positions bytes/key metadata unchanged;
- rendered pixels unchanged;
- existing cached frames remain valid;
- visible controls update immediately;
- no stale hit-test or drag geometry;
- Undo/Redo and save/reopen remain correct.

### 6.2. Audit dynamic render dependencies

Re-audit `QueryDynamicFlags`, wave timing and all supervised parameters.

AE should invalidate pixels only when a dependency that can change the rendered
image actually changes.

Static warps should retain useful cache hits. Time-varying wave output must remain
correctly marked time-dependent.

### 6.3. Reuse prepared warp plans

Cache/reuse prepared LUT/sampling data when the effective render state is unchanged.

Candidate cache key must include all state that can affect the plan, including as
applicable:

- dimensions;
- origins;
- retained Grid Positions;
- easing;
- edge mode;
- quality;
- wave-evaluated geometry/time.

No cache entry may contain live AE parameter handles. Cache ownership must remain
MFR-safe and bounded.

## 7. Wave 2 — MFR-aware CPU scheduling

Current CPU rendering intentionally limits per-frame fan-out, but AE can also render
multiple frames concurrently.

Implement a measured adaptive scheduler rather than fixed nested oversubscription.

Goals:

- one active heavy frame may use more CPU parallelism;
- multiple MFR frames reduce internal per-frame fan-out;
- total FSTR worker pressure remains near a measured CPU budget;
- no starvation of AE MFR;
- no shared mutable render state;
- cancellation polling remains on a safe calling thread.

Tune only from real measurements. Do not hardcode assumptions about Apple Silicon
performance/efficiency cores without evidence.

Acceptance:

- deterministic output;
- ThreadSanitizer / MFR regression PASS;
- no >5% repeatable regression in single-frame latency unless total Render Queue /
  Preview throughput improves substantially and the tradeoff is documented;
- cancellation latency does not regress materially.

## 8. Wave 3 — exact SmartFX ROI

Current SmartPreRender intentionally checks out the complete logical source canvas
for correctness.

Introduce minimum-source ROI only after proving it seam-free.

Order:

1. ordinary separable grid;
2. Bicubic filter halo;
3. cropped/translated worlds and nonzero origins;
4. sparse/transparent source storage;
5. Clamp;
6. Wrap/Mirror only after their potentially nonlocal dependency behavior is proven;
7. projective/Four Corners/3D ROI as a separate later proof.

The ROI must be conservative: over-requesting is acceptable during development;
under-requesting pixels is a correctness failure.

Measure end-to-end impact on chains with expensive upstream effects, not only the
FSTR kernel itself.

## 9. Wave 4 — enable the existing Metal grid renderer safely

The repository already contains:

- Metal Bilinear;
- Metal Catmull-Rom Bicubic;
- CPU/GPU parity tests;
- sequential and MFR determinism tests;
- lifecycle/setup/setdown tests;
- reusable GPU plan buffers;
- `SmartRenderGpu` host integration.

However normal SmartPreRender currently disables GPU eligibility.

Do **not** simply flip the flag globally.

Add one explicit eligibility predicate. Metal is advertised only when every semantic
requirement of that frame is supported; otherwise the frame stays on the verified CPU
path.

First production scope:

- ordinary separable grid/Layer Plane semantics supported by the existing GPU kernel;
- 32-bpc GPU world path currently supported by the host integration;
- supported edge modes and Final/Preview sampling;
- no unsupported sparse/projective behavior silently routed to the old kernel.

Required real-AE gate:

- exact loaded candidate identity;
- CPU vs Metal decoded-pixel comparison;
- Final Bicubic parity within the existing contract;
- RAM Preview build/replay/invalidation;
- Render Queue;
- MFR;
- repeated setup/setdown;
- long soak;
- normal cancellation before/after short GPU work;
- no crash/device-state leak.

GPU PASS in standalone tests is not enough to enable production AE dispatch.

## 10. Wave 5 — Metal Four Corners / native 3D projective renderer

This is expected to be the largest single performance opportunity.

The current CPU projective renderer performs per-pixel projective mapping and
sampling in a serial/scalar path. Implement a Metal projective path with the
**same mathematical product semantics**, not a lower-quality approximation.

Required mapping remains conceptually:

`source = H(W^-1(H^-1(q)))`

Support, in stages:

- Four Corners;
- automatic native Layer Plane projection;
- native 3D layer transforms;
- parenting;
- camera projection;
- extended native-text Layer Plane behavior;
- invalid/degenerate plane policy;
- Clamp / Wrap / Mirror;
- Bilinear Preview;
- Final Catmull-Rom Bicubic;
- alpha and 32f negative/>1 values;
- origins / expanded worlds / supported sparse semantics.

Per-frame geometry/homography is prepared once. No per-pixel heap allocation.
Render, overlay and picking remain semantically aligned with the same geometry model.

If the AE GPU interface cannot cover a project/depth combination, use the optimized
CPU fallback. Never render an unsupported frame approximately.

## 11. Wave 6 — fast CPU projective fallback

GPU availability must not be required for acceptable performance.

Optimize `PlaneRenderer` without changing its result:

### 11.1. Parallel rows / tiles

Reuse the proven bounded parallel scheduling model with safe cancellation.

### 11.2. Apple Silicon SIMD

Use NEON float4/channel operations for the sampling hot path and efficient 8/16
conversion where compatible with current rounding/tolerance.

### 11.3. Incremental scanline homography

For neighboring pixels, projective homogeneous numerators/denominator vary linearly.

Use scanline increments instead of rebuilding the complete matrix expression for
every pixel, with periodic exact anchor re-evaluation if needed to keep numerical
drift inside the unchanged quality contract.

### 11.4. Specialize hot loops

Separate hot-path variants where profiling proves value:

- Bilinear vs Bicubic;
- Clamp / Wrap / Mirror;
- dense vs sparse;
- common identity/affine/projective cases.

Avoid branches in the inner pixel loop when compile-time/runtime dispatch can happen
outside it.

### 11.5. Reuse prepared geometry/sampling state

Anything constant for the frame is prepared once outside the pixel loop.

## 12. Wave 7 — GPU pipeline overhead

Only after real profiling shows GPU overhead matters.

Candidates:

- precompiled `.metallib` instead of runtime shader-source compilation;
- persistent plan cache when geometry has not changed;
- avoid redundant plan copies to shared Metal buffers;
- tune threadgroup geometry from measured device characteristics;
- evaluate a two-pass separable Bicubic kernel for ordinary grids
  (horizontal 4 taps + vertical 4 taps instead of 16 direct taps);
- reuse bounded intermediate GPU buffers if two-pass wins.

Two-pass Bicubic is accepted only if output satisfies the unchanged quality contract
and real end-to-end AE measurements show a meaningful win after buffer/dispatch cost.

Do not remove `waitUntilCompleted` or alter command-buffer lifetime merely for speed
unless the AE GPU contract and real host evidence prove asynchronous return is safe.

## 13. Wave 8 — RAM Preview-specific optimization

Treat RAM Preview separately from Render Queue.

Measure and improve:

- time to first playable frame;
- time to fill a fixed preview range;
- warm replay behavior;
- invalidation after a true image edit;
- rebuild after a guide drag;
- memory footprint;
- cache retention across UI-only changes.

Rules:

- UI-only changes must not invalidate pixel cache;
- actual render changes must never leave stale frames;
- do not automate RAM Preview through unverified localized menu-command tricks;
- do not purge a user's global caches as a normal benchmark step;
- cached playback is not counted as renderer throughput.

## 14. Wave 9 — interactive latency

A fast final render with a laggy viewer is not acceptable.

Measure:

- guide drag → fresh displayed frame;
- Four Corners drag;
- 3D camera/layer movement;
- timeline scrubbing;
- parameter changes.

Goal: reduce perceived latency without changing the final renderer.

Viewer/interaction optimizations must not introduce a lower-quality hidden final
render path.

## 15. Wave 10 — profiler-driven advanced work only

Only pursue these if measurements still attribute substantial time to FSTR Stretch:

- wider SIMD over multiple pixels;
- tile-local cache layout/prefetch;
- Metal function constants/specialized pipelines;
- persistent static-deformation GPU plans;
- projective scanline vectorization;
- memory-bandwidth/layout improvements;
- additional CPU/GPU specialization.

Do not perform speculative architectural rewrites for single-digit wins.

A micro-optimization with <3% repeatable end-to-end improvement should normally be
rejected unless it unlocks a larger measured optimization.

## 16. Regression matrix after every major performance change

At minimum:

- 8 / 16 / 32 bpc;
- Preview Bilinear / Final Bicubic;
- Clamp / Wrap / Mirror;
- static and animated deformation;
- Grid Positions keyframes;
- wave animation;
- alpha;
- negative / >1 float;
- ordinary Layer Plane;
- Four Corners;
- native text;
- supported 3D;
- Full / Half / Quarter;
- sparse/expanded worlds and origins;
- MFR ON / OFF;
- RAM Preview;
- Render Queue / aerender;
- cancellation and rerender;
- save/reopen/restart;
- exact identity and project compatibility.

Any affected old bug fixture remains part of the regression set.

## 17. Before → Change → After evidence

Every accepted optimization must record:

- baseline candidate;
- changed candidate;
- exact commits / Build IDs / artifact hashes;
- identical fixture and host conditions;
- quality comparison;
- performance samples;
- p50/p95/spread;
- memory;
- CPU/GPU utilization if relevant;
- regressions and limitations.

Never transfer timing or host PASS to a different binary.

## 18. Stop / rollback criteria

Immediately revert or isolate a performance change if it:

- changes Final pixels outside the accepted contract;
- creates artifacts, stale preview frames or alpha/HDR errors;
- damages MFR determinism;
- makes cancellation unreliable;
- causes unbounded cache/GPU memory growth;
- requires security/host hacks;
- depends on undocumented behavior that cannot be bounded and verified.

Stop optimizing a path when measurement shows FSTR Stretch is no longer the material
bottleneck and the remaining time is dominated by AE, upstream/downstream effects,
encoding, I/O or hardware bandwidth. Record that boundary instead of degrading image
quality.

## 19. 0.9.4 final acceptance

0.9.4 is not accepted merely because unit benchmarks are faster.

Required:

1. complete quality/regression matrix PASS;
2. real target-AE Render Queue/aerender performance comparison to frozen 0.9.3;
3. real RAM Preview build/invalidation/replay comparison;
4. no meaningful stability/memory/cancellation regression;
5. exact candidate identity and package verification;
6. technical retrospective/update of reusable AE performance know-how;
7. explicit documentation of achieved speedups, test hardware and scope.

Performance multipliers may be advertised only from retained real-host measurements.

Public macOS release remains a separate final gate: Developer ID signing,
notarization/applicable stapling and Gatekeeper-clean quarantined download/install
must pass for the exact public artifact.

No merge/publication/release is implied by this plan.
