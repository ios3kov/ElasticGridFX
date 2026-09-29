# Current development / release status

2026-09-29. Branch: fix/final-validation, draft PR #5. **NOT READY FOR RELEASE.**

**Production cycle: Stage 9 of 10 IN PROGRESS; Stage 8 SKIPPED BY USER, not PASS.**
See development-stages.md. Stages 1–7 are completed for the current scope. Stage 8 is
performance/profiling (skipped on explicit user request), Stage 9 is the approved 2D/3D perspective plane, Stage 10
is final compatibility/release validation.
No main merge or production release in this stage. User authorized a reversible
test installation of the guide UI update, contingent on candidate checks and a
safe AE restart (never discard unsaved work).

## User-machine evidence now received

### Stage 9 requested guide UI affordances (source implementation)

Installation preparation found a pre-existing Release test defect:
test_metal_parity.mm placed the CPU reference render inside assert, so NDEBUG
removed the render and compared Metal against zeros (max_abs 1.50966, exit 4).
Moved the call to explicit checked execution without changing tolerance or any
renderer code. All 18 parity cases now PASS, max_abs <= 4.76837e-7.
Evidence: work/stage9-core/guide-install-metal-parity{,-fixed}.log.
This is standalone Metal evidence, not AE acceptance; installation still pending.
The determinism fixture had the same NDEBUG side-effect defect (reference renders
skipped). Assertions now remain enabled in all three Metal test translation units,
without changing production compile flags. Optimized sequential 384/concurrent
256 determinism runs PASS; parity rerun PASS. These fixes invalidate neither the
recorded failure nor the requirement to retest a clean final candidate.

2026-09-29: markers enlarged from 5 to 10 frame units, retaining screen-space
hit testing. Active viewer cursor uses open hand; guide capture uses closed hand
on Mac via a main-thread-checked AppKit shim and AE CUSTOM cursor protocol.
Mouse release/error/invalid topology and context exit/deactivation clear state.
Details and required live scenarios: guide-ui-v0.5.md.

Local arm64 release build PASS; Rust tests 15/15 and CTest 15/15 PASS.
Use explicit Rust 1.98.1 compiler path: default PATH Cargo/rustc 1.80.1 cannot
build edition 2024 (initial attempts failed, corrected without changing tools).
Static audit: review_required, existing workflow findings and test-auth heuristic;
no findings in changed UI/shim/build files. Report: work/stage9-core/guide-ui-audit.json.
Real AE visual/cursor acceptance NOT_RUN: installed immutable fd69988 unchanged.
No installable artifact is published and Stage 9 remains IN PROGRESS.

### Stage 9 initial geometry foundation

Latest slice: explicit positive finite raster-to-plane surface scale per axis,
shared by all bit-depth paths. This allows host-supplied downsample/PAR coordinate
bases while preserving exact identity copies and applying the inverse scale before
sampling. Uniform/anisotropic equivalence and invalid-scale rejection tests PASS;
15/15 Release CTest targets PASS. No host Half/Quarter/PAR acceptance is claimed:
the adapter must still establish AE coordinate conventions and avoid double camera
or PAR transforms. Native plugin/installation remains unchanged.
ASan/UBSan plane-render tests also PASS. Evidence retained in work/stage9-core
(Testing/Temporary/LastTest.log, plane-scale-sanitized, plane-scale-audit.json).
Static audit remains review_required with no findings in the changed renderer/tests.

Latest slice: added 8/16-bpc plane region APIs through a shared typed sampler.
Integer output rounds only after both Catmull-Rom axes, clamps sampled values to
255 / AE 32768, and preserves exact pass-through bytes. Float remains unclamped.
New test compares existing Final within one integer level and requires bit-exact
sparse/full output, padding safety, empty source and malformed-stride rejection.
This does not wire the new renderer into native AE; installed fd69988 is unchanged.
Verification: 15/15 Release CTest targets PASS; new integer-depth tests under
ASan/UBSan PASS. Static audit remains review_required with no findings in changed
renderer/new tests. Evidence retained in work/stage9-core: LastTest.log under
Testing/Temporary, plane-depth-sanitized and plane-depth-audit.json. No speed claim.

