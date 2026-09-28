# Current development / release status

Date: 2026-09-28. Branch: fix/final-validation, draft PR #5.
**NOT READY FOR RELEASE.** Main is not changed or merged by this work.

## Current target-Mac action

Target report 7b291509 was received: the per-user FSTR FX copy is not candidate
6d3b846, and live-image confirmation was BLOCKED by a native/sampler path
contradiction; the app-plugin scan also reported an error. Those issues are not
claimed fixed. The user now explicitly authorized replacing only that reported
copy with backup/rollback. See the authorized replacement section below and its
acceptance plan. Actual replacement and AE rendering still await target execution.

## Source history

- main inspected: 5b4843f94a04d2b5f4d205feb6415954a2f2d9bb.
- Functional baseline: b51b95407a022172fe739996fc4a65254418d38b (SmartFX snapshot).
- 53c2a29: exact uniform-grid pixels and ROI-origin overflow protection.
- 0882cdf: ownership-safe project-roundtrip cleanup.
- 64aa67e: parameter metadata and deterministic build/package identity.
- 37bc8bc: isolated preflight outputs; prior source/package checkpoint.
- Latest runtime-tooling stage: see runtime-acceptance-2026-09-28.md and its plan;
  exact resulting commit is identified by the PR/Actions records.

## Previous runtime-tooling verification scope

Local Linux: Python 53/53; new smoke VM 14/14; prior roundtrip VM 11/11; strict
GCC Release C++ 10/10 with normal full fuzz/soak. Python compile, shell syntax and
whitespace checks PASS. Renderer/host sources and parameter schema are unchanged
in this stage. Filesystem, image and VM fixtures are not real AE/Metal tests.

CI must be evaluated for the exact commit. The macOS source workflow builds/signs
and verifies the candidate and additionally runs tooling regression, read-only
inspection and smoke preparation. Four hardware Metal stages are compile-only
on the hosted runner; real device execution is not inferred.

Historical checkpoint 37bc8bc passed final-validation 36457025422, all five PR CI
jobs in 36457033148 and macOS source/package 36457025659. Earlier failed run
36455408377 remains FAIL; its clean-source scratch-output defect was fixed, not
ignored. Archived reports are not rewritten to certify newer candidates.

## Remaining mandatory gates

| Gate | Status / required evidence |
|---|---|
| Original AE no-deformation report | BLOCKED: no real target AE here. Reproduce with patterned source and actually loaded Build ID; test drag, waves, reset and rerender |
| Loaded artifact identity | Generated About/startup ID and signed manifest implemented; actual AE observation NOT RUN. File hashes do not prove loading |
| Parameter/project compatibility | IDs/ordinals preserved; baseline Rust ordinal regression PASS. Actual old projects, out-of-range keys, Undo/Redo, restart and save/reopen NOT RUN |
| Patterned image smoke | Seven-frame capture/five pixel checks implemented; negative/positive fixtures PASS. Actual AE capture NOT RUN. PNG smoke is not HDR, geometry or GPU parity |
| Installation | Single reported copy may now be replaced under explicit user authorization, with original retained for rollback. Tooling/native checks are tracked separately; actual target replacement NOT RUN. Global conflict-free installation remains unverified |
| Renderer compatibility | Real 8/16/32-bpc AE, alpha/HDR, ROI/downsample/PAR, MFR, cancellation, render queue/aerender and Metal tests BLOCKED |
| Performance | Real AE preview/render, memory and physical Metal profiling BLOCKED; no new speed claim |
| Delivery | BLOCKED by the above. Final immutable candidate and all mandatory evidence must match; no installation, main merge or release performed |

## Next work

Complete the predefined installer/rollback gate, then provide the immutable
candidate in the explicitly authorized single-copy update package. Inspect the
target installation result before any claim of replacement or host acceptance.
Resolve the diagnostic path/scan evidence and complete actual loaded-ID and AE
functional tests. No broader plugin removal, preference reset or process kill is
authorized. Synthetic tests must not be represented as target-host results.

The old roundtrip tool is a persistence diagnostic, not full image acceptance.
The new smoke restores bit depth/removes only its created objects, but normal
AE project dirty/UI state may change; a fresh authorized empty project is needed
for subsequent tests. No preferences or user projects are reset to hide this.

Only after every mandatory gate passes should the exact candidate be approved,
compatibility/release records updated and reviewed changes considered for merge.
Authoritative process: ../DEVELOPMENT_RULES.md.

## Authorized single-copy replacement (2026-09-28)

The user explicitly authorized replacing ONLY the reported per-user FSTR FX
copy, retaining the old version for rollback. `authorized-update-plan-2026-09-28.md`
defines this narrower installation gate. The immutable candidate remains 6d3b846,
not a new native build. New update/rollback tooling pins the old executable hash,
uses an atomic whole-directory exchange, records a PREPARED receipt before any
swap, preserves the original inode/metadata, and refuses unknown changed state.
It neither starts nor terminates Adobe applications and has no sudo/delete path.

Local tooling: 98 Python tests PASS, five macOS-only cases NOT RUN (103 total),
plus the existing 11+14 JSX mock cases PASS. Actual pinned candidate unpacking and
all payload hashes verified. The three new native replacement/signature/xattr
cases require the exact-head Authorized update safety workflow; the other two
native sampler cases belong to the existing diagnostic. Consult PR checkpoint
for actual CI completion and delivered package identity. No target-Mac replacement
has been executed in this session; sending a tested installer is not installation.

The previously observed diagnostic path contradiction and incomplete app-plugin
scan remain unresolved. This scoped replacement does not certify loaded identity,
global conflict absence, image correctness, physical Metal or production readiness.
