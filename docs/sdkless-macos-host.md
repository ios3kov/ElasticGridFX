# SDK-less macOS host (v0.6)

The test build does not require the Adobe After Effects C++ SDK, an Adobe Developer account, or CMake.
The host uses the open-source `after-effects` Rust crate (pre-generated AE bindings) and `pipl`.

## Requirements

- macOS (Apple Silicon is the first runtime target)
- After Effects installed
- Xcode Command Line Tools
- Internet access on the first Cargo build if Rust/crates are not cached

## Mandatory preflight

Before Cargo/plugin packaging, the build script runs:

- core + bridge ASan/UBSan;
- CPU/GPU plan parity under sanitizers;
- strict Release warning build;
- 1080p/4K CPU performance smoke;
- identity fast-path benchmark;
- Final bicubic CPU smoke;
- real Metal CPU/GPU image-parity test;
- real 4K Metal benchmark;
- Rust AE-host compile/tests.

Standalone command:

```bash
./tools/preflight_macos.command
```

Full build/install:

```bash
./tools/build_macos_sdkless.command --install
```

Output: `dist/mac/ElasticGrid.plugin`.

## Release gate

A `.plugin` is not packaged until every preflight step passes on macOS. After that, the remaining external-host gate is
the actual After Effects load/runtime smoke test on the user's Mac.