Latest continuation: standalone float PlaneRenderer now supports compact source
checkout and independently positioned output rectangles on a logical canvas.
Missing checkout taps are transparent, not stretched. Empty input yields zero;
outside/invalid-plane copying uses logical coordinates. Compact and zero-filled
full inputs are tested bit-exact, including cropped output/padding. Regions
extending outside the logical canvas are rejected explicitly; host integration,
expanded output, PAR/downsample and 8/16-bpc remain open. No native installation.
The initial new test failed to compile due to mixed pointer constness in a test
initializer list; corrected the list type and reran the complete checks.
Verification: 14/14 Release CTest targets and region test under ASan/UBSan PASS.
Evidence: work/stage9-core/Testing/Temporary/LastTest.log, plane-region-sanitized,
plane-region-audit.json. Static audit remains review_required for existing findings;
no findings in changed renderer/test files. No full-AE acceptance is claimed.

Latest slice: user approved outside/invalid-plane pass-through. Added standalone
dense 32-bit float PlaneRenderer: a single Catmull-Rom sample after coordinate
composition; exact copy outside/identity/invalid plane, structured fallback
diagnostics, stride/overlap validation and row-level cancellation. No channel clamp.
This new path is not wired into Cargo/native host. Sparse/PAR/downsample, 8/16-bpc,
overlay/handles/camera and real-AE acceptance remain open. Existing renderer and
installed fd69988 remain unchanged. Final-filter comparison and safety tests are
part of the new plane-render CTest target; this is not full Stage 9 completion.
Verification: Release build and 14/14 CTest targets PASS; plane-render test under
AddressSanitizer/UndefinedBehaviorSanitizer PASS. Flat-plane output agrees with
existing Final Bicubic within 1e-5 on the HDR/impulse fixture. Static audit remains
review_required (existing findings), not release PASS. Evidence is retained in
work/stage9-core/Testing/Temporary/LastTest.log and plane-render-audit.json.

Follow-up: standalone PlaneWarp now composes plane projection with the existing
inverse grid/easing calculation. Analytic test proves column motion follows a
slanted plane rather than screen-horizontal motion. Exact identity is retained;
invalid projection and outside-plane states are explicit, not fabricated source
pixels. Owned immutable evaluated guides reject malformed/nonfinite input.
All 13 Release CTest targets PASS on 2026-09-29, including new plane-warp tests.
Pixel sampling, sparse/PAR/downsample integration, AE controls and camera binding
remain NOT RUN/not implemented for this path; no installed plugin change.

The user requested skipping further performance acceleration work on 2026-09-29.
Historical Stage 8 results below remain valid only within their stated scope.
Added standalone PlaneTransform: normalized four-corner homography and inverse,
strict-convex validation, mirrored winding support, nonfinite/degenerate/horizon
rejection. No renderer, Rust ABI, parameter IDs, saved state or installed plugin
changed. This is geometry groundwork, NOT an AE perspective feature acceptance.
Verification 2026-09-29 on target Mac: Release CMake build and all 12 CTest
targets PASS; standalone PlaneTransform tests with AddressSanitizer and
UndefinedBehaviorSanitizer PASS. Static scanner remains review_required for
existing findings; no findings in the new geometry/test files. Evidence retained
under work/stage9-core (CTest LastTest.log and code-audit.json). No AE acceptance,
new plugin artifact, performance claim or release gate is implied.
Next: define and implement plane-local sampling/interaction wiring, preserving
one resampling pass and avoiding double application of AE layer/camera transforms.

### Stage 8 — initial measurements available; full performance gate open

#### Bounded profile — PNG encoding observed, native bottleneck still unproven

Runner `510ce07`, SHA-256
`f505fa38eaff8d0c4602ecfe24a13dfa3639547b2d7801f467db90d3df0148b6`.
Run `identity-probe-ef3b2cec0c39468e887d311686bea943`: PROFILE_CAPTURED,
three one-second/10ms-interval samples after 5, 20 and 40 observed output files.
Each confirmed the same owned aerendercore PID 69278, process key and fd69988
Mach-O UUID. Final exit 0, exact payload before/after, 60 outputs with digest
matching all twelve prior MFR-series runs. Instrumented time is not a benchmark.

Thresholded leaf histograms show PNGIO longest_match counts 60/71/54,
png_write_find_filter 5/absent/9 and deflate_slow absent/absent/8; color-conversion
functions also appear. Waiting threads dominate raw counts. These counts are
not CPU percentages; absence means below threshold or outside the short sample,
not zero cost. The result supports investigating PNG export overhead before
attributing the end-to-end baseline to native rendering. It does not yet prove
the primary ElasticGrid bottleneck or justify a native optimization.

Target UI inspection found an empty Untitled Project. Attempted fixture selection
through the native Open dialog was cancelled without opening/saving a project.
AE exposes only the window/menu accessibility tree, and observed UI automation
latency is unsuitable for precise first-frame/cache-completion timings. RAM
Preview remains NOT RUN; next step is a guarded host-side measurement of fresh
frames (separately labelled from playback) and reliable lifecycle instrumentation,
not timing delayed screenshots or using unverified menu IDs. No user cache purge.

