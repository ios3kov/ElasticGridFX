# Current status

Updated: 2026-10-03. This is the concise entry point for the current product and
continuation. [Dated checkpoints](current-status.md) retain historical detail;
their older holds and release-policy statements do not override the state below.

## Active local update — rules8.0.0

Branch feat/next-update; no push, PR or publication authorized. Current update
remains incomplete across macOS/Windows. Source version is 0.9.4 Dev 1; installation of these latest changes is pending.
Local source checks: 73 Rust tests, 266 Python tests, 16 JavaScript safety files
and strict Clippy PASS. These are not After Effects or Windows runtime acceptance.
See the U0–U11 table in
[feature backlog](feature-backlog.md). Installed candidates and checks below
are development evidence, separate from the published release.

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
