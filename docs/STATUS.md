# Current status

## Latest checkpoint — 2026-10-08

- Local redraw fix: held identical SDK16.16 targets return Idle (no invalidation
  or UPDATE_NOW); release preserves Commit/Unchanged and final coordinate. Each
  immutable Frame owns one bounded129x129 raster keyed by exact source mapping
  and supplier layout; repeated DRAW reuses owned Rc pixels without cloning the
  buffer. Drawbot objects remain callback-local. Default137 tests/diagnostic140
  tests and strict diagnostic Clippy PASS; new regressions cover duplicate motion,
  release, cache replacement/drop, zoom/layout/frame separation. Initial red
  regression was expected compile FAIL (Idle variant absent), preserved. Dev189
  census / Dev190 ordinary identifiers assigned. Packaging/native comparison
  PENDING; no claim to have fixed original memory/hang cause.

- Current overriding checkpoint: diagnostic sourcef33452e/BID
  EGFX-a1257392f0d14499e13b2ba7 remains installed; AE is absent after owned guard
  termination in auto-r3. Cleanup PASS. Older ordinary-installed and AE-open
  statements below are historical. Heavy memory acceptance BLOCKED.
- Read-only redraw audit: unchanged held targets still return Step::Preview and
  queue viewer invalidation/UPDATE_NOW; every loupe DRAW rasterizes129x129 and
  creates a Drawbot Image. Final927 images created/dropped, live0; last~5s
  ~160-171 images/sec while async request/receipt counters were stable. No
  per-DRAW composition request loop observed. Callback-origin attribution and
  host allocation lifetime remain UNKNOWN; no leak-fix claim. Frozen transaction
  tests4 PASS; scanner exit1 reviewed existing mock auth-URL false positive.
  No source edits/new native session. Findings and ordered remediation:
  outputs/resource-census-v2-oct08/loupe-redraw-findings.json.

- Current L3: magnify the evaluated composition, including FSTR, all visible
  layers/effects and alpha. One immutable frame at press; only overlays move
  during Surface/Perspective corner drag. Release commits the Point once and
  renders. Layer viewer retains its evaluated layer view. Earlier upstream-only
  source experiments remain history and do not implement the current contract.
- Ordinary Dev188 source0cba0ddff0a2c0f3ef3fff49bef6d9a036c70f1d /
  BID EGFX-dad3f547f71993690f57a67f is installed. All archive/file hashes and
  strict ad-hoc signature PASS. AE25.6 loaded canonical UUID
  9B7E199F-643C-3EBF-90E4-D5EB64BCDC33 PASS in owned fixtures. Full-resolution
  U8 ARGB Straight request; owned copy premultiplied. Includes cursor retention
  and terminal warm-error handling. No diagnostic feature/logger enabled.
- Source checks: Rust134, strict Clippy and host contracts17 PASS. Scanner exit1
  reviewed unchanged mocked-payload fixture false positive at
  tests/test_target_ae_acceptance.py:54; readiness not_assessed. Windows packet
  Python20 PASS locally. These checks are not native pixel/loupe acceptance.
- Native comparison-r2 with original idle cache ON stopped at256MiB growth.
  Baseline833858768, peak1141026264 bytes. Growth started before helper input.
  Source/readback not accepted; subsequent-609 follows owned guard closure.
  Allocation cause UNKNOWN. Idle rendering is a diagnostic lead, not proven
  product causality or an approved workaround.
- Same ordinary188 tiny32bpc/Final multilayer Perspective fixture with temporary
  idle cache OFF: r3 peak977972456 and r5 peak984575472 bytes, no45s limit hit.
  Actual TopRight406,174->514,84; other corners/layer position unchanged.
  Existing TopLeft/Affected Lines keys and midpoint retained PASS. r4 helper
  refused because AE was not foreground; its launcher exit0 was not input PASS.
- Ordinary188 manual Perspective multilayer r7: user confirmed evaluated
  composition/alpha/background in the lens, image freeze, no hand/no alert
  USER_REPORTED_PASS. Current native Undo readback NOT_RUN. Previous r5 short
  helper inputs omitted unheld hover and did not establish custom-route acceptance.
- r7 growth guard fired at44.5249s: footprint1101344216, armed baseline828485008
  (growth272859208). OWNED_AE_KILL_SENT explicitly recorded; this closure is
  guard termination, not an established spontaneous crash. Idle cache temporarily
  OFF. Footprint remained about790MiB through30s, rising during the manual check.
  Allocation cause UNKNOWN: one host composition request/cache allocation and
  repeated/leaked work must be distinguished before a product fix. Native memory
  acceptance NOT_PASSED; do not widen limits or suppress this result.
- Cleanup VERIFIED: AE absent, exact9 temporary AEHL components restored with
  unchanged tree hashes; actual25.6 preferences full original hash unchanged.
  r3 initial cleanup FAILED because private restore command omitted BASE; repaired
  explicitly and verified before more work. Later cleanup runs PASS and now
  attempt component restoration even if preference restoration fails. Only the
  known idle-cache key is restored; unrelated preference bytes never overwritten.
- Manual completion census source9e86b2f / BID EGFX-0b93988477d2e9b604e2ba03:
  user explicitly removed timed closure for manual sessions. New opt-in
  --until-user-done preserves exact PID/birth and1.5GiB/256MiB memory limits,
  closes on a fresh fixture completion marker; default45s behavior unchanged.
  Guard tests13 PASS. OwnedPID12308 memory peak965193032 bytes, then870231392;
  no limit hit. Two nonnull loupe receipts/two checkins OK/zero checkin errors;
  one1228800-byte copy retained, same peak, replaced rather than accumulated.
  Post-user-Undo points match baseline; intermediate moved points NOT_READ.
  Original heavy hang cause UNKNOWN; tiny diagnostic PASS does not certify it.
  Closure USER_DONE_OWNED_AE_KILL_SENT; cleanup PASS, exact9 restored and actual
  idle-cache key/full preference hash unchanged. Earlier private restoration
  path assertion failed after copying harness; repaired and verified before this
  launch, not hidden as an initial PASS.
- Heavy showTime saved-copy trial ordinary188 / ownedPID15431 opened successfully
  in AE25.6,1920x1080,32bpc,Full/Final,mode2,time4.16, with idle cache temporarily
  OFF. Exact canonical loaded UUID verified. User reported corner drag followed
  by AE closure. Guard record proves STOPPED_ON_LIMIT then OWNED_AE_KILL_SENT:
  baseline897085800 ->1176843080 bytes (growth279757280),176.58s after observer
  start. This is controlled termination, not evidence of a spontaneous AE crash.
  Main-thread sample shows event-loop wait; a stopped sample cannot establish
  normal responsiveness or allocation ownership. Heavy drag acceptance BLOCKED;
  Undo and RAM Preview NOT_RUN. No proven plugin leak or product fix. No new
  cap increase/relaunch. Cleanup PASS: actual preference full hash unchanged,
  exact9 components restored. Earlier startup gate expired before opening;
  second harness attempt hit ExtendScript JSON-unavailable reporting error and
  loading cap. Both preserved as harness failures, not plugin acceptance.
- User authorized budget calibration after the heavy guard closure. Added explicit
  saved-fixture --growth-budget-mib384 option; default256MiB unchanged. Total
 1.5GiB/loading1.25GiB caps and exact PID/birth identity unchanged; no user-timed
  closure. Guard17 tests PASS: observed279757280-byte growth fits the experimental
  profile, >384MiB growth/absolute cap/loading cap still stop, unbounded profiles
  rejected before process access. This is a test-budget adjustment, not a plugin
  fix or acceptance.1920x1080RGBAfloat frame is33177600 bytes; experimental budget
  leaves headroom above the recorded peak. Actual AE allocation count UNKNOWN.
  Native-r4 saved-copy calibration pending; idle cache temporarily OFF/restored.
- Calibration native-r4 ordinary188 ownedPID20551 with explicit384MiB growth
  budget: heavy TopRight1242,543->1170,612 readback PASS; only that corner changed.
  User confirms Undo returned image; all corners/layer position/mode read back
  identical to baseline PASS. Gesture peak1163407744 bytes then fell; no guard
  event at this checkpoint. Tiny bounded3-frame RAM Preview atFull/Final/32bpc
  start/stop observed through overlay visibility and time4:04->4:05; AE remains
  responsive, no alert observed. This does not certify full8s preview stability
  or establish the original hang cause. Heavy loupe/freeze visual confirmation
  still pending. Manual session remains open until user completion; cleanup pending.
- Native-r4 later stopped on growth limit at669.315s:993341848->1398750112
  bytes, growth405408264 >384MiB. ExplicitSTOPPED_ON_LIMIT/OWNED_AE_KILL_SENT;
  no user-done marker. User said they had not finished and subsequently reported
  AE closed. Previous gesture/Undo and bounded3-frame observation remain scoped;
  session memory acceptance BLOCKED, heavy visual loupe/freeze NOT_CONFIRMED.
  Do not describe this as spontaneous AE crash or complete PASS. No further
  increase/automatic relaunch; attribute late allocations before repeating.
  Cleanup now PASS: original idle-cache key/full preference hash restored and
  exact9 components returned. Earlier pending-cleanup note is superseded.
- Late-memory investigation: no new AE session or manual stress run. Ordinary188
  has no resource counters, so prior guard data cannot attribute late allocations.
  Source audit finds bounded replaced loupe copy, balanced frame checkin path and
  Drawbot Image RAII release; not proof of host cache lifetime or absence of leaks.
  Resource-census diagnostic schema2 adds warm/gesture async-call attempts and
  preparation-error counters, transient UI bitmap/image lifetime tokens. Image
  bytes are an estimate of supplied storage, not host texture/cache allocation;
  token drop does not certify successful host release. Records changed scalars
  at most once/second, cap1024; no new render/SDK requests, no worker I/O, no
  shipping quality/state/UI changes.137 diagnostic Rust tests PASS. Host census
  of late heavy growth NOT_RUN; allocation cause UNKNOWN. Diagnostic preparation
  is not a product memory fix. Ordinary188 remains installed.
- Resource-census-v2 sourcef33452e / BID EGFX-a1257392f0d14499e13b2ba7:
  optimized Mac diagnostic build,137 Rust tests and strict Clippy PASS. Package,
  sealed archive/manifest, ad-hoc signature and installed exact manifest PASS.
  Diagnostic is temporarily installed; ordinary188 is preserved in Backups and
  outputs/loupe-dev188. Automatic saved-copy startup BLOCKED: Cua reports Mac
  locked. No UI readiness receipt, project open or Preview dispatch; stop-requested
  written; controller cleanup PASS, AE absent, original preference full hash
  unchanged and exact9 components restored. No manual stress test. Unlock
  required for dependent automated native census. Late-allocation cause UNKNOWN.
- Census-v2 automatic-r2 ownedPID31654: heavy1920x1080 selected before3-frame
  Preview.5 pixel checkins OK/zero errors; borrowed input/output live0. Later
  selected Effect Controls causes3 warm async polls,1nonnull receipt/1checkinOK,
  one8294400-byte copy, zero gesture calls/preparation errors. No unbounded loupe
  request/copy accumulation observed. Guard killed at301.319s/1206907816 bytes.
  Harness initial active comp was640x480, so armed baseline801435048 includes
  neither heavy loading nor its UI; heavy memory-budget acceptance BLOCKED, not
  product leak evidence. Controls-visible UI and host cache allocations remain
  unmeasured. Cleanup PASS with exact original preferences/components. Corrected
  auto-r3 chooses exact heavy comp before fixture-ready/baseline; same limits.
- Census-v2 corrected auto-r3 ownedPID33270 chooses heavy1920x1080 from start,
 32bpc/Final, baseline917910200 bytes. Bounded3-frame Preview start/stop observed;
  effect panel opened with existing layer selected. Warm3 async polls/1nonnull
  receipt/1checkinOK/zero preparation errors, one8294400-byte loupe copy. Borrowed
  worlds live0; no further requests observed during passive interval. Bounded
  automatic resource-lifetime observation PASS; full RAM/gesture/visual acceptance
  NOT_RUN, original hang cause UNKNOWN. Do not turn a negative short reproduction
  into a leak fix or host-cache ownership claim. AE remains open awaiting user
  completion, guards active; cleanup PENDING. Exact candidate loaded UUID
 3E67E99E-81B6-3871-BA3A-1ECB6E46C336 verified. Evidence:
  outputs/resource-census-v2-oct08/native-auto-r3/acceptance-checkpoint.json.
- Default timed native trials retain45s/1.5GiB total/256MiB growth guards. Heavy scene acceptance BLOCKED /
  RAM Preview NOT_RUN; original reported heavy hang remains UNKNOWN. No cap
  increase, shipping idle-cache workaround, original-project edit or stale Cua
  relaunch. Ordinary188 reinstalled after census; exact installed files/archive verified PASS.
- Surface187 diagnostic fixture historical native point/Undo/key PASS and user
  original-cell/no-hand/no-alert report are retained in evidence, not transferred
  to ordinary188 or its newly accepted composition source. Current Windows
  MSVC/AE NOT_RUN; only Apple Rust target installed. New local commits have not
  been pushed. Demo OFF; no main/release action.

Evidence: outputs/loupe-dev188/{installed-verification.json,
native-composition/acceptance.json,native-composition-r2/acceptance.json,
native-composition-idle-off-r3/acceptance.json,
native-composition-idle-off-r4/acceptance.json,
native-composition-idle-off-r5/acceptance.json,
native-manual-composition-r7/acceptance.json};
outputs/loupe-dev188-composition-*.log and audit-review.json;
outputs/loupe-windows-packet-oct08.log;
outputs/loupe-upstream-dev187/native-surface;
outputs/loupe-memory-census-oct08/native-user-completion/acceptance.json;
outputs/heavy-dev188-oct08/native-r3/{acceptance.json,guard/memory.jsonl,
guard/limit-sample.txt,cleanup-result.json}.

Older dated notes below remain history; use this checkpoint for current holds.

2026-10-08 upstream Dev186 native FAIL: exact installed/loaded source55ae84f,
BID EGFX-5d90502ff77a1e03e35ec89e verified; AE25.6 rejects upstream input receipt
with `BEE_CheckoutEffectInputFrame currently only supports straight matte mode`.
This alert was observed directly through Cua and independently reported by user;
no gesture dispatched.45s guard closed owned PID90174; all9 components restored,
AE absent, idle-cache key unchanged. Fix: upstream request uses Straight; owned
pixels normalize to premultiplied before the unchanged lens raster. A failed
initial request now consumes its warm budget too, avoiding per-DRAW alert retries.
Dev187 probe Rust134/source17 PASS; alpha0/128/255 regression verifies compositing.
Native repeat and strict Clippy pending; no release acceptance. Evidence:
outputs/loupe-upstream-dev186/native-surface; outputs/loupe-upstream187-*.log.

2026-10-07 source prerequisite prepared locally: nondefault
`loupe-upstream-probe` Dev186 requests the affected layer before FSTR through
SDK LayerRenderOptionsSuite2; layer-space sampling uses the same native/view
projection as corner placement. No project flag mutation, no additional request
per mouse movement, unchanged ordinary composition source. This experiment
intentionally omits lower-layer composition and is NOT the accepted product fix.
Probe/default Rust133 each, probe strict Clippy and host source17 PASS.
First compile used incorrect Fixed conversion names and failed; corrected against
the installed wrapper before successful checks. Native source/coordinate coverage
Surface + Perspective (2D/3D, nonidentity layer transform) NOT_RUN. No installation
or publication. Next: bounded single-layer upstream pixel proof, then a separate
full-composition exclusion route and multilayer acceptance; no inferred PASS.
Evidence: outputs/loupe-upstream-{tests,default-tests,clippy,contracts}.log.

Updated: 2026-10-07. This is the concise entry point for the current product and
continuation. [Dated checkpoints](current-status.md) retain historical detail;
their older holds and release-policy statements do not override the state below.

## Active update — Mac and Windows, rules8.0.0

**Reopened2026-10-07 user report:** loupe shows incorrect enlarged content;
Surface/Perspective do not reliably hold the picture during corner dragging,
with a reported dependency on editing internal grid deformation. Earlier small
fixture USER_REPORTED visual PASS is retained as historical scoped evidence,
not current release acceptance. Release visual correctness FAIL/OPEN pending
reproduction and correction. Exact before/after-grid workflow clarification
requested; both before/after-grid paths remain in the native acceptance scope.

Dev180 loupe acceptance FAIL (latest user report): lens remains entirely white,
not just a white cell at one location. User photo IMG_2530.HEIC shows the white
lens over a checkerboard in a Comp viewer at50% / Full. Source-frame selection,
coordinate mapping and image upload remain separate unproven causes; do not
claim cache invalidation fixed content. Current observed AE has an unrelated
working project open; no script or gesture was dispatched into it. Next causal
check must use a small owned asymmetric-color fixture and compare frame content,
sampling coordinates and uploaded lens pixels without adding render requests.
Nondefault loupe-image-probe (Dev182) prepared for that check: max16 private
numeric records covering receipt size/region/channel range and first lens
sampling coordinates/range per gesture. No raw image, project name, extra frame
request or ordinary-build logging. Probe Rust132/source17 PASS; native NOT_RUN.

Dev182 guarded native color fixture: exact installed/loaded identity PASS;
Surface Top Right640,0->560,80 only, layer position unchanged. Receipt and lens
sample channel ranges0..255, so a uniform-white source is not demonstrated.
First sample center640,0, one frame-coordinate unit maps to4 source pixels.
No held visual capture or Undo acceptance; original report remains FAIL/OPEN.
45s trial cleanup PASS, all9 foreign components returned, idle-cache key unchanged.
Evidence: outputs/loupe-image-dev182/native-ecw-r2/loupe-image.jsonl.
Dev183 nondefault probe adds bounded raster-interior range and SDK supplier
layout capability/preference checks to separate compositing from image upload.
No ordinary runtime behavior or additional frame requests changed.

Dev183 exact sourcee60d49b/BID EGFX-956e1f13872853f2042f5fc2 native probe:
lens raster interior range0..255 over1764 pixels; supplier supportsARGB/BGRA,
prefersBGRA. Thus neither uniform receipt nor uniform prepared raster explains
white output in this color fixture. Image upload remains a hypothesis, not a
confirmed cause. Dev184 now selects the supplier-supported preferred pixel
layout per SDK DrawbotSuite.h:178-180 and losslessly converts only the UI bitmap.
No new render/frame request. Regression checks preserve all RGBA channels and
transparent exterior for both layouts; original visual symptom pending native.
Evidence: outputs/loupe-image-dev183/native-ecw/loupe-image.jsonl.

2026-10-07 loupe source scope CONFIRMED: original affected-layer cells without
FSTR distortion remain visible, with background/lower layers. Latest user answer
explicitly retains original cells. No outstanding own-layer visibility question.
Full-composition exclusion is implementation OPEN; upstream-only rendering cannot
satisfy the multilayer contract. SDK routes are in corner-loupe-plan.md. Cursor
fix e5eec9e remains local, not installed/native accepted.

2026-10-07 Dev184 native manual USER_REPORTED PASS: lens is colored,
Surface picture freezes during corner drag, CmdZ restores previous view.
This confirms the white-content regression is no longer observed in this fixture;
not certification of all coordinates/modes or the original heavy-scene hang.
New user contract: loupe must exclude FSTR rendering during corner movement.
Confirmed source scope retains original affected-layer pixels and visible lower
layers. Existing L3 cannot be silently interpreted as upstream-only. Independent cursor defect:
set_drag_cursor(true) ran on every DRAG and overwrote the transparent lens cursor
with a hand. It now preserves active lens hiding; release/cancellation cleanup
unchanged. Corrected ordinary candidate/native cursor verification pending.

Dev184 ordinary source8319751 / BID EGFX-364f96a71fa962fb92433677 installed:
all file hashes/strict signature PASS; exact AE25.6 loaded canonical path/UUID
DDE88B0B-E8DB-3EBA-8853-7F43BD6B4BDF PASS. Default Rust133/source17/Clippy PASS.
Guarded Surface fixture: only TopRight changed640,0->560,80; Cua CmdZ restored
previous gesture592,48. Other corners/layer position and existing TopLeft /
Affected Lines keys unchanged. Native point commit/one-step Undo PASS for this
fixture; held-image/loupe visual contents NOT_RUN, Windows184 NOT_RUN. No RAM
Preview. Cleanup PASS: AE absent, idle-cache key unchanged,9 components returned.
Normal184 remains installed, diagnostics removed. No push/main/release action.
Evidence: outputs/loupe-dev184/checkpoint.json and native-surface readbacks.
Remaining causal limitation: short permitted helper cannot reliably capture a
held lens image; original white-lens report remains FAIL/OPEN until direct visual
comparison on ordinary184. Supplier preference correction is not proof that
ARGB support is defective. No release promotion from these scoped results.

