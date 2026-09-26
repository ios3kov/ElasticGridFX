# v0.6 performance / quality audit

Date: 2026-09-26

## Non-negotiable rule

Final-render speedups are accepted only when they preserve the intended final-quality sampling result.
There is no `-ffast-math` build and no replacement of final bicubic with bilinear.

## Audit findings fixed

### 1. Release tests were accidentally weaker than intended

CMake Release defines `NDEBUG`. Several tests used `assert(function_call(...) == 0)`, so those function calls were
compiled out in Release. This made an earlier Release test pass less meaningful than it appeared.

Fix: every verification target now explicitly undefines `NDEBUG` (`-UNDEBUG` / `/UNDEBUG`). Release tests now
execute the assertions and their side effects. Core, bridge and GPU-plan suites pass with assertions actually active.

### 2. No-op frames still paid the full resampling cost

Fix: prepared warps now detect exact identity geometry and use a parallel row-copy path. This is mathematically exact
for the same pixel format and avoids interpolation entirely.

### 3. A separate Metal warp algorithm could drift from CPU quality

Fix: the CPU builds the sampling plan once (indices + fractional coordinates / Catmull-Rom weights) and Metal
consumes that exact plan. Metal does not independently approximate the inverse warp or weights.

### 4. Metal hot path allocated plan buffers every frame

Fix: Metal now uses an MFR-safe reusable shared-memory buffer pool. The Rust host also reuses per-render-thread
sampling-plan vectors after warmup, removing steady-state plan allocations.

## Development-container verification

Environment: Linux/x86-64 container. These are engineering baselines, **not Mac/After Effects numbers**.

Current median-ish 4K full-bridge measurements from repeated runs:

- 8-bpc bilinear deformed: ~26.4 ms/frame
- 16-bpc bilinear deformed: ~16.3 ms/frame
- 32-bpc bilinear deformed: ~7.8 ms/frame
- 8-bpc bicubic deformed: ~69 ms/frame
- 16-bpc bicubic deformed: ~57 ms/frame (run-to-run variance observed)
- 32-bpc bicubic deformed: ~32 ms/frame

Identity 4K fast path:

- 8-bpc: ~1.5-1.7 ms/frame
- 16-bpc: ~2.8-3.1 ms/frame
- 32-bpc: ~6.2-7.1 ms/frame

The identity cost scales mostly with bytes copied, as expected.

## Verification status

Passed in the development container:

- GCC Release: core / bridge / GPU-plan tests
- Clang Release with `-Werror`: core / bridge / GPU-plan tests
- Clang ASan + UBSan: core / bridge / GPU-plan tests
- CPU-generated GPU sampling plan vs CPU renderer parity test

Mac-only gates intentionally remain unclaimed until run on the target Mac:

- Metal shader compile/pipeline creation
- CPU vs Metal pixel parity across Bilinear/Bicubic and Clamp/Wrap/Mirror
- 4K Metal end-to-end benchmark
- Rust AE host macOS compile
- After Effects load/runtime smoke

## Metal parity tolerance

The macOS parity test compares real Metal output to the CPU 32f reference for all three edge modes and both quality
modes. Allowed differences are only small floating-point instruction-order/FMA differences:

- max absolute error <= 2.5e-5
- RMS error <= 3.0e-6

A larger mismatch blocks plugin packaging.
