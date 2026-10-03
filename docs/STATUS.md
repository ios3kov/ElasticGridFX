# Current status

Updated: 2026-10-03. This is the concise entry point for the current product and
continuation. [Dated checkpoints](current-status.md) retain historical detail;
their older holds and release-policy statements do not override the state below.

## Published macOS release

[FSTR Stretch v0.9.3-perf.1](https://github.com/ios3kov/ElasticGridFX/releases/tag/v0.9.3-perf.1)
is the published ordinary release. Package version is 0.9.3-perf.1; the unchanged
plugin About version is 0.9.3 Develop Build 2.

- Shipping source: `f611312bd7b76ebe5bc5f2bd8b48b44f50c0c761`.
- Build ID: `EGFX-6147dc406abc596e7f2d1b60`.
- Public ZIP SHA-256: `a563f8e14961e19ee0740d5eb063c89e4bbec830ac1053d23fa09de02f2a6d14`.
- Verified host scope: AE25.6x101, macOS26.6.2, Apple Silicon.
- Clean-environment installation and animated legacy Columns/Rows acceptance
  remain USER-REPORTED, with environment details unspecified.
- Adopted macOS/project rules baseline: 6.0.0 / `bb8b769404ddd5b97462812a4e6b430e8bfefe13`.
  Developer ID/notarization are not prerequisites under that baseline.

Read [the user guide](USER_GUIDE.md), [release record](release-0.9.3-perf.1.md),
[publication/baseline evidence](release-publication-rules6-2026-10-02.json) and
[public install/project check](public-install-project-check-2026-10-02.json).

## Performance and remaining limits

The quality-preserving CPU optimization is complete and accepted by the user.
Five matched 1080p/60-frame/Full/Final/32-bpc PNG exports with MFR requested OFF
reduced median total time from 65.870721 to 48.766815 seconds (25.97% less).
This includes startup and encoding. Paired measured/warmup pixels match exactly.
RAM Preview scope and latency limits are documented separately; no more timing
series is planned for this accepted macOS cycle.

The earlier MFR control abort did not recur in six bounded retries (360 frames).
Issue21 is closed as not reproduced by user decision. Cause remains UNKNOWN,
fix NONE; this does not certify general MFR reliability. Reopen for a new crash
with exact artifact/project identity and pre-termination evidence.
Broad HDR/OCIO, per-character 3D and untested host/platform compatibility remain
outside the verified scope. Persistent grid display when unselected is a
[future feature](persistent-viewer-grid.md), not an unfinished release gate.

Read the [performance/release retrospective](retrospective-0.9.3-perf.1.md),
[quality contract](performance-quality-contract.md) and
[MFR closure record](mfr-closure-retry-2026-10-02.json).
Reusable findings were contributed in
[central rules PR16](https://github.com/ios3kov/AE-Development-Rules/pull/16);
the [local know-how record](AE_ENGINEERING_KNOWHOW.md) retains its original
review-stage provenance rather than rewriting history.

## Windows continuation

As checked on 2026-10-03, Windows work is isolated on
[`feat/windows-x64-aex`](https://github.com/ios3kov/ElasticGridFX/tree/feat/windows-x64-aex),
checkpoint `003607dd4795596975d2f7a7e4b192cf653dfa34`. The branch consciously
adopts rules6.2.0 for Windows; it does not silently change this macOS baseline.
Its exact-head Windows build/static and macOS source gates passed. A Windows x64
AEX artifact exists, but Windows AE runtime is NOT RUN/BLOCKED. Load/identity,
guide interaction, Undo/Redo, save/reopen, first-application matrix, Render Queue,
MFR/aerender and Windows performance still require the selected Windows+AE host.
Neither Windows GPU nor new product features are part of this CPU port.

These links are pinned to that reviewed checkpoint so this summary cannot
silently inherit later branch results:
[Windows port status](https://github.com/ios3kov/ElasticGridFX/blob/003607dd4795596975d2f7a7e4b192cf653dfa34/docs/windows-port-status.md),
[validation procedure](https://github.com/ios3kov/ElasticGridFX/blob/003607dd4795596975d2f7a7e4b192cf653dfa34/docs/windows-ae-validation.md).
Next engineering step is Windows AE validation of the exact candidate;
compilation is not runtime acceptance. No Windows merge/release is recorded here.

## Documentation maintenance

This cleanup is documentation-only under the adopted rules6.0.0 baseline:
Process/Core, Engineering §§24/25 and Workflow apply. No native code, build,
installation, product contract or historical evidence verdict changes.

The documentation index groups current entry points, retained evidence and old
checkpoints. The full old status remains at its original address, clearly marked
historical; the duplicate unversioned verification file becomes a small pointer
to the preserved v0.6 report. Root README directs readers here. Source and binary
identities above refer to the existing release, not a rebuilt documentation HEAD.
Tracked documentation participates in new Build IDs: any future validation
artifact must be built from its own exact final source checkpoint.

Maintenance verification PASS: 296 local Markdown paths/anchors, complete
100-file documentation index, unchanged source/build/evidence bytes, unchanged
historical status body and preserved full v0.6 report. Git whitespace and root
SHA256 manifest are checked before commit. The static scanner exited1 for the
already-reviewed false positive in the mocked local artifact-manifest unit test
at tests/test_target_ae_acceptance.py:54; this is not an auth/network endpoint.
CI on this documentation commit is reported separately from the existing
release's tests. See [documentation index](README.md).
