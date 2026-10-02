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
