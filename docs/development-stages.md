# ElasticGridFX production stages

Authoritative process: ../DEVELOPMENT_RULES.md.

This is the project-wide stage map used for the required “Stage X of Y” status.
A stage is complete only when its own acceptance criteria and evidence are complete.
Percentages are descriptive progress only; failed/blocked checks never count as PASS.

## Stage 1 of 10 — Research and parity specification — COMPLETE

- Original GridWarp behavior researched with user authorization.
- Product scope, saved-project compatibility and parameter semantics documented.
- Adobe SDK / host constraints and reference implementations reviewed.

Exit evidence: parity/spec docs and audit records committed.

## Stage 2 of 10 — Core deformation engine — COMPLETE

- Elastic guide model, inverse warp, Bilinear/Final Catmull-Rom Bicubic.
- 8/16/32-bpc CPU paths, deterministic identity, edge modes, animation.
- Portable correctness, fuzz/soak and sanitizer coverage.

Exit evidence: core regression suite PASS for frozen candidate lineage.

## Stage 3 of 10 — AE native host and project compatibility foundation — COMPLETE

- Rust AE host, arbitrary grid state, parameter IDs/wire schema.
- SmartFX snapshotting, MFR-safe ownership/lifecycle, Build Identity.
- Existing project wire compatibility retained.

Exit evidence: host/source contracts and build identity gates PASS.

## Stage 4 of 10 — Hardening, diagnostics and reversible installation — COMPLETE

- Security/resource/ownership audit.
- Clean package identity, signature inspection, diagnostics.
- Scoped atomic updater and rollback with retained original.

Exit evidence: update/rollback macOS safety gates PASS.

## Stage 5 of 10 — Target installation and runtime identity infrastructure — COMPLETE

- Authorized test installation path established on target Mac.
- Installed payload / backup reporting and live-image UUID mapping tooling.
- Process-guard false positive fixed.

Exit evidence: target installation workflow and diagnostic tooling verified.
This stage does not mean the current fd69988 runtime has passed functional QA.

## Stage 6 of 10 — SmartFX sparse/bounds compatibility fix — COMPLETE IN SOURCE

- Reproduced compact-world edge-stretch defect in portable fixture.
- Added transparent sparse logical-canvas renderer.
- Wired SmartRender to full logical-canvas semantics, correct origins and empty input.
- Added Adjustment Layer -> ElasticGrid -> Corner Pin deterministic regression.
- Grid Positions Build ID label made readable.
- Immutable test candidate: fd69988 / EGFX-f442513cb6528f14295d6d45.

Exit evidence: fd69988 exact source/CI/macOS gates PASS.
Real host confirmation belongs to Stage 7.

## Stage 7 of 10 — Target After Effects functional acceptance — COMPLETE

Goal: prove the exact loaded fd69988 fixes the reported visual/chain defects in
After Effects 2025 on the target Mac.

Required PASS in one controlled run:
- complete installed fd69988 payload/signature;
- actual loaded-image identity and same AE PID;
- bypass/identity/reset pixel integrity;
- static and animated deformation;
- no bright streak/black/flat output;
- Adjustment Layer -> ElasticGrid -> identity Corner Pin preserves pixels;
- moved Corner Pin changes pixels;
- test-owned cleanup succeeds.

Exit evidence: target run EGFX-AE-dbcc0ec13f7e48b387f55aa749c623a6,
runner 7371408, immutable plugin fd69988, same AE PID 42039, identity and
ten-frame pixel smoke PASS. See current-status.md for hashes and scope.
Earlier blocked reports remain historical and are not rewritten as PASS.

## Stage 8 of 10 — Render / RAM Preview profiling and optimization — RESUMED

2026-10-01: user explicitly resumed acceleration without quality loss.
Current work/evidence: [performance-resume-2026-10-01.md](performance-resume-2026-10-01.md).
Native bounded optimization and eight CI source gates PASS at 41283e3.
Target AE speed acceptance remains IN PROGRESS. The user requested uninterrupted
development with stage status reports, then continued work.

Current scoped blocks (2026-10-02 unlocked host):
- 8.1 Native exact sampler optimization: PASS, including 16 same-input pattern comparisons.
- 8.2 Measurement safety/precision: Python 248/248 and all 12 JSX suites PASS;
  actual two-host-turn binding-ready schema-3 preparation and straight RGBA16 PASS.
