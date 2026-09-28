# Current development / release status

Date: 2026-09-28. Branch: fix/final-validation, draft PR #5.
**NOT READY FOR RELEASE.** Main is not changed or merged by this work.

## Source history

- main inspected: 5b4843f94a04d2b5f4d205feb6415954a2f2d9bb.
- Functional baseline: b51b95407a022172fe739996fc4a65254418d38b (SmartFX snapshot).
- 53c2a29: exact uniform-grid pixels and ROI-origin overflow protection.
- 0882cdf: ownership-safe project-roundtrip cleanup.
- 64aa67e: parameter metadata and deterministic build/package identity.
- 37bc8bc: isolated preflight outputs; prior source/package checkpoint.
- Latest runtime-tooling stage: see runtime-acceptance-2026-09-28.md and its plan;
  exact resulting commit is identified by the PR/Actions records.

## Latest verified scope

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
| Installation | Create-only inspected installer fixture-tested. Actual Mac installation NOT RUN; requires explicit scope, stopped hosts, native signature and all custom scan roots. Existing builds never overwritten |
| Renderer compatibility | Real 8/16/32-bpc AE, alpha/HDR, ROI/downsample/PAR, MFR, cancellation, render queue/aerender and Metal tests BLOCKED |
| Performance | Real AE preview/render, memory and physical Metal profiling BLOCKED; no new speed claim |
| Delivery | BLOCKED by the above. Final immutable candidate and all mandatory evidence must match; no installation, main merge or release performed |

## Next work

Evaluate the exact-head source/packaging gates and keep failure diagnostics.
Then complete actual loaded-ID observation and run the prepared tooling in an
authorized, isolated target AE environment. Do not route internal trial-and-error
QA to the user or pretend synthetic tests validate the host. Existing-build
upgrade/removal needs a separately authorized reversible plan; the current
installer intentionally refuses it.

The old roundtrip tool is a persistence diagnostic, not full image acceptance.
The new smoke restores bit depth/removes only its created objects, but normal
AE project dirty/UI state may change; a fresh authorized empty project is needed
for subsequent tests. No preferences or user projects are reset to hide this.

Only after every mandatory gate passes should the exact candidate be approved,
compatibility/release records updated and reviewed changes considered for merge.
Authoritative process: ../DEVELOPMENT_RULES.md.
