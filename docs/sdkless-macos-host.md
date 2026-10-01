# SDK-less macOS host

The macOS development build uses pinned `after-effects 0.4.0`, its generated AE
bindings, and `pipl 0.1.1`. Xcode Command Line Tools and Rust 1.85+ are required.
The agreed physical validation scope is Apple Silicon / After Effects 2025 25.6.
Other build targets do not inherit this runtime acceptance.

## Build and validation

```bash
./tools/build_macos_sdkless.command
```

This builds only. It runs the complete 20-stage physical-Mac preflight, seals the
ad-hoc signed development bundle and verifies an actual extraction of the branded
ZIP. It never installs. `tools/install_macos.command` inspects a candidate;
replacement requires the separately authorized guarded transaction. Public
release still requires its signing/notarization/Gatekeeper gates.

Default output: `dist/mac/ElasticGrid.plugin`; user-facing sealed copy:
`dist/mac/delivery/FSTR Stretch.plugin.zip`. Packaging/signature success is not
actual After Effects runtime acceptance.

## Optional observer and cache identity

`render-diagnostics` is absent from default features. It observes existing
callbacks; logs are diagnostic and perturb timings. Active Cargo features enter
Build Identity, so ordinary and instrumented packages have distinct provenance.
See [observation design](performance-observation-design.md).

Performance development retains product version 0.9.3 and uses AE effect Develop
build 2; the optional observer uses build 3. Accepted and initial performance
artifacts used build 1. The existing PiPL property drives both resource `eVER`
and the SDK wrapper's GlobalSetup `my_version`, checked against the pinned crate
sources. Distinct versions isolate cached effect frames without deleting user
cache data. Actual target callbacks/cache behavior still require observation.
