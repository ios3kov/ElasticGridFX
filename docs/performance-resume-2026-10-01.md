# Stage 8 resumed — quality-preserving Render / RAM Preview acceleration

Date: 2026-10-01. User explicitly resumes the performance requirement previously
skipped on 2026-09-29. Risk Critical; Delivery Development.
Rules: AE-Development-Rules 4.1.0 candidate / `05bd9a8d71c11280d972b96caf64b776f2a075d7`.
Baseline source: `2e3d066729eab4a6a6e23e5a45e8d20f60606f1a` (clean before tooling changes).
Accepted candidate history remains `2ccc5f6 / EGFX-bd19dee13315abc0b7e6090e`;
this record does not assert current loaded identity or replace its evidence.

## Findings and next experiment

Repository review covered performance/quality contracts, historical Stage 8
records, CPU/Metal benchmarks, sparse rendering, the current plane bridge,
host snapshots and plane render dispatch.

Historical 1080p/32-bpc/Final MFR OFF/ON process medians were 34.655/34.514 s
for 60 PNG frames. The difference was inside variability. Bounded profiles
observed PNG encoding; these runs do not identify a native renderer bottleneck
or measure RAM Preview. They used fd69988, before the current plane renderer.

Current host routes normal plane rendering through `eg_render_plane_region`,
and native comp-space text through `eg_render_plane_layer`. Both use
`PlaneRenderer.cpp`, a sequential per-pixel projective sampler. The older CPU
renderer's row caches, SIMD and GCD scheduling do not accelerate this path.
The architecture document still describes this implemented plane path as planned;
reconcile it with the source as part of this block.

PERF-01: add a native plane benchmark with varied pixels, negative/extended float
channels, alpha, dense/sparse storage, neutral/deformed/perspective geometry,
Final Bicubic and cancellation polling. Keep raw frame samples and output
bytes separately from timing. This is NOT AE or RAM Preview evidence.

PERF-02: measure/profile baseline before choosing a bounded optimization.
Compare at least five matched, alternating baseline/candidate samples after
warmup, retaining source/binary hashes, hardware/toolchain and spread.

PERF-03: retain complete sampling math and per-channel arithmetic order;
verify baseline/candidate byte equality at all depths and existing plane,
sparse/expanded/PAR/downsample, cancellation and concurrency regressions.
No approximation, lower-quality path or GPU enablement.

PERF-04: run sanitizers, strict warnings, Rust host tests/Clippy, Python and JSX
safety checks. A new installable candidate requires separate package and
target-host gates. Real AE Render and RAM Preview improvement remains NOT RUN
until the new exact candidate is tested with controlled host measurements.

## Baseline verification

macOS arm64 / Apple Clang 21; CMake Release, assertions enabled:
C++ 20/20 PASS. Rust 1.98.1 locked/offline host tests: 61/61 PASS.
All 12 JSX mock safety files PASS (not AE execution).
Python: 236 passed in sandbox; three native process/path checks were blocked by
sandbox permissions. Rerun of both affected modules outside sandbox: 14/14 PASS,
including those three cases. Aggregate baseline: 239 distinct cases PASS.
The initial Cargo 1.80.1 invocation failed before compilation; switching to the
already-installed Rust 1.98.1 resolved this toolchain mismatch.

Static code scanner exit 1: existing workflow-action pinning and persisted-checkout
credential findings require review; a rate-limit heuristic targets a local test,
not a network endpoint. Scanner output is not release certification.

## Status

Initial native timing/optimization: PASS for the standalone finite scope below.
Current candidate: revalidation after the exceptional-float correction below.
New target-AE Render/RAM Preview timing: NOT RUN.
No installation, project mutation, cache purge, merge or release.

## First bounded optimization and verification

PERF-01–03 implementation is complete for the native scope. Exact eligibility
requires zero cross-axis/perspective coefficients in both transform matrices,
no independent projected source and unit raster surface scale. Mapping/taps are
computed once per axis with the original double/float math. Four horizontal
float rows retain the original Bicubic intermediate values. Independent RGBA
channels are interleaved without reassociating any channel's sums. General
geometry retains per-pixel mapping and gathers each source pointer once.
No row threading, GPU enabling, precision/filter change or pixel approximation.

Internal cached/scalar matrix: exact pixel/report parity at 8/16/32 bpc,
Bilinear/Final, Clamp/Wrap/Mirror, neutral/deformed/mirrored/fractional/near-axis/
rotated/perspective/extremely scaled planes, sparse/expanded/empty input,
non-unit PAR/downsample basis, negative/extended floats, NaN/Inf, abort/retry and
concurrent independent frames. External comparison against the unmodified
`2e3d066` renderer: 24/24 small configurations and all seven timed scenes are
byte-identical, including output padding. These are native pixel checks, not
universal color-management or AE host certification.

Hardware: Apple M1 Pro, 8 cores, 16 GiB, macOS 26.6.2. Compiler: Apple Clang 21.
Both binaries use CMake Release -O3 -DNDEBUG -ffp-contract=fast -Werror, matching
the native host's contraction setting; no fast-math. One cold render, two
warmups, one measured frame per fresh process, five serial alternating pairs.
No agent builds/tests ran concurrently with the final timing series. Other
ambient machine activity is not controlled; raw ranges/outliers are retained.

