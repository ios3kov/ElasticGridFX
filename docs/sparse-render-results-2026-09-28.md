# Sparse CPU rendering checkpoint — 2026-09-28

Plan: sparse-render-stage-plan.md. Source baseline a871596; its native source
matches 0250b08 and the installed candidate 6d3b846. This is a staged core change,
NOT a claim that the installed AE effect or Corner Pin chain is fixed.

## Reproduction and correction

A 20x11 source rectangle at (19,13) in a 71x43 logical canvas, 32-bpc Final Bicubic,
was compared with the same data in an explicitly zero-filled full canvas. The
legacy compact-world entry produced 5,172 differing channels, maximum absolute
error 5.91231. The new test compiled with EGFX_BASELINE aborts on that discrepancy
(exit 134). The old 10-test suite passes, demonstrating a coverage gap.

Cause in this portable fixture: edge sampling resolves against the compact
storage rectangle, extending its edge colors into missing transparent regions.
A separate eg_render_frame_sparse entry resolves on the logical canvas first,
then maps into storage. Missing taps are zero, output outside that canvas is
zero, and empty input clears active output pixels without touching row padding.
This interpretation requires the host's complete logical-source request; an
arbitrary incomplete ROI cannot be assumed transparent.

Keep the legacy eg_render_frame and GPU-plan contract unchanged. No negative
indices enter Metal. Dense and sparse CPU kernels are specialized, keeping
boundary handling out of the dense pixel loop; all interpolation taps, Final
Catmull-Rom, float/HDR precision and identity rules remain. No full-canvas padded
image, new mutex or per-frame heap allocation after warmup is introduced in the
tested single-thread allocation fixture. CPU plan cache resets prevent sparse
sentinels leaking into the next dense frame.

## Executed source checks

Linux x86_64, independent build directories:

- Baseline GCC Release: 10/10 PASS; new negative fixture FAIL as expected above.
- Final GCC Release -Werror: 11/11 PASS, normal full fuzz/default 50,000-cycle soak.
- Clang Release -Werror: 11/11 PASS with the same default test scope.
- GCC ASan/UBSan/LeakSanitizer: 11/11 PASS, fuzz scale0.1, soak3,000.
- GCC TSan: sparse/MFR/determinism 3/3 PASS.
- Dense AND sparse allocation audit: zero counted steady-state heap allocations
  after warmup at each depth/quality, threads1. This is not an OS/GCD allocation audit.
- Shell syntax, YAML parsing and git diff whitespace checks: PASS.

New regression covers 8/16/32 bpc, both qualities, all three edge modes,
uniform/deformed axes, negative/HDR values, positive/negative/outside origins,
1-pixel source, positive/negative padded strides, output ROI/outside canvas,
empty input, invalid spans, cancellation, repeated cache use and concurrent calls.
Integer and identity comparisons are exact; deformed float max tolerance2.5e-5.

An earlier Clang RelWithDebInfo sanitizer build hit the command timeout and was
interrupted before tests. It is NOT recorded as PASS; the independent completed
GCC sanitizer run above supplies the actual result. CI/macOS results must be read
for the exact committed head. No actual AE or physical Metal test ran locally.

## Dense-path performance sanity, not an AE speed claim

Serial alternating baseline/candidate runs on the same four pinned Linux CPUs;
five samples each, two warmup frames, sixteen measured frames, 3840x2160 deformed
images, abort polling. No competing builds or tests were running during capture.
Median milliseconds per frame (baseline -> candidate):

| Case | Before | After | Before range | After range |
|---|---:|---:|---:|---:|
| 8-bpc Bicubic | 43.4203 | 44.4476 | 39.1796-50.8168 | 43.0600-51.8192 |
| 16-bpc Bicubic | 35.0998 | 30.9529 | 31.9228-37.1360 | 28.2625-32.8552 |
| 32-bpc Bicubic | 28.2829 | 29.2119 | 27.6986-29.9480 | 25.3200-41.0580 |
| 32-bpc Bilinear | 15.1038 | 13.1034 | 12.7464-19.6990 | 12.4353-15.3668 |

Shared-host noise is substantial; these values neither establish a speedup nor
close target performance acceptance. No median slowdown exceeded the 5%
investigation threshold in this run. Sparse speed versus a padded reference and
actual Render/RAM Preview latency/throughput remain to be measured on the target.

## Evidence and next stage

Retain sparse-before.log; baseline-tests.log/xml; sparse-final-tests.log/xml;
clang-tests.log/xml; asan-debug-tests.log/xml; tsan-tests.log/xml;
bench-compare.log/json and its comparison driver, source diff and file hashes.
PR checkpoint will identify the resulting commit and exact CI evidence.

Rust host wiring, world-origin/full-size proof, SmartFX bounds/empty requests,
output initialization and Corner Pin chain reproduction remain next. Do not
reinterpret this separate tested CPU API as a completed user-visible fix or
ship an unverified replacement. Perspective controls remain a separate plan.
