# Exact identity regression — 2026-09-28

Scope: C++ bridge/sampling plans. No AE parameter schema, Rust host ABI or deformation architecture change. Base: b51b95407a022172fe739996fc4a65254418d38b (source snapshot f35a351443e7c0173a3ace6346a502906a409611).

## Acceptance defined before implementation

Uniform guides must preserve exact 8/16/32-bpc pixels for every supported easing value, including negative/HDR float samples, padded/negative row strides and cropped output. Exported GPU sampling plans must have exact unit taps on uniform axes. A genuinely deformed axis must not be rounded into identity. Preserve Final Catmull-Rom quality, cancellation, concurrency and zero-allocation steady state.

## Baseline and change

A 1919x33 high-frequency float fixture, 7 column / 11 row guides, easing=1 and easing distance=1 reproduced unwanted pixel changes on a uniform grid. Bilinear: 50,362 differing channels, maximum absolute difference 0.00189996. Bicubic: 54,444 differing channels, maximum absolute difference 0.000926375. The new regression linked against the old implementation also fails its 16-bpc byte-equality assertion. The old 9-test suite passed, demonstrating the coverage gap.

Cause in this fixture: normalized-coordinate/Hermite float roundoff produces nonzero subpixel sampling weights. The bridge now recognizes only bit-exact uniform evaluated axes and emits exact integer/unit taps (including cropped and exported GPU plans). Full-frame identity uses the existing exact copy path for all easing values. No epsilon-based deformation approximation, lower-quality interpolation, new allocation or lock was introduced. ROI origin addition now uses int64 before float conversion to avoid signed overflow.

This reproduces a core defect; it does not establish the cause of every previously reported AE artifact or prove viewer dragging is fixed.

## Executed local checks

Linux x86_64, separate clean CMake build directories:

- GCC Release -Werror: PASS, 10/10 tests.
- Clang Release -Werror: PASS, 10/10 tests.
- Clang ASan + UBSan + LeakSanitizer: PASS, 10/10; fuzz scale 0.1, soak 3,000.
- GCC TSan: PASS, MFR and determinism, 2/2.
- Normal Release runs use full deterministic fuzz (180,000 operations) and default 50,000-cycle soak.
- Clang high-warning audit and Static Analyzer: PASS in the first run on this production-source state. A later redundant analyzer run timed out; it is not recorded as another PASS.
- Allocation/hot-path audit: PASS; zero steady-state heap allocations for CPU rendering, plan generation and ROI planning; no blocking mutex primitives found by the existing scan.
- macOS/Rust/Metal source gate: must be read from CI for the resulting commit; local Linux checks do not replace it.
- Actual AE / real Metal / installed runtime identity: BLOCKED in this environment. No plugin is approved for user delivery.

## Performance sanity (not a speed claim)

Same-machine serial alternating before/after runs, 32-bpc Final Bicubic deformed render with abort polling, 4 workers. Initial 4K medians (three short samples) were 13.7744 / 17.3696 ms; 8K medians 57.8315 / 58.0086 ms. The suspicious 4K result was rechecked with seven alternating 24-frame samples and the same four-CPU affinity: before median 14.8097 ms (13.6605–17.5365), after 14.4488 ms (13.2609–16.2482). A repeatable regression was not established; shared-host noise prevents a precise performance conclusion. Real target-Mac profiling remains required.

## Reproduction

Run CMake Release build followed by ctest --output-on-failure. The added target is elasticgrid_identity_tests / ctest elasticgrid_identity. Assertions remain enabled in Release. The macOS preflight now runs the same identity fixture under ASan/UBSan. Final-validation CI retains commit-addressed source bundles and JUnit results; archived historical reports are not evidence for a newer plugin.