| Native Final scene | Baseline median ms | Candidate median ms | Speedup |
|---|---:|---:|---:|
| 1080p, 8 bpc, deformed region | 369.482 | 29.9678 | 12.329x |
| 1080p, 16 bpc, deformed region | 380.568 | 29.8900 | 12.732x |
| 1080p, 32 bpc, deformed region | 373.180 | 30.3362 | 12.301x |
| 1080p, 32 bpc, sparse native layer plane | 298.564 | 28.2116 | 10.583x |
| 1080p, 32 bpc, sparse perspective | 229.699 | 179.793 | 1.278x |
| 1080p, 32 bpc, neutral region | 18.7396 | 7.09438 | 2.641x |
| 4K, 32 bpc, deformed region | 1492.500 | 116.892 | 12.768x |

The 4K peak RSS rose from 267,075,584 to 268,386,304 bytes (+1,310,720); 1080p
increases were about 0.54–0.56 MiB for deformed cached cases. Scratch is O(width +
height), not a full-frame intermediate; it is released at return.
Per-sample timings, cold times, p95/ranges, RSS, source/binary/benchmark hashes
and output hashes: [comparison JSON](performance-plane-comparison-2026-10-01.json).
The preserved private RGBA dumps and raw time logs are in the controlled workspace.
A large candidate outlier (95.809 ms in one 1080p float sample) is retained; the
reported acceleration is a median comparison, not a latency guarantee.

PERF-04 local verification: Release C++ 21/21 PASS, host contraction + strict
warnings C++ 21/21 PASS, ASan+UBSan 21/21 PASS (Apple leak detection disabled),
new independent-frame test under TSan PASS, locked/offline Release Rust 61/61
PASS, strict Clippy PASS, Python 239/239 PASS, all 12 JSX safety files PASS,
shell/YAML syntax and changed-plane Clang Static Analyzer PASS.
The static skill scanner's existing workflow pinning findings remain open.

CI now triggers native host regression for host/src/tests/tools/build changes,
not only the historical first-application files. The new cache regression is
in macOS preflight ASan/UBSan/TSan and portable CI TSan. Hosted/package results
are separate from these local checks.

## Next required host block

Prepare the clean exact-candidate package, verify extracted signature/payload,
then perform controlled target-AE pixel/Render/RAM Preview measurements with a
reversible installation and preserved projects. The current installed on-disk
metadata was read and matches the accepted `2ccc5f6 / EGFX-bd19dee13315abc0b7e6090e`;
loaded identity was not newly verified. It was not replaced.

Stage 8 remains IN PROGRESS: native improvement does not close actual Render or
RAM Preview acceptance. Perspective remains expensive and is the next profiling
area after exact-candidate host attribution. Do not enable GPU or add frame
thread fan-out without measuring host/MFR behavior.

Native baseline profile was repeated after all builds/tests stopped: one-second
Apple sample at 1-ms interval, owned benchmark process only. Sample SHA-256
`beb5d25c8a5ea474da08767e4e8261f04f4634740470328f16395b7d707ed1c6`.
This is qualitative attribution, not CPU percentages or a timing baseline.
The earlier exploratory sample overlapped a build and is excluded from final
profile evidence. Current scanner: 23 unpinned-action, 5 checkout-credential
candidates; no findings in changed plane/benchmark/regression files.

## Exceptional-float CI correction

Initial commit `aefeb4b` passed the physical Mac's complete 20-stage preflight,
two clean reproducible builds, signed bundle and extracted ZIP validation.
Build `EGFX-f900e4be32c6302ed3612a67`, package SHA-256
`07e4bfea17caa5c98c38893d572dc4fd44affca7bc6a8396e9fb931b25c44ae7`.
It was not installed or handed off: Linux Clang Release and GCC TSan failed the
new float pixel parity test. The initial timing table above describes its finite
sampling path; corrected-candidate timing is pending.

Local x86/Rosetta reproduction with contraction disabled exposed cached
`ffc00000` versus general `7fc00123`: SIMD and scalar additions chose different
NaN payloads when Infinity*0 and an input NaN met in the same channel. Finite
pixels were not the failing case. The correction resamples only NaN output
channels with the original scalar loop; no tolerance change or NaN canonicalization.
The test keeps byte equality and now prints the first differing element/bit pattern.

The benchmark accepts explicit 32-bpc exceptional inputs. Against the unchanged
`2e3d066` renderer, all eight dense/sparse region/layer/perspective/identity scenes
pass on arm64 and all eight pass on x86 via Rosetta, including output padding.
See [exceptional-float evidence](performance-plane-nonfinite-comparison-2026-10-01.json).
These checks do not establish native Intel Mac/Windows/AE compatibility. Strict
host-math C++ 21/21 and arm64 contraction-on/x86 contraction-off cache tests pass;
sanitizers, hosted CI and corrected timing/package gates are being repeated.