Evidence: exported `ElasticGridFX-Stage8-profile.json`; full raw samples remain
private in the controlled workspace. Fourteen aerender tooling tests PASS,
including malformed/thresholded histogram checks. Static audit remains
review_required for existing unrelated workflow findings; not a release PASS.

#### Serial requested-MFR comparison — 2026-09-29

Runner commit `27241ee`, SHA-256
`917fe0adde3e3aee6dd506c6b6c7985704db3846c91065d51c169a9239751d30`.
Series `0ff9c4318e49481899ce2fc5f6b6e2b3`: OFF/ON warmups followed by five
serial alternating OFF/ON pairs on the identical pinned AEP and fd69988 plugin.
All 12 exits successful; mapped candidate and installed payload checks passed;
every run produced 60 valid 1080p PNGs with the same aggregate digest as the
previous baseline. No plugin change, GUI project mutation, cache purge or
competing build/test was performed during measurement.

| Requested MFR | Full-process median | Range | Nearest-rank p95 | Sampled core RSS maximum |
|---|---:|---:|---:|---:|
| OFF | 34.6547193 s | 33.4859687–35.7392201 s | 35.7392201 s | 2117337088 bytes |
| ON | 34.5143172 s | 33.2587832–34.7046569 s | 34.7046569 s | 1909194752 bytes |

Median difference is about 0.4%, within observed variability: **no demonstrated
acceleration**. Actual concurrent frame execution was not instrumented, so this
is comparison of CLI-requested modes, not proof of MFR utilization. The host
reports total render time 22–23 whole seconds for measured runs; per-frame
reports are retained with their whole-second resolution, not converted into
misleading millisecond latency percentiles. Startup/output/shutdown/observation
are included in wall time; child-process memory is excluded from core RSS.
Raw logs report Full/Best, RGB Millions of Colors PNG output and sRGB working
profile. The saved fixture remains 32bpc Final Bicubic, but PNG equality is not
HDR fidelity. Cache state remains uncontrolled. Do not compare this later series
to the earlier baseline as an optimization: no production code changed.

Evidence: controlled workspace series.json and raw logs; exported
`ElasticGridFX-Stage8-MFR-comparison.json` and companion evidence ZIP.
Tooling tests: 13 aerender unit tests PASS. Static skill audit: review_required,
not PASS; existing workflow action pinning/checkout-credential candidates remain,
plus a rate-limit heuristic on a local test (not a network auth endpoint).
Next: controlled cache/RAM Preview lifecycle measurements and bottleneck
attribution before choosing an optimization. Stage 8 remains open; Stage 9
perspective-plane requirements remain recorded, not implemented by this work.

#### Initial serial end-to-end baseline measured (Stage 8 still open)

Series `fbf5afad5a214b2a877167af9c50d4c0`: one warmup plus five measured
fresh-process runs of the same pinned 1080p/32bpc/Final Bicubic animated AEP,
60 frames, MFR OFF, max CPU argument 100. Apple M1 Pro, 8 physical/logical cores,
16 GiB RAM, macOS 26.6.2, AE 25.6x101. No cache purge or sampler was used.
The mapped text-file list proved the unique ElasticGrid binary path in each
owned aerendercore; exact installed payload/signature checked before and after.
Per-run runtime UUID is not sampled; the separate preflight established it.
All six runs produced 60 correctly sized PNGs with identical aggregate digest
`d6c1a18e3b99af44f9c4a4cf14cef610dde66f29fc4df9086088f081519ba206`.

Measured full-process seconds: 38.6251985, 38.6976323, 38.3036168, 42.6839684,
42.5909023. Median 38.6976323; nearest-rank p95 42.6839684; sampled maximum RSS
2450243584 bytes. Startup, PNG encoding, process/file observation overhead and
shutdown are included. This is not per-frame render latency, exact OS peak RSS,
cold-cache evidence, HDR fidelity or RAM Preview. Fixture color metadata was
inherited rather than separately recorded, so compare only this exact AEP until
an explicit color-settings fixture is generated. The spread is material; do not
claim small speed improvements against a single run. No optimization performed.
Raw logs, output frames and series.json are retained in the controlled workspace.
Next: explicit environment/fixture color record, frame-level timings, MFR pair,
controlled cache scenarios, and separately observed RAM Preview lifecycle.

