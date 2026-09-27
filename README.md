# ElasticGrid FX

Clean-room After Effects grid-warp effect inspired by GridWarp behavior, with a speed-first architecture.
No original GridWarp source code is used.

## Current version: v1.0 candidate source — plugin metadata stays v0.9 until the target-Mac gate passes

The macOS build does **not** require the Adobe After Effects SDK or CMake. It uses the C++ render core plus the
open-source `after-effects` and `pipl` Rust crates for the AE host ABI/PiPL.

### Implemented

- direct grid-guide dragging in Comp/Layer Viewer;
- keyframeable grid state;
- Elasticity / Falloff / Tension / Min Spacing with no guide crossing;
- Smart Render CPU: 8 / 16 / 32 bpc;
- Metal Smart Render: 32-bpc AE GPU worlds, zero image readback/upload;
- Clamp / Wrap / Mirror;
- Preview Bilinear;
- default Final Catmull-Rom Bicubic;
- wave animation with correct dynamic AE cache invalidation;
- SIMD CPU hot path and identity fast path;
- Multi-Frame Rendering safe render core and Metal sampling-plan pool.

### Non-negotiable quality rule

`Final (Bicubic)` is never replaced by bilinear or another lower-quality approximation for speed.
No `-ffast-math` is used. CPU and Metal consume the same precomputed sampling indices and Catmull-Rom weights.
The real-Metal parity gate allows only tiny floating-point instruction-order/FMA differences:

- max absolute error <= `2.5e-5`;
- RMS error <= `3.0e-6`.

Any larger mismatch blocks the `.plugin` build.


## Final validation cycle (v1.0 candidate)

Before producing the next user test plug-in, the code goes through six final validation gates:

1. deterministic fuzzing / malformed-state hardening;
2. leak + soak testing;
3. deterministic output under CPU/MFR/Metal;
4. allocation + lock audit of the per-frame hot path;
5. full regression and performance recheck;
6. target-Mac/After Effects runtime gate.

Current status:

- Fuzzing: **PASS** — 180,000 deterministic randomized operations per normal run (60,000 codec/malformed blobs, 100,000 axis operations, 20,000 bridge/render cases), plus sanitizer fuzz smoke. No crash, hang, UB, accepted-invalid-state invariant break, or failed-decode mutation was observed.
- Leak / soak: **CPU PASS / MAC METAL SOAK PENDING** — 50,000 portable setup/plan/render/teardown cycles hold steady RSS after warm-up; 3,000 cycles pass ASan+UBSan+LeakSanitizer. Mac preflight now runs 128 Metal setup/setdown cycles, 512 concurrent renders and 2,048 shared-state soak renders.
- Determinism: **CPU/MFR PASS / MAC METAL DETERMINISM PENDING** — repeated 8/16/32-bpc renders are byte-identical across 1/2/4 worker counts; 384 concurrent MFR renders match the same reference exactly; GPU sampling plans are byte-identical across 256 rebuilds. Metal determinism is wired into Mac preflight.
- Allocation / lock audit: **CPU PASS / MAC METAL COMPILE-RUNTIME PENDING** — steady-state CPU render, GPU-plan build and ROI planning are verified at 0 heap allocations after warm-up. Core/bridge/Metal hot-path sources contain no blocking mutex primitives; Metal lifecycle and plan-pool synchronization were converted to atomics/CAS.
- Final regression: **PORTABLE PASS / RUST+METAL TARGET-MAC PENDING** — GCC strict 9/9, Clang strict 9/9, ASan+UBSan+LSan 9/9, TSan MFR+determinism 2/2, hot-path audit PASS. Same-machine v0.9 comparison shows no meaningful CPU regression (median deltas about -1.6% to +0.7% across the measured 4K/8K cases). Cargo is not installed in this container, so the Rust AE-host build remains part of the Mac gate.
- Mac runtime gate: **READY / TARGET-MAC EXECUTION PENDING** — all automated gates are wired, including real Metal lifecycle/parity/determinism/4K/8K, bundle verification, a real 32-bpc Final Bicubic AE render, and temporary project save/reopen/keyframe roundtrip. Viewer drag/Undo/Redo/downsample/PAR checks remain manual.

See `docs/final-validation-v1.0.md`.

## Code-freeze cycle

A final code-only freeze cycle is tracked in `docs/code-freeze-v1.0.md`. Step 1 static analysis is **C++ PASS / Rust Clippy target-Mac pending**: Clang `-Weverything` and the Clang Static Analyzer are clean after fixes. The one-click Mac preflight now runs `cargo clippy --all-targets -- -D warnings` before the sanitizer/performance/runtime gates; `clang-tidy` and `cppcheck` are also run automatically when installed.
Step 2 manual unsafe/FFI audit is **code-complete / Rust target-Mac compile pending**: ABI layouts are pinned, corrupt GridState vector lengths are bounded before allocation, ROI subtraction is overflow-safe, and production parameter setup no longer uses an allocation `unwrap()`.
Step 3 dependencies/licenses/SBOM is **source/policy complete / target-Mac lock+audit pending**: direct Rust dependencies are exact-pinned, RustSec audit is mandatory, and the Mac preflight generates a CycloneDX SBOM plus exact license inventory from `Cargo.lock`. The informational `bincode` unmaintained advisory is documented rather than hidden.
Step 4 clean reproducible build is **portable PASS / target-Mac plugin reproduction pending**: preflight builds the frozen host twice from a clean source snapshot with an isolated Cargo cache and networking disabled, then requires byte-identical unsigned dylib/PiPL/plist outputs.


