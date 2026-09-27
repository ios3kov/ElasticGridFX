# ElasticGrid FX — AE Hot Loader shell adapter

Branch: `feature/hot-loader-shell`

This integration keeps the public After Effects identity unchanged:

- Name: `ElasticGrid FX`
- Match name: `com.elasticgrid.fx.warp`
- Category: `ElasticGrid FX`
- PiPL and parameter IDs remain owned by the existing Rust host build.

## Layout

```
ElasticGrid.plugin
└── Contents
    ├── MacOS/ElasticGrid                 # stable C++ shell
    ├── Frameworks/ElasticGridImpl.dylib # default Rust implementation
    └── Resources/ElasticGrid.rsrc       # existing PiPL
```

The stable shell owns `PluginDataEntryFunction2` and `EffectMain` at the bundle boundary.

The Rust host dylib is unchanged as the effect implementation and still owns:
- parameters;
- arbitrary data;
- custom UI;
- CPU render;
- Smart Render;
- Metal/GPU setup and render;
- sequence/global data.

The shell forwards every AE command to the active implementation `EffectMain`.

## Hot reload

External implementation path:

`~/Library/Application Support/AE Hot Loader/implementations/elasticgrid/current.dylib`

Build and stage a new implementation while AE remains open:

```bash
zsh tools/stage_hot_reload_macos.command
```

Then click **Reload Plugins** in `Window → AE Hot Loader`.

The shell loads the candidate from a unique runtime path and atomically switches its `EffectMain` pointer.

Old implementation dylibs remain loaded until AE exits. A failed load does not replace the current implementation.

## First installation

The shell itself is a new stable plug-in binary and therefore requires one normal AE restart after installation.

After that, implementation-only changes are designed to reload without restarting AE.

## Compatibility gate

Before merging this branch:

1. existing ElasticGrid projects open with the same match name;
2. parameters and GridState survive save/reopen;
3. viewer custom UI remains functional;
4. CPU/SmartFX/Metal render paths work through the shell;
5. candidate implementation can be staged and activated without AE restart;
6. an existing effect instance renders after the swap;
7. repeated reloads are stable;
8. a bad candidate leaves the previous implementation active.
