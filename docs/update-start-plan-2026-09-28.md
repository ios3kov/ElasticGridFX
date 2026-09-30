# Missing installation report: bounded read-only diagnosis

Baseline updater 923f806; user output retains the old 4d211183... executable hash
and has no backup directory. This does not reveal which early guard failed.
The delivered updater only writes a shareable report after replace/rollback
returns; interpreter/import, self-integrity, platform and running-host refusals
can therefore leave no report. Do not claim an installation or invent a cause.

Scope: one separately source-identified diagnostic launcher. No plugin package,
installer execution, backup creation, security changes, process termination or
AE interaction. Existing signed candidate and replacement logic stay unchanged.

Acceptance fixed before implementation:
- Create a private, uniquely named text report BEFORE attempting Python.
- Preserve missing-Python and interpreter-failure output; no silent early exit.
- Check current platform/Python, known Adobe processes, authorized target hash,
  and a bounded search for the exact previously delivered updater. Validate its
  pinned identity and each member; never import or run downloaded updater code.
- Explain current blockers without claiming to reconstruct past terminal output.
- Record own SHA256; retain all old plugin/install state unchanged.
- Shell/Python control-flow, tampering, search-bounds and early-failure tests;
  execute the real shell on both Linux and macOS fixtures before delivery.
- No target-AE success or native install inferred from these tests.

References checked: Python 3.9 str.removeprefix reference (existing updater needs
>=3.9); previous authorized-update plan and delivered 923f806 source. Process
classification uses the same conservative Adobe-host list, with no guard bypass.

## Implemented checkpoint

CHECK_UPDATE_START_MAC.command creates report.txt before attempting Python; it
records missing interpreter/nonzero interpreter exit, current blocking Adobe
PIDs, interpreter version, unchanged target state and bounded package checks.
The earlier delivered 923f806 InstallToolIdentity.json is itself SHA256-pinned
before trusting member hashes. Source is only compiled for syntax, never imported.
A changed execute bit is reported separately from modified bytes. No updater is
launched, no existing report/state is deleted and no presumed cause is asserted.
Current-state PASS is deliberately not an installation or historical diagnosis.

Local focused tests: 9/9 PASS, including the real shell with absent/failing Python,
real report modes 0700/0600, changed package bytes/permissions, symlinks and search
limits. The same 9 tests run on Linux and macOS before helper delivery. Existing
native plugin and old updater package are unchanged; the old missing-report
limitation remains documented rather than claimed fixed in the shipped ZIP.
Actual Mac output is required to determine which current startup guard blocks.
