# Four deformation modes and disabled Demo — 2026-10-05

Authorized product extension under frozen rules8.0.0; applies to Mac and Windows.
Existing installer obligations and exact Dev52/Dev56 acceptance remain separate.
No old acceptance transfers to a new plugin artifact. No main/merge/release.

## Requirements and coordinate contract

| ID | Behavior | Verification |
|---|---|---|
| M1 | Comp mode retains ordinal1/current Layer Plane behavior, including automatic 3D handling | Old-mode pixel/ID regression; native old project reopen |
| M2 | Layer mode adds ordinal3, fitting 2D sourceRectAtTime bounds; existing 3D automatic plane handling retained | Transformed small 2D footage/text; animated source bounds; UI/render agreement |
| M3 | Surface mode (formerly Flat) retains ordinal2/current Four Corners; neutral grid does not pin source to destination | Skewed neutral pass-through and deformed core regression |
| M4 | Perspective adds ordinal4: pin source canvas (or already-projected text basis) to destination quad and inverse grid before one sampler | Analytic corner/center coordinates; pixels at all depths/qualities; corner drag |
| M5 | All modes share grid, Wave, Show Grid, sampling quality, abort and saved animation; no mode switch rewrites parameters | Owned pre-render snapshot, Undo/save/reopen and animated mode matrix |
| D1 | Demo design is FSTR FX inside each cell; future preview/export watermark | Design only; actual licensing integration is a prerequisite |
| D2 | Demo rollout is OFF even without license; no watermark or public fake activation toggle | Both render routes remain unchanged; explicit compile-time disabled policy |

UI display names now: Comp mode / Layer mode / Surface mode / Perspective. The last caption
is kept short to preserve native popup sizing without dummy options. Existing popup
ordinals1/2 retain meaning; new choices append so saved projects are not reinterpreted.
Surface replaces the earlier Flat caption by user request; Perspective unchanged.
Mode remains static. Public corner controls/Fit Layer are enabled in Surface (2D)
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

## New native feedback — 2026-10-05 Dev80

User reports Comp mode and Layer mode both show the grid inside the layer.
Confirmed current implementation: Comp retains old Layer Plane behavior; 2D
kind1 returns no derived region, so the default domain is the input layer canvas.
Requested correction interpretation (awaiting short confirmation): Comp domain
is entire composition, Layer domain is layer bounds. This supersedes M1's old
behavior preservation only after confirmed scope. Must reconcile the shared
UI/render coordinate domain, source bounds, transforms, camera and saved state;
no UI-only expansion that lies about render deformation.

Requested display order: Comp mode / Layer mode / Flat mode / Perspective.
Existing saved PlaneMode ordinals2=Flat,3=Layer cannot simply be swapped.
A versioned/proxy selector migration must preserve the selected mode and existing
keys on old project reopen/Undo. No ordinal reorder has been implemented yet.

User confirmed Comp mode must span the entire composition, even for a smaller
layer. V4 owned plane bindings derive Comp corners with fromComp (2D) or
fromCompToSurface (3D raster); 3D native text retains comp-space sampling.
Dedicated kinds5/6/7 distinguish these domains. Layer/Flat/Perspective retain
v3 bounds. Upgrade exact v3 via receipt generation3 and reversible owned
expression transaction; old v1/v2 and undone bindings retain prior policy.
New expression source reference: Adobe Expression Language Reference,
https://helpx.adobe.com/after-effects/desktop/work-with-expressions/expression-language-reference/expression-language-reference.html
V3 regression and V4 transformed 640x480 Comp/smaller-layer expression tests
PASS; native grid/render alignment still pending. Menu order is still unchanged
until a saved-mode-safe selector migration is implemented.

Dev88 display order implemented with appended ModeSelector standard native
CONTROL_ONLY popup (no saved data stream), following SDK25.6 Supervisor.
Existing PlaneMode retains ID, type, ordinals1=Comp/2=Flat/3=Layer/4=Perspective
and all old keys; hidden as __FSTR Mode Value, still first binding index1.
Display maps1/2/3/4 to stored1/3/2/4. UserChangedParam updates the existing
canonical popup with CHANGED_VALUE inside the host's undoable user edit;
UpdateParamsUI changes only a cloned UI-control popup via UpdateParamUI.
No saved values or keys are changed to reorder the menu. Reset/Undo/reopen
must re-synchronize display from the canonical stream. Native acceptance pending.
95 Rust tests, Clippy and 15 host contracts PASS; old failing runs retained.


### Comp output correction (user feedback after Dev88, 2026-10-05)