## One-click Mac test build

Double-click:

`BUILD_AND_INSTALL_MAC.command`

It runs, in order:

1. ASan + UBSan core/bridge/quality/MFR tests;
2. ThreadSanitizer MFR stress test;
3. strict Release compile (`-Werror`, no fast-math);
4. CPU 4K + 8K benchmarks;
5. real Metal CPU/GPU image parity using the production render entry point;
6. real Metal 4K + 8K production-path benchmarks;
7. Rust/AE host Release tests;
8. target-Mac Cargo.lock + RustSec audit + CycloneDX SBOM;
9. locked + offline dependency test;
10. `.plugin` assembly + ad-hoc signing;
11. PiPL / entrypoint / signature / dependency verification;
12. install to Adobe MediaCore.

A full report is written to:

`dist/mac/preflight-report.txt`

After installation, fully quit/reopen After Effects into a **new empty unsaved project**, then run:

`tools/ae_runtime_check_macos.command --full`

This checks the installed bundle/loading logs, performs a real 32-bpc `Final (Bicubic)` frame render, then saves/closes/reopens a temporary `.aep` and verifies parameter/keyframe persistence plus another render. The gate refuses to modify an existing or dirty project. Viewer drag/Undo/Redo/downsample/PAR checks remain manual. See `docs/mac-runtime-gate-v0.9.md`.

## Current hardening status

- 1. FFI / crash-safety: **PASS** — release panic catching enabled; C++/Objective-C exceptions are contained at the host ABI; malformed FFI inputs covered by tests.
- 2. Metal lifecycle: **CODE PASS / MAC GATE PENDING** — RAII-owned native state, setup/setdown cleanup, in-flight render teardown barrier, repeated-init/concurrent-render Metal test added.
- 3. Render cancel / abort: **PASS** — CPU polls AE cancellation between bounded row batches on the host thread; Metal polls immediately before and after command-buffer execution; dedicated cancel status maps to `InterruptCancel`.
- 4. Edge cases: **PASS** — 1×1, max 128×128 topology, 8K plan sizing, negative time, extreme wave/spacing, NaN/Inf hardening covered.
- 5. AE spatial correctness: **CODE PASS / MAC VIEWER GATE PENDING** — ROI-aware SmartFX checkout, non-zero input/output origins, downsampled canvas coordinates and stable full-layer max bounds are implemented; partial-ROI output is regression-tested against full-frame rendering.
- 6. Project/keyframe compatibility: **CODE PASS / AE UNDO-SAVE GATE PENDING** — stable effect identity, frozen parameter IDs, backward-readable v0.8 GridState wire format, explicit schema versioning and migration tests added.
- 7. Full regression/audit: **PORTABLE PASS / RUST+METAL TARGET-MAC PENDING** — GCC strict 9/9, Clang strict 9/9, ASan+UBSan+LeakSanitizer 9/9, TSan MFR+determinism 2/2, allocation/lock audit PASS, quality/plan parity PASS. Same-machine comparison against clean v0.9 is within benchmark noise; Rust host compile/tests and real Metal remain Mac-only.
- 8. Mac runtime build/smoke: **PACKAGE PASS / TARGET-MAC EXECUTION PENDING** — one-click build, Metal lifecycle/parity/4K/8K gates, bundle verification, and a safe real-AE 32-bpc render smoke are wired; actual PASS requires running them on the target Mac.

Portable final audit: **9/9 PASS** on GCC and Clang with `-Werror`; **9/9 PASS** under ASan/UBSan/LeakSanitizer; MFR + determinism **2/2 PASS** under GCC TSan; allocation/lock audit **PASS**. Cargo is unavailable in this container, so Rust host compile/tests stay pending for target Mac. Mac-only Metal/AE claims are intentionally not marked PASS until the scripts run there. See `docs/verification-v1.0.txt` and `docs/performance-v0.9.md`.

### Final portable regression status

The final post-hardening portable gate is green: 9/9 Release tests, ASan/UBSan/LeakSanitizer, and GCC ThreadSanitizer MFR/determinism checks pass. See `docs/final-regression-v1.0.md`. Real Metal performance and After Effects integration remain gated on the target Mac and are intentionally not inferred from the Linux validation host.


### Code freeze

Functional code is now frozen. Aggregate frozen-source hash: `e8d043c7bbab6d7674887558c8c80d6afb55d02ba1e3e9a3cf087236074ddeb3`. See `docs/code-freeze-manifest-v1.0.txt`. No functional edits are permitted until the target-Mac/After Effects gate runs, except fixes for blockers discovered by that gate.


### Target-Mac runtime gate ready

All runtime/build scripts pass shell syntax validation and the full gate is documented in `docs/mac-runtime-ready-v1.0.md`. Actual Metal and After Effects results remain intentionally pending until run on the target Mac.

## GitHub CI

Portable regression, static analysis and sanitizers run automatically on pushes to `main` and pull requests. The expensive macOS/Metal source gate is a manual GitHub Actions workflow and does not replace the final real-After-Effects runtime gate.

See `docs/github-ci-v1.md`.