- 8.2a Optional callback attribution: all 60 plane_region frames verified for
  same-toolchain original native source and optimized observer; loaded identity PASS.
  Sampling phase medians 348.734/33.595 ms are instrumented diagnostics only.
- 8.2b Source/build isolation: same-current-toolchain 40-frame GUI matrix and
  60-frame plane sequence are decoded RGBA16 exact. Older artifact FAIL retained.
- 8.3 Target plane/3D/AEP matrix: 40 frames functional PASS on all three builds;
  separate legacy Adjustment capture harness remains open (also missing on original).
- 8.4 Target Render: IN PROGRESS; ordinary quality pilot and uniquely versioned
  uninstrumented original-source control precede five matched timing pairs.
- 8.5 RAM Preview cache-build/playback/invalidation: NOT RUN for this candidate.
- 8.6 UI response/cancel lifecycle and final scoped candidate validation: pending.

All eight CI checks at b492e60 PASS. Existing immutable f611312 ordinary/observer
artifacts retain physical-Mac/package evidence. [Actual matrix and plane records](performance-host-matrix-2026-10-02.json).
Each bounded temporary transaction restores and verifies the original plugin.
No main merge or release is implied.

2026-09-29: user explicitly requested skipping performance acceleration.
Retain collected measurements as historical evidence; incomplete RAM Preview,
profiling and optimization requirements are not PASS. No acceleration claim.
This skips the dedicated optimization stage, not pixel-quality/stability checks.

Research/preparation may run in parallel with Stage 7, but no performance change
is approved before the Stage 7 pixel baseline is trustworthy.

Required:
- equal-quality Final Bicubic baseline;
- Render Queue/aerender cold/warm timing, p50/p95 and peak memory;
- RAM Preview build/invalidation timing on the target host;
- drag-to-fresh-frame latency;
- MFR/threading/allocation/ROI/copy/CPU↔GPU attribution;
- Before -> Change -> After evidence for every significant optimization;
- no hidden resolution/bit-depth/filter/precision downgrade.

## Stage 9 of 10 — 2D/3D perspective deformation plane — ACCEPTED IN RELEASE SCOPE

Current: 0.9.1 includes the accepted native-text plane, 3D mode lock, panel
refresh and perimeter corrections. See release-0.9.1.md and current-status.md
for exact artifact identity and evidence. Four Corners is a bounded 2D
deformation region, disabled in 3D. This is not broad platform/PAR certification.

### Historical regression and acceptance checkpoints

The following REOPENED state and candidate identities describe earlier builds,
not the installed/published 0.9.1 candidate.

2026-09-30: user evidence shows displaced overlay when enabling native 3D on a
text layer. Earlier solid-only UI acceptance is insufficient. See current-status.md.
The acceptance statements below are historical, superseded by this regression.

2026-09-30: user replied “работает” to the combined manual drag/Undo/Redo and
legacy animated-project acceptance request. Recorded as USER-REPORTED PASS,
not instrumented automation. Current installed candidate is 099e492 /
EGFX-97a79761c3f9f35751e06853. Controlled square-pixel AE 2025 tests: 40 frames,
29 checks PASS, exact loaded identity PASS, cleanup CLEAN. Native visual scene
confirms layer/parent/camera alignment, mode-dependent targets and panel order;
old static AEP load/save/reopen PASS. See current-status.md for run identities.
Non-square 3D UI and broad color/platform compatibility are not implied.

### Historical Stage 9 development record

