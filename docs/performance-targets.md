# Performance targets and current baseline

Targets are engineering acceptance criteria, not marketing claims.

## Current target-path checkpoint — 2026-10-01

Source 41283e3 accelerates the currently integrated native plane sampler. On the
physical M1 Pro / 16 GiB target, five alternating native Final comparisons give
32.0415 ms at 1080p/32 bpc and 127.145 ms at 4K/32 bpc for eligible deformed
planes. The general sparse-perspective case is 180.010 ms at 1080p. These are
full native C-bridge medians; they exclude AE checkout, conversion, export and
RAM Preview. Exact settings, samples and pixel parity are in
[the corrected comparison](performance-plane-corrected-comparison-2026-10-01.json).
They do not establish real-time host performance or the Metal targets below.

Actual target AE 25.6 / M1 Pro / 16 GiB: five matched fresh-phase ordinary
MFR-requested-OFF 1080p/32-bpc/Full/Final 60-frame aerender pairs give median total time
65.870721 -> 48.766815 s (25.97% reduction). Startup/PNG export/polling included;
no cold-cache or per-frame claim. All 360 pair/warmup outputs exact.
[Raw scoped comparison](performance-host-render-2026-10-02.json).
GUI RAM Preview plays all 60 frames at 30 fps on both versions, with cache
invalidation and cancellation verified. Cache-build/first-playable latency is
not accepted as observer-free latency from screenshot bounds. Ten same-zoom
fresh-phase UI runs now bracket median full-cache readiness at
(11.546,12.924] -> (1.753,6.007]s; four pairs improve, one overlaps. All outliers
and capture/order/cache limits remain in [interval evidence](performance-host-preview-intervals-2026-10-02.json).
Ordinary MFR-requested-ON throughput remains INCOMPLETE after a control host
crash; complete and separate diagnostic pixel scopes remain exact.
[MFR scopes](performance-host-mfr-2026-10-02.json).
Actual guide dragging is blocked by the
available custom-canvas automation. Stage 8 remains IN PROGRESS.
[Preview scope](performance-host-preview-2026-10-02.json).
Host-specific absolute targets remain open until these latency gates are measured;
the native medians do not establish them.

The older CPU path numbers below are historical and do not describe the active
plane sampler. AE GPU dispatch remains disabled; physical Metal backend tests
are separate supporting evidence.

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

- default quality is `Final (Bicubic)`;
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
