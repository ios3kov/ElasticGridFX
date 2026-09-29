# Current development / release status

2026-09-29. Branch: fix/final-validation, draft PR #5. **NOT READY FOR RELEASE.**

**Production cycle: Stage 8 of 10 IN PROGRESS; Stage 7 COMPLETE for the defined target functional smoke.**
See development-stages.md. Stages 1–7 are completed for the current scope. Stage 8 is
performance/profiling, Stage 9 is the approved 2D/3D perspective plane, Stage 10
is final compatibility/release validation.
No main merge, new user installation or production release in this stage.

## User-machine evidence now received

### Stage 8 preparation — measurements BLOCKED

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
