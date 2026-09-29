# Current development / release status

2026-09-30. Branch: fix/final-validation, draft PR #5. **NOT READY FOR RELEASE.**

**Production cycle: Stage 9 of 10 REOPENED; Stage 10 BLOCKED by text-layer 3D overlay regression; Stage 8 SKIPPED BY USER, not PASS.**

## Latest product decision — perspective REGION, 2026-09-30

### Current installation and text-fixture safety checkpoint

Installed candidate is now **1eed79aad052f144668c2ebed9426a7934f35f6b**,
Build ID **EGFX-25e03a7ae1a9304311095c8e**. Receipt
EGFX-update-82e493859f9b47ab950b0b4821a9f27a is INSTALLED_FOR_TEST;
1232ad2 is retained for rollback. Installed payload reverified against manifest.
After AE restart, native screenshot confirms Grid Positions and its stopwatch
visible in Effect Controls, with no diagnostic text. This supersedes the pending
installation statements below. Exact live-image identity and new keyframe
roundtrip on this candidate are NOT RUN; do not reuse 1232ad2 pixel PASS as new evidence.

The formerly owned text-plane.aep path now opens an image/adjustment-layer
composition, as seen during that UI check. Preserve it; it is not a valid native
text regression fixture anymore. Do not infer ownership from a historical path.
The text probe now also requires run marker, expected dimensions/layer count,
native text property, expected effect, unlocked layer and boolean toggle input
before mutation. Existing path/name checks remain. Node negative guards and
positive control PASS; static audit review_required with existing workflow/test
findings. No renderer/UI projection changes or new plugin installation this step.

Next: create a fresh uniquely owned native-text fixture in a clean project and
capture effect-canvas versus layer-plane coordinates for rotation/camera cases.
The 3D text displacement is still unresolved; Stage 9 stays OPEN.

### Grid Positions correction requested by user

Hiding the entire Effect Controls row was a misunderstanding. Restore its
CONTROL flag and native animation stopwatch; remove only the custom diagnostic
text. Keep parameter ID, arbitrary data, interpolation and saved keys unchanged.
This supersedes NO_ECW_UI statements below. Focused source regression requires
visible CONTROL, rejects NO_ECW_UI/INVISIBLE and requires a blank custom draw.
Host contracts 9/9 and Rust release tests 24/24 PASS, including arbitrary
interpolation and wire roundtrip. Scanner grid-control-audit.json retains
review findings, not security approval. Current AE screenshot confirms missing
ECW row while Timeline Grid Positions and existing keys are still present.
Native installation/visual verification of this correction: BLOCKED by open
unsaved user project (Untitled Project *, image composition with animated grid).
Do not close/discard it. Save user work before replacing/restarting AE.
Prepared clean release candidate 1eed79aad052f144668c2ebed9426a7934f35f6b,
Build ID EGFX-25e03a7ae1a9304311095c8e, arm64. Bundle signature, entrypoints,
PiPL branding and sealed payload/package identity PASS. Package SHA-256:
e25903bb11c1b1726aca5af3d90dbc0babe92ca412258ac8e3b79d395384da8b.
Artifacts: workspace work/grid-control-1eed79a. Fresh AE check still shows
Untitled Project *; installed build is unchanged. Runtime verification NOT RUN.

### Installed test checkpoint — 1232ad2, FSTR Stretch

Candidate `1232ad2dda62c1cdf2b2667d525c71b3c409d66c`, Build ID
`EGFX-5d92143dc7e660b4f19ba4a9`, package SHA-256
`cabece0f9f9d5d4ec919d06af16d6be48b48fc63f5fdb0b102c3297e64c5752c`.
Clean isolated build, bundle verification and sealed package identity PASS.
Rollback-safe installation receipt `EGFX-update-a6c9c4f36bc047c2a96a6886abb402b3`
is INSTALLED_FOR_TEST; previous 099e492 bundle is retained as previous.plugin.
The owned text fixture was saved before normal AE shutdown; no user project was discarded.

Real AE 25.6.0 acceptance `EGFX-PLANE-2bec9679489749318662e8cf24ed4f8d`:
exact loaded identity PASS in PID 34893, 40 frames / 32 checks PASS,
contract perspective-region-v1, owned fixture cleanup CLEAN.
At 8/16/32 bpc, neutral skew identity max error is 0; tested exterior samples
remain exact while the interior changes under deformation. Same-build AEP
roundtrip and raster-layer 3D render checks PASS. Native AE menu inspection
confirmed Effect > FSTR Effects > FSTR Stretch.
Report: workspace outputs/stretch-region-acceptance/EGFX-PLANE-2bec9679489749318662e8cf24ed4f8d.zip.
The report's old scope prose still mentions a projective ramp; its comparator
contract/results are regional. Future runner scope text is corrected; original
evidence is preserved unchanged.

This is NOT text-layer 3D overlay acceptance. That regression remains unresolved;
Stage 9 is OPEN and release BLOCKED. Timeline animation visibility / hidden ECW
row have source-contract coverage, not fresh native visual verification.
Tooling plane tests: 18/18 PASS. Static scanner remains review_required, not
security PASS. Historical installed checkpoints below are superseded by this one.

User-facing branding: **FSTR Stretch**, AE category **FSTR Effects**.
The user selected Stretch after considering alternatives; this supersedes
FSTR ElasticGrid. Internal match name, install path and saved IDs are retained.
Stretch rename verification: generated PiPL contains FSTR Stretch / FSTR Effects
and unchanged com.elasticgrid.fx.warp; Rust 24/24, host contracts 9/9 and build
identity tests 19/19 PASS. Scanner fstr-stretch-name-audit.json remains
review_required (exit 1), existing workflow/test findings. New package and
installation evidence is recorded above; 099e492 is now a rollback checkpoint.
PiPL display name/category and About are updated in source. Match name
`com.elasticgrid.fx.warp`, parameter IDs and serialized grid data are unchanged.
Grid Positions uses NO_ECW_UI (not INVISIBLE): hide the redundant diagnostic
row in Effect Controls while retaining its Timeline animation track. Build ID
is kept in About; it is no longer drawn by the grid-control label fallback.
Native menu/category verification passed; animation-track visual verification remains open.
Historical branding check: generated PiPL resource contained FSTR ElasticGrid, FSTR Effects and unchanged
com.elasticgrid.fx.warp (inspected after compilation). Rust 24/24 PASS without
the removed unused Build ID constant warning. Source contracts now require
NO_ECW_UI and reject hiding the Timeline track; they retain the About marker.
Scanner fstr-branding-region-audit.json: existing workflow/test review items,
review_required (exit 1), not a release approval.
Full Python suite after updating the obsolete short-Build-ID UI assertion:
207/207 PASS. Previous run's failure was that superseded UI requirement; the
replacement verifies hidden ECW diagnostics, retained animation and branding.

User explicitly withdrew full-image Corner Pin projection. Four Corners now
defines only the perspective region for guide deformation: neutral guides are
exact identity; non-neutral guides warp within the plane; exterior stays intact.
Direct native 3D text remains required. This supersedes older projection claims
and the former zero-wave projection acceptance oracle below.

Implementation: additive eg_render_plane_region reuses PlaneWarp::prepare and
the existing 8/16/32-bit samplers. Rust Four Corners render calls this regional
entry, not projected/between. Removed unused source projection snapshot fields;
old additive core/ABI APIs remain for compatibility tests, not product behavior.
Native pixel comparator now identifies its contract as perspective-region-v1:
neutral skew equals original; wave changes the interior but preserves exterior
RGBA. Old native Corner Pin rendering remains diagnostic only, not a product
reference. Synthetic positive and negative controls pass (10 tests). Applying
the new comparator to retained 099e492 frames yields FAIL for neutral identity
and regional locality at all three depths, as expected; historical reports were
not overwritten. New native regional verification passed as recorded above.
New package/install/native regional verification: PASS. 3D text host integration is
still incomplete; no claim that this change resolves its overlay displacement.

Local verification: CMake/CTest 18/18 PASS; Rust 24/24 PASS. New real bridge
checks cover skewed neutral identity for all depths, both qualities/all edge
modes, unchanged exterior under guide movement, and preserved row padding.
Rust-to-C++ test confirms neutral skew identity through the new ABI.
Scanner perspective-region-audit.json: review_required, exit 1, existing
workflow pinning/credential and test-auth heuristic findings. Not security PASS.

2026-09-30 update supersedes the acceptance below: the user supplied a screenshot
showing the Layer Plane overlay displaced immediately after enabling 3D on a text
layer, without rotation. Build label is EGFX-97a79761c3f9. The previous native UI
fixture covered a solid, not text; its PASS cannot establish text-layer alignment.
Root cause is not yet confirmed. Investigation: reconcile effect canvas coordinates
with native layer coordinates in ui_projection.rs; do not apply guessed offsets.
Required regression: owned text fixture, 2D -> 3D -> 2D, then rotation/camera,
overlay and inverse picking agreement, with the solid fixture retained.
Fresh host reproduction was initially BLOCKED by unsaved user changes. After
permission, AE was already a clean empty project; the agent did not close or save
the former user project. Owned baseline now REPRODUCED (FAIL):
`outputs/text-plane-3d-20260930/text-plane.aep`, run 8aca677833f245ab9133529a719d3097.
640x480 text at [320,200], Layer Plane, zero wave: switching only threeDLayer to
true moves the overlay by [320,200] while the text stays fixed. Screenshots were
inspected in chat before and after, both script transports returned 0.
New `ae_text_plane_probe.jsx` preserves the reproduction and refuses foreign work.
Research: Adobe documents that text is always continuously rasterized and that
continuous rasterization applies transforms before effects, unlike raster layers.
Sources: https://helpx.adobe.com/after-effects/desktop/add-text/create-and-edit-text-layers/creating-editing-text-layers.html
and https://helpx.adobe.com/no/after-effects/desktop/work-with-layers/manage-layers/layers.html .
This supports an effect-canvas/native-layer space mismatch; merely subtracting
position would not establish rotated/camera-aligned deformation. Next work must
reconcile render-space and UI-space contracts, not only move the drawn overlay.
No replacement artifact has been built or installed for this regression.

