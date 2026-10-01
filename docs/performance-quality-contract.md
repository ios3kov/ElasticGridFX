# Render / RAM Preview performance and quality contract

User requirement, 2026-09-28: Render and RAM Preview must be extremely fast with high quality and no degradation. This is a release requirement, not a claim about current performance. It applies to both the native renderer and actual After Effects workflows.

## Non-negotiable quality

Final stays Catmull-Rom Bicubic. Never silently substitute Bilinear, lower resolution, reduced bit depth, relaxed numerical precision, missing interpolation taps, or stale/wrong frames to make a timing look better. User-selected AE preview resolution is recorded and must be identical in before/after comparisons. Existing explicit Preview quality remains available but is not evidence for Final performance.

Preserve 8/16/32-bpc semantics, alpha and intended HDR/negative float values. CPU exact-identity requirements remain unchanged. Existing real Metal parity limits (max absolute 2.5e-5, RMS 3.0e-6) are not relaxed. A correction to demonstrably wrong baseline pixels is documented separately, not disguised as either a speed benefit or a quality regression. Dense and sparse checkouts must agree with their correctly zero-filled layer reference.

## What must be measured

Use a saved, versioned target-AE fixture and a known loaded candidate. Record AE/macOS version, CPU/GPU, memory, composition size/frame rate, working space, bit depth, MFR, effect state, input dimensions/origins, and active CPU/Metal path.

Measure separately:

- Render Queue/aerender: cold first-frame latency, warmed per-frame p50/p95, total time and peak memory.
- RAM Preview: time to generate/cache a fixed frame range; time to first playable frame; cached playback behavior; invalidation/rebuild after changing one guide or wave parameter. Cached playback FPS alone is not renderer speed.
- UI: guide-drag response and fresh rendered updates, not merely overlay redraw.
- Internal attribution: AE checkout/preparation, sampling-plan generation, CPU rendering, GPU upload/dispatch/readback where used, synchronization, allocations and cache reuse.

Use identical inputs/settings, serial alternating baseline/candidate runs, warmups and at least five measured samples; retain raw times and distributions. Do not run competing builds/tests during profiling. No speed claim when differences are within noise. A reproducible slowdown above 5% is an investigation gate, not automatic permission to lose quality; confirm the threshold on the controlled target fixture before approving an optimization.

Absolute ms/frame and preview-build targets must be frozen after the actual target-machine baseline is available. Do not invent a universal real-time/4K/8K guarantee from an unspecified machine or from a Linux microbenchmark.

## Optimization order

First remove incorrect pixels. Then profile and target the measured bottleneck: unnecessary full-frame requests/copies; excessive empty-region work; rebuilding identical plans; allocation or thread oversubscription; redundant CPU/GPU transfers. Evaluate SIMD, existing row caches, MFR and hardware Metal only where measured evidence supports them. Keep unchanged dense-frame hot paths free of unnecessary sparse-boundary work. Do not enable the currently disabled AE GPU dispatch until its real host path passes correctness and lifecycle tests.

Every performance-sensitive change requires a recoverable baseline, before/after pixel comparison and timing sanity. Only improvements verified in real AE can close the Render/RAM Preview release gate. Portable checks are supporting evidence, not substitutes.

## Current state

2026-10-01: the user resumes performance acceleration. The current source baseline
is `2e3d066`; Stage 8 is IN PROGRESS. The fd69988 measurements below are retained
historical evidence, predating the currently active plane renderer. Current work
and exact verification: [performance-resume-2026-10-01.md](performance-resume-2026-10-01.md).
No new AE or RAM Preview speed PASS is implied by standalone native benchmarks.

### Historical fd69988 measurement checkpoint

Pinned installed native candidate: fd69988 / EGFX-f442513cb6528f14295d6d45.
Stage 7 target functional smoke passed; see current-status.md for exact evidence
and scope. Stage 8 has an initial 1080p/32bpc/Final Bicubic process-start baseline.
Next measurement uses the same immutable AEP, OFF/ON warmups followed by five
serial alternating OFF/ON pairs. Acceptance requires unique mapped candidate
identity, unchanged installed payload, successful exits, 60 valid outputs per run
and byte-identical output across both requested modes. Retain coarse host timing
records without treating whole-second reports as accurate per-frame latency.
Cache state remains uncontrolled: no cold/warm-cache or renderer acceleration
claim from this experiment. Actual MFR concurrency is not instrumented.
RAM Preview, HDR target fidelity and physical Metal performance remain open.
No production release, new install or main merge is authorized by this document.
