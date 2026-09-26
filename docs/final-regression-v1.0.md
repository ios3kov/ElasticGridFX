# ElasticGrid FX v1.0 — Final Regression

Date: 2026-09-27

This document records the final portable regression pass after static analysis, FFI hardening, dependency pinning and reproducibility work.

## Functional regression

Release CMake suite: 9/9 PASS.

Covered targets:
- core grid model and codec
- C ABI bridge
- CPU/GPU sampling-plan parity
- Final Bicubic reference quality
- MFR concurrency
- deterministic fuzz smoke
- setup/render/teardown soak
- deterministic output
- steady-state allocation audit

## Sanitizers

Clang ASan + UBSan + LeakSanitizer: PASS for all 9 verification targets. No sanitizer or leak report was emitted.

The soak test's RSS is not used as a leak verdict under ASan because ASan's allocator/quarantine intentionally retains pages. LeakSanitizer is the authoritative leak gate in that configuration.

## ThreadSanitizer

GCC ThreadSanitizer:
- MFR concurrency: PASS
- determinism concurrency: PASS

The container's Swift-flavoured Clang 17 TSan runtime cannot link on Linux because it expects libdispatch/Blocks symbols. This is a toolchain/runtime limitation, not a source failure; the same tests pass under GCC TSan. The target-Mac preflight still runs Apple's/macOS-compatible TSan configuration where available.

## Performance smoke

A final CPU performance smoke was run in the shared Linux validation environment with four worker threads. Timings were noisy due to shared-host scheduling, so they are treated only as a regression smoke, not as a product claim. Median-of-three observations were approximately:

- 4K 32-bpc Final Bicubic: 23.2 ms/frame
- 4K 16-bpc Final Bicubic: 28.3 ms/frame
- 4K 8-bpc Final Bicubic: 31.9 ms/frame
- 4K 32-bpc Bilinear: 9.7 ms/frame
- 8K 32-bpc Final Bicubic: 98.5 ms/frame

The authoritative performance gate remains the target-Mac production Metal benchmark in `tools/preflight_macos.command`.

## Result

Portable regression gate: PASS.
Mac/Rust/Metal/After Effects runtime gates: pending target Mac.