### Direct native text decision and core work — 2026-09-30

User explicitly selected direct native 3D text support, not precomposition.
Core `PlaneWarp::prepareBetween(source, destination, ...)` now composes
`Hsource(gridInverse(HdestinationInverse(pixel)))` in one sampling pass.
It handles already projected input independently from the destination quad;
legacy rectangular-source and regional paths remain unchanged. This is core
groundwork ONLY: no AE host caller, new package or installed fix yet.

Evidence: new mapping test failed to compile before the API existed; after
implementation CMake build and CTest 18/18 PASS, including projective source
mapping, nonuniform guides, transparent destination exterior and float sampling.
`git diff --check` PASS. Scanner `work/text-plane-dual-projection-audit.json`
returns review_required (exit 1): existing workflow pinning/credential findings
and test-auth heuristic; not a security certification.

Remaining required work: obtain an evaluated source/destination projection
snapshot through supported AE calls, establish render-thread safety and camera/
layer cache dependencies, bridge it to the core and use the same geometry for
overlay/inverse hit testing. Do not reuse stale UI matrices during render or
silently move only the overlay. Then build an identified candidate and verify
the native text reproduction, rotation, camera, dragging and pixel regression.

### Native projection bridge checkpoint — 2026-09-30

Added `eg_render_plane_between` without changing the 152-byte frame ABI.
Invalid source/destination quads return bad input without writing output, unlike
the retained legacy invalid-plane pass-through. Existing sampling/bit-depth paths
are reused. Rust State owns optional source corners, cloned with the frame
snapshot, and routes them to the new bridge when present. The provider is NOT
implemented: State::read still sets None; this does not fix native text yet.

Verification: CTest 18/18 PASS including 8/16/32-bit bridge parity and rejection
without output writes; Rust 24/24 PASS including real C++ linkage and frozen ABI;
diff whitespace check PASS. Scanner text-plane-bridge-audit.json remains
review_required, exit 1, existing workflow/test heuristic findings.

Integration risk confirmed by SDK guide: AEGP threading is main-thread-only,
while PFInterface is described as accessible to effects. This does not establish
that the existing UI Layer suite calls are permitted during SmartPreRender/MFR.
Do not move UI calls directly into render or use an idle/UI cache as a frame
dependency. Required next research/prototype: supported evaluated transform
acquisition and dependency invalidation in interactive AND aerender execution.
Sources: https://ae-plugins.docsforadobe.dev/aegps/implementation/#threading and
https://ae-plugins.docsforadobe.dev/effect-details/accessing-camera-light-information/ .
No candidate package, installation or new native-AE PASS claimed.

Historical acceptance: user replied “работает” to the combined remaining manual checks on 2026-09-30.
Drag/Undo/Redo and old animated-project acceptance are USER-REPORTED PASS, not
automated evidence. Installed candidate remains 099e492. The latest gate ledger is
`stage10-final-gate-2026-09-30.md`; older Stage 9 OPEN statements below are historical.
See development-stages.md. Stages 1–7 are completed for the current scope. Stage 8 is
performance/profiling (skipped on explicit user request), Stage 9 is the approved 2D/3D perspective plane, Stage 10
is final compatibility/release validation.
No main merge or production release in this stage. User authorized a reversible
test installation of the guide UI update, contingent on candidate checks and a
safe AE restart (never discard unsaved work).

## User-machine evidence now received

### Latest user feedback — Stage 9 corrections required

User confirmed basic drag/Undo/save behavior but supplied screenshots showing
Four Corners does not project the image and the overlay does not follow the 3D
layer. Therefore the historical automated PASS below does not establish the
intended plane behavior. Requirements, five UX corrections and remaining tasks
are recorded in `stage9-user-corrections.md`. New core implementation is in
development; installed candidate is now 099e492 (test-only, baseline pixel gate PASS). UI source includes
square-pixel 3D camera projection/inverse hit-testing, Layer Plane naming,
mode-dependent corner disabling and reordered controls with Render Quality last.
Visual alignment and mode-dependent targets are now verified in the owned square-pixel
scene; historical static AEP values survive loading/save/reopen. Drag/Undo, old
keyframed AEP migration and non-square projection remain unverified. Stage 9 remains OPEN.

### Consolidated target-AE verification — 2026-09-29

Installed 099e492 remains unchanged. Controlled-color run
`EGFX-PLANE-9a2ea8679da941f4855d9ea42bd1be20`: exact loaded identity PASS,
40 frames / 29 plugin checks PASS, cleanup CLEAN. The inherited project was
`sRGB IEC61966-2.1`, linearize=true. Setting ONLY the owned fixture project to
None / linearize=false reduced 8-bpc coordinate error from ~0.024958 to
0.001974; 16/32 bpc ~0.0000242. Thresholds unchanged. This establishes color-context
dependence, not full linear/OCIO/HDR certification. Historical failures retained.

Owned visual scene `outputs/plane-ui/EGFX-plane-833892011d9e4898844914c13837b054`:
AE screenshots inspected in this chat show grid/handles aligned to a 640x480 solid
at 75% scale / Y rotation 35 degrees, first with default camera, then Z-rotated
parent (15 degrees) and shifted explicit camera (+80,-35). Layer Plane has disabled
corner/reset fields and no corner targets; Four Corners enables fields/targets.
Render Quality is last; displayed Build ID matches installed candidate.
Visual verification only, not numeric alignment tolerance or drag verification.
Mouse drag automation failed with `noWindowsAvailable`; no success inferred.
An initial follow-up script refused a canonicalized path different from the exact
owned path (Foreign project). Reusing the original exact path succeeded; ownership
guard not weakened. Cleanup report CLEAN. No user project modified.

Historical migration: copied de31498-owned AEP from run
`EGFX-PLANE-531656cd2c5243818311ac9b1dcb152e` to fresh output directory;
`outputs/plane-migration/d46bf522cf0b4423938d757bf3e2faf2/migration.json` PASS,
cleanup_clean=true. Plane mode, four corner values, wave amplitude, grid dimensions
and quality preserved through load/save/reopen with 099e492. Original unchanged;
source AEP hash and installed identity recorded alongside report. This historical
fixture has no corner keyframes, so old animation migration remains NOT RUN.

Remaining Stage 9 acceptance: native guide drag/Undo/Redo and legacy keyframed AEP.
Stage 10 final compatibility gate is not closed. No additional features or
performance work added. New probe safety mocks reject foreign-project mutation.

### Current installed checkpoint — 099e492; baseline pixel gate PASS

2026-09-29: half-resolution source endpoint correction rebuilt from clean commit
`099e49208ebe70a81707dbe905f291e139b9eeb0`, installed through the same atomic
updater after normal AE/Dynamic Link exit (no force kill). Previous f842e8d retained.

- Build ID: `EGFX-97a79761c3f9f35751e06853`.
- ZIP SHA-256: `4c48ca09c83d764e36c2640c8aff971c0de616876fde75ee0ab0a9127773ef10`.
- Receipt: `EGFX-update-ce1f0338f91f4d7e8c4bb8c5fdb2ca45`, INSTALLED_FOR_TEST.
- Clean build, bundle signature/payload, 18 CTest, 24 Rust, 201 Python,
  Clippy and bridge ASan/UBSan PASS. Scanner findings unchanged (review required).
- Real AE run: `EGFX-PLANE-9acd8e0934084142ba899906bbab8724`.
  Exact loaded identity PASS; 34 frames / 26 comparisons PASS; cleanup CLEAN.
- Half identity vs effect-off: max/mean difference **0** at 8/16/32 bpc.
  Existing comparator thresholds unchanged. Full-resolution identity also PASS.
- Evidence: local `outputs/plane-acceptance/EGFX-PLANE-9acd8e0934084142ba899906bbab8724.zip`.
- CI at last inspection: five workflows PASS, macOS source gate queued; not
  presented as all-green CI for this candidate.

### Expanded zero-wave perspective check — FAIL under investigation

Run `EGFX-PLANE-8af1eccd07904f00a0257b54f5d79051` tested unchanged installed
099e492: loaded identity PASS, 37 frames / 29 checks, 28 PASS, cleanup CLEAN.
The new independent inverse-homography oracle checks a smooth horizontal source
coordinate ramp and transparent exterior, explicitly rejecting stationary content
and stationary content with a quad mask. Local comparator/safety tests: 15 PASS.

