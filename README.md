# ElasticGrid FX

After Effects separable grid-warp effect: editable column/row guides, not a freeform 2D mesh. The implementation uses a C++ render core and a Rust AE host.

## Current status — 2026-09-28

**Development / final validation, NOT approved for release or user binary delivery.** Plugin metadata remains 0.9.0. Historical v1.0 freeze reports below describe earlier source states, not certification of the current branch.

Work continues on `fix/final-validation`, based on the latest SmartFX parameter-snapshot source `b51b95407a022172fe739996fc4a65254418d38b`; `main` is not changed by this work.

- Render fix `53c2a299265caf3e653a766eaf0f6e03b728a621`: exact uniform-grid pixels for all easing values; precise cropped/GPU sampling plans; overflow-safe ROI origin addition. [Details and local results](docs/identity-fix-2026-09-28.md).
- Test-safety fix `0882cdf0fc7ff64a273232f1d6b7097a1d995db6`: refusing a roundtrip test no longer unconditionally closes a user project; ownership-guarded cleanup, fresh workspace and bounded AppleScript wait. [Scope and limitations](docs/project-test-safety-2026-09-28.md).
- Confirmed local source checks: GCC and Clang Release 10/10 each; ASan/UBSan/LSan 10/10; TSan 2/2; 11 script control-flow tests. These are not real AE/Metal integration results.

[Current release blockers and next steps](docs/current-status.md) are authoritative for this branch. Use actual commit-specific Actions results; do not infer PASS from the presence of a test script or an old artifact.

## Architecture and quality

The existing source implements guide editing/keyframes, elasticity/falloff/spacing, waves, CPU 8/16/32-bpc rendering, Clamp/Wrap/Mirror and a Metal 32-bpc path. Supported-host claims require the corresponding runtime evidence. Final quality remains Catmull-Rom Bicubic; it is not replaced by Bilinear for speed. Real Metal parity thresholds remain max absolute error 2.5e-5 and RMS 3.0e-6.

See [architecture](docs/architecture.md), [project compatibility](docs/project-compatibility-v0.9.md), [GPU planning](docs/gpu-plan.md) and [third-party notices](THIRD_PARTY_NOTICES.md).

## Developer verification

Portable core:

```sh
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release -DELASTICGRID_BUILD_BENCH=OFF
cmake --build build --parallel 2
ctest --test-dir build --output-on-failure
node tests/test_ae_project_safety.js
```

The Node test mocks host calls and cannot replace AE testing. macOS development builds use `tools/build_macos_sdkless.command`; hosted macOS CI compiles Metal but has no hardware/runtime approval. Do not install test candidates into a working AE environment before the release requirements are satisfied.

Final-validation CI stores a Git source bundle, commit identity and JUnit/log artifacts. It does not publish a release. Source and test artifacts have finite retention; promote required release evidence to durable storage before expiration.

## Development rules and historical records

Read [DEVELOPMENT_RULES.md](DEVELOPMENT_RULES.md) before each significant stage. The user's authoritative rules apply in full.

Earlier reports remain available in `docs/final-validation-v1.0.md`, `docs/code-freeze-v1.0.md`, `docs/final-regression-v1.0.md`, and `CHANGELOG.md`. Their hashes/results are historical and must not be represented as verification of a later build.
