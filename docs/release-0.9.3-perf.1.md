# FSTR Stretch 0.9.3-perf.1 — public experimental prerelease

Prepared 2026-10-02 for macOS Apple Silicon and After Effects 2025 (25.6 tested).
This is a public TEST PRERELEASE, not a stable or fully release-certified build.
The user explicitly elected publication without Developer ID/notarization on
2026-10-02. Those checks are NOT PASSED under the adopted rules; macOS may block
the downloaded plugin. No security-settings bypass is included or required by
these instructions. The package does not claim Gatekeeper-clean installation.
0.9.3-perf.1 identifies this package revision; AE About still shows 0.9.3,
Develop Build 2. The original signed plugin bytes and embedded identity are unchanged.

Build ID: EGFX-6147dc406abc596e7f2d1b60
Plugin source: f611312bd7b76ebe5bc5f2bd8b48b44f50c0c761
Merged development checkpoint: cb429e1ffab06cd79cb0873ba8909b2a8369198d

## Changes and evidence

Exact axis mappings and four-row Bicubic cache accelerate eligible plane rendering.
General perspective/other geometry retains the original mapping path. Sampling
precision, per-channel arithmetic, alpha/HDR and saved parameter contracts remain.
Five matched MFR-requested-OFF Full/Final/32-bpc 60-frame PNG exports reduced
median total time from 65.870721 to 48.766815 seconds (25.97% less).
This includes startup and PNG writing; it is not per-frame or RAM Preview latency.
All 360 measured/warmup paired outputs decode exactly. Same-toolchain 40-frame
plane/3D/AEP and 10 queue-chain comparisons are exact. The user accepted the speed
scope; missing exact first-display timing and Full control Preview timings remain
unmeasured. Actual guide edit/Undo and 60/60-frame Preview completion were checked.

## Known limits and release blockers

An ordinary control MFR render once aborted after 19/60 frames. Four bounded
candidate/control enabled/bypass follow-ups completed; the cause remains unknown.
Six additional ordinary MFR-requested-ON100 runs completed360/360 frames with
exact output. Issue #21 is CLOSED as not reproduced by explicit user decision;
original failure retained, cause unknown, no fix/general MFR certification.
[Retry/closure record](mfr-closure-retry-2026-10-02.json).
https://github.com/ios3kov/ElasticGridFX/issues/21
No Windows, Intel, broad HDR/OCIO or physical GPU certification; GPU dispatch stays
disabled. Old projects with animated Columns/Rows remain outside verified migration.
The bundle is ad-hoc signed, not Developer ID signed or notarized. Developer ID, notarization and clean quarantined distribution checks are NOT
PASSED under AE-Development-Rules 5.1.0 Release sections 26/28. Publication as a
TEST PRERELEASE is by the explicit user exception, not full Release Gate PASS.
Do not remove quarantine or disable Gatekeeper to load it.

## Controlled install and rollback

The same candidate is already installed in the current test environment; no
replacement is needed there. For an authorized controlled test, quit Adobe hosts,
back up the old plugin outside all Adobe plugin roots, and keep a single active copy.
Extract FSTR Stretch.plugin.zip, then place only FSTR Stretch.plugin in the chosen
MediaCore/FSTR FX folder. Verify BuildIdentity.json and its manifest/hash before
loading. If macOS blocks it, stop; do not bypass security checks. For rollback quit
Adobe hosts and restore the backed-up plugin, with no duplicate active copy.

## Focused user-validation question

On a COPY of a representative project, confirm unchanged output, guide edit/Undo,
save/reopen, and complete Preview/render; report any crash with AE version and
Build ID. This does not request another speed timing series. Keep original projects.
Full standards-compliant release would require Developer ID Application signing, notarization Accepted,
applicable stapling/Gatekeeper and a clean quarantined download/install test,
plus completion of the applicable final regression/compatibility/migration gates.
Signing/post-processing creates a new candidate that must be verified separately.

## Publication verified — 2026-10-02

