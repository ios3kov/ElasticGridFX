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