16/32-bpc zero-wave skew checks PASS (maximum coordinate errors ~0.000243 and
0.0000835). 8-bpc check FAIL: maximum 0.024958 exceeds the predefined 0.02 bound;
at output (5,82), expected green ~0.024958, actual 0. Other worst samples jump to
~0.050996. This may involve host low-light quantization, but cause is NOT proven.
No threshold change or plugin change made. Exterior alpha zero in all depths;
the historical 26 checks still PASS. Overall expanded acceptance remains FAIL.
Evidence: local `outputs/plane-acceptance/EGFX-PLANE-8af1eccd07904f00a0257b54f5d79051.zip`.
Next: controlled native-effect/reference test to distinguish host quantization
from plugin sampling before changing implementation or acceptance tolerances.

### Native Corner Pin comparison — diagnostic evidence, not acceptance PASS

Run `EGFX-PLANE-7320acf3e2ed43488cf15d7582114f2d` on unchanged 099e492 adds
three native Corner Pin frames (40 total). ElasticGrid is disabled for each native
capture; native points use the same quad, then the reference effect is removed
and ElasticGrid is reacquired/enabled. Loaded identity PASS, cleanup CLEAN.

The independent oracle also fails for **native Corner Pin at 8 bpc**:
maximum error 0.026762 versus ElasticGrid's 0.024958. Native 16/32 bpc maximum
error ~0.008018, both PASS; ElasticGrid 16/32 bpc also PASS. Exterior alpha zero.
Native source-boundary/sampling conventions differ, so exact pixel equality is
not asserted. This demonstrates the low-light discrepancy is not unique to
ElasticGrid, but does not establish its precise color/quantization cause.

The 0.02 bound and overall FAIL are preserved. Reference diagnostics cannot turn
a failed plugin assertion into PASS. Evidence is the same-named local ZIP under
`outputs/plane-acceptance/`. Next: record/control the owned project's color context
and isolate quantization before revising any oracle assumptions. The production
plugin and installation were not changed.

Stage 9 remains OPEN: native 3D overlay alignment/hit testing, native corner target
visibility, legacy AEP migration and explicit zero-wave skew validation remain.
Baseline 3D image-change checks are not overlay evidence. No release/main merge.

### Historical installed checkpoint — f842e8d; functional FAIL

2026-09-29: user authorized installation. AE exited normally; Dynamic Link
exited subsequently without force termination. Atomic replacement completed
`INSTALLED_FOR_TEST`; previous de31498 retained under receipt
`EGFX-update-ba1c8a7114f74c3a973cb019c4169496`. Installed payload/signature PASS.

- Commit: `f842e8d3c5e4656c4f9494bebdd4cd8881b362e7`.
- Build ID: `EGFX-12fde3e033eb6c086655828f`.
- Package SHA-256: `37a6a563644718c508306d9f7008828c483868ab05bc051a8dc1957cb74f955a`.
- Exact-commit GitHub workflows: all six completed successfully.
- Real AE run `EGFX-PLANE-2f689f7df8144029982a2ff55304d759`:
  loaded identity PASS; 34 frames; 23/26 comparisons PASS; cleanup CLEAN.
- **FAIL**: Four Corners identity at Half Resolution differs from effect-off
  original at 8/16/32 bpc (mean absolute difference ~0.0303–0.0306,
  max ~0.614–0.616). Full-resolution identity comparisons PASS.
- Evidence: local `outputs/plane-acceptance/EGFX-PLANE-2f689f7df8144029982a2ff55304d759.zip`.
- Runner explicitly pins both historical and current candidates; no identity
  override or pixel threshold change. Six runner control-flow tests PASS.

Source correction: preserve `(full layer dimension - 1) * downsample` as an
owned render-snapshot source extent, rather than using rounded raster size minus
one. Added explicit projected bridge entry point; old ABI and entry points are
unchanged. A 128x96 half-resolution identity regression failed before this change
and passes with the explicit 63.5x47.5 extent (both qualities, all edge modes).
Local CTest 18/18 and Python 201/201 PASS; Rust ABI coverage also tests explicit
extent and invalid input. Scanner retains existing workflow/test review findings.
This is source evidence only; installed f842e8d remains the failed candidate.

Next: create a NEW immutable candidate and repeat target-AE acceptance. Native overlay alignment,
legacy AEP migration and zero-wave skew acceptance remain separate open gates.
Do not treat baseline 3D render changes as proof of overlay alignment. Stage 9
remains OPEN; release BLOCKED. Failed candidate remains installed for diagnosis.

### Historical installed checkpoint — de31498

2026-09-29: user explicitly requested installing the new build, recording current
status and pushing all current source updates. The earlier attempt was stopped
before replacement because Adobe dynamiclinkmanager was still running. On this
attempt the normal host guard passed; no process was killed or guard bypassed.

- Plugin commit: `de314981005606741bc75c517d8bb33798b46a1d` (clean arm64 build).
- Build ID: `EGFX-0fa68430a170b3612e8d00f7`.
- ZIP SHA-256: `6a43f734c7dc5fd298b356b12db75ab5986c171a2ae241ef9c85ffc0ac22ec55`.
- Atomic updater: **INSTALLED_FOR_TEST**; installed files and signature verified.
- Rollback receipt: `EGFX-update-371ef6e56f92407d8b04b16af52e2c10`.
- Previous installed de0becf is retained in that operation's backup.
- Candidate/evidence: `work/plane-candidate-de31498/` (local, not a public release).
- AE was not launched during installation. Subsequent real target-AE automated
  acceptance is now **PASS**: 34 frames, 26 numeric checks, loaded identity PASS,
  cleanup CLEAN. Native viewer dragging plus UI Undo/Redo remain separate gates.

Implemented in this candidate: Four Corners/Existing Grid mode, four native points,
Fit Layer reset, shared render/overlay/inverse-drag geometry, evaluated wave guides,
8/16/32-bit sampling with both qualities and all edge modes, invalid-plane original
image fallback. Existing Grid remains the default; previous parameter names/types
and grid wire schema remain unchanged.

Source verification: CTest 18/18, Rust 22/22, Clippy (existing macro exceptions),
geometry ASan/UBSan, clean build, signed bundle and package identity PASS. Existing
static workflow/test audit findings remain review items, not a release PASS.

Overall: Stages 1–7 completed for their historical scope; Stage 8 skipped by user;
Stage 9 in progress; Stage 10 not started. Persistent deselected viewer grid is
deferred by user. GPU dispatch remains disabled. No main merge or release.
Next: remaining native viewer drag/Undo/Redo acceptance, delegated to the user.
Installation success and source-tooling PASS do not close Stage 9.

### Stage 9 automated target-AE acceptance — PASS; native interaction remains open

Run `EGFX-PLANE-531656cd2c5243818311ac9b1dcb152e` on AE 25.6.0 arm64:
34 frames, all 26 numeric comparisons PASS, exact loaded de31498 identity PASS,
same AE process, installed payload verification and cleanup CLEAN. No plugin
rebuild/reinstallation. Release remains BLOCKED; Stage 9 is not fully complete.

The earlier 34-frame run was blocked by `saveFrameToPng`: even a native white
solid with no import/effects exported as ~0.1 at 32 bpc, versus 1.0 via Render
Queue at the same depth. The fixture now uses an owned one-frame Render Queue,
verifies PNG/straight RGBA/16-bit output template settings, preserves actual
project depth, and verifies Full/Half output resolution. No pixel normalization
or comparator threshold reduction. Full frames are 128x96; Half frames 64x48.
PNG comparisons do not establish HDR/negative-float fidelity.
See `stage9-depth-probe-2026-09-29.md` for diagnosis and evidence.

### Earlier tooling and attempt history

The Stage 9 acceptance path is now automated around the already installed immutable
candidate de31498; no plugin/runtime source changed after that candidate. The runner
pins commit `de314981005606741bc75c517d8bb33798b46a1d` and Build ID
`EGFX-0fa68430a170b3612e8d00f7`, and immutable ZIP SHA-256
`6a43f734c7dc5fd298b356b12db75ab5986c171a2ae241ef9c85ffc0ac22ec55`.
It verifies the installed payload/signature and the live loaded image in the same
AE process, and refuses unsafe/saved user projects. The executable launcher
`RUN_ELASTICGRID_PLANE_AE_TEST_MAC.command` performs no install/replacement and
requires one empty unsaved target-AE project.

The owned fixture captures **34 frames**: 24 comparisons across 8/16/32 bpc for
Existing Grid vs Four Corners identity, wave, skew, invalid-plane and half-resolution
behavior; two AEP save/reopen roundtrip frames; and eight 3D cases covering layer
position, scale, rotation, parenting, active-camera movement, camera switching and
no-camera fallback. `tools/plane_smoke_pixels.py` performs numeric comparisons;
`CAPTURED` alone is never PASS. The runner closes only its exact saved test project
and must prove a fresh empty project after cleanup.

Source/control-flow evidence: Final Validation PASS (199 Python/source tests plus
18/18 CTest), PR CI PASS (GCC, Clang, TSan, ASan/UBSan, static analysis), and
macOS source gate PASS, including the fast acceptance-tooling regression, Apple
toolchain/Metal compilation, full preflight and signed install-free bundle.
The acceptance regression now refuses both an altered candidate ZIP and a manifest
whose package digest is not the known de31498 SHA-256. Validated tooling checkpoint:
`0291bb807daa8f7df689831d3c6e00c7ac84908b`; macOS workflow run `36616083315`.

