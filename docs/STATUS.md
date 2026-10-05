# Current status

Updated: 2026-10-05. This is the concise entry point for the current product and
continuation. [Dated checkpoints](current-status.md) retain historical detail;
their older holds and release-policy statements do not override the state below.

## Active update — Mac and Windows, rules8.0.0

Current scope: implement the newly authorized four deformation modes and disabled
Demo design for both platforms; retain native Install/Restore delivery obligations.
Added centered corner loupe with a central target in Flat/Perspective, UI only,
Mac and Windows; see [loupe contract](corner-loupe-plan.md). Native loupe
acceptance is pending.
See [mode/design contract](four-modes-demo-plan.md). Previously accepted installer
payloads remain unchanged until a new plugin candidate is separately accepted. Branch feat/next-update is published for the
user-authorized Windows CI continuation; source fixes/docs on that branch are
allowed. No main, PR, merge or release action is authorized by this continuation.
The earlier local-only and Windows-deferred decisions were superseded by the
user's later Windows implementation and specific branch/CI authorization.

Installed now: **0.9.4 Dev64**, source000ff91aeb91249fa94f54d7e7b73890ee965f9a,
Build ID EGFX-38c123bf765bbee9c1bc241a. Package/signature and exact loaded-image
identity PASS; centered corner loupe implemented in shared host code. 92 Rust
and Clippy all-targets PASS, 22 scoped Python checks PASS. New loupe native
visual drag/release/Undo acceptance NOT RUN; user check requested in owned scene.
Windows Dev64 CI37291232120 PASS; Build ID EGFX-448748c343eede5cd2db801a,
AEX SHA256 b2c145401ef8ac3b5b656a07c33e2c1aadfa8b5c1d0fd2195938f7568d860bf2.
Downloaded packet checksum/embedded identity PASS; Windows AE acceptance of
this candidate NOT RUN. Mac source CI37291232084 PASS.
Evidence: outputs/corner-loupe and outputs/FSTR-Stretch-0.9.4-Windows-Dev64.
Dev60 earlier scoped four-mode pixel/migration evidence stays artifact-specific.
Installer payloads still pin separately accepted Dev52/Dev56. Demo remains OFF.
Native activation is still blocked on marketplace SDK access. No release/merge.

Dev68 correction source8d98424a0aaf95df3147e13d6dced56ab9ce11b6: native AE Point
gestures now start the same lens; 93 Rust tests, Clippy and 15 host-contract
checks PASS. Mac release/package/signature PASS; not yet installed. Windows
CI37293880927 PASS; downloaded Dev68 AEX identity/checksum PASS
(EGFX-6916efe6320bedc00b79c89c). Windows packet:
outputs/FSTR-Stretch-0.9.4-Windows-Dev68/FSTR-Stretch-Windows-Dev68.zip.
Mac CI37293880981 in progress. Native acceptance NOT RUN.
AE now contains a different unsaved user project; fixture guard refused without
changing it. Concurrent-use confirmation requested before restart/replacement.
Evidence: outputs/corner-loupe-dev68. Dev64 remains installed.

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
