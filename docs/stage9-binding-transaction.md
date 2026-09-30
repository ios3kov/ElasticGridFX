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

## Opt-in native adapter experiment

`--cfg fstr_binding_probe --cfg fstr_lifecycle_probe` now enables an adapter and
four appended invisible research-only Point parameters (streams 24..27).
The default schema is unchanged. The fixture-only deferred callback initializes
only its newly added second effect, using one Undo group, checks exact readback,
then repeats the transaction to require AlreadyInstalled without another Undo.
Stream names/types/count, key state, effect match name and layer lock are checked.
Scoped stream references are reacquired for every access; no PF handles survive
the callback. Expressions derive sourceRect corners through thisLayer.toComp.
They do not depend on layer/effect display names and have no render consumer yet.

This is not production targeting: the existing exact fixture-name/active-layer
guard remains intentionally research-only. Never save this research-schema
project over user work. Original effect hidden streams must remain empty.
Native acceptance: automatic binding readback and expression evaluation,
idempotency, one native Undo clears all four expressions, Redo restores them,
remove second effect, close fixture unsaved, verify rollback to ordinary build.
Host check compiles; native acceptance is NOT RUN until separately recorded.

d5e8a32 / EGFX-6fd53cd4be4b6b47eb276759 native attempt (AE PID 51936):
add PASS, binding FAIL with Struct error and AE "bad tracked memory ID (23::34)".
Undo/Redo NOT RUN. Owned fixture closed without saving; receipt
EGFX-update-5d6795d913b243ebbda58be502e3f5a0 ROLLED_BACK, ordinary payload verified.
No successful write is evidenced. Wrapper GetExpression locks an unchecked
returned memory handle. Follow-up explicitly handles a successful null result,
bounds UTF16 decoding and balances non-null memory and suite calls. A mocked
null-result ABI regression and all research-config tests PASS (34/34). Native
repeat required; null handling is a hypothesis for this specific host error.

## Native hidden binding acceptance — 0db7077

Clean source 0db707786ffb0f8b91c4dc07b576a892eaaae354,
EGFX-d92302fb2f590dbb00d841c9, package SHA-256
371de04b4580f4258b53d5babfae13d2291aaa9d41bd9099af0ed40337c588e8.
AE 25.6.0 arm64, PID 52634. Research flags both enabled.

- Automatic installation and immediate idempotency readback: PASS.
- Expression evaluation on all four hidden corners: PASS.
- Original effect's hidden expressions remain empty: PASS.
- One native Cmd+Z clears all four; Cmd+Shift+Z restores all four: PASS.
- 2D -> 3D -> Y30 -> 2D, without rewriting expressions: PASS; finite values,
  rotation changes X and final 2D matches initial within 0.0001 pixel.
- Exact live image UUID/path/payload: PASS, same PID.
- Temporary second effect removal and fixture close without saving: PASS.
- Ordinary 1eed79a restored, payload verified; receipt
  EGFX-update-48f23f82e9944340890dd6f6bac83b58 ROLLED_BACK.

Evidence: outputs/binding-empty-0db7077, including journals, verify-fixed.txt,
undo.txt, redo.txt, transitions.txt, remove.txt and live-identity.json.
First verifier wrongly expected 27 JSX properties and reported FAIL; schema.txt
shows the additional native Compositing Options group at 28. The verifier now
checks that exact built-in match name. Original FAIL preserved; corrected
verification uses a separate report on the same immutable plugin. Node positive
and negative schema controls PASS. Research Rust tests 34/34 PASS. Scanner
review_required remains existing workflow/test findings, not security approval.

This confirms the research binding mechanism only. Production target discovery,
pending deletion/Undo cancellation, existing animated Grid Positions persistence,
save/reload, non-interactive first render, shared overlay/picking/render geometry,
camera/parent coverage remain NOT RUN or unimplemented. Stage 9 stays OPEN.
