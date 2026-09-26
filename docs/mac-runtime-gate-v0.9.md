# ElasticGrid FX v0.9 — target Mac runtime gate

This is the last release-candidate gate. It must run on the target Mac with After Effects installed.

## 1. Build / Metal preflight

Double-click `BUILD_AND_INSTALL_MAC.command`.

The build is blocked unless all of these pass:

- strict C++ compile (`-Werror`);
- ASan + UBSan;
- ThreadSanitizer MFR stress;
- Final Bicubic reference tests;
- production Metal lifecycle stress;
- CPU ↔ Metal image parity;
- production Metal 4K and 8K benchmarks;
- Rust/AE host tests;
- bundle/PiPL/entrypoint/signature verification.

Report: `dist/mac/preflight-report.txt`.

## 2. Real After Effects runtime smoke

After install, fully quit After Effects so the new plug-in cannot be hidden by the old process cache.
Open After Effects into a **new empty unsaved project with 0 items** and run:

`tools/ae_runtime_check_macos.command --full`

The smoke script intentionally refuses to touch an existing or dirty project.

The automated gate verifies:

- After Effects enumerates match name `com.elasticgrid.fx.warp`;
- the effect can be applied to a layer;
- required parameters are available;
- 32-bpc `Final (Bicubic)` with Wave + Mirror performs a real frame render;
- the output frame is written successfully;
- a temporary `.aep` is saved, closed and reopened;
- ordinary parameter values and Wave keyframes survive the project roundtrip;
- the reopened project renders a 32-bpc frame successfully;
- the installed bundle/signature and plug-in loading logs are inspected.

Expected automated final lines include:

`After Effects runtime smoke: PASS`

`After Effects project roundtrip: PASS`

`Automated After Effects runtime gate: PASS`

## 3. Manual visual/project gate

The remaining checks cannot be made trustworthy through ExtendScript alone. Before calling the candidate release-ready, manually verify in After Effects:

- guide drag in Comp and Layer Viewer;
- 1/2 and 1/4 resolution preview alignment;
- non-square pixel aspect ratio;
- precomp and an upstream buffer-expanding effect;
- Grid Positions keyframes + interpolation;
- Undo / Redo after guide drag;
- save `.aep`, close, reopen, and confirm the grid/keyframes/render are identical.

No quality/performance acceptance may bypass CPU ↔ Metal parity or Final Bicubic checks.
