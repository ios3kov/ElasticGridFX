# FSTR Stretch 0.9.1 — macOS Apple Silicon

Maintenance release: live 3D panel refresh and native-text perimeter correction.

- Plane controls disable automatically on 3D layers; saved 2D settings and
  Grid Positions animation remain intact.
- Antialiased pixels outside fractional native-text bounds deform with the
  layer plane instead of leaving an undeformed fringe.
- Four Corners retains its bounded deformation-region behavior.
- AE effect version increments to invalidate previously cached frames.

## Immutable artifact

Plugin source: e1848d5348c8059c0307c5d8c1241163ffd92ab1.
Build ID: EGFX-879b31e5a95527a385827c24.
Archive SHA256: 23b75997e310e0acbefcde69570e177199c23d415f67abf00ee064d28efe4d2a.
Subsequent changes to contract tests or documentation do not relabel the binary.
Publish the verified archive without rebuilding it.

## Evidence / scope

AE 25.6.0 arm64 loaded identity PASS, PID 61184. The original perimeter
reproduction and four rotations match the accepted corrected renders exactly;
cold and warm output match without manual cache purge or parameter edits.
The original fringe is transparent in exports from 8/16/32-bit projects.
Core 19/19, Rust 42/42, Python 207/207, Clippy, bundle/signature verification PASS.
Full macOS source gate 36714712234 and five PR CI checks PASS. Eight animated
aerender frames match with MFR ON/OFF; restart identity and frame parity PASS.

Verified target remains ordinary flat native text/raster layers, square pixels,
macOS Apple Silicon and AE 2025 (25.6 tested). No Windows, Intel, universal
HDR/OCIO or per-character 3D certification. GPU dispatch remains disabled;
performance optimization and persistent unselected grid display remain deferred.

Ad-hoc signed, not Developer ID signed or notarized. Do not bypass macOS
security protections. If macOS blocks the downloaded bundle, stop and use the
supported security-review process.

## Install / rollback

Save projects and quit all Adobe hosts. Back up the old ElasticGrid.plugin
outside plugin roots, then extract FSTR-Stretch-0.9.1-macOS-arm64.zip and put
ElasticGrid.plugin in the user Adobe Common Plug-ins/7.0/MediaCore/FSTR FX folder.
Keep only one active copy. Reopen AE: FSTR Effects → FSTR Stretch.
For rollback, quit hosts and restore the previous bundle; preserve project
backups when opening a newer-version project with an older plugin.

User explicitly authorized PR #6 merge and release. Published as v0.9.1 from
merge b40245f1072797de871474ed9c17f1ec5f8d026b. GitHub asset SHA256 verified
equal to the tested archive above; no rebuild during publication.
