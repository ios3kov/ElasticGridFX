# Current development / release status

## Stage 8 resumed — 2026-10-01 — IN PROGRESS

The user explicitly resumed extremely fast Render / RAM Preview with unchanged
Final quality. This supersedes the earlier skip for new work; it does not change
the historical 0.9.3 acceptance or approve a new artifact.

Branch: `perf/plane-render-throughput`, baseline `2e3d066`.
The current plane renderer is measured independently from the older separable
CPU path. Implementation caches exact axis mappings/taps and four horizontal
rows only for structurally axis-aligned, nonprojected planes at unit raster
scale. Other geometries keep the general mapping. Per-channel tap/arithmetic
order, Final Catmull-Rom, integer rounding, float/HDR, C ABI and saved data remain
unchanged. Independent-frame caches are call-local, with no retained source data.

Corrected source `41283e3d0935160da669d7ec69bc4ae01d51f264`: native
baseline/candidate pixels are byte-identical in 31 finite scenes and 16 additional
exceptional-float arm64/x86-Rosetta checks. Five alternating pairs show 12.06x
median improvement at 1080p/32 bpc and 12.84x at 4K/32 bpc for eligible deformed
planes. Perspective: 1.26x. No target-AE speed claim follows from these timings.

Corrected code passed strict host-math C++ 21/21, ASan+UBSan 21/21 and TSan.
Its complete physical-Mac 20-stage preflight, Rust 61/61/Clippy, real Metal,
dependency audit, two clean reproducible builds, signed bundle and extracted ZIP
checks PASS. Linux GCC/Clang/static/ASan+UBSan/TSan and two regression CI jobs PASS;
hosted macOS source gate PASS. All eight checks at source 41283e3 PASS.

Validation artifact: Build `EGFX-ce45a845413a943552df0258`, source `41283e3`, ZIP
SHA-256 `a3c001599dc615996a666eac2a6976b2576adf4a0d5ccd4f85cc1daf86dbf362`.
User explicitly authorized temporary replacement. Candidate 41283e3 was temporarily
installed for test; exact old `2ccc5f6 / EGFX-bd19dee13315abc0b7e6090e` inode,
content/modes/xattrs were retained and verified in a transaction backup. Receipt
`EGFX-update-b2a4807ee046479ab986a4bd9b1a972b`. Loaded GUI-AE and aerender image identity PASS for the exact candidate. The
unchanged legacy PNG smoke passed five ordinary pixel checks but did not produce
the Adjustment Layer chain frame; the full matrix remains BLOCKED pending baseline
comparison. An immutable animated 1080p/32-bpc/Final 60-frame AEP was prepared in
a controlled workspace. Its alternating aerender series retained nine successful
runs but stopped on a changed process key at run 9; five measured pairs were
not obtained. Decoded RGB8 equality FAIL (max 1/255), including a baseline repeat.
No target speed claim is accepted. Exact external native checks on the same
pattern pass all 16 cases; this is not AE checkout proof. New fixtures now require
verified straight RGBA16 with recorded color state and encoded-header validation.
Python 240/240 and all 12 JSX control-flow files PASS; mocks are not host proof.
Original installed payload is restored; Adobe hosts stopped. GUI continuation
is waiting for the user to unlock the Mac. RAM Preview remains NOT RUN. No cache purge, main merge
or public release. Initial aefeb4b artifact is withheld after
its x86 NaN-payload FAIL; corrected tests retain exact equality.
See [the resumed performance record](performance-resume-2026-10-01.md).

### Resume checkpoint — AI-STATE-001

- Goal: quality-preserving Render/RAM Preview acceleration, requested by the user
  on 2026-10-01; [quality contract](performance-quality-contract.md) covers this
  scope. Stage 8 exit requires actual host timings and unchanged output.
- Rules: v5.0.0 candidate / `b27f45467e0a9152fc82c1072438dfed07f0c36e`, refreshed
  at the user's explicit request, with AI_ENTRYPOINT read first. Stable published
  standard remains v4.0.0. Applicable native Critical risk; Development work and
  limited Validation artifact, no Release request.
- Components: existing Rust `after-effects 0.4.0` SDK-less host and C++ plane
  sampler, Apple Silicon/macOS 26.6.2 physical checks; AE 2025 / 25.6 target scope.
  No new SDK/host API use; reuse current integration evidence only for unchanged
  contracts. New API use requires source/signature/version verification.
- Branch: `perf/plane-render-throughput`; source checkpoint `41283e3`. Subsequent
  tooling/documentation checkpoint changes no renderer/host code. Preserve exact artifact
  identity above; recheck HEAD/tree before dependent actions.

| Confirmed scope / permission | Source | Still applies |
|---|---|---|
| Continue this repository under central rules | Original user request, 2026-10-01 | Source edits and relevant checks |
| Extremely fast Render/RAM Preview, no quality loss | User clarification, 2026-10-01; quality contract | No filter/precision/resolution downgrade |
| Push performance branch and open draft PR | Explicit user answer, 2026-10-01 | [PR #20](https://github.com/ios3kov/ElasticGridFX/pull/20), ongoing branch updates |
| Reread current AI_ENTRYPOINT and continue | User instruction, 2026-10-01 | Adopt current v5.0.0 candidate baseline for continuing work |
| Temporary installed-plugin replacement | Explicit user message, 2026-10-01 | Exact 41283e3 candidate, preserve original for rollback |
| Close projects without saving in After Effects | Explicit user message and AE-only clarification, 2026-10-01 | AE project closure allowed; does not extend to other apps |
| Finish development continuously with stage statuses | Explicit user instruction, 2026-10-01 | Continue applicable development/validation; report each completed block, then continue |

Temporary candidate installation is authorized; original is restored between experiments. Controlled host
validation continues on synthetic projects; merge, release and cache purge
are outside the current authorization. Closing AE projects without saving is
explicitly authorized. The task-state record points to the conversation; it is
not independent authorization.

Next: finish straight-RGBA16 host validation after Mac unlock, compare the missing legacy chain capture
on the original plugin, and run the existing straight-RGBA16 plane/3D/AEP matrix.
Accept Render timings only after pixel parity; RAM Preview needs externally
observed cache-build/playback measurements. Independent source/CI/documentation work continues alongside host validation. Stage 8 remains IN PROGRESS.

Canonical rules now point to the user-selected AE-Development-Rules baseline.
Earlier project evidence and the accepted 0.9.3 runtime candidate below remain
historical evidence for those exact artifacts.

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

At this accepted checkpoint Stage 8 optimization was **SKIPPED BY USER**, not PASS.
It is resumed above for new development. Windows/Intel, other AE
versions, broad HDR/OCIO and physical GPU execution remain outside the accepted
scope.

After acceptance, PR #10 → #12 → #15 were merged to `main`; PR #16 merged the
Stage 10 documentation. The plugin tree at the PR #15 merge matched the exact
accepted `2ccc5f6` tree; later merged differences were documentation/checksums only.
0.9.3 is still **not published**.

The §25.1 technical retrospective is now recorded in
[retrospective-0.9.3.md](retrospective-0.9.3.md). Under the current macOS public
distribution rule, release readiness remains **BLOCKED** on Developer ID signing,
notarization/applicable stapling and a Gatekeeper-clean quarantined download/install
test. The accepted development artifact remains ad-hoc signed; no security bypass is
an acceptable release step.

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
