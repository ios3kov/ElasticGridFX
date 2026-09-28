# ElasticGrid — original-parity Hot Loader adapter

The previous Hot Loader branch was based on obsolete main render code and did not include the later `fix/original-gridwarp-parity` work. Real AE exposed that directly: custom guides moved while pixels could remain unchanged, and SmartFX produced seam/line artifacts.

This branch is now rebuilt from the parity tree, then the hardened Hot Loader shell/protocol is re-applied.

Identity remains ElasticGrid FX / `com.elasticgrid.fx.warp`; behavior and visible parameter surface are derived from the supplied GridWarp binary. Effect API 13.29, Protocol ABI 2, StateABI 5, Grid wire v3, Rust 1.98.1.

The first live correctness gate intentionally uses CPU SmartFX with full-source checkout. Metal returns only after real AE passes deformation with no seam artifacts. Every independent test uses the clean-run installer.


## Live status — 2026-09-28

The original-parity clean live gate is **FAILED** in real After Effects 25.6.

Observed:
- the effect loads;
- the custom grid UI is visible and draggable;
- rendered pixels do not follow the dragged guides correctly;
- bright vertical/horizontal render artifacts appear.

This means the current implementation is not accepted as a GridWarp parity baseline even though CI is green.

No further user manual build is allowed until the automatic/research phase is complete. The next stage is binary decomposition + Adobe SDK/open-source comparison + deterministic render/UI regression harness + code audit/debugging/profiling. A new user test package is produced only after those gates are exhausted.


## Automatic SmartFX contract fix — 2026-09-28

Binary decomposition and Adobe/open-source cross-check identified a concrete host-contract bug:

- the original GridWarp SmartRender explicitly checks out render parameters from AE;
- ElasticGrid was reading the ordinary parameter view during SmartRender;
- the custom UI therefore could update its guide state while SmartRender saw stale/invalid render-time state.

The implementation now:
- checks out the original render dependency set during SmartRender;
- renders only from those checked values;
- explicitly invalidates the AE viewer after a guide mutation;
- treats the untouched original 4-guide/N+2 grid as exact identity even with the original Stretch Easing default;
- has automated gates that reject ordinary-param reads inside SmartRender.

This change is **not yet a user-test release**. It must pass the automatic regression/audit/performance stages first.


## Visualization parity implementation — 2026-09-28

The recovered rendered-Visualization contract is now implemented on the CPU parity path:

- evaluated column/row guides are composited after the warp;
- PF colors are interpreted as A,R,G,B;
- Stroke Width and Opacity use the recovered antialiased coverage equation;
- blending follows the recovered unpremultiplied source-over formula;
- full source width/height and PF_InData output origin are carried separately from SmartFX ROI coordinates;
- 8/16/32f CPU paths share the same compositor;
- HDR RGB is not clipped on the 32f path.

New automated regression checks cover:
- exact default/off behavior;
- full-coverage and 50% opacity guide pixels;
- ROI/full-frame visualization spatial equivalence.

Metal remains intentionally disabled in the live parity gate until CPU behavior is proven; GPU Visualization parity is a later gate.


## SmartPreRender / drag parity correction — 2026-09-28

Static decomposition of the original closed two more host-level mismatches.

SmartPreRender now mirrors GridWarp:
- checks out source rect `0,0,PF_InData.width,PF_InData.height`;
- does not multiply source dimensions by downsample;
- sets `preserve_rgb_of_zero_alpha = TRUE`;
- unions host checkout result/max rectangles;
- clips result rect to the original output request.

Guide drag now mirrors the original event contract:
- arbitrary Grid Positions is marked changed by the parameter write;
- event flags are exactly `HANDLED_EVENT | UPDATE_NOW`;
- the previous extra `ALWAYS_UPDATE` and App-suite invalidate call were removed.

The hot-reload generation token remains as an intentional shell-only extension in pre-render data.


## Clean-run match-name hardening — 2026-09-28

The clean installer now detects duplicates by all three identities:
- bundle filename;
- `CFBundleIdentifier = com.elasticgrid.fx`;
- embedded AE match name `com.elasticgrid.fx.warp`.

This closes the remaining case where an old/test bundle was renamed but would still register the same effect in AE. The match name is stored as a literal in the PiPL/resource payload, so the installer scans files inside every candidate `.plugin` bundle in known Adobe load roots and archives conflicts before installation.
