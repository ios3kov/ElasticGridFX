# FSTR Stretch 0.9.3 — техническая ретроспектива и AE know-how

> **Historical checkpoint:** this retrospective covers the earlier 0.9.3 candidate
> only. Its release/signing verdicts describe the rules and state on 2026-10-01.
> See the [performance/release retrospective](retrospective-0.9.3-perf.1.md)
> for the current candidate and adopted rules 6.0.0.

Дата: 2026-10-01  
Scope: вся разработка ElasticGridFX / FSTR Stretch до принятого 0.9.3 development candidate.

Этот документ выполняет §25.1 DEVELOPMENT_RULES: он не пересказывает changelog,
а фиксирует знания, которые были получены на реальном цикле разработки и
проверки After Effects-плагина.

## 1. Точный финальный продукт, к которому относится ретроспектива

Принятый runtime candidate:

- source commit: `2ccc5f674b8b94b534d4c3ad6abdec3524e3624f`;
- Build ID: `EGFX-bd19dee13315abc0b7e6090e`;
- version: **0.9.3 development**;
- target: **macOS Apple Silicon / After Effects 2025 25.6**;
- ZIP SHA-256: `1448a5231fc561e6b10491d1b9d1839de22f21a11266a43e2990244a25176ebc`;
- binary SHA-256: `7dc57622442eaddcc1f061e044e77db3796a2db44e4061c807eee6aa60c15beb`.

Exact-candidate automated evidence: Rust 61/61, C++ 20/20, Python 239/239,
strict Clippy, PR CI and full macOS source/build/signature/package gate PASS.
See [Stage 10 final acceptance](stage10-final-acceptance-2026-10-01.md).

After acceptance, PR #10 → #12 → #15 were merged into `main`; the merge tree
after PR #15 was byte-for-byte identical to the accepted source tree. PR #16
then changed only documentation and `SHA256SUMS.txt`. Current merged docs head:
`be49dd9114e4a7bd57a6a021d905aa92c925579e`.

The current public GitHub release is still **0.9.1**. 0.9.3 is accepted and merged,
but is not yet a public release.

### Evidence labels used below

- **PROVEN / VERIFIED** — automated or instrumented evidence proves the stated fact in the named scope.
- **USER-REPORTED** — user confirmed the behavior in target AE.
- **OBSERVED / RESEARCH** — useful host/source observation, but not a complete production proof.
- **FAILED / REJECTED** — approach was exercised or reviewed and intentionally abandoned.
- **UNKNOWN / NOT VERIFIED** — no sufficient evidence; do not infer support.

## 2. Пять обязательных выводов

### 2.1. Что теперь точно известно, что можно и работает в After Effects

1. **Native effect + Rust host + C++ renderer is viable.** FSTR Stretch loads as a
   normal AE effect, renders through the native host, persists arbitrary parameter
   data and survives save/reopen in the accepted scope. **PROVEN / USER-REPORTED**.
2. **A CPU elastic grid warp can support 8/16/32 bpc, Bilinear/Final Bicubic,
   edge modes, animation, SmartFX and MFR-safe snapshotting** without changing the
   core pixel-quality contract. **PROVEN / VERIFIED** in regression; target-host
   behavior was repeatedly accepted through the development lineage.
3. **Projective deformation does not require a custom 3D engine.** A shared
   projective plane can drive render, overlay, picking and drag; layer/camera
   perspective can be expressed as a 2D projective mapping over the deformation
   plane. **PROVEN / VERIFIED** for the accepted square-pixel AE 25.6 scope.
4. **Native text can be handled**, but its effect-canvas and native-layer
   coordinate domains must be treated explicitly; the solution cannot be a fixed
   Position subtraction. **PROVEN / VERIFIED** through Stage 9 host probes and
   accepted product behavior.
5. **Guide density can be changed non-destructively** if visible controls are a
   read-only/derived view over retained animated deformation instead of a topology
   mutation. **PROVEN / USER-REPORTED**.
