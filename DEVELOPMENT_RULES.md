# Development rules

Canonical standard, explicitly selected by the user on 2026-10-01:
https://github.com/ios3kov/AE-Development-Rules

Start with [AI_ENTRYPOINT.md](https://github.com/ios3kov/AE-Development-Rules/blob/main/AI_ENTRYPOINT.md), then select the rules applicable to the actual task. This file is a project pointer and additions, not a modified copy of the standard.

Baseline consciously updated on 2026-10-02: **5.1.0**, immutable tag `v5.1.0`
peeled commit `54fa9966fd4eab10f35f1fbc8aa18f94ff42925b` (tag object
`91737f9a05d52b62dfb8de33bad3f9fab5d4cd08`). AI_ENTRYPOINT was read first;
changes from b27f454 and versioned errata were reviewed. The current map adds
explicit state/automation/completion, feature overlays and progress-based stop
criteria. Existing controls already cover this task; this checkpoint updates
routing/documentation, with no identity-engine or artifact-format migration.
Prior 5.0.0 candidate / b27f454 and 4.1.0 / 05bd9a8 remain historical provenance.

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