Instrumented aerender preflight now PASS for identity and frame count:
`identity-probe-f3945ed06df441ab82369bb224ad43e6`, target aerendercore PID 58659,
live UUID A7C24F56-9776-3971-9CB7-972DD1F7AF8F with pinned installed fd69988
payload verified before/after, 60 output files, digest
`d6c1a18e3b99af44f9c4a4cf14cef610dde66f29fc4df9086088f081519ba206`.
Sampled peak RSS 2368389120 bytes; instrumented elapsed 36.4678 seconds is NOT
a speed baseline. The macOS renderer is detached (PPID 1); association uses its
exact executable, `-aerenderpid` token and unique output path, not its name alone.
The stdout marker is absent, and `/usr/bin/time` on the launcher measures the
wrong memory scope. `tools/aerender_identity_probe.py` records separate diagnostic
evidence; the existing benchmark continues to fail closed rather than accepting
an unproven Build ID. These are internal diagnostic runs, not release artifacts.
Next: integrate render-process observation without sampler-contaminated timing,
validate output repeatability, then collect at least five controlled samples.
RAM Preview and optimization remain NOT RUN.

Follow-up: the clean target project became available. The fixture failed at
output_template because the exact `PNG Sequence` template is absent. A bounded
target probe found `png`, whose observed format is PNG Sequence, with Resize
and Crop false. The generator now accepts that explicitly observed template
and verifies PNG format and unchanged output geometry. Mock tests reject resize.
Fixture `EGFX-perf-160f855b30cc4ba3a4d609139a168e85` was then PREPARED in real
AE: 1920x1080, 32bpc, Final Bicubic, animated, 60 frames. No timings yet.
The temporary exploratory template probe emitted an uncaught JavaScript dialog;
its broad settings traversal was replaced with primitive-only named fields and
guarded error/cleanup handling. The production fixture generator completed with
exit 0; this probe error is not evidence of a native renderer failure.

Authoritative rules fully re-read on 2026-09-29. Local aerender 25.6x101 help
confirms `-mfr ON|OFF max_cpu_percent`; the prepared harness omitted that
required third argument. It now supplies 100, with a command-contract test.
The first real synthetic 1080p/32bpc fixture attempt
`697e395cad7347bc93454d245a3b6720` failed at render_queue before save.
No timing baseline was collected. Per-operation stage and numeric host error
diagnostics now distinguish template, output path, output format and save errors.
Depth restoration is moved before owned-project close to avoid dirtying the
replacement project. A later read-only guard found a saved non-test project
with six items, revision 3091. It was preserved; authorization to discard an
empty test project does not cover it. Further target runs require an idle
empty unsaved project. Offline tooling checks can continue independently.
An additional audit finding remains open: the aerender harness expects a stdout
Build ID marker, while the native build embeds the marker in its binary. Actual
emission is unverified; use observed live-image identity in the render process
if stdout lacks the marker, without weakening the gate. RAM Preview, cold/warm
timings, peak memory and optimization remain NOT RUN. No speed claim is made.

### Stage 7 target acceptance PASS — latest evidence

Run `EGFX-AE-dbcc0ec13f7e48b387f55aa749c623a6` completed with exit 0 and
functional PASS on AE 25.6.0 arm64, PID 42039. The immutable loaded candidate
is fd69988 / EGFX-f442513cb6528f14295d6d45, proven by live path/UUID plus exact
installed payload/signature before and after execution. All ten frames were
captured; seven relational comparisons and per-frame dynamic-range/alpha coverage
checks PASS. Static and moved Corner Pin PNGs were also visually inspected:
no black output or bright streaks observed in this fixture.

Runner: `73714087703e645d84dca51cf5c592e0c97c458f`.
Runner ZIP SHA-256: `6ed14e1cddc4cffde3c2f276f8039ebaca093d7dab7e390b2b42b61774127b7c`.
Evidence ZIP SHA-256: `65dfdf4ea8543eae4a70f21cc618393b80ead3f52ebc7ac95cadc13857635a70`.
The observed None working-space sentinel is the string `None`, not empty text;
linearizeWorkingSpace=false is independently verified. Initial color settings
and bit depth are restored; test comps, footage, solid and empty folder removed.
Regression: 179 Python tests plus JSX ownership/capture suites PASS.
Scope is the controlled 32bpc-host/SDR-PNG smoke, not HDR fidelity, arbitrary
user grids, GPU, performance or full compatibility. The nested generic smoke
record still lists broader release requirements; top-level functional PASS is
the scoped Stage 7 verdict. Release remains BLOCKED. Stage 8 profiling is next.

