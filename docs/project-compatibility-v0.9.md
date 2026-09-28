# Project / Keyframe Compatibility — current original-parity branch

> Current source of truth for `feature/ae-hot-loader-shell`. Earlier v0.8/v0.9 notes are migration history only.

## Stable effect identity

- Match name remains `com.elasticgrid.fx.warp`.
- Existing `Params` enum variant names are frozen because the Rust AE host derives parameter IDs from those names.
- The current original-derived visible topology limit is **50 internal column guides / 50 internal row guides**.

## Grid Positions wire format

The current `GridState` keeps the same six top-level serde/bincode fields for migration, but the compact header is now **wire schema v3**.

Current v3 semantics:
- `columns` / `rows` count internal guides;
- each axis stores `N+2` normalized positions including the 0/1 boundaries;
- new states keep boundary pins only;
- maximum accepted current topology is 50 internal guides per axis.

The decoder also recognizes older v1/v2 payloads where they can be migrated safely. The abandoned 2D-mesh payload is reset to a valid uniform separable grid rather than misrendered.

Unknown versions, oversized vectors, non-finite positions and noncanonical states are rejected.

## Keyframes

- Same-topology GridState keyframes interpolate guide positions linearly.
- Different topologies step at the midpoint.
- Columns/Rows remain authoritative for topology.
- Pin state is canonicalized to boundary-only state in the current parity model.

## Undo / redo

Viewer dragging writes GridState through `ArbitraryDef::set_value`, which marks the arbitrary parameter changed. The original-derived event flags are exactly `HANDLED_EVENT | UPDATE_NOW`.

Real Undo/Redo remains an After Effects runtime gate and is not claimed automatically.

## Migration rule

Do not change the six top-level serialized field types/order without an explicit migration. Increment the wire schema version for incompatible interpretation changes. Unknown versions must fail rather than render incorrectly.
