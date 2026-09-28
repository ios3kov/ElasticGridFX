# Authorized single-copy replacement — acceptance plan

Baseline tooling: dd1bfee7f474a6266ca788ae71811ea2148b074e.
Candidate stays 6d3b846 / EGFX-603e9d3e4025d271e0488201; no native rebuild.
Rules re-read: FSTR-Line DEVELOPMENT_RULES.md on 2026-09-28.

User explicitly approved replacing only the reported per-user
`~/Library/Application Support/Adobe/Common/Plug-ins/7.0/MediaCore/FSTR FX/ElasticGrid.plugin`,
with the current version preserved for rollback. This is authorized test installation,
not production release acceptance. Report 7b291509 records old executable SHA256
4d21118301725178fbc6ba3ecea5e4ed053c9275accea10b6db0f51e3c657acf.

Acceptance fixed before implementation:
- Pin destination and old binary identity; refuse missing, changed or symlinked targets.
- Validate the existing candidate ZIP, payload and native signature; do not rebuild,
  re-sign, download code during installation, elevate, alter preferences or kill AE.
- All Adobe render hosts must be stopped before replacement/rollback. Never launch AE.
- Swap entire directories atomically on the same filesystem with Apple renamex_np.
  The original directory/inode and its metadata become the backup; no lossy copy.
- A durable PREPARED receipt exists before swap. Verify the original backup and the
  installed candidate. On post-install validation failure, restore only if both sides
  still match the expected recorded state and hosts remain stopped.
- Explicit rollback verifies receipt, exact target, old snapshot and current candidate.
  Retain both versions, all failures and reports. No recursive delete or replacement
  of unrelated/unknown state. Repeated invocation must not discard the backup.
- Test failed preparation, corrupt ZIP, wrong old identity, running host, symlinks,
  swap failure, post-install verification failure, interrupted receipts, manual
  rollback and idempotence with isolated fixtures. A real macOS swap/rollback and
  xattr-preservation test must pass before handing over this installer.
- Native renderer unchanged; use existing candidate's completed source/package evidence.
  New tooling gets Python/CLI tests and exact-commit macOS tests; Linux CI stays enabled.

Scope limit: this replacement does not claim conflict-free global installation,
loaded-image identity or functional AE/Metal PASS. The previous report has an
unresolved app-plugin scan error and native/sampler path contradiction; neither
is silently waived or described as fixed. Their resolution remains a runtime gate.

Primary references: Apple's xnu bsd/sys/stdio.h and bsd/man/man2/rename.2
(RENAME_SWAP=2, RENAME_NOFOLLOW_ANY=16), Apple's APFS Tools and APIs guide.
Python shutil documentation warns that copying cannot preserve all Mac metadata;
retaining the original directory by atomic swap avoids that loss.
