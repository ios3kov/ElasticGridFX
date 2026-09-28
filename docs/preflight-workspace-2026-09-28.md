# Preflight workspace regression — 2026-09-28

Baseline: 64aa67e10110f25a7b1c7d0ff8dd4ff861793444. Scope: test build directories
and evidence retention only; no renderer, host parameter or identity algorithm
change. The clean-Git requirement remains mandatory.

## Observed failure and reproduction

macOS source run 36455408377, job 109040373961, passed Rust 11/11 (including saved
Falloff ordinals), Clippy, native sanitizers, allocation/lock audit, dependency
audit and the Metal compile-only stages. It failed stage 20 with
`reproducible source snapshot requires clean Git`. No final plugin was packaged.

The scripts used five unignored root directories: `.hotpath-audit/`,
`.metal-bench-macos/`, `.metal-determinism-macos/`, `.metal-lifecycle-macos/` and
`.metal-parity-macos/`. In an isolated baseline worktree, running the real
portable hotpath audit passed but created `?? .hotpath-audit/test_allocations`.
The clean snapshot then failed with the same error. This proves that the tool's
own generated output was sufficient to cause the failure; the old CI log did
not include a complete dirty-file listing.

## Acceptance and implementation

Move each test's outputs to a distinct child of the existing `.preflight-macos/`
workspace, already ignored by Git and excluded from the reproduction export.
Do not broaden ignore rules or silently discard source changes. A symlinked
workspace parent is refused before any cleanup. Old root scratch directories
are neither deleted nor retroactively ignored by this change.

The macOS workflow preserves source status and diagnostic reports even when a
required check fails. Only successful builds upload the installable candidate;
failed-run diagnostics are kept under a separate artifact name. No sanitizer,
reproducibility, identity or host acceptance check is removed or downgraded.

## Tests

The five new Python tests execute/check actual declared script paths and the
actual tar export pipeline: distinct workspace children; generated outputs
leave source identity clean; export excludes outputs while validating every
source byte; unknown source/legacy scratch remains dirty; and the portable
script refuses a symlink parent without touching a sentinel file.

They expose failures on the baseline and pass after the correction. Combined
Python tests: 29/29 PASS locally. Shell syntax and whitespace checks PASS.
Exact-commit CI and the post-commit real hotpath run are recorded separately in
the PR checkpoint; source-contract tests do not prove macOS compilation or AE
runtime behavior. Real After Effects and hardware Metal remain BLOCKED.
