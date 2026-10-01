# Fit Layer coordinate correction — 2026-09-30

## Baseline and scope

Stage 10, issue #14. Baseline 59a73ea0b6856ae8ff0aea280021ff12b377a82d,
Build EGFX-64cfcd845067bdbc72d82bfa, named ZIP SHA256
ca2388c7daf572f4018de26b0f7212dbe164de4291d4484f8292250159c5e772.
The user accepted density behavior and supplied initial/edited/fitted corner
screenshots. All eight displayed components were audited. Fit subtracts one
from width/height while initial points reach the full source boundary.

Rules reread: FSTR-Line/DEVELOPMENT_RULES.md blob
701a8c1ae3acb4dbfe1d7eda94acbf8095b88608. Work is on a new branch stacked on
PR #12; no merge, release or user-machine mutation is authorized here.

## Required contract, fixed before implementation

- Fit yields TL (0,0), TR (W,0), BR (W,H), BL (0,H), matching initial controls.
- W/H are the full source-layer dimensions from PF_InData, not comp size,
  zoom, rounded/downsampled buffer size, or the last pixel's index.
- No frame-only origin or downsampling fields are read during the Fit UI call.
- Width/height 1 must produce a nondegenerate boundary quad. Invalid dimensions
  or coordinates outside the existing 16.16 setter's range must not saturate.
- Repeating Fit at already fitted values is a no-op on point value-change flags.
  Explicit Fit may update changed corner keys at the current time using AE's
  normal Undo/key behavior; it must not write Grid Positions, counts or old keys.
- Preserve parameter IDs, order, types, density behavior, serialized values and
  all render algorithms. No automatic migration of old W-1/H-1 saved corners.

## Coordinate audit boundary

A source boundary W/H and a raster index limit W-1/H-1 are different domains.
Do NOT globally remove raster subtractions or rescale saved deformations.
The Layer Plane overlay/index-span discrepancy in #14 is not an instrumented
host misalignment result. Full/Half/Quarter and viewer callbacks must be checked
before changing that mapping; this fix must not silently change renderer output
or introduce an assumed half-pixel/preview correction. Retain that explicit
host-alignment gate rather than claiming the two domains are identical.

## Tests and gates

Before/after source regression for the old subtraction and unconditional writes.
Rust tests must call the production Fit helper, compare all eight values for
landscape/portrait/odd/tiny dimensions, check idempotence and fractional inputs,
and run actual plane FFI with deformed pixels at 8/16/32 bpc, both filters and
edge modes, padding/origins, Full/Half/Quarter-sized cases. Geometry forward/
inverse tests retain fractional and translated quads. Existing key/density,
projection, first-add and plane regression remain required.

Run all available Python/Node, C++ regression, Rust/Clippy and full macOS
source/build/signature/extracted-package checks. Record exact commit, Build ID,
manifest and ZIP hash. A new source candidate is not the previously tested ZIP.
A separate small packaging correction may resolve #13 before signing this new
candidate: Finder version derived from validated build metadata, never edited
inside an already signed/tested artifact.

Actual AE verification is NOT RUN in the Linux development environment. Target
AE 25.6 / Apple Silicon: initial -> edit -> Fit, repeat Fit, no-op on/between
corner keys, Undo/Redo, different source/comp sizes, zoom/preview resolutions,
1-pixel axes, save/reopen, density/key and supported 2D/3D sanity. Preserve
original projects and the prior plugin. No host PASS from source or CI alone.

## Sources checked

- https://ae-plugins.docsforadobe.dev/effect-basics/PF_InData/
- https://ae-plugins.docsforadobe.dev/effect-basics/parameters/
- https://ae-plugins.docsforadobe.dev/effect-basics/PF_ParamDef/
- https://ae-plugins.docsforadobe.dev/effect-ui-events/ui-callbacks/

The implemented project's initial defaults and the user's screenshots establish
its boundary convention; historical SDK point-default behavior is not a reason
to change parameter registration or silently remap existing projects.
