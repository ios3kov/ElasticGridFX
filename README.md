# ElasticGrid FX

Native After Effects separable column/row grid warp, with C++ rendering and a
Rust AE host. Not a freeform 2D mesh. Metadata remains **0.9.0 development**.

## Current status — 2026-09-28

**NOT approved for release or user binary delivery.** Work stays on
`fix/final-validation` / draft PR #5; `main` is unchanged.

Latest stage: read-only installation inspection, explicit create-only test
installation, and a seven-frame patterned smoke with five decoded-pixel checks.
Local verification: Python 53/53, smoke control-flow 14/14, prior roundtrip
control-flow 11/11, strict C++ Release 10/10. Synthetic/mocked tests are not AE
integration tests. Exact-head Linux/macOS CI results are recorded in PR checkpoints.

Earlier fixes preserve uniform-grid pixels, guard roundtrip project ownership,
correct host parameter metadata without changing saved IDs, and generate
commit/source/target-aware Build ID plus signed-payload/ZIP manifests. Baseline
37bc8bc passed its macOS source/package gate; this is historical evidence only,
not verification of a later package or actual AE loading.

[Current blockers](docs/current-status.md) and
[runtime tooling details](docs/runtime-acceptance-2026-09-28.md) distinguish
implemented behavior, verified scope and missing host acceptance.

## Verification and installation

```sh
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release -DELASTICGRID_BUILD_BENCH=OFF
cmake --build build --parallel 2
ctest --test-dir build --output-on-failure
python3 -m unittest discover -s tests -p 'test_*.py' -v
node tests/test_ae_project_safety.js
node tests/test_smoke_safety.js
```

`tools/build_macos_sdkless.command` performs the macOS preflight and creates the
signed candidate, ZIP and manifest. It **does not install**. The legacy
BUILD_AND_INSTALL_MAC launcher is now build-only; --install is rejected.

`tools/install_macos.command` defaults to read-only inspection. --help documents
explicit test installation into an authorized existing scope. Different existing
builds are preserved/refused; no automatic upgrade, removal, sudo or process kill.

`tools/ae_smoke_test_macos.command` defaults to preparation only (NOT RUN).
Execution requires explicit test authorization, an exact AE app and installed
bundle. It captures bypass/identity/static-wave/animation/reset images. Missing,
stale, blank, pass-through or frozen-animation results cannot pass its checks.
Image PASS does not establish loaded identity, HDR/GPU correctness or release
readiness; the runner keeps the full gate BLOCKED pending those observations.

## Quality and engineering records

Final quality remains Catmull-Rom Bicubic, not a lower-quality speed substitute.
CPU paths cover 8/16/32 bpc; real Metal parity thresholds remain max absolute
2.5e-5 and RMS 3.0e-6. Current SmartFX uses full-source checkout and disables GPU
dispatch pending real target-host verification; compiling Metal does not test it.

Read [DEVELOPMENT_RULES.md](DEVELOPMENT_RULES.md) before significant stages.
Architecture, compatibility, third-party notices and dated reports remain in
`docs/` and `THIRD_PARTY_NOTICES.md`. Historical freeze/PASS records do not certify
new artifacts. CI evidence has finite retention and must be preserved for release.