First real Stage 9 target-AE attempt: report
`EGFX-PLANE-94219e58cc354fadae128742a2ccbdb4` was **BLOCKED** during the
first 8-bpc frame publication (`pixels_8`, `d8-original`). The JSX called
`saveFrameToPng` successfully but immediately reused stale ExtendScript `File`
metadata and reported `Missing frame d8-original`; no frame entered the numeric
matrix. Identity and pixel status therefore remained **NOT RUN** and this attempt
is not a plugin/render FAIL. The fixture fail-safe reset only its owned test state.

Tooling fix `724001cc6772d5c4fdc6015a66bb138a3144d8b8` now matches the proven
Stage 7 capture behavior: recreate the `File` object and wait up to 50 x 100 ms
for a non-empty PNG, while rechecking project ownership. It never retries the
render, kills/restarts AE or touches user work. Final Validation PASS, PR CI PASS,
and macOS source gate run `36620529285` PASS for this fix. The installed/runtime
candidate remains immutable de31498; no `src/` or `host-rust/` file changed.

Second real Stage 9 target-AE attempt: report
`EGFX-PLANE-814b2dbd8b67486c8f657c0d6ca1855c` captured **33 of 34**
frames successfully: the complete 8/16/32-bpc plane matrix, AEP save/reopen,
3D layer position/scale/rotation/parenting, camera movement and camera switching.
The fixture then blocked only at its final synthetic no-camera assertion
(`No-camera state not reached`) after disabling already-created camera layers.
Identity/pixel aggregation therefore remained **NOT RUN** and this is not a
plugin/render FAIL.

Tooling fix `e6dbc26409644f2202a6349ecf2124ae01f31b8a` now captures the
no-camera/Default Camera fallback **before any camera layer is created**, then
creates cameras for the remaining 3D cases. Frame ordering and the external
comparator were updated together; the old disable-cameras assumption was removed.
Final Validation PASS, PR CI PASS, and macOS source gate run `36623152320` PASS.
The installed/runtime candidate remains immutable de31498; no `src/` or
`host-rust/` file changed.

Third real Stage 9 target-AE attempt: report
`EGFX-PLANE-77f143f3385447ebbb2ce00e80548756` captured the complete
24-frame 8/16/32-bpc plane matrix plus both AEP roundtrip frames (**26 frames**),
then blocked before the no-camera frame because target AE 25.6 returned a non-null
`CompItem.activeCamera` despite the owned composition containing no camera layer.
Identity/pixel aggregation therefore remained **NOT RUN**; this is not a
plugin/render FAIL.

The scripting guide documents `activeCamera == null` when no enabled camera
layers exist, but target-host evidence contradicts that assumption. Tooling fix
`b327964ce218055c7c1dfffe2baabae5e1b18227` no longer uses
`activeCamera` to prove the Default Camera case. The owned fixture instead proves
its topology directly (`comp.numLayers === 1` and that sole layer is the test
footage), captures `3d-no-camera`, then creates Camera A/B for the remaining
cases. Final Validation PASS, PR CI PASS, and macOS source gate run
`36625654300` PASS. Runtime de31498 is unchanged.

A fresh controlled target-AE rerun is still required before any 2D/3D/camera
runtime PASS claim. Native viewer corner/guide dragging and UI Undo/Redo remain
the final manual interaction checks because they require real host UI event
semantics rather than scripted parameter assignment.

### Stage 9 native four-corner source integration

Appended native Four Corners/Existing mode, four points and Fit Layer. Existing
mode is the default and retains old parameter IDs, wire schema and rendering.
Four Corners is now wired to both render callbacks with owned SmartFX snapshots,
shared wave evaluation, common geometry for drawing and inverse dragging, native
corner grips and invalid-plane diagnostics/pass-through. No second camera
projection is applied to ordinary layer-space rendering. See the P9-3 coordinate
contract in perspective-plane-plan.md; host-coordinate assumptions need AE tests.
18 CTest, 22 Rust tests, Clippy and geometry ASan/UBSan PASS. Existing static audit
findings remain outside changed scope. The Stage 9 fixture/runner now covers the
34-frame matrix described above, but real target-AE execution remains NOT_RUN.
The installed runtime candidate is still immutable de31498; all later changes are
tests/tools/workflows/docs only. Stage 9/10 are not complete.

### Stage 9 perspective sampling modes

Plane core and additive `eg_render_plane_sampled` now support Draft Bilinear and
Final Catmull-Rom with Clamp/Wrap/Mirror. Existing frame ABI and the original
Final/Clamp entry point remain unchanged. Index reduction occurs in double before
integer conversion; mirror periods use int64. Sparse storage is still transparent
outside the checkout, with edge modes resolved on the logical canvas.
Release CTest 18/18 and sampling ASan/UBSan PASS. The new independent scalar oracle
covers all six mode pairs in 8/16/32 bpc, skewed/off-canvas planes, negative/HDR
float values, sparse/full equivalence, padding and invalid-mode rejection.
Evidence: work/stage9-core/plane-sampling-{sanitized,audit.json} and CTest logs.
Static audit: existing workflow/test review findings only; not a release PASS.
This remains core/ABI evidence, not native controls or AE perspective acceptance.

### Stage 9 native plane bridge

Additive EgPlaneFrame/EgPlaneImage/eg_render_plane bridge compiled in Cargo and
exercised by a real Rust-to-C++ identity/invalid-plane ABI test. Existing render
dispatch, parameter IDs, project wire schema and installed de0becf are unchanged.
Checks: Release CTest 17/17, Rust 20/20, Clippy and bridge ASan/UBSan PASS.
Static audit retains existing workflow/test findings, no changed-scope findings.
Evidence: work/stage9-core/{plane-bridge-sanitized,plane-bridge-audit.json,
Testing/Temporary/LastTest.log}. Bridge contract and remaining modes are in
perspective-plane-plan.md. Next: quality/edge completeness, then append the
four-corner controls and connect immutable host snapshots/overlay/render.
Stage 9 and real AE plane acceptance remain incomplete. No release claim.

### Persistent viewer grid request

**DEFERRED BY USER** to a future version. Requirements and GridWarp reference
are preserved in persistent-viewer-grid.md. It is no longer a blocker for the
current 2D/3D plane and final compatibility scope. Do not implement it now.

User confirmed a toggle for passive grid visibility after deselection, hidden
during playback and absent from all rendered/cached frames. Current effect-only
DRAWBOT API is selection-dependent. Research and scope are recorded in
persistent-viewer-grid.md. Architecture decision required before an interactive
renderer/overlay prototype; no fake toggle, pixel burn-in or installation made.
Installed de0becf and its accepted appearance remain unchanged.

### Stage 9 continuation: expanded perspective canvas

User accepted the thinner guide/intersection appearance of installed de0becf
("так лучше") and requested continuing. This accepts that UX feedback only,
not the entire camera/plane or release matrix. Installed candidate is unchanged.

Standalone PlaneRenderer now supports expanded source/output storage with signed
origins. Outside the logical canvas is transparent zero; interior uses the same
single Final Bicubic sample and all previous bit-depth behavior. Checked int64
arithmetic prevents signed origin overflow. No new UI or native renderer dispatch.
Release CTest 16/16 PASS; expanded-canvas ASan/UBSan PASS on macOS arm64.
First test compile failed due to mixed const pointers in an initializer list;
fixed with explicit std::array<const PlaneWarp*,3> and reran successfully.
Static audit retains existing workflow/test findings, none in changed scope.
Evidence: work/stage9-core/Testing/Temporary/LastTest.log,
plane-expanded-sanitized, plane-expanded-audit.json. Real AE plane acceptance
remains NOT_RUN. Next: connect the native plane snapshot/bridge while preserving
existing parameter IDs, legacy renderer and saved-project compatibility.

### Stage 9 requested guide UI affordances (source implementation)

Latest installed test candidate: **de0becfd770a57af458c4776360961fe9cdc3556**,
Build ID **EGFX-69dc027b98ab5b10bf9222fd**, package SHA-256
`56bbd5d791118cc0c13f143aeb43517544bcef59d7af70300190f58c88d65bac`.
Clean offline arm64 build, bundle signature/manifest checks and postcommit 19/19
Rust tests PASS. User authorized interrupting AE for replacement; saved their
open project, quit normally, and atomic updater returned INSTALLED_FOR_TEST.
Rollback receipt: EGFX-update-7dde400314364ea1989cebf37d511531 (previous d26419c).
Runtime loaded identity and visual acceptance remain NOT_RUN, explicitly left
to the user per their request. No release/Stage 9 PASS is claimed.
Evidence: work/guide-candidate-de0becf/{build.log,tests.log,verification.log,
ElasticGrid.artifact.json}; source and packaging checks are not AE visual QA.

Historical follow-up preparation:

