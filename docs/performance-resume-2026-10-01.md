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

## Initial checkpoint (before corrected candidate installation)

Initial native timing/optimization: PASS for the standalone finite scope below.
Corrected native candidate: PASS as recorded below; target-AE checks remain NOT RUN.
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

## Corrected candidate — final native verification

Source `41283e3d0935160da669d7ec69bc4ae01d51f264`. This is a new timing record;
the initial JSON/table above remains historical evidence for aefeb4b. Same
hardware, compiler and serial five-pair protocol; all builds/tests stopped before
the final series. 24 small finite cases and seven timed scenes are byte-identical.
The 16 separate exceptional-float external checks are also exact.

| Native Final scene | Baseline median ms | Candidate median ms | Speedup |
|---|---:|---:|---:|
| 1080p, 8 bpc, deformed region | 381.4780 | 30.0448 | 12.697x |
| 1080p, 16 bpc, deformed region | 378.9990 | 30.3583 | 12.484x |
| 1080p, 32 bpc, deformed region | 386.4830 | 32.0415 | 12.062x |
| 1080p, sparse native layer, 32 bpc | 300.9650 | 30.7686 | 9.782x |
| 1080p, sparse perspective, 32 bpc | 227.4470 | 180.0100 | 1.264x |
| 1080p, neutral, 32 bpc | 19.8748 | 7.1328 | 2.786x |
| 4K, deformed region, 32 bpc | 1632.9900 | 127.1450 | 12.844x |

Full raw timing/RSS ranges, hashes and samples:
[corrected comparison JSON](performance-plane-corrected-comparison-2026-10-01.json).
4K peak RSS: 267,059,200 -> 268,369,920 bytes (+1,310,720). The 1080p/16-bpc
96.7042-ms candidate outlier is retained. No latency guarantee or host FPS claim.

Corrected C++ strict host-math and ASan/UBSan suites: 21/21 each; TSan cache matrix
PASS. Linux GCC/Clang, static analysis, ASan/UBSan, TSan and both regression jobs
PASS at 41283e3. Complete local physical-Mac 20-stage preflight PASS, including
Rust 61/61/Clippy, real Metal gates, dependency/SBOM audit and two clean
reproducible builds. Signed bundle and real ZIP extraction verification PASS.
Hosted macOS validation is separate and was still running at this checkpoint.

Validation Build `EGFX-ce45a845413a943552df0258`, source 41283e3 (clean), macOS arm64,
Final ZIP SHA-256 `a3c001599dc615996a666eac2a6976b2576adf4a0d5ccd4f85cc1daf86dbf362`.
Ad-hoc signature is for this bounded development test, not a public release.
Package/preflight/raw evidence is retained in the controlled workspace; no private
RGBA/log files are uploaded to the public PR. Installed/loaded identity and
Render/RAM Preview results are NOT RUN. A reversible test-install authorization
was answered explicitly: temporary replacement is authorized. Candidate 41283e3
is installed for test; exact original is retained and verified in transaction
backup `EGFX-update-b2a4807ee046479ab986a4bd9b1a972b`. Live identity/pixel smoke
is in progress. No user-project/cache mutation.

The user requested a fresh canonical-rules read during the package block.
AI_ENTRYPOINT was read first, then applicable updates. Continuing baseline:
v5.0.0 candidate / `b27f45467e0a9152fc82c1072438dfed07f0c36e` (stable published v4.0.0).
The earlier 4.1.0 reference remains the initial evidence's provenance. Current
scope is covered by the existing quality contract; no new Stage 0/reference audit.
Apply AI-STATE-001, AI-AUTO-001 and API-SOURCE-001 for subsequent host work.
No new/changed Adobe API calls were introduced by the sampler optimization.


## Target-host validation checkpoint

All eight hosted checks at exact source 41283e3 PASS, including macOS/Metal.
The user explicitly authorized temporary installation and AE-only closure of
projects without saving. Test transactions retain the exact original bundle;
normal asynchronous Adobe helper shutdown is awaited, never force-terminated
or excluded from the replacement guard. An initial series stopped safely on
live helpers and retained its candidate warmup without claiming a pair result.

The corrected candidate's live GUI-AE and separate aerender image UUID/path
match the sealed binary; both identity checks PASS. Aerender returned success
and 60 correctly sized PNGs for the saved animated 1080p/32-bpc Final fixture
(project SHA-256 650f2eb5f4bdfe859b7ff091c1cdb4203fa7f2c46ed6bf1856e7dde00067da3d).
These instrumented observations are not benchmarks. The unchanged legacy smoke
produced seven ordinary images and passed five partial pixel checks, but no
Adjustment Layer chain frame; the complete smoke remains BLOCKED. The original
plugin comparison is pending.

