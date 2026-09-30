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
parameters. UPDATE_PARAMS_UI and USER_CHANGED_PARAM rebuild the cosmetic
definition as before. DRAW changes only mismatched DISABLED bits, not popup
names/counts. No handles are retained and render callbacks are untouched.
An unavailable owning layer during construction defers cosmetics until the
next UI callback.

SDK references: AEGP_PFInterfaceSuite1::AEGP_GetEffectLayer,
AEGP_LayerSuite::AEGP_GetLayerFlags, PF_UpdateParamUI; locally pinned
after-effects 0.4.0 wrappers and Adobe header bindings. SDK selector guidance:
https://ae-plugins.docsforadobe.dev/effect-basics/command-selectors/

## Mandatory verification

- Unit state table for both saved mode ordinals: PASS.
- Rust tests: 42/42 PASS on macOS arm64 with user-local Rust 1.98.
- Strict Clippy uses the repository's two existing allowances for dependency
  macro style lints (`drop_non_drop`, `question_mark`); no new allowances.
- Release compilation: PASS, clean source 951f4a7.
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

## Second runtime candidate (3cd2d9a): original symptom remains

Live identity confirmed. With Effect Controls continuously visible, scripting
the native 3D switch changed the grid/render but left Four Corners enabled.
Thus UPDATE_PARAMS_UI alone does not cover the external switch. Third candidate
uses DRAW solely for the SDK-permitted DISABLED bit changes, without rewriting
popup names or choice counts in event context. Existing UPDATE_PARAMS_UI retains
the effective-mode label. Runtime acceptance remains open.

## Third candidate: scoped runtime result

- Source: 951f4a7, clean arm64 Release; Build ID EGFX-5a6dbf03b692020f0a89957e.
- ZIP SHA-256: f7017359f06bfbd7d055dd78b85ad5b60544360332ec5cd1c49e6e845bec6693.
- Installed with rollback receipt EGFX-update-677a9886ecf543549ee6ac723a106d5f.
- Installed/live image identity PASS, AE PID 47998, report
  `outputs/FSTR-Stretch-panel-refresh/identity-v3/EGFX-check-139c7e0d84f54be2a2c9f82adb2f8733.zip`.
- Rust 42/42 and repository Clippy policy PASS on final source.
- Text Four Corners -> 3D -> 2D with panel continuously open: visually PASS;
  selector, all corner fields and Fit Layer immediately disabled/restored.
- Two Grid Positions keys, all four nondefault corner values and stored mode
  remained intact: PASS (`v3-regression.txt`).
- Duplicate existing comp, create fresh text effect and fresh raster effect:
  PASS, no recurrence of first-candidate errors in this fixture.
- Raster 3D -> Undo -> Redo: visual PASS for control enable states.
- Save/reopen test project, raster kind 3 -> 1, and restoration of the original
  user checkpoint: PASS (`v3-finish.txt`); AE window title confirms checkpoint
  reopened without a dirty marker. Initial host-safety blocker was resolved by
  explicit user authorization and a fresh recovery save before shutdown.
- Immediate event refresh intentionally retains the stored mode's label while
  disabling it; the next normal panel update shows Layer Plane (3D).
- No timers/polling introduced. Full performance certification not part of
  this UI-only patch; unchanged renderer is not re-certified by UI evidence.

This is an installed corrective test build, not a new published release.
All screenshot observations are recorded in the current chat tool evidence.
The release v0.9.0 and main branch have not been changed by this patch.

Offline scanner: 26 review findings in existing workflow files and a test
fixture, none in changed source. Mutable action references and credential
persistence remain existing supply-chain review items; the authentication
heuristic matches a test fixture, not a live service. Not a security certificate.