Follow-up: d26419c was installed with rollback retained (Build ID
EGFX-7726b4f7add8cb5e7329ee90; package SHA-256
fff53c06678ca531fba80789b404ce2fb1b4d384f215e39ae77ee6ebfaa0800c).
User acceptance rejected overly thick lines and fixed central gaps. New source
halves guide width (1.5 outer / 0.5 inner), slims grips, and recomputes vertical
cuts at every transformed horizontal intersection, including during drags.
Visual AE acceptance is explicitly delegated to the user at their request;
it remains NOT_RUN for the follow-up candidate, not PASS. Renderer unchanged.
Source verification: 19/19 Rust release tests PASS, Clippy PASS with existing
macro exceptions. Static scan has existing workflow/test findings, no changed-UI
findings (work/stage9-core/intersection-guide-audit.json). Real AE visual checks
remain with the user. Replacement waits until their color dialog/project work
is safely finished; d26419c is still installed at this source commit.

Prior implementation/install records:

Latest scope: elongated grips/gaps matching the user reference, hover-only hand,
and white/black outlined guides. See guide-ui-v0.5.md for criteria and research.
Source implementation complete; Rust release tests 17/17 and Clippy PASS (existing
documented macro lint exceptions). Static scan requires review for existing
workflow/test findings, none in this UI change. Rust strip warning is the known
toolchain libLLVM lookup issue, not a passed packaging gate.
New real-AE acceptance is BLOCKED: the open baseline project has unsaved user
changes; no discard/restart attempted. Installed baseline is 7c421ab,
Build ID EGFX-3b99c0fe65c3192ed33a8baf,
package SHA-256 f8190bb652af2daaa1da45bc5b5b71ab0fec53f7d73b917ec062bb33b6e5d10e.
The user reports preliminary success with this baseline; not a complete matrix PASS.
Release remains blocked; no new installation in this source-edit step.

Historical records below describe prior candidates, not the latest installed state.

Real AE 25.6 test of installed efa9bf2 displayed the larger markers but exposed
a modal `PF_CursorType invalid (0)` warning on deactivation. Acceptance FAIL.
PF_SetCursor must not receive NONE on this target: reset now requests Arrow,
and the Mac AppKit CUSTOM sentinel is returned only through AdjustCursor, never
sent to the App suite. A new candidate and real-AE retest are required.

Installation preparation found a pre-existing Release test defect:
test_metal_parity.mm placed the CPU reference render inside assert, so NDEBUG
removed the render and compared Metal against zeros (max_abs 1.50966, exit 4).
Moved the call to explicit checked execution without changing tolerance or any
renderer code. All 18 parity cases now PASS, max_abs <= 4.76837e-7.
Evidence: work/stage9-core/guide-install-metal-parity{,-fixed}.log.
This is standalone Metal evidence, not AE acceptance; installation still pending.
The determinism fixture had the same NDEBUG side-effect defect (reference renders
skipped). Assertions now remain enabled in all three Metal test translation units,
without changing production compile flags. Optimized sequential 384/concurrent
256 determinism runs PASS; parity rerun PASS. These fixes invalidate neither the
recorded failure nor the requirement to retest a clean final candidate.

2026-09-29: markers enlarged from 5 to 10 frame units, retaining screen-space
hit testing. Active viewer cursor uses open hand; guide capture uses closed hand
on Mac via a main-thread-checked AppKit shim and AE CUSTOM cursor protocol.
Mouse release/error/invalid topology and context exit/deactivation clear state.
Details and required live scenarios: guide-ui-v0.5.md.

Local arm64 release build PASS; Rust tests 15/15 and CTest 15/15 PASS.
Use explicit Rust 1.98.1 compiler path: default PATH Cargo/rustc 1.80.1 cannot
build edition 2024 (initial attempts failed, corrected without changing tools).
Static audit: review_required, existing workflow findings and test-auth heuristic;
no findings in changed UI/shim/build files. Report: work/stage9-core/guide-ui-audit.json.
Real AE visual/cursor acceptance NOT_RUN: installed immutable fd69988 unchanged.
No installable artifact is published and Stage 9 remains IN PROGRESS.

### Stage 9 initial geometry foundation

Latest slice: explicit positive finite raster-to-plane surface scale per axis,
shared by all bit-depth paths. This allows host-supplied downsample/PAR coordinate
bases while preserving exact identity copies and applying the inverse scale before
sampling. Uniform/anisotropic equivalence and invalid-scale rejection tests PASS;
15/15 Release CTest targets PASS. No host Half/Quarter/PAR acceptance is claimed:
the adapter must still establish AE coordinate conventions and avoid double camera
or PAR transforms. Native plugin/installation remains unchanged.
ASan/UBSan plane-render tests also PASS. Evidence retained in work/stage9-core
(Testing/Temporary/LastTest.log, plane-scale-sanitized, plane-scale-audit.json).
Static audit remains review_required with no findings in the changed renderer/tests.

Latest slice: added 8/16-bpc plane region APIs through a shared typed sampler.
Integer output rounds only after both Catmull-Rom axes, clamps sampled values to
255 / AE 32768, and preserves exact pass-through bytes. Float remains unclamped.
New test compares existing Final within one integer level and requires bit-exact
sparse/full output, padding safety, empty source and malformed-stride rejection.
This does not wire the new renderer into native AE; installed fd69988 is unchanged.
Verification: 15/15 Release CTest targets PASS; new integer-depth tests under
ASan/UBSan PASS. Static audit remains review_required with no findings in changed
renderer/new tests. Evidence retained in work/stage9-core: LastTest.log under
Testing/Temporary, plane-depth-sanitized and plane-depth-audit.json. No speed claim.

Latest continuation: standalone float PlaneRenderer now supports compact source
checkout and independently positioned output rectangles on a logical canvas.
Missing checkout taps are transparent, not stretched. Empty input yields zero;
outside/invalid-plane copying uses logical coordinates. Compact and zero-filled
full inputs are tested bit-exact, including cropped output/padding. Regions
extending outside the logical canvas are rejected explicitly; host integration,
expanded output, PAR/downsample and 8/16-bpc remain open. No native installation.
The initial new test failed to compile due to mixed pointer constness in a test
initializer list; corrected the list type and reran the complete checks.
Verification: 14/14 Release CTest targets and region test under ASan/UBSan PASS.
Evidence: work/stage9-core/Testing/Temporary/LastTest.log, plane-region-sanitized,
plane-region-audit.json. Static audit remains review_required for existing findings;
no findings in changed renderer/test files. No full-AE acceptance is claimed.

Latest slice: user approved outside/invalid-plane pass-through. Added standalone
dense 32-bit float PlaneRenderer: a single Catmull-Rom sample after coordinate
composition; exact copy outside/identity/invalid plane, structured fallback
diagnostics, stride/overlap validation and row-level cancellation. No channel clamp.
This new path is not wired into Cargo/native host. Sparse/PAR/downsample, 8/16-bpc,
overlay/handles/camera and real-AE acceptance remain open. Existing renderer and
installed fd69988 remain unchanged. Final-filter comparison and safety tests are
part of the new plane-render CTest target; this is not full Stage 9 completion.
Verification: Release build and 14/14 CTest targets PASS; plane-render test under
AddressSanitizer/UndefinedBehaviorSanitizer PASS. Flat-plane output agrees with
existing Final Bicubic within 1e-5 on the HDR/impulse fixture. Static audit remains
review_required (existing findings), not release PASS. Evidence is retained in
work/stage9-core/Testing/Temporary/LastTest.log and plane-render-audit.json.

Follow-up: standalone PlaneWarp now composes plane projection with the existing
inverse grid/easing calculation. Analytic test proves column motion follows a
slanted plane rather than screen-horizontal motion. Exact identity is retained;
invalid projection and outside-plane states are explicit, not fabricated source
pixels. Owned immutable evaluated guides reject malformed/nonfinite input.
All 13 Release CTest targets PASS on 2026-09-29, including new plane-warp tests.
Pixel sampling, sparse/PAR/downsample integration, AE controls and camera binding
remain NOT RUN/not implemented for this path; no installed plugin change.

The user requested skipping further performance acceleration work on 2026-09-29.
Historical Stage 8 results below remain valid only within their stated scope.
Added standalone PlaneTransform: normalized four-corner homography and inverse,
strict-convex validation, mirrored winding support, nonfinite/degenerate/horizon
rejection. No renderer, Rust ABI, parameter IDs, saved state or installed plugin
changed. This is geometry groundwork, NOT an AE perspective feature acceptance.
Verification 2026-09-29 on target Mac: Release CMake build and all 12 CTest
targets PASS; standalone PlaneTransform tests with AddressSanitizer and
UndefinedBehaviorSanitizer PASS. Static scanner remains review_required for
existing findings; no findings in the new geometry/test files. Evidence retained
under work/stage9-core (CTest LastTest.log and code-audit.json). No AE acceptance,
new plugin artifact, performance claim or release gate is implied.
Next: define and implement plane-local sampling/interaction wiring, preserving
one resampling pass and avoiding double application of AE layer/camera transforms.

### Stage 8 — initial measurements available; full performance gate open

#### Bounded profile — PNG encoding observed, native bottleneck still unproven

Runner `510ce07`, SHA-256
`f505fa38eaff8d0c4602ecfe24a13dfa3639547b2d7801f467db90d3df0148b6`.
Run `identity-probe-ef3b2cec0c39468e887d311686bea943`: PROFILE_CAPTURED,
three one-second/10ms-interval samples after 5, 20 and 40 observed output files.
Each confirmed the same owned aerendercore PID 69278, process key and fd69988
Mach-O UUID. Final exit 0, exact payload before/after, 60 outputs with digest
matching all twelve prior MFR-series runs. Instrumented time is not a benchmark.