Current installed test candidate: de31498 / EGFX-0fa68430a170b3612e8d00f7.
Four-corner native controls and render/overlay/drag integration are implemented;
portable/native ABI checks and installed payload verification pass. Stage 9 target
AE tooling is source-validated and pins the exact candidate commit, Build ID and
ZIP SHA-256. The executable Stage 9 launcher is non-installing and fail-closed.
It automates loaded identity, 34 pixel frames across 8/16/32 bpc, AEP save/reopen
and 3D position,
scale, rotation, parenting, camera movement/switch and no-camera fallback, followed
by safe cleanup. The first real target-AE attempt was BLOCKED before the first
numeric frame because the fixture checked stale ExtendScript File metadata directly
after `saveFrameToPng`; identity/pixels remained NOT RUN, so this was not a plugin
FAIL. Tooling fix 724001c adds the same bounded file-publication wait used by the
proven Stage 7 smoke; Final Validation, PR CI and macOS source gate PASS.
The second real run then captured 33/34 frames and blocked only on the fixture's
post-disable no-camera assertion; identity/pixel aggregation remained NOT RUN.
Fix e6dbc26 captures Default Camera before creating camera layers and passes
Final Validation, PR CI and macOS source gate. The third real run then showed that
target AE 25.6 exposes a non-null `activeCamera` even though the owned comp has no
camera layer, contrary to the scripting-guide contract; it blocked after the 24
depth frames plus two roundtrip frames, before identity/pixel aggregation. Fix
b327964 proves the no-camera case by owned layer topology instead of
`activeCamera`, and passes Final Validation, PR CI and macOS source gate. The next
34-frame run exposed host saveFrameToPng darkening at 32 bpc, independently
reproduced without effects/import. After switching the fixture to guarded Render
Queue PNG capture, run EGFX-PLANE-531656cd2c5243818311ac9b1dcb152e passed all
34 frames / 26 comparisons, exact loaded identity and CLEAN cleanup. Native viewer dragging and UI Undo/Redo remain
separate real-host interaction gates. See current-status.md for evidence.
Installation/tooling PASS is not stage completion.

- Four-corner projective plane with transformed grid/handles/hit-test/render.
- 3D layer position/scale/rotation/parenting and active-camera perspective.
- Defined singular/edge-on/outside-plane behavior.
- Existing nonprojected path stays an explicit fast path.
- Saved projects remain compatible.

Research may proceed in parallel; implementation begins only after the current
functional bugfix is accepted and its performance baseline is recorded.

## Stage 10 of 10 — COMPLETE for agreed 0.9.3 development scope

Current accepted development candidate: `2ccc5f674b8b94b534d4c3ad6abdec3524e3624f`,
Build `EGFX-bd19dee13315abc0b7e6090e`, FSTR Stretch 0.9.3, macOS Apple
Silicon / AE 25.6. Automated source/package gates PASS; target-host acceptance is
USER-REPORTED PASS for the requested functional, Layer Plane, RAM Preview and
cancellation/re-render scenarios. #7, #8, #9, #13 and #14 are closed.

The earlier v0.9.1 release remains historical. The accepted 0.9.3 source has now
been merged through PR #10 → #12 → #15; PR #16 merged documentation only.
Version 0.9.3 is **not published**. See stage10-final-acceptance-2026-10-01.md,
stage10-final-gate-2026-09-30.md and retrospective-0.9.3.md.

Under the current public macOS distribution rule, publication remains BLOCKED
until the exact public artifact is Developer ID signed, notarized/stapled where
applicable, and verified through a quarantined Gatekeeper-clean download/install.

### Historical pre-release checkpoint

Blocked by reopened Stage 9 text-layer 3D overlay regression. Existing evidence
below remains historical and does not waive the new failure.

2026-09-30: final regression started against the unchanged installed 099e492.
See stage10-final-gate-2026-09-30.md for scoped evidence and outstanding gates.

Required as applicable:
- clean install/update/rollback/restart;
- 8/16/32 bpc, alpha/HDR/negative float, PAR/downsample/ROI;
- Render Queue, aerender, RAM Preview, MFR, cancellation;
- drag/Undo/Redo/save/reopen/restart;
- target macOS/AE matrix and physical GPU evidence;
- final audit, profiling, documentation, artifact identity and clean Git state.

Only Stage 10 completion can mark the product release-ready. No merge to main or
production release occurs without explicit user instruction.

## Current overall progress

Stages 1–7 retain their recorded evidence; Stage 9 is accepted within the
documented scope. Stage 8 was skipped at acceptance and is now resumed (not PASS).
Stage 10 is COMPLETE for the agreed 0.9.3 macOS Apple Silicon / AE 25.6
development scope. This does not certify Windows/Intel, other AE versions,
broad HDR/OCIO, physical GPU execution, or public signing/notarization.
No percentage or performance-completion claim. Version 0.9.3 is merged but
unpublished; the public macOS distribution gate remains BLOCKED on
Developer ID/notarization/Gatekeeper-clean delivery.
