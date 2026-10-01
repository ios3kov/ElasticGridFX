# FSTR Stretch 0.9.4 — Stage 1/10 performance baseline and test contract

Date: 2026-10-01  
Process mode: **Release / Critical**  
Status: **IN PROGRESS — tooling/source verification first; real target-AE measurements BLOCKED in this development session**

Authoritative rules: `ios3kov/AE-Development-Rules` commit
`320f80902fe5f50041935712e8d144ac61d29687`, rules blob
`740f752dd9bd437391e141aea333646798bd547e`.

## 1. Goal

Create a trustworthy **Before** baseline for FSTR Stretch 0.9.4 before changing
render, SmartFX/MFR, cache or GPU production behavior.

No production renderer optimization is allowed to start from theory alone.
Subsequent performance work must use **Before → Change → After** evidence under
comparable conditions.

## 2. Frozen baseline artifact

The baseline runtime is the exact accepted 0.9.3 candidate:

- source commit: `2ccc5f674b8b94b534d4c3ad6abdec3524e3624f`;
- Build ID: `EGFX-bd19dee13315abc0b7e6090e`;
- version: 0.9.3 development;
- target: macOS Apple Silicon / After Effects 25.6;
- source SHA-256: `efdd146d3324f650e91ad435b26977898a3574f78a866a66f95bec6e47eff9a4`;
- delivery ZIP SHA-256: `1448a5231fc561e6b10491d1b9d1839de22f21a11266a43e2990244a25176ebc`;
- binary SHA-256: `7dc57622442eaddcc1f061e044e77db3796a2db44e4061c807eee6aa60c15beb`.

Historical 0.9.3 functional acceptance is not a 0.9.4 performance measurement.

## 3. Scope and risks

### Target scope for this stage

- After Effects 2025 25.6;
- macOS Apple Silicon;
- square-pixel synthetic fixtures;
- Final Catmull-Rom Bicubic as the primary quality path;
- Preview Bilinear measured separately, never substituted for Final;
- 8 / 16 / 32 bpc;
- MFR ON / OFF where aerender supports explicit control;
- ordinary grid/Layer Plane and Four Corners in the first automated fixture set;
- native 3D/RAM Preview require separate real-host coverage before Stage 1 closes.

### Primary risks

- measuring a stale or different plugin binary;
- timing AE/encoding/I/O and attributing it incorrectly to FSTR Stretch;
- cache-state mismatch between runs;
- output changes hidden by a speed result;
- MFR oversubscription;
- stale fixture or old PASS reuse;
- unsafe mutation of a user project;
- using PNG timing output as a 32f/HDR quality oracle;
- declaring GPU/RAM Preview performance from portable tests.

## 4. Acceptance criteria fixed before implementation

Stage 1 is complete only when:

1. the exact 0.9.3 Build ID is observed in the measured runtime;
2. synthetic fixture ownership and project hash are pinned;
3. each automated timing condition has explicit warmup plus at least five measured runs;
4. p50/median, p95/spread, peak RSS and per-frame timing are recorded;
5. repeated runs of one candidate produce an identical encoded output digest;
6. baseline/candidate comparisons refuse or FAIL when encoded outputs differ;
7. MFR ON and OFF are recorded separately;
8. at least 1080p and 4K 32-bpc Final are measured for ordinary grid static/animated;
9. 8/16/32-bpc coverage exists in the agreed core matrix;
10. Four Corners has a measured baseline;
11. RAM Preview baseline records cold-build/replay/invalidation behavior separately from aerender;
12. native 3D projective baseline is measured in real AE;
13. output quality claims remain bounded: PNG/encoded equality is not presented as HDR/32f proof;
14. deep profiling identifies the material bottleneck before a high-complexity optimization is accepted.

Until items 11–14 are available, Stage 1 remains **BLOCKED / IN PROGRESS**, even if
portable tooling and aerender timing support are complete.

## 5. Test cases

### PERF094-BASE-001 — baseline identity and environment

- Type: runtime AE / performance.
- Artifact: exact 0.9.3 candidate above.
- Preconditions: controlled target Mac; explicit AE 25.6 executable; no ambiguous plugin copy.
- Expected: package/installed payload verify; runtime Build ID equals
  `EGFX-bd19dee13315abc0b7e6090e`; environment metadata recorded.
- Tolerance: exact identity only.
- Current status: **BLOCKED** — this development session has no target AE runtime.
- Evidence when run: benchmark report + runtime logs + environment record.
- Cleanup: test-owned workspace only.

### PERF094-FIX-001 — ordinary grid synthetic fixture

- Type: integration/runtime AE.
- Fixture: synthetic pattern, Final Bicubic, controlled working-space state.
- Matrix: 1080p/4K; 8/16/32 bpc; static/animated.
- Expected: fresh owned AEP + fixture manifest, no user project touched.
- Current status: **NOT RUN** in real AE; portable runner contracts are covered by CI.
- Evidence: fixture.json, project SHA-256, capture.json.

### PERF094-FIX-002 — Four Corners synthetic fixture

- Type: integration/runtime AE.
- Geometry: deterministic perspective quad on the same synthetic source.
- Expected: same ownership/safety guarantees as PERF094-FIX-001.
- Current status: **NOT RUN** in real AE; fixture support is introduced in Stage 1 tooling.
- Evidence: fixture/capture manifest.

