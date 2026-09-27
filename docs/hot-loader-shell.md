# ElasticGrid FX — AE Hot Loader adapter

Branch: `feature/ae-hot-loader-shell`.

This branch adapts ElasticGrid FX to the stable-shell architecture used by AE Hot Loader. It does not change the production `main` branch.

## Host-visible identity

The After Effects identity is preserved:

- name: `ElasticGrid FX`;
- category: `ElasticGrid FX`;
- match name: `com.elasticgrid.fx.warp`;
- effect API: `13.28`;
- existing parameter IDs and GridState serialization remain unchanged.

## Bundle layout

```
ElasticGrid.plugin
└── Contents
    ├── MacOS/ElasticGrid
    ├── Frameworks/libelasticgrid_impl.dylib
    └── Resources/ElasticGrid.rsrc
```

`MacOS/ElasticGrid` is the stable C++ shell registered by AE at startup.

`libelasticgrid_impl.dylib` is the existing Rust/C++/Metal implementation. The shell forwards the normal AE `EffectMain` ABI to the active implementation.

## Hot-reload ABI

The implementation exports:

- `EffectMain`;
- `AEHotLoader_ImplementationABI() = 1`;
- `AEHotLoader_ImplementationStateABI() = 1`;
- `AEHotLoader_ImplementationKey() = "elasticgrid"`;
- `AEHotLoader_ImplementationLabel()`.

The shell validates all identity/state exports before publishing a new `EffectMain` pointer.

A parameter-schema, persistent sequence/global-data, or incompatible GPU-state change must increment the state ABI and requires a rebuilt shell plus one AE restart.

## Reload workflow

First install of the shell requires one normal AE restart.

For implementation-only changes while AE remains open:

```bash
zsh tools/stage_hot_reload_macos.command my-build-label
```

The candidate is staged to:

`~/Library/Application Support/AE Hot Loader/implementations/elasticgrid/current.dylib`

Then use:

`Window → AE Hot Loader → Reload Plugins`

The shell copies the candidate to a unique runtime path, validates it, and atomically switches subsequent calls to the new implementation. Old dylib images remain loaded until AE exits so in-flight calls cannot jump into unloaded code.

## Failure behavior

A candidate is rejected before the active pointer changes when:

- `dlopen` fails;
- `EffectMain` is missing;
- protocol ABI differs;
- state ABI differs;
- implementation key is not `elasticgrid`.

The previous implementation remains active.

## Merge gate

Before this adapter can merge:

1. original project files still resolve `com.elasticgrid.fx.warp`;
2. GridState and parameter IDs survive save/reopen;
3. custom viewer UI works;
4. CPU, Smart Render, MFR and Metal paths work through the shell;
5. an implementation update activates without AE restart;
6. an existing effect instance renders after the swap;
7. repeated reload/noop cycles are stable;
8. an invalid candidate leaves the previous implementation active.