User reports grid dragging works but distortion clips at source layer bounds.
Confirmed interpretation: Comp owns the whole composition destination; Layer
retains the layer destination. Dev84 grid-domain evidence does not accept this
output contract. Dev92 work: immutable PF point dependency bounding rectangle,
SmartFX request intersection/max output and legacy FRAME_SETUP expansion;
IExpandBuffer in PiPL/runtime; separate Comp C ABI entry dispatch permitting
expanded destination while retaining original source extent and scalar/cache
samplers. No new serialized field/parameter/ID, no double sampling. Core regression
covers 8/16/32bpc, both qualities, all edge modes, source guard, neutral exactness,
Layer clipping and row padding. Native output/origin/Undo/save/reopen checks pending.
Primary SDK25.6 AE_Effect.h:1625 (SmartFX world origins), 2908 (FRAME_SETUP origin),
3055 (legacy input location in output), 2512–2527 (result/max rectangle).

Dev92 pre-host checks: 96 Rust tests, 28 CTest targets and 15 host contract checks PASS.
Clippy first failed assignment-formatting lint; corrected rerun PASS. Numerical
samplers unchanged. Native output and legacy expansion remain NOT RUN.

CIa26fbae Mac37304253224 FAIL: one obsolete first-application static assertion
expected Kind0..4 before v4 Comp kinds5..7. Other281 Python checks PASS. Update
assertion to0..7, retaining all identity/ownership/no-project-mutation guards.
Native installer CI37304253222 PASS; Windows37304253327 pending at checkpoint.


Dev92 native AE25.6 acceptance: actual expanded-output pixels PASS, Layer output
bitexact unchanged and Comp switch roundtrip exact. Loaded image UUID/path matches
EGFX-c2b0f08cc2b1b1d7c33e8ccb. User reports drag/Undo/mode/menu PASS. Windows
37304253327 PASS on a26fbae. See outputs/comp-output-dev92/native-pixels.json.

### Additive Edge None request (2026-10-05)

Source: user after Dev92 acceptance requests first Edge Behavior item that leaves
layer without additional edge drawing. Contract: None samples transparent black
outside the original source image, retaining bicubic/bilinear boundary filtering.
Comp remains a full composition destination; it does not fill missing source pixels.
UI order None / Clamp / Wrap / Mirror, new instance default None. Preserve stored
Clamp/Wrap/Mirror ordinals1/2/3; append None4 to hidden canonical data popup and
use CONTROL_ONLY presentation selector, same native pattern as ModeSelector.
No animation stopwatch. Save/Undo/reopen must retain old project selections.
Shared CPU plane/cache/sparse and Metal taps must support transparent samples
without out-of-bounds access, retaining existing fast paths and tap arithmetic.
Acceptance: visible four choices/order/default, four modes, 8/16/32bpc and both
qualities, old-mode outputs unchanged, neutral exactness, partial source storage,
negative origins, and native None alpha beyond deformed image; Windows CI and
real AE checks separately labelled. Demo/licensing/installers/loupe untouched.

Edge None source checkpoint:97 Rust tests/28 CTest/282 Python/all JavaScript
checks/15 corrected host contracts and Clippy PASS. Initial host contract failed
only static nonanimated-control count after appending EdgeSelector; fixed to
include it. Shader offline compiler NOT RUN locally (Metal toolchain absent);
runtime test compiled, sandbox exposes no GPU, authorized native rerun pending.
New Dev96 package/native UI/old saved values/output and Windows CI pending.

Dev96 native: old saved Wrap2 retained, fresh defaultNone4 PASS; all four modes
rendered. None produces245102 transparent pixels vs Clamp full-frame fill. This
current user's deformation compresses the source footprint (bbox99,123–375,348),
so a first ad hoc check incorrectly required pixels outside the original layer
and returned FAIL. Retain that report; it does not demonstrate clipping. Separate
core expanded None fixture does prove moved pixels outside source storage. Native
expansion with deliberate outward drag still pending. GPU runtime parity native
PASS for all24 quality/edge/case combinations after sandbox no-device result.
Cua visual FAIL: saved old edge row remains visible beside CONTROL_ONLY selector.
Dev100 correction additionally marks canonical EdgeMode NO_ECW_UI, retaining
same saved values and source/render code; new package/UI verification pending.


Dev100 package/signature/atomic installation and loaded-image identity PASS.
Native None4 save/reopen PASS. First capture closed/reopened before PNGs appeared;
second capture without closing PASS: None245102 transparent pixels versus Clamp0,
640x480 output. Current native deformation bbox99,163–375,388. UI not accepted:
Effect Controls locked to none; user asked to unlock/select and inspect order/output.
Windows CI37306339582 PASS and exact downloaded AEX/manifest identity PASS.
Mac CI37306339475 FAIL from test_quality.cpp exhaustive switch missing None.
Corrected independent quality oracle to return transparent taps, with float/8/16bpc
coverage for all four edges; strict ASan/UBSan compile/run PASS. Production source
and installed artifact unchanged by reference-test correction. Rerun pending.


