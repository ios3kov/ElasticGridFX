# Project / Keyframe Compatibility — v0.9

## Stable effect identity

- Match name remains `com.elasticgrid.fx.warp`.
- Existing `Params` enum variant names are frozen because the Rust AE host derives parameter IDs from those names. New parameters may be added, but existing variants must not be renamed.

## Grid Positions wire format

`GridState` remains the same six-field serde/bincode struct shape used by v0.8. To add explicit migration without breaking old projects, the existing `columns: u16` field now carries a compact header:

- v0.8 legacy: `columns = 1..128`;
- v0.9+: bit 15 = versioned marker, bits 8..14 = schema version, low 8 bits = actual column count.

Current schema version is 1. The decoder accepts the legacy v0.8 form and schema v1, and rejects unknown future versions instead of silently misreading project data.

## Keyframes

- Same-topology GridState keyframes interpolate guide positions linearly.
- Different topologies step at the midpoint; the separate Columns/Rows parameters remain the authoritative topology and `grid_snapshot` resizes state safely if they disagree.
- Pin state steps at the midpoint rather than blending.

## Undo / redo

Viewer dragging writes GridState through `ArbitraryDef::set_value`, which sets After Effects `CHANGED_VALUE`; the event is marked handled/update-now. This keeps edits in the host parameter transaction path instead of mutating hidden global state. Real undo/redo behavior remains a target-AE runtime gate.

## Migration rule for future releases

Never change the six top-level serialized field types/order. Increment the compact schema version and migrate the interpretation of existing fields (or encode additional version-specific payload inside one of the vector fields). Unknown schema versions must fail loudly rather than render incorrectly.
