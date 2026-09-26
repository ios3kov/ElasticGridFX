# macOS first test

The first user test build is intentionally gated.

Before a `.plugin` is handed over:

1. Comp/Layer guide overlay works.
2. Guides drag with elastic falloff and cannot cross.
3. Grid state is keyframeable.
4. `tools/preflight_macos.command` passes on the test Mac.
5. The build script packages/signs `ElasticGrid.plugin`.

Then the install command is:

```bash
./tools/build_macos_sdkless.command --install
```

Adobe After Effects SDK and CMake are not required.
