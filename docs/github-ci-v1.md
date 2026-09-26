# GitHub CI / validation

## Automatic CI

Every push to `main` and every pull request runs:

- GCC Release build + portable regression tests;
- Clang Release build + portable regression tests;
- Clang static analysis / clang-tidy / cppcheck / Rust Clippy;
- ASan + UBSan + leak detection;
- GCC ThreadSanitizer for MFR and determinism.

The fuzz and soak tests use reduced CI iteration counts; full validation remains available through the project scripts.

## macOS source gate

`.github/workflows/macos-source-gate.yml` is intentionally manual via **Actions → macOS source gate → Run workflow**.

It runs the macOS/Metal source-level preflight and produces an unsigned-install-free `ElasticGrid.plugin` artifact plus reports. This workflow does **not** replace the real After Effects runtime gate because GitHub-hosted runners do not have the target user's After Effects installation/project environment.

## Release rule

A green GitHub CI result means the portable/static/sanitizer checks passed. A release candidate still requires the documented target-Mac + After Effects runtime gate.
