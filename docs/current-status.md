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

**The AE host does not call the new entry yet.** Opt-in requires verified origin,
reference-size and full-source checkout semantics; not every missing ROI is zero.
Do not describe this as an installed fix for streaks or Corner Pin.

Local source checks: baseline 10/10; GCC Release 11/11; Clang Release 11/11;
ASan/UBSan/LeakSanitizer 11/11; TSan sparse/MFR/determinism 3/3. Sparse/dense
steady-state allocation checks pass. Detailed reproduction, thresholds, raw-log
names and performance limits: sparse-render-results-2026-09-28.md.
Read exact resulting commit's Actions/PR checkpoint for CI and macOS outcomes;
queued jobs and hardware compile-only stages are never host/runtime PASS.

## Next gates, in order

1. Validate SmartFX full reference dimensions, world origins, legal result/max
   rectangles, empty/bounds-only requests and output initialization. Wire sparse
   CPU semantics only after the host contract is established. Reproduce
   raster/text/Adjustment Layer with Corner Pin before/after and Fast Blur.
2. Fix clipped custom-control text; verify loaded identity, drag/Undo/Redo,
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
