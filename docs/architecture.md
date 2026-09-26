# Architecture

## Host boundary

1. `host-rust/` — thin After Effects ABI/PiPL/Smart Render shim using permissively licensed
   `after-effects` + `pipl` crates with built-in bindings.
2. `src/bridge/` — stable C ABI boundary with validation/sanitization.
3. `src/core/` — clean-room C++ grid solver, inverse mapping, sampling-plan builder and CPU renderer.
4. `src/gpu/` — Metal backend consuming the same sampling plan as CPU.

This separation keeps host plumbing out of image math and makes the core testable without AE.

## Fast path

The warp is separable: vertical guides define X remapping and horizontal guides define Y remapping.

1. Evaluate guide positions and optional wave.
2. Enforce monotonic spacing.
3. Build inverse X/Y LUTs with a monotonic linear segment walk.
4. Precompute bilinear or bicubic indices and weights.
5. Render pixels without guide searches/easing math in the hot pixel loop.

An exact identity grid bypasses sampling entirely and copies rows.

## CPU execution

- 32-bpc float path is vectorization-friendly.
- 8/16-bpc use 4-channel SIMD on Apple Silicon NEON and x86-64 SSE.
- Non-SIMD bilinear fallback uses fixed-point weights.
- macOS row scheduling uses Grand Central Dispatch; other platforms use bounded worker threads.
- AE Multi-Frame Rendering remains enabled.

## Metal execution

- AE owns input/output GPU worlds; the plugin does not round-trip 4K images through CPU memory.
- GPU rendering is advertised only for Metal 32-bpc Smart Render.
- CPU creates the exact sampling plan; Metal consumes the same indices/weights.
- plan arrays live in reusable shared MTLBuffers; Apple Silicon can access them without a separate staging copy.
- per-render-thread Rust scratch and an MFR-safe Metal buffer pool remove steady-state plan allocations.
- one compute dispatch performs the image warp.

## Quality policy

- `Preview (Bilinear)` is explicitly a preview choice.
- `Final (Bicubic)` is the default.
- CPU and Metal Final use the same Catmull-Rom sample indices/weights.
- timing targets never authorize lowering Final quality.

## Release gate

A user `.plugin` is packaged only after sanitizer, strict-warning, sampling-plan parity, real Metal image parity,
real 4K Metal benchmark and macOS Rust host compilation all pass. After packaging, After Effects load/runtime
smoke remains the final external-host gate.