Dev180 local correction: successful corner release preserves only scoped hover
ownership for a repeated press without cursor motion. Effect-panel DRAW now uses
the same effect/time/mode claim instead of re-enabling native Point picking.
New gestures discard prior loupe pixels and queue one ECW refresh; no per-move
frame requests are added. Cancelled gestures clear the claim. Rust132 and
source contracts17 PASS; native loupe content/held-image acceptance NOT_RUN.
These code-path corrections do not certify the original heavy-scene hang.
Dev180 source2f1aae7 / BID EGFX-6befdfecb8c7b1ba6c4cf2d7 is packaged and
installed; all installed files and strict signature PASS. AE25.6/PID56614 loaded
UUID9BABC9E5-9462-3F72-9CCA-77C8A206A9A2 from the canonical path. First
trial missed the corner (unchanged readback): gesture/Undo NOT_RUN. Second
Surface trial stopped before scripting after two UI-read timeouts. Both trials
ended with AE absent, idle-cache key unchanged, all9 components restored.
Loupe pixels and held-image before/after internal-grid edits remain NOT_RUN;
release BLOCKED. Private checkpoint: outputs/owned-corner-dev180/checkpoint.json.


Latest2026-10-07 user acceptance: ordinary Dev177 Surface picture holds during
corner movement, grid/loupe move, release updates picture; USER_REPORTED PASS.
User accepts transient gray coordinate rows on corner hover for this release.
Dev179 disabled-topic research confirms DRAW delivery but native caption and
stopwatch remain gray; prototype is not promoted. Ordinary Dev177 restored with
all file hashes and strict signature PASS. No product UI/key format rewrite.

Exact pushed c48385c CI: Mac37651811024 and Windows37651811038 both PASS.
Windows native AE acceptance of this corner-drag change remains NOT_RUN;
Perspective visual hold/loupe/release/Undo is now USER_REPORTED PASS on Mac177. Original heavy-scene hang
regression is not certified by small guarded fixtures. Main/release not published.


**Current release contract2026-10-07:** user explicitly requires finishing the
release WITH frozen-image Surface/Perspective corner dragging. Do not split
this requirement into a later release or substitute ordinary recovery. During
movement only grid/loupe change; one canonical corner commit and selected-quality
render on release, old keys/Undo/coordinate controls retained. Implementation and
native acceptance remain OPEN. Dev170 is a diagnostic, NOT a release candidate.
Source199054a pushed with explicit user authorization for Mac/Windows CI.
Main/merge/release publication remains unauthorized. No deadline is proven.

Latest native evidence: Dev172 sourcea20d235 / BID EGFX-54de45fa482b59bcd7382511
was built, installed and loaded from the canonical Mac path; UUID
420F804A-3423-313B-93A4-ACE6BF92AAD8 matched. Surface/PID32252 produced12 tentative
updates and one final Point commit, with no native Point supervision during
movement. Other corners/layer position unchanged; four numeric fields enabled
after the gesture. Peak948464752bytes, no guard limit. Surface Undo and held-image/
loupe visual acceptance NOT RUN (deadline expired before Undo dispatch).

Perspective/PID33221 delivered the same owned transaction and successful one-step
Undo with existing Point and Affected Lines keys. It also exposed a real defect:
PF CHANGED_VALUE shifted BOTH Point keys0/24->12/36. Undo restored0/24 and retained
radius3/6. An independent native Adobe Corner Pin/PID33953 control changed ONLY
current Point key0->12; second24 stayed unchanged, Undo restored both. Therefore
animated corner commit acceptance FAIL for Dev172, not a compatibility PASS.
Peak1018904056bytes for Perspective,1018173680bytes for native control; no guard
limit. Each45s trial ended with AE absent, idle-cache key unchanged, all9 temporarily
excluded components verified returned. Private results under
outputs/frozen-corner-dev172/native-surface-r2/,
native-perspective-r2/ and native-point-control-r2/. Deadline misses and one refused
non-foreground helper trial remain NOT RUN, not plugin failures. No security grant
or user preference was changed; no user project/RAM Preview opened.

Ordinary Dev173 source199054a / BID EGFX-1aaa6971d1fe3ced9c2fc0ed / UUID
A90B7554-027B-346A-ABAB-C18BE7A0AA76 built, packaged, installed and exactly loaded.
Default native-plane + owned-corner-drag includes no diagnostic journal/census.
Mac Surface/PID39293 current-key edit and one Undo PASS: Point keys0/24->12/24->0/24;
Affected Lines3/6 retained. Perspective/PID40064 between-key insertion preserves
both old keys and creates only the current key. However one Undo reverts its value
but leaves a third key: initial key-removal observation FAIL; a later stationary
baseline below shows this key predates the moving gesture. No held-image/live-loupe
visual acceptance follows from these readbacks. Both45second guarded trials ended
with AE absent, unchanged idle-cache key and all9 excluded components restored.
Private evidence: outputs/owned-corner-dev173/native-surface/ and native-perspective/.

Mac CI37648115557 and Windows CI37648115646 on199054a were IN_PROGRESS at that
checkpoint. They do not prove runtime acceptance. Original drag/RAM hang root
cause remains UNKNOWN; no user project or RAM Preview was opened in these trials.

Dev175/PID42388 discriminating baseline read after the stationary setup gesture
shows the third Point key already present BEFORE the moving gesture. One Undo
restores exactly that baseline. Thus the Dev173 observation does NOT establish a
separate Insert/Set undo-group defect; the atomic-batch hypothesis is unproven and
its raw-SDK experiment is removed from the product. Keep199054a's simpler validated
Keyframe Suite path and current-key-only behavior. A first175 trial stopped before
DoScript because a private harness module was omitted; cleanup PASS, gesture NOT_RUN.

Installed ordinary Dev177 sourceaf3138e / BID EGFX-4d300dce5296aab8446a758d /
UUID615266E8-3099-32E8-AD83-F91C65A1D509 retains199054a's functional code; only
build counters and documentation differ. Exact build/package/install/load PASS.
130 Rust tests and16 source contracts PASS. Corrected native Perspective/PID43428
sets hit state on unanimated Top Right; readback proves Top Left still has only
two keys before movement. Moving Top Left creates a third key24/8; one Cmd+Z
restores the exact original two-key track, midpoint and Affected Lines3/6. PASS.
Peak1004191512bytes, no cap event;45s cleanup PASS, cache key unchanged, all9
components returned. Private outputs/owned-corner-dev177/native-perspective/result.json.

CI199054a Mac37648115557 and Windows37648115646 PASS. Both downloaded CI ZIP
SHA256 and manifest source identities verified. Windows PE/AEX checks/tests PASS;
Windows AE runtime NOT_RUN on this function. The local177 bytes are separately
identified from CI173; equivalent functional source is not exact-artifact proof.
The user authorized publication of this update branch for CI; main/merge/release
remain unauthorized. No original heavy project or RAM Preview repeated.

Earlier checkpoint remaining (Surface visual now USER_REPORTED PASS above):
held image/live grid/live loupe first/repeated press and final update,
cancellation/restored coordinate UI in a bounded ordinary manual check; latest
Windows native confirmation. Visual check readiness requested. Original hang root
UNKNOWN; release WITH the corner-drag function remains mandatory and OPEN.

### Earlier checkpoints — preserved evidence

**ACTIVE after user continuation2026-10-07.** Installed Dev170 exact canonical
files/modes/signature PASS, source908bcc8 / BID EGFX-6eaeae9fbb8a77354cf5cc58.
The paused PID13780 startup stopped before project opening; no gesture. On resume,
PID13833 held a nonempty project and~10.5GB footprint; no experiment was run in it.
Closed that exact AE without saving under existing user authorization. Cause of
that footprint UNKNOWN. PID13039 input delivery remained UNKNOWN (deadline
missed after Cua). PID15386 exact loaded identity PASS; logical-coordinate Cua
corner drag throws AXError.notImplemented, so gesture NOT RUN, not native routing
FAIL.45second guard completed with no memory limit; preference restore full hash
PASS, all9 excluded components restored. AE absent at checkpoint. Next: existing
authorized CGEvent helper, screenshot plus exact owned-window Retina geometry,
one<=1second gesture and automatic point/layer-position readback. No new
permission, renderer changes, freeze acceptance, push or release. Dev170 hides
four ECW rows only as an isolated diagnostic; retain production coordinate UI.
Private evidence: outputs/corner-ownership-dev170/native-priority/result.json,
native-priority-logical/pause-cleanup.json, resume-host and
native-priority-logical-r2/result.json. Historical installed Dev169 statements
below are superseded by this installed Dev170 checkpoint.

**New startup incident2026-10-07 — live gesture remains on hold.** User's AE
crash-alert screenshot reports last thread4288156/AEDoScriptCommand/result1.
That exact thread matches private sample PID16099: main thread in BirthPrefs
startup modal with DoScript error-report handling. FSTR/AEHL absent from sampled
loaded-image table; no fixture or gesture had opened. Association with our early
startup-script preflight is established; crash root cause UNKNOWN. Full native
crash dump NOT FOUND in inspected AE/system diagnostic locations; screenshot and
322 exact-PID unified-log records retained locally. This replaces the earlier
ambiguous desktop-ready blocker; read-only Cua now observes a new AE PID17712 with
user working project. No new launch/script/input/project change in this triage.

Prepared private runner repair: read-only main-project UI readiness precedes the
FIRST DoScript, tied to exact PID/birth and fresh<=15second observation; missing/
modal/incomplete/stale/reused-PID UI cannot dispatch the command.8 offline barrier
checks and Python syntax PASS; native retry NOT RUN. Do not repeat the old
SDK-before-UI startup sequence or claim a product crash fix. Existing short helper
transaction still gates Retina/window geometry, ARMED/current identity/caps,
>=10seconds remaining and immediate point/layer readback. PID16099 cleanup had
restored actual25.6 full preference hash/all9 components before this alert report;
current user-session preferences are not overwritten. Dev170 remains installed;
freeze implementation/native acceptance OPEN, no push/release. Private incident:
outputs/corner-ownership-dev170/startup-alert-20261007/triage.json.

**Dev167 REJECTED — native freeze FAIL and new memory-pressure incident.**
User reports continued effect updates during corner drag, then AE/Photoshop/
Telegram stalls. Private census exact BID EGFX-412cf851b1695fc8cfc7c32c/PID91860
records3 gestures (5204/6112/1998ms), SmartPreRender HI/full68/32/24 and identical
SmartRender counts;124 per stage, not248 frames or a timing benchmark. At collection
AE no longer running, so current hang sample NOT RUN. macOS JetsamEvent dated
2026-10-07 11:13:10+0200 lists AE PID91860 largestProcess; memory-pressure event
confirmed, exact FSTR/host/cache allocation cause UNCONFIRMED. No AE termination
performed in incident response, no Photoshop/Telegram control, no further live run.
The DRAW/AdjustCursor flag route does not meet the native corner contract in this
fixture; do not retry/promote it. Dev168 ordinary recovery removes both experimental
features by building default/native-plane only; renderer, loupe, keys, selected
quality remain unchanged. Build/install PASS; this is quarantine, not a hang fix.
Freeze contract remains OPEN. Next offline research: an owned corner interaction
that keeps tentative coordinates outside saved parameter streams and commits once
on release, while preserving loupe, Undo and old keys. Native Point hit priority,
UI compatibility and memory growth must be resolved before another host trial.
Private incident evidence: outputs/deferred-corner-dev167/incident.

**Historical installed diagnostic:0.9.4 Dev169**, source2b53747 /
EGFX-376c92a761bbecece5122c43. Ordinary recovery checkpoint below is historical;
new rendering acceptance is not claimed. Current host/diagnostic details follow.

**Previous installed ordinary recovery:0.9.4 Dev168**, source57151ca /
EGFX-09791ccfe60cf0457f6663e0. Exact release features default/native-plane only;
no deferred-corner or quality census. Warning-free build, package/bundle/PiPL/
exports/signature/archive/manifest/native installer PASS; native Installed/
Previous saved, exact canonical files/executable bits/signature and installer
process exit PASS. AE absent before/after; no relaunch and no new native trial.
Ordinary source identical to Dev164 except About/PiPL development counter; prior
ordinary tests are reused only for unchanged code, not Dev168 native acceptance.
Private recovery record: outputs/corner-recovery-dev168/verification.json.
Rules reread2026-10-07: AI_ENTRYPOINT, Process§2–3, Engineering Debugging Protocol,
§17–19/38, Native§23, Workflow§8. Freeze FAILED, memory-pressure incident CONFIRMED,
allocation root cause UNKNOWN. No numerical speed benchmark is requested; memory,
resource-lifetime and stability diagnostics are still required after this failure.
No further live gesture until an offline review and a bounded diagnostic plan
identify a discriminating signal, memory stop condition and owned-host baseline.

Dev169 passive census: snapshots, loupe copies, borrowed worlds and checkins;
no additional frame requests or quality changes. Ordinary120/probe122 Rust,
strict probe Clippy,16 host contracts and10 external guard tests PASS. Exact
source2b53747 / EGFX-376c92a761bbecece5122c43 release build, bundle/PiPL/exports/
signature/archive/manifest/native installer PASS. Features exactly
default/native-plane/resource-census-probe. The initial installer launch rejection
was resolved by the user's later explicit confirmation; Dev169 is installed.
Native loaded UUID3826B5EF-8479-3D2B-A9E4-ADF619F5F8A2/path PASS. Diagnostic
resource results and limitations: [contract](resource-census-plan.md).

Startup observations PID1992/PID2529 stopped before a gesture. The first growth
bound mixed cold loading with idle; the second completed setup but exceeded its
initial1GiB cap. The evidence-based loading/idle cap is now1.25GiB, with unchanged
total1.5GiB/growth256MiB/maximum45seconds. No further cap increase. All runs are
saved disposable copies; exact PID/path/birth guards verify each signal.

PID3572 armed at1070084744bytes then exceeded the absolute cap at1740308152bytes
after8.989seconds, with no mouse gesture. Last delivered UI census has snapshots
9/9/live0, worlds8/8/live0, eight successful pixel checkins, no loupe copy/poll.
The stopped sample contains active FSTR rendering; a last UI record cannot
exclude subsequent work. Cua getApp timed out in that interval; no screenshot
ran. A later empty AE PID3656 was separately proven unsaved/zero-items and closed
with exact identity checks. This lookup is excluded from follow-up experiments.

**Idle-render confound localized; original hang cause still UNKNOWN.** Local
AE preference has Cache Frames When Idle enabled with8000ms delay. With no Cua/AX,
mouse or playback, PID6028 reproduced growth at8.996seconds:1081471792bytes idle
to1703626704bytes, expected guard stop/exit PASS. Thus Cua is not necessary for
this idle growth. With only the runtime idle-cache setting disabled, PID6756
completed20seconds without a limit; maximum1088548432bytes, ending989884872bytes.
Preference readback false during the test. The initial on-disk restore/hash claim
is INVALID: it checked the25.0 preference file rather than the running25.6 file.
The off preference had persisted in25.6 despite suppressing save-on-quit. A later
fresh-frame preflight rejected this changed baseline before opening the scene.
With AE absent, restored only the actual25.6 key00 to its original01 (original
true confirmed by PID6028 runtime readback); every other byte/mode preserved.
The off run recorded only a113x64 thumbnail; its full viewer image may have
reused a warm host cache. This is not a fresh full-frame rendering/stability PASS
or proof that caching caused the original drag/RAM Preview incident. No product
setting, MFR, quality, render source or memory cap was changed.

**Fresh full-frame observation PASS (scoped):** PID9346 / AE25.6x101,
same installed Dev169, background idle cache temporarily disabled. A0.125px
corner change in the disposable in-memory copy forces a fresh render; largest
borrowed world33177600bytes confirms1920x1080/32bpc, two successful pixel
checkins, all delivered snapshot/world scopes balanced.20seconds, maximum
1148727056bytes, no limit. No native gesture or RAM Preview; freeze and original
hang repair remain OPEN. Earlier fresh-frame attempts stopped before project
opening (changed preference baseline, then incorrect host-version suffix check),
not renderer failures. Runtime host version is25.6x101, not25.6.*.

**Current cleanup PASS:** AE absent, all9 authorized Hot Loader test bundles
returned byte/mode-exact, actual25.6 idle-cache key01 restored explicitly;
full captured preference-file hash unchanged after the successful test. Dev169
remains installed. Private fresh-frame evidence:
outputs/resource-census-dev169/fresh-frame-idle-off-r3/result.json.
Incorrect25.0 restoration claims are superseded by
outputs/resource-census-dev169/actual-preference-restored.json.

**Next diagnostic: Dev170 corner input ownership**, nondefault
corner-ui-ownership-probe. It hides only the four canonical Point ECW rows using
NO_ECW_UI while preserving types/IDs/Timeline keys. This is a discriminating
native-hit-priority experiment, not accepted product UI or frozen output. A
bounded private UI journal separates custom CLICK/DRAG from native point
supervision; ordinary behavior and renderer unchanged. One guarded<=1second
gesture only after loaded identity/ARMED/current screenshot. Do not promote
hidden corner rows; production must retain their coordinate controls/animation.
Source908bcc8 / BID EGFX-6eaeae9fbb8a77354cf5cc58: default120/probe122
Rust, strict default/probe Clippy,16 host contracts,410 checksums, warning-free
release, bundle/PiPL/exports/signature/archive/manifest/native installer PASS.
Exact features default/native-plane/resource-census-probe/corner-ui-ownership-probe.
Historical packaging checkpoint: Installer awaited action-time confirmation;
the user subsequently installed Dev170. Exact disk verification PASS in the
current checkpoint above; actual delivered native corner routing still NOT RUN.
Private package/check results: outputs/corner-ownership-dev170/verification.json.
See [diagnostic contract](resource-census-plan.md). New-source Windows CI/push,
main, release and original-project writes remain unauthorized.

Current user acceptance: ordinary Dev164 is only slightly smoother and still
jerky — responsiveness FAIL. Approved2026-10-07: hold the last image during
Surface/Perspective corner movement; grid/loupe remain live, selected-quality
render resumes on release. Internal line deformation remains live.
Historical Dev167 preparation (superseded by rejection above):
local nondefault deferred-corner-probe tests125, ordinary tests120,
strict probe Clippy and16 host contracts PASS. UI-only NEVER_UPDATE / release
ALWAYS_UPDATE|UPDATE_NOW experiment; no saved data, quality setter, extra source
frame or render-worker UI calls. Native Point loop honoring these response flags
was UNKNOWN before the failed trial; ordinary recovery is now Dev168. Both earlier hang
incidents remain OPEN. Separate review covers per-owner/window/time state,
context clearing, once-only release and no repeated DRAW invalidation.
Build/package PASS: clean sourcef3943d51a6ccd3d712f99b9d440ee9603d47cdc9,
BID EGFX-412cf851b1695fc8cfc7c32c. Warning-free release, bundle/PiPL/exports/
signature/archive/manifest and native installer verified. Features exactly
default/native-plane/interactive-quality-probe/deferred-corner-probe.
Previous installed diagnostic: **0.9.4 Dev167**, sourcef3943d5 /
EGFX-412cf851b1695fc8cfc7c32c. After user confirms AE available, closed empty
Developer session and copied saved owned Dev164 scene; no foreign plugin edits.
Native installer reports Installed/Previous saved; exact canonical payload,
executable bits/signature and installer process exit PASS. No closed UI query.
Normal AE PID91860 loads UUID0475D29A-CB43-374F-8AB1-D2F905E4C59D from canonical
FSTR path, exact identity PASS. Opening transport times out; later registration
file and responsive UI confirm completed opening. One registered FSTR, owned
32bpc Full/Final Surface scene, selected visible grid/grips PASS. Short user
corner/release freeze result FAIL, with memory-pressure incident recorded above; no long drag
or RAM Preview. Earlier foreign Calibration component absent from loaded sample;
other AEHL components remain loaded and unchanged, not an isolated-host claim.
Private evidence: outputs/deferred-corner-dev167. No native behavior promotion,
new-source Windows CI, push, main change or release.

**HOLD — native interactive hang.** Ordinary Dev152 source39616fa / Build ID
EGFX-c3c62a10f10d4026fcab974d: user reports slightly faster but jerky corner drag,
then AE hangs during a15-second diagnostic gesture. User confirms only AE hung;
this is distinct from the earlier whole-Mac RAM Preview incident, still OPEN.
Exact-process Apple sample retained privately in
outputs/corner-clean-dev152/drag-profile/sample.txt. All1340 samples of one AE
render executor stop at BEEp_AbortProc / objc_retainAutoreleasedReturnValue,
called through the32bpc plane renderer's AE abort callback. This localizes the
captured worker; it does not prove callback misuse, duplicate-render attribution
or the original RAM incident's cause. SDK25.6 AE_Effect.h:2678–2700,2727 and2784
permit periodic abort calls with the current effect_ref; reviewed synchronous
bridge uses the callback's current PF_InData and polling stays on its caller.
Do not remove cancellation or reduce output quality as a speculative repair.
Main thread sampled in AppKit event wait; captured footprint11.0GiB peak11.3GiB.
After shutdown, a16GiB Mac reports18.15GiB swap used and61% memory free; these
post-incident values do not establish memory pressure during the gesture.
Verified PID44395 did not exit after SIGTERM; exact-path-checked SIGKILL completed.
Owned pre-test project copy retained; no other application closed, no RAM Preview.
No promotion, release or further long native drag. Previous discriminating candidate:
Dev156 loupe-source-disabled-probe, a nondefault/local-only feature suppressing
all secondary loupe frame requests, including warm requests. No diagnostic log,
render algorithm, keys, parameter values, quality, async-manager flag or ordinary
Dev152 behavior change. Lens pixels are intentionally unavailable only in this
probe. Build/package checks and a separately bounded native comparison are
pending; this is diagnosis, not a product fix or acceptance.