Thresholded leaf histograms show PNGIO longest_match counts 60/71/54,
png_write_find_filter 5/absent/9 and deflate_slow absent/absent/8; color-conversion
functions also appear. Waiting threads dominate raw counts. These counts are
not CPU percentages; absence means below threshold or outside the short sample,
not zero cost. The result supports investigating PNG export overhead before
attributing the end-to-end baseline to native rendering. It does not yet prove
the primary ElasticGrid bottleneck or justify a native optimization.

Target UI inspection found an empty Untitled Project. Attempted fixture selection
through the native Open dialog was cancelled without opening/saving a project.
AE exposes only the window/menu accessibility tree, and observed UI automation
latency is unsuitable for precise first-frame/cache-completion timings. RAM
Preview remains NOT RUN; next step is a guarded host-side measurement of fresh
frames (separately labelled from playback) and reliable lifecycle instrumentation,
not timing delayed screenshots or using unverified menu IDs. No user cache purge.

Evidence: exported `ElasticGridFX-Stage8-profile.json`; full raw samples remain
private in the controlled workspace. Fourteen aerender tooling tests PASS,
including malformed/thresholded histogram checks. Static audit remains
review_required for existing unrelated workflow findings; not a release PASS.

#### Serial requested-MFR comparison — 2026-09-29

Runner commit `27241ee`, SHA-256
`917fe0adde3e3aee6dd506c6b6c7985704db3846c91065d51c169a9239751d30`.
Series `0ff9c4318e49481899ce2fc5f6b6e2b3`: OFF/ON warmups followed by five
serial alternating OFF/ON pairs on the identical pinned AEP and fd69988 plugin.
All 12 exits successful; mapped candidate and installed payload checks passed;
every run produced 60 valid 1080p PNGs with the same aggregate digest as the
previous baseline. No plugin change, GUI project mutation, cache purge or
competing build/test was performed during measurement.

| Requested MFR | Full-process median | Range | Nearest-rank p95 | Sampled core RSS maximum |
|---|---:|---:|---:|---:|
| OFF | 34.6547193 s | 33.4859687–35.7392201 s | 35.7392201 s | 2117337088 bytes |
| ON | 34.5143172 s | 33.2587832–34.7046569 s | 34.7046569 s | 1909194752 bytes |

Median difference is about 0.4%, within observed variability: **no demonstrated
acceleration**. Actual concurrent frame execution was not instrumented, so this
is comparison of CLI-requested modes, not proof of MFR utilization. The host
reports total render time 22–23 whole seconds for measured runs; per-frame
reports are retained with their whole-second resolution, not converted into
misleading millisecond latency percentiles. Startup/output/shutdown/observation
are included in wall time; child-process memory is excluded from core RSS.
Raw logs report Full/Best, RGB Millions of Colors PNG output and sRGB working
profile. The saved fixture remains 32bpc Final Bicubic, but PNG equality is not
HDR fidelity. Cache state remains uncontrolled. Do not compare this later series
to the earlier baseline as an optimization: no production code changed.

Evidence: controlled workspace series.json and raw logs; exported
`ElasticGridFX-Stage8-MFR-comparison.json` and companion evidence ZIP.
Tooling tests: 13 aerender unit tests PASS. Static skill audit: review_required,
not PASS; existing workflow action pinning/checkout-credential candidates remain,
plus a rate-limit heuristic on a local test (not a network auth endpoint).
Next: controlled cache/RAM Preview lifecycle measurements and bottleneck
attribution before choosing an optimization. Stage 8 remains open; Stage 9
perspective-plane requirements remain recorded, not implemented by this work.

#### Initial serial end-to-end baseline measured (Stage 8 still open)

Series `fbf5afad5a214b2a877167af9c50d4c0`: one warmup plus five measured
fresh-process runs of the same pinned 1080p/32bpc/Final Bicubic animated AEP,
60 frames, MFR OFF, max CPU argument 100. Apple M1 Pro, 8 physical/logical cores,
16 GiB RAM, macOS 26.6.2, AE 25.6x101. No cache purge or sampler was used.
The mapped text-file list proved the unique ElasticGrid binary path in each
owned aerendercore; exact installed payload/signature checked before and after.
Per-run runtime UUID is not sampled; the separate preflight established it.
All six runs produced 60 correctly sized PNGs with identical aggregate digest
`d6c1a18e3b99af44f9c4a4cf14cef610dde66f29fc4df9086088f081519ba206`.

Measured full-process seconds: 38.6251985, 38.6976323, 38.3036168, 42.6839684,
42.5909023. Median 38.6976323; nearest-rank p95 42.6839684; sampled maximum RSS
2450243584 bytes. Startup, PNG encoding, process/file observation overhead and
shutdown are included. This is not per-frame render latency, exact OS peak RSS,
cold-cache evidence, HDR fidelity or RAM Preview. Fixture color metadata was
inherited rather than separately recorded, so compare only this exact AEP until
an explicit color-settings fixture is generated. The spread is material; do not
claim small speed improvements against a single run. No optimization performed.
Raw logs, output frames and series.json are retained in the controlled workspace.
Next: explicit environment/fixture color record, frame-level timings, MFR pair,
controlled cache scenarios, and separately observed RAM Preview lifecycle.

Instrumented aerender preflight now PASS for identity and frame count:
`identity-probe-f3945ed06df441ab82369bb224ad43e6`, target aerendercore PID 58659,
live UUID A7C24F56-9776-3971-9CB7-972DD1F7AF8F with pinned installed fd69988
payload verified before/after, 60 output files, digest
`d6c1a18e3b99af44f9c4a4cf14cef610dde66f29fc4df9086088f081519ba206`.
Sampled peak RSS 2368389120 bytes; instrumented elapsed 36.4678 seconds is NOT
a speed baseline. The macOS renderer is detached (PPID 1); association uses its
exact executable, `-aerenderpid` token and unique output path, not its name alone.
The stdout marker is absent, and `/usr/bin/time` on the launcher measures the
wrong memory scope. `tools/aerender_identity_probe.py` records separate diagnostic
evidence; the existing benchmark continues to fail closed rather than accepting
an unproven Build ID. These are internal diagnostic runs, not release artifacts.
Next: integrate render-process observation without sampler-contaminated timing,
validate output repeatability, then collect at least five controlled samples.
RAM Preview and optimization remain NOT RUN.

Follow-up: the clean target project became available. The fixture failed at
output_template because the exact `PNG Sequence` template is absent. A bounded
target probe found `png`, whose observed format is PNG Sequence, with Resize
and Crop false. The generator now accepts that explicitly observed template
and verifies PNG format and unchanged output geometry. Mock tests reject resize.
Fixture `EGFX-perf-160f855b30cc4ba3a4d609139a168e85` was then PREPARED in real
AE: 1920x1080, 32bpc, Final Bicubic, animated, 60 frames. No timings yet.
The temporary exploratory template probe emitted an uncaught JavaScript dialog;
its broad settings traversal was replaced with primitive-only named fields and
guarded error/cleanup handling. The production fixture generator completed with
exit 0; this probe error is not evidence of a native renderer failure.

Authoritative rules fully re-read on 2026-09-29. Local aerender 25.6x101 help
confirms `-mfr ON|OFF max_cpu_percent`; the prepared harness omitted that
required third argument. It now supplies 100, with a command-contract test.
The first real synthetic 1080p/32bpc fixture attempt
`697e395cad7347bc93454d245a3b6720` failed at render_queue before save.
No timing baseline was collected. Per-operation stage and numeric host error
diagnostics now distinguish template, output path, output format and save errors.
Depth restoration is moved before owned-project close to avoid dirtying the
replacement project. A later read-only guard found a saved non-test project
with six items, revision 3091. It was preserved; authorization to discard an
empty test project does not cover it. Further target runs require an idle
empty unsaved project. Offline tooling checks can continue independently.
An additional audit finding remains open: the aerender harness expects a stdout
Build ID marker, while the native build embeds the marker in its binary. Actual
emission is unverified; use observed live-image identity in the render process
if stdout lacks the marker, without weakening the gate. RAM Preview, cold/warm
timings, peak memory and optimization remain NOT RUN. No speed claim is made.

### Stage 7 target acceptance PASS — latest evidence

Run `EGFX-AE-dbcc0ec13f7e48b387f55aa749c623a6` completed with exit 0 and
functional PASS on AE 25.6.0 arm64, PID 42039. The immutable loaded candidate
is fd69988 / EGFX-f442513cb6528f14295d6d45, proven by live path/UUID plus exact
installed payload/signature before and after execution. All ten frames were
captured; seven relational comparisons and per-frame dynamic-range/alpha coverage
checks PASS. Static and moved Corner Pin PNGs were also visually inspected:
no black output or bright streaks observed in this fixture.

