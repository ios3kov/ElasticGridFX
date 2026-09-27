# ElasticGrid FX v1.0 candidate — Mac / After Effects Runtime Gate

Status: READY TO RUN ON TARGET MAC
Date: 2026-09-27

## Automated gate

1. Run `BUILD_AND_INSTALL_MAC.command`. It validates Xcode tools, ensures Rust >=1.85 + Clippy (auto-installing/updating user-local stable Rust when needed), runs the full physical-Mac preflight, builds/signs and installs the plugin.
2. Fully quit and reopen After Effects into a new empty unsaved project.
3. Run `tools/ae_runtime_check_macos.command --full`.

The automated path validates:
- macOS toolchain and Rust host build
- Clippy/static-analysis gate
- ASan/UBSan and concurrency verification
- Metal lifecycle/setup-setdown
- CPU ↔ Metal image parity
- Metal determinism under sequential and concurrent render
- production-path 4K and 8K Metal benchmarks
- dependency/RustSec/SBOM gate using the committed locked dependency graph
- two clean offline reproducible Release builds
- plugin bundle structure and code signature
- After Effects plugin discovery/application by match name
- real 32-bpc Final Bicubic frame render
- temporary AEP save → close → reopen
- parameter/keyframe persistence after reopen
- second real frame render after reopen

## Manual AE checks after automated PASS

These require interaction with the Viewer and are therefore intentionally manual:
- drag column and row guides in Comp/Layer Viewer
- verify elastic propagation and non-crossing constraints
- Undo / Redo guide movement
- 1/2 and 1/4 preview resolution correctness
- non-square pixel-aspect composition
- precomp and upstream resized-layer/origin cases
- multiple effect instances with MFR enabled

## Freeze rule

Functional code hash is frozen in `code-freeze-manifest-v1.0.txt`. If the target-Mac gate fails, change only the blocker, then rerun the full portable regression and regenerate the freeze manifest before another Mac attempt.
