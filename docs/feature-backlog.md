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
- The request does not yet define reset behavior for animated Grid Positions
  (current time versus all keys). Resolve that product detail before implementation;
  recording the feature does not authorize deleting animation keys.
- Resetting Grid Positions does not imply disabling independently configured
  wave animation, elasticity or plane/corner transforms.

Status: REQUESTED / NOT IMPLEMENTED. No host tests or acceptance PASS recorded.

## Other continuation and candidates

- Windows x64 CPU port: build/static checks passed at the checkpoint linked in
  [current status](STATUS.md#windows-continuation); Windows AE runtime remains
  NOT RUN. Separate portability work, not a new deformation feature.
- macOS pkg installer and broader AE-version support: discussed candidates,
  not implementation commitments.
- GPU/Metal acceleration: historical plans exist; no new cycle is approved.
- The accepted macOS CPU performance cycle is complete; do not label it unfinished.
