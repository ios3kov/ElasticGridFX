# GridWarp binary inspection notes

Source inspected: `GridWarp.aex` supplied by the user with permission from the author to reverse engineer.
SHA-256: `d26054ae75b0561a4d391a3d8866212f922239754d9cc7d658a4df4e727db91e`

The clean-room project uses the binary as a behavior/compatibility reference. Recovered machine-code routines, license code, keys and proprietary source are not copied.

## Confirmed directly from the PE binary

- PE32+ Windows x86-64 DLL / After Effects `.aex`.
- Linker family: MSVC 14.x.
- Exported host entry points include `EffectMain` and `PluginDataEntryFunction2`.
- Product strings include `GridWarp`, `NewMediaFX`, `NMFX GridWarp`.
- Description string: `Elastic Column & Row Stretcher.`
- Effect UI strings embedded in the binary include:
  - Num Columns
  - Num Rows
  - Elasticity
  - Tension Radius
  - Falloff Profile
  - Elasticity Strength
  - Min Line Spacing
  - Stretch Easing
  - Easing Distance
  - Wave Animation
  - Wave Amplitude
  - Wave Frequency
  - Wave Phase
  - Wave Speed
  - Wave Axis
  - `Both|Columns Only|Rows Only`
  - Rendering
  - `Clamp|Wrap|Mirror`
  - Render Quality
  - `Draft (Bilinear)|Better (Bicubic)`
- Drawbot / custom-UI suite names are present, consistent with on-screen controls being drawn through AE's custom UI path.
- AEGP stream/keyframe suite names are present. Presence alone does not prove a particular internal data model, so ElasticGrid does not depend on that inference.
- The binary contains and exports aescripts licensing functions. ElasticGrid does not copy or reimplement that licensing subsystem.
- No direct CUDA, OpenCL or DirectX runtime DLL imports are present in the PE import table. This proves only that those runtimes are not directly imported by this build; it does not prove every possible render path is CPU-only.

## Confirmed from the supplied installation guide

- Windows package: `GridWarp.aex`.
- macOS package: `GridWarp.plugin`.
- Documented effect menu path: `Effect > NewMediaFX > GridWarp`.
- Trial/registration is handled through aescripts licensing.

## What ElasticGrid intentionally changes

- independent data structures and algorithms;
- prepared 1D sampling tables to reduce per-pixel math;
- corruption-detected versioned guide-state serialization;
- pin/lock support in the clean-room grid solver;
- Metal-first macOS GPU plan;
- explicit MFR/thread-safety design;
- no bundled third-party licensing code.


## 2026-09-28 — SmartFX selector decomposition

The supplied binary was decomposed far enough to map the render selectors and their parameter dependencies.

Confirmed command handlers:
- `PF_Cmd_PARAMS_SETUP` -> function at image VA `0x180005380`
- `PF_Cmd_EVENT` -> function at image VA `0x180011390`
- `PF_Cmd_SMART_PRE_RENDER` -> inline handler at `0x18000ed01`
- `PF_Cmd_SMART_RENDER` -> function at image VA `0x1800066d0`

The original Smart Render handler explicitly performs host parameter checkouts before rendering and checkins afterward. The observed parameter indices are:

`4, 10, 11, 12, 15, 16, 17, 18, 19, 22, 23, 24, 25, 26, 29, 30`

Using the recovered ParamsSetup order, these correspond to:
- Grid Positions;
- Min Line Spacing;
- Stretch Easing;
- Easing Distance;
- Wave Amplitude/Frequency/Phase/Speed/Axis;
- Enable Visualization;
- Column Stroke Color;
- Row Stroke Color;
- Stroke Width;
- Opacity;
- Edge Behavior;
- Render Quality.

Notably, Tension Radius, Falloff Profile and Elasticity Strength are not Smart Render inputs; they affect guide dragging/state construction rather than pixel evaluation.

This is a materially stronger result than string-level inspection: the original does **not** rely on the normal `params[]` array during Smart Render. It checks non-layer parameters out from the host for the current render time.

### Cross-check against Adobe/open-source reference code

Adobe's SmartFX contract requires non-layer render parameters to be obtained through parameter checkout because Smart Pre-Render/Smart Render do not receive the ordinary parameter array. Adobe's ColorGrid sample follows the same pattern: it checks its arbitrary-data parameter out during Smart Render and renders from that checked value.

The current ElasticGrid live failure is consistent with violating this contract: the custom UI changed GridState, while Smart Render read the ordinary Rust parameter view instead of a render-time checkout.

The next implementation step is therefore contract-level, not another image-warp tweak:
1. Smart Render will checkout the exact render dependencies above.
2. Pixel evaluation will use only those checked values.
3. Custom UI mutation will explicitly invalidate the host view in addition to marking the arbitrary parameter changed.
4. Automated source gates will reject direct Smart Render reads from the ordinary parameter array.
