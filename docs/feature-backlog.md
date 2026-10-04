# Future update feature backlog

Updated: 2026-10-04. The user authorized implementing this update locally.
Finish Mac first; Windows resumes after Mac completion. Individual items remain
incomplete until their scoped evidence below is closed. This work does not reopen the published 0.9.3-perf.1 release.

## Confirmed user requests

### Grid visible without selection

Latest user clarification on 2026-10-03 supersedes the older blanket
hide-during-preview requirement:

- During ordinary editing of the selected effect, show the interactive grid
  by default; hide it during ordinary preview playback when the optional
  persistent display is off.
- Provide an optional **Show Grid** switch, default off, for keeping the grid
  visible while working with other layers/effects, including preview display.
- Superseding user decision2026-10-04: when **Show Grid** is on, the grid may
  appear in exported/rendered/cached pixels. Implement shared pixel visualization.
  Defaultoff leaves ordinary output unchanged.
- A passive persistent overlay must not intercept pointer events or replace
  the selected-effect interaction.

Latest explicit answer (2026-10-03, repeated question): **Show Grid remains
visible during RAM Preview playback**, including when another layer/effect is
selected. This replaces the earlier answer to hide during playback.
The earlier feasibility research is in
[persistent viewer grid](persistent-viewer-grid.md); its original PV-3 statement
is historical where it conflicts with this later user clarification.

### Reset Grid Positions only

User request recorded on 2026-10-03: provide a button to reset only the
**Grid Positions** parameter, returning its grid deformation/offsets to the
neutral geometry for the current topology.

- Preserve all other effect settings, including Columns/Rows, guide density,
  Layer Plane/Four Corners mode, corner controls, elasticity, wave controls,
  edge behavior and render quality.
- This is a targeted Grid Positions reset, not the effect's global Reset.
- Support ordinary Undo/Redo; preserve parameter IDs and saved-state compatibility.
- For animated Grid Positions, reset only at the current composition time:
  replace an existing key at that time or add a neutral key if none exists.
  Preserve all keys at other times and their interpolation settings. Do not
  clear the animation or reset the entire timeline. The new/current key may
  affect interpolation in adjacent intervals under ordinary AE semantics.
- For unanimated Grid Positions, reset its static value without automatically
  enabling animation. All other parameters remain unchanged.
- Resetting Grid Positions does not imply disabling independently configured
  wave animation, elasticity or plane/corner transforms.

Status: IMPLEMENTED locally in U2; see native Reset checkpoints in STATUS.md.
Final integrated Mac candidate acceptance remains open; Windows acceptance is deferred.

## Next update interface plan

User decisions recorded on 2026-10-03: all user-facing parameter, button,
section and option names must remain **English** and explain their purpose in
ordinary language. Russian explanations in the conversation are not UI labels.
Implementation is in the local update branch; labels and checks below are
development state, not a released interface.

### Simplified deformation controls — superseding decision, 2026-10-03

The user removed Follow Shape, Follow Strength, Smooth Stretch and Smooth Width
from the visible interface. For new effects use one Smoothstep influence profile,
100% follow strength, 100% boundary smoothing and 25% smoothing width. These are
engineering starting defaults, not a visually certified universal optimum.
Keep their existing IDs, types, ordinal mappings, values and animation streams
hidden for old-project compatibility. Never overwrite existing project settings.
Affected Lines remains editable. The older naming table and live-feedback requests
below are historical for the four hidden controls; they no longer require exposed
sliders or a profile menu. Both platform builds share this policy.

Source implementation and local checks are separate from AE acceptance. Verify
new-instance defaults, old-project pixel/key parity and panel absence in AE before
calling this complete. Local checks: 73 Rust tests and 13 host source-contract
tests PASS; whitespace check PASS. These do not certify host visuals or Windows
runtime. No current installation claim follows from source changes.

### Clear parameter names (historical proposals)

User approved **Affected Lines** and **Follow Strength**. All visible names must
use simple English at approximately B1 level; the other labels below remain
proposals to refine rather than approved final wording.

