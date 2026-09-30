# Automatic binding transaction — isolated implementation

Stage 9, 2026-09-30. Research implementation, not production integration.
`host-rust/src/binding_transaction.rs` is compiled under tests only. Shipping
parameter schema, renderer, UI, installed plugin and project data are unchanged.

## Requirements and boundaries

- BT-1: initialize exactly four dedicated hidden coordinate streams, never
  Grid Positions or the public Four Corners controls. Existing saved IDs remain.
- BT-2: exact expected active binding is a no-op, including no Undo entry.
- BT-3: keyed, foreign, disabled or partially initialized bindings are conflicts;
  no opportunistic repairs or overwrites.
- BT-4: identify the effect independently of active selection, validate again
  after entering Undo, and reject deletion/replacement/lock or schema mismatch.
- BT-5: on a write/readback failure restore touched streams where still owned;
  the failing setter may already have mutated the stream. Foreign changes and
  new keyframes during the operation must survive attempted recovery.
- BT-6: close a successfully opened Undo group exactly once on all Result paths.
  Failure to restore or close is explicit, never PASS. Undo itself is not rollback.

The Host adapter must enforce stable identity on every operation and map errors
to Result. The transaction assumes normal Result-returning host calls, not panic
or process-crash recovery. No host handles or selected-layer shortcuts exist in
this pure module. Expression strings are supplied by the future versioned
binding generator; mocks deliberately do not simulate AE expression evaluation.

## Verification scope

Unit cases exercise initial install/repeat, all eight partial setter failures,
foreign/disabled/keyed/partial conflicts, deletion before writes and recovery,
changes while opening Undo, begin/end errors, readback/recovery errors, and a
foreign expression appearing during a failed write. These prove the transaction
policy against a deterministic adapter, not real AE Undo or binding persistence.

Executed on macOS arm64 with Rust 1.98.1, release/offline/locked: 33/33 tests
PASS (9 transaction cases plus 24 existing tests). Static code audit retains
review_required (exit 1), existing workflow/test findings; no security approval.
Real AE adapter, native Undo/Redo and automatic binding remain NOT RUN.

## Next integration gate

1. Add append-only hidden research parameters and a versioned expression source,
   without enabling a production renderer dependency yet.
2. Implement main-thread adapter with scoped suite context and reacquired target
   identity. Do not copy the earlier active-layer/named-fixture read probe into
   production targeting. Parameter writes are forbidden in UpdateParamsUi/render.
3. On an owned native fixture verify automatic install, readback, repeat,
   Undo/Redo, deletion before idle, saved-project reload, and Grid Positions keys.
4. Resolve initialization before the first non-interactive render; idle availability
   is not guaranteed and stale coordinates must never be silently rendered.
5. Only then connect the evaluated coordinates to shared render/overlay/picking
   geometry and run the required 2D/3D, rotation, camera and bit-depth checks.

No promise that an idle-based strategy satisfies production requirements yet.
Stage 9 stays OPEN. Stage 8 remains skipped by user; no GPU/performance work.
