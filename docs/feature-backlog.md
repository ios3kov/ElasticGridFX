# Future update feature backlog

Updated: 2026-10-03. These are future product requests, not implemented features
or blockers for the published 0.9.3-perf.1 release. No native code or parameter
contract changes are authorized by recording this list.

## Confirmed user requests

### Grid visible without selection

Show the transformed guide grid when the effect/layer is deselected, with an
opt-in switch. Hide it during RAM Preview playback; never include it in rendered
or cached pixels or intercept pointer events. Preserve the existing interactive
selected-effect overlay. Feasibility and detailed acceptance remain in
[persistent viewer grid](persistent-viewer-grid.md).

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

| Existing label | Proposed English label | User-facing meaning |
|---|---|---|
| Tension Radius | Influence Range | How far neighboring grid lines are affected. |
| Falloff | Influence Shape | How movement fades across the affected area. |
| Smoothstep | Smooth Fade | Smoothly reduce influence toward the range boundary. |
| Gaussian | Soft Center | Stronger influence near the moved line, weaker farther away. |
| Linear | Even Fade | Reduce influence at a constant rate with distance. |
| Smoothstep (Legacy) | Smooth Fade (Legacy) | Preserve the saved legacy option; not the default recommendation. |
| Elasticity Strength | Neighbor Movement | How strongly neighboring lines follow the moved line. |
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
- Influence Range and Neighbor Movement must provide immediate, meaningful
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

Status: PLANNED / NOT IMPLEMENTED. This records product requirements and pending
technical-design work, not runtime feasibility, completed code or test PASS.

## Other continuation and candidates

- Windows x64 CPU port: build/static checks passed at the checkpoint linked in
  [current status](STATUS.md#windows-continuation); Windows AE runtime remains
  NOT RUN. Separate portability work, not a new deformation feature.
- macOS pkg installer and broader AE-version support: discussed candidates,
  not implementation commitments.
- GPU/Metal acceleration: historical plans exist; no new cycle is approved.
- The accepted macOS CPU performance cycle is complete; do not label it unfinished.