Previous installed diagnostic: **0.9.4 Dev156**, source6a2d9cdf3a8e1a457d445e6d967a79403c5af08e,
BID EGFX-0408ee6c01d9e195b8c242e9.116 probe +116 ordinary Rust regression
tests, strict probe Clippy,16 host contracts and existing plane ASan/UBSan matrix
PASS. Release build has no warnings; bundle/PiPL/exports/signature/archive/
manifest and native Install/Restore package PASS. Installer reports Installed,
previous saved; exact canonical installed files/permissions/signature and loaded
UUID/path PASS. No installer re-query after outcome exit. Feature record contains
only default/native-plane/loupe-source-disabled-probe, no logger or PREVIEW probe.
First opening transport used an incorrect application identifier and failed
before script execution; corrected Info.plist identifier opened the owned copy
but exceeded15-second transport timeout. Cua subsequently confirms completed
opening and responsiveness; separate1-second sample has idle render executors,
so this timeout is not classified as another hang. Guarded effect selection PASS;
Surface mode,32bpc,100%/Full/Final, Show Grid OFF and native corner grips visible.
User comparison: slowness persists without loupe source requests; adaptive
resolution and Preview quality visibly improve drag smoothness versus Full/Final.
No numerical benchmark or hang recurrence acceptance; Undo not separately confirmed. This
diagnostic intentionally has no lens pixels and is not for normal product use.
No further long drag, RAM Preview, publication, promotion or release. Local
checkpoint: outputs/interactive-hang-dev156/verification.json. Dev156 changes
were local at that checkpoint; normal Dev152 and original RAM incidents remain OPEN.

Active follow-up: [interactive plane throughput plan](interactive-plane-throughput-plan.md).
Optimize the large general sampler through AE scheduling, preserving exact pixels,
selected quality and cancellation. Dev160 implemented:120 ordinary +120 source-disabled
regressions, strict ordinary Clippy,16 source contracts and existing plane
ASan/UBSan matrix PASS. **Last installed ordinary:0.9.4 Dev160**, source
b585977f90bb7ea1ce15086aaea811dafa9c9c1a / EGFX-44e5125b553fe875bb72e116.
Canonical files/permissions/signature and loaded UUID/path PASS. Loupe press /
repress and Undo USER PASS; Full/Final corner drag still jerky, responsiveness
FAIL. Both exact-source Mac37485816445 and Windows37485816125 CI PASS; Windows
native interaction NOT RUN on this source. No quality substitution enabled.
User-authorized61bdfe5 and b585977 pushed for CI, no main/merge/release action.
Both hang incidents remain OPEN, no promotion. Private current checkpoint:
outputs/interactive-plane-dev160/verification.json.

Installer reopening root cause reproduced without reinstallation: reading the
closed app through Cua starts a new process. After Close, verify process exit
without querying the installer binding. Evidence: private
outputs/interactive-plane-dev160/installer-relaunch-probe.jsonl. Installer source
already exits after success; no speculative installer code patch required.

Temporary Preview while dragging remains research, not implemented. SDK quality
is HI/LO, not an export discriminator. Local-only/nondefault Dev162 census observes
existing callbacks without changing pixels/keys or requesting frames.122 probe /
120 ordinary tests, strict probe Clippy and16 host contracts PASS.
**Previous installed diagnostic:0.9.4 Dev162**, local source
1f90454666f0327b5f495d0dc216af362fc139a9 / EGFX-59ae18a6c89d2e8e727b577d.
Release build, bundle/PiPL/exports/signature/archive, canonical installed payload
and loaded UUID/path PASS. Features exactly default/native-plane/interactive-quality-probe.
Owned one-effect scene open at32bpc Full/Final, loupe retained. Installer exited;
no closed binding query or repeated launch. Initial AppleEvent opening timed out;
later result file and Cua confirm completed opening. Point.value script reads
still return AE invalid numeric result; scalar/key-count scope checks PASS.
First automated gesture refused before input (not foreground); second moved the
layer rather than a corner and was immediately undone, visually restored. Neither
is accepted as corner census evidence. Later user corner gesture records
SmartPreRender11 HI/full and SmartRender11 HI/full over1067ms; no LO/reduced
requests observed. These are stage counts, not22 frames or a speed benchmark.
Screenshot confirms Top Right changed; Undo restoration NOT CONFIRMED for this
gesture. No RAM Preview/long drag. Private checkpoint:
outputs/interactive-quality-dev162/verification.json. This diagnostic is not the
automatic Preview implementation or a normal delivery. New source not pushed;
prior green CI belongs to b585977. See throughput plan for sources and remaining checks.

Local next candidate Dev164: general sampler resolves four source-row bases and
four X offsets once per footprint, replacing repeated16-tap bounds checks. Map,
weights, summation, NaN fallback, quality and loupe unchanged; common Mac/Windows
code, no host API or frame request. Exact old/new-address full-image matrix PASS
in production O3/fast and isolated O1/fast ASan/UBSan;28 CTest,120 ordinary Rust,
strict Clippy and16 host contracts PASS. Separate review PASS for sparse/empty
checkout, int64 offsets, pointer bounds, immutable source and local worker data.
Skill scanner omits C++: NOT ASSESSED, no empty-finding pass claim. Prepared-axis
experiment rejected because O1/fast differs by one ULP; original math restored.
An all-cache O1/fast parity difference also reproduces on unchanged HEAD renderer
and tests; standard preflight O1 and production O3/fast matrices PASS. This is
recorded as an existing configuration-specific parity limit, not hidden by the
new isolated-address check. Build/install/loaded identity PASS; native response pending. No push,
release or incident closure. Private evidence: outputs/prepared-axis-dev164.

**Current installed ordinary:0.9.4 Dev164**, source
99f6e2b8f2fc5ba3f031bb5a4eda5d1de29474e8 / EGFX-26e3006115f77318033d8d3a.
Warning-free release build, package/exports/PiPL/signature/archive/manifest/native
installer and exact installed bytes/permissions/signature PASS. Features only
default/native-plane; no diagnostic logging, quality census or source-disabled
probe. Installer reports Installed/Previous saved, exits; no closed UI query.
AE PID58561 loaded canonical binary UUID72BF227D-5DB5-3DA6-B5B0-0B6F39138683,
path/UUID PASS. Cold UI/AppleEvent opening times out; later completed registration
file and responsive Cua confirm the saved owned copy is open, not a new hang.
One FSTR render graph,32bpc Full/Final Surface, selected Grid Positions and visible
grips. Current Top Right1308,447 retained from prior user gesture; do not claim
that gesture's Undo restored1192,551. Short new-candidate responsiveness/Undo/loupe
acceptance PENDING. No RAM Preview, release, incident closure or new-source CI.

Current mode display order: Comp mode / Layer mode / Surface mode / Perspective.
Surface is the former Flat caption; saved ordinal2 and internal enum stay unchanged.

Current scope: implement the newly authorized four deformation modes and disabled
Demo design for both platforms; retain native Install/Restore delivery obligations.
Added centered corner loupe with a central target in Surface/Perspective, UI only,
Mac and Windows; see [loupe contract](corner-loupe-plan.md). Native loupe
acceptance is pending.
See [mode/design contract](four-modes-demo-plan.md). Previously accepted installer
payloads remain unchanged until a new plugin candidate is separately accepted. Branch feat/next-update is published for the
user-authorized Windows CI continuation; source fixes/docs on that branch are
allowed. No main, PR, merge or release action is authorized by this continuation.
The earlier local-only and Windows-deferred decisions were superseded by the
user's later Windows implementation and specific branch/CI authorization.

Previous candidate: **0.9.4 Dev100**, ordinary source6d895e4024fcceff7fa80c5223f05daebe30cddd,
Build ID EGFX-4613b9d0b6f5453e20445fb7. Package/signature, atomic replacement
and loaded-image identity PASS. Dev92 Comp drag/Undo/mode/menu acceptance remains
bound to that artifact; Dev100 adds None transparent edge behavior. New default
None4 and old saved Wrap2 preservation checked on Dev96; Dev100 None4 save/reopen
PASS. Native AE25.6 Dev100 None output has245102 transparent pixels, while Clamp
fills the whole640x480 destination. Evidence: outputs/edge-none-dev100/native-pixels.json.
Initial frame capture closed/reopened the project too soon and produced no PNGs;
subsequent capture while project stayed open produced both frames. UI/manual
acceptance is pending: Effect Controls is locked to none, user asked to unlock
and inspect. Canonical edge row now has NO_ECW_UI as well as INVISIBLE.

97 Rust/28 CTest/282 Python/all JavaScript/15 host contract checks and Clippy PASS
for the None implementation. Native CPU/Metal parity24 cases PASS; not host GPU
or real speed evidence. Windows CI37306339582 PASS for installed source;
downloaded AEX integrity PASS, Build ID EGFX-12022254e434feabf472062a,
SHA2564efca39bb44072196307651bd2a173ac2d787918d1ad3c6d313c7da759d5e4d2.
Windows new behavior in AE NOT RUN. Mac CI37306339475 FAIL: quality test reference
switch omitted None under warnings-as-errors. Reference now includes transparent
taps and all four edges in8/16/32bpc; exact ASan/UBSan strict compile/run PASS.
Production renderer/binary unchanged by this test correction. Follow-up Mac37307305064 and Windows37307305067 PASS on test-only ecb9ac8.
No release, merge, installer replacement or diagnostic build.

New user decision: fixed Clamp and disabled Edge Behavior in Layer/Perspective;
Comp/Surface remain editable. Saved choices are preserved. Previous **0.9.4 Dev104**, source9bdf4470a7bd46000d8bc349d33eeb41729a9375,
Build ID EGFX-ab9b2b3b3a3d8e9cb7493c49. Package/signature/atomic replacement/loaded identity PASS.
98 Rust/15 host contracts/Clippy PASS. Native AE25.6 temporary-wave pixel comparisons
PASS: Layer/Perspective Clamp and saved None outputs are byte-exact, Comp/Flat differ.
Original choice restored; reference scene and prior binary backed up. Native Cua UI PASS: disabled Clamp in Layer/Perspective, active None restored
in Comp; Flat active None, hidden canonical edge row absent. Windows CI37311196442
PASS; Mac37311196440 PASS. Windows native AE NOT RUN. Initial installer
rejected branded archive before swap; correct internal archive installed successfully.
Neutral-grid comparison initially failed an invalid expectation that edges must differ;
separate deformed fixture verified intended policy. No transfer of old UI acceptance.

Dev68 native loupe FAIL: user saw no loupe. Probe70/70b observations established
that transient PF effect references cannot identify the same native Point drag.
Probe74 stable AE stream identity restored the loupe, but native visual FAIL:
incorrect/fallback image and AE internal verification failure from
PF_GetContextAsyncManager in the Composition UI context. Original failures remain
recorded. Probe78 requests frames only from the Effect Controls custom UI context
and copies owned pixels; viewer drawing makes no async-manager request.
Probe78 Perspective USER-REPORTED PASS: correct image, disappears on release,
no error. Dev80 additionally hides the local hand cursor only while a lens is
successfully drawn, for Mac and Windows. User's edited test scene preserved
and reopened. Hidden-Effect-Controls coverage remains pending.
Evidence: outputs/corner-loupe-probe70, corner-loupe-probe70b,
corner-loupe-probe74, corner-loupe-probe78, corner-loupe-dev80.

Dev80 source published only to authorized feat/next-update for CI.
Windows37300965301 FAIL at source2c2c931 (ANSI cursor resource passed to
LoadCursorW); fixed in d62e9cf and new CI queued. Mac37300965421 result
remains separately bound to source2c2c931. New Windows artifact identity/native
acceptance pending.

Comp whole-frame output and sorted Comp / Layer / Flat / Perspective native
workflow user acceptance PASS on Dev92. Retained Dev84/Dev88 partial/failure
records remain historical; Dev92 supersedes the clipping defect. New None edge
behavior has native pixel checks; manual UI acceptance remains pending.

Prior Dev68 CI remains bound to source8d98424: Windows37293880927 and
Mac37293880981 PASS. Windows Dev68 native acceptance NOT RUN, and these old
artifacts do not include the latest loupe fixes. New Windows package is pending.
Dev60 scoped mode/pixel/migration evidence remains artifact-specific.
Installer payloads still pin separately accepted Dev52/Dev56. Demo remains OFF.
Native activation waits for marketplace SDK access. No release/merge.

Previously accepted Mac: **0.9.4 Dev52**, source740dbaa89d7b9738123de6ede5e968a5af1c2375,
Build ID EGFX-01242dd7b423e5aa1384abae. Native License/About/Close and scoped
pixel/key/save/reopen evidence recorded below; marketplace activation remains
blocked on author SDK access. Windows: **0.9.4 Dev56**, source
cbb7468b36e9b94b4c575b3de1c27863ebe8caf2, Build ID
EGFX-376f97bdd493cd2fbce61edd, AEX SHA256
669436b62a06baef0eba895e8e72b50f89e448ff396c24c43e16a2f93c15bd6c.
Exact Windows build/24 CTest/86 Rust/Clippy/PiPL PASS at CI37189319759;
all ten delivered manual checklist items USER-REPORTED PASS. Actual Windows
OS/AE version and loaded identity were not captured. Broader MFR/aerender/
controlled-host/migration release checks are not inferred from that report.
Real speed measurement was explicitly excluded by the user; optimized shared
CPU renderer/quality remain unchanged. See windows-update-parity-2026-10-04.md.

U8 native app/exe implementation now exists. Mac exact Dev52 disposable-root
frontend fixtures and five native coordinator CTest targets PASS; final Mac app
built/signed/strict signature verification PASS from60fe49c. Visual capture
returned a computer-use timeout; real administrator install/Restore NOT RUN.
Windows native disposable-root fixtures and unchanged Dev56 fetch PASS at
CI37201385419; executable compilation with warnings as errors PASS, but linker
manifest conflict stopped packaging. Fix: request the same requireAdministrator
level in linker-generated and explicit manifests; final inspection/packaging
pending. Original failures remain FAIL. See native-installer-plan.md and newest
U8 checkpoints below. Current delivery is a validation candidate, not a release.

### Retained Mac Dev44 acceptance scope

The following records remain bound to their original artifact and are not
transferred to Dev52 or Windows without the separately stated evidence.

Dev44 Mac AE25.6x101 acceptance PASS: ordinary binding/addition Undo/Redo;
delete/Undo/Redo; corresponding frame pixels exact; saved initialized and
canceled-initialization states reopen correctly without idle reinstallation;
original4Grid/3Radius keys and values1,3,6 retained. Addition still has two Undo
actions (initialization, then effect addition); no single-action claim.
MFR requested ON75/OFF: both60-frame aerender runs complete and all decoded
frames match exactly. Actual concurrent callbacks are not instrumented; no speed
claim. One default camera with a Y20 parent and one 3D layer passes grid on/off
and exact restored-frame checks. [Dev44 record](update-094-dev44-lifecycle-2026-10-04.json).

Show Grid is static/defaultoff. User removed the never-in-output requirement;
enabling it renders grid pixels in playback/export/downstream effects. Earlier
Dev20 scoped defaultoff/switch/export/reopen/key/depth/plane/quality/Wave and
unselected playback checks retain their exact artifact scope:
[Dev20 Show Grid](show-grid-mac-dev20-2026-10-04.json).
The initial Dev20 ordinary Undo failure is preserved in
[its lifecycle record](update-094-dev20-lifecycle-2026-10-04.json), not relabeled.
Dev44 fixes the observed redo loss without changing the rendering ABI.

Legacy Tension Radius animation immediately influences old deformation under
the user's choice. Original streams/keys are retained; initial appearance may
change. Live scalar feedback is USER-REPORTED PASS. Earlier native density and
reverse-gesture evidence retains its exact source scope; see
[feature backlog](feature-backlog.md) and dated checkpoints below.

Expanded Wave native UI inspection PASS on Dev44 after user expansion; five
controls and common popup width verified. Legacy spacing/range and animated density have now passed the scoped Mac
checks linked below. Remaining current work: finish native installer packaging and real administrator
Install/Restore acceptance for both targets. Current accepted plugin and native
fixture scope are stated above; a full release remains incomplete.

## Published macOS release

