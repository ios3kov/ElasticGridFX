# Development rules

Canonical standard, explicitly selected by the user on 2026-10-01:
https://github.com/ios3kov/AE-Development-Rules

Start with [AI_ENTRYPOINT.md](https://github.com/ios3kov/AE-Development-Rules/blob/main/AI_ENTRYPOINT.md), then select the rules applicable to the actual task. This file is a project pointer and additions, not a modified copy of the standard.

Baseline consciously updated on 2026-10-02: **6.0.0**, immutable tag `v6.0.0`,
peeled commit `bb8b769404ddd5b97462812a4e6b430e8bfefe13`.
AI_ENTRYPOINT was read first; the 5.1.0→6.0.0 diff, Release §§26/28 and
Engineering §34 migration instructions were reviewed. macOS distribution no
longer requires Developer ID, notarization or Apple distribution services;
absence is not a release blocker. Exact artifact identity, actual documented
install/AE loading, runtime checks and truthful Evidence remain required.
The product does not invoke the changed starter-kit distribution wrappers;
no caller/interface migration is necessary. Native product tools stay unchanged.
Prior 5.1.0 / 54fa9966fd4eab10f35f1fbc8aa18f94ff42925b and older Evidence
remain historical provenance. No historical test verdicts are rewritten.

## Current continuation: quality-preserving performance

The user resumed Render / RAM Preview acceleration on 2026-10-01. The earlier
Stage 8 skip remains historical; it does not cancel the resumed task.

- Artifact: native AE effect, Rust host + C++ CPU renderer; macOS Apple Silicon,
  AE 2025 / 25.6 accepted scope.
- Risk: **Critical** for renderer/performance changes. Delivery: **Development**;
  validation and release are separate gates.
- Applicable: Process Core §§1–13; Engineering §§14–21, §24, §§31–34, §38; Native §23;
  Workflow. Read the Validation/Release profile before corresponding handoff.
- Product scope is already defined. No new discovery or external-reference audit
  is triggered by continuing performance work on this existing renderer.
- Preserve [the quality contract](docs/performance-quality-contract.md): Final
  Catmull-Rom Bicubic, resolution, precision, 8/16/32-bpc, alpha/HDR semantics,
  identity pixels, plane mapping and saved parameter/wire compatibility.
- Measure Before → Change → After on matching fixtures; isolate native sampling
  from encoding/startup and RAM Preview playback. Never transfer old host PASS
  to a new binary or claim AE acceleration from a standalone benchmark.
- Keep projects, installed plugins, rollback backups and historical evidence.
  Installation/retention is separately explicitly authorized here; Development
  alone does not authorize installation, cache purge, merge or release.

The user requested a fresh rules read during this block. AI_ENTRYPOINT was read
first; the prior 4.1.0 / 05bd9a8 adoption remains provenance for initial evidence.
Apply AI-STATE-001/AI-AUTO-001 checkpoints and API-SOURCE-001 before any new or
changed Adobe API call. No Adobe API call changes in the current native sampler.

Read [current status](docs/current-status.md) and
[the resumed performance record](docs/performance-resume-2026-10-01.md).
Required FAIL, BLOCKED or NOT RUN is not PASS.

The prior FSTR-Line pointer and 2026-09-29 blob
`701a8c1ae3acb4dbfe1d7eda94acbf8095b88608` remain historical provenance.
New work uses the central standard selected above.

2026-10-02 direct human decisions: speed tests are accepted; stop more timing
series and retain the unchanged accelerated f611312 candidate installed with its
original backup. Record USER_ACCEPTED separately from Test Status; absent exact
latency/Full control measurements remain NOT RUN. MFR failure/stability is a
separate open risk, not waived by speed acceptance. No quality compromise,
main merge or public release is authorized.
[Decision and verified installation](docs/performance-user-acceptance-2026-10-02.json).

## Later release decisions — 2026-10-02

The earlier no-merge/no-release statement above is historical. Subsequent direct
human instructions authorized merges and publication without Developer ID or
notarization. PR22 and PR23 merged after exact-head CI passed. The human now
reports both remaining distribution/legacy-project checks passed; evidence is
USER-REPORTED. Promote the existing v0.9.3-perf.1 GitHub release without rebuilding
or changing assets. Full standards certification is not claimed; omitted signing
checks remain omitted, and the historical MFR abort cause remains unknown.

## Current documentation closeout — 2026-10-02

Light documentation scope under frozen6.0.0; Process/Core, Engineering §§24/25
and Workflow apply. The user authorized README, user-guide and final-cycle
retrospective reconciliation. Reusable findings are contributed to the central
AE knowledge base separately; this does not change this project's baseline.
No new discovery/reference audit, API, build, installation or host timing gate.
The earlier performance/release decisions above retain their historical scope.

## Next update baseline — 2026-10-03

The human explicitly requested this update under **AE-Development-Rules8.0.0**,
immutable tag v8.0.0 / `132b7cd32873ba7328e3128ffbb33e1929b74d45`.
The official ZIP SHA256 was verified, AI_ENTRYPOINT read first, and migration
from the project6.0.0 and Windows6.2.0 baselines reviewed in CHANGELOG/§34.
Historical release/Windows results retain their original standards and identity.
New mandatory overlays: FEATURE-SET-001, TASK-CLOSE-001 and CLEANUP-001.
The project does not invoke starter-kit distribution wrappers; no wrapper caller
migration is needed. Update the existing plan/status rather than copying rules.

Scope: native effect/UI/animation/state plus guarded installation helpers,
macOS Apple Silicon and Windows x64 CPU; Critical / Development. Read Process
§§1–13/27, Engineering §§14–21/24/25/31–34/36–39/41, Native§23, Workflow and
applicable source/parameter supervision before affected API changes; apply
Validation/Release/platform gates only for that actual delivery phase.
No GPU, broader-version certification, main merge, public release or destructive
installation is authorized merely by the Development scope. Existing permissions
and direct user decisions remain separately scoped.

See [feature/update plan](docs/feature-backlog.md) for the requirement/task/check
mapping and [current checkpoint](docs/STATUS.md).
