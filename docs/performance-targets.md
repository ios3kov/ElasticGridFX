# Performance targets and current baseline

Targets are engineering acceptance criteria, not marketing claims.

## Historical CPU development baseline

Linux/x86-64 development container, 3840×2160, full C bridge including grid/wave evaluation,
LUT/sampling-plan preparation and pixel render. Repeated-run approximate values:

- 8-bpc bilinear deformed: ~26.4 ms/frame
- 16-bpc bilinear deformed: ~16.3 ms/frame
- 32-bpc bilinear deformed: ~7.8 ms/frame
- 8-bpc bicubic deformed: ~69 ms/frame
- 16-bpc bicubic deformed: ~57 ms/frame
- 32-bpc bicubic deformed: ~32 ms/frame

Identity fast path at 4K:

- 8-bpc: ~1.5-1.7 ms/frame
- 16-bpc: ~2.8-3.1 ms/frame
- 32-bpc: ~6.2-7.1 ms/frame

Apple Silicon uses a separate NEON/GCD CPU path. Real Mac numbers must come from macOS preflight.

## Interactive targets

- 1080p CPU preview: direct guide dragging remains responsive.
- 4K Metal Bilinear Preview: < 12 ms/frame end-to-end backend target.
- no per-pixel allocation;
- no steady-state GPU-plan or Metal plan-buffer allocation after warmup.

## Final render targets

- original-parity default quality is `Draft (Bilinear)`; `Better (Bicubic)` remains the higher-quality user option;
- 4K Metal Final: < 20 ms/frame end-to-end backend target on supported Apple Silicon where practical;
- same sampling indices/weights as CPU final path;
- Multi-Frame Rendering remains enabled and deterministic.

The target is not met by reducing quality. If hardware cannot meet a timing target, quality wins.

## Correctness gates

- no guide crossing at valid settings;
- identity grid pixel-identical in the same integer format;
- deterministic output across render thread partitions;
- negative CPU row strides supported;
- malformed row strides / unsupported bit depths rejected;
- 8/16/32-bpc regression coverage;
- ASan and UBSan clean;
- strict Clang warning build clean;
- CPU/GPU sampling-plan parity;
- real Metal image parity within documented floating-point tolerance;
- arbitrary grid state remains corruption-safe and keyframeable.

## Benchmark policy

Any comparison with GridWarp must use the same resolution, quality, bit depth, cache state,
AE version and hardware. Record exact OS/CPU/GPU and project settings.


## Current parity benchmark evidence — 2026-09-28

GitHub-hosted Linux runner, Release/Clang, current visualization-capable CPU bridge:

- 1080p 8-bpc Bilinear identity: ~0.174 ms/frame;
- 1080p 8-bpc Bilinear deformed: ~2.50 ms/frame;
- 1080p 8-bpc Bilinear deformed + rendered Visualization: ~2.67 ms/frame;
- 1080p 8-bpc Bicubic deformed: ~3.34 ms/frame;
- 4K 32-bpc Bicubic deformed with AE-style abort polling: ~11.9 ms/frame;
- 4K GPU-plan generation: ~0.09 ms/frame.

These are CI regression measurements, not product/Mac claims. Visualization adds roughly 0.18 ms/frame in that specific 1080p CI case. Real Apple Silicon / After Effects profiling remains mandatory before a performance claim.
