# FSTR Stretch.plugin — packaging and static-count follow-up

## Stage 10; implementation checkpoint, not final AE acceptance

User requirements: Columns and Rows are manual-only setup values; Grid Positions
remains animatable. The installed file must be named **FSTR Stretch.plugin**.
The static-count change is commit `9199c088b2e31d3c552521d71f4f0d2d5663bb24`.
This separate packaging change does not edit renderer, topology or parameter code.
Process reviewed: FSTR-Line/DEVELOPMENT_RULES.md, blob
`701a8c1ae3acb4dbfe1d7eda94acbf8095b88608`.

### Prior candidate evidence, not evidence for the next bundle

The user confirmed on `94d6706` / `EGFX-e967ab9a87fe4e44340d0f46`: first-add error
removed, text and Checkerboard-precomp deformation, save/restart, RAM Preview,
and cancellation followed by full rerender. These are USER-REPORTED checks in
issues #9, #7 and #8, not instrumented pixel/loaded-payload certification.
That candidate remains unchanged; it still allowed animation of Columns/Rows.

Static-count source `9199c08`: exact-head regression `36747040491` completed
successfully (50 Rust tests, Clippy, 19 C++ tests, 215 Python tests on macOS).
Full macOS source/build run `36747040461` was still in progress at this writing;
no completion is inferred here. Final results/identities are recorded separately
in PR #10 so evidence recording does not rebuild a checked binary.

## Packaging decisions and acceptance fixed before handoff

- Keep the existing internal `dist/mac/ElasticGrid.plugin` build and historical
  pinned tools intact. They are build inputs/legacy workflows, NOT a second install.
- Normal build emits the user-facing archive and manifest under `dist/mac/delivery`.
  Its only bundle is **FSTR Stretch.plugin**. Internal executable `ElasticGrid`,
  resources, signature, bundle identifier, effect match name and saved IDs stay intact.
- Verify input archive/manifest, copy only validated regular files, preserve all
  payload bytes and executable flags, seal and reverify the new ZIP/manifest.
  The new ZIP hash differs because paths changed; do not reuse the old ZIP hash.
- Record the packaging source and input ZIP hash separately from embedded Build ID.
  Dirty tooling, symlink paths, modified inputs and existing output directories
  stop packaging. No installer, cleanup, process termination or security bypass.
- macOS gate must verify the signature/entrypoints of the renamed bundle AND of
  a real extraction of the final spaced-name ZIP, then verify extracted hashes.
  Portable tests never claim codesign or AE PASS. The manifest keeps runtime
  verification NOT RUN; signature results are in the exact-head build log.
- The create-only installer accepts either bundle filename; its read-only scanner
  catches both, including the branded name when its metadata is absent. Existing
  bundles in user/system roots prevent a conflicting installation. Historical
  pinned updaters are not retargeted and must not be used for this new candidate.

References checked:
https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFBundles/BundleTypes/BundleTypes.html
https://ae-plugins.docsforadobe.dev/effect-basics/PF_ParamDef/#parameter-flags

## Checks at this source checkpoint

Eleven new filesystem/ZIP/safety tests PASS locally: spaced filename, exact file
set/bytes/modes/Build ID, changed archive/payload rejection, input preservation,
no output overwrite, symlink rejection, user/system legacy conflicts, branded
create-only installation and idempotency (host/signature calls mocked).
Full Linux Python: 226 discovered, 220 PASS, 6 macOS-only skips. All Node test
files PASS. No Rust/C++ production source changed in this packaging step.
Latest-head hosted regression and macOS package/extracted-signature verification
are required before handoff. Actual AE tests on the new artifact remain NOT RUN.

## Installation and rollback for the named candidate

Save projects and quit Adobe hosts normally. Keep a backup of the current bundle
OUTSIDE all Adobe plugin roots. Inspect both user and system MediaCore/FSTR FX
folders and AE-specific plugin folders for both filenames; do not install beside
an old active copy. Place only `FSTR Stretch.plugin` in the single chosen user
MediaCore/FSTR FX folder. Do not disable Gatekeeper or strip quarantine if blocked.

Restart AE and check the new About commit/Build ID, not only the version string.
New-effect check: Columns/Rows have no stopwatch and remain editable; Grid Positions
still records/plays keys. Counts 1/4/50, Undo/Redo, first add and save/reopen must
be checked on the target. Changing counts after animating still resets the changed
axis by existing design; choose counts before keyframing Grid Positions.

Old projects already containing animated Columns/Rows are NOT VERIFIED. Test only
copies and retain originals. No migration script deletes their keys.

Rollback: quit Adobe hosts; move the new named bundle outside plugin roots; restore
the exact backed-up old bundle to its original location. Never leave both names
active. No host verification, installation, merge, release or overall gate closure
is implied by building the archive.
