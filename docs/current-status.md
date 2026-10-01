# Current development / release status

## Final Stage 10 acceptance — 2026-10-01

**Stage 10 COMPLETE for the agreed macOS Apple Silicon / AE 25.6 development scope.**

Exact accepted candidate: `2ccc5f674b8b94b534d4c3ad6abdec3524e3624f`,
Build `EGFX-bd19dee13315abc0b7e6090e`, FSTR Stretch 0.9.3 development,
ZIP SHA-256 `1448a5231fc561e6b10491d1b9d1839de22f21a11266a43e2990244a25176ebc`.

Automated gates for this exact source/package are PASS: Rust 61/61, C++ 20/20,
Python 239/239, strict Clippy, PR CI and full macOS source/build/signature/package
gate. The user loaded this exact candidate in target AE and confirmed the final
About, functional behavior, Layer Plane alignment, RAM Preview and three
cancel→rerender cycles. #7, #8, #9, #13 and #14 are closed as scoped evidence.

See [Stage 10 final acceptance](stage10-final-acceptance-2026-10-01.md).

Stage 8 optimization remains **SKIPPED BY USER**, not PASS. Windows/Intel, other AE
versions, broad HDR/OCIO, physical GPU execution and public signing/notarization
remain outside the accepted scope. **No merge or 0.9.3 release is authorized yet.**

Everything below is preserved historical development context unless explicitly
marked current.

## About footer candidate gate failure and correction — 2026-10-01

Candidate source `e4f1ed10c650f9f70e606ed97f34d4dc16042ae4` removed the redundant
standalone `fstr.tech` footer, but the mandatory first-application and macOS
source gates **FAILED** before handoff. The failure is preserved as evidence and
is not relabeled as PASS.

Failed runs:
- push first-application regression **36853623602** — FAIL in Rust host contracts;
- PR first-application regression **36853629081** — same FAIL;
- macOS source gate **36853623609** — FAIL during full preflight;
- PR CI **36853629066** — PASS for portable C++/sanitizer/static-analysis scope only.

Root cause: research-only `host-rust/src/lifecycle_probe.rs` still referenced
the removed generated constant `build_identity::ABOUT`. The production About
path had already moved to `ABOUT_BYTES`, while internal diagnostics use
`DIAGNOSTIC`. This compile error is unrelated to rendering or the copyright
byte itself, but it blocks delivery under the development rules.

Correction: the lifecycle research journal now uses
`build_identity::DIAGNOSTIC`, matching the intended separation between
user-facing About bytes and internal provenance. No renderer, parameters,
serialization, Fit Layer, grid-density, package naming or project data behavior
changes in this correction.

The previous failed runs remain failures. A new candidate must pass the complete
required regression and macOS source/package gates before it can be handed off.
Target-AE verification is still required for the final About rendering. No merge
or release is authorized.

## About footer cleanup — 2026-10-01

Per user review, the standalone `fstr.tech` footer is redundant and removed.
The final visible copy is now:

```text
FSTR Stretch
Version 0.9.3

Professional mesh deformation for Adobe After Effects

© 2026 FSTR.tech. All rights reserved
```

The single-byte copyright encoding correction remains unchanged. This source
change creates a new candidate identity, so automated gates must pass again
before handoff. No merge or release is authorized.

## About copyright encoding correction — 2026-10-01

The user opened the newly branded 0.9.3 About dialog and the new text appeared,
but the copyright glyph rendered as `¬©`. This directly identifies an encoding
mismatch in the visible dialog: the Rust wrapper copied UTF-8 bytes `C2 A9`
into AE's legacy `A_char[256]` return-message buffer, and the host displayed
both bytes as characters. Adobe's own About dialog on the same host displays a
normal copyright glyph.

The correction keeps the real `©` symbol and writes the About field as exact
legacy bytes with a **single `0xA9` copyright byte**. The visible copy is also
updated per user request to remove terminal periods:

```text
FSTR Stretch
Version 0.9.3

Professional mesh deformation for Adobe After Effects

© 2026 FSTR.tech. All rights reserved
```

The internal BuildIdentity/diagnostic path remains unchanged and separate.
This source change creates another new candidate identity, so the prior package
and Build ID must not be relabeled. Mandatory automated gates and a final
target-AE About check are required again. No merge or release is authorized.

## Target-AE acceptance and About branding — 2026-10-01

The user loaded the exact 0.9.3 candidate `ae0c6c47d3318f3e1b7cb0edb4290f6ce04b0dd7`
(Build ID `EGFX-2ac782234af13eae3b01f67d`), supplied an About screenshot
matching those identifiers, and after the requested manual checks reported that
the rest works. Record this as **USER-REPORTED acceptance of the requested
manual scenario**, not as full platform/compatibility certification.

The next source change is presentation-only: the user-facing About dialog is
approved as:

```text
FSTR Stretch
Version 0.9.3

Professional mesh deformation for Adobe After Effects

© 2026 FSTR.tech. All rights reserved
fstr.tech
```

Build ID, commit, target and source-state text are removed from the visible
About dialog, but provenance remains in `BuildIdentity.json` and the
noninteractive diagnostic string so support/package verification can still
identify the exact binary.

Because this changes source after the accepted `ae0c6c4` artifact, any plugin
built from the new commit is a **new candidate** with a new commit/Build ID.
The old artifact hashes and host PASS must not be transferred to it; mandatory
automation/package checks and target-AE handoff must be repeated. Version
0.9.3 remains a development version and is not a published release.

The remaining Layer Plane overlay/index-span alignment item in #14 stays open
until its real AE viewer mapping is measured and resolved or explicitly
accepted. No merge or release is authorized.

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
