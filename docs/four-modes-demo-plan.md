# Four deformation modes and disabled Demo — 2026-10-05

Authorized product extension under frozen rules8.0.0; applies to Mac and Windows.
Existing installer obligations and exact Dev52/Dev56 acceptance remain separate.
No old acceptance transfers to a new plugin artifact. No main/merge/release.

## Requirements and coordinate contract

| ID | Behavior | Verification |
|---|---|---|
| M1 | Comp mode retains ordinal1/current Layer Plane behavior, including automatic 3D handling | Old-mode pixel/ID regression; native old project reopen |
| M2 | Layer mode adds ordinal3, fitting 2D sourceRectAtTime bounds; existing 3D automatic plane handling retained | Transformed small 2D footage/text; animated source bounds; UI/render agreement |
| M3 | Flat mode retains ordinal2/current Four Corners; neutral grid does not pin source to destination | Skewed neutral pass-through and deformed core regression |
| M4 | Perspective adds ordinal4: pin source canvas (or already-projected text basis) to destination quad and inverse grid before one sampler | Analytic corner/center coordinates; pixels at all depths/qualities; corner drag |
| M5 | All modes share grid, Wave, Show Grid, sampling quality, abort and saved animation; no mode switch rewrites parameters | Owned pre-render snapshot, Undo/save/reopen and animated mode matrix |
| D1 | Demo design is FSTR FX inside each cell; future preview/export watermark | Design only; actual licensing integration is a prerequisite |
| D2 | Demo rollout is OFF even without license; no watermark or public fake activation toggle | Both render routes remain unchanged; explicit compile-time disabled policy |

UI names: Comp mode / Flat mode / Layer mode / Perspective. The last caption
is kept short to preserve native popup sizing without dummy options. Existing popup
ordinals1/2 retain meaning; new choices append so saved projects are not reinterpreted.
Flat/Perspective are short engineering-selected labels based on the user request.
Mode remains static. Public corner controls/Fit Layer are enabled in Flat (2D)
and Perspective; old 3D Flat effective automatic behavior is retained. Other modes
have derived bounds and no editable corners. Selector stays usable for new modes.

Layer mode uses actual layer-local vector/footage bounds on 2D, before ordinary
AE transforms; PF scales Point dependencies and frame origin is removed once.
On 3D raster, input is layer-local and already uses the layer canvas. On 3D text,
retain the current camera-projected sourceRect basis, avoiding double projection.

Perspective pins the same effect-input canvas addressed by public points; for
comp-space text its unedited plane is the source transform. Outside the pinned
quad is transparent. Invalid quad falls back to original pixels with existing
UI diagnostics, not a render dialog. Output stays within the current logical
canvas; expanded output beyond that canvas is not part of this implementation.
Fit Layer preserves its current source-boundary contract. Mode switches preserve
all corner/grid keys; current values determine the new mode's output.

## Binding migration and safety

The existing five hidden streams stay unchanged in IDs/types. Upgrade only exact,
unkeyed, enabled owned v2 bindings (or pristine/exact v1 when not Undo-preserved)
to v3. Point expressions retain previous toComp math except 2D Layer mode, where
they return layer-local sourceRect bounds. Kind4 identifies this local-bounds
snapshot; kind0 pending is not permission to render an unknown deformation.
Receipt generation2 distinguishes first upgrade from Undo of that upgrade. Mark
before the binding Undo group; restore the exact old generation on failed upgrade.
Foreign/partial/keyed expressions remain refused. Rendering uses checked-out PF
values only, never AEGP mutation or mutable global geometry.

## Demo design

One future FSTR FX inscription per destination cell, anchored to cell geometry,
including deformed/animated cells and guide-density redistribution. A bounded
vector/glyph implementation should use the evaluated frame snapshot; license
verification belongs outside the per-pixel/per-frame loop. Valid registration
removes text without modifying any saved grid/scalar stream. No global diagonal
watermark, quality/speed reduction or added restriction is authorized.
Current policy is Disabled; provider SDK entitlement verification is absent.
No on/off user control or fake licensed status is introduced. Watermark rasterizer
and provider activation are deliberately NOT IMPLEMENTED while disabled.

## Sequence / evidence

1. Implement mode schema, generation-safe binding, shared perspective dispatch
   and explicit disabled demo policy; scoped Rust/C++/expression checks.
2. Build a uniquely identified Mac candidate and verify owned AE mode/UI/pixels,
   drag/Undo, animated dependencies and old project reopen.
3. Windows MSVC/FFI/CI and user host validation of the new artifact.
4. Reconcile installer payload selection only after new plugin acceptance.

Mac Dev60 scoped pixel acceptance PASS: four neutral modes/3D baseline, Flat skew
pass-through, Perspective skew/Wave/Show Grid, half-resolution odd-size source,
animated sourceRect text bounds (error <1e-9), old animated project pixel/key
metadata and save/reopen. Evidence: outputs/update-094-four-modes-mac. Corner
drag and full new-mode save/reopen acceptance remain NOT RUN. Windows source
51a8d3c MSVC/CI PASS37288366891; new Windows host acceptance NOT RUN.

Added [centered corner loupe](corner-loupe-plan.md), preserving this contract. All previous installer/test failures
and accepted payload identities remain retained. No real speed measurement required.

SDK source review: installed Adobe SDK25.6 AE_Effect.h:3033–3070 documents
full-resolution input dimensions and automatic Point scaling;3152–3154
defines pre-effect origin for frame calls. Expressions use sourceRectAtTime,
toComp and effect-local propertyGroup/index lookup. Native AE verification of
that numeric lookup and dynamic bounds is still required. The v2 expression
strings remain byte-for-byte available for owned migration and old-mode math.
