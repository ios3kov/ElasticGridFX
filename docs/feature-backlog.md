# Future update feature backlog

Updated: 2026-10-03. These are future product requests, not implemented features
or blockers for the published 0.9.3-perf.1 release. No native code or parameter
contract changes are authorized by recording this list.

## Confirmed user requests

### Grid visible without selection

Latest user clarification on 2026-10-03 supersedes the older blanket
hide-during-preview requirement:

- During ordinary editing of the selected effect, show the interactive grid
  by default; hide it during ordinary preview playback when the optional
  persistent display is off.
- Provide an optional **Show Grid** switch, default off, for keeping the grid
  visible while working with other layers/effects, including preview display.
- Never include the grid in exported frames, render output or cached image
  pixels. Preview visibility must be a viewer-only overlay, not an image effect.
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

Status: REQUESTED / NOT IMPLEMENTED. No host tests or acceptance PASS recorded.

## Next update interface plan

User decisions recorded on 2026-10-03: all user-facing parameter, button,
section and option names must remain **English** and explain their purpose in
ordinary language. Russian explanations in the conversation are not UI labels.
Implementation has not started; the following are proposed English labels for
that approved interface direction, not shipped controls.

### Clear parameter names

User approved **Affected Lines** and **Follow Strength**. All visible names must
use simple English at approximately B1 level; the other labels below remain
proposals to refine rather than approved final wording.

| Existing label | Proposed English label | User-facing meaning |
|---|---|---|
| Tension Radius | Affected Lines | How far neighboring grid lines are affected; the value remains a range in grid steps, not an exact integer count. |
| Falloff | Influence Shape | How movement fades across the affected area. |
| Smoothstep | Smooth Fade | Smoothly reduce influence toward the range boundary. |
| Gaussian | Soft Center | Stronger influence near the moved line, weaker farther away. |
| Linear | Even Fade | Reduce influence at a constant rate with distance. |
| Smoothstep (Legacy) | Smooth Fade (Legacy) | Preserve the saved legacy option; not the default recommendation. |
| Elasticity Strength | Follow Strength | How strongly neighboring lines follow the moved line. |
| Stretch Easing | Stretch Smoothing | Soften changes in image stretch at cell boundaries. |
| Easing Distance | Smoothing Width | Width of the area used to smooth those transitions. |

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
- Stretch Smoothing and Smoothing Width must immediately update the visible
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

### Grid Positions starts collapsed

Grid Positions must be collapsed by default in Effect Controls. The user can
expand it for editing/animation. Preserve its stopwatch, existing animation and
parameter identity; verify supported host UI behavior before implementation.

### Automatic safe line spacing

Remove Min Line Spacing from the ordinary user-facing controls. Calculate a
safe spacing automatically so the user can compress the grid as much as possible
without crossing neighboring lines or creating degenerate cells.

The automatic policy and behavior at different densities, image sizes and depths
must be designed and tested. Preserve the stored parameter ID and existing values
for project compatibility; do not silently change old project output while hiding
this control. The final legacy/new-instance policy is a technical-design item.

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
  active copies and unexplained destructive replacement. No installer code or
  installation action is authorized merely by recording this requirement.
- Broader AE-version support remains a discussed candidate, not a commitment.
- GPU/Metal acceleration: historical plans exist; no new cycle is approved.
- The accepted macOS CPU performance cycle is complete; do not label it unfinished.

## Active update task and check mapping — rules8.0.0

Branch: feat/next-update. Parent work is retained: backlog decisions plus the
Windows port checkpoint003607d have been combined locally without altering main
or the original Windows branch. The accepted macOS source f611312 and installed
release remain the historical comparison baseline, not a PASS for new bytes.
New update version is not yet assigned; packaging/host target scope is AE25.x
macOS arm64 / Windows x64 until verified otherwise.

| ID | Requirement / block | Acceptance / check phase | Current state |
|---|---|---|---|
| U0 | Adopt frozen rules8.0.0 and preserve prior work | Verified release ZIP/tag identity; explicit migration; retained decision ledger | Adoption recorded |
| U1 | Repair Windows roundtrip validation evidence | Regression rejects invalid files and missing/failed per-run completion; no AE transport false PASS | Implemented; 15 Python + 13 JSX mock cases PASS; Windows AE NOT RUN |
| U2 | Current-time Grid Positions reset | Core neutral-state tests; native build; AE exact-time key preservation and Undo/Redo validation | Pending |
| U3 | Simple English labels and collapsed Grid Positions | Preserve IDs/option values; native builds; AE panel inspection/old-project load | Pending |
| U4 | Collapsible Wave Animation section | Group only changes UI; waves/keys/saved values and playback match; build + AE checks | Pending |
| U5 | Live range/strength feedback and affected-range drawing | Editable deformation model defined first; unit numeric/serialization tests; real AE slider/drag/Undo checks | Design dependency open |
| U6 | Automatic spacing | Legacy/new-instance policy defined first; no crossings/degeneracy; old-project output retained | Design dependency open |
| U7 | Viewer-only grid with Show Grid off by default | Public API/overlay feasibility before implementation; selected/unselected/playback/camera/multiple-instance lifecycle; export/cache never includes guides | Feasibility open |
| U8 | Safe Mac/Windows installation and rollback | Dry-run/path/identity/backup tests; exact candidate install, load and rollback on each target platform | Pending design and helpers |
| U9 | Windows runtime closure and dependency instructions | Corrected exact candidate identity; x64 dependencies; load/UI/pixel/roundtrip/first-application/MFR/aerender host packet | NOT RUN; Windows host unavailable here |
| U10 | Integration, validation and final task reconciliation | All U1–U9 requirements/checks accounted for with exact candidate Evidence; docs/manifest/CI; cleanup inventory; no old PASS transfer | Pending |

The selected-effect PF event surface alone does not provide a confirmed drawing
surface after deselection or during playback. U7 must not be faked by rendering
the grid into pixels, adding project layers, retaining a DRAWBOT context outside
its callback, or silently replacing the composition renderer. Continue independent
blocks while researching a supported solution. Existing Grid Positions keys must
not be silently rewritten to implement U5/U6.

Each block updates this table and STATUS with actual checks and limitations.
Native source changes require fresh builds/identity; runtime, performance,
migration and installation checks are not inferred from old release/CI results.
No significant block is complete merely because its code compiles.

Platform scope confirmed by the user on 2026-10-03: this update targets both
macOS Apple Silicon and Windows x64 together. Shared product behavior must match;
platform builds, installation and host evidence are tracked separately. No
macOS-only release substitutes for completion of the agreed two-platform scope.
