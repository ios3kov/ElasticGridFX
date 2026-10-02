# AE engineering know-how — performance/release cycle

Date: 2026-10-02. Source: [current-cycle retrospective](retrospective-0.9.3-perf.1.md).
These patterns are based on f611312 / EGFX-6147dc406abc596e7f2d1b60 and the named
AE25.6x101/macOS26.6.2/Apple Silicon fixtures, not a promise for every AE product.

| Pattern | Confidence / outcome | Evidence and reuse limit |
|---|---|---|
| Identify the exercised render route before optimizing | PROVEN / accepted | [Attribution](performance-host-attribution-2026-10-02.json); observed routes and instrumented timings do not transfer to ordinary runtime acceptance. |
| Cache separable axis/Bicubic work with unchanged arithmetic | PROVEN / accepted | [Corrected comparison](performance-plane-corrected-comparison-2026-10-01.json), [exceptional floats](performance-plane-nonfinite-comparison-2026-10-01.json); retain general/exceptional paths, call-local storage and independent checks. |
| Match source toolchain and export contract for parity | PROVEN for recorded outputs / accepted | [Host matrix](performance-host-matrix-2026-10-02.json); preserve earlier-artifact deltas and do not call RGBA16 PNG proof universal 32-bit/HDR fidelity. |
| Separate native sampling, total export, cache fill and playback | PROVEN exports / OBSERVED UI bounds / accepted | [Render](performance-host-render-2026-10-02.json), [Preview](performance-host-preview-intervals-2026-10-02.json); retain outliers, preset uncertainty, capture/cache limits and missing exact latency. |
| Require complete decoded frames, not launcher exit0 | PROVEN / accepted | [MFR incident](performance-host-mfr-2026-10-02.json); expected coverage and process/image identity matter. |
| Bound retries; non-reproduction is not a causal fix | PROVEN retry outcomes / UNVERIFIED cause / accepted | [Retry closure](mfr-closure-retry-2026-10-02.json); requested MFR is not measured concurrency, closure is not stability certification. |
| Check the downloaded immutable payload and actual loaded identity | PROVEN on existing Mac / accepted | [Public install](public-install-project-check-2026-10-02.json); clean-environment acceptance is separately [USER-REPORTED](release-user-acceptance-2026-10-02.json). |

## Transfer to the central engineering base

The generalized document is prepared in AE-Development-Rules at commit
`9484dac9bf232831646b5930b34b7141e31b91c8`, branch
`docs/reusable-ae-render-validation`, file `docs/AE_ENGINEERING_KNOWHOW.md`.
It uses immutable references to already-public, sanitized source evidence; it
contains no user project, raw crash log, media, credentials or private environment path.
The central note adds no new mandatory policy or tooling/interface change.

Transfer status: **PUBLISHED FOR CENTRAL REVIEW** in
[AE-Development-Rules PR16](https://github.com/ios3kov/AE-Development-Rules/pull/16),
exact head `9484dac9bf232831646b5930b34b7141e31b91c8`.
[Generalized central note](https://github.com/ios3kov/AE-Development-Rules/blob/9484dac9bf232831646b5930b34b7141e31b91c8/docs/AE_ENGINEERING_KNOWHOW.md).
The human explicitly authorized this second-repository push/PR after automatic
approval review requested separate consent. Local central starter-kit self-test
(dry-run, required POSIX) PASS; PowerShell runtime NOT RUN locally and is covered
by central PR CI. Publication for review is not a central merge or new rules release.
The product's frozen adopted baseline remains6.0.0; preparing notes against the
current central main does not silently adopt its newer6.1.0 baseline here.

Reusable source tooling already exists in `tools/render_observation.py`,
`tools/live_identity.py`, `tools/perf_fixture_runner.py` and the native comparison
fixtures. Review host, ownership, route, precision and cleanup assumptions before
adapting it. No new host/API calls or runtime checks were introduced in this task.