Five alternating target-host pairs are IN PROGRESS. Output PNG byte hashes
differ between versions and between new-candidate runs. Accept no performance
claim until independent pixel/encoding comparison explains these differences.
No competing agent builds/tests run during the series. RAM Preview remains
NOT RUN; Stage 8 remains IN PROGRESS.


## Host series rejected; measurement precision correction

The alternating series retained nine successful runs (one warmup each, four
measured candidate and three measured baseline), then rejected a changed process
key at baseline run 9. The original key-changing observation was not recorded;
do not infer a zombie/exit race or relax the guard without reproduction. The
runner now preserves that rejected observation for diagnosis. Original payload
was restored before the rejected baseline run, and all Adobe hosts subsequently
stopped normally. Five measured pairs were NOT obtained.

Actual PNG headers show RGB8, although the project rendered at 32 bpc. Independent
decoding finds max difference 1/255 in 838–896 channels across 60 frames for
candidate comparisons; a baseline repeat also differs in 19 channels in its
last frame. Exact equality is FAIL and remains FAIL. Adobe documents dithering
when converting/rendering/exporting to 8 bpc in
[Color basics](https://helpx.adobe.com/after-effects/desktop/adjust-colors/color-basics/color-basics.html);
this is a possible explanation, not established attribution. No before/after
Render acceleration claim is accepted.

[Rejected raw timing/pixel record](performance-host-observation-2026-10-01.json).
[Additional exact native pattern record](performance-host-pattern-parity-2026-10-01.json):
16 original/current C-bridge comparisons at 1080p use the actual opaque pattern
normalized to float, frame times 0/3/4/59, both last-center/buffer-size extents and
divide/multiply normalization. Complete raw float arrays compare byte-for-byte;
SHA-256 is retained. This excludes AE's source conversion and output encoding.

New fixtures default to the straight RGBA16 template already used by the unchanged
plane gate. Validate actual settings and encoded PNG precision/channels; record
controlled working space/linearization in schema 2. Historical schema 1 remains
readable. Reject wrong precision, missing alpha, matting, resize/crop and unsafe
project ownership; no filter, project render depth or pixel tolerance is lowered.
All Python tooling tests: 240/240 PASS. All 12 JSX control-flow files PASS (mock
safety only). The production plugin source/package is unchanged from 41283e3.

The Mac locked before the next GUI run. The user was asked to unlock manually;
independent tooling/source work continues. Original is restored. Full target
pixel matrix, RGBA16 Render series, RAM Preview and interaction gates remain open.


## Optional callback attribution — source checkpoint

All eight hosted checks at `1fcb0f3` PASS. The output-precision/process-observation
corrections are committed separately from the native optimization.

Add optional `render-diagnostics` instrumentation to existing callback branches;
no new Adobe API, cache purge, render decision or parameter/state change. Default
builds contain no diagnostic logging/locks. The bounded private CSV records phase
endpoints, rational frame time, dimensions/depth/route, completion/output flags
and process-relative starts; failed callbacks remain distinguishable. The reader
pins Build ID/schema and rejects malformed/incomplete observations; full-cap logs
are truncated. Missing callbacks cannot establish cache hits and observed interval
overlap cannot prove CPU concurrency. Logging itself perturbs execution; repeat
acceptance timing with the uninstrumented exact candidate.

Feature identity is now explicit in Build Identity. A default and instrumented
package from identical source must differ. Feature Rust 65/65, strict Clippy
and Python 246/246 PASS. Package sealing, loaded identity, real callback capture and pixel parity
remain pending. This source/tooling block does not accept target throughput or
RAM Preview; the Mac is still locked and the original installed payload restored.


## Cache-version separation before fresh host profiling

Ordinary observer-source checkpoint `dabf4d1` passed complete physical-Mac
20-stage preflight, Rust 61/61, clean reproducible builds and sealed/extracted
bundle verification. Retain its artifact; no installation was performed.

Direct resource inspection shows accepted and 41283e3 plugins both used encoded
0.9.3 Develop build 1 (`eVER` 301057). The SDK cache guide identifies effect
version as part of the cache key. This adds an uncontrolled confounder to earlier
process-start runs; it does not prove cache reuse or explain the pixel mismatch.
The next checkpoint selects existing effect Develop builds 2/3 for ordinary and
instrumented variants. Existing pinned PiPL and GlobalSetup propagation sources
were verified; no new API, saved-state change or cache purge.

A separate independent straight-RGBA16 decoder is prepared in the private host
workspace. Exact calibration includes low RGB bits and low alpha; it passes all
sample values with no premultiplication or 8-bit reduction. This is preparation,
not actual target pixel evidence. Final host comparisons remain pending Mac
unlock.

## Actual host route and source isolation — 2026-10-02

Exact-source f611312 passes all eight hosted CI checks. Ordinary Build
EGFX-6147dc406abc596e7f2d1b60 (Develop build 2) passes the physical-Mac 20-stage
preflight, clean reproducible builds, signature and extracted ZIP. Optional
observer EGFX-50e7279b47205ab9285ac7d9 (build 3) passes feature Rust 65/Clippy and
sealed/extracted package checks. Both exact images were loaded in AE 25.6
headless owned-project pilots; the original installation was fully restored.
These immutable packages remain f611312 when subsequent tooling/docs change.

The historical hashed 60-frame AEP is a legacy CPU fallback workload. Actual
observer records contain 60 successful 32-bpc legacy outputs and 60 pre-render
callbacks; sampling median 2.199 ms, total 144.440 ms, observed sampling overlap 1.
Logging/startup/output encoding are not acceptance timing. This scene cannot
measure the bounded plane optimization. No cache-hit or playback inference.

`_HIDDEN X-Factor 16` produces 60 independently checked noninterlaced 1920x1080
RGBA16 PNGs from that same AEP. Direct CLI Format/Channels overrides were
read-only, generated no images and were rejected despite exit 0. The saved color
context stays fixed by AEP hash; this pilot does not independently read it back
or convert the old AEP into a controlled new fixture.

Calibrated FFmpeg rgba64be decoding preserves all uint16 channels, including
low alpha/low bits. Original older-toolchain artifact versus f611312 remains
strict FAIL: maximum 2/65535, 44,689 changed channels in 53/60 frames, alpha exact.
Original repeat and ordinary/observer pair match all 60 complete PNG files.
No tolerance or filter change. PNG precision does not prove native float HDR.

Research clone fcdfe907e2896e53dd20d7ebfa93e0190da68e53 retains all accepted
2ccc5f6 native/bridge `src/` bytes. Only optional observation/feature identity,
existing effect Develop build 4 and supporting tests/checksums were added.
Current-toolchain observer EGFX-70cb973b561335f81da67803 passes feature gates,
loaded-image/fresh legacy callbacks and all 60 PNGs match ordinary f611312.
The physical baseline preflight included a checksum-only checkpoint before
Release compilation, with unchanged code/flags/test inputs; this is documented
rather than attributed to one uninterrupted source SHA. Research branch is local,
with no hosted CI or release claim. Inference: a build/toolchain change suffices
for the changed legacy pixels; plane optimization/observer are not required.
The precise FP/compiler/flag cause remains unresolved. Older-artifact FAIL is
retained separately from equal-toolchain source parity.

New schema-3 source explicitly selects Four Corners, reads back full-image
corners, and records expected plane_region; explicit footage Layer Plane records
legacy_cpu. The reader checks route/dimensions/depth and every rational frame
time against a hash-checked fixture. Schema 1/2 remains historical/readable.
Python 248/248 and all 12 JSX control-flow suites PASS. Actual new fixture,
40-frame plane/3D/AEP matrix, five accepted render pairs, RAM Preview and UI/cancel
checks remain BLOCKED / NOT RUN while the Mac is locked. No merge, release or
cache purge. [Public diagnostic](performance-host-diagnostic-2026-10-02.json)
retains records, identities, hashes, comparisons and limitations without project
paths, private samples or pixels.

## Unlocked host checkpoint — 2026-10-02

All eight exact b492e60 CI checks PASS. Actual 40-frame plane/3D/AEP matrix PASS
on original, ordinary candidate and same-current-toolchain original source.
The latter matches the candidate exactly in decoded RGBA16 for all 40 frames;
the older-artifact 29-channel FAIL is retained separately. A fresh two-host-turn
fixture has ready native binding and both observers confirm all 60 plane_region
frames, depth, identity and output precision. Independent decoded comparison
PASS across all 60. Sampling-phase medians 348.734/33.595 ms are diagnostic, not
accepted total render/playback performance. Original restored after observers;
ordinary pilot and unique uninstrumented Develop-5 control precede paired timing.
Python 248/248, 12 JSX suites PASS. Legacy chain capture and RAM Preview remain
open. [Scoped machine evidence](performance-host-matrix-2026-10-02.json).