6. **RAM Preview invalidation and normal Render Queue cancellation/retry work**
   for the exact 0.9.3 candidate in the requested final scenarios.
   **USER-REPORTED** (#7, #8).
7. **Fit Layer can safely operate in public layer-boundary coordinates W/H while
   raster sampling remains W−1/H−1.** The two domains do not need to be collapsed
   into one convention. **PROVEN / USER-REPORTED** (#14).

### 2.2. Что нельзя, ненадёжно или не стоит делать

1. Do not call or mutate AEGP state from render/MFR workers just to “make binding
   happen”. Main-thread lifecycle and render-time ownership are different domains.
2. Do not assume deferred idle work has completed before AE asks for the first frame.
3. Do not use `UpdateParamsUi` as a hidden mutation hook; its safe role is UI state.
4. Do not derive native-text 3D alignment from a constant Position offset.
5. Do not assume `CompItem.activeCamera == null` means “no explicit camera layer”
   in AE 25.6.
6. Do not use `saveFrameToPng` as authoritative 32-bpc pixel evidence; this project
   reproduced host-side darkening independent of the effect.
7. Do not treat compact SmartFX storage bounds as the logical source canvas.
8. Do not change Columns/Rows by resizing or rewriting the animated Grid Positions
   value at the playhead.
9. Do not globally replace W/H with W−1/H−1 or vice versa. Public coordinates and
   raster indices are different contracts.
10. Do not mutate, re-sign, rename by rebuilding, or otherwise “repair” an already
    identified candidate in place. New bytes mean a new candidate and new evidence.
11. Do not infer what AE loaded from the file visible on disk. Duplicate plugin
    roots and stale processes make loaded-image identity a separate fact.
12. Do not call an ad-hoc-signed archive a finished public macOS distribution.
    Under the current development rules, Developer ID + notarization + clean
    Gatekeeper download/install remain mandatory.

### 2.3. Новые reusable know-how / patterns

- **Pending-neutral first frame pattern:** permit only a mathematically exact,
  dependency-complete identity frame while host binding is still pending; retain
  fail-closed behavior for every non-neutral or invalid state.
- **PreRender snapshot pattern:** SmartPreRender owns every value needed by
  SmartRender; worker render code never inspects mutable live parameter arrays.
- **Retained-state / derived-control pattern:** animation data is authoritative;
  UI density is a view. Display changes do not rewrite history.
- **Logical-canvas sparse rendering:** resolve sampling in logical source space,
  then map taps into compact storage; absent taps are transparent, not clamped
  compact-edge colors.
- **Shared projective mapping:** render, overlay, hit-test and inverse drag must
  use one coordinate model; separate “almost equivalent” UI math eventually drifts.
- **Boundary-domain separation:** public AE point coordinates, normalized plane
  coordinates and raster pixel indices need named conversions, not magic -1/+1.
- **Immutable-candidate evidence:** commit + Build ID + source digest + payload
  manifest + ZIP SHA + loaded image identity form one evidence chain.
- **Fail-closed host tooling:** uncertain project ownership, dirty state, loaded
  identity, file publication or callback result is BLOCKED/NOT RUN, never PASS.

### 2.4. Какие решения дали лучший результат

The highest-value architectural choices were:

1. preserving parameter IDs/wire compatibility and making changes append-only;
2. keeping the core renderer separate from AE host/UI lifecycle;
3. using inverse mapping and a single final source sample for projective render;
4. isolating host-dependent behavior behind targeted probes before production integration;
5. treating user-visible density as UI control topology, not storage topology;
6. creating exact-candidate Build Identity and package manifests early;
7. preserving failed runs and candidate boundaries instead of “upgrading” old evidence;
8. using bounded fixes that preserve unaffected render semantics.

### 2.5. Что нужно переиспользовать в следующих AE-плагинах

Reuse by default:

- BuildIdentity generation and exact package manifests;
- loaded-image UUID/path verification;
- test-owned AE fixtures with fail-closed ownership checks;
- source guards that first demonstrate the old failure;
- SmartFX dependency snapshots and worker-safe immutable data;
- strict separation of public coordinates from pixel indices;
- pre-sign version stamping from authoritative metadata;
- create-only packaging that preserves signed payload bytes;
- explicit USER-REPORTED vs VERIFIED evidence labels;
- reversible test installation and duplicate-plugin scans;
- a post-project retrospective before closing the engineering cycle.

## 3. Архитектурные выводы по рендеру

### 3.1. Sparse SmartFX: logical canvas matters more than storage rectangle

**PROVEN / VERIFIED.**

The compact-world baseline extended compact storage edge colors into regions that
were logically transparent. A dedicated regression measured thousands of differing
channels against an explicit zero-filled logical canvas.

Successful rule:

1. interpret output coordinates in the full logical source canvas;
2. resolve edge behavior there;
3. map only existing taps into compact storage;
4. treat missing logical taps as transparent;
5. never assume an arbitrary incomplete ROI is transparent unless the host contract
   establishes that it is complete.

This removed the bright/edge-stretch class of artifacts while keeping dense rendering
specialized and avoiding a full padded image.

Evidence: [sparse-render-results-2026-09-28.md](sparse-render-results-2026-09-28.md),
Stage 6/7 acceptance records.

### 3.2. SmartFX/MFR ownership

**PROVEN / VERIFIED.**

The reliable model is immutable per-request data:

- checkout every render dependency in SmartPreRender;
- copy/own the state needed by the render;
- SmartRender consumes that owned snapshot;
- no render-worker AEGP writes;
- cancellation remains an error/interruption path, never a fake successful frame.

This pattern was also essential to the first-application fix: the decision to render
an initial neutral frame had to be based on a fully checked snapshot, not a live
parameter array.

### 3.3. Projective plane architecture

**PROVEN / VERIFIED in the accepted scope.**

The successful conceptual model is:

`source = H(W^-1(H^-1(q)))`

where H maps plane-local coordinates into the current projected plane and W is the
elastic deformation. The important engineering consequences:

- use inverse mapping;
- solve/precompute the projective transform per frame, not per pixel;
- sample the original source once;
- keep the old nonprojected/separable path as an explicit fast path;
- use the same transform semantics for render, overlay and interaction.

A general projected warp is not representable by the old pair of independent 1D
screen-space LUTs without approximation.

Evidence: [perspective-3d-research-2026-09-29.md](perspective-3d-research-2026-09-29.md),
Stage 9 plane tests and target-AE acceptance.

### 3.4. Native text perimeter

**PROVEN / VERIFIED for the tested native-text scope.**

Four Corners is a bounded deformation region; automatic native Layer Plane is not.
Using bounded-region outside-quad pass-through for native text left fractional
antialiased perimeter pixels undeformed. The correct Layer Plane behavior extends
end-cell inverse mapping beyond the nominal vector bounds while Four Corners keeps
its bounded exterior semantics.

The first reopened-project export still showed stale cached output; after cache
invalidation/purge, the original repro matched the fresh duplicate. That produced a
second reusable lesson: renderer-semantic changes require an effect-version/cache
invalidation strategy, not just correct new code.

Evidence: [perimeter-residual-2026-09-30.md](perimeter-residual-2026-09-30.md).

## 4. AE lifecycle / AEGP know-how

### 4.1. Deferred binding is asynchronous

**OBSERVED / RESEARCH; behavior fix USER-REPORTED.**

The 0.9.1 first-application failure had a concrete source-level hazard:

- plane marker starts “pending”;
- binding is deferred from sequence setup to a main-thread idle callback;
- the old frame path rejected pending state with BadCallbackParameter;
- AE can request a frame before the deferred binding completes.

The exact failing selector/callback order in the user’s original session was not
instrumented, so the complete causal chain should not be rewritten as PROVEN.
What is proven is that the source had the race-compatible rejecting path and that
the narrowly guarded neutral-first-frame correction removed the user-visible failure.

Successful correction: allow only an exact untouched neutral identity state through
the existing identity renderer while pending; non-neutral, keyed, malformed or
otherwise unsafe pending states still fail closed.

Evidence: [first-application-516-2026-09-30.md](first-application-516-2026-09-30.md),
issue #9, PR #10.

### 4.2. Binding transactions must be explicit and reversible

**PROVEN / VERIFIED in research fixture; production use bounded by scope.**

The hidden binding research established a robust transaction pattern:

- reacquire target/stream identity for every operation;
- never target “current selection” as product identity;
- reject keyed/foreign/partial bindings rather than repair them opportunistically;
- use one Undo group;
- exact already-installed state is a no-op;
- write → readback → on failure restore only streams still owned;
- Undo itself is not rollback logic;
- null/unchecked host memory handles must be handled explicitly.

A first native adapter attempt produced AE “bad tracked memory ID”; fixing null
expression-memory handling led to successful automatic install, expression evaluation,
single-step Undo/Redo and 2D↔3D transitions in the owned research fixture.

Evidence: [stage9-binding-transaction.md](stage9-binding-transaction.md).

### 4.3. Main thread vs worker callbacks

**PROVEN / VERIFIED as an engineering constraint.**

Sequence/lifecycle callbacks may occur in contexts where AEGP access is not legal.
Research guards explicitly avoided AEGP calls from worker callbacks. The reusable
rule is stronger than “it usually works”: every host API used from render or lifecycle
code needs an explicit threading contract; otherwise move the operation to a proven
main-thread transaction and pass immutable results to rendering.

## 5. Coordinate systems: the largest source of AE-specific bugs

### 5.1. Native text effect canvas is not the same thing as native layer coordinates

**PROVEN / VERIFIED.**

Target AE showed a native 3D text overlay offset after enabling 3D. Host
`sourcePointToComp` probes showed Position being applied in the native layer
projection even at zero rotation. The grid domain, however, came from the effect
canvas. Applying the native layer projection to already transformed effect-canvas
coordinates double-counted a transform.

Rejected: subtracting Position by hand. It cannot survive rotation, parenting or
camera projection.

Useful research result: host-evaluated point expressions using
`sourceRectAtTime` + `toComp` can carry the perspective quad without render-thread
AEGP calls. The research proved feasibility and helped establish the eventual shared
mapping model; it was not used as permission to overwrite user Four Corners controls.

Evidence: [stage9-text-coordinate-baseline-2026-09-30.md](stage9-text-coordinate-baseline-2026-09-30.md).

### 5.2. “No camera” cannot be inferred from activeCamera nullness

**PROVEN / VERIFIED host quirk.**

AE 25.6 returned a non-null `CompItem.activeCamera` in a test composition that had
no explicit camera layer. Acceptance tooling therefore switched to a topology proof:
verify the owned composition contains no camera layer before camera creation.

Reusable rule: scripting convenience properties can expose a host/default-camera
concept that is not identical to project-layer topology.

Evidence: [stage9-chat-finalization-2026-09-29.md](stage9-chat-finalization-2026-09-29.md).

### 5.3. Public boundary coordinates vs raster indices

**PROVEN / VERIFIED in source/math; final visible alignment USER-REPORTED.**

The project found a real contract mismatch in Fit Layer:

- initial AE point defaults reached full W/H boundaries;
- Fit wrote W−1/H−1;
- deformed output differed;
- 1-pixel axes could collapse into degenerate fitted quads.

The correct bounded fix was to make Fit restore the public layer boundary:
TL(0,0), TR(W,0), BR(W,H), BL(0,H), while leaving raster sampling formulas alone.

Do not infer from this that sampler normalization should use W/H. Pixel-index
sampling legitimately uses extent−1. The domains need explicit conversion, not a
global “minus one cleanup”.

The remaining Layer Plane source-level convention difference was intentionally not
“fixed by theory”; final target AE visual alignment was confirmed by the user and
issue #14 closed without changing renderer sampling.

Evidence: [fit-layer-coordinate-fix-2026-09-30.md](fit-layer-coordinate-fix-2026-09-30.md),
issue #14.

## 6. Animation, arbitrary data, Undo and control density

### 6.1. Display density must not be storage topology

**PROVEN / VERIFIED; final behavior USER-REPORTED.**

The destructive baseline did two unsafe things:

- count-change handler rewrote Grid Positions;
- render snapshot logic resized/reset state from display counts.

That makes a UI choice mutate animation history and can create/replace a key at the
current playhead.

The accepted architecture:

- Columns/Rows are static control-density settings;
- count changes request redraw only;
- renderer does not consume display counts;
- retained Grid Positions remains authoritative;
- the UI derives a stable nested set of visible controls over the retained curve;
- inserted visual handles distribute edits into retained neighboring knots;
- only a real nonzero drag writes Grid Positions;
- reentrant density changes cancel the current drag instead of retargeting it.

Fewer controls hide detail but do not delete it. More controls expose extra grab
positions but do not create a new independent coefficient at every key.

Evidence: [grid-density-preserve-animation-2026-09-30.md](grid-density-preserve-animation-2026-09-30.md),
PR #12.

### 6.2. Rejected alternative: v4 DetailMap architecture

**FAILED / REJECTED.**

PR #11 explored an independent fixed 257-sample detail map and a v4 serialization
extension to provide extra local degrees of freedom. It was technically substantive
and tested locally, but it introduced a new compatibility boundary: old plugins
could not read newly edited v4 keys, and it expanded render/serialization architecture
for a requirement that could be met with the simpler retained-curve model.

The branch was explicitly closed as superseded by PR #12. Its main lesson is not
that richer detail storage is impossible; it is that a compatibility-expanding data
model should not be introduced when a simpler model satisfies the accepted UX.

Evidence: closed PR #11 vs accepted PR #12.

### 6.3. Parameter identity and wire compatibility

**PROVEN / VERIFIED.**

Existing parameter IDs/types/order and the legacy arbitrary-data shape must be
treated as project-file ABI. Additive parameters are safer than renames/reorders.
Unknown future schema versions should fail loudly rather than be silently interpreted.

Undo/Redo succeeds when edits flow through AE parameter transactions
(`set_value` + changed-value semantics) instead of hidden global mutation.

Evidence: [project-compatibility-v0.9.md](project-compatibility-v0.9.md), Stage 9/10 tests.

## 7. AE capture / automation know-how

### 7.1. ExtendScript File state can be stale after frame publication

**PROVEN / VERIFIED host/tooling behavior.**

The first Stage 9 automated run stopped at “Missing frame” even though the issue was
tooling, not plugin render. Reopening the File object and using a bounded publication
wait fixed the race.

Reusable rule: immediately cached ExtendScript file metadata is not sufficient proof
that a freshly generated frame is absent.

### 7.2. saveFrameToPng is not reliable 32-bpc reference evidence

**PROVEN / VERIFIED for AE 25.6 fixture.**

A later Stage 9 run exposed darkening in `saveFrameToPng` at 32 bpc. The effect was
reproduced independently without the plugin/import path, so the acceptance harness
moved to guarded Render Queue PNG capture.

Reusable rule: host convenience export APIs can have color/depth behavior distinct
from the production render path. Validate the capture mechanism itself before using
its pixels as an oracle.

### 7.3. UI interaction still needs host evidence

**PROVEN process lesson.**

JSX/source tests can prove ownership, state transitions and pixel captures, but they
cannot automatically certify native mouse drag feel, cursor semantics or the exact
interactive Undo experience. Keep those checks explicit instead of pretending an
export script covered them.

## 8. Build, identity, packaging and distribution know-how

### 8.1. Build identity must be independent from user-facing About

**PROVEN / VERIFIED.**

Support identity is stored in `BuildIdentity.json` and a diagnostic string. The
product About can stay clean. Build ID derives from clean source identity, target,
profile, toolchain and build settings; package SHA identifies the final external file.

This separation made it possible to simplify About without losing exact binary
traceability.

### 8.2. AE About return_msg is a legacy byte buffer, not UTF-8

**PROVEN / USER-REPORTED.**

Writing UTF-8 copyright bytes `C2 A9` to `PF_OutData.return_msg` produced `¬©`
in AE 25.6. A single legacy byte `0xA9` rendered the intended ©.

Reusable rule: never assume modern text encoding for legacy Adobe C ABI character
buffers. Test non-ASCII product strings on the actual host.

### 8.3. Version metadata must have one authority

**PROVEN / VERIFIED.**

Finder version was once hardcoded to 0.9.0 while BuildIdentity/AE version had moved
forward. The fix derives both `CFBundleShortVersionString` and `CFBundleVersion`
from validated BuildIdentity before signing, refuses already-signed mutation, and
verifies the values before and after package extraction.

Evidence: issue #13, PR #15, `tools/bundle_version.py`.

### 8.4. User-facing renaming need not mutate the signed payload

**PROVEN / VERIFIED.**

The product bundle can be delivered as `FSTR Stretch.plugin` while keeping internal
executable/effect identity unchanged. The safe packaging approach copies only
validated regular files, preserves bytes and executable modes, seals the renamed ZIP,
then verifies the exact payload again.

Do not rebuild just to rename the outer bundle.

### 8.5. Disk identity is not loaded identity

**PROVEN / VERIFIED.**

The development cycle encountered user/system duplicate plugin locations and needed
an explicit loaded-image identity chain. The robust diagnostic model combines:

- expected package/build identity;
- installed bundle verification;
- live process image UUID/path mapping;
- same AE PID where required.

A file on disk proves only that file exists, not that AE is executing it.

### 8.6. Public macOS distribution is still incomplete

**UNKNOWN / NOT VERIFIED for 0.9.3 public delivery.**

The accepted 0.9.3 development archive is ad-hoc signed. It is not Developer ID
signed/notarized and has not passed the new Gatekeeper-clean quarantined-download
distribution gate in DEVELOPMENT_RULES §28.

Therefore:

- functional Stage 10 remains complete in its development scope;
- public 0.9.3 release readiness is **BLOCKED** until Developer ID signing,
  notarization, applicable stapling, Gatekeeper validation and clean download/install
  are completed on the exact public artifact;
- no `xattr`, Open Anyway or Gatekeeper-disable workaround may be part of release instructions.

## 9. Diagnostic techniques that produced the most value

### 9.1. Negative fixture first

Before fixing sparse render, coordinate Fit, density behavior and first-application
logic, targeted tests were made to fail on the old source. This prevented “tests
that merely agree with the new implementation”.

**Reusable:** preserve one minimal baseline counterexample for every subtle host bug.

### 9.2. Exact production FFI, not a reimplementation

Coordinate and density tests called real production C++ FFI/render paths across
8/16/32 bpc rather than only comparing a Python model. This caught domain mistakes
that neutral frames would hide.

### 9.3. Immutable candidate chain

The most useful identity tuple was:

`commit → source digest → Build ID → signed payload files → ZIP SHA → installed bundle → live loaded image`.

Whenever any source change occurred—even About-only—the prior package evidence was
not transferred. That discipline caught the lifecycle-probe compile failure after the
About split before handoff.

### 9.4. Preserve failures

Failed CI, blocked AE runs and flawed verifiers were retained as failed records.
The later fix was recorded separately. This was essential for distinguishing plugin
defects from acceptance-tool defects.

## 10. Notable failed/rejected approaches

| Approach | Status | Why rejected / lesson |
|---|---|---|
| Blanket suppression of first-frame BadCallbackParameter | FAILED / REJECTED | Could hide genuinely invalid pending/deformed state |
| Fake “ready” plane marker before binding | REJECTED | Invents host state and breaks correctness |
| Poll/wait or AEGP writes from render worker | REJECTED | Wrong threading/lifecycle model |
| Mutating binding from UpdateParamsUi | REJECTED | UI callback is not a safe hidden state transaction |
| Fixed Position subtraction for native text | FAILED / REJECTED | Fails under rotation/camera/parenting |
| `activeCamera == null` as no-camera proof | FAILED | AE 25.6 returned a default/non-null camera concept without a camera layer |
| Immediate File metadata after frame export | FAILED | Stale ExtendScript metadata caused false missing-frame diagnosis |
| `saveFrameToPng` as 32-bpc oracle | FAILED | Host-side darkening independent of plugin |
| Compact-world clamp as logical-canvas sampling | FAILED | Smears storage edges into transparent logical regions |
| Resizing Grid Positions on Columns/Rows change | FAILED | Destructive to animation/keyframes |
| PR #11 v4 DetailMap for accepted density requirement | REJECTED | Unnecessary serialization/compatibility expansion |
| Global W/H↔W−1/H−1 rewrite | REJECTED | Conflates public boundary and raster-index domains |
| Editing signed candidate in place | REJECTED | Destroys exact evidence identity |
| Hardcoded Finder version | FAILED | Metadata diverged from real build version |
| UTF-8 © in legacy AE return buffer | FAILED | Rendered as `¬©` |
| Treating ad-hoc signature as public macOS release | REJECTED by current rules | Must pass Developer ID/notarization/Gatekeeper gate |

## 11. Remaining unknowns / unsupported scope

These are not defects inferred from absence; they are simply not certified:

- Windows;
- Intel macOS;
- After Effects versions other than the tested 25.6 scope;
- broad HDR/OCIO/linear-color workflows;
- physical GPU execution/parity in the accepted product; GPU dispatch is disabled;
- full non-square-PAR 3D UI matrix;
- per-character 3D text behavior;
- migration behavior for historical projects that actually animated Columns/Rows;
- exact instrumented callback site/order of the original 516 dialog;
- public Developer ID/notarized/Gatekeeper-clean 0.9.3 distribution.

Stage 8 performance optimization was **SKIPPED BY USER**, not PASS. Existing
benchmarks must not be turned into a speedup claim.

## 12. Final answers for future AE plugin work

### What should be copied directly

- build identity + exact artifact manifest pipeline;
- safe test installer / rollback / duplicate scan;
- live-image identity probe;
- SmartFX immutable snapshot pattern;
- test-owned AE project guards;
- production-FFI pixel regressions across bit depths;
- projective render/overlay/picking unification;
- retained-state/derived-control density model;
- bounded coordinate-domain conversions;
- evidence labels and immutable-candidate discipline.

### What should trigger immediate suspicion in future projects

- render code reading live UI state;
- any hidden write caused by a display-only control;
- any “just subtract one pixel” coordinate fix without a domain definition;
- any AEGP call without a documented threading/lifecycle reason;
- any native text solution based only on normal raster-layer assumptions;
- any test oracle that has not itself been validated at the target bit depth;
- any “installed = loaded” assumption;
- any packaging step after signing that changes payload bytes;
- any public macOS release instruction containing a security bypass.

## 13. Evidence index

Primary records:

- [Stage 10 final acceptance](stage10-final-acceptance-2026-10-01.md)
- [Development stages](development-stages.md)
- [Runtime acceptance tooling](runtime-acceptance-2026-09-28.md)
- [Sparse render result](sparse-render-results-2026-09-28.md)
- [Stage 9 finalization](stage9-chat-finalization-2026-09-29.md)
- [Native text coordinate baseline](stage9-text-coordinate-baseline-2026-09-30.md)
- [Binding transaction](stage9-binding-transaction.md)
- [Native text perimeter correction](perimeter-residual-2026-09-30.md)
- [First-application 516 investigation](first-application-516-2026-09-30.md)
- [Animation-preserving density](grid-density-preserve-animation-2026-09-30.md)
- [Fit Layer coordinate correction](fit-layer-coordinate-fix-2026-09-30.md)
- [Project compatibility](project-compatibility-v0.9.md)
- issue #7 — RAM Preview final-candidate acceptance
- issue #8 — cancel/re-render final-candidate acceptance
- issue #9 — first-application blocker and acceptance
- issue #13 — bundle version metadata
- issue #14 — coordinate audit / final Layer Plane acceptance
- PR #11 — rejected DetailMap/v4 alternative
- PR #12 — accepted animation-preserving density
- PR #15 — accepted 0.9.3 candidate
- PR #16 — Stage 10 documentation-only closure

## 14. Retrospective verdict

FSTR Stretch demonstrated that the difficult part of an AE native effect is not the
warp formula alone. The highest-risk failures came from **host semantics**:
lifecycle timing, coordinate domains, SmartFX sparse buffers, cache identity,
project serialization, custom UI interaction, capture APIs, plugin discovery and
packaging metadata.

The successful development pattern was therefore:

**separate pure render math from host state → instrument the host narrowly →
preserve project ABI → make host-derived data immutable before rendering →
pin every candidate exactly → verify on the real host → retain negative evidence.**

The final 0.9.3 development candidate is accepted for its documented macOS Apple
Silicon / AE 25.6 scope and merged to main. The engineering retrospective is now
recorded. The remaining product-level blocker before a normal public macOS 0.9.3
release is the newly required **Developer ID + notarization + Gatekeeper-clean
distribution gate**, not a known functional defect in the accepted AE scenario.