All entries below are historical evidence, including superseded blockers.

### Fixture calibration and Corner Pin oracle

Target probe found inherited sRGB with linearizeWorkingSpace=true. The fixture
now explicitly uses an unmanaged, non-linearized 32bpc project and restores
the prior project color settings during cleanup. Solid source and its empty
test-created folder are removed. The target probe confirmed zero remaining items.
Report `EGFX-AE-46d6329b9e194f848626a9a0f3a7faa8` then established that every
bypass RGB sample equals the known input times 0.1 within 2/65535; alpha is
unchanged. This local observation is the basis for the export calibration,
not an assumption that all AE exports behave this way. The comparator accepts
only scale 1 or 0.1 and requires a full-pixel match against the effect-disabled
fixture before applying that same scale to every frame. PNG remains an SDR
smoke oracle, not HDR/extended-range proof. Arbitrary darkening is rejected.
Moved Corner Pin uses the known four corners with opaque interior, transparent
exterior and a 3px antialias exclusion band. Interior holes and opaque exterior
are regression-tested. Other frames still require full opacity. Prior generic
alpha/dynamic-range refusals remain historical BLOCKED reports. A fresh packaged
run is required for the new oracle; successful retrospective analysis is not
a replacement for that run.

### Target capture evidence after authorized project reset

The user authorized discarding the empty unsaved test state on 2026-09-29.
Runner `76e3f21cb50b0b873cbf8329db474db2637cb60c`, package SHA-256
`29ee2cd9ec55ec6b3365af0eb6c5ca00bb152be7eb83ef7e98944d90fbc2bce2`,
produced report `EGFX-AE-0ac17ad731d24d3e9cea4bdd9c2198e5`.
Identity PASS, JSX CAPTURED all ten PNGs, functional acceptance BLOCKED:
the comparator rejects low dynamic range even in bypass (red 0.00096–0.07912).
This is not yet localized to a host color configuration or fixture issue.
The moved Corner Pin also has legitimate transparent exterior pixels while
the current comparator demands opaque alpha everywhere. Review this oracle
against geometric coverage; do not broadly relax alpha or brightness thresholds.
Bounded PNG publication wait was necessary: immediate File checks failed
before the host completed writing. Removing the wait was reverted after reading
the successful capture record. The pixel cleanup leaves the addSolid-created
source item, so the next safe cleanup must verify that test-owned object before
resetting the project. No general occupied-project reset is authorized.
Next: control and record the fixture color configuration, fix owned-solid cleanup,
and validate the Corner Pin exterior against an explicit coverage expectation.
Stage 7 stays open; this is neither renderer PASS nor renderer failure evidence.

### Stage 7 runner path fixes — 2026-09-29

Report `EGFX-AE-fd8177e566514d90b6227201607a6a6e` reached ARMED from
CLEAN revision 1 and completed owned-project cleanup. Identity was blocked by
two sampler privacy masks and a resource symlink inside MochaAE.bundle.
The runner now matches separated complete masked path segments against the
independent libproc observation. Visible path segments, UUID, payload, signature,
same-process and conflict gates remain required. Adjacent/partial masks fail.
Conflict discovery treats .bundle as a native bundle boundary like .plugin:
it reads bundle identity and detects ElasticGrid even under another suffix,
without scanning nested resource links. Unknown links in the discovery tree
still block. No plugin payload or installed files are changed.
The retained sample now maps to the expected UUID and the application plugin
root scans successfully. These diagnostic checks are not functional PASS;
a new packaged real-AE run is required.

Packaged runner commit `45594f9dd264e1aeff7109ce236f4387cfbb39bc`
(ZIP SHA-256 `b416b615c1a754db797e1cdd5a4052074a57f74b614518b14e98ae5cd0d78e24`)
was executed as `EGFX-AE-fff7f118bbb44553b4c55012521dc18c`.
Live identity PASSED in AE 25.6.0 arm64 PID 42039: expected UUID/path/payload,
no scan errors or duplicate candidate. Pixel capture exited 90 after writing
bypass.png; capture status FAIL, stage capture, pixel assertions NOT RUN.
Functional acceptance remains BLOCKED. This does not isolate the failure to
the renderer: the previous capture record omitted frame-specific diagnostics.
The next runner records the exact frame and numeric host error/line without
arbitrary exception text. Local validation: 177 Python tests, both smoke safety
JS suites and native libproc fixture checks PASS for the path fixes.

