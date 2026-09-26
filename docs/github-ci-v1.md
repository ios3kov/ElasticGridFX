# GitHub CI / validation

## Automatic CI

Every push to `main` and every pull request runs:

- GCC Release build + portable regression tests;
- Clang Release build + portable regression tests;
- portable C++ static analysis with Clang high warnings / analyzer / clang-tidy / cppcheck;
- ASan + UBSan + leak detection;
- GCC ThreadSanitizer for MFR and determinism.

The fuzz and soak tests use reduced CI iteration counts; full validation remains available through the project scripts.

On Apple Clang, the static-analysis script suppresses only `-Wpoison-system-directories`, an SDK/Homebrew search-path diagnostic emitted by hosted runners; source warnings remain `-Werror`.

Rust Clippy is intentionally **not** executed on the Linux job. The pinned `after-effects 0.4.0` host crate is macOS/Windows-only and does not compile as a Linux host crate. Clippy remains mandatory in the real macOS source gate, so this is a platform-correct split rather than a skipped release check.

## macOS source gate

`.github/workflows/macos-source-gate.yml` is intentionally manual via **Actions → macOS source gate → Run workflow**.

It runs the macOS/Metal source-level preflight, including mandatory Rust Clippy, and produces an unsigned-install-free `ElasticGrid.plugin` artifact plus reports.

The first real hosted-Mac run exposed two Rust-host blockers that portable Linux validation could not see: the AE plugin trait requires a mutable `handle_command` receiver, and Rust 1.98 check-cfg requires the cfg names expanded by `after-effects 0.4.0` to be declared. Both are fixed in the validation branch and remain covered by the macOS gate.

The next Clippy pass found two redundant casts and an over-wide float-slider helper in ElasticGrid code; those were corrected directly. Two style lints are emitted inside the pinned third-party `ae::define_effect!` macro itself, so their allows are scoped only to that macro invocation rather than weakening project-wide Clippy. This workflow does **not** replace the real After Effects runtime gate because GitHub-hosted runners do not have the target user's After Effects installation/project environment.

## Release rule

A green GitHub CI result means the portable/static/sanitizer checks passed. A release candidate still requires the documented target-Mac + After Effects runtime gate.
