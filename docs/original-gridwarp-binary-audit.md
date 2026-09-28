# Original GridWarp binary audit

Reference supplied by the user: `GridWarp.aex`.

- PE32+ x86-64 Windows After Effects effect
- SHA-256: `d26054ae75b0561a4d391a3d8866212f922239754d9cc7d658a4df4e727db91e`
- exports: `EffectMain`, `PluginDataEntryFunction2`
- original identity: `GridWarp` / `NewMediaFX` / `NMFX GridWarp`
- PiPL effect API: 13.29

Recovered parameter order/defaults: Grid (Num Columns 1..50 default 4, Num Rows 1..50 default 4, Grid Positions); Elasticity (Tension Radius 0..20 default 3, Smoothstep|Gaussian|Linear, Strength 100%, Min Spacing 0.5%, Stretch Easing 50%, Easing Distance 25%); Wave Animation (Amplitude 0, Frequency 1, Phase 0, Speed 0, Axis Both); Visualization (Enable off, column/row stroke colors, Width 2, Opacity 100%); Rendering (Clamp, Draft/Bilinear); Reset Grid.

Recovered behavior used by this independent implementation: N columns/rows means N internal guides and stores N+2 boundary-inclusive positions; raw normalized drag target is used before monotonic/min-spacing projection; elasticity strength affects the grabbed guide and neighbors; falloff order is Smoothstep/Gaussian/Linear; Gaussian uses exp(-4*t^2); rendering is a separable column/row inverse warp; SmartFX correctness path checks out the complete source.

The previous hot-loader branch missed these later parity fixes, which is why real AE showed a moving overlay with unchanged pixels and bright ROI/seam lines.

Parity guide wire format is v3; Hot Loader StateABI is therefore 5.


## Additional recovered render behavior

The reference's Visualization group is a real output compositing feature. When enabled, it draws the evaluated column and row guides over the warped result using the configured A/R/G/B colors, Stroke Width and Opacity. It is not equivalent to the editor overlay.

Spatially, the original uses `PF_InData.width/height` and `PF_InData.output_origin_x/y` directly. Static disassembly offsets were independently matched to AEXCompat's generated AE ABI contract.

The line coverage function is an antialiased centered band:

`clamp(stroke_width/2 + 0.5 - abs(pixel_center - guide_center), 0, 1)`

followed by opacity/color-alpha scaling and source-over blending.

This behavior is now a required automatic parity test before another live AE package is issued.


## Visualization parity implementation

The independent implementation now includes the recovered rendered-Visualization stage on its CPU path. The compositor uses evaluated guide positions, full source dimensions, output origin, A/R/G/B stroke colors, Stroke Width and Opacity, with the antialias/source-over math documented above. Automatic tests include full-frame vs ROI spatial equivalence.


## Exact defaults / host integration confirmed

Binary ParamsSetup confirms both visualization stroke colors default to opaque `#0030FF` (PF A,R,G,B bytes `FF 00 30 FF`), Stroke Width defaults to 2.0, and Opacity to 100%.

Binary SmartPreRender confirms the source checkout is `0,0,width,height` using full `PF_InData.width/height`, with `preserve_rgb_of_zero_alpha = TRUE`. Result/max rectangles follow the host checkout rather than being replaced by a guessed ROI.

Binary drag handling returns exactly `PF_EO_HANDLED_EVENT | PF_EO_UPDATE_NOW` after writing the changed arbitrary Grid Positions parameter; it does not set `PF_EO_ALWAYS_UPDATE`.
