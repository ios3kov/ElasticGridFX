# 3D switch: immediate Effect Controls synchronization

Stage 10/10, post-release corrective work; release v0.9.0 remains unchanged.

## Baseline and requirements

The user reproduced active Four Corners controls after enabling the native
text layer's 3D switch in AE 25.6 arm64. Read-only inspection confirmed
`threeD=true`, stored mode 2, hidden kind 2, v2 expression with no error.
Released plugin: 36a04e3, Build ID EGFX-2c67eaa5e9c441ca5866edaf.
Rendering already locks the effective mode; panel synchronization is FAIL.

- R1: immediately disable the selector, four points and Fit Layer in 3D.
- R2: restore the selected 2D mode and appropriate controls on return to 2D.
- R3: do not change values, expressions, keys, persistent IDs or rendering.
- R4: no polling, worker-thread AEGP calls or unconditional redraw loop.

## Scoped implementation

The cosmetic UI path reads the owning layer's actual AEGP 3D flag instead of
the hidden expression snapshot. UPDATE_PARAMS_UI must not check out/evaluate
parameters. Only the existing UPDATE_PARAMS_UI and USER_CHANGED_PARAM paths
perform cosmetic updates. No layer handles are retained and neither render nor
DRAW callbacks call this path. An unavailable owning layer during construction
defers cosmetics until the next UI callback.

SDK references: AEGP_PFInterfaceSuite1::AEGP_GetEffectLayer,
AEGP_LayerSuite::AEGP_GetLayerFlags, PF_UpdateParamUI; locally pinned
after-effects 0.4.0 wrappers and Adobe header bindings. SDK selector guidance:
https://ae-plugins.docsforadobe.dev/effect-basics/command-selectors/

## Mandatory verification

- Unit state table for both saved mode ordinals: PASS.
- Rust tests: 42/42 PASS on macOS arm64 with user-local Rust 1.98.
- Strict Clippy uses the repository's two existing allowances for dependency
  macro style lints (`drop_non_drop`, `question_mark`); no new allowances.
- Release compilation: pending.
- Review: UI-only calls, borrowed layer lifetime, no serialization changes.
- AE exact-candidate test: 2D Four Corners -> 3D -> 2D, with Effect Controls
  continuously open; text and raster; Undo/Redo; preserved keys and corners;
  no ongoing redraw while idle. NOT RUN until candidate is loaded.
- Host installation/restart blocked while the user's project has unsaved
  changes. Do not close it or overwrite the released package.
  Read-only host probe on this turn confirmed `dirty=true`, one project item.

## First runtime candidate (453074f): rejected

Installed with authorization, verified live Build ID
EGFX-fe5b61df9760fd3f4da78baa. Existing 3D layer displayed correctly locked
controls, but duplicating its comp failed with AE `child not found in parent`;
creating a separate test produced BadCallbackParameter. Cause not proven.
The added DRAW cosmetic update is removed from the next candidate to keep
updates in the original host UI selectors; construction without a valid layer
defers cosmetic updates. These errors are not accepted as PASS.
User project was saved to a fresh recovery checkpoint before restarting;
original on-disk project and v0.9.0 release were not overwritten.

Offline scanner: 26 review findings in existing workflow files and a test
fixture, none in changed source. Mutable action references and credential
persistence remain existing supply-chain review items; the authentication
heuristic matches a test fixture, not a live service. Not a security certificate.
