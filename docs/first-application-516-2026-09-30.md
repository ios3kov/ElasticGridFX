# First application / BadCallbackParameter 516 — 2026-09-30

## Status and baseline

Stage 10 remains BLOCKED by issue #9. The user observes an immediate error on ordinary, text and precomposition layers with released 0.9.1, source e1848d5348c8059c0307c5d8c1241163ffd92ab1, Build ID EGFX-879b31e5a95527a385827c24. About/error screenshots are in the development conversation. This is user-observed FAIL, not a developer AE reproduction.

Investigation branch: fix/first-application-516, based on main 7e743984a0d102066d6a70dcb7df64493bfffd24. Main, published artifacts and the user's installation must remain unchanged. Rules reread: FSTR-Line/DEVELOPMENT_RULES.md blob 701a8c1ae3acb4dbfe1d7eda94acbf8095b88608.

## Source finding and proposed scope

The default native-plane marker is zero (pending). The existing comp_space_kind rejects every pending frame with BadCallbackParameter, while binding is deferred from SequenceSetup to main-thread idle. Therefore even an untouched neutral first frame is rejected if requested before binding. This return path is established in source; its occurrence in the reported AE session is not instrumented and remains a host-verification requirement. UI callbacks and binding failures may independently return the same generic code.

Investigate a narrowly proven neutral-frame path rather than ignoring callback errors or moving AEGP writes into render callbacks. No general fallback for uninitialized deformed 3D content; no fake ready marker, polling wait in render, globally suppressed errors, architecture rewrite, public parameter/schema changes, GPU or performance work.

## Acceptance decided before implementation

- Preserve the pending guard for deformed grids, nonzero waves/easing, invalid state, and pending non-neutral saved projects.
- Only a strictly checked initial identity state may use the existing exact identity renderer without a projected plane. Verify the actual C++ CPU identity path numerically at 8/16/32 bpc, including sparse/nonzero-origin input and float extended range; no resampling or tolerance-based claim.
- SmartPreRender must checkout every dependency used for the decision and retain an owned snapshot; SmartRender must not inspect a live parameter array. Legacy frame selectors must use their appropriate parameter access. Failed host callbacks must still propagate.
- Deferred binding, marker 0 -> ready invalidation, foreign/partial/keyed binding protection, public parameter IDs, Grid Positions animation, Undo/Redo and 2D/3D mode behavior remain intact.
- Add automated regression for the rejecting baseline, permitted neutral case and prohibited cases. Run affected Rust contracts/Clippy, portable C++ regressions, Python/Node safety tests, and exact-head macOS source gate before any installable handoff.
- Real AE: first addition after restart AND later additions in the same session, ordinary solid/raster, native text and Checkerboard precomp, 8/16/32 bpc; no initial dialog; neutral pixels match bypass; immediate subsequent deformation updates. Include already-3D text/raster, save/reopen and background render. An eventual successful render after dismissing a dialog is not PASS.
- Native host tests need a new identified candidate. No automatic installation, merge or publication. No claim of fixing the user's runtime failure before those tests.

## Environment and stop condition

The present local environment is Linux without Rust/macOS/AE. Direct GitHub cloning fails DNS; connected GitHub reads/writes and hosted CI are available. Hosted tests cannot substitute for GUI AE. Gather available automated evidence, retain BLOCKED/NOT RUN for missing host evidence, and do not ask the user to repeat failed manual Preview/cancel tests.

Issues #7 (RAM Preview) and #8 (cancel/re-render) remain open until #9 is actually resolved on the tested candidate. Existing historical reports are not rewritten. Source reports and generated artifacts must identify their own commit; hashes of changed tracked files must be regenerated before handoff.

## References checked

- https://ae-plugins.docsforadobe.dev/effect-basics/errors/ — propagate errors; interruption is not a successful frame.
- https://ae-plugins.docsforadobe.dev/aegps/aegp-suites/ — deferred idle wakeup is not synchronous initialization.
- https://ae-plugins.docsforadobe.dev/smartfx/smartfx/ — checked-out pre-render dependencies and per-request data.
