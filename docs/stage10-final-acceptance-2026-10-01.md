# FSTR Stretch 0.9.3 — Stage 10 final acceptance — 2026-10-01

## Exact accepted candidate

- Plugin source commit: `2ccc5f674b8b94b534d4c3ad6abdec3524e3624f`
- Build ID: `EGFX-bd19dee13315abc0b7e6090e`
- Version: **0.9.3 development**
- Target: **macOS Apple Silicon / After Effects 2025 25.6**
- Source SHA-256: `efdd146d3324f650e91ad435b26977898a3574f78a866a66f95bec6e47eff9a4`
- Delivery ZIP: `FSTR Stretch.plugin.zip`
- Delivery ZIP SHA-256: `1448a5231fc561e6b10491d1b9d1839de22f21a11266a43e2990244a25176ebc`
- Delivery manifest SHA-256: `ae48d9761fab67ca529b2da17b96eb89b6182a2de6ecf95db4a2beb6ee8f797f`
- Binary SHA-256: `7dc57622442eaddcc1f061e044e77db3796a2db44e4061c807eee6aa60c15beb`
- macOS source gate: run **36854097655**, artifact **11157821821**
- Outer Actions artifact digest: `sha256:67a267119350ab1e830021c8ea9cedb4c04b753e04a53870c9b2838d81fb0fc1`

The accepted artifact is unchanged after verification. This documentation record is
kept on a separate docs branch so it does not create a new plugin Build ID.

## Automated evidence for the exact candidate

- Rust host/FFI tests: **61/61 PASS**
- C++ regression: **20/20 PASS**
- Python tooling/safety: **239/239 PASS**
- Strict Clippy: **PASS**
- PR CI **36854104891**: GCC/Clang, ThreadSanitizer, ASan+UBSan and portable static analysis **PASS**
- Push first-application regression **36854097603**: **PASS**
- PR first-application regression **36854104954**: **PASS**
- macOS source/build/signature/package gate **36854097655**: **PASS**
- Named and extracted delivery bundle signature/payload/version checks: **PASS**
- Both Finder version fields: **0.9.3**

Hosted Metal hardware execution remains compile-only; this does not claim physical
GPU validation.

## Target-AE user acceptance

The user loaded the exact candidate above and confirmed **«всё работает»**.
The following explicit final checks are recorded as **USER-REPORTED PASS**:

- final About text and normal copyright glyph;
- first application / no BadCallbackParameter dialog;
- visible deformation and save/reopen behavior retained;
- Fit Layer behavior and Undo/Redo retained;
- Columns/Rows density changes do not break the existing animation scenario;
- Layer Plane grid visually matches the image (#14);
- RAM Preview: cache build → replay → deformation change → rebuilt replay (#7);
- Render Queue cancellation: three active cancel → full rerender cycles without restarting AE (#8).

Issues **#7, #8, #9, #13 and #14 are closed** against this evidence chain.

These are user-reported target-host checks, not independent instrumented pixel-buffer
or timing captures. They support the agreed acceptance scope, not universal platform
certification.

## Final About copy

```text
FSTR Stretch
Version 0.9.3

Professional mesh deformation for Adobe After Effects

© 2026 FSTR.tech. All rights reserved
```

The copyright is written as one legacy `0xA9` byte in AE's return-message buffer.
Build ID/commit/source provenance remains separate in `BuildIdentity.json` and the
noninteractive diagnostic path.

## Stage status and limitations

**Stage 10 is COMPLETE for the agreed macOS Apple Silicon / AE 25.6 development scope.**

Stage 8 optimization remains **SKIPPED BY USER**, not PASS. There is no new
performance claim.

Not certified / outside the agreed scope:
- Windows and Intel macOS;
- other After Effects versions;
- broad HDR/OCIO/linear-color certification;
- physical GPU execution/parity;
- non-square-PAR 3D UI beyond the documented limitations;
- Developer ID signing/notarization/public distribution.

The current bundle is ad-hoc signed for the controlled development workflow.

## Release / merge state

After this acceptance, the user authorized the pre-merge verification and merge.
PR #10 → #12 → #15 were merged to `main`; the tree after the PR #15 merge was
identical to the accepted `2ccc5f6` source tree. PR #16 subsequently merged only
documentation and `SHA256SUMS.txt`.

The accepted runtime artifact remains the exact candidate above; no post-acceptance
binary rebuild is relabeled as the accepted file. Version 0.9.3 is still **not a
published GitHub release**.

See [0.9.3 technical retrospective](retrospective-0.9.3.md). Under the current
public macOS distribution rule, a normal public release remains BLOCKED until a
Developer ID signed/notarized Gatekeeper-clean artifact is produced and verified.