Installation receipt EGFX-update-bf076f0a16f44fb983e717d9e8779b3d reports
INSTALLED_FOR_TEST, no install/rollback errors and a retained original backup.
The installed candidate is 6d3b846463410f37198fda4b625e56e4cea44c22 /
EGFX-603e9d3e4025d271e0488201; its ZIP SHA-256 matches the pinned manifest.
This supersedes the earlier OLD/no-backup startup report, not its historical facts.

Enabled/disabled screenshots show bright streaks with ElasticGrid active and
none with it disabled, on a nonuniform grid. The custom Grid Positions label is
clipped; its full runtime Build ID is not established by those screenshots.
The user also reports black output for ElasticGrid -> Corner Pin on an
Adjustment Layer while the Fast Blur comparator works. This chain failure is
not reproduced in a local AE session or closed by the source tests below.

## Confirmed requirements

See perspective-plane-plan.md: repair effect-chain compatibility separately;
then add four-corner placement of the deformation plane and layer/camera-driven
3D perspective, with the same transform for grid, handles, hit tests and pixels.
Those modes are **planned**, not implemented. Degenerate/outside-plane behavior
is an explicit design decision, not an inferred user requirement.

Render and RAM Preview must be extremely fast without hidden quality reduction.
performance-quality-contract.md remains authoritative; target timings require
real AE measurements. No claim of real-time 4K/8K on unspecified hardware.

## Implemented and verified in this stage

An explicit CPU entry, eg_render_frame_sparse, interprets absent pixels in a
compact returned world as zero on an explicitly known logical canvas. It avoids
stretching the compact rectangle's edges and needs no full-canvas image copy.
Empty input clears active output pixels. Legacy CPU and GPU-plan ABI/semantics,
17 saved parameters, Rust host and installed candidate remain unchanged.

The AE SmartRender source now calls the sparse entry only after requesting the
complete logical canvas in SmartPreRender. Compact returned worlds use their
host-provided origins; absent pixels are transparent. Empty adjustment input is
explicitly cleared. The installed user candidate is still the older 6d3b846
binary, so do not describe the user's current AE output as fixed yet.

Local source checks: baseline 10/10; GCC Release 11/11; Clang Release 11/11;
ASan/UBSan/LeakSanitizer 11/11; TSan sparse/MFR/determinism 3/3. Sparse/dense
steady-state allocation checks pass. Detailed reproduction, thresholds, raw-log
names and performance limits: sparse-render-results-2026-09-28.md.
Read exact resulting commit's Actions/PR checkpoint for CI and macOS outcomes;
queued jobs and hardware compile-only stages are never host/runtime PASS.

## Exact fd69988 candidate

Exact-head final validation, PR CI, update-safety/live-diagnostic workflows and
macOS source/package gate have passed. The immutable macOS artifact has Build ID
EGFX-f442513cb6528f14295d6d45 and package SHA-256
68c135f1a2a9390a0032a31d9073c8111bff6dd43691b9222ec50fad3913982e.
A new reversible updater is pinned specifically to installed 6d3b846 -> fd69988;
it refuses any different current payload and retains 6d3b846 as rollback.
This still does not constitute AE runtime PASS.

## fd69988 updater delivery integrity defect

The first attempt to run the fd69988 updater on the target Mac stopped before
any plugin replacement with `Installer files changed; download the verified
package again`.

The delivered ZIP was re-inspected byte-for-byte. Its embedded fd69988 plugin
payload is intact and still hashes to
`68c135f1a2a9390a0032a31d9073c8111bff6dd43691b9222ec50fad3913982e`.
The updater packaging layer was wrong: `InstallToolIdentity.json` correctly
recorded `tools/authorized_update.py` as executable, but the ZIP writer forced
every non-`.command` member to mode 0644. The updater's own self-integrity
check therefore rejected the package before touching the installed plugin.

The packaging fix preserves each member's executable bit from the identity
manifest, validates ZIP member modes after writing, adds a regression test for
an executable Python tool, and on macOS CI extracts the final ZIP with `ditto`
then re-runs the exact hashes/execute-bit identity comparison. The failed
delivery remains BLOCKED evidence and is not reused.

## Target-Mac acceptance attempt — installed candidate mismatch

A corrected target acceptance launch on 2026-09-29 stopped before live-image
sampling or JSX/pixel execution with:

`BLOCKED: manifest identity mismatch`

The runner had already verified its own pinned fd69988 manifest/package before
reaching the installed-payload check. Therefore this evidence means the installed
ElasticGrid payload does not match the pinned fd69988 BuildIdentity expected by
Stage 7. No AE functional result can be inferred; status is **AE NOT RUN**.

