# FSTR Stretch

Native After Effects grid deformation effect, listed under **FSTR Effects**.
Repository/internal name: ElasticGridFX. C++ CPU renderer with a Rust AE host.

## Current release: 0.9.3-perf.1

[Download the macOS Apple Silicon release](https://github.com/ios3kov/ElasticGridFX/releases/tag/v0.9.3-perf.1) ·
[User guide / руководство](docs/USER_GUIDE.md) ·
[Release evidence](docs/release-0.9.3-perf.1.md)

Published as Latest Release on 2026-10-02. The package revision is **0.9.3-perf.1**;
After Effects About identifies the unchanged plugin as **0.9.3 Develop Build 2**.
The ZIP filename retains `TEST` from initial publication; assets were not renamed
or rebuilt when the release was promoted. Repository instructions below and the
user guide supersede the archived README's historical prerelease/signing-policy text.

- **Layer Plane:** grid interaction and deformation follow the layer plane,
  including ordinary native 3D text/raster layers and camera perspective.
- **Four Corners (2D only):** defines a perspective deformation region; it does
  not corner-pin the entire source image. In 3D, Layer Plane is enforced and
  corner controls are disabled; saved 2D values are preserved.
- **Grid Positions:** keyframe animation and Undo/Redo remain available.
  Columns/Rows are static topology settings for new animation.
- **Final (Bicubic):** CPU rendering with 8/16/32-bpc paths. GPU dispatch is disabled.
- **Exact plane-render optimization:** eligible geometry uses cached axis mapping
  and Bicubic rows without reducing quality or changing saved parameter contracts.

Verified runtime scope: AE 2025 **25.6x101**, macOS **26.6.2**, Apple Silicon
(M1 Pro / 16 GiB in measured runs), square pixels and ordinary flat text/raster
layers. Other AE/macOS versions, Windows/Intel, per-character 3D and broad HDR/OCIO
configurations are not certified. Clean-environment installation and legacy animated
Columns/Rows acceptance are **USER-REPORTED**, with environment details unspecified.

The bundle is ad-hoc signed. Developer ID and notarization are not prerequisites
under the adopted AE-Development-Rules **6.0.0**. Browser-downloaded quarantined
installation and exact-candidate loading were checked on the existing test Mac;
this is not a promise of warning-free installation on every Mac.

## Performance results

Five matched AE Full/Final/32-bpc, 1080p, 60-frame PNG exports with MFR requested
OFF reduced median total time from **65.870721 to 48.766815 seconds**
(**25.97% less**, 1.3507× ratio of medians). This includes startup and PNG export.
All 360 paired measured/warmup outputs decode exactly. RAM Preview completed
60/60 frames and played at the composition's 30 fps; observed cache-fill bounds
are documented separately from exact first-playable latency.

The user accepted the speed scope. No further timing series is planned.
A standalone native speedup is not an AE/Preview speedup guarantee.
[Comparison table and measurement limits](docs/retrospective-0.9.3-perf.1.md#performance-comparison).

A rare MFR control abort did not recur in the six requested retries; issue
[#21](https://github.com/ios3kov/ElasticGridFX/issues/21) was closed as not reproduced.
Its cause remains unknown and no crash fix or general MFR certification is claimed.
Persistent grid display while the effect/layer is unselected remains deferred.

## Artifact identity

- Plugin source: `f611312bd7b76ebe5bc5f2bd8b48b44f50c0c761`
- Build ID: `EGFX-6147dc406abc596e7f2d1b60`
- Public outer ZIP SHA-256: `a563f8e14961e19ee0740d5eb063c89e4bbec830ac1053d23fa09de02f2a6d14`
- Inner plugin ZIP SHA-256: `b67947316a1b871050a7cb45b40502f7d3550d1074734e0547e3954fa0b3a25a`

The release contains the tested archive, not a rebuilt documentation checkpoint.
The root `SHA256SUMS.txt` covers tracked repository files except itself; it is
**not** the release archive manifest. Verify it from the repository root with
`shasum -a 256 -c SHA256SUMS.txt`. Regenerate it after tracked-file changes,
excluding itself, using sorted repository-relative paths.

## Development and evidence

- [Current status and scope](docs/current-status.md)
- [Performance/release technical retrospective](docs/retrospective-0.9.3-perf.1.md)
- [Reusable AE engineering know-how](docs/AE_ENGINEERING_KNOWHOW.md)
- [Historical 0.9.3 development retrospective](docs/retrospective-0.9.3.md)
- [Ten-stage map](docs/development-stages.md)
- [Final gate: current verdict and historical evidence](docs/stage10-final-gate-2026-09-30.md)
- [Architecture](docs/architecture.md) and [approved plane requirements](docs/perspective-plane-plan.md)
- [Development rules](DEVELOPMENT_RULES.md)

Dated reports and candidate-specific diagnostics are retained for traceability.
Their old PASS/BLOCKED states describe those checkpoints, not the current release.

## Developer verification

```sh
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release -DELASTICGRID_BUILD_BENCH=OFF
cmake --build build --parallel 2
ctest --test-dir build --output-on-failure
python3 -m unittest discover -s tests -p 'test_*.py' -v
node tests/test_ae_project_safety.js
node tests/test_smoke_safety.js
```

Portable checks do not replace real AE verification. Hosted Metal tests can be
compile-only. Preserve artifact identity, user projects and rollback backups;
build scripts do not implicitly authorize installation or release.

### Plane-specific native benchmark

```sh
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release -DELASTICGRID_BUILD_BENCH=ON
cmake --build build --parallel 2 --target elasticgrid_plane_bench
./build/elasticgrid_plane_bench 1920 1080 5 32 region dense
```

Modes: `region`, `layer`, `perspective`, `identity`; storage: `dense` or `sparse`.
The JSON reports cold and warmed native frame times at Final Bicubic with abort
polling. An optional filename writes exact pixel/padding bytes for comparison. A following
`nonfinite` argument injects exceptional 32-bpc pixels for byte-parity checks.
This is a standalone plane-bridge benchmark, not After Effects or RAM Preview.

### Controlled target-AE performance fixture

`tools/perf_fixture_runner.py` defaults to verified straight RGBA16 output and
records the owned project's unmanaged color state in fixture schema 2. It refuses
a template whose actual settings have the wrong depth, alpha, matting or geometry.
`--output-precision 8` is retained for legacy dithering research, not exact-fidelity
acceptance. Schema-1 observations remain historical; their unknown output depth
is not upgraded retroactively. PNG is not a float/HDR oracle.
