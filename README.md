> **Development branch (PR #10):** Columns/Rows are now static setup values;
> Grid Positions remains animatable. The next test package installs **FSTR Stretch.plugin**.
> First-add/deformation, restart, preview and cancellation were user-confirmed on
> the earlier 94d6706 candidate, not this new package. New-candidate AE acceptance
> and old animated-count projects remain unverified. No merge/release has occurred.
> [Current status](docs/current-status.md) · [Install/rollback and packaging](docs/branded-package-2026-09-30.md).

# FSTR Stretch

Native After Effects grid deformation effect, listed under **FSTR Effects**.
Repository/internal name: ElasticGridFX. C++ CPU renderer with a Rust AE host.

## Current release: 0.9.1

[Download the published macOS Apple Silicon release](https://github.com/ios3kov/ElasticGridFX/releases/tag/v0.9.1).
See [release evidence and installation/rollback](docs/release-0.9.1.md).

- **Layer Plane:** grid interaction and deformation follow the layer plane,
  including ordinary native 3D text/raster layers and camera perspective.
- **Four Corners (2D only):** defines a perspective deformation region; it does
  not corner-pin the entire source image. In 3D, Layer Plane is enforced and
  corner controls are disabled; saved 2D values are preserved.
- **Grid Positions:** keyframe animation and Undo/Redo remain available.
- **Final (Bicubic):** CPU rendering with 8/16/32-bpc paths. GPU dispatch is disabled.
- 0.9.1 fixes live 3D panel refresh and undeformed native-text perimeter fringes.

Verified target: AE 2025 **25.6.0**, macOS Apple Silicon, square pixels and
ordinary flat text/raster layers. This is not Windows/Intel, other AE versions,
per-character 3D or universal HDR/OCIO certification. The bundle is ad-hoc signed,
not Developer ID signed or notarized; do not bypass macOS security protections.

Publication does not mean every development gate is closed. Separate target-host
RAM Preview and render-cancellation evidence remains outstanding. Performance
optimization was skipped by the user, not passed. Persistent grid display while
the effect/layer is unselected is deferred.

## Artifact identity

- Plugin source: `e1848d5348c8059c0307c5d8c1241163ffd92ab1`
- Build ID: `EGFX-879b31e5a95527a385827c24`
- Release ZIP SHA-256: `23b75997e310e0acbefcde69570e177199c23d415f67abf00ee064d28efe4d2a`

The release contains the tested archive, not a rebuilt documentation checkpoint.
The root `SHA256SUMS.txt` covers tracked repository files except itself; it is
**not** the release archive manifest. Verify it from the repository root with
`shasum -a 256 -c SHA256SUMS.txt`. Regenerate it after tracked-file changes,
excluding itself, using sorted repository-relative paths.

## Development and evidence

- [Current status and open checks](docs/current-status.md)
- [Ten-stage map](docs/development-stages.md)
- [Final gate: current verdict and historical evidence](docs/stage10-final-gate-2026-09-30.md)
- [Architecture](docs/architecture.md) and [approved plane requirements](docs/perspective-plane-plan.md)
- [Development rules](DEVELOPMENT_RULES.md)

Dated reports and candidate-specific diagnostics are retained for traceability.
Their old PASS/BLOCKED states describe those candidates, not the current release.
Do not use historical pinned installers to install 0.9.1.

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