This is consistent with the last confirmed installation receipt in this branch,
which identified 6d3b846 / EGFX-603e9d3e4025d271e0488201. Stage 7 now requires
the existing reversible updater to move that exact installed candidate to the
immutable fd69988 payload, retain 6d3b846 as rollback, then restart/open AE and
rerun the corrected acceptance package. A successful installer receipt is
required before the next functional attempt is counted.

## Target-Mac acceptance attempt — live path diagnostic alias

After the corrected fd69988 installation, the returned acceptance report confirms
the exact installed candidate on disk:

- Build ID: `EGFX-f442513cb6528f14295d6d45`;
- commit: `fd69988c10b25268eb8cad6ee6ced7f6a28bee9d`;
- binary SHA-256: `8fc61c9c4dd20f1f398e470d3f0b75003bcfac4aa45a4669de120b188865426d`;
- Mach-O UUID: `A7C24F56-9776-3971-9CB7-972DD1F7AF8F`;
- AE 25.6.0 arm64 was running.

The run stopped before JSX/pixel execution because the live-image diagnostic
reported `Native path contradicts reported image`. It also recorded a ValueError
for the app-parent `Plug-ins` scan root. Pixel status therefore remains NOT RUN.

Research found that macOS can expose the same Data-volume object through both
`/Users/...` and `/System/Volumes/Data/Users/...`. The diagnostic is updated
to normalize only these known VFS/firmlink aliases for text constraints while
still requiring the pinned on-disk payload, Mach-O UUID and underlying file
identity. A symlinked scan root is ignored only when it aliases another
independently listed plugin root; unknown symlink targets remain blocking.
No plugin or user state is changed by this diagnostic fix.

## Acceptance delivery execute-bit defect caught before handoff

The first acceptance package built after the live-path diagnostic fix was inspected
before user delivery. Its bytes and embedded fd69988 payload were correct, but the
ZIP writer forced Python members to mode 0644 while `AcceptanceToolIdentity.json`
marked `tools/live_identity.py` and `tools/target_ae_acceptance.py` executable.
That package is not distributed.

The acceptance packager now preserves each member's executable bit from the
identity manifest, validates ZIP modes after writing, includes a regression test
for executable Python members, and on macOS CI extracts the final ZIP with
`ditto` and re-runs the exact hashes/execute-bit comparison. The plugin payload
remains the same immutable fd69988 candidate.

## Real acceptance report — effect not resident before identity sampling

The returned run `EGFX-AE-549be4237ee647eabe9dba06df8c665b` proves the exact
fd69988 payload on disk and AE 25.6.0 arm64, but `loaded_images` was empty and
pixel status remained NOT RUN. The orchestrator sampled live process images before
the intentionally empty project had instantiated ElasticGrid.

The corrected sequence is now:
1. guard an empty, unsaved, clean test project;
2. create only test-owned footage/comp/layer and instantiate ElasticGrid;
3. sample the same AE PID and prove pinned path/Mach-O UUID/Build ID;
4. close only that exact armed test project without saving and create a fresh
   empty project;
5. run the existing ten deterministic pixel captures;
6. verify the installed payload again.

If ownership changes, cleanup refuses to close the project. The plugin binary is
unchanged. Conflict scanning is also narrowed to Adobe-documented macOS roots:
Common MediaCore plus `/Applications/Adobe After Effects [version]/Plug-ins/`.
Only the documented root itself may be resolved when it is a symlink; inner
symlinks remain refused.

## Target-Mac acceptance reports — opaque project guard

The two later Stage-7 reports `EGFX-AE-0968a7a315644ba3b87fa9519d59a7db` and
`EGFX-AE-1b9cdc075220463e8319610a14f84ec4` stopped before identity sampling and
pixel capture. Their `91` is the JSX arm script's deliberate guard exit, not an
AppleScript transport failure. The old phase record did not write AE version or
the individual guard predicate until after the combined guard, so it cannot
prove whether the project was saved/occupied/dirty or whether the undocumented
`Project.dirty` attribute was absent.

The follow-up acceptance tooling preserves all saved/occupied/dirty refusals,
records only opaque guard/revision tokens plus AE version before mutation, and
does not relabel an AE script exit as a transport fault. For an absent `dirty`
attribute only, it accepts a separately readable fresh `Project.revision == 1`
alongside an empty unsaved project; getter errors, malformed values and any other
revision remain blocking. These historical reports remain `BLOCKED`; a new
same-PID identity plus ten-frame target-AE result is still required to close
Stage 7.

## Latest target-AE run — occupied project

