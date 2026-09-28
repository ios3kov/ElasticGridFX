# Current development / release status

Date: 2026-09-28. Branch: fix/final-validation, draft PR #5.
**NOT READY FOR RELEASE.** No main merge or production release.

## Current user-Mac checkpoint

The user's executable hash is still the reported original `4d211183...657acf`;
the previous terminal output found no Test Backups directory. The returned
startup report listed real AE plus crashpad_handler and other Adobe helpers.
The user subsequently closed AE but reports the same refusal. That earlier
report is not a current process snapshot; the remaining blocker is not known.

A crashpad-only false positive was reproduced in the original process guard:
it matches `After Effects` anywhere in the executable's parent path. The fix
excludes only the exact `crashpad_handler` leaf. Other Adobe helpers, including
dynamiclinkmanager, remain conservative blockers; the refusal now lists their
PID/full executable. There is no ignore-processes switch or process termination.
The updater launcher now writes a private terminal log before starting Python.
See `process-guard-plan-2026-09-28.md` for acceptance, reproduction and limits.

## Candidate and authorized scope

Keep the immutable plugin candidate `6d3b846463410f37198fda4b625e56e4cea44c22`,
Build ID `EGFX-603e9d3e4025d271e0488201`. The user authorized replacement of ONLY
`~/Library/Application Support/Adobe/Common/Plug-ins/7.0/MediaCore/FSTR FX/ElasticGrid.plugin`,
with the original retained for rollback. Old hash, destination, atomic directory
exchange, original inode/metadata retention, signature checks and rollback logic
are unchanged. The new package has a distinct `ElasticGridFX-Test-Update-ProcessFix`
folder so it cannot be mistaken for the previous updater.

No C++/Rust render source or saved parameter schema changes in this correction.
The plugin ZIP is reused byte-for-byte, not rebuilt to match new tool changes.
No user-Mac installation success can be claimed until new output is received.

## Verification

Local Linux: original lone-reporter refusal reproduced; full Python suite
124 PASS / 6 macOS-only SKIPPED (130 discovered); Node VM control-flow 25/25;
Bash syntax and whitespace checks PASS. Native process classification and
update/rollback/signature/xattr tests must pass on exact-head macOS CI before
handoff. PR #5 records completed runs, final package hashes and any failures.
Source fixtures and successful packaging are not After Effects runtime evidence.

The standalone historical StartCheck still checks integrity of the old 923f806
installer only, despite its corrected process classification; do not use it to
validate the new installer. The new updater verifies its own packaged identity.

## Remaining mandatory acceptance

Actual installation and intact backup; actually loaded candidate identity;
patterned-image deformation and original guide-drag report; Undo/Redo,
save/reopen/restart and legacy projects; actual host 8/16/32-bpc alpha/HDR,
ROI/downsample/PAR, MFR, cancellation and aerender; physical Metal and profiling.
All remain NOT RUN/BLOCKED. The previous live-diagnostic path contradiction and
incomplete app-plugin scan are not fixed by the process-guard correction.
GPU dispatch remains disabled pending target-host validation.

## Prior work and evidence

Dated reports retain rendering identity fixes, safe project tests, parameter
semantics, source/package identity and scoped update history. See
`identity-fix-2026-09-28.md`, `host-identity-2026-09-28.md`,
`runtime-acceptance-2026-09-28.md`, `live-check-2026-09-28.md`, and
`authorized-update-plan-2026-09-28.md`. Historical PASS results certify their
original source/fixture scope only. Authoritative process: ../DEVELOPMENT_RULES.md.
