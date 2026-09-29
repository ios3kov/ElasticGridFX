# Current development / release status

2026-09-28. Branch: fix/final-validation, draft PR #5. **NOT READY FOR RELEASE.**
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
