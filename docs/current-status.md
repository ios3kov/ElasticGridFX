# Current development / release status

2026-09-29. Branch: fix/final-validation, draft PR #5. **NOT READY FOR RELEASE.**

**Production cycle: Stage 7 of 10 — target After Effects functional acceptance.**
See development-stages.md. Stages 1–6 are completed for the current scope; Stage 7
is blocked only on the corrected real target-Mac acceptance run. Stage 8 is
performance/profiling, Stage 9 is the approved 2D/3D perspective plane, Stage 10
is final compatibility/release validation.
No main merge, new user installation or production release in this stage.

## User-machine evidence now received

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
