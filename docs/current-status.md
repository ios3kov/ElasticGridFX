# Current development / release status

## Development: Fit Layer boundary correction — 2026-09-30

Stage 10, branch `fix/fit-layer-coordinates`, stacked on PR #12. The source
corrects Fit to (0,0)/(W,0)/(W,H)/(0,H), with the existing corner order and full
source-layer dimensions. It skips writes to already-fitted corners, including
repeat Fit. It does not write Grid Positions/counts or migrate stored keys.
Rendering, layer/comp projections and the density implementation are unchanged.
The extra Layer Plane overlay/index-span audit still requires native AE
alignment evidence; a global change to raster coordinates was not made.

Version 0.9.3 development; not released. The source also derives Finder version
from validated BuildIdentity before signing (#13), and the native verifier
checks both version fields on the named and extracted bundle. This is not an
in-place edit of a prior candidate. Exact final CI/artifact evidence is in the
correction PR; live target AE is NOT RUN in this Linux session.

The user reported the density candidate 59a73ea working, then supplied the Fit
coordinate mismatch. That historical density acceptance is not a new-candidate
host PASS. See [coordinate acceptance](fit-layer-coordinate-fix-2026-09-30.md).

## Historical density checkpoint

## Development: animation-preserving control density — 2026-09-30

Stage 10 remains incomplete. Branch `fix/grid-density-preserve-animation`,
based on 7f72201 / PR #10. User supersedes the reset-on-count-change policy.
Columns/Rows now select visible controls over the retained animated deformation;
count changes do not write Grid Positions or resize/reset render snapshots.
The default stored lattice and wire format remain unchanged. New handles edit
that retained curve through interpolation/elastic influence, not through a
lossy conversion of old keys. Fewer visible lines do not discard hidden detail.

Source implements the requirement; exact native AE UI/key metadata checks are
NOT RUN here. See [acceptance and evidence](grid-density-preserve-animation-2026-09-30.md).
Version is 0.9.2 development; the AE effect version is bumped to invalidate old
render caches. This is not a published release. `FSTR Stretch.plugin` is retained.

## Historical static-count/package checkpoint

## Development: static counts and branded test package — 2026-09-30

Stage 10 remains incomplete. Branch `fix/first-application-516`, draft PR #10.
No merge or release is authorized by this development step.

- Source `9199c088b2e31d3c552521d71f4f0d2d5663bb24` makes Columns and Rows
  manual-only setup controls; Grid Positions remains animatable.
- This packaging checkpoint emits **FSTR Stretch.plugin** in the user-facing ZIP.
  Internal executable, effect match name, parameter IDs and rendering remain unchanged.
- Exact automated results, artifact commit/Build ID/hash and host-verification
  boundaries are recorded in PR #10. New package acceptance in AE is NOT RUN.

## Last user-tested candidate (not the new static-count package)

Commit `94d6706db5539d9f3372adf0efc8edeeaa38a622`;
Build ID `EGFX-e967ab9a87fe4e44340d0f46`;
ZIP SHA-256 `99ca836d73cdadb5b3efc793fd16fd6f47f5ceca2ae8944e7612ac4836912b30`.

About identity was shown by the user. First addition, text/Checkerboard-precomp
visible deformation, save/restart, RAM Preview and cancellation/re-render were
confirmed by the user in issues #9, #7 and #8. These are USER-REPORTED results,
not a new instrumented payload/PID or pixel-matrix verification. No result is
transferred to a later Build ID or differently packaged artifact.

## Remaining gates

The latest-head automated regression and signed/extracted package checks must
pass before a test handoff. Target AE must confirm static editable counts,
continued Grid Positions animation, Undo/Redo, fresh addition and save/reopen.
Old projects that already keyframe Columns/Rows are NOT VERIFIED; preserve
originals and test copies. No migration script deletes their keys.

Full evidence reconciliation for #9, #7 and #8 remains open. Stage 8 optimization
stays SKIPPED BY USER, not PASS; there is no performance claim. GPU, Windows/Intel,
non-square 3D UI and broader HDR/OCIO certification remain outside this scope.
The requested bundle filename is a packaging change, not a new effect identity.

## References and preserved history

- [Static counts and compatibility](static-grid-counts-2026-09-30.md)
- [Packaging decision, checks, installation and rollback](branded-package-2026-09-30.md)
- [First-application investigation](first-application-516-2026-09-30.md)
- [Unmodified previous status/history](status-0.9.1-history.md)
- [Published 0.9.1 record](release-0.9.1.md)

The last published release remains e1848d5 / EGFX-879b31e5a95527a385827c24,
not the installed development candidate or latest source. Its historic PASS
records do not close the subsequently reported first-application defect or
certify the new static-count/renamed package.