Runner: `73714087703e645d84dca51cf5c592e0c97c458f`.
Runner ZIP SHA-256: `6ed14e1cddc4cffde3c2f276f8039ebaca093d7dab7e390b2b42b61774127b7c`.
Evidence ZIP SHA-256: `65dfdf4ea8543eae4a70f21cc618393b80ead3f52ebc7ac95cadc13857635a70`.
The observed None working-space sentinel is the string `None`, not empty text;
linearizeWorkingSpace=false is independently verified. Initial color settings
and bit depth are restored; test comps, footage, solid and empty folder removed.
Regression: 179 Python tests plus JSX ownership/capture suites PASS.
Scope is the controlled 32bpc-host/SDR-PNG smoke, not HDR fidelity, arbitrary
user grids, GPU, performance or full compatibility. The nested generic smoke
record still lists broader release requirements; top-level functional PASS is
the scoped Stage 7 verdict. Release remains BLOCKED. Stage 8 profiling is next.

All entries below are historical evidence, including superseded blockers.

### Fixture calibration and Corner Pin oracle

Target probe found inherited sRGB with linearizeWorkingSpace=true. The fixture
now explicitly uses an unmanaged, non-linearized 32bpc project and restores
the prior project color settings during cleanup. Solid source and its empty
test-created folder are removed. The target probe confirmed zero remaining items.
Report `EGFX-AE-46d6329b9e194f848626a9a0f3a7faa8` then established that every
bypass RGB sample equals the known input times 0.1 within 2/65535; alpha is
unchanged. This local observation is the basis for the export calibration,
not an assumption that all AE exports behave this way. The comparator accepts
only scale 1 or 0.1 and requires a full-pixel match against the effect-disabled
fixture before applying that same scale to every frame. PNG remains an SDR
smoke oracle, not HDR/extended-range proof. Arbitrary darkening is rejected.
Moved Corner Pin uses the known four corners with opaque interior, transparent
exterior and a 3px antialias exclusion band. Interior holes and opaque exterior
are regression-tested. Other frames still require full opacity. Prior generic
alpha/dynamic-range refusals remain historical BLOCKED reports. A fresh packaged
run is required for the new oracle; successful retrospective analysis is not
a replacement for that run.

### Target capture evidence after authorized project reset

The user authorized discarding the empty unsaved test state on 2026-09-29.
Runner `76e3f21cb50b0b873cbf8329db474db2637cb60c`, package SHA-256
`29ee2cd9ec55ec6b3365af0eb6c5ca00bb152be7eb83ef7e98944d90fbc2bce2`,
produced report `EGFX-AE-0ac17ad731d24d3e9cea4bdd9c2198e5`.
Identity PASS, JSX CAPTURED all ten PNGs, functional acceptance BLOCKED:
the comparator rejects low dynamic range even in bypass (red 0.00096–0.07912).
This is not yet localized to a host color configuration or fixture issue.
The moved Corner Pin also has legitimate transparent exterior pixels while
the current comparator demands opaque alpha everywhere. Review this oracle
against geometric coverage; do not broadly relax alpha or brightness thresholds.
Bounded PNG publication wait was necessary: immediate File checks failed
before the host completed writing. Removing the wait was reverted after reading
the successful capture record. The pixel cleanup leaves the addSolid-created
source item, so the next safe cleanup must verify that test-owned object before
resetting the project. No general occupied-project reset is authorized.
Next: control and record the fixture color configuration, fix owned-solid cleanup,
and validate the Corner Pin exterior against an explicit coverage expectation.
Stage 7 stays open; this is neither renderer PASS nor renderer failure evidence.

### Stage 7 runner path fixes — 2026-09-29

Report `EGFX-AE-fd8177e566514d90b6227201607a6a6e` reached ARMED from
CLEAN revision 1 and completed owned-project cleanup. Identity was blocked by
two sampler privacy masks and a resource symlink inside MochaAE.bundle.
The runner now matches separated complete masked path segments against the
independent libproc observation. Visible path segments, UUID, payload, signature,
same-process and conflict gates remain required. Adjacent/partial masks fail.
Conflict discovery treats .bundle as a native bundle boundary like .plugin:
it reads bundle identity and detects ElasticGrid even under another suffix,
without scanning nested resource links. Unknown links in the discovery tree
still block. No plugin payload or installed files are changed.
The retained sample now maps to the expected UUID and the application plugin
root scans successfully. These diagnostic checks are not functional PASS;
a new packaged real-AE run is required.

Packaged runner commit `45594f9dd264e1aeff7109ce236f4387cfbb39bc`
(ZIP SHA-256 `b416b615c1a754db797e1cdd5a4052074a57f74b614518b14e98ae5cd0d78e24`)
was executed as `EGFX-AE-fff7f118bbb44553b4c55012521dc18c`.
Live identity PASSED in AE 25.6.0 arm64 PID 42039: expected UUID/path/payload,
no scan errors or duplicate candidate. Pixel capture exited 90 after writing
bypass.png; capture status FAIL, stage capture, pixel assertions NOT RUN.
Functional acceptance remains BLOCKED. This does not isolate the failure to
the renderer: the previous capture record omitted frame-specific diagnostics.
The next runner records the exact frame and numeric host error/line without
arbitrary exception text. Local validation: 177 Python tests, both smoke safety
JS suites and native libproc fixture checks PASS for the path fixes.

Installation receipt EGFX-update-bf076f0a16f44fb983e717d9e8779b3d reports
INSTALLED_FOR_TEST, no install/rollback errors and a retained original backup.
The installed candidate is 6d3b846463410f37198fda4b625e56e4cea44c22 /
EGFX-603e9d3e4025d271e0488201; its ZIP SHA-256 matches the pinned manifest.
This supersedes the earlier OLD/no-backup startup report, not its historical facts.

Enabled/disabled screenshots show bright streaks with ElasticGrid active and
none with it disabled, on a nonuniform grid. The custom Grid Positions label is
clipped; its full runtime Build ID is not established by those screenshots.
The user also reports black output for ElasticGrid -> Corner Pin on an
Adjustment Layer while the Fast Blur comparator works. This chain failure is
not reproduced in a local AE session or closed by the source tests below.

## Confirmed requirements

See perspective-plane-plan.md: repair effect-chain compatibility separately;
then add four-corner placement of the deformation plane and layer/camera-driven
3D perspective, with the same transform for grid, handles, hit tests and pixels.
Those modes are **planned**, not implemented. Degenerate/outside-plane behavior
is an explicit design decision, not an inferred user requirement.

Render and RAM Preview must be extremely fast without hidden quality reduction.
performance-quality-contract.md remains authoritative; target timings require
real AE measurements. No claim of real-time 4K/8K on unspecified hardware.

## Implemented and verified in this stage

An explicit CPU entry, eg_render_frame_sparse, interprets absent pixels in a
compact returned world as zero on an explicitly known logical canvas. It avoids
stretching the compact rectangle's edges and needs no full-canvas image copy.
Empty input clears active output pixels. Legacy CPU and GPU-plan ABI/semantics,
17 saved parameters, Rust host and installed candidate remain unchanged.

The AE SmartRender source now calls the sparse entry only after requesting the
complete logical canvas in SmartPreRender. Compact returned worlds use their
host-provided origins; absent pixels are transparent. Empty adjustment input is
explicitly cleared. The installed user candidate is still the older 6d3b846
binary, so do not describe the user's current AE output as fixed yet.

Local source checks: baseline 10/10; GCC Release 11/11; Clang Release 11/11;
ASan/UBSan/LeakSanitizer 11/11; TSan sparse/MFR/determinism 3/3. Sparse/dense
steady-state allocation checks pass. Detailed reproduction, thresholds, raw-log
names and performance limits: sparse-render-results-2026-09-28.md.
Read exact resulting commit's Actions/PR checkpoint for CI and macOS outcomes;
queued jobs and hardware compile-only stages are never host/runtime PASS.

## Exact fd69988 candidate

Exact-head final validation, PR CI, update-safety/live-diagnostic workflows and
macOS source/package gate have passed. The immutable macOS artifact has Build ID
EGFX-f442513cb6528f14295d6d45 and package SHA-256
68c135f1a2a9390a0032a31d9073c8111bff6dd43691b9222ec50fad3913982e.
A new reversible updater is pinned specifically to installed 6d3b846 -> fd69988;
it refuses any different current payload and retains 6d3b846 as rollback.
This still does not constitute AE runtime PASS.

## fd69988 updater delivery integrity defect

The first attempt to run the fd69988 updater on the target Mac stopped before
any plugin replacement with `Installer files changed; download the verified
package again`.

The delivered ZIP was re-inspected byte-for-byte. Its embedded fd69988 plugin
payload is intact and still hashes to
`68c135f1a2a9390a0032a31d9073c8111bff6dd43691b9222ec50fad3913982e`.
The updater packaging layer was wrong: `InstallToolIdentity.json` correctly
recorded `tools/authorized_update.py` as executable, but the ZIP writer forced
every non-`.command` member to mode 0644. The updater's own self-integrity
check therefore rejected the package before touching the installed plugin.

The packaging fix preserves each member's executable bit from the identity
manifest, validates ZIP member modes after writing, adds a regression test for
an executable Python tool, and on macOS CI extracts the final ZIP with `ditto`
then re-runs the exact hashes/execute-bit identity comparison. The failed
delivery remains BLOCKED evidence and is not reused.

## Target-Mac acceptance attempt — installed candidate mismatch

A corrected target acceptance launch on 2026-09-29 stopped before live-image
sampling or JSX/pixel execution with:

`BLOCKED: manifest identity mismatch`

