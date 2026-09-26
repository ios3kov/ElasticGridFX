# Code audit / refactor / profiling — v0.4

Scope: clean-room C++ warp core, C ABI bridge, and macOS SDK-less host build path.
The audit does not claim a real After Effects/macOS runtime pass; that still requires the test Mac.

## High-priority findings

### Fixed

1. **8/16-bpc CPU renderer was the dominant bottleneck.**
   The old integer path converted every channel through scalar float math and `std::lround`.
   It was replaced by:
   - SIMD 4-channel sampling on Apple Silicon NEON and x86-64 SSE;
   - fixed-point bilinear fallback when SIMD is unavailable;
   - cheap bounded rounding for the remaining scalar bicubic fallback.

2. **macOS created worker threads per frame.**
   The macOS renderer now dispatches row chunks through Grand Central Dispatch, reusing the
   system worker pool and cooperating better with After Effects Multi-Frame Rendering.

3. **`-ffast-math` conflicted with defensive finite-value checks.**
   The host build no longer uses `-ffast-math`. It keeps safe optimizations
   (`-fno-math-errno`, `-ffp-contract=fast`) without assuming NaN/Inf never occur.

4. **FFI validation was too trusting.**
   Added bit-depth, row-stride, finite parameter and thread-count validation.
   Negative row stride is explicitly tested.

5. **Inverse LUT construction did a binary search for every output coordinate.**
   It now walks monotonic guide segments once per axis.

6. **Performance measurement did not include the host bridge/setup cost.**
   Added `elasticgrid_bridge_bench`, which includes grid evaluation, LUT construction,
   sampling-plan preparation and render.

### Blocker found before the first user `.plugin`

`Tension Radius`, `Falloff`, and `Elasticity Strength` are present in the v0.3 host UI,
but direct guide dragging/arbitrary grid state has not yet been wired into the host. Therefore
those controls do not yet affect output. A user test build will not be declared ready until the
Comp/Layer guide overlay + drag state is connected to the core.

## Debug / regression status

Passed in the development container with both GCC and Clang:

- Release C++ build with `-Wall -Wextra -Wpedantic`: PASS, no warnings.
- Core tests: PASS.
- Bridge tests: PASS.
- AddressSanitizer: PASS.
- UndefinedBehaviorSanitizer: PASS.
- 8/16/32-bpc identity paths: PASS.
- Bicubic identity path: PASS.
- Negative row stride: PASS.
- 1-thread vs 4-thread deterministic output: PASS.
- Invalid bit depth / undersized row stride rejection: PASS.
- NaN/Inf render parameter sanitization: PASS.

## Profiling snapshot

Linux/x86-64 development container, 3840×2160, full C bridge path. These are engineering
numbers only, not Mac/AE marketing comparisons.

| Path | v0.3 baseline | v0.4 optimized |
|---|---:|---:|
| 8-bpc bilinear | ~80 ms | ~26 ms |
| 16-bpc bilinear | ~94 ms | ~15 ms |
| 32-bpc bilinear | ~12.5 ms | ~7.5 ms |
| 8-bpc bicubic | ~182 ms | ~68 ms |
| 16-bpc bicubic | ~92–152 ms* | ~54 ms |

`*` Earlier integer bicubic measurements were noisy because the scalar rounding path dominated.

Apple Silicon has a dedicated NEON implementation; real numbers are collected by
`tools/preflight_macos.command` on the test Mac before plugin packaging.

## Release gate before user test build

1. Wire keyframeable grid state.
2. Draw guides in Comp/Layer viewer.
3. Drag guides with elastic propagation and min-spacing/no-crossing behavior.
4. Re-run macOS preflight: sanitizers + 8/16/32 benchmarks.
5. Build/sign `.plugin`.
6. Only then send the test build to the user.
