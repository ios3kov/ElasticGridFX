# FSTR Stretch 0.9.0 — macOS Apple Silicon

Release scope: native After Effects effect, category **FSTR Effects**.

## Included

- Elastic guide deformation with animated Grid Positions.
- Layer Plane follows the layer; automatic perspective plane for ordinary 3D text.
- Four Corners defines a deformation region in 2D, not an initial corner-pin warp.
- 3D layers lock the effect to Layer Plane; 2D settings and keys are preserved.
- Contrast guide/grip overlay, contextual open/closed hand cursor.
- CPU SmartFX sparse/transparent input handling and Final Bicubic quality.

## Verified scope and limits

Verified on After Effects 25.6.0, macOS Apple Silicon, square pixels,
ordinary flat native text and raster layers. Tests cover 8/16/32 project depths,
mode transitions, saved bindings/keys, UI lock and background-render parity.
This is not a universal HDR/OCIO, Windows, Intel or all-AE-version certification.
Per-character 3D and non-square pixel native text planes are outside the scope.
GPU dispatch is disabled. Performance optimization was skipped by user request.
Persistent grid display without selection remains deferred.

The plugin is ad-hoc signed, **not Developer ID signed or notarized**. Do not
disable macOS security protections to install it. If macOS blocks a downloaded
copy, stop and use the supported security-review process; this release makes no
claim of frictionless installation on every Mac. The author's local copy is
already installed and verified; no reinstallation is required there.

## Installation / rollback

1. Save work and quit After Effects and other Adobe hosts before replacement.
2. Extract `FSTR-Stretch-0.9.0-macOS-arm64.zip`.
3. Keep a backup of the existing `ElasticGrid.plugin` outside Adobe plugin roots.
4. Put the extracted `ElasticGrid.plugin` in the user folder
   `~/Library/Application Support/Adobe/Common/Plug-ins/7.0/MediaCore/FSTR FX/`.
   Do not keep a second active copy elsewhere in Adobe plugin roots.
5. Open AE and select **FSTR Effects → FSTR Stretch**.

To roll back, quit Adobe hosts, move the new bundle out of plugin roots and
restore the backed-up bundle to its original location. Do not delete projects
or preferences. New internal bindings use append-only parameters; keep a project
backup before opening a new-version project with an older plugin.

## Immutable package identity

- Plugin source: `36a04e3a5ab9ccf2ca8872c18b1344b94de65c5c`.
- Build ID: `EGFX-2c67eaa5e9c441ca5866edaf`.
- Archive SHA256: `f34598ec3f579c56467fba35bcf42fd09e222e9931fbfa976a1f5da08061efe8`.
- Binary SHA and full signed payload inventory: accompanying `ElasticGrid.artifact.json`.
- Documentation-only descendants and merge commits do not relabel the binary.
- Publish the already-tested archive bytes; do not rebuild during publication.

User authorized merge to main and release on 2026-09-30. Publication is gated on
successful mandatory CI including the full macOS source gate. Runtime evidence
is recorded in `docs/current-status.md`; CI is complementary, not AE verification.
