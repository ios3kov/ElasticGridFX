# Current development / release status

Date: 2026-09-28. Branch: fix/final-validation. **NOT READY FOR RELEASE.**

## Latest stage

See host-identity-2026-09-28.md for the host metadata/Build ID change and its
predefined acceptance criteria. Local Python tooling tests are 24/24 PASS;
existing Node control-flow tests 11/11 PASS. Real AE/Metal remains BLOCKED.
The earlier macOS workflow 36451642934 passed for e1aa77e; it does not certify
this later source. Read exact-head Actions results/checkpoint comments.

## Source chain

- main at inspection: 5b4843f94a04d2b5f4d205feb6415954a2f2d9bb; not modified here.
- Latest functional baseline: b51b95407a022172fe739996fc4a65254418d38b, fix/smartfx-param-snapshot.
- f35a351443e7c0173a3ace6346a502906a409611: source identity/evidence workflow.
- 53c2a299265caf3e653a766eaf0f6e03b728a621: uniform-grid pixel and ROI fix.
- 0882cdf0fc7ff64a273232f1d6b7097a1d995db6: project-test ownership protection.

The current documentation update does not change production code. Prior test-build branch 9fa06ff adds packaging workflow changes only; its binary is not a verified release of this work. The original-gridwarp-parity PR remains separate; no merge to main has been performed.

## Verified scope

Linux x86_64 source verification: GCC Release 10/10, Clang Release 10/10, Clang ASan/UBSan/LeakSanitizer 10/10, GCC TSan concurrency 2/2, allocation/lock audit PASS. Normal Release uses full fuzz and 50,000-cycle soak; sanitizer runs use fuzz scale 0.1 and 3,000-cycle soak. Script ownership/control-flow: 11 cases PASS under Node 22.16.0; not an AE host test. See the two dated fix reports for the actual baseline failures, acceptance criteria and limitations.

Final-validation and macOS source workflows must be evaluated for the exact resulting commit. Queued/running jobs are not PASS. Real Metal execution is not available on the hosted macOS source runner. The first local Clang analyzer run passed; a later redundant run exceeded the command time limit and is not counted as another successful run. clang-tidy/cppcheck and Rust Clippy were not run in the local Linux environment.

## Required remaining gates

| Gate | Status | Required completion |
|---|---|---|
| Original no-deformation/blurred-lines report in AE | BLOCKED: no authorized target-AE runtime in this session | Reproduce on patterned input with known loaded Build ID; test drag, waves, reset and rerender, preserving baseline/output |
| Runtime artifact identity | Generated metadata and final payload/ZIP verification implemented; target packaging pending exact-head CI; loaded AE identity BLOCKED | Verify the full About/startup ID of the actually loaded plugin against the signed candidate manifest |
| Host parameter semantics | Metadata corrections + 5 source guards PASS; Rust ordinal test awaits target CI; AE verification BLOCKED | Confirm old project keys/load behavior and visible labels in AE; all 17 IDs and ordinal meanings preserved, no-op legacy wave switch hidden |
| Deformation-sensitive AE smoke | NOT RUN / coverage inadequate | Current flat solid/nonempty PNG smoke cannot detect pass-through. Replace or supplement with patterned baseline/changed-frame comparisons; retain evidence and unique run IDs |
| Safe clean installation | BLOCKED / runner paths need reconciliation | Resolve root MediaCore versus FSTR FX installation path, identify conflicts without deleting unrelated plugins; validate only in an authorized test scope |
| Project lifecycle and interaction | BLOCKED: AE unavailable | Run guarded save/reopen/keyframes, drag/Undo/Redo, restart, repeated execution, cold/warm cache and migration on the exact candidate |
| Render compatibility | BLOCKED: AE/physical Metal unavailable | 8/16/32 bpc, alpha/HDR, ROI/downsample/PAR, CPU/Metal parity, actual MFR, render queue/aerender and cancellation |
| Target performance | BLOCKED: physical target unavailable | Real AE Render/RAM Preview and memory/Metal profiling; local noisy measurements are only sanity checks |
| User delivery | BLOCKED by gates above | Final clean immutable artifact and complete required evidence; do not rebuild an alleged identical replacement after verification |

## Next implementation order

1. Complete commit-specific CI and investigate any failures; retain source/verification evidence.
2. Verify the corrected host metadata and saved-ordinal regression on macOS/AE; do not equate source-contract guards with host migration acceptance.
3. Complete actual loaded Build ID verification, a patterned AE deformation test and consistent safe installation discovery. Generated metadata and source-contract tests are preparatory, not host acceptance.
4. Run the real target-Mac/AE/Metal gate in an authorized isolated environment. Until such execution is possible, retain BLOCKED status; do not ask the user to perform internal trial-and-error QA.
5. Only after every mandatory gate passes, identify and publish that exact candidate, update compatibility/release records and consider merging the reviewed source.

## Rules and evidence

Authoritative process: ../DEVELOPMENT_RULES.md. This status separates implemented changes, verified source behavior and unverified host behavior. No plugin is approved or handed over by this status update.
