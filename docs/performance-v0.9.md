# ElasticGrid FX v0.9 — Performance Baseline

This is a regression baseline for the final hardening branch. It is not a promise of target-Mac/After Effects performance: real Metal and AE timings are recorded by `tools/preflight_macos.command` on the target Mac.

## Portable CPU baseline

Release Clang 17, auto thread count, deformed grid, production-quality sampling. The `abort-poll` runs use the same bounded host-cancellation polling mode as the After Effects CPU path.

| Frame | Depth | Quality | Host abort polling | Time |
|---|---:|---|---|---:|
| 3840×2160 | 8 bpc | Bicubic | on | ~24.7 ms |
| 3840×2160 | 16 bpc | Bicubic | on | ~19.7 ms |
| 3840×2160 | 32 bpc | Bicubic | on | ~17.9 ms |
| 7680×4320 | 32 bpc | Bicubic | on | ~68.8 ms |
| 3840×2160 | 32 bpc | Bilinear | off baseline | ~9.3 ms |
| 7680×4320 | 32 bpc | Bilinear | off baseline | ~39.2 ms |

The cancellation path returns to the AE calling thread every 2048 rows. Callers without a host abort callback keep the one-dispatch fast path.

## Quality constraints

Performance changes are rejected if they change Final sampling from Catmull-Rom Bicubic, enable fast-math, clip HDR float data, or exceed the CPU/Metal parity thresholds documented in the README.

## Target-Mac gates

The final preflight measures the production Metal path at 4K and 8K, then runs CPU/Metal image parity before the plug-in is built and installed.

## v1.0 hot-path allocation hardening

Steady-state fixed-geometry CPU render preparation now reuses thread-local buffers. Allocation audit measures 0 heap allocations/frame after warm-up (previously 18 for deformed Bicubic in the bridge path). GPU-plan construction measures ~0.19 ms at 4K and ~0.39 ms at 8K in the current Linux environment. Final image quality math is unchanged.