| Existing label | Proposed English label | User-facing meaning |
|---|---|---|
| Tension Radius | Affected Lines | How far neighboring grid lines are affected; the value remains a range in grid steps, not an exact integer count. |
| Falloff | Follow Shape | How movement fades across the affected area. |
| Smoothstep | Smooth | Smoothly reduce influence toward the range boundary. |
| Gaussian | Soft | Stronger influence near the moved line, weaker farther away. |
| Linear | Even | Reduce influence at a constant rate with distance. |
| Smoothstep (Legacy) | Old Smooth | Preserve the saved legacy option; not the default recommendation. |
| Elasticity Strength | Follow Strength | How strongly neighboring lines follow the moved line. |
| Stretch Easing | Smooth Stretch | Soften changes in image stretch at cell boundaries. |
| Easing Distance | Smooth Width | Width of the area used to smooth those transitions. |

Label changes must preserve parameter IDs, saved numeric option mappings and
existing animation/state formats. Do not remove a legacy numeric option or
reorder its meaning just to simplify its visible name.

### Live viewport feedback for every editable control

- Update the rendered image and applicable guide display continuously while
  the user changes a value, drags a slider, selects an option or moves a handle.
  Do not require releasing the slider, clicking Apply, moving another line or
  changing time to see the result.
- Affected Lines and Follow Strength must provide immediate, meaningful
  viewport feedback, rather than only altering a later drag. Visualize the
  affected range while adjusting it and interacting with the grid.
- Smooth Stretch and Smooth Width must immediately update the visible
  deformation where those controls have an effect.
- Preserve legitimate dependencies: width at zero smoothing, and wave settings
  at zero wave amount, need not manufacture a visible deformation. Explain or
  disable inactive controls clearly; changing them must still refresh applicable UI.
- Live feedback must retain Final quality, ordinary cancellation, bounded memory,
  Undo/Redo and saved-project/animation compatibility. It does not authorize
  lower-resolution or lower-quality rendering presented as Final.

Design dependency: current influence settings govern drag operations; the stored
Grid Positions contains the resulting positions. Simply requesting a redraw
cannot retroactively reevaluate that deformation. Before implementation, define
and verify the editable deformation representation and its compatibility path
so range/neighbor changes can respond live without silently rewriting existing
Grid Positions keys, deleting animation or changing historical projects.

### Grid Positions has no disclosure arrow

Latest user correction on 2026-10-03 supersedes the collapsed-by-default design:
Grid Positions has no disclosure arrow or empty expanded area. Put a **Reset**
button to its right on the same row, with no “Now” suffix and no separate reset
row. Preserve the native stopwatch, animation and parameter identity. The reset
still affects only the current time; verify the topic-row UI in both hosts.

### Equal popup widths

User correction on2026-10-03: all popup controls use one common width, chosen to
fit the longest option without clipping. Includes the controls inside Wave
Animation. Keep the native value-column alignment and preserve option values,
parameter IDs, keyboard interaction and saved projects. SDK25.6 standard popup
width is host-owned; ui_width is for PF_PUI_CONTROL custom areas. First retain
native controls with short readable options: Old Smooth; Preview/Final;
Both/Columns/Rows. Values/formulas stay unchanged and no dummy/padded options
are added. Verify that all controls use the native130-unit minimum on the
actual Mac and Windows panels; source captions alone do not prove equal widths.
If a host still chooses different widths, custom-row/menu feasibility remains
required. No fixed native-width setter or cross-platform acceptance is claimed.

### Automatic safe line spacing

Remove Min Line Spacing from the ordinary user-facing controls. Calculate a
safe spacing automatically so the user can compress the grid as much as possible
without crossing neighboring lines or creating degenerate cells.

The automatic policy and behavior at different densities, image sizes and depths
must be designed and tested. Preserve the stored parameter ID and existing values
for project compatibility; do not silently change old project output while hiding
this control. Implementation policy: an appended hidden static flag defaults to automatic for
new instances and uses USE_VALUE_FOR_OLD_PROJECTS=false for older projects.
Automatic requests zero spacing, invoking the existing core numerical floor
(1e-6 normalized units) and density cap. The retained hidden Min Line Spacing
stream supplies unchanged saved values/keys for legacy projects. No key rewrite.
New-instance default and static flag verified in AE25.6; old-project migration remains to be verified.

### Collapsible Wave Animation section

