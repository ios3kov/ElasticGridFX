# GitHub CI / validation

## Automatic CI

Every push to `main` and every pull request runs:

- GCC Release build + portable regression tests;
- Clang Release build + portable regression tests;
- shell syntax audit for all `.command` / `.sh` scripts;
- portable C++ static analysis with Clang high warnings / analyzer / clang-tidy / cppcheck;
- ASan + UBSan + leak detection;
- GCC ThreadSanitizer for MFR and determinism.

The fuzz and soak tests use reduced CI iteration counts; full validation remains available through the project scripts.

On Apple Clang, the static-analysis script suppresses only `-Wpoison-system-directories`, an SDK/Homebrew search-path diagnostic emitted by hosted runners; source warnings remain `-Werror`.

Rust Clippy is intentionally **not** executed on the Linux job. The pinned `after-effects 0.4.0` host crate is macOS/Windows-only and does not compile as a Linux host crate. Clippy remains mandatory in the real macOS source gate, so this is a platform-correct split rather than a skipped release check.

## macOS source gate

`.github/workflows/macos-source-gate.yml` is intentionally manual via **Actions → macOS source gate → Run workflow**.

It first requires the committed `host-rust/Cargo.lock` and validates it with `cargo metadata --locked`; the workflow never regenerates the dependency graph. It then runs the macOS source-level preflight, including mandatory Rust Clippy, Apple Metal shader compilation, Objective-C++ Metal backend/test compilation, and produces an unsigned-install-free `ElasticGrid.plugin` artifact plus reports.

GitHub-hosted macOS ARM runners currently expose no `MTLDevice`. Therefore only the four **hardware execution** stages (Metal lifecycle, image parity, determinism and 4K/8K benchmark) switch to compile-only under the workflow-only `ELASTICGRID_METAL_COMPILE_ONLY=1` flag. The normal/local preflight never sets this flag, so a physical target Mac must still execute and pass all four stages.

The first real hosted-Mac run exposed two Rust-host blockers that portable Linux validation could not see: the AE plugin trait requires a mutable `handle_command` receiver, and Rust 1.98 check-cfg requires the cfg names expanded by `after-effects 0.4.0` to be declared. Both are fixed in the validation branch and remain covered by the macOS gate.

The next Clippy pass found redundant casts/rebinding and an over-wide float-slider helper in ElasticGrid code; those were corrected directly. The pinned `after-effects 0.4.0` macro itself emits `clippy::drop_non_drop` and `clippy::question_mark` on Rust 1.98. Item-macro attributes cannot suppress those expansion lints, so only those two known upstream style lints are allowed on the Clippy command line; every other warning remains `-D warnings`. This workflow does **not** replace the real After Effects runtime gate because GitHub-hosted runners do not have the target user's After Effects installation/project environment.

The reproducible-build stage performs two fully clean, locked, offline Release builds using the same canonical `CARGO_TARGET_DIR`, deleting it completely between builds. This deliberately holds the build path constant so the gate measures deterministic rebuilding under identical settings rather than Cargo/rustc build-path variance. A mismatch prints SHA-256 values, first differing byte offsets, and Mach-O UUIDs for the dylib.

The hosted gate also verifies packaging metadata produced by the pinned toolchain. `pipl 0.1.1` generates the macOS PiPL resource but does not generate the `PkgInfo` and `Info.plist` files expected by the project's bundle step; ElasticGrid now creates those deterministic metadata files in its own build script while keeping the dependency pin unchanged.

## Release rule

A green GitHub CI result means the portable/static/sanitizer checks passed. A release candidate still requires the documented target-Mac + After Effects runtime gate.

### Gate hardening note

The first hosted compile-only Metal run caught a malformed shell insertion before any plugin code was executed. The four Metal helper scripts were rebuilt with the compile-only branch after the linker command, and automatic `bash -n` coverage was added to normal CI so future shell syntax regressions fail immediately.


## Latest hosted result

Validation head `f3bca10` passed automatic CI and macOS source gate #33 on 2026-09-27.

- all 20 macOS preflight stages: PASS;
- clean reproducible plugin build: PASS;
- bundle verification/signing: PASS;
- artifact: `ElasticGridFX-mac-source-gate`;
- artifact digest: `sha256:7dd867b6933a68f72081bf14cc9887e158c5b1d1e164cdb0fbded65e7aad91d6`;
- committed `host-rust/Cargo.lock` hash: `5d77f2ce76302850bd390de5f44d34e772b3d451b706fab257e08fff998d957e`.

This still does not replace physical-Mac Metal execution or a real After Effects runtime/project test.
