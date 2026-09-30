# Target-Mac diagnostic plan — 2026-09-28

Baseline plugin: clean 6d3b846463410f37198fda4b625e56e4cea44c22, Build ID
EGFX-603e9d3e4025d271e0488201. PR #5 / fix/final-validation; no main change.
Rules: FSTR-Line DEVELOPMENT_RULES blob a1760fde8763f789b50b91c20407938b4fcaea4a,
sections 4, 7–10 and final rule rechecked. The current execution host is Linux;
no connected tool can execute on the user's Mac. The minimum justified request
is one non-installing diagnostic of the unique target environment, not serial
trial-and-error tests delegated to the user.

## Scope and acceptance before implementation

Build a separately identified diagnostic artifact (no plugin binary), pinned to
the already verified baseline candidate. Do not rebuild that plugin merely to
change diagnostic code. Inspect actual installed copies, app/version and one
running target AE. Do not install, replace files, save/close/create projects,
run JSX, reset preferences, kill AE, download software or change permissions.
The only writes are a new private per-run result directory and report ZIP.

Use Apple's process sampler to obtain the live image path/UUID and compare the
UUID with the exact signed payload's Mach-O LC_UUID. Match all pinned payload
hashes before/after sampling and the same process identity. This maps an observed
image UUID to a known Build ID; it does NOT read the Build ID string from memory,
prove pixel behavior or approve release. A missing/ambiguous/truncated report,
unsupported image format, unavailable permission or process replacement blocks
confirmation. Never substitute an on-disk ID for a live observation.

Sampling briefly pauses threads; keep it bounded (1 second, 10 ms samples,
15-second transport timeout), require an idle user-authorized AE, and do not use
these measurements for performance. Keep full stack data private locally; include
only relevant AE/plugin identity and sanitized paths in the shareable report.
No automatic retry after timeout. Do not upload any result automatically.

## Required tests and evidence

- Unit negative cases: malformed Mach-O/sample, no/multiple loaded candidates,
  different UUID/path, modified payload, permission failure, process replacement,
  stale report and no-application paths; explicit distinctions between observed
  image identity, direct runtime Build ID and readiness.
- macOS integration: an owned fixture process dlopens a fixture dylib; sample that
  process and confirm identity using the real Apple tool. This proves diagnostic
  mechanics, NOT After Effects loading. Never stop/sample arbitrary CI processes.
- Verify the diagnostic ZIP contents/hash against its own clean commit; baseline
  plugin manifest and observed UUID are retained as fixture data, not new claims.
- Existing portable checks and exact-commit CI; actual user-Mac outcome stays NOT
  RUN until the returned diagnostic evidence is inspected. No release or install.
