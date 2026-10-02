# Windows x64 port status

Updated: 2026-10-02

## Identity and scope

- Branch: `feat/windows-x64-aex`.
- Base: `main@9d0162de64d01ceb41f6a1374a73544729ed0ec2`.
- Windows-port rules baseline: AE-Development-Rules 6.2.0 /
  `d966078a9e45fee7ec9ad14f211a9da753d64b8a`.
- Historical project evidence keeps its original 6.0.0 baseline.
- Development only: no merge, release, publication or installation is authorized.
- Windows CPU first. Windows GPU and persistent grid display remain out of scope.

## Completed implementation

- Windows lifecycle captures the `GlobalSetup` thread identity so later AEGP
  registration/idle/auto-binding work cannot promote a render worker.
- Optional render diagnostics no longer require Unix-only filesystem APIs.
- Build identity can read an MSVC compiler banner.
- Windows PiPL resource emission uses a local compatibility shim around the
  existing registry `pipl 0.1.1`; macOS continues to use the original dependency.
  The shim changes only Windows resource emission to a binary-file RC resource,
  matching upstream fix `83dcc93734fd5db1335b6ec83cba7a6505a39dcc`.
- Added non-installing `tools/package_windows.py`: identified DLL -> `FSTR Stretch.aex`
  with Build ID and SHA-256 checks; AE runtime fields stay `NOT RUN`.
- Added package regression tests.
- Added a Windows Server 2022 gate for MSVC x64 core tests, Rust host build/tests,
  AEX packaging, PE x64/EffectMain export and PiPL resource inspection.

## Compatibility

- Target working baseline: AE 2025+ on Windows x64.
- PiPL Effect API 13.28 maps statically to AE 23.4.
- StreamSuite6 is documented from AE 22.5; current source also uses KeyframeSuite5.
- Therefore AE 23.4+ is a static compatibility target, not a runtime claim.
- AE 2025+/23.4 Windows load, UI, idle/auto-binding, render, MFR, save/reopen and
  Undo are **NOT RUN** until a Windows AE environment validates the exact artifact.

## Current evidence

Windows workflow run: `37052513941`.
Status at this checkpoint: **IN PROGRESS**.

A successful compile/core/PE gate will prove only its recorded static/build scope;
it will not change any AE runtime item to PASS.
