put('.github/workflows/first-application-regression.yml', [
    (5,1,r'''    branches: [fix/first-application-516, feat/grid-density-independent]
'''),
])
put('.github/workflows/macos-source-gate.yml', [
    (5,1,r'''    branches: [chore/github-ci-v1, fix/original-gridwarp-parity, fix/smartfx-param-snapshot, fix/final-validation, fix/first-application-516, feat/grid-density-independent]
'''),
])
put('CHANGELOG.md', [
    (1,0,r'''
## 0.9.2-dev — non-destructive control density (not released)

- Keep Columns/Rows changes out of Grid Positions key data and render geometry.
- Derive evenly subdivided/thinned virtual controls over the preserved cage.
- Preserve untouched same-density drag behavior; extra controls edit independent local geometry.
- Keep old keys byte-identical until an actual edit; v4 adds a fixed-size detail map.
- Compose detail coordinates before the original CPU/plane sampler, with no extra image resampling.
- Invalidate the old count-dependent AE cache version; target-host acceptance pending.
'''),
])
put('CMakeLists.txt', [
    (185,0,r'''
# Count-independent edit field: production coordinate maps and samplers.
add_executable(elasticgrid_detail_tests tests/test_detail.cpp src/bridge/elasticgrid_ffi.cpp src/bridge/plane_ffi.cpp)
target_link_libraries(elasticgrid_detail_tests PRIVATE elasticgrid_core)
target_include_directories(elasticgrid_detail_tests PRIVATE src)
add_test(NAME elasticgrid_detail_tests COMMAND elasticgrid_detail_tests)
'''),
])
put('README.md', [
    (0,6,r'''> **Development branch:** non-destructive control density (0.9.2 candidate).
> Columns/Rows change the visible controls, not existing keys or rendered
> deformation. Additional controls edit a count-independent refinement field.
> New detail edits use backward-readable wire v4; keep old-project backups.
> Native AE acceptance is still pending.
> [Current status](docs/current-status.md) · [Specification and limits](docs/non-destructive-density-2026-09-30.md).
'''),
])
put('docs/architecture.md', [
    (51,0,r'''
## Non-destructive control density (development candidate)

`guide_density` separates transient viewer controls from the persistent GridArb
shape. Render reads only canonical data; count events only request redraw.
Additional controls edit a fixed 257-sample monotone material-coordinate map
per axis, independent of visible counts. Rendering composes its inverse after
the legacy inverse warp and samples the source once. Empty maps preserve the
exact historical path. Unedited keys stay wire v3; v4 appends actual detail
geometry. Fixed-basis detail interpolation never changes other stored keys. See non-destructive-density-2026-09-30.md for
exact semantics, tests and outstanding native acceptance. Earlier staged/planned
sections above are historical; current-status.md governs release status.
'''),
])
put('docs/current-status.md', [
    (1,0,r'''
## Development: non-destructive control density — 2026-09-30

Stage 10 remains INCOMPLETE. New branch `feat/grid-density-independent` builds
on 7f72201 without modifying PR #10's immutable candidate, main or the release.
Columns/Rows now control a read-only view over stored Grid Positions;
count changes never rewrite a key or resize rendering data. Additional guides
edit an independent, fixed-size material-coordinate field only on real drag.
Unedited keys retain byte-identical wire v3; detail edits use wire v4.
Older plugin versions cannot read v4 edits: preserve original projects.
Source tests and candidate/host boundaries: [density specification](non-destructive-density-2026-09-30.md).
Version 0.9.2 is a development candidate. Exact-head Rust/Clippy/macOS build and
native AE acceptance must be recorded separately; no previous PASS is reused.

## Historical checkpoint boundary

The static-count package below still reset axes on count changes; that behavior
is superseded by the new requirement and must not be presented as accepted.
'''),
])
put('docs/non-destructive-density-2026-09-30.md', [
    (0,0,r'''# Non-destructive guide density — 2026-09-30

## Stage 10: implementation candidate; no target-AE PASS

User approved: changing Columns/Rows changes control density throughout an
existing animation. Insert between old guides or evenly hide guides; never
reset deformation, create/update a Grid Positions key, or edit other keys.
This supersedes the reset policy in static-grid-counts-2026-09-30.md.

Baseline source: 7f7220168421491fe1cbe3ebf74041810c5ebb52, tree
c0a5cbe8b35bbe62ecc601a4e63c8e4a2c7c3259. Last user-confirmed host remains
94d6706. Main, PR #10's candidate and published releases are not modified.
Rules reviewed: FSTR-Line DEVELOPMENT_RULES.md blob
701a8c1ae3acb4dbfe1d7eda94acbf8095b88608.

## Reproduced source defect and change boundaries

Two old paths were destructive: UserChangedParam wrote a resized Grid Positions
value; both ordinary and SmartFX reads resized that value using display counts.
Six source checks against the baseline produced five assertion failures.
This is source evidence, not an instrumented native reproduction.

Count events now request only redraw and end an in-flight guide interaction.
They do not read/write Grid Positions, enumerate keys, acquire AEGP suites,
migrate data, or launch a deferred operation. Counts remain CANNOT_TIME_VARY.
Rendering reads a validated owned canonical snapshot with NO count input.
Only a real guide drag may write the current Grid Positions value/key.
An exact no-op or zero-strength drag writes nothing.

## Geometry retained separately from controls

The legacy GridArb vectors remain the base deformation. A read-only view derives
the visible guide coordinates. An increase retains the base anchors and divides
selected intervals equally, using balanced integer allocation of extra guides.
A decrease selects a quantile-spaced subset, but never deletes hidden geometry.
Distribution is deterministic relative to the saved base, not a history of
previous count-slider gestures. Restoring a count restores the same controls.
The rendering result is independent of all these display choices.

Simply projecting more handles onto a four-node cage would not add local editing
freedom. That limited prototype was rejected before a build/handoff. Instead,
additional guides edit a **fixed 257-sample monotone map per axis in material
coordinates**, independent of visible counts. Its local support is finer than
the supported 1–50 guide layouts. At zero radius a new handle can move without
moving the old anchors; regression tests require this, not just handle motion.
The original base vectors and other keys are not rewritten by such edits.

Let B be the original source-to-destination warp and H the new material map.
The composed forward warp is B(H(u)); rendering evaluates H^-1(B^-1(x)) and
samples the original image once. No intermediate raster or quality reduction.
Empty H selects the exact old path. Easing and waves still act on the retained
base, so changing visible counts does not alter their amplitude or interpolation.
Viewer positions and pointer inversion use the actual C++ mapping, not a
separate approximate renderer. 2D wave controls now use the evaluated base too.

Existing same-density handles, before any detail edit, retain the original
elastic-drag implementation. Once detail exists, intentional edits use its
fixed basis. The edit solver preserves order/boundaries, normalizes the grabbed
handle response, bounds large pointer motion and validates before mutation.
Detail interpolation is pointwise on the fixed basis; no other keys are visited.
Displayed counts and base topology are latched in drag refcons; a change cancels
that interaction instead of silently targeting a different guide.

## Serialization, compatibility and identity

No saved parameter IDs/types/order or legacy render ABI is changed. The 160-byte
render ABI and 152-byte plane ABI remain frozen; new opt-in detail entry points
use a separately checked 32-byte map record and owned synchronous buffers.

With empty detail maps, serialization stays **byte-identical six-field v3**.
Only an actual extra-control edit appends two bounded geometry maps using v4.
Unflatten accepts v0–v3 and v4; malformed, oversized, nonfinite, nonmonotone or
future-version data is rejected. A count change never upgrades a key's wire.
The native host uses bincode legacy configuration; its COPY/FLATTEN/UNFLATTEN
paths are tested by exact roundtrips with the same serializer configuration.

**Old plugin versions cannot read newly edited v4 keys.** Keep original AEPs and
save new test work separately; restoring an old plugin is not a v4 down-converter.
Old projects with already animated counts or mixed base topologies are not
silently repaired. Existing legacy interpolation behavior for those keys remains;
target-host migration acceptance is still required. No migration deletes keys.

Version is 0.9.2 DEVELOPMENT, not a release. Incremented effect version prevents
reusing cached frames from the old count-dependent renderer. Named bundle stays
FSTR Stretch.plugin. A new commit/Build ID/hash is mandatory for any candidate.
GPU dispatch remains disabled; any accidental GPU detail request fails rather
than rendering without the detail field. No new platform or HDR/OCIO claim.

## Checks and acceptance criteria

Required before candidate handoff:
- Source guards prove count callbacks cannot write keys or resize render state.
- All 1–50 density pairs produce ordered guides; growth/thinning cycles retain
  canonical bytes. Count changes on/interpolated between keys preserve them.
- Same-density legacy drag parity; new local detail independence; no-op safety;
  extreme/invalid input; v3/v4 roundtrips and bounded malformed decoding.
- Production CPU 8/16/32-bit and plane mapping, identity parity, actual detail
  response, cancellation/retry and concurrent deterministic rendering.
- Pixel equality after count changes, including active detail, animated base,
  waves and easing. Counts never enter the render API.
- Strict C++ analysis, Rust/Clippy, Python/Node safety, sanitizers, signed build,
  repository hashes and immutable package identity.

Local Linux C++: 20/20 PASS, including new production detail FFI checks.
Strict high-warning audit and Clang Static Analyzer PASS. Rust and the native
macOS bundle require CI; final results belong to the exact resulting commit.
Local source/mocks/CPU tests are not target After Effects verification.

Native AE 25.6 Apple Silicon acceptance (NOT RUN by developer):
create Grid Positions keys; change each count at a key and between keys; capture
key count/times/values and frames across the timeline before/after. Both must
remain unchanged. Check 4→9→1→50→4, one axis and both; then drag an added handle
and verify a real local deformation without a jump or writes to other keys.
Check 8/16/32 bpc, Full/Half, text/solid/precomp, 2D/3D, Undo/Redo, save/reopen,
first-add, preview and cancellation for the exact candidate. Compare decoded
pixels, not PNG metadata or an export alone as proof of preview correctness.

`tests/ae_density_invariance.jsx` provides a guarded test-owned capture fixture
using ordinary neutral Grid Positions keys plus procedural wave animation.
It checks key count/times and output captures; it does not pretend to construct
nontrivial custom arbitrary values or certify the entire host gate. The script
returns CAPTURED_NOT_FULL_ACCEPTANCE and must run in a controlled later host
turn after automatic plane binding. No purge, dialog suppression or user-file
cleanup is performed. User project originals remain untouched.

## Primary references

- https://ae-plugins.docsforadobe.dev/effect-basics/PF_ParamDef/
- https://ae-plugins.docsforadobe.dev/effect-details/arbitrary-data-parameters/
- https://ae-plugins.docsforadobe.dev/effect-ui-events/PF_EventUnion/
- after-effects 0.4.0 `pf/parameters.rs`: bincode legacy COPY/FLATTEN/UNFLATTEN,
  arbitrary interpolation and host-owned handle lifecycle.

No implementation/source or hosted test closes Stage 10 by itself. Earlier
94d6706 user-reported successes stay historical and are not relabeled as this
candidate's results. Merge and release require separate authorization.
'''),
])
put('docs/static-grid-counts-2026-09-30.md', [
    (0,0,r'''> **Superseded behavior:** the user subsequently required non-destructive density.
> The reset-on-manual-count-change behavior described below is historical, not
> the new contract. See [non-destructive density](non-destructive-density-2026-09-30.md).

'''),
])
put('host-rust/Cargo.lock', [
    (212,1,r'''version = "0.9.2"
'''),
])
put('host-rust/Cargo.toml', [
    (2,1,r'''version = "0.9.2"
'''),
])
put('host-rust/build.rs', [
    (187,1,r'''            bugversion: 2,
'''),
])
put('host-rust/src/binding_probe.rs', [
    (66,1,r'''fn initial_identity_values(grid:&GridArb,mode:i32,
'''),
    (70,1,r'''    if mode != 1 || wave.to_bits() != 0.0f64.to_bits()
'''),
    (86,3,r''''''),
    (94,1,r''''''),
    (98,3,r'''    // The display counts are intentionally not dependencies of this decision.
    // Only the original canonical value can prove the exact initial identity.
    let matches=|grid:&GridArb|initial_identity_values(grid,mode,wave,easing,spacing);
'''),
])
