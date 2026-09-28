# ElasticGrid FX — AE Hot Loader adapter

Branch: `feature/ae-hot-loader-shell`.

This branch adapts ElasticGrid FX to the stable-shell architecture used by AE Hot Loader. It does not change production `main`.

## Host-visible identity

The After Effects identity is preserved:

- name: `ElasticGrid FX`;
- category: `ElasticGrid FX`;
- match name: `com.elasticgrid.fx.warp`;
- effect API: `13.28`;
- existing parameter IDs remain frozen;
- GridState wire format/version remains frozen.

PiPL metadata and shell registration metadata are compared in CI.

## Bundle layout

```
ElasticGrid.plugin
└── Contents
    ├── MacOS/ElasticGrid
    ├── Frameworks/libelasticgrid_impl.dylib
    └── Resources/ElasticGrid.rsrc
```

`MacOS/ElasticGrid` is the stable C++ shell registered by AE at startup.

`libelasticgrid_impl.dylib` contains the Rust/C++/Metal implementation. The shell keeps the host-visible `EffectMain` stable and forwards calls to the active implementation generation.

## Hot-reload contract

Current adapter contract:

- Shell ABI: `1`
- Implementation Protocol ABI: `2`
- StateABI: `4`
- implementation key: `elasticgrid`
- pinned Rust toolchain: `1.98.1`

Required implementation exports:

- `EffectMain`
- `AEHotLoader_ImplementationABI`
- `AEHotLoader_ImplementationStateABI`
- `AEHotLoader_ImplementationKey`
- `AEHotLoader_ImplementationLabel`
- `AEHotLoader_ImplementationRuntimeABI`
- `AEHotLoader_SetGeneration`

The Runtime ABI freezes the Rust/compiler/dependency family for a running AE session. The bundled implementation establishes that baseline; an external candidate cannot define it.

## Generation-safe persistent state

The shell assigns a content-fingerprint generation to every accepted dylib before publishing its `EffectMain`.

ElasticGrid currently generation-tags:

- Metal GPU state;
- SmartFX pre-render state.

GPU state also stores the destroy-function pointer from the implementation generation that created the native Metal resource. Old dylibs remain loaded, so stale resources can be destroyed by their creator generation rather than by incompatible newer code.

State-contract CI freezes:

- Params names/order;
- GridState wire version;
- empty plugin global/sequence state contract;
- SmartFX pre-render payload layout;
- MetalGpuData layout;
- Protocol ABI and StateABI parity between shell and implementation.

Any intentional state/schema change requires an explicit StateABI bump and verifier update.

## MFR / reload synchronization

Concurrent MFR calls are allowed.

Reload is fail-fast:

- if EffectMain calls are in flight, reload returns busy/retry;
- AE's main thread is never blocked waiting for a render;
- old/new generations are not allowed to overlap on the same shell publication point.

## Reload workflow

First installation of the shell requires one normal AE restart.

### Source workflow

```bash
zsh tools/stage_hot_reload_macos.command my-build-label
```

The source staging script:

- validates shell/PiPL metadata;
- validates the frozen state contract;
- pins Rust `1.98.1`;
- pins macOS deployment target `11.0`;
- supports both rustup-managed and standalone pinned Cargo;
- signs and atomically stages `current.dylib`.

### Packaged test kit

The CI artifact contains:

- `ElasticGrid.plugin`
- `ElasticGridImpl-candidate.dylib`
- `INSTALL_HOT_LOADER.command`
- `STAGE_CANDIDATE.command`

The installer refuses ambiguous duplicate copies, backs up the existing bundle, verifies signature/arm64, and restores the previous installation if replacement fails.

Then use:

`Window → AE Hot Loader → Reload Plugins`

Removing the staged candidate is an explicit rollback request; Reload returns to the bundled default implementation.

## Failure behavior

The previous implementation remains active when:

- `dlopen` fails;
- required exports are missing;
- Protocol ABI differs;
- StateABI differs;
- Runtime ABI differs;
- implementation key is not `elasticgrid`;
- ABI strings are malformed;
- effect is currently busy rendering;
- the per-process generation limit is reached.

## CI status

Current hardened checkpoints:

- Hot Loader Shell CI **#57 — SUCCESS**
- full project CI **#135 — SUCCESS**

Coverage includes:

- PiPL ↔ shell metadata parity;
- state-contract verifier;
- state behavior/unit tests;
- default → candidate → unchanged → bundled rollback;
- deployment target check;
- dylib dependency check;
- GCC / Clang;
- ASan / UBSan;
- TSan;
- static analysis.

## Remaining live AE gate

Before merge:

1. existing projects still resolve `com.elasticgrid.fx.warp`;
2. GridState and parameter IDs survive save/reopen;
3. custom viewer UI works;
4. CPU and Smart Render work;
5. MFR works under real AE scheduling;
6. Metal setup/render/setdown works;
7. reload during active render returns retry, not hang;
8. reload between SmartPreRender/SmartRender never consumes stale generation data;
9. existing instances render after A→B→C swaps;
10. removing the candidate rolls back to bundled default;
11. repeated reload/noop cycles remain stable.

No merge to `main` before this live gate passes.
