# ElasticGrid FX

Native After Effects separable column/row grid warp, with a C++ renderer and Rust
host. **0.9.0 development; not ready for release.** Draft PR #5 / fix/final-validation.

## Current priorities

Repair bright streaks and the reported Adjustment Layer -> ElasticGrid -> Corner
Pin black output. Then add four-corner placement of the deformation plane and
layer/camera-driven 3D perspective. Grid interaction and pixels must agree.
Render and RAM Preview must be extremely fast without hidden quality degradation.

- [Approved plane/chain requirements](docs/perspective-plane-plan.md)
- [Performance and quality contract](docs/performance-quality-contract.md)
- [Current status and remaining gates](docs/current-status.md)

## Latest source work

Added an explicit sparse CPU rendering entry: missing compact-world pixels are
transparent on a known full logical canvas, not stretched edge pixels. Regression
compares sparse storage against an explicitly zero-filled full image at 8/16/32
bpc. Dense/GPU interfaces and saved parameter schema remain unchanged.

**The new entry is not connected to the AE host yet.** Current installation
6d3b846 is unchanged. Corner Pin, actual host coordinates and clipped Grid
Positions text remain open; planned 2D/3D modes are not implemented.

Local GCC/Clang Release: 11/11 each; ASan/UBSan/LeakSanitizer: 11/11; TSan: 3/3.
[Stage acceptance](docs/sparse-render-stage-plan.md) and
[results/limitations](docs/sparse-render-results-2026-09-28.md) preserve the failing
baseline and distinguish portable checks from actual AE/Metal acceptance.
Exact-commit CI results are recorded in PR checkpoints.

## Developer verification

```sh
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release -DELASTICGRID_BUILD_BENCH=OFF
cmake --build build --parallel 2
ctest --test-dir build --output-on-failure
python3 -m unittest discover -s tests -p 'test_*.py' -v
node tests/test_ae_project_safety.js
node tests/test_smoke_safety.js
```

macOS preflight includes the new sparse regression. Hosted Metal stages may be
compile-only; actual AE GPU dispatch is disabled pending host verification.
Final quality remains Catmull-Rom Bicubic; no silent lower-quality speed fallback.

## Safe operation

Build scripts do not install implicitly. Inspection is read-only by default.
The separately authorized updater is limited to the previously approved copy and
preserves its original for rollback; it is not permission for broad replacements.
No new binary is delivered by this source stage. User receipt confirmed the prior
test installation; installed-file success is not full render or release approval.

Read [DEVELOPMENT_RULES.md](DEVELOPMENT_RULES.md) before each significant stage.
[Architecture](docs/architecture.md), historical dated evidence and third-party
notices remain available. Historical PASS reports never certify a newer artifact.
