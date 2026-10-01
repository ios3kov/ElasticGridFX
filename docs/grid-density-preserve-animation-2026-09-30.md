# Animation-preserving guide density — 2026-09-30

## Stage 10 / approved change

User supersedes the axis-reset policy: Columns/Rows are static control-density
settings. Increasing/decreasing either must not change the image, motion, or
any Grid Positions key, including while parked on a key with its stopwatch on.
Retain CANNOT_TIME_VARY, FSTR Stretch.plugin and the first-application fix.
Baseline: 7f7220168421491fe1cbe3ebf74041810c5ebb52 / PR #10. Separate branch
fix/grid-density-preserve-animation. No merge/release/installation authorized.

Rules reread: FSTR-Line/DEVELOPMENT_RULES.md, blob
701a8c1ae3acb4dbfe1d7eda94acbf8095b88608. Scope: macOS arm64 / AE 25.6,
CPU rendering and the existing 2D/3D planes. Developer environment is Linux,
without AE; native execution is NOT RUN until actual evidence exists.

## Baseline, design and implementation

The old count handler called sync_grid_topology and wrote Grid Positions;
ordinary/SmartFX snapshots also resized and reset its axes using display counts.
Four new source-contract assertions reject that baseline. This is a source
reproduction, not a claim that a debugger captured the user's exact callback.

The implementation separates retained deformation from visible controls:

- Count handlers request a redraw only. No Grid Positions setter, keyframe API,
  migration or hidden conversion runs. Neither renderer snapshot reads counts.
- Rendering and AE's existing arbitrary-data interpolation use original owned
  Grid Positions values, with validation but no resampling/axis reset.
- UI drawing, picking, cursor and dragging share one derived control view.
  Below the retained knot count a nested maximin subset distributes removals;
  above it, widest source intervals are bisected while old anchors are retained.
  Ties are deterministic. Each count from 1 to 50 has a stable nested layout.
- New positions follow the existing render inverse map, including easing, and
  the existing wave evaluator. Read-only sampling never writes retained state.
- Real guide drags act on the retained curve. Original knot handles preserve
  established drag behaviour; inserted handles distribute elastic movement to
  retained neighbours, including when radius is zero. Only a nonzero resulting
  edit calls the Grid Positions setter. A reentrant density change cancels the
  in-flight drag rather than redirecting it to a different handle.

Reducing visible controls does NOT discard underlying deformation detail.
Increasing them supplies additional positions to grab the existing curve; it
is not an insertion of independent polynomial coefficients into every old key.
The interpolation-based handles still operate on the retained lattice. This
limitation is explicit, not hidden behind a lossy resampling of animation.

Wire format, parameter IDs and existing key interpolation stay unchanged.
Legacy keys already containing different stored topologies retain their old
interpolation semantics; this change does not repair previously broken keys.
Old projects with animated count parameters need separate host review on copies.
No migration script deletes keys or overwrites a user's project.

Version/AE effect version move to 0.9.2 development to invalidate old render
caches. Dependency versions remain locked; only the root package version changes.
The renderer ABI/algorithms and named package pipeline are unchanged.

## Acceptance set defined before implementation

The initial plan is preserved at 7dece81d367a1f5a8b46e4d05b20612d21255115.

1. Count changes on/between keys leave encoded Grid Positions, key metadata and
   rendered pixels unchanged. No new/replaced key, reset or approximation.
2. Counts 1/4/7/19/50, repeated up/down/restore cycles, both axes independently;
   correct number of controls, stable endpoints and retained hidden shape.
3. Shared drawing/picking, usable added handles, zero-delta/no-click writes;
   explicit drags still deform with original exact-knot behaviour.
4. Production CPU FFI comparisons at 8/16/32 bpc, Bilinear/Bicubic, waves/easing,
   edge modes and dense/sparse input; existing core/serialization regression.
5. Pending first-frame identity is independent of display density but still
   rejects non-neutral/invalid retained data. No AEGP writes from workers.
6. Source guards, C++ strict analysis, sanitizers, Rust/Clippy, macOS source/build,
   signed named/extracted payload checks and complete artifact identity.
7. Actual AE on the final candidate: count change with animated Grid Positions,
   key count/times/values, before/after frame comparison, Undo/Redo, save/reopen,
   2D/3D, preview and cancellation sanity. CI never substitutes for this gate.

## Local evidence (not AE)

- Baseline source: four expected assertion failures; new wiring: four PASS.
- C++ Release: 20/20 test targets PASS, including the new control-density target.
  Its FFI matrix checks exact pixels for repeated density cycles, both filters,
  three edge modes, waves/easing, dense/sparse buffers and 8/16/32 bpc.
- Python: 230 discovered, 224 PASS, 6 macOS-only skips. All Node suites PASS.
- Strict syntax/high-warning stage passed for all listed C++ files. The full
  local analyzer command hit the session timeout before completing; NOT PASS.
  Separate Clang Static Analyzer on the new control_grid_ffi.cpp: PASS.
- New Rust tests cover immutable encoded keys/interpolated lattices, nested
  layouts, real extra-handle edits, zero-delta writes and malformed inputs.
  Rust/Clippy require the exact-head macOS CI result (not available locally).
- ASan/UBSan/LeakSanitizer on the new C++ density/FFI target: PASS.
  Final-source macOS/packaging results are recorded in the PR when run.

No new candidate was installed or executed in AE during local implementation.
No host PASS, old-project migration success, pixel-matrix certification on AE,
or release readiness is implied. The prior 94d6706 USER-REPORTED checks remain
historical and are not relabeled as evidence for this candidate.

## SDK references

- https://ae-plugins.docsforadobe.dev/effect-basics/PF_ParamDef/
- https://ae-plugins.docsforadobe.dev/effect-details/parameter-supervision/
- https://ae-plugins.docsforadobe.dev/effect-details/arbitrary-data-parameters/
- https://ae-plugins.docsforadobe.dev/effect-ui-events/PF_EventUnion/

These justify the static count flag, separating redraw from value changes,
retaining host-owned arbitrary-data interpolation, and four bounded drag refcons.