Published as [v0.9.3-perf.1](https://github.com/ios3kov/ElasticGridFX/releases/tag/v0.9.3-perf.1), prerelease=true, draft=false,
target cb429e1. The public asset was downloaded again and SHA-256 matched
`a563f8e14961e19ee0740d5eb063c89e4bbec830ac1053d23fa09de02f2a6d14`. This proves asset identity, not quarantined
Gatekeeper/clean installation. The local extraction signature and payload match
were independently verified before upload; the installed plugin is unchanged.
[Publication record](prerelease-publication-0.9.3-perf.1.json);
[package record](prerelease-package-0.9.3-perf.1.json).

Developer ID/notarization remain NOT DONE / NOT RUN by direct user decision.
This supersedes the previous publication hold only for this experimental test
release; full Release Gate remains NOT PASSED, not an approved standard PASS.

## Historical local-only preparation (superseded publication decision)

# FSTR Stretch 0.9.3-perf.1 — controlled validation package

Prepared 2026-10-02 for macOS Apple Silicon and After Effects 2025 (25.6 tested).
This is a local validation package, NOT a public release or release-ready candidate.
0.9.3-perf.1 identifies this package revision; AE About still shows 0.9.3,
Develop Build 2. The original signed plugin bytes and embedded identity are unchanged.

Build ID: EGFX-6147dc406abc596e7f2d1b60
Plugin source: f611312bd7b76ebe5bc5f2bd8b48b44f50c0c761
Merged development checkpoint: cb429e1ffab06cd79cb0873ba8909b2a8369198d

## Changes and evidence

Exact axis mappings and four-row Bicubic cache accelerate eligible plane rendering.
General perspective/other geometry retains the original mapping path. Sampling
precision, per-channel arithmetic, alpha/HDR and saved parameter contracts remain.
Five matched MFR-requested-OFF Full/Final/32-bpc 60-frame PNG exports reduced
median total time from 65.870721 to 48.766815 seconds (25.97% less).
This includes startup and PNG writing; it is not per-frame or RAM Preview latency.
All 360 measured/warmup paired outputs decode exactly. Same-toolchain 40-frame
plane/3D/AEP and 10 queue-chain comparisons are exact. The user accepted the speed
scope; missing exact first-display timing and Full control Preview timings remain
unmeasured. Actual guide edit/Undo and 60/60-frame Preview completion were checked.

## Known limits and release blockers

An ordinary control MFR render once aborted after 19/60 frames. Four bounded
candidate/control enabled/bypass follow-ups completed; the cause remains unknown.
No general MFR stability certification or crash fix is claimed. Track issue #21:
https://github.com/ios3kov/ElasticGridFX/issues/21
No Windows, Intel, broad HDR/OCIO or physical GPU certification; GPU dispatch stays
disabled. Old projects with animated Columns/Rows remain outside verified migration.
The bundle is ad-hoc signed, not Developer ID signed or notarized. Public native
distribution remains BLOCKED under AE-Development-Rules 5.1.0 Release sections 26/28.
Do not publish this validation ZIP or remove quarantine/disable Gatekeeper to load it.

## Controlled install and rollback

The same candidate is already installed in the current test environment; no
replacement is needed there. For an authorized controlled test, quit Adobe hosts,
back up the old plugin outside all Adobe plugin roots, and keep a single active copy.
Extract FSTR Stretch.plugin.zip, then place only FSTR Stretch.plugin in the chosen
MediaCore/FSTR FX folder. Verify BuildIdentity.json and its manifest/hash before
loading. If macOS blocks it, stop; do not bypass security checks. For rollback quit
Adobe hosts and restore the backed-up plugin, with no duplicate active copy.

## Focused user-validation question

On a COPY of a representative project, confirm unchanged output, guide edit/Undo,
save/reopen, and complete Preview/render; report any crash with AE version and
Build ID. This does not request another speed timing series. Keep original projects.
Public release requires Developer ID Application signing, notarization Accepted,
applicable stapling/Gatekeeper and a clean quarantined download/install test,
plus completion of the applicable final regression/compatibility/migration gates.
Signing/post-processing creates a new candidate that must be verified separately.

## Preparation checkpoint

This local package is not published as a GitHub Release asset. See
[package record](validation-package-0.9.3-perf.1.json) for hashes and checks.
PR #20 merged as cb429e1; its tree matches reviewed f78a5de, with all seven CI
checks PASS. Packaging does not relabel or rebuild the tested binary.
No Developer ID Application identity is available. An Apple Development
identity is available but is not used as a public distribution certificate.
No credentials were exported or changed. Apple sources checked 2026-10-02:
https://developer.apple.com/help/account/certificates/create-developer-id-certificates
https://developer.apple.com/documentation/security/notarizing-macos-software-before-distribution