Place all visible wave controls in one collapsible **Wave Animation** section:
Wave Amplitude, Wave Frequency, Wave Phase, Wave Speed and Wave Axis. Keep their
existing IDs, values, keyframes and option mappings. Do not expose a second
nonfunctional wave-enable control; zero amplitude retains its current meaning.
The section organizes controls; collapsing it must not disable the animation.

### Planned acceptance

1. All new user-facing labels are clear English; no technical formula names are
   needed for choosing the ordinary influence profile.
2. Every applicable control gives live viewport feedback during editing.
3. Automatic spacing permits strong compression without crossed/degenerate cells.
4. Wave controls appear together in a collapsible Wave Animation section.
5. Grid Positions reset still affects only the current time as specified above.
6. Undo/Redo, existing keyframes/projects, parameter IDs and Final-quality output
   retain their contracts; verify on both platform scopes when available.

Status: DEVELOPMENT STARTED under rules8.0.0. This records product requirements and pending
technical-design work, not runtime feasibility, completed code or test PASS.

## Other continuation and candidates

- Windows x64 CPU port: build/static checks passed at the checkpoint linked in
  [current status](STATUS.md#windows-continuation); Windows AE runtime remains
  NOT RUN. Separate portability work, not a new deformation feature.
- Simple installation/update on both macOS and Windows is now required by the
  user. Evaluate a pkg/installer versus an executable command or guided helper;
  format is not predetermined. Plan clear instructions, detection of an existing
  plugin, preservation of the previous version outside active Adobe plugin
  folders, a discoverable backup location and a simple rollback. Avoid duplicate
  active copies and unexplained destructive replacement. The later explicit
  instruction to implement the whole update authorizes local installer development;
  this does not authorize publication or broaden system access.
- Broader AE-version support remains a discussed candidate, not a commitment.
- GPU/Metal acceleration: historical plans exist; no new cycle is approved.
- The accepted macOS CPU performance cycle is complete; do not label it unfinished.

## Active update task and check mapping — rules8.0.0

Branch: feat/next-update. Parent work is retained: backlog decisions plus the
Windows port checkpoint003607d have been combined locally without altering main
or the original Windows branch. The accepted macOS source f611312 and installed
release remain the historical comparison baseline, not a PASS for new bytes.
Update version is 0.9.4 (current ordinary candidate Dev16); packaging/host target scope is AE25.x
macOS arm64 / Windows x64 until verified otherwise.

| ID | Requirement / block | Acceptance / check phase | Current state |
|---|---|---|---|
| U0 | Adopt frozen rules8.0.0 and preserve prior work | Verified release ZIP/tag identity; explicit migration; retained decision ledger | Adoption recorded |
| U1 | Repair Windows roundtrip validation evidence | Regression rejects invalid files and missing/failed per-run completion; no AE transport false PASS | Implemented; 15 Python + 13 JSX mock cases PASS; Windows AE NOT RUN |
| U2 | Current-time Grid Positions reset | Core neutral-state tests; native build; AE exact-time key preservation and Undo/Redo validation | Core PASS; Mac diagnostic8a2e30c native current-time key preservation + static/animated Undo/Redo PASS; ordinary fb9ae4f reopen/current-time Reset/Undo8 checks PASS; Windows pending |
| U3 | Simple English labels, single-row Grid Positions, equal popup widths | Preserve IDs/option values; native builds; AE panel inspection/old-project load; common popup width without clipping | Mac8a2e30c visible popups equal130 units, longest captions fully fit; inline Reset matches Fit Layer; ordinary fb9ae4f four visible popups PASS; 6314811 native wide/narrow Reset visibility and matching Fit Layer dimensions PASS; Dev44 expanded Wave Axis width/captions PASS; Windows pending |
| U4 | Collapsible Wave Animation section | Group only changes UI; waves/keys/saved values and playback match; build + AE checks | Implemented; shared Rust tests PASS; Mac collapsed and Dev44 expanded five-control section inspected; Dev44 native full Wave/chain pixel smoke PASS; Windows AE NOT RUN |
| U5 | Live range feedback and affected-range drawing | Editable deformation model defined first; unit numeric/serialization tests; real AE slider/drag/Undo checks | Immediate saved-field response implemented after user permits changed old appearance; retained keys/wire unchanged; numeric fractional/eased/wave drag PASS; Mac Dev8 old Radius keys, native drag/Undo/Redo, scripted response/save-reopen PASS; Dev12 scripted range-boundary response, annotation/export separation, drag/Undo/Redo/save-reopen PASS in bounded Mac fixture; Dev16 first-frame hidden-default regression fixed and native reverse drag/Undo/Redo/saved-copy reopen PASS; mouse scalar scrubbing USER-REPORTED PASS; Dev44 lifecycle plus immediate saved-key change/Undo PASS in scoped Mac fixtures |
| U6 | Automatic spacing | Legacy/new-instance policy defined first; no crossings/degeneracy; old-project output retained | Implemented; 67 Rust tests PASS and Mac new-instance default verified; Mac Dev44 ordinary0.9.3 missing-stream/animated-spacing migration and save/reopen PASS in bounded neutral fixture; Windows deferred |
| U7 | Optional rendered Show Grid, defaultoff | Superseding pixel-output contract; selected/unselected/playback/plane/layout lifecycle; defaultoff exact output; on appears in exports;8/16/32-bit alpha/origin/cancel | Shared CPU source implemented;84Rust/272Python + strict Clippy PASS; exact Dev20 Mac scoped default/switch/export/reopen/key/depth/plane/quality/Wave/deselected/playback/other-layer PASS; Dev44 ordinary Undo/Redo, deletion, save/reopen (including canceled binding), scoped camera/parent and60-frame MFR requested ON/OFF exact pixels PASS; Windows NOT_RUN |
| U8 | Safe Mac/Windows installation and rollback | Dry-run/path/identity/backup tests; exact candidate install, load and rollback on each target platform | Mac plugin checks precede resumed native installer work; protected snapshot receipts and exclusive fresh publication implemented, five native test targets PASS; frontend/package acceptance pending; Windows deferred |
| U9 | Windows runtime closure and dependency instructions | Corrected exact candidate identity; x64 dependencies; load/UI/pixel/roundtrip/first-application/MFR/aerender host packet | NOT RUN; Windows host unavailable here |
| U10 | Integration, validation and final task reconciliation | All U1–U9 requirements/checks accounted for with exact candidate Evidence; docs/manifest/CI; cleanup inventory; no old PASS transfer | Pending |

The user removed U7 export/cache exclusion on2026-10-04: optional rendered grid
pixels are now authorized. The old additive-only research is historical for its
original contract. No project layers, retained DRAWBOT contexts or composition
renderer replacement are needed. Existing Grid Positions keys must not be
silently rewritten to implement U5/U6/U7.

Each block updates this table and STATUS with actual checks and limitations.
Native source changes require fresh builds/identity; runtime, performance,
migration and installation checks are not inferred from old release/CI results.
No significant block is complete merely because its code compiles.

Platform sequencing superseded by the user on 2026-10-04: finish macOS Apple
Silicon first, then return to Windows x64. Shared feature requirements remain;
Windows build, host and installer checks are deferred and keep NOT RUN status.
They do not block the separately accepted Mac delivery. Local-only authorization
continues: no push, PR, remote CI, main change or release publication.

### Static choice controls — latest user correction, 2026-10-03

Deformation Plane (mode), Falloff (profile type), Edge Behavior and Render Quality
must not expose a
stopwatch or allow new animation. Apply to both Mac and Windows. Retain parameter
IDs/types and numeric option mappings; do not delete keys in code. Opening old
projects that already animate these choices is an explicit compatibility check:
AE handling of the new CANNOT_TIME_VARY flags is not assumed to preserve old
output until verified. U3/U10 include this check before acceptance/release.


### Density redistribution and version display — 0.9.4 Dev 1

Changing Columns/Rows redistributes visible guides uniformly across the current
deformed plane, preserving the image deformation and all Grid Positions keys.
An appended hidden, static ControlLayout stream stores bounded source references;
the render lattice is unchanged. Later dragging uses the saved source references.
Legacy layouts remain readable; source-reference reflow is triggered only by an
explicit count change. AE callback/Undo/save/reopen acceptance remains pending.

A read-only Version row displays 0.9.4 Dev 1 (Dev 2 for diagnostics). Package,
PiPL and About version derive from the same Cargo version.

| U11 | Uniform density without deformation change | Real FFI tests for all counts, immutable grid/key bytes, saved layout roundtrip; native AE count-change, Undo and reopen | Source implemented; Dev44 Mac animated7/6→11/9, Undo/Redo and save/reopen exact pixels/key metadata PASS; prior uniform guide/drag checks retained; Windows deferred |


### Scripted density checkpoint — 2026-10-03

AE scripted count setters do not send UserChangedParam in the owned Mac fixture.
A pure read-only fallback redistributes guides when visible count differs from
retained topology; the first actual deformation drag freezes the references.
No Grid Positions setter is invoked merely by drawing or changing counts.
74 Rust tests PASS, including callback-free redistribution with immutable saved
bytes. Fresh candidate native acceptance follows separately.

Initial 0.9.4 candidate 6d67c27 / EGFX-981a1e675022a0a6abb7d6d4 loaded identity
PASS on Mac AE25.6. New hidden defaults are 1/100/100/25; legacy fixture remains
2/100/0/25 with its three keys. Four legacy PNG frames match prior candidate
exactly. Version row and hidden controls visually confirmed. One native
Unsupported effect control warning appeared on initial open; cause remains
UNKNOWN, no no-warning/complete integration PASS is claimed. The first density
evidence script used unavailable JSON.stringify; corrected script has separate
outputs. These harness errors do not overwrite prior evidence.


### Hidden arbitrary registration correction

SDK25.6 AE_Effect.h:400 requires arbitrary data to combine TOPIC/CONTROL or
NO_ECW_UI; lines2313–2321 describe hidden-data edits from custom viewer UI or
a supervised UserChangedParam. The new ControlLayout initially used INVISIBLE
alone, producing Unsupported effect control during open/save. Add NO_ECW_UI
alongside INVISIBLE. VersionRow already uses TOPIC. The regression source guard
checks this prerequisite; native reopen/save acceptance remains required.

During keyboard quitting the owned fixture was unexpectedly saved at17:23; its
current hash is cc66fb173ecc37303f96864deaab70739b0559b65852a6f46b88b021a11a0ef8.
The original fixture hash is no longer claimed immutable in this continuation.
User projects were not involved. Historical frames/results retain their original
identities; further work uses a new fixture path and explicit DO_NOT_SAVE_CHANGES
for project close. No previous PASS is transferred to this new fixture.


### Mac checkpoint db97762 — 0.9.4 Dev 1

Build EGFX-883ae0b0eae4536bb555bc3d, native-plane ordinary release-profile test
build. Bundle integrity and loaded UUID/path identity PASS. Four old-project PNG
frames match the previous candidate exactly. Density7/6 changes neither pixels
nor the three Grid Positions keys at0/.5/1. Save/reopen of a new AEP retains
counts, keys and exact current-time pixels. Required hidden-arbitrary registration
now prevents the earlier Unsupported effect control warning during this bounded
open/save/reopen cycle. UI shows Version0.9.4 Dev1 and hides the four controls.

Fresh fixture: outputs/update-094-density-final-mac/density-fixture.aep; its saved
roundtrip copy is outputs/update-094-registration-mac/density-saved.aep. The
initial original fixture was accidentally saved during keyboard quitting; no
immutable-original claim extends across that event. Subsequent project close
uses the explicit owned-project DO_NOT_SAVE_CHANGES API.

After Mac unlock, actual guide dragging at density7/6 changes current pixels
and adds only the .2 key; the 0/.5/1 frames remain exact. The first capture
scenario failed Redo and is retained in drag-undo-redo-result.json. Repeated
validation without assigning comp.time during capture passes Undo/Redo: three
original keys and exact baseline pixels after Undo, four keys and exact dragged
pixels after Redo. Precise causality of the earlier harness interference is not
claimed. Evidence: read-only-undo-redo-result.json. Saving a fresh gesture-saved.aep
and reopening retains counts7/6, four keys and exact current pixels
(gesture-roundtrip-result.json PASS). Initial no-change/refused attempts remain
historical input-delivery evidence. U5/U7/U8/U9/U10 and Windows remain incomplete.

Local source gates: 74 Rust tests PASS; strict default-feature Clippy PASS;
14 host-contract tests PASS. The earlier full Python266 and JavaScript16-file
gates cover the initial0.9.4 implementation; later changes were Rust/native UI
registration and the added host-contract guard. Diagnostics-feature suite78 PASS; full Python rerun remains separate. All work stays local.


### Old-deformation live response — confirmed 2026-10-03

The user rejected new-moves-only Affected Lines. U5 must also respond for existing
saved deformation; preserving exact initial appearance/keys remains required.
The stored baked axes cannot reconstruct gesture history. Investigate geometric
field response and validate its observable behavior before saved-format changes.
See live-influence-design.md. The old pending legacy question is superseded.


### U8 shared installation decision contract

User-facing direction: Mac native installer and Windows native installer with
Install/Restore actions, clear running-host/conflict messages, previous-version
backup outside Adobe scan directories and no Developer ID requirement. Format
integration/native frontends are pending; no final pkg/exe is available yet.
Read-only tools/installer_contract.py shares the policy for both targets:
verified clean ordinary candidate matching expected version/Build ID/architecture;
fresh complete scan and stopped Adobe hosts; guarded destination/backup paths;
refuse unknown/multiple/changed copies and pending recovery; distinguish exclusive
fresh install, identical already-installed bytes, and backup-before-replacement.
Four unit scenarios include both platforms and negative/tamper/uncertain cases.
This is a decision contract, not a secure executor, installer or OS runtime PASS.
Native Mac exchange and immutable prepared-journal primitives now have disposable
filesystem tests (see STATUS.md); coordinator/recovery, frontend and Windows
implementation remain incomplete. No ready pkg/exe or installation PASS is claimed.

Native integration MUST collect observations itself, bind a full payload snapshot
(bytes/modes/layout) and process/scan provenance, then recheck under a transaction
lock immediately before publication. Package paths never select destinations.
On Mac resolve target under system root / and backups under /Library; on Windows
resolve target under native CommonProgramFiles and backups under ProgramData.
Require same-volume atomic publication/replacement; no copy/delete fallback for
unsupported atomic operations. Journal before mutation and retain recoverable
payload/previous tree after interruption. Restore verifies current/previous
identities and never overwrites a newer changed installation. Test interrupted
states, duplicates, symlinks/reparse paths, permissions and recovery before
privileged installation. Hashes alone do not authenticate the publisher; package
trust comes from the agreed distribution channel. No download/update service,
credentials or new UI-control permissions are introduced by this contract.

U8 next native block: an internal Mac replacement coordinator acquires a
nonblocking destination-scoped lock, calls the frontend's trusted environment
and full-payload verifier under that lock, durably prepares the journal,
rechecks before exchange and verifies the swapped layout afterward. Recovery
requires the original expected identities as well as a matching journal; it
never trusts journal-supplied destinations or automatically restores unknown
bytes. Inspect is read-only; Restore is a separate explicit operation. Every
post-journal failure retains the record and both trees. Fault/refusal tests must
prove no replacement after verification failure or lock contention and no
restore over a changed current payload. This is an internal coordinator, not
the final frontend/collector or a claim of full privileged installation safety.
The native snapshot-v1 collector now covers regular bundle bytes, sorted names,
mode/owner/group/flags and extended attributes with bounded nofollow traversal.
Extended ACLs, links and unsupported object types are refused. Native fixture
and read-only installed-bundle checks are recorded in STATUS.md. Coordinator
tests use authentic pre-mutation snapshots; durable metadata and distribution
authentication are still frontend obligations, not established by a SHA alone.


### Mac-first completion checkpoint — 2026-10-04

[Scoped closure evidence](update-094-mac-closure-checkpoint-2026-10-04.json) closes
the observed Mac spacing migration, saved-field range response and animated
density lifecycle scenarios on exact Dev44. It does not transfer those results
to a newly built final artifact. Next U8 block: trusted native fixed-root/host/
duplicate collector and staging, then user-facing Install/Restore and a real
package cycle. The protected snapshots-v1 receipt and exclusive fresh-publication
primitive preserve both trees and refuse unknown recovery. Their native fixture
checks are not the final installer frontend or privileged installation proof.
