# ElasticGridFX production stages

Latest MFR disposition: issue21 CLOSED as NOT REPRODUCED by explicit user
decision after six complete60-frame ordinary renders (three original control,
three retained candidate). Cause unknown, no fix/general certification. Earlier
open-status checkpoints below are historical. [Record](mfr-closure-retry-2026-10-02.json).

Postrelease check2026-10-02: one owned user-scene copy PASS for selected-frame
render parity, UI Undo, save/reopen and functional GUI Preview. Source preserved;
full release/MFR gates remain open. [Record](postrelease-project-validation-2026-10-02.json).

2026-10-02: experimental TEST prerelease v0.9.3-perf.1 PUBLISHED by explicit
user exception without Developer ID/notarization. Public asset hash matches.
Full Release Gate NOT PASSED; MFR #21 stays open.
[Publication](prerelease-publication-0.9.3-perf.1.json).

Release preparation2026-10-02: local validation package0.9.3-perf.1 ready
with unchanged tested0.9.3 Develop Build2. Public release BLOCKED on
Developer ID/notarization/clean distribution and incomplete final gates;
MFR issue #21 remains open. See [package notes](release-0.9.3-perf.1.md).

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

## Stage 8 of 10 — speed scope USER_ACCEPTED; reliability followup OPEN

2026-10-01: user explicitly resumed acceleration without quality loss.
Current work/evidence: [performance-resume-2026-10-01.md](performance-resume-2026-10-01.md).
Native bounded optimization and eight CI source gates PASS at 41283e3.
2026-10-02 direct human speed acceptance closes further timing. Missing technical
results retain their actual status; quality/stability and release are separate. The user requested uninterrupted
development with stage status reports, then continued work.

Current scoped blocks (2026-10-02 unlocked host):
- 8.1 Native exact sampler optimization: PASS, including exceptional floats and same-input checks.
- 8.2 Measurement safety/precision: Python 248/248 at c62bb74 and 15 JSX suites PASS;
  actual two-turn binding-ready schema-3 preparation and straight RGBA16 PASS.
- 8.2a Optional callback attribution: both observers cover all 60 plane_region frames;
  loaded identity PASS. Sampling medians 348.734/33.595 ms are diagnostics only.
- 8.2b Source/build isolation: same-toolchain 40-frame matrix, 60-frame sequence,
  ten queue-chain states and 360 fresh-pair PNGs exact. Older-artifact FAIL retained.
- 8.3 Target plane/3D/AEP matrix and queue Adjustment/Corner Pin chain: scoped PASS.
  Historical saveFrameToPng failure remains a separate retained harness limitation.
- 8.4 Target Render total time: five MFR-requested-OFF ordinary fresh-phase pairs PASS;
  65.870721 -> 48.766815 s median, 25.97% less time. Per-frame/cold-cache timing open.
- 8.4a Ordinary MFR-requested-ON series INCOMPLETE: three measured pairs plus warmup;
  fourth control crashes with19/60 frames, correctly rejected. Complete240 and
  partial19 decoded frames exact. Separate instrumented60-frame ON pair exact;
  ordinary throughput/root cause remain open.
- 8.5 RAM Preview: full-cache60-frame/30-fps playback, cached restart and wave
  invalidation functional PASS. Ten equal-zoom fresh-phase UI intervals recorded;
  median bounds (11.546,12.924] -> (1.753,6.007]s, four improved/one inconclusive
  pair. Capture overhead/order/cache limits retained; observer-free cache-build,
  first-playable timing remain technically INCOMPLETE, with further timing
  closed by human acceptance. Later Full/Skip0 readback is covered in8.7.
- 8.6 Escape cancellation and actual guide edit/fresh image/Undo PASS. Exact
  guide latency was not measured; further timing closed by human acceptance.
- 8.4b Stability-only candidate/control × enabled/bypass isolation: four60/60
  scoped PASS;120 candidate RGBA16 decoded and120 control pairs encoded-exact.
  Prior crash not reproduced; cause/fix unresolved. Planned experiment complete;
  new distinguishing signal required before more retries. Candidate retained.
  [Isolation](mfr-stability-isolation-2026-10-02.json).
- 8.7 Full/Skip0 preset readback PASS; five candidate Full runs60/60 at30fps.
  New Full control timings NOT RUN because the human accepted speed scope.
  [Decision and retained candidate](performance-user-acceptance-2026-10-02.json).

All eight CI checks at c62bb7422547bfd911fb27be0375a52d55bccb7e PASS; CI-hardening
checkpoint c4efa51f0c0f2d5bd8e2900c83ca8169851503af also eight PASS. Later edits
require their own exact-head checks. Immutable f611312 candidate and same-toolchain
uninstrumented control retain full physical-Mac/package evidence.
[Matrix/chain](performance-host-matrix-2026-10-02.json),
[Render samples](performance-host-render-2026-10-02.json),
[Preview lifecycle](performance-host-preview-2026-10-02.json).
[Preview interval series](performance-host-preview-intervals-2026-10-02.json),
[MFR retained failure/diagnostics](performance-host-mfr-2026-10-02.json).
Earlier bounded transactions restored/verified the original. The final ordinary
candidate is intentionally retained installed by the later human decision, with
original backup independently verified.
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
documented scope. Stage 8 was historically skipped and resumed; speed scope is now USER_ACCEPTED,
while the independent ordinary MFR incident remains unresolved (not fixed).
Stage 10 is COMPLETE for the agreed 0.9.3 macOS Apple Silicon / AE 25.6
development scope. This does not certify Windows/Intel, other AE versions,
broad HDR/OCIO, physical GPU execution, or public signing/notarization.
No percentage or performance-completion claim. Version 0.9.3 is merged but
unpublished; the public macOS distribution gate remains BLOCKED on
Developer ID/notarization/Gatekeeper-clean delivery.
