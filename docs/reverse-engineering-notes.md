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


## 2026-09-28 — visualization and spatial-contract decomposition

Further static decomposition of the supplied `GridWarp.aex` confirms that **Visualization is part of rendered pixels**, not only an AE custom-UI overlay.

The SmartRender path conditionally calls the visualization compositor at image VA `0x180016c70` after the main warp when **Enable Visualization** is non-zero.

Recovered call contract:

1. output world
2. pixel format
3. `PF_InData::width`
4. `PF_InData::height`
5. `PF_InData::output_origin_x`
6. `PF_InData::output_origin_y`
7. evaluated column-line array
8. column-line count
9. evaluated row-line array
10. row-line count
11. Column Stroke Color
12. Row Stroke Color
13. Stroke Width
14. Opacity normalized to 0..1

The `PF_InData` offsets were cross-checked against the generated ABI contract in AEXCompat:
- width: 252 / `0xfc`
- height: 256 / `0x100`
- output_origin_x: 276 / `0x114`
- output_origin_y: 280 / `0x118`

This resolves a previous uncertainty: the original compositor uses the host's **full source width/height and output origin**, rather than an inferred local ROI coordinate system.

### Line rasterization recovered from `0x180016c70`

For each evaluated guide, the original computes the line center in output-buffer coordinates from the full layer extent plus `output_origin`, then evaluates per-pixel antialias coverage:

`coverage = clamp(half_width + 0.5 - abs(pixel_center - guide_center), 0, 1)`

Effective source alpha is:

`coverage * (Opacity / 100) * (stroke_color.alpha / 255)`

The compositor then performs source-over alpha blending in unpremultiplied color space. The PF color byte order observed is **A,R,G,B**.

The same routine is used for vertical column guides and horizontal row guides. It supports the output pixel-format paths used by the plug-in (8/16/32f).

Constants confirmed in the binary:
- antialias pixel-center term: `0.5`
- wave displacement scale: `0.4`
- clamp upper bound: `1.0`
- epsilon: `1e-6`
- color normalization: `255.0`

### Axis/wave behavior recovered

The axis evaluator at `0x18000f310` and spacing projector at `0x18000f1b0` confirm:
- N columns/rows means N internal guides and N+2 stored boundary-inclusive positions;
- wave displacement uses `amplitude_fraction * 0.4 * uniform_segment_spacing`;
- phase is degrees (`pi/180`);
- frequency/speed are evaluated in cycles (`2*pi`);
- minimum spacing is clamped to `max(1e-6, min(requested, 0.5/(N+1)))`;
- the final axis is projected forward/backward to remain monotonic.

### Open parity gap

The current ElasticGrid implementation exposes Visualization parameters and checks them out, but does **not yet composite those guide lines into rendered pixels**. Therefore it cannot be called exact GridWarp behavioral parity yet.

This gap must be closed and regression-tested automatically before another user live test.


## 2026-09-28 — exact defaults, SmartPreRender and drag-event contract

Further static decomposition closed three remaining host-contract uncertainties.

### Visualization defaults

The two color parameter initializers in ParamsSetup write the literal PF color bytes:

`FF 00 30 FF`

With the confirmed PF byte order A,R,G,B this is exactly:

- Alpha = 255
- Red = 0
- Green = 48
- Blue = 255

So both **Column Stroke Color** and **Row Stroke Color** default to `#0030FF` at full alpha. The existing ElasticGrid defaults are correct and are no longer inferred.

The same ParamsSetup region reconfirms:
- Stroke Width: min 0.5, max 100, default 2.0
- Opacity: min 0, max 100, default 100

### SmartPreRender request

The original SmartPreRender at `0x18000ed01` copies the host output request and then overrides only the source checkout geometry/zero-alpha policy:

- request.left = 0
- request.top = 0
- request.right = `PF_InData.width`
- request.bottom = `PF_InData.height`
- `preserve_rgb_of_zero_alpha = TRUE`

It leaves the request field/channel mask inherited from the host. It then checks out input layer 0 at the current render time.

The returned checkout rectangles are folded into the pre-render output:
- result rect unions the checkout result, then is intersected with the original output request;
- max-result rect unions the checkout max-result and the full source rect.

This matches Adobe distortion-effect guidance: a spatial sampler that can pull pixels from elsewhere requests the complete source frame and preserves RGB under zero alpha.

Important correction: `PF_InData.width/height` are full source-layer dimensions and are **not downsampled**. The previous ElasticGrid pre-render path multiplied them by downsample factors, which does not match the original or Adobe's contract.

### Custom drag event

The original event dispatcher confirms:
- click with `send_drag` transitions to the drag handler;
- drag mutation writes the updated arbitrary Grid Positions state back to the parameter;
- the returned event flags are exactly `0x9`.

From Adobe's event constants, `0x9` is:

`PF_EO_HANDLED_EVENT | PF_EO_UPDATE_NOW`

It does **not** include `PF_EO_ALWAYS_UPDATE`.

The original drag handler does not acquire the App suite or call `PF_InvalidateRect` in this path. Its render invalidation is driven by changing the arbitrary parameter value plus the event flags. ElasticGrid should mirror that contract instead of keeping extra update behavior unless a separately measured host workaround proves necessary.