### Mode-specific edge simplification (2026-10-05)

User decision: Layer and Perspective always evaluate Clamp; disable Edge Behavior
and display Clamp there. Comp and Flat retain editable None/Clamp/Wrap/Mirror.
Do not overwrite canonical saved choice on mode switch or UPDATE_PARAMS_UI;
returning to editable modes restores it. This intentionally changes evaluation
of old Layer/Perspective scenes with non-Clamp choices, preserving stored data.
Shared legacy and SmartFX parameter snapshots apply the same policy; no platform
sampler fork. Reject stale selector action while locked without writing saved
choice. Acceptance: all saved choices map to Clamp only in Layer/Perspective,
editable-mode roundtrip, native disabled/enabled visual, native evaluated pixels,
Mac/Windows exact builds. Implementation Dev104; native/CI verification pending.

Dev104 source9bdf447, EGFX-ab9b2b3b3a3d8e9cb7493c49:98 Rust/15 host contracts/Clippy PASS.
Package/signature/atomic installation/loaded identity PASS. Initial branded ZIP
rejected before swap, old plugin untouched; corrected internal ZIP installed.
First neutral fixture yielded equal edge outputs (expected exact pass-through),
so it could not prove editable-edge behavior. A temporary Wave Amplitude15 fixture
showed Comp/Flat different and Layer/Perspective byte-exact Clamp/None outputs.
Wave and canonical choices restored. First wave script guard refused incorrect
output path, corrected without scene mutation. Last PNG was asynchronous; immediate
reader failed, subsequent completed-frame comparison PASS. User UI confirmation
pending. Windows37311196442 PASS, Mac37311196440 IN_PROGRESS; native Windows NOT RUN.
Evidence outputs/edge-fixed-dev104/native-wave-pixels.json, loaded-identity and CI.


Dev104 native visual inspection via Cua PASS: Layer and Perspective show disabled
Clamp; Comp restores active None; Flat had active None. Hidden canonical edge row
absent. User-modified owned scene preserved as edge-ui-check.aep before switching.
New additive user decision: rename displayed Flat mode to Surface mode on both
platforms, retaining saved ordinal2, internal Mode::Flat, deformation and keys.
Dev108 changes captions/build number only; new visual/package/CI verification pending.

Dev108 b7ac185 EGFX-60af8664e175098b8c7743ae installed: package/signature/atomic replacement/loaded
identity PASS. Existing UI-order tests2 and host contracts15 PASS. Native caption
confirmation pending: user operates AE, Cua rejected action on changed state;
no alternative input used. User asked to confirm Surface display. Windows CI
37312709671 PASS; downloaded AEX checksum/build/source identity PASS; native
Windows AE NOT RUN. Mac37312709680 IN_PROGRESS. Source scanner exit1 reviewed
existing mocked manifest/signature test as false positive, not a network auth route.
Scanner is not release certification. Original Dev104 Mac CI37311196440 now PASS.


### Perspective source-bound clipping bug (2026-10-05)

User Dev108 screenshot shows corners203,-89 /357,-88 /358,-31 /0,241 but
image cut at original source top edge. Expected full corner-pin mapping outside
source rectangle. Rules8.0.0 Smart Entry, Process research/state/evidence,
Engineering Debugging, Native spatial/output/stride requirements reread.
Root cause: host output bounds/legacy FrameSetup expand Comp only; between-plane
renderer also clips destination to source canvas. Fix extends Perspective output
to valid corner bbox on both host paths and disables only destination source-rect
clipping for between-plane renderer. Outside-quad transparency and original
source sampling/storage retained, no ABI/ordinals/keys/quality changes.
Regression translated quad crossing negative source x/y with padded output,
8/16/32bpc and both qualities: old source assert FAIL reproduced (exit134).
Initial ad hoc compile omitted CpuRenderer dependency and failed link, retained;
corrected reproduction used unmodified committed bridge from separate file.
Dev112 implementation shared Mac/Windows. Native validation/CI pending; old
Surface/Comp/Layer and all retained obligations remain.

Dev112 EGFX-422a6cfe0dcda2e3448e5aff package/signature/atomic replacement PASS.
Regression after-fix ASan/UBSan PASS;98 Rust/15 host contracts PASS. Native AE25.6
32bpc Final screenshot-quad output PASS: alpha bbox81,27–438,357 (source top116),
15407 visible pixels above former crop. User scene preserved separately, check
copy left open for manual review. Exact source45350b2 pushed to authorized branch,
CI/native Windows pending. No main/merge/release. Dev108 Surface caption observed
in AE previously; new caption acceptance no longer blocked on hidden panel.
