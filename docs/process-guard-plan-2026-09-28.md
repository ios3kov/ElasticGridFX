# Narrow process-guard correction — 2026-09-28

Baseline: 8e60cce10283fd86bdd74fea5bc343bc7bc04ee9 on fix/final-validation.
User reports AE is closed but installation still refuses. The prior uploaded
startup report contains both a real AE executable and crashpad_handler children;
it is not a current process snapshot after closing AE. Current cause is not yet
proven. Source review finds the broad path substring guard also refuses when
only crashpad_handler remains, because its parent directory contains After Effects.

Scope: correct this reproducible false positive only; make remaining blockers
explicit by PID/name; retain early launcher output instead of demanding a
nonexistent receipt. No new renderer, native plugin, install target, swap/rollback,
permissions, candidate hash or user-process termination. Existing authorization
is only the per-user FSTR FX/ElasticGrid.plugin copy with an intact old backup.

Acceptance fixed before editing production code:
- A lone crashpad_handler in the actual reported AE bundle path must not count
  as an open rendering host. Similar but unknown executable names still block.
- Real AE, aerender, Premiere, Media Encoder and dynamiclinkmanager still block;
  other Adobe helpers remain conservative blockers rather than being guessed safe.
- Both installer and standalone historical startup checker use the same tested
  classification. Invalid/failed/empty process collection is not permission to install.
- Actual replacement/rollback guards still run before modifications and before
  swap. A new refusal lists the exact observed PID/executable, no bypass flag.
- Launcher creates a private log before Python and keeps exit status on all
  ordinary exits, including missing Python and interpreter failure.
- Unit regression must fail on the original crashpad guard, then pass; full
  portable tooling regression and native macOS process/installer fixtures gate
  handoff. Download and verify the exact CI package; reuse candidate 6d3b846 ZIP.
- The user Mac, real installation, loaded effect and rendering remain unverified.

Primary rationale: Crashpad describes itself as a crash-reporting system:
https://chromium.googlesource.com/crashpad/crashpad/
Dynamic Link remains a rendering-related conservative blocker:
https://helpx.adobe.com/lu_en/after-effects/desktop/work-with-other-applications/work-with-dynamic-link/dynamic-link-effects.html
No code copied from these references. The scope is not an exhaustive Adobe
process architecture audit and does not authorize killing background helpers.

## Implemented and local checks

The exact `crashpad_handler` leaf is excluded, without wildcard exemptions for
other Adobe helpers. The PID/full executable of each remaining blocker is
included in the refusal. `ps -axww -o pid=,comm=` prevents width truncation and
avoids matching user arguments. Malformed/failed/empty collection refuses;
PID 0 kernel_task is accepted as a normal OS record, not a rendering process.
The standalone historical checker mirrors the classifier; regression tests
compare them, but its package-integrity check still targets historical 923f806.
Do not use that old package checker to validate the newly packaged updater.

The updated launcher creates mode-0700 log directories and a mode-0600 log
before Python. Interpreter/startup errors and exit code are retained. New package
unpacks to ElasticGridFX-Test-Update-ProcessFix to avoid confusing it with the
old updater. The embedded candidate ZIP stays byte-identical to 6d3b846.

Local Linux results: original crashpad-only refusal reproduced; all Python
checks: 124 PASS / 6 macOS-only SKIPPED (130 discovered); 25 Node VM control-flow
cases PASS; Bash syntax and whitespace checks PASS. This stage does not change
C++ or Rust renderer sources. macOS process/update/native exchange tests and the
exact CI package must be verified at the resulting commit before handoff.
No actual user installation or AE render success is inferred from these tests.