The runner had already verified its own pinned fd69988 manifest/package before
reaching the installed-payload check. Therefore this evidence means the installed
ElasticGrid payload does not match the pinned fd69988 BuildIdentity expected by
Stage 7. No AE functional result can be inferred; status is **AE NOT RUN**.

This is consistent with the last confirmed installation receipt in this branch,
which identified 6d3b846 / EGFX-603e9d3e4025d271e0488201. Stage 7 now requires
the existing reversible updater to move that exact installed candidate to the
immutable fd69988 payload, retain 6d3b846 as rollback, then restart/open AE and
rerun the corrected acceptance package. A successful installer receipt is
required before the next functional attempt is counted.

## Target-Mac acceptance attempt — live path diagnostic alias

After the corrected fd69988 installation, the returned acceptance report confirms
the exact installed candidate on disk:

- Build ID: `EGFX-f442513cb6528f14295d6d45`;
- commit: `fd69988c10b25268eb8cad6ee6ced7f6a28bee9d`;
- binary SHA-256: `8fc61c9c4dd20f1f398e470d3f0b75003bcfac4aa45a4669de120b188865426d`;
- Mach-O UUID: `A7C24F56-9776-3971-9CB7-972DD1F7AF8F`;
- AE 25.6.0 arm64 was running.

The run stopped before JSX/pixel execution because the live-image diagnostic
reported `Native path contradicts reported image`. It also recorded a ValueError
for the app-parent `Plug-ins` scan root. Pixel status therefore remains NOT RUN.

Research found that macOS can expose the same Data-volume object through both
`/Users/...` and `/System/Volumes/Data/Users/...`. The diagnostic is updated
to normalize only these known VFS/firmlink aliases for text constraints while
still requiring the pinned on-disk payload, Mach-O UUID and underlying file
identity. A symlinked scan root is ignored only when it aliases another
independently listed plugin root; unknown symlink targets remain blocking.
No plugin or user state is changed by this diagnostic fix.

## Acceptance delivery execute-bit defect caught before handoff

The first acceptance package built after the live-path diagnostic fix was inspected
before user delivery. Its bytes and embedded fd69988 payload were correct, but the
ZIP writer forced Python members to mode 0644 while `AcceptanceToolIdentity.json`
marked `tools/live_identity.py` and `tools/target_ae_acceptance.py` executable.
That package is not distributed.

The acceptance packager now preserves each member's executable bit from the
identity manifest, validates ZIP modes after writing, includes a regression test
for executable Python members, and on macOS CI extracts the final ZIP with
`ditto` and re-runs the exact hashes/execute-bit comparison. The plugin payload
remains the same immutable fd69988 candidate.

## Real acceptance report — effect not resident before identity sampling

The returned run `EGFX-AE-549be4237ee647eabe9dba06df8c665b` proves the exact
fd69988 payload on disk and AE 25.6.0 arm64, but `loaded_images` was empty and
pixel status remained NOT RUN. The orchestrator sampled live process images before
the intentionally empty project had instantiated ElasticGrid.

The corrected sequence is now:
1. guard an empty, unsaved, clean test project;
2. create only test-owned footage/comp/layer and instantiate ElasticGrid;
3. sample the same AE PID and prove pinned path/Mach-O UUID/Build ID;
4. close only that exact armed test project without saving and create a fresh
   empty project;
5. run the existing ten deterministic pixel captures;
6. verify the installed payload again.

If ownership changes, cleanup refuses to close the project. The plugin binary is
unchanged. Conflict scanning is also narrowed to Adobe-documented macOS roots:
Common MediaCore plus `/Applications/Adobe After Effects [version]/Plug-ins/`.
Only the documented root itself may be resolved when it is a symlink; inner
symlinks remain refused.

## Target-Mac acceptance reports — opaque project guard

The two later Stage-7 reports `EGFX-AE-0968a7a315644ba3b87fa9519d59a7db` and
`EGFX-AE-1b9cdc075220463e8319610a14f84ec4` stopped before identity sampling and
pixel capture. Their `91` is the JSX arm script's deliberate guard exit, not an
AppleScript transport failure. The old phase record did not write AE version or
the individual guard predicate until after the combined guard, so it cannot
prove whether the project was saved/occupied/dirty or whether the undocumented
`Project.dirty` attribute was absent.

The follow-up acceptance tooling preserves all saved/occupied/dirty refusals,
records only opaque guard/revision tokens plus AE version before mutation, and
does not relabel an AE script exit as a transport fault. For an absent `dirty`
attribute only, it accepts a separately readable fresh `Project.revision == 1`
alongside an empty unsaved project; getter errors, malformed values and any other
revision remain blocking. These historical reports remain `BLOCKED`; a new
same-PID identity plus ten-frame target-AE result is still required to close
Stage 7.

## Latest target-AE run — occupied project

The revised acceptance runner from `fa7be425ce78083519f4a812a4f3689fe151df90`
was executed on the target AE 25.6x101. Report
`EGFX-AE-065fc56488584561ac00fd7ddd4936cb` (SHA-256
`fb9c9967af76f1b2e6bbcebeae6a18b8699640d50653b639f21922d1fa91fa0f`)
reached the JSX arm phase and returned `guard: OCCUPIED`, revision `32` before
any test-owned object was created. The script exit `91` is correctly reported as
that guard refusal, not as an AppleScript transport failure.

No live image was sampled and no pixel frames were captured; both statuses remain
`NOT RUN`, functional status remains `BLOCKED`, and this report is not renderer
evidence. The only condition for a safe retry is a newly created empty unsaved
AE project. The runner will continue to refuse rather than close or alter an
occupied user project.

## Target-AE acceptance automation

A single non-installing target runner is implemented for fd69988. The first
user-side attempt exposed a runner-only manifest type bug before any AE test ran:
a decoded dict was passed to a path-based verifier. That failure is preserved as
NOT RUN for AE functionality. The runner now keeps both the manifest path and
decoded data distinct, with regression tests covering both verifier and smoke
handoffs. It first requires live-image identity PASS for the exact installed candidate, then runs
the guarded ten-frame patterned AE smoke in the same AE PID. Functional PASS
requires all direct deformation/animation/reset checks plus Adjustment Layer ->
ElasticGrid -> identity/moved Corner Pin pixel assertions. Its shared ZIP includes
only sanitized summary and synthetic frames; raw process sample remains private.
Release status stays BLOCKED even when this functional gate passes.

## Stage 8 preparation while Stage 7 remains open

Performance measurement tooling is prepared and source-verified, but no target
performance measurement has run and Stage 8 is NOT started for acceptance.

- `e51703b`: controlled aerender benchmark + report comparison.
- `1a177da`: synthetic 1080p/4K, 8/16/32-bpc static/animated AE fixture generator.
- `c432da6`: fixture ownership test made an explicit Final Validation gate.
- The new gate found a mock-only null-project substitution bug; `67f9e8d`
  fixes the test harness without changing production fixture code.
- Exact-head Final Validation and all five PR CI jobs at `67f9e8d` PASS.

The benchmark requires Final Bicubic, a pinned synthetic AEP hash, unique output
directories, runtime Build ID from aerender, warmups plus at least five measured
samples, wall/user/sys/peak-RSS evidence and matching fixture/MFR settings for
comparison. Timing results never imply quality equivalence.

Actual synthetic AE fixture generation, aerender measurements and RAM Preview
profiling remain NOT RUN until Stage 7 target-AE functional acceptance passes.

The Stage 7 plugin candidate remains immutable `fd69988` /
`EGFX-f442513cb6528f14295d6d45`. Later docs/test-tooling commits change source
identity and may produce different CI Build IDs if rebuilt; those builds are not
substitutes for the pinned Stage 7 artifact.

## Next gates, in order

1. SmartFX source now wires the tested sparse CPU renderer: logical canvas is
   snapshotted during pre-render, max bounds are fixed to that canvas, returned
   compact-world origins are preserved, and a None adjustment input is rendered
   as transparent instead of leaving output untouched. The ordinary Render path
   remains dense; Metal remains disabled. This is SOURCE-VERIFIED only until a
   new exact candidate reproduces raster/text/Adjustment Layer chains in AE.
2. The automated AE smoke now also creates an Adjustment Layer and captures
   ElasticGrid before Corner Pin, immediately after identity Corner Pin, and
   after moved corners. Pixel validation rejects a black/flat frame, requires
   identity Corner Pin to preserve pixels, and requires moved corners to change
   them. This is prepared automation; actual AE execution still requires the new
   candidate. The custom-control label also uses a taller readable short Build ID.
   Verify loaded identity, drag/Undo/Redo,
   save/reopen/restart, actual 8/16/32-bpc alpha/HDR/ROI/PAR/MFR/cancel/aerender.
3. Profile equal-quality Render/RAM Preview, then optimize proven costs. AE GPU
   dispatch stays disabled pending real host/Metal correctness and lifecycle QA.
4. Prototype and test the approved 2D plane, then 3D/camera integration, preserving
   saved-project compatibility and interaction/render agreement.

The live-diagnostic path contradiction and incomplete app-plugin scan remain
open. Physical AE/Metal and target profiling are unavailable in this execution
environment. No new installable candidate is delivered by this checkpoint.
Older dated reports remain evidence only for their original source and scope.
Authoritative process: ../DEVELOPMENT_RULES.md.
