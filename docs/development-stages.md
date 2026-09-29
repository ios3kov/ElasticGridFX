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

## Stage 8 of 10 — Render / RAM Preview profiling and optimization — SKIPPED BY USER

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

## Stage 9 of 10 — 2D/3D perspective deformation plane — IN PROGRESS

- Four-corner projective plane with transformed grid/handles/hit-test/render.
- 3D layer position/scale/rotation/parenting and active-camera perspective.
- Defined singular/edge-on/outside-plane behavior.
- Existing nonprojected path stays an explicit fast path.
- Saved projects remain compatible.

Research may proceed in parallel; implementation begins only after the current
functional bugfix is accepted and its performance baseline is recorded.

## Stage 10 of 10 — Final compatibility / release gate — NOT STARTED

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

Stage-count view: 7 completed, Stage 8 skipped by user (not PASS), Stage 9 in
progress, Stage 10 open. No performance completion or release claim.
This is a stage-count view, not a claim about remaining engineering effort.