### PERF094-FIX-003 — 8K preparation capability

- Type: integration/runtime AE.
- Scope: 7680×4320 fixture preparation for selected runs where target memory permits.
- Expected: no resize/crop; no claim that every baseline matrix cell must run at 8K.
- Current status: **NOT RUN** in real AE.

### PERF094-RQ-001 — 1080p 32-bpc Final ordinary grid

- Type: performance / aerender.
- Conditions: static and animated; MFR ON and OFF separately.
- Samples: 2 warmups + minimum 5 measured runs per condition.
- Expected: stable encoded output digest; median/p95/peak RSS recorded.
- Current status: **BLOCKED** — requires target AE/aerender.

### PERF094-RQ-002 — 4K 32-bpc Final ordinary grid

- Type: performance / aerender.
- Conditions/criteria: same as PERF094-RQ-001.
- Current status: **BLOCKED**.

### PERF094-RQ-003 — core bit-depth matrix

- Type: performance / aerender.
- Matrix: 8/16/32 bpc with identical geometry/quality/MFR state per comparison.
- Expected: separate records; no cross-bit-depth timing comparison presented as identical work.
- Current status: **BLOCKED**.

### PERF094-RQ-004 — Four Corners projective baseline

- Type: performance / aerender.
- Primary condition: 4K 32-bpc Final, then supporting depths if practical.
- Expected: exact fixture identity and repeatable output digest.
- Current status: **BLOCKED**.

### PERF094-RAM-001 — RAM Preview cold/build baseline

- Type: runtime AE / performance.
- Requirement: fixed frame range; Final-quality viewer state; time to first playable
  frame and total build/cache time.
- Current status: **BLOCKED** — no stable direct scripting Preview API and no target
  AE runtime in this session. Do not substitute aerender.

### PERF094-RAM-002 — warm replay baseline

- Type: runtime AE / performance.
- Requirement: cached playback reported separately from build throughput; memory recorded.
- Current status: **BLOCKED**.

### PERF094-RAM-003 — invalidation/rebuild baseline

- Type: runtime AE / performance.
- Requirement: change one real render dependency, verify stale frames are not reused,
  measure rebuild; separately record UI-only density change behavior.
- Current status: **BLOCKED**.

### PERF094-3D-001 — native 3D projective baseline

- Type: runtime AE / performance.
- Requirement: owned 3D fixture with rotation and established automatic binding,
  exact loaded identity, Final render timing.
- Current status: **BLOCKED** — needs a real two-turn AE lifecycle fixture/run;
  a one-turn source/mock must not be called PASS.

### PERF094-QA-001 — repeated-output determinism for timing runs

- Type: integration/performance.
- Requirement: all measured runs of the same exact fixture/candidate/MFR state produce
  the same encoded output manifest digest.
- Expected: one unique digest.
- Current status: **NOT RUN** until target benchmark execution.
- Limitation: encoded PNG equality is not an HDR/32f oracle.

### PERF094-CMP-001 — baseline/candidate comparison guard

- Type: static/integration tooling.
- Requirement: refuse incomparable fixture/MFR geometry; mark comparison FAIL when
  encoded output digest differs; report timing delta without weakening quality gates.
- Current status: **NOT RUN** until CI for the Stage 1 tooling commit.

### PERF094-PROF-001 — real-host deep profile

- Type: performance/profiling.
- Requirement: attribute CPU/GPU/RAM, SmartFX checkout, plan preparation, sampling,
  MFR/threading and transfer/synchronization costs in the material 0.9.3 scenario.
- Current status: **BLOCKED** until target baseline exists.

## 6. Tooling policy

Existing performance tooling is retained and hardened instead of replaced:

- `tools/perf_fixture_runner.py` — creates only owned synthetic fixtures;
- `tests/perf_fixture.jsx` — AE-side fixture creation;
- `tools/aerender_benchmark.py` — exact-candidate timing and memory;
- `tools/perf_compare.py` — comparable Before/After reporting.

Stage 1 tooling additions:

- fixture schema v2 records geometry and controlled color-management state;
- schema v1 remains readable for historical records;
- 8K fixture preparation is supported, not mandatory for every run;
- deterministic Four Corners fixture is supported;
- every benchmark report gets Test Case ID + unique Test Run ID;
- repeated timing samples must have one stable encoded output digest;
- compare reports make encoded-output mismatch an explicit FAIL;
- timing comparison still does not become a broad HDR/quality certification.

## 7. What Stage 1 does not authorize

- no render-path optimization;
- no Metal enablement in production AE;
- no SmartFX ROI reduction;
- no MFR scheduling change;
- no cache invalidation behavior change;
- no version bump;
- no installation/update;
- no release.

Those changes begin only after the applicable baseline/profile gate exists.

## 8. Next step after tooling CI

1. Run the exact 0.9.3 baseline on the controlled target Mac/AE.
2. Preserve the raw reports and hashes.
3. Perform the real RAM Preview and native 3D baseline cases.
4. Run deep profiling on the slowest/material path.
5. Only then select the first production optimization from measured evidence.

If target runtime access is unavailable, Stage 1 remains **BLOCKED** and no
theoretical speedup is promoted to production.
