# Windows x64 port status

This is the original port checkpoint. The active two-platform update now follows
rules8.0.0 on `feat/next-update`; see [current status](STATUS.md) and
[task mapping](feature-backlog.md#active-update-task-and-check-mapping--rules800).
The scope and authorization statements below describe the original port phase.

Updated: 2026-10-03

## Identity and scope

- Branch: `feat/windows-x64-aex`.
- Base: `main@9d0162de64d01ceb41f6a1374a73544729ed0ec2`.
- Windows-port rules baseline: AE-Development-Rules 6.2.0 /
  `d966078a9e45fee7ec9ad14f211a9da753d64b8a`.
- Historical project evidence keeps its original 6.0.0 baseline.
- Risk profile: Critical native; delivery gate: Development.
- No merge, release, publication or installation is authorized.
- Windows CPU first. Windows GPU and persistent grid display remain out of scope.

## Portability audit closure

The initial blockers are implemented:

- **W1 lifecycle/main thread:** Windows captures the thread used for
  `PF_Cmd_GLOBAL_SETUP`. Adobe documents Global Setup/Setdown as main-thread-only;
  AEGP work remains confined to that thread or the idle hook. Sequence/render
  selectors may still arrive concurrently and do not promote workers.
- **W2 PiPL:** Windows uses a local compatibility shim around pinned
  `pipl 0.1.1`, changing only resource emission to the upstream byte-safe
  binary-file RC form from commit
  `83dcc93734fd5db1335b6ec83cba7a6505a39dcc`. macOS keeps the original path.
- **W3 Python launcher:** `build.rs` uses `python` on Windows and `python3`
  elsewhere. MSVC compiler identity is recorded correctly.
- **W4 packaging:** `tools/package_windows.py` creates a non-installing
  `FSTR Stretch.aex` plus manifest, requiring exact Build ID/SHA identity.
- **W5 CI:** Windows Server 2022/MSVC x64 builds and tests the portable core and
  Rust host, enforces Clippy `-D warnings`, packages the AEX, verifies x64 PE,
  `EffectMain`, byte-exact PiPL, and records imports/dependent DLL inventories.

Additional Windows portability fixes:

- Custom UI drag refcons use Adobe `A_intptr_t` rather than platform `isize`.
- Optional render diagnostics compile on Windows without Unix-only filesystem APIs.
- Build Identity declares `.aex` for Windows and includes `AESDK_ROOT` in its
  settings hash so an external SDK override cannot silently reuse the same Build ID.
- The local PiPL compatibility shim is recorded in `THIRD_PARTY_NOTICES.md`.
- The Windows build-validation finalizer is bound to the exact GitHub Actions
  run/attempt/SHA and cannot emit PASS evidence outside that CI context.

## Runtime validation packet

The validation artifact contains:

- exact `FSTR Stretch.aex`;
- artifact manifest and build/static validation JSON;
- PE headers, exports, imports, dependent-DLL and PiPL evidence;
- `tools/windows_ae_validation.py`;
- `tools/windows_first_application.py`;
- existing pixel comparator and guarded AE JSX fixtures;
- `docs/windows-ae-validation.md`.

The runtime tools do **not** install or replace a plug-in. They verify that an
already-installed candidate matches the manifest and that the exact loaded module
path/SHA/Build ID belongs to the selected After Effects process.

Automatable Windows AE scope prepared:

- load/identity;
- guarded disposable project setup;
- CPU render/pixel smoke;
- save/reopen state roundtrip;
- cold-start first application and deferred auto-binding;
- evidence retention.

Still requiring actual Windows+AE execution and/or interactive observation:

- custom UI guide drag/cursor;
- Undo/Redo through real UI;
- full first-application matrix (8/16/32 bpc, solid/text/precomp, applicable 2D/3D);
- absence of transient modal error on first application;
- Render Queue/MFR/`aerender`;
- Windows performance measurements.

## Compatibility

See [Windows API compatibility audit](windows-api-compatibility.md).

Current classification:

- AE 2025+/25.x Windows x64: **UNKNOWN runtime** until tested.
- AE 23.4 Windows x64: **STATIC-COMPATIBLE candidate floor**, not VERIFIED.
- Windows ARM64: **UNSUPPORTED in this scope**.
- Windows GPU: **UNSUPPORTED in this scope**.
- Existing macOS product: must remain green in the separate macOS source gate.

The normal Windows build uses the pinned built-in `after-effects-sys 0.4.0`
bindings when `AESDK_ROOT` is unset. Supplying `AESDK_ROOT` is a different
build configuration and now changes Build Identity.

## Evidence boundary

A green Windows build/static gate proves only:

- MSVC/x64 compilation;
- portable C++ test suite;
- Rust host tests;
- warning-free Rust host under the configured Clippy gate;
- AEX packaging;
- exact PE/export/PiPL structure;
- captured PE imports/dependent DLL inventories;
- exact source/Build ID/artifact SHA identity.

It does **not** prove After Effects runtime behavior.

The final handoff candidate must be rebuilt from the final documentation/source
HEAD and its artifact validation JSON is the authoritative run/Build ID/hash
record. Do not reuse an earlier artifact after a tracked documentation change,
because documentation participates in the project's source identity.

## Stop condition for this stage

Development may advance to Windows AE validation only when:

1. exact-head Windows build/static gate is PASS;
2. exact-head macOS source gate is PASS;
3. the retained Windows validation artifact identifies that exact source;
4. no install/merge/release action has been taken.

Windows AE runtime remains `NOT RUN/BLOCKED` until an appropriate Windows
After Effects environment executes the prepared validation packet.
