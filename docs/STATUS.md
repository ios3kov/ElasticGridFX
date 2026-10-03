# Current status

Updated: 2026-10-03. This is the concise entry point for the current product and
continuation. [Dated checkpoints](current-status.md) retain historical detail;
their older holds and release-policy statements do not override the state below.

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
