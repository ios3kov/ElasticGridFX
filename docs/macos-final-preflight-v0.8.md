# macOS final preflight — v0.8

The final test plugin is produced only after all mandatory gates pass on the target Mac.

## Mandatory gates

1. ASan + UBSan: core, bridge, GPU-plan, final-quality and MFR tests.
2. ThreadSanitizer MFR stress.
3. Strict Clang Release compile with `-Werror` and no fast-math.
4. CPU 4K/8K benchmark capture.
5. Real Metal CPU/GPU pixel parity for Bilinear/Bicubic × Clamp/Wrap/Mirror.
6. Real Metal 4K/8K benchmark using the same `eg_metal_render` entry point as After Effects.
7. Rust AE host Release tests and locked dependency build.
8. Bundle validation: PiPL, exported entry points, Info.plist, dynamic dependencies and code signature.
9. After Effects load/runtime smoke on the installed plugin.

## Stop conditions

Do not distribute or call the build ready if any of these occur:

- sanitizer or race report;
- CPU/Metal parity above max abs 2.5e-5 or RMS 3.0e-6;
- Metal shader/pipeline creation failure;
- missing `EffectMain` or `PluginDataEntryFunction2` export;
- invalid code signature or PiPL metadata;
- AE reports the plugin failed to load;
- Final mode silently falls back to a lower-quality sampler.

## Performance principle

Optimizations may change scheduling, caching, SIMD/GPU execution and memory traffic. They must not change the defined Final sampling result beyond the explicit floating-point parity tolerance.
