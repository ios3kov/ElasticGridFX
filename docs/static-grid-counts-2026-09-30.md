# Static Columns and Rows — 2026-09-30

## Stage 10: source change; target AE acceptance pending

The user requested manual-only guide counts because animated counts break the
intended deformation animation. Baseline: PR #10, commit
`94d6706db5539d9f3372adf0efc8edeeaa38a622`, Build ID
`EGFX-e967ab9a87fe4e44340d0f46`. That candidate is retained unchanged.
Rules reviewed: FSTR-Line/DEVELOPMENT_RULES.md, blob
`701a8c1ae3acb4dbfe1d7eda94acbf8095b88608`.

## Implementation and limits

Set `ae::ParamFlag::CANNOT_TIME_VARY` on Columns and Rows during parameter setup.
Keep SUPERVISE, integer type, IDs/order, default 4, valid range 1–50, normal
editable UI and the existing manual topology-change handler. Grid Positions
remains animatable. Render code, grid interpolation/wire format, first-frame
fix and other parameter policies are unchanged. This is not a renderer rewrite.

Use the native behavior flag rather than hiding a control, making it UI-only,
or disabling interpolation (which would still allow hold keys):
https://ae-plugins.docsforadobe.dev/effect-basics/PF_ParamDef/#parameter-flags
The pinned after-effects 0.4.0 wrapper exposes CANNOT_TIME_VARY.

Choose counts before animating Grid Positions. Manually changing counts later
still resets the changed axis under the existing algorithm; this change does
not promise to preserve deformation after a topology change.

No migration script deletes old keyframes or rewrites user projects. Loading
old AEPs that already animate Columns/Rows is NOT VERIFIED: the host's treatment
of those streams needs testing on copies before compatibility can be claimed.
Preserving IDs and GridArb serialization alone does not prove that migration.

## Checks and next gate

Added three source-contract tests: only counts lose time variation; controls
remain editable/supervised; manual topology synchronization stays connected.
New assertions reject the baseline (three assertion failures), then pass with
the minimal change. These tests inspect source, not AE's live UI.

Local Linux: targeted 12 tests PASS; full Python suite 215 discovered,
209 PASS and 6 macOS-only skips; C++ 19/19 PASS; all Node safety test files PASS.
Rust/Clippy and the signed macOS bundle must be checked for the resulting commit
in CI. A new Build ID/hash is required; earlier candidate evidence is historical.

Target AE 25.6 Apple Silicon gate (NOT RUN by developer): no stopwatch for
Columns/Rows in Effect Controls and Timeline; counts stay manually editable;
Grid Positions still accepts and plays keys; check manual counts 1/4/50,
Undo/Redo, first add, save/reopen and unchanged-topology animated AEPs.
Separately inspect old animated count streams and expression eligibility; do
not silently certify old projects or overwrite originals after opening them.

On the baseline, the user reported first-add/deformation, save/restart,
RAM Preview and cancellation/re-render success. These are USER-REPORTED results
in issues #9, #7 and #8, not new-candidate host tests or full process closure.

No installation, merge or release is part of this source change. The requested
`FSTR Stretch.plugin` filename remains a separate packaging task; do not rename
an already identified archive or claim that packaging task is complete here.
