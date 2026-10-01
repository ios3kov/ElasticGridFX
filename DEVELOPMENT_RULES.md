# Development rules

Canonical standard, explicitly selected by the user on 2026-10-01:
https://github.com/ios3kov/AE-Development-Rules

Start with [AI_ENTRYPOINT.md](https://github.com/ios3kov/AE-Development-Rules/blob/main/AI_ENTRYPOINT.md), then select the rules applicable to the actual task. This file is a project pointer and additions, not a modified copy of the standard.

Baseline reviewed on 2026-10-01: **4.1.0 candidate**, commit
`05bd9a8d71c11280d972b96caf64b776f2a075d7`. The standard identifies 4.0.0 as
its published stable baseline; this task follows the user-selected current
repository. Pin both version and commit for subsequent milestone evidence.

## Current continuation: quality-preserving performance

The user resumed Render / RAM Preview acceleration on 2026-10-01. The earlier
Stage 8 skip remains historical; it does not cancel the resumed task.

- Artifact: native AE effect, Rust host + C++ CPU renderer; macOS Apple Silicon,
  AE 2025 / 25.6 accepted scope.
- Risk: **Critical** for renderer/performance changes. Delivery: **Development**;
  validation and release are separate gates.
- Applicable: Process Core §§1–13; Engineering §§14–21, §24, §34; Native §23;
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
  Development does not authorize installation, cache purge, merge or release.

Read [current status](docs/current-status.md) and
[the resumed performance record](docs/performance-resume-2026-10-01.md).
Required FAIL, BLOCKED or NOT RUN is not PASS.

The prior FSTR-Line pointer and 2026-09-29 blob
`701a8c1ae3acb4dbfe1d7eda94acbf8095b88608` remain historical provenance.
New work uses the central standard selected above.