[FSTR Stretch v0.9.3-perf.1](https://github.com/ios3kov/ElasticGridFX/releases/tag/v0.9.3-perf.1)
is the published ordinary release. Package version is 0.9.3-perf.1; the unchanged
plugin About version is 0.9.3 Develop Build 2.

- Shipping source: `f611312bd7b76ebe5bc5f2bd8b48b44f50c0c761`.
- Build ID: `EGFX-6147dc406abc596e7f2d1b60`.
- Public ZIP SHA-256: `a563f8e14961e19ee0740d5eb063c89e4bbec830ac1053d23fa09de02f2a6d14`.
- Verified host scope: AE25.6x101, macOS26.6.2, Apple Silicon.
- Clean-environment installation and animated legacy Columns/Rows acceptance
  remain USER-REPORTED, with environment details unspecified.
- Adopted macOS/project rules baseline: 6.0.0 / `bb8b769404ddd5b97462812a4e6b430e8bfefe13`.
  Developer ID/notarization are not prerequisites under that baseline.

Read [the user guide](USER_GUIDE.md), [release record](release-0.9.3-perf.1.md),
[publication/baseline evidence](release-publication-rules6-2026-10-02.json) and
[public install/project check](public-install-project-check-2026-10-02.json).

## Performance and remaining limits

The quality-preserving CPU optimization is complete and accepted by the user.
Five matched 1080p/60-frame/Full/Final/32-bpc PNG exports with MFR requested OFF
reduced median total time from 65.870721 to 48.766815 seconds (25.97% less).
This includes startup and encoding. Paired measured/warmup pixels match exactly.
RAM Preview scope and latency limits are documented separately; no more timing
series is planned for this accepted macOS cycle.

The earlier MFR control abort did not recur in six bounded retries (360 frames).
Issue21 is closed as not reproduced by user decision. Cause remains UNKNOWN,
fix NONE; this does not certify general MFR reliability. Reopen for a new crash
with exact artifact/project identity and pre-termination evidence.
Broad HDR/OCIO, per-character 3D and untested host/platform compatibility remain
outside the verified scope. Persistent grid display when unselected is a
[future feature](persistent-viewer-grid.md), not an unfinished release gate.
The [future update backlog](feature-backlog.md) also records the requested
Grid Positions-only reset button: reset at the current time while preserving
keys at other times; do not clear the animation.
The next-update plan also requires clear English UI labels, live viewport
feedback, automatic safe spacing and a collapsible Wave Animation section.
The editable influence model and legacy-spacing compatibility need design
before implementation; no new controls or runtime behavior are shipped.

Read the [performance/release retrospective](retrospective-0.9.3-perf.1.md),
[quality contract](performance-quality-contract.md) and
[MFR closure record](mfr-closure-retry-2026-10-02.json).
Reusable findings were contributed in
[central rules PR16](https://github.com/ios3kov/AE-Development-Rules/pull/16);
the [local know-how record](AE_ENGINEERING_KNOWHOW.md) retains its original
review-stage provenance rather than rewriting history.

## Windows continuation

As checked on 2026-10-03, Windows work is isolated on
[`feat/windows-x64-aex`](https://github.com/ios3kov/ElasticGridFX/tree/feat/windows-x64-aex),
checkpoint `003607dd4795596975d2f7a7e4b192cf653dfa34`. The branch consciously
adopts rules6.2.0 for Windows; it does not silently change this macOS baseline.
Its exact-head Windows build/static and macOS source gates passed. A Windows x64
AEX artifact exists, but Windows AE runtime is NOT RUN/BLOCKED. Load/identity,
guide interaction, Undo/Redo, save/reopen, first-application matrix, Render Queue,
MFR/aerender and Windows performance still require the selected Windows+AE host.
Neither Windows GPU nor new product features are part of this CPU port.

These links are pinned to that reviewed checkpoint so this summary cannot
silently inherit later branch results:
[Windows port status](https://github.com/ios3kov/ElasticGridFX/blob/003607dd4795596975d2f7a7e4b192cf653dfa34/docs/windows-port-status.md),
[validation procedure](https://github.com/ios3kov/ElasticGridFX/blob/003607dd4795596975d2f7a7e4b192cf653dfa34/docs/windows-ae-validation.md).
Next engineering step is Windows AE validation of the exact candidate;
compilation is not runtime acceptance. No Windows merge/release is recorded here.

## Documentation maintenance

This cleanup is documentation-only under the adopted rules6.0.0 baseline:
Process/Core, Engineering §§24/25 and Workflow apply. No native code, build,
installation, product contract or historical evidence verdict changes.

The documentation index groups current entry points, retained evidence and old
checkpoints. The full old status remains at its original address, clearly marked
historical; the duplicate unversioned verification file becomes a small pointer
to the preserved v0.6 report. Root README directs readers here. Source and binary
identities above refer to the existing release, not a rebuilt documentation HEAD.
Tracked documentation participates in new Build IDs: any future validation
artifact must be built from its own exact final source checkpoint.

Maintenance verification PASS: 296 local Markdown paths/anchors, complete
100-file documentation index, unchanged source/build/evidence bytes, unchanged
historical status body and preserved full v0.6 report. Git whitespace and root
SHA256 manifest are checked before commit. The static scanner exited1 for the
already-reviewed false positive in the mocked local artifact-manifest unit test
at tests/test_target_ae_acceptance.py:54; this is not an auth/network endpoint.
CI on this documentation commit is reported separately from the existing
release's tests. See [documentation index](README.md).

## Next update development — 2026-10-03

The user authorized implementing the feature plan under frozen rules8.0.0 /
132b7cd32873ba7328e3128ffbb33e1929b74d45. Branch feat/next-update preserves the
backlog and locally integrates the existing Windows port; main and the original
Windows branch remain unchanged. The latest explicit Show Grid decision is to
keep the viewer-only overlay visible during RAM Preview playback when enabled.
Read the [active task/check mapping](feature-backlog.md#active-update-task-and-check-mapping--rules800).

Next block: repair Windows roundtrip completion evidence, then current-time reset
and UI organization. Live influence, automatic spacing and persistent/playback
overlay need design/feasibility evidence before their dependent implementation.
No new plugin has been built, installed or accepted; Windows AE runtime is NOT RUN.

### U1 — Windows evidence repair

Implemented per-run nonce/Build ID completion after fixture assertions and cleanup,
atomic result publication, decoded PNG validation and loaded-module recheck.
Invalid/stale/missing completion and corrupt frames fail closed. Fifteen focused
Python tests and thirteen actual-JSX mock control-flow cases PASS; these are not
Windows AE runtime tests. Runtime remains NOT RUN. The validation guide now records
the audited Visual C++ x64 runtime dependency. Next: U2/U3/U4 native UI block.

### U2/U3/U4 — shared native UI implementation

Applies to macOS Apple Silicon and Windows x64 together. Added supervised Reset
Grid Positions → Reset Now, writing only the current GridState value through AE's
parameter transaction; no key enumeration/deletion, timing or other parameter
writes. Grid Positions and the new Wave Animation group start collapsed. The
approved labels Affected Lines and Follow Strength replace the two old labels;
remaining label simplification is pending. Stored IDs/types, grid wire format,
wave ranges/defaults/ordinals and render math are retained.

The added UI controls exposed a fixed-index dependency in native plane binding.
Binding now resolves unique hidden stream names and revalidates names/types before
access, preserving fail-closed schema checks without relying on old positions.
Test fixtures use hidden names and support both flat and grouped wave controls.

Validation: 64 Rust tests PASS, including reset topology and hidden-stream schema
regressions; 13 actual-JSX control-flow mock cases plus flat/grouped lookup PASS;
all 16 JSX fixtures parse; 8 Windows runner tests PASS. The earlier release compile
passed with a local rust-objcopy LLVM lookup warning; final clean-source build and
package are pending. These results do not prove AE panel layout, current-time key
insertion/replacement, Undo/Redo, old-project migration or Windows runtime. Those
checks remain NOT RUN on the new candidate. No new plugin installed or released.

API basis: SDK25.6 AE_Effect.h USER_CHANGED_PARAM/change flags, after-effects0.4.0
parameter/group wrappers and [Adobe parameter supervision](https://ae-plugins.docsforadobe.dev/effect-details/parameter-supervision/).
Next: clean candidate build and host validation, remaining labels, then the live
influence/automatic-spacing design. Persistent/playback overlay remains a separate
feasibility item; safe installers and Windows host closure remain required.

### New-candidate host checkpoint — investigation in progress

Built clean source45b33ba on macOS; candidate EGFX-f16f0eb6f275980990b1f645.
Bundle entrypoints/PiPL/signature and package/source hashes PASS; local Clippy PASS.
The permission-preserving installer helper requires the canonical internal archive
root ElasticGrid.plugin, whereas host verification binds the archive to the actual
installed name FSTR Stretch.plugin. Separate verified wrappers contain identical
native payloads; the first archive-name refusal occurred before replacement and
is retained. The canonical payload was installed atomically with the old plugin
retained outside Adobe active directories. No rollback requested.

AE25.6x101 smoke on this candidate FAIL at frame_identity; pixels NOT RUN, acceptance
not claimed. A disposable owned scene reproduced a black viewer. Investigation
exposed input parameter0 being included in the new hidden-stream discovery scan;
that scan now starts with effect parameter1. SDK25.6 headers allow input index0,
so this change is a conservative exclusion of an irrelevant input, not a claim
that the public SDK universally forbids0. The revised candidate still needs a
build and host retry before this failure can be marked fixed. The temporary scene
was closed without saving, under the user's explicit AE permission.

The Mac and Windows native source workflows now include feat/next-update, ensuring
both systems validate this shared branch when pushed. Windows runtime remains
NOT RUN. Evidence is retained in local outputs/next-update-45b33ba-mac; older
release/runtime PASS records are not transferred to this candidate.

### Shared UI corrections — 2026-10-03, in progress

The user replaced collapsed Grid Positions with a single title row without a
disclosure arrow, with Reset on the right. A topic-only arbitrary UI and inline
Drawbot button are being implemented; native layout/hit testing is not yet
accepted. The current-time reset scope and preserved animation requirement remain.
Deformation Plane, Falloff, Edge Behavior and Render Quality now declare
CANNOT_TIME_VARY in shared parameter setup. IDs, types, option mappings and
defaults are unchanged. New-host stopwatch/animation refusal and compatibility
with previously animated project values require fresh Mac/Windows checks.

Clean candidate4f653fd / EGFX-3f04fc41b9b8054153c0cb7d loaded identity PASS on
AE25.6x101. Seven direct-effect frames and five independent pixel comparisons
PASS, including identity, deformation changes and wave time changes. Full smoke
remains FAIL at frame_chain_before_corner; adjustment-layer chain and UI reset
are not accepted. Evidence: outputs/next-update-4f653fd-mac (local, not published).
These results do not cover the subsequent UI/static-choice edits.

Shared UI/static-choice source checkpoint: 65 Rust tests PASS, strict Clippy PASS
(with the existing documented drop-non-drop/question-mark allowances), diff
whitespace check PASS. These are local Mac source checks, not Windows compilation
or AE panel/migration acceptance.

### Native panel/static choices — 2026-10-03

Clean Mac source54c215b, build EGFX-bd6fc793fc4111918cb06880 installed with
verified archive/signature and previous-candidate backup outside Adobe.
AE25.6x101 native fixture assertions PASS: all four static choices report
canVaryOverTime=false, reject setValueAtTime, retain zero keys and the same
value; Grid Positions still permits animation. Screenshot confirms no
stopwatches on these four choices, no Grid Positions disclosure, and inline
Reset. Screenshot also exposed duplicate custom/native title text; removed
custom caption drawing. This subsequent source fix requires a new panel check.
Old animated-choice project compatibility, actual Reset key/Undo semantics
and Windows host behavior remain NOT RUN. Local evidence retained in
outputs/next-update-ui-mac; no release claim.

### Inline Reset alignment — user correction, 2026-10-03

Installed candidate3877a6c / EGFX-05609e48d846d45beb491845 visually confirms
removal of duplicated Grid Positions caption and retention of its stopwatch.
User requires Reset to align with the native value column and match Fit Layer
size/shape. The next source uses PF_EffectWindowInfo title offset, a 130-unit
pill button matching the observed native 260-pixel button at Retina scale,
and host ButtonFill/ButtonText colors rather than fixed grays. Fresh host layout
check pending. Also corrected mutation routing: DO_CLICK only arms; DRAG release
inside the button writes Grid Positions, as required by SDK change-flag validity.

User explicitly chose local-only work: no push or draft PR for feat/next-update.
Windows MSVC build/CI is NOT RUN here; no MSVC/Windows host is available locally.
Independent shared code and local verification continue.

### Native alignment rejection and automatic spacing checkpoint — 2026-10-03

Candidate fc81a6d does not display Reset in the Effect Controls title row.
The assumption that PF_EffectWindowInfo.horiz_offset describes the native value
column is not accepted as proven; next diagnostic build records the public
event rectangles once under the existing test-only render-diagnostics feature.
No diagnostic observation is included in default builds. Scene creation switched
to Four Corners before idle and displayed BadCallbackParameter once; after
dismissal the image rendered. Cause is not isolated and remains a separate FAIL.

U6 source adds a hidden AutomaticSpacing flag after all existing parameter IDs:
new effects default to automatic spacing; old projects missing this new stream
use false via USE_VALUE_FOR_OLD_PROJECTS and preserve legacy spacing values/keys.
Automatic mode requests zero, leaving the existing monotonic safety floor and
density limit in core. Local shared Rust tests: 67 PASS. Native new/old project
parameter defaults and Windows runtime remain NOT RUN.

Diagnostic candidate58737cb records title/current frame (17,153)-(665,170)
and horiz_offset=94617420, outside the title rectangle. The latter is unusable
for this AE25.6 arbitrary topic. Reset now follows the row center plus the
observed native 18-unit value inset, uses 130x16 logical units and pill corners.
This is a measured layout policy, not a claimed SDK value-column API. Native
comparison and panel-width coverage are still pending.

Native ordinary candidate31e1ada shows Reset with matching 130x16 logical
size, pill corners and theme colors next to enabled Fit Layer. Visual comparison
found a 2-logical-unit horizontal discrepancy; inset corrected from18 to16.
Grid Positions retains its native stopwatch and no disclosure. Min Line Spacing
is hidden; Wave Animation remains collapsed. This is Mac AE25.6 visual coverage;
Windows UI and additional panel widths remain NOT RUN.

### Installed Reset correction / new spacing host check — 2026-10-03

Candidate9b5c26b is installed. Independent loaded-identity verification PASS:
outputs/next-update-reset-aligned-mac/loaded-identity/
EGFX-check-ab108c38e46b43ee83e4e1de938ce378.zip.
Fresh owned-scene API evidence native-new-spacing.json on AE25.6x101:
AutomaticSpacing=1; canVaryOverTime=false; legacy MinSpacing=0.5 retained;
Grid Positions canVaryOverTime=true. Shared suite67 PASS; strict Clippy PASS
at31e1ada; focused final layout test1 PASS after the inset-only correction.

Final visual inspection interrupted because macOS locked before the panel was
shown. Previous31e1ada visual comparison confirms dimensions, pill shape and
colors; final9b5c26b horizontal correction is not yet visually accepted.
No claim of Windows UI verification, old-project spacing migration, Reset
keyframe/Undo runtime proof or complete update readiness. User local-only scope
continues; no push, PR, release or changes to main.

### Reset final layout and event review — 2026-10-03

Mac unlocked; installed9b5c26b visually matches Fit Layer horizontal bounds,
130x16 dimensions, rounded shape and theme colors. Grid Positions remains a
single row with its native stopwatch. New visual comparison observed in Cua.
Coordinate click via Cua still returns AXError.notImplemented. A current source
review against SDK25.6 AE_EffectUI.h:545 avoids effect_win reads during DRAG;
DO_CLICK saves the hit rectangle in the four continuation integers. DRAG uses
that rectangle and the gesture marker, clears it on release and commits only
inside bounds. New regression verifies foreign/nonfinite state rejection and
inside/outside release geometry. Native button/keys/Undo proof still pending.

U3 remaining labels use short wording: Follow Shape (Smooth/Soft/Even/
Smooth Legacy), Smooth Stretch, Smooth Width. Enum IDs, four ordinal slots,
values and formula mapping are unchanged. Standalone host fixtures resolve
legacy/new label aliases and flat/grouped parameters, with bounded nesting.
Compatibility of existing name-based user expressions remains a native check,
not inferred from unchanged IDs.

Full Python suite baseline265: three stale approved-UI contract failures and
three sandbox process/path permission errors; latter require scoped execution,
not weakening guards. Contract expectations updated to the approved static
choices, topic-only grid, group controls and appended compatibility flag.

Shared checkpoint: 68 Rust tests PASS; strict Clippy PASS; Python suite265 PASS
with scoped native-process test access; all16 standalone JavaScript test files
PASS after removing stale hard-coded hidden-stream indices from their models.
The perf preparation fixture itself also retained one numeric hidden-stream
lookup; corrected to the canonical unique name, preserving readiness refusal.
These results are code/mock evidence, not Windows or AE runtime certification.

Fresh Mac candidate3032232: loaded identity PASS, build
EGFX-eef15a14adf9e796c0cb4542, binary SHA256
d8566c46642a77646447f7d273a17951ec6f1553cc015a1d22d34884a20a33f2.
New owned scene __EGFX_UPDATE_3032232 confirms current label values/static flags
and visual Reset/Fit Layer alignment. Evidence outside repository:
outputs/next-update-labels-reset-mac/native-labels.json and loaded-identity/.
Native Reset/Undo, legacy project migration, new render regression and Windows
build/runtime remain pending. No publication or release; local branch only.

### Native Reset rejection / equal popup widths — 2026-10-03

Owned3032232 scene: a real composition drag changes60162 pixels. The authorized
fixed Reset input is delivered, but its frame is byte-identical to the deformed
frame, rather than the neutral baseline. Native Reset acceptance FAIL, not PASS;
Undo/key preservation remain pending. Isolate event dispatch/hit coordinates
with bounded test-only geometry observations before another candidate.

User additionally requires one common popup width, with every option fully
readable, across Deformation Plane, Follow Shape, Wave Axis, Edge Behavior and
Render Quality. SDK25.6 AE_Effect.h:2416 says ui_width/ui_height are ignored
without PF_PUI_CONTROL (expanded custom area); it does not expose a standard
popup-width setter. First test shorter readable options in native controls:
Old Smooth; Preview/Final; Both/Columns/Rows, retaining numeric values/formulas.
This should leave room for every option at the standard minimum width, but
actual common width still needs host observation. A custom-row/menu fallback
must retain IDs, selection, keyboard behavior and host ownership. No padding
strings, dummy options or unsupported reserved fields added.

### Reset/key/Undo proof and native popup widths — 2026-10-03

Diagnostic8a2e30c loaded identity PASS, EGFX-a700873a29770d0ec94313c6.
The earlier unchanged frame was a harness-coordinate failure: Cua captures
window-local pixels, while CGEvent needs global logical coordinates. Measured
AE window origin(1,34), Retina2x. Corrected Reset point(425,300) reaches native
CLICK then DRAG/release inside the recorded rectangle. No Reset logic correction
was needed to obtain the following native results; previous probe does not prove
a product failure. Do not transfer this PASS to the older3032232 bytes.

Owned319x241,8-bpc Final scene in AE25.6x101: static drag changes60162 pixels;
Reset matches the neutral baseline exactly; Undo matches the deformed frame.
Animated test starts with two distinct keys at0 and1 seconds. Reset at0.5 adds
only that key, preserves both endpoint frames exactly and renders neutral at0.5.
Undo restores the two original keys and all three captured frames. Redo restores
the three-key reset state and all three frames. Eight independent comparisons
PASS. Evidence: outputs/next-update-popup-event-mac/reset-static-result.json,
reset-animation-result.json,12 animation PNGs and loaded-identity/.
These are native UI/key/pixel checks, not full render/MFR/migration certification.

Mac visible popups now share130 logical units, aligned with Reset/Fit Layer:
Four Corners, Old Smooth, Mirror and Preview fully fit. Wave Axis remains inside
the collapsed group; its short captions are implemented, but expanded visual
width and Windows widths still need observation. A test attempt to select the
group via JSX was rejected by AE because the group is hidden to selection; no
product error or failed render is inferred from that probe.

Source cleanup removes the incoming send_drag heuristic: SDK declares it an
output request, and native events have separate CLICK/DRAG tags. The default
candidate must be built and verified independently; diagnostic observations are
compiled out without render-diagnostics. U5/U7/U8/U9 and legacy migration remain
open; local-only boundary unchanged.


### Default Reset proof and narrow-panel defect — 2026-10-03

Ordinary fb9ae4f / EGFX-332d09780aa8f63db066d439 loaded identity PASS.
AE25.6x101, 319x241, 8-bpc Final: saved three-key fixture reopened with exact
matching pixels. Reset at0.2 adds only that key, preserves0/0.5/1 frames, renders
neutral at0.2; Undo restores original keys and all four before frames. All eight
checks PASS; saved fixture SHA unchanged. Evidence outside repo:
outputs/next-update-ui-default-mac/reset-default-result.json and loaded-identity/.
Diagnostic observations are compiled out in this ordinary native-plane build.
Four visible popups align and fit on Mac; expanded Wave Axis/Windows pending.

User screenshots15:16:49/15:16:59 reproduce Reset disappearing when the ECW
shrinks, while native Fit Layer remains visible. Root cause is the arbitrary
300-logical-unit row-width rejection in grid_row::button. Remove that rejection
and size to the actual available value column, retaining minimum readable
button width60, maximum130, height16, theme/pill and hit geometry. Regression
covers220/240/260/299/300/400/648 rows and retained continuation bounds. Native
resize verification for these new bytes is pending; shared Rust suite69 PASS; do not transfer fb9ae4f
functional PASS or previous wide-panel appearance to the changed candidate.


ac45b60 / EGFX-3aa42f290e3876afb6ca4992 installed, bundle and loaded identity
PASS;69 Rust tests and strict Clippy PASS. Cua panel-boundary drag refuses with
AXError.notImplemented. A native Window > Workspace > Small Screen switch
provides an independent resize check without extending CGEvent permissions:
Reset remains visible below the removed300-unit cutoff. Narrow comparison
finds one logical unit less width than Fit Layer (two Retina pixels); right
gutter corrected from6 to5, retaining maximum130. New candidate/check pending.
No extra saved workspace or project was created; restore Default after checking.


### Narrow-panel Reset closure — 2026-10-03

6314811 / EGFX-c42b0f5c0b8c8a9017d664b4 installed and loaded identity PASS,
binary SHA256 dca36441b0cb4f21a567356ec9ce0cbfc32b06e155c3b053e9ff427f7e9b1d4a.
69 Rust tests PASS. Native Cua observation in AE25.6x101 confirms Default
wide260-pixel and Small Screen narrow242-pixel Reset match Fit Layer exactly
in horizontal bounds, height and pill shape. Reset remains visible below the
old300-logical-unit rejection; switching back to Default retains visibility.
Original Default workspace restored without saving a workspace/project.
Evidence: outputs/next-update-reset-resize-final-mac/resize-result.json and
loaded-identity/. Previous ac45b60 strict Clippy PASS; final change is only the
right gutter and corresponding test bound. Windows and continuous native mouse
resize still NOT RUN; finite available width is required for any readable
button. No new Reset pixel/keys/MFR claim for this geometry-only candidate.
U3 narrow-panel disappearance is resolved on the verified Mac layouts. U5/U7/
U8/U9 plus legacy migration/integration remain open; update is not complete.


### Live influence investigation — 2026-10-03

U5 requires editable move intent, not additional UPDATE_NOW calls over baked
positions. [Design investigation](live-influence-design.md) records the existing
source route, a bounded candidate state and numerical/serialization gates before
integration. Real C++ FFI regression proves every density1..50 at each retained
topology1..50 uses the same maximum50 internal reference catalog, comparing float
bits and unchanged serialized state across2500 cases. Shared suite70 PASS.
This is a design prerequisite, not an implemented live-influence feature.

Human legacy decision pending: preserve exact old appearance and apply live
influence to new moves, or require old baked deformations to respond too. Missing
gesture history must not be fabricated by inverse fitting or rewriting old keys.
No v4 data is encoded, no render model changed, no platform runtime PASS claimed.
Installed6314811 remains the Reset-resize candidate; subsequent edits in this
checkpoint are documentation plus test-only design coverage. Local-only scope
continues. Show Grid public overlay feasibility, installers, Windows and legacy
integration are still open, so the full requested update remains incomplete.


### Density gesture native checkpoint — db97762

Installed0.9.4 Dev1 / EGFX-883ae0b0eae4536bb555bc3d: actual drag after density7/6
changes only current-time deformation, preserves the three original-key frames,
and inserts .2 key. Undo and Redo restore exact key times and pixels. Captures
that assign comp.time initially failed Redo; this historical harness result is
retained, precise cause not claimed. Read-only-time captures pass. A fresh saved
copy reopens with exact pixels, counts and four keys. Evidence:
outputs/update-094-registration-mac/read-only-undo-redo-result.json and
gesture-roundtrip-result.json. This is bounded Mac acceptance; Windows NOT RUN.
Diagnostics-feature Rust78 PASS. U5 live influence, U7 Show Grid, U8 installers,
U9 Windows and full integration remain open; whole update is not complete.


### Public preview callback experiment — source checkpoint

Opt-in preview-overlay-probe registers SDK25.6 PF_CustomEFlag_PREVIEW. Logs only
callback/window/time in a private exclusive bounded file; unknown/null contexts
never enter ordinary UI conversion or drawing. Production default has no probe.
Shared probe Rust76 PASS; identity23 and host-contract14 PASS; strict Clippy
PASS with two style lints confined to the reviewed SDK-generated entry module.
Default Rust74 and full Python267 PASS. Source checks/build/host observation
are separate gates. Probe version0.9.4 Dev3, no Show Grid feasibility PASS yet.
The user confirms old baked deformation must respond to Affected Lines too;
new-edits-only was rejected. U5 numerical/compatibility design remains open.


### PREVIEW experiment closure / ordinary candidate

Actual AE25.6 selected/unselected stopped/playback experiment logs1711 records
and zero PF_Window_PREVIEW callbacks. After deselection no comp DRAW, seven
Layer DRAW from the separate Layer view. Tested flag is not a demonstrated Show
Grid route; universal impossibility is not claimed. Evidence:
outputs/update-094-preview-probe-mac/feasibility-result.json. Show Grid still OPEN.
Ordinary0.9.4 Dev1 from35a9f3c installed afterward (EGFX-2b54838b4bf29f6a45243d07):
loaded identity PASS and exact current-time saved-gesture pixels PASS. Old released
plugin was not restored; original test copies remain in external backups.

U8: shared read-only installer decision contract implemented with both-platform
positive/negative checks. Native pkg/exe frontends, atomic executor/recovery and
actual installation acceptance remain NOT RUN. U5 old-project live response
needs a compatibility decision about the old Tension Radius animation; original
picture and Grid Positions keys must remain unchanged before edits. Local-only
boundary continues; full requested update remains incomplete.

### U8 native Mac exchange primitive — resumed 2026-10-03

Internal installer/macos/AtomicExchange performs same-volume directory exchange
using public renameatx_np with RENAME_SWAP and RENAME_NOFOLLOW_ANY. Owned parent
FDs, single-component names and expected device/inode checks reject stale,
linked, unsafe or unsupported entries. An exchange followed by verification or
sync failure is explicitly ExchangedNeedsRecovery, never reported unchanged.
The caller still must provide fixed-root validation, payload authentication,
transaction lock, durable journal and recovery. This primitive is not a finished
installer or a privileged helper; no actual Adobe installation was changed.

PreparedJournal writes an exclusive immutable, versioned record of both tree
identities before exchange, syncs file/parent and requests Darwin F_FULLFSYNC.
Failure after record creation retains evidence and refuses publication. Its
bounded reader rejects partial/extra/invalid records, unsafe permissions and
links; recovery classifies unchanged/swapped identities or refuses unknown.
Neither inode classification nor this journal authenticates payload contents.

Native Release CMake test on disposable /private/tmp fixtures PASS: exchange,
restore, file inode/mode/owner/xattr preservation and negative guards for stale
identity, traversal, aliases, missing/non-directory entries, links and unsafe
parents; exclusive journal creation/readback, unchanged/swapped/unknown identity
classification and partial-record rejection. Two fresh child processes receive
SIGKILL after durable prepare and after exchange; readback identifies both states
and the swapped fixture restores successfully. This is process-death evidence,
not power-loss or full privileged-installer recovery acceptance.
Evidence: outputs/update-094-installer-cmake and
outputs/update-094-installer-resume-test.txt. Cross-volume, sync-failure injection,
full coordinator crash recovery, privilege boundaries and Windows runtime remain NOT RUN.
Next U8 step: coordinator combining journal, locks, verified payload snapshots
and identity-safe recovery; process interruption/fault injection before any
end-user installer frontend or real installation test. Everything remains local.
Shared installer contract tests:4 PASS, including26 subtests. Static scanner
completed with exit1 for the previously reviewed auth/rate-limit heuristic at
tests/test_target_ae_acceptance.py:54, a local unit fixture, not an endpoint.
No new installer finding; scanner does not certify readiness. Whitespace and
repository SHA256 manifest are checked before this local checkpoint commit.

### U8 replacement coordinator — local development

ExchangeCoordinator connects destination-scoped nonblocking flock, immutable
prepared journal, identity checks, replacement and explicit Inspect/Restore.
The trusted frontend verifier must recollect fixed-root/scan/host observations
and authenticate/compare full old/new payload snapshots under lock. It runs
before preparing, again before exchange and after exchange. Failure before
exchange never installs; failure afterward retains both trees/journal and reports
NeedsRecovery. Recovery also requires the frontend's authentic original expected
identities to match the journal; neither the journal nor an inode alone permits
overwrite. Existing records are never silently replaced or removed.

Native Release build with -Wall/-Wextra/-Werror and assertions enabled PASS.
Two installer CTest targets PASS on fresh /private/tmp fixtures: successful
replace/inspect/restore; idempotent restored inspection; journal reuse refusal;
verifier failures before/after journal and after swap; changed same-inode payload
preserved on Restore refusal; mismatched expected journal, symlink lock and
actual lock contention rejected. Two coordinator child processes killed with
SIGKILL at prepared/installed checkpoints release their locks; subsequent
Inspect/Restore identifies the correct state and preserves/restores the old tree.
Evidence: outputs/update-094-installer-coordinator-test.txt. These mock payload
verifiers prove coordinator ordering/refusal, not authentication of real bundles.

Still incomplete: native fixed-root/payload/process collector, authenticated
durable snapshot metadata, finalization/new-install path, power-loss and syscall
failure cases, elevation/UI, Mac package and Windows implementation/runtime.
No installed Adobe plugin, project, system permission or remote branch changed.
Next: implement full-payload collection before connecting any real installer UI.

### U8 native payload snapshot — local development

PayloadSnapshot implements read-only snapshot-v1 using system CommonCrypto
SHA256 over sorted relative names, file type/mode/owner/group/flags, xattr names
and values and regular-file bytes. Root device/inode binds collection separately.
Traversal uses directory FDs and nofollow opens, rechecks stat identity/change
timestamps and returns no partial output on error. Bounds:4096 entries, depth32,
256MiB/file,512MiB total file bytes,64KiB attribute names/object,1MiB/attribute,
32MiB total attributes. Links, multi-link files, special files, cross-volume
children and any extended ACL are refused; unsupported metadata is not dropped.
This snapshot detects changes; it does not authenticate a publisher or validate
fixed installation roots. The frontend must preserve trusted original snapshot
metadata durably before mutation; reconstructing expectations from current files
after a crash is forbidden.

Initial native test failed with errno2 because acl_get_fd_np cannot distinguish
missing ACL metadata through its NULL result alone. Replaced that assumption with
fstatx_np plus explicit filesec_query_property(FILESEC_ACL); unsupported ACLs
remain refused. SDK headers and Apple's Libc implementation were inspected:
[ACL retrieval](https://github.com/apple-oss-distributions/Libc/blob/main/posix1e/acl_file.c).
No Apple source was copied into the project; only public system APIs are called.

Release native snapshot test PASS: identical bytes/metadata deterministic;
same-size content, mode and xattr changes affect SHA; renaming the root preserves
digest and inode; links/FIFO/ACL rejected; excessive file size/depth rejected;
failed collection preserves caller output. Coordinator fixture verification now
uses these snapshots rather than comparing a mock payload string. Pre-install
snapshots are retained in the parent before SIGKILL tests; neither restored nor
installed expectations are inferred from post-crash files. All three installer
CTest targets PASS: outputs/update-094-installer-snapshot-tests.txt.

Read-only collection of the actual installed FSTR Stretch.plugin also PASS:
11 entries,1063171 file bytes, snapshot-v1 SHA256
3fe8d933de3424c6953333b70721c5d73ffeb9a0beb47a60fb86cbc4f02b2358.
Evidence: outputs/update-094-installed-payload-snapshot.json; standalone diagnostic
outputs/payload_inspect.cpp. This digest differs in schema from artifact ZIP/hash
or Build ID; it is not a new AE-loaded identity or installation acceptance.
No plugin bytes, permissions or project were changed. Fixed-root/process scan,
package trust/signature/identity collector, durable authenticated recovery metadata,
new-install/finalization, native frontend and Windows remain incomplete.

### U5 immediate live field — source block

Pure compact Smoothstep displacement evaluation is connected to the shared
viewer/normal-render/SmartRender grid preparation. It immediately reads the
existing TensionRadius stream, including old animated values, without touching
saved Grid Positions, scalar keys or wire3. Increasing radius distributes local
deformation and may soften its magnitude; radius<=1 retains the baked axis,
neutral/reset retains exact uniform bits. Visible guide density is independent.
The explicit new internal live_influence field is host/core coordinated,168 bytes
on64-bit with offset160. Frozen core parity callers leave it0; updated host sets1
for normal and SmartRender paths. Older binaries are never mixed with this ABI.

Interactive editing now uses a bounded scalar search against exactly the same
evaluated field, Wave and easing at the fractional source reference. Every trial
starts from the original stored projected axis; only the final validated result
is published, so saturation retains no hidden movement history. Affected Lines
supervision requests rerender without rewriting any grid/animation value. Dev4
cache identity and visible version derive from one build-script number; optional
diagnostic/probe builds becomeDev5/Dev6, not the earlier2/3 candidates.

Core25/25 CTest PASS on initial source; focused final live-field numeric/pixel
test PASS after adding full8/16/32-bpc sampling comparison. Neutral/monotonicity
at every topology1..50; explicit kernel weights; radius response and immutable
axes; fractional handles with Wave/easing; zero-drag stability, saturation/reverse
and invalid-target refusal tested. Both qualities/all edge modes match byte exact
rendering of the same pre-evaluated axes through the existing sampler, including
extended-range float inputs. This proves sampling consistency, not preservation
of old appearance (the user expressly superseded that requirement).
Rust74 PASS; strict Clippy PASS before final build-number consolidation; final
consolidated Rust74/strict Clippy PASS, identity/host-contract37 PASS (two subtests).
Scanner exit1 remains the reviewed mocked-local-fixture auth heuristic; no new
finding. Source checks recorded separately before package. Earlier compile failed on the
old160-byte ABI assertion; updated both Rust/C++ size and new-offset guards.
Unused legacy drag wrapper is now test-only; no warning suppression added.

Evidence: outputs/update-094-live-core-tests.txt,
update-094-live-numeric-pixel-tests.txt, update-094-live-rust-versioned.txt and
update-094-live-clippy-versioned.txt, update-094-live-rust-consolidated.txt and
update-094-live-host-checks.txt. Native AE slider/old-radius-key/drag/Undo/
save-reopen checks, exact new package/load identity and Windows remain NOT RUN.
Show Grid and affected-range visualization remain open. Full update incomplete;
local-only boundary maintained. Next: clean-source ordinary Dev4 package and
owned AE validation; no installer work before plugin integration.

### U5 Mac Dev4: old animated field acceptance block

Ordinary clean source76015ca8eb2f2affb15b62666c7eb4a0c95fb293 was assembled,
ad-hoc signed, verified and installed with a retained backup outside Adobe.
Build ID EGFX-e9e024a3b24f5ebdd238b474; branded ZIP SHA256
2c6afea59a893e746b78fb543c85320d15a08dfb7f40c9640b78356e825acd0c.
Native sample/LC_UUID, installed signature/payload and exact AE25.6 process PASS.
The effect panel visibly identifies0.9.4 Dev4.

An owned legacy fixture was authored and saved under the confirmed loaded Dev1:
Radius keys0→1,.2→3,1→6 and Grid Positions times0,.2,.5,1. Dev4 opens those same
streams/keys without migration or replacement. New influence changes the .2
frame immediately, as expressly requested. Scripted Radius0/6/3 produces distinct
frames; restoring3 matches the first Dev4 frame byte exact. Radius0 matches the
old baked frame byte exact. Save/close/reopen retains three Radius/four Grid keys,
exact scalar values/times and the exact radius3 frame. Scope:319×241,8-bpc Final,
AE25.6x101/macOS26.6.2/arm64. Scripted edits do not prove mouse slider scrubbing.

Native guide attempts are NOT PROVEN: the first helper refused nonforeground AE;
later bounded deliveries reported success but readback showed unchanged pixels
and key counts. Cua drag itself returned noWindowsAvailable. Preserve failures;
do not infer host acceptance from numerical tests or input delivery. A bounded
opt-in probe now records viewer click/hit/drag status and callback mouse points
(schema2), retaining no host context. It is excluded from the ordinary build. Probe Rust76 and strict Clippy PASS;
initial strict check rejected redundant casts, removed without suppressions.
[Exact candidate and hashed local evidence](live-influence-mac-dev4-2026-10-03.json).
Next: diagnose viewer gesture and finish U5 range drawing, then U7. Installer work
remains paused; Windows integration and full-update acceptance remain incomplete.


### U5 gesture investigation: ordinary and PREVIEW-probe diverge

Probe9e2cea8 /0.9.4 Dev6, Build ID EGFX-e44a68598a0cbead2d664b5f,
loaded identity PASS. Native click correctly hit column2;11 drag callbacks completed
without a reported error. Real319×241 pixels changed; Undo restored baseline and
Redo restored edited pixels byte exact, with three Radius/four Grid keys retained.
Evidence outputs/update-094-live-gesture-probe/gesture-events.csv and
undo-redo-pixels.json. Scope is this probe, not ordinary Dev4.

Returning to the verified ordinary76015ca Dev4 produced unchanged pixels for the
same delivered gesture despite exact loaded identity PASS. Earlier attribution
to only focus/coordinates is therefore not established. Preserve the original
failures and positive probe result separately. The next isolated gesture-probe
build uses the same bounded logger/context guard but ordinary COMP/LAYER/EFFECT
registration without experimental PREVIEW; versionDev7. It is test-only and does
not implement Show Grid or change renderer sampling. Ordinary Dev4 is currently
installed; native guide acceptance remains NOT PROVEN pending investigation.


### U5 isolated registration finding and ordinary correction

Negative-control gesture-probe source d2047c1 /Dev7, Build ID
EGFX-910ae70a4c664a9da8006309, loaded identity PASS. The identical bounded
owned-scene gesture produced no DO_CLICK/DRAG callbacks and no pixel change with
COMP/LAYER/EFFECT registration; cursor/draw/idle callbacks were present. The
preceding PREVIEW-registered Dev6 observed click and11 successful drag callbacks,
changed pixels and exact Undo/Redo. Evidence outputs/update-094-gesture-only-mac/
gesture-events.csv, baseline.png, drag.png and loaded-identity; this is bounded
AE25.6 evidence, not a claim about every AE version or Windows.

Ordinary source now requests the documented PREVIEW custom event flag as well;
the negative-control feature still omits it. All builds validate borrowed context
window codes before wrapper conversion, refusing null/unknown/PREVIEW drawing
contexts and releasing cursor state. No context is retained and no preview pixels
are drawn. New ordinary versionDev8; diagnosticsDev9, PREVIEW-probeDev10 and
negative-controlDev11 prevent identity/version confusion with prior packages.
Source and exact ordinary native acceptance are pending; Show Grid remains OPEN.

Dev8 source verification: default Rust75, PREVIEW-probe strict Clippy and
Python host-contract14 PASS. Current installed temporary negative-control Dev7;
next install the clean-source ordinary Dev8 and validate actual gesture/Undo/Redo.


### U5 ordinary Dev8 native closure of gesture defect

Clean source cd33b0286a53b83837a7ecbbdef0f28708b00f95 was assembled, ad-hoc
signed/verified and installed instead of the negative-control probe. Build ID
EGFX-160e38dd8fe6ee59bcd259e5; exact loaded identity PASS. Ordinary build has
PREVIEW registration/context guards, without the diagnostic writer. Native
column2 drag changed actual pixels; Undo matches initial frame byte exact and
Redo matches edited frame byte exact. The original three Radius keys and four
Grid key times remain. Scripted Radius0/6/3 after this real gesture changes pixels
and restores the edited frame exactly. Saving/reopening retains scalar values,
key counts/times and exact edited frame. Baseline matches the preceding Dev4
old-animation frame. Scope319×241/8-bpc Final/AE25.6x101/macOS26.6.2/arm64.

The owned acceptance script initially refused a wrong copy path, then a cached
File.exists object caused a missing-frame verdict despite eventual PNG creation.
Only the corrected fresh-path/fresh-File run and actual PNG/JSON checks are accepted;
retain original scripts/logs. This was validation harness behavior, not a bypass
of a failed plugin check. [Exact record](live-influence-mac-dev8-2026-10-03.json).

U5 affected-range drawing and real scalar mouse scrubbing remain open. U7 Show
Grid, Windows integration and installers remain incomplete; current priority is
remaining plugin behavior, not installer work. No remote operation was performed.


### U5 range feedback — source integration

The range-marker contract in live-influence-design.md precedes the implementation:
SDK25.6 PF_Context.plugin_state stores only four scalar values inside known owning
callbacks, initialized/cleared with context lifecycle. No host/drawing pointer,
transform or project value is retained. The stored-grid fingerprint discards
stale geometry; a strongest-displacement/neutral-center fallback does not claim
recovery of past gesture intent. This marker never routes editing input.

Blue kernel-range boundaries and a short source-reference edge cap use current
field/Wave/easing and current DRAW transforms, respecting DONT_DRAW and invalid
planes/projection. Endpoints/fractional boundary positions use the existing real
core inversion through a bounded stack-backed query. Native drag refreshes scalar
marker state only after publishing the validated current-time grid edit. Render
and serialized streams are untouched. Show Grid remains a separate open gate.

Initial default Rust78 PASS: stale/malformed scalar state refusal, bounded ranges,
neutral reference, real fractional/eased core queries, invalid axes and all prior
host/core Rust regressions. Versioned source aims for ordinary Dev12 (diagnostics13,
PREVIEW probe14, negative control15). Exact native overlay/export acceptance and
final strict checks are pending; installed accepted ordinary remains Dev8.

Final versioned range-source checks: PREVIEW-probe Rust80 and strict Clippy PASS.
Initial default Rust78 PASS; exact native range/export verification remains next.

### U5 ordinary Dev12 native range checkpoint

Source aa6c203 / EGFX-fedbc0cbe18a3a28d105a305 installed and loaded exactly.
Visible radius1/3 boundaries respond to scripted changes. Export baseline equals
Dev8 saved frame byte exact; restoring3 returns exact baseline. A successful
internal-guide gesture changes real pixels; Undo/Redo and a new saved-copy
roundtrip are exact. Four Grid keys and three original Radius keys remain.
Two earlier delivered gestures produced unchanged frames and are not accepted;
Cua direct drag refused noWindowsAvailable. Existing bounded authorized helper
was used for the successful gesture. Original owned fixture was not overwritten.
The new copy is outputs/update-094-range-mac/range-gesture.aep.

Native scalar mouse scrubbing, range multiple-instance/plane lifecycle, U7,
Windows host validation and installers remain incomplete. No remote action.

Dev12 second-instance bounded check: Cua observed a neutral new instance's
center range at radius1. Removing only that new instance restored exact first
pixels/keys. Its own requested export did not produce a PNG; second-neutral
pixel parity remains NOT RUN despite successful state JSON. Broader multi-view,
plane, deletion-Undo and playback lifecycle acceptance remains open.

U7 SDK route recheck: online ItemViewSuite2 adds guide toggles only for26.0+,
not an additive custom drawing surface. SDK25.6 ItemViewSuite1 and the selected
PF event route still do not establish PV-2/3. Full Show Grid remains a functional
blocker. Windows target std/toolchain and Windows AE are absent locally; existing
Mac/shared checks do not become a Windows build or runtime PASS.

### 2026-10-04 first-application default regression

Ordinary installed Dev12 showed EffectMain BadCallbackParameter(516), observed
when returning to ECW after the prior second-instance attempt; do not attribute
the modal timing to the ECW draw selector without a trace. Controlled immediate
add/render on the owned range-gesture copy recorded pending Kind0, hidden smoothing
100, Wave0 and spacing0.5; actual PNG absent and script status42. The initial
identity eligibility guard still requires legacy smoothing0. New-instance default
is now100. Source regression reproduces this rejection; actual production FFI
8/16/32-bit dense/sparse tests preserve exact neutral pixels at both normalized
smoothing0 and1. Planned fix accepts only these two exact defaults with all other
strict default-grid/mode/wave/spacing guards unchanged. Deformed/pending geometry
must still fail closed. Native exact new-candidate reproduction is required.

Fix source checks: default Rust79 PASS, including reproduced new-default
eligibility and exact production neutral pixels with smoothing0/1 across8/16/32
bpc dense/sparse. Probe strict Clippy and Python host/version contracts PASS.
The accepted exception is only exact smoothing0/100, with existing malformed,
deformed, Wave, topology and Four Corners rejection unchanged. Ordinary Dev16;
diagnostic17/preview-probe18/negative-control19. Native acceptance pending.

### 2026-10-04 ordinary Dev16 native regression closure

Source779db86 / EGFX-5e91589da403d19125f68235 ad-hoc verified, installed and
loaded exactly. Cua Version0.9.4 Dev16 observed. Before-render native record
Kind0/easing100/Wave0/spacing0.5 now yields an actual PNG exactly equal to the
first effect's frame; no error modal when returning to ECW. Removing the newly
added second effect preserves first pixels and4Grid/3Radius keys. Legacy starting
frame equals Dev12 saved-copy image. A delivered forward gesture left pixels
unchanged (cause unproven); reverse native drag changes pixels and exact Undo,
Redo and another owned saved-copy roundtrip pass. Original fixture untouched.
SourceRust79/Python18/strictClippy PASS. Native scope is8-bpc319x241 Final,
AE25.6x101/macOS26.6.2 arm64; source exact neutral tests also cover16/32-bpc.

Real scalar mouse scrubbing remains unaccepted: Cua drag noWindowsAvailable;
existing helper cannot address this ECW field under its fixed coordinates.
User was asked to perform this small check; no helper/TCC widening performed.
Show Grid, Windows and the complete update remain unaccepted. Installers stay
paused after the user corrected dependency order. No remote operations.

### 2026-10-04 live slider user acceptance

Human confirmed image and blue boundaries change during mouse movement for the
Affected Lines question about the last installed Dev16 test scene. Record as
USER-REPORTED PASS, distinct from earlier agent-scripted range changes. No more
question or helper scope expansion is needed for this same check. Native broader
plane/lifecycle coverage and U7 Show Grid are separate open checks.

### 2026-10-04 U7 route review and source-contract reconciliation

Reviewed SDK25.6 PF UI callbacks, Panels, Canvas/QueryXForm and PR_Public
against current Adobe SDK guide. Renderer-owned PR contexts expose window,
scale and translation, but are not a generic additive effect-viewer hook.
Owned panel views do not expose existing Composition viewer transforms.
Record and sources: persistent-viewer-grid.md. No new runtime route is proved;
Show Grid remains blocked on a supported implementation, with a scope question
pending. Requirements remain unchanged until the human answers.

Whole Python regression initial271: one obsolete source-wiring expectation
still called pre-live drag(), and three native observation/process tests were
blocked by sandbox ps and /Users/Shared restrictions. Updated the wiring test
to require current evaluated render + absolute-target drag_live(), retaining
layout sharing, density guards and changed-state-only publication checks.
The existing numerical/native evidence remains separate; this source scanner
is not pixel proof. Repeat with needed sandbox access pending; keep first log.

Repeated full Python regression271 PASS after wiring reconciliation and native
owned-child/process sandbox access. No production plugin code changed in this
block; installed Dev16 identity/source remains779db86. This is not a Windows AE
or Show Grid acceptance. User feedback check is now recorded, not pending.

### Dev20 lifecycle regression — 2026-10-04

Exact installed Dev20 / EGFX-bf6ecdde379f0831915ad144: two enabled
instances produce distinct pixels; deletion, Undo restoration and Redo deletion
restore exact corresponding PNG bytes. Grid Positions (4 keys) and original
Affected Lines (3 keys) remain intact. Skew Four Corners and a Y25 3D layer
show enabled grid pixels, and restored state matches the initial PNG exactly.
These scoped captures do not establish camera/parent or Windows acceptance.

FAIL: ordinary Undo after adding an instance cancels the deferred hidden-plane
binding rather than the addition. The idle route reinstalls that binding,
creating another `FSTR research binding` action and clearing Redo. Selecting
the owned addition in native History removes it and restores the baseline,
but is not an acceptable replacement for ordinary Undo/Redo acceptance.
Evidence: outputs/update-094-dev20-mac/lifecycle, including original FAIL
results.json and geometry-results.json. Do not count the first attempted
Undo-add as PASS. Overall lifecycle acceptance remains OPEN.

Dev24 candidate under development: attempt exact-effect initialization on
main-thread SequenceSetup inside the host's Apply Effect action; retain
worker deferral and closed failure for incomplete schemas. No render or
UpdateParamsUI writes are added. Native group merging is a hypothesis, not
verified behavior. Current installed plugin remains Dev20. Installers wait
for plugin validation; all work remains local.

Dev24 native candidate 203a4c3 / EGFX-8d8352fddcca4461285ffced failed
when adding the second effect: AE returned `child not found in parent`.
Its main-thread creation-time binding hypothesis is rejected. The source
restores deferred-only initialization for Dev28; this restores the safe
creation route, not ordinary Undo acceptance. Dev24 is not a deliverable.
Native failure retained at outputs/update-094-dev24-mac/lifecycle/two-failure.txt.
Source tests: Rust84 PASS, strict Clippy PASS, Python272 PASS after repeating
three environment-blocked native-process probes with authorized access.

### Dev32 Undo-respecting deferred initialization candidate

Dev28 b0544c3 / EGFX-732d6d6314a298290dbb44ac restores the safe route:
loaded image identity PASS; owned saved-copy reopen, two-instance addition and
deletion PASS, baseline/deletion PNGs exact Dev20 parity. Dev24 is superseded.
Evidence: outputs/update-094-dev28-mac/{lifecycle,loaded-identity}.

Dev32 candidate retains deferred-only writes. A bounded process-lifetime
registry records the five unique dependency stream IDs only after validated
success or exact existing binding. A subsequently fully blank, disabled,
unkeyed set with the same identity is treated as user Undo, without a setter
or a new Undo action. Foreign/partial/keyed states retain existing conflict
policy. No handles, parameter values or project contents are cached. No
eviction can unexpectedly recreate a cancelled action; registry overflow
fails before writes. StreamSuite6 (SDK25.6, introduced AE22.5) is authoritative.

This fixes a hypothesis for the observed automatic reinstallation, not the
separate initialization action itself. Native ordinary Undo/Redo, deletion
restoration, new-instance identity and close/reopen must pass before acceptance.
Any project/stream identity reuse or loss of Redo is a candidate failure.
The automatic initialization remains a distinct native action; do not claim
one Undo removes a newly added effect. Native Dev32 remains NOT RUN.

Dev32 dd8cf3b / EGFX-aa13d6dae9bd907a1ea0bfcf native FAIL: second effect
creation succeeds but its five hidden expressions remain disabled/blank.
Bounded idle journals report BadCallbackParameter. Undo removes the second
effect and restores baseline, but that is not valid initialized-render proof.
Do not count it as the intended Undo fix. Dev36 will retain the bounded native
error cause to distinguish unsupported/nonunique StreamSuite6 identities from
other adapter failure. No identity reuse assumption is accepted from headers.
Evidence outputs/update-094-dev32-mac/lifecycle/two.json and native journals.

Dev36 81e50a3 / EGFX-33f6f8c4bacc768774c2eeec establishes the native
cause: SDK StreamSuite6 returns [0,0,0,0,0] for these five effect streams
on AE25.6x101. The stream-ID registry route is rejected, not degraded to
effect index, raw handle addresses, names or selection identity. Error
retained at outputs/update-094-dev36-mac/binding-error.txt. Dev40 restores
safe deferred binding and preserves its bounded failure reporting.
Undo repair requires a real instance identity/lifecycle design that preserves
legacy sequence-data compatibility; pending source investigation.

### Dev44 per-instance receipt candidate

The rejected stream-ID cache is replaced by a compact versioned sequence
receipt, queried synchronously for the exact effect with EffectCallGeneric
on the captured main thread. No effect/stream handles survive a callback.
A scoped TLS request carries only read/mark and its scalar reply; no generic
extra pointer is dereferenced. Outside that scope callbacks are no-ops.
Legacy null/empty sequence state initializes generation0; version1 receipts
flatten with a magic/size/version check. Public parameters, GridArb wire3,
original animation streams and rendering ABI remain unchanged. PiPL enables
SequenceDataNeedsFlattening with existing MFR/GetFlattenedSequenceData support.

An owned eligible receipt is marked before the expression Undo group so the
group's prior sequence snapshot can retain it. If binding fails, the receipt
is cleared only while the exact target still validates, and clear failure is
explicit. Initialized blank or exact previous owned binding is respected as
Undo; foreign, partial and keyed streams keep the transaction conflict policy.
No current-time mutation, expression edits from render/UpdateParamsUI, cached
geometry or renderer fallback is introduced. The renderer ignores receipts.

Native ordinary Undo/Redo, deletion/restore and save/reopen PASS on installed
Dev44. The original animation streams and exact corresponding pixels survive;
initialization remains its own Undo action. The versioned receipt roundtrip and
legacy empty state are covered by source tests. MFR requested ON/OFF native comparison passes60 exact frames per run; actual
parallel callback execution is not instrumented. Scoped camera/parent checks PASS; expanded Wave UI PASS after user expansion and native screenshot inspection. Windows NOT RUN.
See [Dev44 evidence](update-094-dev44-lifecycle-2026-10-04.json).

### Dev44 continuation: Wave layout accepted; fresh-scene failure open

Expanded Wave Animation contains all five controls, with visible labels and
Axis popup matching the observed Plane/Edge/Quality width. User-assisted
expansion and agent native screenshot inspection PASS; no screenshot file
is claimed. Shared project-roundtrip export now waits for a fresh nonempty
PNG before closing its owned scene; native roundtrip PASS. Windows smoke
cleanup has an opt-in guarded fresh-project reset, with local regression tests.
These test/tool changes remain local and are not part of the installed artifact.

Full smoke remains FAIL. Separating creation, setters and exports did not
resolve it. A single owned Wave fixture reproduces a black viewer with Plane
Kind0 and disabled hidden bindings; hidden point reads report invalid numeric
result. Exact saved fixture reopen repeats it. Root cause is not established;
new-instance/deferred binding is under investigation. Earlier scoped Undo,
save/reopen and MFR pixel PASS do not establish whole-candidate acceptance.
Diagnostic outputs: outputs/update-094-dev44-mac/wave-isolation and phased-smoke
variants. The experimental phased fixture is not integrated or accepted.
Windows local packet predates these edits and must be regenerated after a
verified checkpoint. Installers stay deferred until plugin validation succeeds.

### Deferred smoke correction and native recovery — 2026-10-04

The black-frame investigation is resolved for the tested scenario. The legacy
monolithic script starts non-neutral rendering before AE can execute deferred
plane binding. Its render-error dialog then prevents later idle callbacks.
Observed native dialog: BadCallbackParameter516 (25::237). Closing that dialog
restores initialization without restarting AE or changing plugin source.

Two complete phased runs before the failure and one after dismissing the dialog
PASS all10 frames and7 pixel comparisons, including Adjustment Layer -> FSTR ->
Corner Pin. The old saved Wave fixture also binds/exports in a fresh session.
A hidden Point scripting .value error alone is not an initialization oracle:
it also occurs in an initialized, correctly rendering fixture. Five enabled
expressions/PlaneKind and independent exported pixels establish the scoped result.

Mac and Windows coordinators now share separate prepare/set/export/cleanup calls.
Exact nonce/item IDs/schema/scene guards and create-only phase outputs are
retained; Windows packet includes the new fixture and Mac acceptance packaging
includes its coordinator dependency. The integrated Mac coordinator passes22
steps and decoded pixels on PID18624, with exact installed Dev44 UUID/hash/Build
ID independently PASS. Native plugin source remains db230124; tool/docs changes
do not change the installed artifact. Windows build/runtime remains NOT RUN.
See [phased smoke evidence](update-094-phased-smoke-2026-10-04.json). Historical
failed captures remain FAIL; installer work stays after platform validation.

Source validation for the coordinator changes:277 unique Python tests PASS
combined (initial sandbox run had3 permission errors; required-access reruns
pass all12 process-guard and2 native-image cases). All script safety checks PASS.
Static audit's one heuristic auth/rate-limit finding is a test mock at
tests/test_target_ae_acceptance.py:54, not an HTTP route; adjudicated false
positive. Audit does not assess release readiness. Logs stay in local
outputs/update-094-phased-validation. No Rust/native source was changed.


### Mac-first closure checks — 2026-10-04

User explicitly deferred Windows until Mac completion; Windows obligations and
NOT RUN records remain. Source tooling73373d9, installed ordinary Dev44/db230124,
AE25.6x101 PID27219; loaded image UUID/path/hash PASS after controlled old/new
compatibility cycle. New Dev44 remains installed; old bundles retained outside
Adobe. No main/remote/CI/publication change.

A freshly created ordinary0.9.3 (54c215b, without AutomaticSpacing) fixture has
three animated spacing keys .013/4/25 and Radius keys1/3/6. Dev44 opens it with
AutomaticSpacing=false, preserves all those keys, and renders the exact old
neutral frame. Density4/4→9/8 and new-copy save/reopen retain pixels and policy.
This is bounded migration evidence, not every legacy project or the published
f611312 exact artifact. On an independently saved deformed animated fixture,
7/6→11/9, native Cmd+Z/Cmd+Shift+Z and new-copy save/reopen preserve exact decoded
pixels, four Grid key times and three Radius key times/values. Editing the old
Radius key3→6 changes137570 decoded channels immediately; native Undo restores
all recorded key metadata and exact pixels. Separate core tests cover encoded
arbitrary data. [Evidence](update-094-mac-closure-checkpoint-2026-10-04.json).

Installer work resumed AFTER these plugin checks. Native SnapshotReceipt stores
immutable pre-mutation full-tree expectations in a private transaction directory;
it contains no destinations. A fresh child process reads them and restores the
original exchange; changed current bytes refuse recovery without deletion.
FreshPublication shares the destination lock, requires absent destination and
same-volume verified staging, writes durable snapshot expectations, then uses
exclusive atomic rename. There is no overwrite/copy/delete fallback. SIGKILL
before and after publication is classified using the retained pre-mutation
receipt; collisions, links, lock contention and changed installations are
refused. Five strict Release CTest targets PASS in disposable fixtures; both new targets
also PASS under AddressSanitizer/UndefinedBehaviorSanitizer.
The native frontend still must collect/authenticate fixed roots, metadata,
duplicate/host state, stage and durably flush files, provide Install/Restore UI,
and pass real privileged package acceptance. These are internal primitives, not
a completed installer or a Mac release. Outputs/update-094-mac-installer-final
retains the native test log. Windows installation remains deferred.


### Version visibility correction — 2026-10-04

Latest user decision: version in About only, no Version row in Effect Controls.
Common source now hides the existing VersionRow with INVISIBLE | NO_ECW_UI and
removes custom drawing, retaining disk ID/order/u8 format and arbitrary dispatch.
About continues to derive 0.9.4 from Cargo; packaging/internal identity is retained.
Development PiPL builds 48/49/50/51 distinguish this update from installed Dev44.
Source/native build and exact new Mac panel/reopen validation precede acceptance;
installer obligations remain open. No Windows runtime or release claim.


About-only correction is now installed on Mac (ordinary Dev48), source
bfe4036e9baa201fc1e24dcf3fb7f8a02e568429, build
EGFX-7f95d683e74ad94347c4e1ec. AE PID35685 loaded its exact UUID/path and
payload hash. Native Effect Controls screenshot confirms no Version row.
The saved animated-density fixture opens unchanged; its frame and all recorded
animation times/values, density and spacing match Dev44, including save/reopen.
15 host contract checks, 23 build identity/About checks, 86 Rust tests and strict
Clippy PASS; signed package verification PASS. About generation and command are
preserved; the native About dialog itself was not opened. Evidence:
outputs/update-094-about-only-mac/verification.json. This supersedes Dev44 as the
installed local candidate; previous payload remains backed up outside Adobe.
Installer/frontend/package obligations and Windows deferral remain unchanged.


### Commercial licensing plan addition — 2026-10-04

User authorized adding activation/license management to the development plan.
U11 in feature-backlog.md records Activate before registration, License after
activation, About/version/support in the window, activated offline operation and
render/MFR isolation. Native placement beside global Reset must be investigated;
no arbitrary host-header caption capability is assumed. Sales channel/provider
and commercial policies require definition before dependent implementation.
This is documentation-only planning, not implemented licensing or release
acceptance. Existing U0–U10 obligations/Evidence remain; U10 now includes U11.
Mac first, Windows deferred and local-only publication restrictions persist.


### U11 integration research — 2026-10-04

Sales channels confirmed by user: aescripts and Plugin Play. SDK25.6 and pinned
Rust binding expose PF_SetOptionsButtonName during ParamSetup; IDoDialog needs
matching PiPL/global flags and an Instance dialog handler. This resolves the
source-level caption uncertainty, not actual host placement or caption refresh.
Public vendor material does not supply native adapter contracts/product IDs/test
entitlements; none found in project/output inventory. User confirmed no
author access yet; provider integration is BLOCKED on vendor materials. No fake licensing or guessed key generator added; installed
Dev48/render unchanged. Native header/prototype and provider acceptance NOT RUN.
See licensing-integration-2026-10-04.md for source references, vendor packet
requirements, dual-market/offline unknowns and dependency-ordered acceptance.


### Independent Mac licensing window — Development

User authorized independent UI before marketplace SDKs. Native License... header
entry and AppKit License/About/Support/Close window implemented for Mac; About
uses Cargo0.9.4 and VersionRow stays hidden. No activation adapter, key collection,
network validation, saved-project changes or renderer changes. Store integration
remains BLOCKED on author materials; native window validation pending.
Development PiPL builds52/53/54/55 distinguish the new UI candidate. Host contract
15 and About/build identity23 tests PASS; cargo check PASS. Mac runtime validation
and signed candidate identity must precede installed/accepted claims.


Independent Mac License UI is now installed as ordinary Dev52 source740dbaa,
build EGFX-01242dd7b423e5aa1384abae. Exact loaded identity PASS; native header
License... beside Reset, click/open, About0.9.4/return and Close PASS in AE25.6.
Support click processed without failure alert; browser destination verification
is incomplete (browser policy-loader error, unrelated subsequent Chrome page).
UI invocation causes AE's ordinary DO_DIALOG dirty mark; all recorded key/value
state and pixels remain unchanged, including save/reopen. Rust86 + strict Clippy,
host contract15 and About/build identity23 PASS. Evidence:
outputs/update-094-license-window-mac/verification.json. Dev48 is retained in a
backup outside Adobe; Dev52 remains installed. Provider activation still BLOCKED
on author materials; Windows deferred; native installer/frontend release work
remains pending. No full commercial release/performance acceptance claim.


### Windows parity resumed — 2026-10-04

User resumed Windows update parity and excluded speed measurements. Implemented
native Win32 License/About/Support/Close bridge, shared License... header and
matching PiPL/GlobalSetup IDoDialog; no new license/backend or render mutation.
CPU source audit confirms the same axis/row caches, exact-copy path, quality,
SmartFX/MFR flags and x64 SSE availability. GCD/std::thread scheduling differs,
Metal is not a Windows backend; no speed promise/benchmark required. Native
Windows build/AE NOT RUN due to no Windows/MSVC/SDK here; old AEX not reusable.
Installed Mac Dev52 unchanged; source development builds56/57/58/59. Local-only
constraint persists. See windows-update-parity-2026-10-04.md; U8/U10/provider
obligations preserved.


Windows parity local verification: 86 Rust tests, strict Clippy, 15 host contract,
23 About/build identity checks, 20 Windows packet tests and 7 selected Release
core/dialog correctness targets PASS on Mac. The dialog wire-layout parser also
passes strict clang warnings including conversion warnings. These are explicitly
not Win32/MSVC/Windows AE results. Windows-only null/worker UI guard test added
to the existing Windows CTest build, still NOT RUN until that runner executes.
Evidence: outputs/update-094-windows-parity. Benchmarks/speed tests not run.


### Windows CI publication and Reset ABI correction — 2026-10-04

User authorized sending feat/next-update for Windows CI after the local-only
checkpoint. Published e5f4491; run37189091016 passed all24 Windows CTest targets
(including native License null/worker guards) and packaging-tool tests, then
failed Rust compilation: Grid Positions Reset continuation used isize while
Windows SDK A_intptr_t binds i64. Replaced the private continuation state with
the exact SDK ABI alias, including diagnostic builds. No stream/parameter or
render changes. Three Reset regression tests pass locally; next exact-head
Windows build pending. Evidence: outputs/update-094-windows-ci-e5f4491/job.log.
Automatically triggered Mac run37189091025 was cancelled to avoid redundant
benchmark work; this is not a passing Mac gate. Main/release and installed Mac
payload unchanged; Windows AE runtime and broader U8/U10 obligations remain open.


### Windows validation candidate built — 2026-10-04

Exact source cbb7468b36e9b94b4c575b3de1c27863ebe8caf2 passed Windows run37189319759:
24 CTest cases (including native License guards), 86 Rust tests, Clippy, optimized
release build, x64 PE/EffectMain/dependencies/imports and byte-exact PiPL. Candidate
0.9.4 Dev56 BuildID EGFX-376f97bdd493cd2fbce61edd; AEX SHA256
669436b62a06baef0eba895e8e72b50f89e448ff396c24c43e16a2f93c15bd6c. Downloaded
archive/manifest/embedded BuildID/inventory hashes verified locally. Reconstructed
Windows CRLF checkout with Windows file permission semantics matches CI source
hash exactly. Evidence: outputs/update-094-windows-ci-e5f4491/delivery-verification.json;
user packet: outputs/FSTR-Stretch-0.9.4-Windows-Dev56. No speed tests requested.
Windows AE load/UI/render/Undo/MFR remain NOT RUN, not inferred from build tests.
Actual marketplace activation and native installers remain open. Installed Mac
Dev52, main and release unchanged. This documentation checkpoint does not change
the candidate: its exact source remains cbb7468, not the documentation-only HEAD.


### Windows manual checklist accepted by user — 2026-10-04

User reported OK for all10 checklist items against the delivered Windows
0.9.4 Dev56 candidate (source cbb7468, BuildID EGFX-376f97bdd493cd2fbce61edd,
SHA256 669436b62a06baef0eba895e8e72b50f89e448ff396c24c43e16a2f93c15bd6c):
AE launch/application; grid drag/Undo; uniform density with preserved deformation;
live Affected Lines including old animation; current-time Reset/keys/resize;
Show Grid with another selection and Preview; Wave controls; save/reopen;
Preview/render correctness; License/About/Support/Close.

Result is USER-REPORTED MANUAL PASS for this checklist, superseding NOT RUN
for these manually exercised workflows only. Exact AE/Windows versions and
loaded binary identity were not captured on the test machine; no screenshot/
automated runtime packet supplied. Separate MFR/aerender/cold-start/migration
coverage is not inferred. Speed measurements excluded by user; no timing claim.
Evidence: outputs/FSTR-Stretch-0.9.4-Windows-Dev56/user-acceptance-2026-10-04.json.
Native installers/update backup workflow and real marketplace SDK activation
remain open; full U10/release reconciliation still required. No merge/release
authority follows from this acceptance; main and installed Mac unchanged.


### U8 native frontends — in development 2026-10-04

User requested native Install/Restore for both targets after Dev56's ten-item
manual acceptance. Implemented fixed-root Mac app and Windows executable with
embedded unchanged accepted payloads (Mac Dev52 EGFX-01242dd7b423e5aa1384abae,
Windows Dev56 EGFX-376f97bdd493cd2fbce61edd). Installer source identity is separate
from plugin source identity; no plugin rebuild or speed measurement is required.
See native-installer-plan.md for U8.1–U8.6 acceptance and failure boundaries.

Local payload/decision Python checks:9 PASS. Mac arm64 frontend compiles with
warnings as errors; native disposable-root fixtures exercise exact signed Dev52
and retained Dev48, fresh/idempotent/update/restore, tampering, blocked hosts,
duplicates, links and incomplete transactions. Isolated roots preserve evidence;
no installed Adobe payload was changed by these checks. Expanded interrupted
publication/locking cases also PASS, as do the five native Mac core CTest
targets. Final package checks remain in progress. Static code
scanner reports one existing heuristic in test_target_ae_acceptance.py (an
acceptance fixture, not an authentication endpoint); no new installer finding.
Scanner output is not release or privileged-executor certification.

Windows native MSVC/runtime fixtures/package and real administrator Install/
Restore acceptance on both platforms remain NOT RUN at this checkpoint. Full
release, marketplace SDK activation, main/merge authority remain separate.
Windows native installation root corrected to x64 ProgramFiles/Adobe/Common;
earlier CommonProgramFiles note is superseded, see plan primary references.
Evidence: outputs/native-installer-development/. Next: finish native checks,
package Mac app, execute Windows installer CI on the authorized feature branch.


U8 package follow-up: Mac installer app from clean c9b62a8 compiled/signed and
strict signature verification PASS. Delivery ZIP SHA256
054cc637ac722fcb3879688cc9a1f6e4a5c62cec3fa3cbe10f59354b5c69567b;
outputs/FSTR-Stretch-0.9.4-Mac-Installer/installer-artifact.json separates
installer source from unchanged Dev52. Native UI observation returned timeout;
visual/administrator installation acceptance NOT RUN, no system replacement.
Windows CI37199357535 failed compile (filesystem ADL name collision and missing
COM declarations), before native fixture execution. Corrected path helper naming,
explicit COM header and Unicode native UI declarations; next run pending.
Unnecessary plugin rebuild/Mac speed workflows cancelled, unchanged accepted
Dev56 is the package payload. Edge sampling CTest also PASS on current Mac tree.


U8 Windows follow-up: b0ccf24 compile PASS but fixture failed before useful
exception reporting. 8bccb2e fixed exclusive-handle security inspection; fresh
install/idempotence PASS, replacement failed backup metadata verification.
f27bb36 CI37200139164 confirms access rules alone differ after NTFS backup
rename; original file identity, bytes, attributes and times matched. Automatic
review rejected a broad ACL comparison proposal, which was not applied. Current
implementation instead compares owner/group and exact preplanned DACL alternatives
(original or verified staged protected-parent DACL), independently requiring
trusted write principals. It binds the retained full snapshot and requires exact
original full snapshot after Restore; backup-ACL tamper fixture added. Next CI
pending, no administrator Windows installation claim. See native-installer-plan.
Native Edge Behavior demonstration produces three distinct outputs for the same
fixture: outputs/edge-behavior-example/comparison.png (left Clamp, center Wrap,
right Mirror). This is shared PlaneRenderer evidence, not AE popup runtime proof.


Windows fixture10d7f1f CI37200489252 disproves the prior re-inheritance
hypothesis: backup permissions are frozen (same owner/group/principal masks,
INHERITED_ACE removed, DACL protected). The active replacement combines old
explicit permissions and staged inheritance. Current correction computes the
exact protected original descriptor from prepared, with all other snapshot
fields exact; Restore explicitly reapplies only that original permission set
and verifies the resulting deterministic full snapshot. No unknown ACL policy
is accepted and no system installer action has run. Native CI pending again;
preceding failures remain FAIL. Diagnostics are only in disposable test code.


Windows c7d60b6 CI37201385419: all disposable native installer fixtures PASS
(including update/Restore/repeat, target tamper, backup ACL tamper, interrupted
operation, blocked host, uppercase duplicate, pending, lock and reparse). Pinned
Dev56 fetch PASS. /W4 /WX exe compilation PASS; linker failed due conflicting
asInvoker/requireAdministrator manifest snippets. Fixed linker request to the
same explicit requireAdministrator policy, never disabling UAC. Final PE manifest
inspection remains pending. Latest Mac package is v2 from60fe49c (ordinary Dev52
unchanged), ZIP SHA256 f364d64d80084afa7c1da908bb6ea11dd07429b198b18f32eebf6a7fc86b59e7.

## Four-mode source checkpoint — 2026-10-05

Implementation under four-modes-demo-plan.md: Comp/Flat preserved ordinals1/2;
Layer/Perspective appended3/4, v3 owned expression migration with generation2
Undo receipt, shared single-sampler pin dispatch and explicit disabled Demo.
Local90 Rust,282 Python,28 CTest, all JavaScript tests and applicable strict
C++/Clippy analyzers PASS before the checkpoint commit. Initial Python run
failed for sandbox process-read restrictions and an obsolete marker-range
guard; the corrected guard and authorized native-process rerun pass. Initial
Clippy argument-count issue and pre-existing strict C++ conversion warnings
were repaired; numerical algorithms remain unchanged. Retained scanner finding
at tests/test_target_ae_acceptance.py is a mocked-fixture false positive.
Mac AE/new Windows artifact acceptance still NOT RUN. Source-check logs are
retained in outputs/four-modes-source-checks outside the repository.

### Surface caption continuation

User requests Flat mode renamed Surface mode for Mac and Windows. Dev108 shared
caption-only implementation; ordinals/internal enum/geometry unchanged.
Installed **0.9.4 Dev108**, sourceb7ac1859817ddbc33618acd49f60a90aa8733689,
Build ID EGFX-60af8664e175098b8c7743ae; package/signature/atomic replacement/loaded image PASS.
Existing UI order tests2 PASS, host contracts15 PASS. Dev104 edge-policy native
visual/pixel checks retained separately. Surface caption native visual pending:
user actively operates AE; Cua stopped input and user asked to confirm label.
Current scene changes preserved locally; other compositions not edited.
Windows37312709671 PASS; downloaded exact AEX/manifest identity PASS:
EGFX-07e85f0eb754ca316d9697e7,
SHA25681d6de5b2e153d7491f0c23eb962a3b36203be80fb25e6f6f6d41386f685bc1f.
Windows AE NOT RUN. Mac37312709680 IN_PROGRESS. Native Install/Restore and
marketplace activation obligations retained; no main/merge/release.
Evidence outputs/surface-dev108 (identity, CI, package and WINDOWS-CHECK-RU.txt).

### Active Perspective clipping fix

User Dev108 runtime FAIL: moved corners cross layer limits but image remains cut
at source rectangle. Dev112 fixes host output bbox and core destination clipping
for Perspective; source sampling/quad outside transparency unchanged. Old-core
regression reproduced; source/API/thread/ownership checks preserve existing
immutable SmartFX and legacy-origin mechanisms. Installed **0.9.4 Dev112**, source45350b22ec7c26735b468b632d13455c49813b25,
Build ID EGFX-422a6cfe0dcda2e3448e5aff. Package/signature/atomic replacement PASS.
98 Rust/15 host contracts PASS, new expanded-destination test PASS under ASan/UBSan
for8/16/32bpc and both qualities. Native AE25.6 screenshot-quad Final/32bpc PASS:
alpha bbox81,27–438,357;15407 nontransparent pixels above original source top116.
User scene preserved as user-preserved.aep; check copy left open in Perspective.
Loaded identity checked separately in outputs/perspective-dev112/loaded-identity;
Mac37317165500/Windows37317165263 on45350b2 PASS. Windows new native behavior NOT RUN.
Evidence outputs/perspective-dev112/native-pixels.json; no release/merge.

### Procedural Demo watermark — dormant implementation

User authorized original procedural FSTR FX per destination cell, without an
external logo/font file. D1 renderer implemented in shared Rust host paths for
Mac/Windows; D2 rollout remains compile-time OFF with explicit bypass before
parameter reads or pixel access. Legacy/SmartFX hooks use evaluated grid and
owned snapshots; no saved IDs/streams, UI toggle or fake license changed.
Vector glyphs transform with each cell, rasterize via existing antialias/depth/
stride/origin/cancellation primitives. No unremovability or encryption claim.
Marketplace SDK verification remains required before enabling rollout.
Dev116 source candidate; installed Dev112 remains unchanged for Perspective
acceptance. Initial compile/test failures retained in outputs/demo-procedural-dev116;
101 Rust tests/15 host contracts and strict Clippy PASS. New tests cover OFF
byte preservation, evaluated-cell renderer, glyph bounds,8/16/32bpc padding and
negative-origin tile parity on a perspective quad. Static scanner exit1 retained:
sole finding is the existing mocked-fixture auth false positive, reviewed.
Dev116 source1c97b7d4758bbbcb91435dbaae63224e4e90b307 release build PASS;
known rust-objcopy/libLLVM resource-helper warning retained (build fallback succeeds).
Mac37319481143/Windows37319481240 CI IN_PROGRESS; enabled-demo AE runtime
NOT RUN because rollout OFF. Source diff reviewed against D1/D2; no cleanup
needed, evidence/backups preserved. Installed Dev112 unchanged.
Existing installer, new Windows native acceptance and
marketplace activation obligations retained; no main/merge/release.

### Dev116 native continuation — 2026-10-05

Current installed ordinary Mac **0.9.4 Dev116**, source0a5b712bfe95e0d29d54b94f23e3a4920df3794d,
Build ID **EGFX-f0e50f471f52dd30b6fdb7cf**. Clean rebuild/package/signature,
atomic replacement retaining Dev112 and actual loaded-image UUID/path PASS.
Source differs from CI1c97b7d only in STATUS/SHA256SUMS (verified); do not
present the different binaries as one artifact. Mac37319481143 and
Windows37319481240 source1c97b7d PASS. Windows artifact11349771011 downloaded,
ZIP CRC/path/size and AEX manifest/SHA/marker/source identity PASS:
EGFX-e220e261736792f3b3608874, SHA256
 dd024117ca8303b36cfec56ce2c3d95dc523c1749bc284e48c6a94ae0e9cc928.

Native AE25.6 fresh owned scene: OFF-demo vs effect-disabled frames exactly
match decoded RGBA at8/16/32bpc;32bpc Perspective alpha bbox161,30–518,360,
15535 visible pixels above measured original source top119. Actual bounded
Comp-line drag changes178265 components; Cmd+Z returns exactly original pixels.
Initial helper foreground refusal emitted no input; authorized activation retry
succeeded. No expanded helper permissions, new OS access or arbitrary UI input.
Current scene separately preserved as gesture-check.aep. No private user media
used or published. Initial download filename assumptions rejected safely before
payload use; exact artifact identity corrected and verified.

Demo stays OFF, no public toggle/provider verification. Enabled-demo AE runtime
NOT RUN by policy. Latest Windows native acceptance and Mac hidden-Effect-Controls
loupe acceptance remain pending human checks requested in this conversation.
Do not mark whole plugin complete or advance privileged installer acceptance until
plugin checks close. Native installer/payload promotion and marketplace SDK
obligations retained; no main/merge/release. Evidence outputs/demo-procedural-dev116
(native-pixels.json, gesture-pixels.json, loaded-identity, package-verification.log,
CI records, windows-identity.json, WINDOWS-CHECK-RU.txt, backups).

### Mac loupe user acceptance and installer payload promotion

User confirmed on installed Dev116: corner magnifier shows the image and closes
on release with Effect Controls visible and switched to Project. Exact reported
scope: one of Surface/Perspective as requested; the chosen mode was not specified.
Retain previous Dev80 two-mode evidence separately; do not infer an unstated mode
or fresh-instance/no-prefetch scenario. Windows Dev116 user acceptance remains
pending: user will test and report. Proceed with Mac native installer packaging
using the unchanged verified Mac Dev116 payload EGFX-f0e50f471f52dd30b6fdb7cf.
Do not promote Windows payload/installer before that platform's plugin acceptance.
Mac administrator Install/Restore acceptance remains NOT RUN. Marketplace SDK
integration and release remain deferred/unfinished, Demo remains OFF.

### Mac installer launch follow-up

Initial c272cc2 package compiled/signed and exact Dev116 embedding/archive
verification passed; 9 payload/decision tests passed. Native window observation
repeatedly timed out. A directly launched exact executable remained in
NSAlert::runModal (outputs/mac-installer-dev116/ui-process-sample.txt), with no
stderr; this does not prove visible UI acceptance. The modal-only entrypoint
never called NSApplication::run or finishLaunching. Complete the documented
launch lifecycle explicitly before showing its first alert.
Apple authority: https://developer.apple.com/documentation/appkit/nsapplication/finishlaunching%28%29
Regression window/Close verification pending; no administrator action ran.
Installer engine and embedded plugin remain unchanged. Windows plugin acceptance
still pending, Demo OFF.

Mac installer launch regression — a1217da: clean native build/signature PASS.
Cua now opens the exact app and shows Version 0.9.4 / Install / Restore / Close;
screenshot confirms readable layout. Close eventually terminates the app
(Cua final observation: App quit). First post-click inventory/tree was stale;
no Install/Restore was selected. Exact 6 embedded files, ZIP CRC, full archive
bytes/executable flags and binary/archive manifest digests PASS.
Evidence: outputs/mac-installer-dev116-launch-fix/{installer-artifact.json,
payload-verification.json,build.log,signature-verification.log}.
Installer source a1217da25ae797626c36c30fba1277c4e9d5f696; embedded Dev116 source
0a5b712 / EGFX-f0e50f471f52dd30b6fdb7cf remains unchanged. Delivery ZIP SHA256
b7a648ca17799a527fe87b227510c30385341856eafa3897af17fca1d4a4a720.
Real administrator Install/Restore still NOT RUN; Windows Dev116 user test
pending. No main/merge/release/publication action performed.

### Installer completion flow correction

User's two screenshots show Done / This version is already installed followed
by the initial Install/Restore window. This is the old frontend loop, not a
second error or proof of new publication/backup. The already-installed native
path was exercised by the user; real replacement and Restore remain NOT RUN.
After a successful native operation (including already-installed), OK now ends
the installer session on Mac and Windows. Failed/cancelled authorization retains
the existing retry/Close behavior. No transaction engine or plugin bytes change.
Mac rebuild/initial window/Close validation and Windows compilation pending for
this source. Windows Dev116 plugin acceptance still pending. Demo remains OFF.

Completion-fix Mac candidate c84cebe: clean build/strict compiler/signature and
exact 6-file embedding, ZIP CRC/bytes/executable bits, manifest digests PASS;
9 payload/decision tests PASS. Evidence outputs/mac-installer-dev116-completion-fix.
ZIP SHA256 fe67ef6162d072366a7ce92ed522c6c65ec7d62f1c09f44a47a0d8d6b977a96b.
Successful-operation exit runtime and administrator Install/Restore NOT RUN;
Windows frontend compilation NOT RUN. These remain distinct from the previous
initial-window PASS and user already-installed result. Exact plugin unchanged.

### Host-close installer continuation

User requests continuing the selected installation after closing AE instead of
returning to the main menu. Mac/Windows now retain Install/Restore and display
Continue/Cancel on typed pre-transaction HostsRunning. Mac maps this initial
check only to CLI exit75 / NSAppleScript error75; errors during runNative remain
exit1, never automatically retried. Windows preflight catches the typed refusal
before calling the engine. Every Continue repeats verification; incomplete scan
and other errors retain existing refusal/recovery. No host is force-closed.
Updated isolated fixtures exercise no filesystem mutation on running host,
installation after clearing the host, blocked Restore and Restore after clearing.
Mac strict build + fresh/repeat/update/restore/tamper/host/duplicate/link/pending/
interrupted/lock fixtures PASS, output /private/tmp/egfx-native-frontend-1FHLig.
Evidence outputs/mac-installer-host-continuation/native-fixtures.log.
9 Python payload/decision checks PASS. Windows compilation and native GUI
Continue/cancel/real administrator flows pending. Plugin bytes unchanged, Demo OFF.

Host-continuation Mac package fe8a902 strict build/signature PASS, exact6-file
embedding and complete ZIP/manifest verification PASS. Delivery:
outputs/mac-installer-dev116-continue/FSTR-Stretch-Installer-Mac.zip
SHA256 931470f17ab10fa606039c8051a3af62026967623239086e6ba6ba08554eec65.
GUI Continue/Cancel after live AE refusal and real Install/Restore NOT RUN.
Windows code/fixtures updated locally, compilation still pending.

### Animated grid midpoint jump — saved scene investigation

User reports discrete last-key transition in currently open showTime.aep.
Read-only JSX metadata: Grid Positions final times2.4/5.84, both linear6612;
CUSTOM_VALUE retrieval unsupported by AE scripting, so no key writes attempted.
Read-only saved binary scan finds last adjacent states4x4 then3x4 with both
column axes neutral, rows deformed. Native before PNGs2.4/3.2/4.08 identical;
4.12/4.16/5.84 identical;181589 RGBA components change exactly at midpoint4.12.
Evidence outputs/grid-animation-jump, project remains unmodified; retained
local saved-file copy, no public project/material upload.
Reset incorrectly used viewer counts to replace retained animated topology.
Now Reset neutralizes existing retained dimensions. Legacy interpolation of
unequal axes normalizes only a neutral endpoint to the deformed endpoint's
topology, losing no deformed knot; matching-topology fast path unchanged.
This fixes this reported neutral-column mismatch without rewriting stored keys.
Two unequal non-neutral lattices still retain previous step behavior; arbitrary
non-neutral topology morph is outside this narrow correction and not claimed.
102 Rust checks PASS with actual reported pair regression across .499/.5/.501
and unchanged serialized keys; strict Clippy precision fixture spelling repaired,
follow-up PASS. Dev120 ordinary (121/122/123 diagnostic variants). New binary
packaging/install/AE after-render and Windows verification pending. Existing
installer embeds old Dev116 and is not this new plugin candidate. Demo OFF.

Dev120 clean Mac source3c39a69 package/signature/manifest PASS, BID
EGFX-5fa3daad62f2d9425b6364d3. Before restart the current session was saved to
outputs/grid-animation-jump/showTime-session.aep (original Desktop project not
overwritten); AE closed. Atomic test replacement refused BEFORE swap with
PermissionError13: previous native installer has protected the destination.
Receipt outputs/grid-animation-jump/backups/EGFX-update-bf36c963e8234436ab3b6998a5a385b0/receipt.json
is FAILED_BEFORE_SWAP; do not claim Dev120 installed. System Dev116 retained.
Package the exact Dev120 in the native administrator installer for human
authorization. After installation, reopen saved session and compare same2.4–5.84
frames, retain original key count/types and endpoint geometry. Native Dev120
acceptance and current Windows compilation NOT RUN. No main/push/release.

### Critical Mac RAM Preview incident — Dev120

User reports AE stalls during plugin animation RAM Preview then whole Mac
hangs; forced reboot confirmed. Installed disk identity is Dev120 source3c39a69
EGFX-5fa3daad62f2d9425b6364d3. Loaded identity during incident not independently
verified. ResetCounter19:46 reports force_off. After reboot observed severe
load/swap pressure; this alone is not plugin attribution. Earlier Jetsam16:21/
16:25 reports list AE largest process on16GB Mac; do not assign those events to
latest hang. Old AE disk-write diagnostics likewise are not latest crash proof.
Evidence outputs/ram-preview-hang-dev120/incident.json, local-only diagnostic
summary. No repeated RAM Preview, host closure or software installation performed.
Source review risk: Surface/Perspective Effect-pane DRAW calls loupe async full
frame preparation without an active corner gesture; global timestamp/refresh
interaction and playback requests need investigation. Retained64MiB copy bound
is after host render request and is not a host-memory budget. Root cause remains
UNCONFIRMED; no speculative fix accepted. Release/installer promotion blocked
on incident resolution; previous green tests do not certify RAM Preview safety.
Next: identify exact incident scene/time/settings, inspect request lifecycle,
then bounded isolated verification before any user full-preview acceptance.

### RAM Preview request mitigation candidate — Dev124

Read-only SDK inspection confirms the open showTime-session.aep copy: active
1920x1080/25fps/8s composition,32bpc, Surface canonical2, Final2, ShowGrid0,
eight Grid Positions keys. Current UI Project tab is visible; incident-time pane
state remains unknown. No time/parameter/key/project mutation or playback.
Dev124 limits inactive loupe preparation to one initial owner/time warm request
and polling at that same time. Advancing playback times cannot enqueue replacement
loupe frames; cached owners require an active mouse-down corner gesture. Global
window refresh after a completed copy is now gesture-only. Shared Mac/Windows
policy; renderer, key formats, output quality and Demo OFF unchanged.
103 Rust tests and strict all-target Clippy PASS. Policy regression simulates
10000 advancing times with pending warm request and10000 with cached frame; these
are not live AE request counts. Static review found only the prior mocked verifier
fixture heuristic at tests/test_target_ae_acceptance.py:54; no auth endpoint.
Root cause UNCONFIRMED. Full RAM Preview NOT RUN; no stability acceptance or
release promotion. Candidate also needs loupe regression after changing time and
switching Effect Controls to Project: the retained frame is time-qualified, so
a hidden pane may no longer refresh it. Preserve this requirement as pending,
not silently accepted or removed. Build/package identity and bounded host checks
remain to be recorded separately.

Dev124 Mac artifact build/signature/archive/manifest PASS, source674c984, BID
EGFX-b6c190318a974348695d86e0. Native installer package prepared in local
outputs/ram-preview-hang-dev120/dev124/installer; not installed, no promotion.
Post-reboot swap7.69GiB observed19:59; heavy host verification not attempted.
[Frame/redraw source audit](ram-preview-request-audit-2026-10-05.md) found no other
independent async frame requester; conditional density invalidation, no-op corner
writes, repeated UI/parameter updates are review candidates, not proven hang causes.

### Redraw/dependency corrective work — Dev128

User authorized repairing the source-audit findings under rules8.0.0. R1 density
keeps viewer reflow but sets ForceRerender only for Show Grid/Demo pixel overlays;
AE's own dependency invalidation is not disabled. R2 no-op corner DRAG compares
exact SDK16.16 values before Point write/CHANGED_VALUE/UPDATE_NOW; gesture
continuation/release remains handled. R3 popup values and disabled flags are
compared against the current host definitions before UpdateParamUI; no global UI
cache. R4 callback-local primitive dependency cache shares reads between plane
and SmartPreRender construction, including binding kind/points/mode and pending
neutral guard. The already-owned exact retained grid is reused for that guard.
No handle/pointer retained, every checkout checked in before caching, no value
carried to another frame or effect instance. Keys/IDs/defaults/pixels/ROI unchanged.
Shared Mac/Windows source; Demo remains OFF. Dev128 ordinary/129-131 probes.
108 Rust tests PASS; strict Clippy PASS before final comment/test tightening.
Static review only the existing unittest.mock fixture heuristic at
test_target_ae_acceptance.py:54; no product auth route. Final checks/build/package
follow below. Native feedback/Undo/after-scrub loupe and full RAM Preview NOT RUN.
Critical Mac incident still UNCONFIRMED and blocks promotion; no stability claim.

Final Dev128 source checks:108 Rust tests PASS, strict all-target Clippy PASS,
diff whitespace PASS. Separate review: cached primitives retain exact bits;
checkout temporaries dispose before insertion; only owned retained grid reused;
current host UI comparisons avoid persistent Undo-stale state; no-op corner
release still ends gesture. No renderer or saved-stream byte changes. Logs local
outputs/redraw-work-dev128. Mac package build follows from clean committed source.

Dev128 clean Mac package/signature/archive PASS, source10c808f, BID
EGFX-5de4d8b966a262b8e0741f0c. Native installer package SHA256
719539541617fe38df3f43b2de06f99690835240ecff16e2dc15bae5ffd39e90.
Saved a separate showTime-before-dev128.aep copy before quitting AE. System disk
payload now matches every file hash of the Dev128 manifest; native installer
reported AlreadyInstalled after user interaction. Do not infer loaded acceptance.
Published source10c808f to authorized feat/next-update CI only. Native installer
CI37353761077 PASS. Mac CI37353761076 FAIL: three source-string guards still
expect pre-cache/pre-UI-guard spelling. Updating the guards to follow cached SDK
loaders and checked-out retained grid without removing old fail-closed assertions.
Product/installed binary unchanged by this test-only correction. Windows and
follow-up Mac CI results remain pending. AE currently closed, playback NOT RUN.

Follow-up request-policy review found a concrete multi-instance defect in the
Dev124/128 mitigation: alternating owners resets its single warm key, allowing
new idle playback requests. A regression test FAIL before repair is retained in
outputs/redraw-work-dev128/multi-owner-before.log (no live Mac reproduction).
Dev132 uses32 bounded owner warm records; completed owners cannot restart after
frame-copy eviction. Pending owners can poll only their original time/scale.
New idle owners after capacity fail closed; active corner gestures remain allowed.
No pixel/source/key change. Dev128 installed until exact Dev132 replacement.
This repairs the reproduced policy defect, not proof of the whole-Mac hang cause.

Dev132 final policy checks:110 Rust tests and strict all-target Clippy PASS.
Terminal receipts mark warm work finished even if copy/validation/checkin fails,
preventing idle retry loops; active gestures still permitted. Source review
bounds owner history and checks no frame handle survives checkin.
Earlier Dev128 follow-up Windows CI37354389182 PASS on test-only c14bb0c; it does
not cover the new multi-owner policy. Mac follow-up still running at checkpoint.

Dev132 source5caf51b, BID EGFX-dcb60c943ae33947aa41755b: Mac bundle/
signature/archive/manifest PASS. Installer ZIP SHA256
79a2bf3db94dbf228e29039d47c857347111a677d583c498bd069e81a62306af.
User confirmed native administrator installation completed. System plugin files
match the candidate manifest exactly; installed codesign deep/strict PASS.
Windows CI37355265964 PASS on this exact source. Mac CI37355265994 PENDING
at this checkpoint, full preflight running with no failed steps. Loaded host
identity, native feedback/Undo/after-scrub hidden-panel loupe and RAM Preview
NOT RUN. A short non-playback user check requested; no heavy preview launched.
Local evidence outputs/redraw-work-dev132/verification.json. Original critical
Mac hang remains OPEN, cause UNCONFIRMED; no release/merge/promotion.

Dev132 short native check USER-REPORTED PASS: corner motion/release, Undo, and
loupe after changing time with Effect Controls hidden. User reports lens appears
only after movement; new requirement is appearance on stationary corner press.
Dev136 candidate records only a hit-tested corner hover at foreground UI pointer
position, scoped to owner/viewer/time/scale. A held left-button press at that
position starts the native Point gesture without a coordinate change. No DRAW
mouse union read, input synthesis, timer, permission, idle async-request change,
key write or renderer change. Hover clears on non-corner motion and lifecycle
release; stale owner/view/time/pointer mismatches reject it. Mac/Windows native
pointer queries are read-only. Native stationary-press acceptance remains NOT RUN.

Dev132 Mac CI37355265994 PASS; Windows37355265964 PASS on source5caf51b.
Dev136 sourcea89b949:111 Rust tests, strict Clippy and24 focused Python guards
PASS; Windows CI37356962042 PASS, Mac37356962076 PENDING. First local hand-built
package incorrectly named the resource elasticgrid_ae.rsrc instead of the
canonical ElasticGrid.rsrc; user reported missing effect. Live-image UUID PASS
was insufficient to establish effect registration. Failed package retained in
outputs/corner-click-dev136 and explicitly NOT an accepted candidate.
Corrected package follows tools/build_macos_sdkless.command resource/plist
layout, identical source/binary identity EGFX-32a69e569c44fdc85fe98a7f. Installer
refused same-ID/different-payload overwrite; its authenticated Restore retained
both versions, then corrected Install succeeded. Exact installed file hashes/
permissions PASS. Native app.effects reports exactly one FSTR Stretch with
matchName com.elasticgrid.fx.warp; preserved scene reopens without missing-effect
warning. Corrected evidence outputs/corner-click-dev136-fixed. Original scene
never overwritten after the missing-effect opening. Stationary press acceptance
requested again, still NOT RUN; original Mac hang remains OPEN/UNCONFIRMED.

Dev136 stationary/repeated press USER-REPORTED FAIL: first press works, later
presses fail and severe UI slowdown appears. No RAM Preview was launched by the
agent. Separate current scene copy retained in outputs/corner-repeat-dev140;
AE closed and authenticated Restore returned exact Dev132 bytes (manifest PASS).
Original Mac hang remains unconfirmed, not equated to this new regression.

Dev140 corrective scope: completed gesture-frame work cannot restart from DRAW
refreshes or host timestamp changes. Pending polls retain the original exact
owner/view/time/scale/stamp; one terminal receipt or error completes gesture work
before refresh. One fixed UI snapshot per gesture, lens sample position still
follows the corner; no output-renderer/quality/key changes. A new press after
release resets the request. Native supervision of the same active corner does
not reset it. Release is processed before arming the next hover, avoiding a
subsequent DRAW clearing the newly armed corner. Tests cover repeated timestamps,
terminal/error completion and release/repress of the same corner. Native click/
repeated-gesture/slowness acceptance remains NOT RUN; Dev132 kept installed
until candidate package validation. No main/merge/release authorization implied.

Dev140 source91a6872, BID EGFX-c835dacf0bb762dcc5ac460d:114 Rust tests,
strict all-target Clippy and24 focused Python contract checks PASS. Native Mac
bundle and branded bundle verifier PASS, signature/ZIP/manifest PASS. Installer
reported Installed; system file hashes/executable permissions match exactly.
Native AE registry count=1, matchName com.elasticgrid.fx.warp PASS; live loaded
UUID/path matches this installed build PASS. Preserved scene reopened. Repeated
stationary press/drag/Undo and absence of slowdown requested, still NOT RUN.
Windows CI37358711191 and Mac CI37358711209 PENDING at checkpoint. Local
evidence outputs/corner-repeat-dev140/verification.json. No RAM Preview launched,
original whole-Mac incident remains OPEN/UNCONFIRMED, no promotion.

Dev140 repeated stationary press USER-REPORTED FAIL: first press works, later
presses still require corner motion. Slowness acceptance not separately confirmed.
Dev144 regression reproduces the missing transition without AE: same pointer/
corner pressed/released three times, no extra hover/motion. Before fix FAIL log
outputs/corner-repress-dev144/before-fix.log. Root in this state model: the hover
anchor is consumed on start and clear() deletes it at native release. The former
Dev140 test only reset request state, never exercised the real hover-start path.

Dev144 shares hover-start between AdjustCursor and DRAW; preserves the scoped
owner/view/time/scale/pointer anchor on native start/release. Full lifecycle clear
and non-corner hover still invalidate it. Single-corner native movement updates
that active corner's pointer anchor so repress after movement works without a
new hover event. Existing pointer/scope guards reject other targets. The Dev140
one-frame-per-gesture budget, completion-before-refresh and error guards remain.
No timers/OS event injection/background requests/renderer/keys/schema changes.
116 Rust tests and20 Python contracts PASS; strict Clippy/build follow. Native
repeated-press/slowness acceptance NOT RUN, original Mac-hang incident OPEN.

Contract count correction: the combined first-application/host unittest run
records20 tests, not24; earlier Dev140 checkpoint overcounted this suite. Logs
remain retained; Rust test totals unaffected.

Dev144 sourcea4720c5, BID EGFX-27b9340f03d6e79f2fd1ec21:116 Rust tests,
strict Clippy and20 combined Python contract tests PASS. Regression FAIL before
fix, PASS after. Bundle/PiPL/entrypoints/signature/ZIP/manifest PASS. Native
installer reported Installed; system hashes/executable permissions PASS.
AE registry count=1 com.elasticgrid.fx.warp and loaded UUID/path PASS. Preserved
scene reopened. User requested three stationary presses and separate slowness
feedback, both still NOT RUN. Mac CI37359801465 and Windows37359801528 PENDING.
Local evidence outputs/corner-repress-dev144/verification.json. Original Mac
RAM Preview incident OPEN/UNCONFIRMED; no playback, promotion, merge or release.

Dev144 stationary repeated press USER-REPORTED FAIL unchanged. Prior source
state-machine regression PASS is not native acceptance or proof of root cause.
No further speculative behavior patch. Probe151 (gesture-probe) adds bounded
transition evidence: generic CLICK/DRAG/lifecycle, clear/release/start, hovered
corner scope predicates, AdjustCursor state, DRAW activation and frame success.
Start/release events are not deduplicated; repetitive state snapshots are.
Existing4096-record local-only limit retained; gesture-probe suppresses generic
DRAW/stage0-5 chatter and does not enable PREVIEW callbacks. No native pointer
coordinates, project paths, frames or host pointers in new records. Product
behavior/one-frame-per-gesture budget unchanged.118 diagnostic Rust tests PASS;
strict diagnostic Clippy/build follow. Human stationary press vs moved control
sequence needed to distinguish lost state from native host event delivery.

Probe151 sourcee5ddc05, BID EGFX-f909f058d6c78b4fe8902137:118 diagnostic
Rust tests, strict diagnostic Clippy, canonical bundle/PiPL/entrypoints/signature/
archive/manifest PASS. Native installer reported Installed; exact installed
payload files/signature PASS. Native AE registry count=1 with canonical matchName
PASS; separate preserved scene reopened. Surface mode selected through guarded
SDK scripting; Cua screenshot confirms grid/corner grips visible. Loaded-image
UUID NOT RUN. Journal-ready baseline has4 records; human two stationary held
presses plus a moved control requested, native trace NOT RUN. No speculative
behavior correction, RAM Preview or release action. Original whole-Mac incident
remains OPEN/UNCONFIRMED. Local evidence outputs/corner-press-probe151.

Probe151 human stationary/moved capture completed:134 records retained locally.
Repress start succeeds with scope guard63, but viewer DRAW is delayed841ms until
native corner movement; first activation DRAW follows5ms after start. SDK25.6
requires PF_InvalidateRect together with UPDATE_NOW; the new-press AdjustCursor
branch omitted invalidation. Dev152 queues one callback-owned view invalidation
at the accepted new-press transition; repeated active cursor callbacks do none.
Diagnostic155 records success without PREVIEW callbacks. Source guard FAIL
before repair retained. Native fix/slowdown acceptance NOT RUN; original RAM
Preview incident OPEN/UNCONFIRMED. No rendering/key/idle-request policy change.

Dev152/diagnostic155 source70f8cee, BID EGFX-302271519bf02304b962c5ac:
118 diagnostic Rust tests, strict Clippy and21 focused Python contracts PASS.
SDK call-placement guard FAIL before/PASS after. Canonical bundle/PiPL/export/
signature/archive/manifest and native installer package PASS. Release compile
completed with Rust objcopy debug-strip warnings (missing toolchain LLVM shared
library), retained build.log; no warning-free-build claim. Initial installer
builder argument used a directory instead of ZIP and failed before any install;
fresh installer-archive packaging from verified ZIP PASS. Candidate not installed:
Mac locked; unlock requested. Native repeated-press/slowdown NOT RUN, original
whole-Mac hang OPEN/UNCONFIRMED. Probe151 loaded UUID/path now PASS and134-record
private capture retained. Authorized branch push succeeded; Mac CI37362662279
in progress, Windows37362662281 queued on70f8cee. No main/merge/release.

2026-10-06 continuation: user unlocked Mac. Current different AE session saved
to a separate owned copy before quit; original files untouched. Diagnostic155
installed through native Install/Restore: Installed message PASS, canonical
FSTR FX/FSTR Stretch.plugin file hashes/executable permissions/signature PASS.
Initial inspection guessed the wrong MediaCore path without FSTR FX subfolder;
canonical-path check corrected and exact manifest verified. AE registry count1,
matchName and loaded UUID/path PASS; owned Surface validation scene open with
visible grips/Effect Controls. Three stationary presses plus drag/Undo/no-slowdown
requested; journal-ready4 records, native fix acceptance still NOT RUN.
Mac CI37362662279 PASS on70f8cee. Windows37362662281 initial failure is runner
allocation, annotation "The job was not acquired by Runner of type hosted even
after multiple attempts"; zero steps executed. One bounded job retry requested
through GitHub connector successfully; result pending. Original RAM Preview
whole-Mac incident remains OPEN/UNCONFIRMED. No playback or release action.

2026-10-06 diagnostic155 native stationary press USER-REPORTED PASS, drag
USER-REPORTED FAIL sluggish. Private613-record trace captured:6 native releases,
500 repeated start/supervision records; no repeated Effect-context gesture frame
preparation transition during drags. Diagnostic logger flushes every recorded
line synchronously. This is concrete diagnostic overhead, not proof of the user
latency cause. First discriminating step: ordinary Dev152, same source behavior
and invalidation fix, without gesture-probe/file logging. No speculative render
or pointer behavior change. Native drag/no-slowdown acceptance pending; original
whole-Mac RAM Preview incident OPEN/UNCONFIRMED.

Ordinary Dev152 source39616fa, BID EGFX-c3c62a10f10d4026fcab974d:
116 default-feature Rust tests/strict Clippy/release build PASS; no build warnings
(process-local toolchain LLVM lookup corrected, previous warnings retained).
Canonical bundle/PiPL/exports/signature/archive/manifest/native installer PASS.
Native Installed message, canonical installed files/executable permissions and
signature PASS. AE registry count1, loaded UUID/path PASS. Cargo compiler record
has only default/native-plane; diagnostic log prefix absent in binary. Separate
current scene copy saved before quit; reopened with selected Surface grid/grips.
Human drag/Undo/no-slowdown and stationary press recheck requested, NOT RUN.
User asked why installer appears twice: source InstallerApp.mm exits after
successful outcome OK; another start panel appeared after agent re-queried the
completed app through Cua. Do not query/reopen the installer after outcome/exit;
use disk manifest checks instead. No second installation was invoked. Native
frontend visual startup claim remains observation-specific. Windows runner retry
37362662281 attempt2 PASS on70f8cee; current39616fa differs only STATUS.md, so CI
evidence remains bound to70f8cee. Original RAM Preview incident OPEN/UNCONFIRMED.

2026-10-08 resource census auto-r3 (source f33452e, owned PID33270):
heavy 1920x1080 32bpc/Final baseline917910200; user drag captured Top Right
[1153,600]. User reports Undo returned picture. Exact original coordinate
Undo NOT_CONFIRMED: readback transport timed out and file contains intermediate
[1466,552], versus initial[1242,543]; final counters recorded four gesture API attempts
(not necessarily four user gestures). All four loupe checkins succeeded; one
8,294,400-byte copy retained,927 UI bitmap/image tokens created and dropped.
At456.335s guard growth cap exceeded: footprint1374369448, growth456459248
bytes >384MiB. Guard explicitly killed owned AE, not a spontaneous crash.
Preferences exact original hash and nine isolated components restored PASS.
Evidence outputs/resource-census-v2-oct08/native-auto-r3/manual-session-result.json.
No release readiness claim; original hang cause UNKNOWN; diagnostic remains
installed, ordinary Dev188 retained for restoration after investigation.
