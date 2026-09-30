# Animation-preserving guide density — 2026-09-30

## Stage 10 / approved change

User explicitly supersedes the axis-reset policy: Columns/Rows are static control-density settings. Increasing/decreasing either count must not change the image, motion, or any Grid Positions key (including while parked on a key with its stopwatch enabled). Retain `CANNOT_TIME_VARY`, `FSTR Stretch.plugin`, first-application fix and existing effect/parameter IDs. Baseline: 7f7220168421491fe1cbe3ebf74041810c5ebb52, PR #10. Separate branch; no merge, release or user-machine changes authorized by this work.

Rules reread: FSTR-Line/DEVELOPMENT_RULES.md, blob 701a8c1ae3acb4dbfe1d7eda94acbf8095b88608. Scope: macOS arm64 / AE 25.6, CPU paths, existing 2D/3D plane behavior. Current developer environment is Linux without After Effects; native execution remains NOT RUN until evidence exists.

## Baseline and design

Current `sync_grid_topology` writes Grid Positions on count changes; both ordinary and SmartFX grid snapshots also resize/reset the saved axes using those counts. These are independent violations of the approved invariant.

Separate retained deformation from visible controls. Rendering/interpolation use the original owned Grid Positions data, not the display count. Count changes redraw controls only: no arbitrary-data setter, keyframe API or migration writes. No approximate downsampling of the retained curve. Hidden deformation detail must survive reducing visible controls, otherwise exact image preservation is not possible.

Control positions are derived from the retained curve at the current frame. Increasing count subdivides source intervals while retaining existing canonical anchors; decreasing selects a balanced subset. Derivation is deterministic, read-only and bounded to the existing 1–50 visible-guide range. Drawing and picking must use the same derived layout. Additional handles act on the retained curve through interpolation/elastic influence; they must be usable, not dummy decorations. The saved knot lattice is not destroyed or resampled merely by changing density. Only an explicit nonzero guide drag may write Grid Positions. No promise of additional independent deformation coefficients from a display-only count change.

Keep the current wire format and interpolation for existing keys. Old projects that animated Columns/Rows, or already contain keys with different stored topology, require separate compatibility review; do not rewrite them or claim they have been repaired. Count invariance applies to the deformation represented by the saved Grid Positions stream.

## Required acceptance set (defined before implementation)

- Before/after encoded Grid Positions bytes, key count/times, and rendered pixels identical for count changes, on keys and between keys. No new key when changing either count.
- Counts 1/4/7/19/50 and repeated increase/decrease/restore cycles; no axis reset; same endpoints and retained shape.
- Uniform extra controls and balanced removals; draw/hit-test agreement; real dragging still deforms; zero movement/click/count change does not write.
- Actual production FFI pixel comparisons at 8/16/32 bpc, Bilinear/Bicubic, waves/easing, dense/sparse input; existing renderer/serialization regression remains green.
- First-add pending identity cannot start depending on display density; invalid/non-neutral states still fail closed.
- Unit/source safety tests, strict C++ analysis, Rust/Clippy, full macOS source/build/named-package checks, hashes and clean source. Preserve FAIL records.
- Real AE on the final identified candidate: stopwatch behavior; density change with animated Grid Positions; Undo/Redo; save/reopen; key metadata and numeric render comparison; 2D/3D, preview and cancellation sanity. No CI/mock result substitutes for AE.

SDK references: https://ae-plugins.docsforadobe.dev/effect-basics/PF_ParamDef/ (CANNOT_TIME_VARY vs CANNOT_INTERP); https://ae-plugins.docsforadobe.dev/effect-details/parameter-supervision/ (value changes vs redraw); https://ae-plugins.docsforadobe.dev/effect-details/arbitrary-data-parameters/ (host-owned interpolation and serialized state).

## Status

Acceptance/design recorded. Implementation and tests pending. Original published and installed artifacts unchanged. No final host PASS or release readiness claim.
