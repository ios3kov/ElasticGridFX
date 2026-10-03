# Live influence: bounded design investigation

2026-10-03. U5, rules8.0.0 / Critical / Development. The earlier sections retain historical proposals; the superseding decision and
implementation below are current. Acceptance is scoped by the exact candidate record. Local-only
boundary and all other U0–U10 obligations remain. [Scope](feature-backlog.md).

## Required behavior and current cause

Affected Lines must change the current deformation while the user edits it.
The later user decision hides Follow Shape, Follow Strength, Smooth Stretch and
Smooth Width and sets unified new-instance defaults; saved legacy values remain.
Wave Animation remains a separate group.
Changing scalar controls must not rewrite Grid Positions keys. Reset changes
only current-time Grid Positions; AE owns key insertion and Undo/Redo.

Current ui::drag_inner mutates retained axis positions through the real C++
eg_control_axis_drag. Radius/strength are inputs to that gesture, not retained
causes of the deformation. Later render snapshots receive baked positions;
changing radius/strength cannot reapply missing edit intent. UPDATE_NOW or extra
render requests alone would not resolve this mathematical dependency.

## Reversible investigation before integration

A possible new state keeps a valid base axis and separately editable move
coefficients, evaluating the influence kernel from current scalar parameters.
Old saved states do not contain the source moves. Reconstructing them from the
baked grid is not an exact migration, especially after monotonic projection,
zero strength/radius, or successive drags. Do not infer gesture history or
silently convert old animation keys. The user explicitly rejected new-edits-only behavior on2026-10-03: Affected
Lines must also respond on old baked deformation. Preserve the initial saved
appearance and existing keys; investigate a geometric field response rather than
claiming recovery of missing gesture history. This supersedes the earlier pending
legacy question. The numerical representation is still unaccepted/unimplemented.

The legacy virtual references are nested source-knot selections/subdivisions,
not arbitrary pointer coordinates. For each retained topology, all permitted
visible densities1..50 use a subset of the same maximum-density catalog. This
limits editable reference coefficients to50 per axis rather than an unbounded
list of gestures. A real Rust/C++ FFI regression checks all2500 combinations of
base topology and density, exact float reference bits, and immutable saved bytes.
This proves only the legacy catalog bound, not the proposed deformation algorithm.
The newly requested visible-space density reflow produces arbitrary source
references depending on the current deformation. It invalidates the assumption
that every editable handle belongs to this catalog. A bounded driver-knot or
other validated representation is required before live influence integration.

Candidate representation: immutable base positions, canonical endpoint pins,
a bounded finite coefficient representation per axis (bound/design still open), and an explicit wire-version tag.
Grid Positions remains the single animated arbitrary stream. No extra animated
parameter or platform-specific state. Do not append fields to bincode's existing
six-field struct without a version-specific reader/writer. Encoding within
versioned axis payloads is an option to investigate; no v4 format is accepted or
written by the current plugin. Legacy v0/v1/v2/v3 readers remain unchanged.

## Open numerical requirements

The evaluator must be pure, bounded and shared by overlay/hit test and render
snapshot preparation. Zero moves must return exact base bits; fixed endpoints
and finite strictly ordered axes must hold at every supported topology. Kernel
profile ordinals and the existing zero-strength semantics must be explicit.
Fractional reference normalization must match the real control reader.

Linear superposition followed by monotonic projection is not automatically
identical to sequential projected gestures. Saturation must not retain invisible
overshoot that makes reverse dragging appear unresponsive. Resolve these cases
before choosing the coefficient update rule; do not ship an approximate inverse
of a poorly conditioned influence matrix. Slider changes must not mutate base,
coefficients, animation keys, or rendering quality.

## Integration and acceptance after decisions

1. Prove numeric slider response, finite/error rejection, cancellation, saturated
   drag reversal, fractional handles and density changes using the actual core.
2. Define/freeze versioned bounded serialization and mixed legacy/new-key
   interpolation. Reject malformed lengths/tags before publishing state.
3. Satisfy the confirmed old-deformation response and document downgrade limits before
   writing a new saved format. Preserve old output and keys in the agreed scope.
4. Feed evaluated axes into immutable native-plane/SmartFX snapshots and the
   viewer from one implementation; retain thread ownership and caches.
5. Check current-time Reset, Undo/Redo, save/reopen, scalar/key expressions,
   8/16/32-bpc Final, first application and MFR on exact Mac/Windows candidates.
6. Draw affected-range feedback only inside valid host callbacks. U7's unselected
   and playback overlay feasibility remains a separate dependency.

## Superseding migration decision and first implementation block

The user explicitly chose immediate application of existing Tension Radius
animation on2026-10-03. Exact old appearance on opening is no longer required;
saved Grid Positions and scalar keys must remain intact. No baseline clone,
new parameter stream, gesture reconstruction or new wire format is needed.

Implement a bounded geometric field: subtract the uniform source position from
each retained knot, smooth those displacements with a normalized compact
Smoothstep kernel whose radius is the current Affected Lines value, add them back
to uniform positions, pin endpoints and project to safe spacing. Radius<=1 keeps
the baked shape exactly; neutral axes remain bit exact. Increasing radius spreads
local changes and can soften their magnitude. This is a new geometric behavior,
not a recovery of the old gesture history or a quality reduction in sampling.
Use retained topology units; changing visible Columns/Rows never changes this
field. Apply before Wave and existing interpolation/sampling on the shared core.

An explicit internal render-ABI flag selects the new evaluator for the updated
host; legacy core parity callers remain on their frozen old route. The AE stream
ID/default/range and saved-grid six-field encoding stay unchanged. Both normal
UI evaluation and immutable SmartRender snapshots enable the same flag. Dragging
must solve against the same evaluated field (including Wave/easing), rather than
applying raw pointer delta to the baked axis. Bound the scalar search, store only
valid projected axes, and verify fractional references, saturation/reversal,
animation input, neutral/reset and nonmutation before native acceptance.

Status: shared evaluator and live drag solver implemented on76015ca. Mac Dev4
loaded identity, immediate old animation response, scripted pixel changes and
keys/save/reopen PASS in the bounded owned fixture. Ordinary Dev8 native drag/Undo/Redo and save/reopen also PASS in the owned
fixture after correcting registration. Mouse scalar scrubbing and affected-range
drawing remain unaccepted; Windows NOT RUN. Previous
coefficient/wire-v4 proposals are superseded. U7 remains a separate open gate.
See [native evidence](live-influence-mac-dev4-2026-10-03.json).

[Ordinary Dev8 native evidence](live-influence-mac-dev8-2026-10-03.json).
