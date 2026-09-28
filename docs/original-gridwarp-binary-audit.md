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