The revised acceptance runner from `fa7be425ce78083519f4a812a4f3689fe151df90`
was executed on the target AE 25.6x101. Report
`EGFX-AE-065fc56488584561ac00fd7ddd4936cb` (SHA-256
`fb9c9967af76f1b2e6bbcebeae6a18b8699640d50653b639f21922d1fa91fa0f`)
reached the JSX arm phase and returned `guard: OCCUPIED`, revision `32` before
any test-owned object was created. The script exit `91` is correctly reported as
that guard refusal, not as an AppleScript transport failure.

No live image was sampled and no pixel frames were captured; both statuses remain
`NOT RUN`, functional status remains `BLOCKED`, and this report is not renderer
evidence. The only condition for a safe retry is a newly created empty unsaved
AE project. The runner will continue to refuse rather than close or alter an
occupied user project.

## Target-AE acceptance automation

A single non-installing target runner is implemented for fd69988. The first
user-side attempt exposed a runner-only manifest type bug before any AE test ran:
a decoded dict was passed to a path-based verifier. That failure is preserved as
NOT RUN for AE functionality. The runner now keeps both the manifest path and
decoded data distinct, with regression tests covering both verifier and smoke
handoffs. It first requires live-image identity PASS for the exact installed candidate, then runs
the guarded ten-frame patterned AE smoke in the same AE PID. Functional PASS
requires all direct deformation/animation/reset checks plus Adjustment Layer ->
ElasticGrid -> identity/moved Corner Pin pixel assertions. Its shared ZIP includes
only sanitized summary and synthetic frames; raw process sample remains private.
Release status stays BLOCKED even when this functional gate passes.

## Stage 8 preparation while Stage 7 remains open

Performance measurement tooling is prepared and source-verified, but no target
performance measurement has run and Stage 8 is NOT started for acceptance.

- `e51703b`: controlled aerender benchmark + report comparison.
- `1a177da`: synthetic 1080p/4K, 8/16/32-bpc static/animated AE fixture generator.
- `c432da6`: fixture ownership test made an explicit Final Validation gate.
- The new gate found a mock-only null-project substitution bug; `67f9e8d`
  fixes the test harness without changing production fixture code.
- Exact-head Final Validation and all five PR CI jobs at `67f9e8d` PASS.

The benchmark requires Final Bicubic, a pinned synthetic AEP hash, unique output
directories, runtime Build ID from aerender, warmups plus at least five measured
samples, wall/user/sys/peak-RSS evidence and matching fixture/MFR settings for
comparison. Timing results never imply quality equivalence.

Actual synthetic AE fixture generation, aerender measurements and RAM Preview
profiling remain NOT RUN until Stage 7 target-AE functional acceptance passes.

The Stage 7 plugin candidate remains immutable `fd69988` /
`EGFX-f442513cb6528f14295d6d45`. Later docs/test-tooling commits change source
identity and may produce different CI Build IDs if rebuilt; those builds are not
substitutes for the pinned Stage 7 artifact.

## Next gates, in order

1. SmartFX source now wires the tested sparse CPU renderer: logical canvas is
   snapshotted during pre-render, max bounds are fixed to that canvas, returned
   compact-world origins are preserved, and a None adjustment input is rendered
   as transparent instead of leaving output untouched. The ordinary Render path
   remains dense; Metal remains disabled. This is SOURCE-VERIFIED only until a
   new exact candidate reproduces raster/text/Adjustment Layer chains in AE.
2. The automated AE smoke now also creates an Adjustment Layer and captures
   ElasticGrid before Corner Pin, immediately after identity Corner Pin, and
   after moved corners. Pixel validation rejects a black/flat frame, requires
   identity Corner Pin to preserve pixels, and requires moved corners to change
   them. This is prepared automation; actual AE execution still requires the new
   candidate. The custom-control label also uses a taller readable short Build ID.
   Verify loaded identity, drag/Undo/Redo,
   save/reopen/restart, actual 8/16/32-bpc alpha/HDR/ROI/PAR/MFR/cancel/aerender.
3. Profile equal-quality Render/RAM Preview, then optimize proven costs. AE GPU
   dispatch stays disabled pending real host/Metal correctness and lifecycle QA.
4. Prototype and test the approved 2D plane, then 3D/camera integration, preserving
   saved-project compatibility and interaction/render agreement.

The live-diagnostic path contradiction and incomplete app-plugin scan remain
open. Physical AE/Metal and target profiling are unavailable in this execution
environment. No new installable candidate is delivered by this checkpoint.
Older dated reports remain evidence only for their original source and scope.
Authoritative process: ../DEVELOPMENT_RULES.md.
