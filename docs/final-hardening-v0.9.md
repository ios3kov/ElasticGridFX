# ElasticGrid FX v0.9 — Final Hardening Plan

This document is the live gate for the final test plug-in. A step is marked PASS only after code changes (if any), regression tests, and documentation updates are complete. Mac-only checks remain pending until executed on the target Mac.

| Step | Gate | Status | Notes |
|---|---|---|---|
| 1 | FFI / crash-safety | PASS | Release Rust panic catching enabled. C++ exceptions already contained in portable FFI. Exported Metal ABI now also catches C++ and Objective-C exceptions. Null/malformed FFI tests added. Portable 5/5 PASS. |
| 2 | Metal lifecycle | CODE PASS / MAC GATE PENDING | Fixed missing `MetalGpuData` owner (v0.8 macOS compile blocker). RAII exact-once destroy, in-flight render teardown barrier, repeated init/setdown and concurrent-render Metal test added to preflight. |
| 3 | Render cancel / abort | PASS | CPU cancellation is polled only on the AE calling thread between 2048-row batches; C++ workers never call AE. Dedicated cancel code maps to `InterruptCancel`. Metal polls before/after its short synchronous command buffer. |
| 4 | Edge cases | PASS | 1x1 8/16/32-bpc, max 128x128 topology, 8K plan sizing, negative time, extreme spacing/wave, NaN/Inf rejection/sanitization covered by tests. |
| 5 | AE spatial correctness | CODE PASS / MAC VIEWER GATE PENDING | SmartFX ROI/origin correction implemented; checkout source rect computed from exact warp sampling plan. Downsampled canvas dimensions are explicit. Partial ROI matches full-frame crop byte-for-byte in portable regression. Non-square-PAR/viewer placement remains a target-AE visual gate. |
| 6 | Project/keyframe compatibility | CODE PASS / AE RUNTIME GATE PENDING | Stable match name/parameter IDs documented; v0.8 GridState remains readable; v0.9 schema marker added without changing six-field layout; migration and roundtrip Rust tests added. Undo/redo + actual .aep save/reopen remain target-AE gates. |
| 7 | Full regression/audit | PORTABLE PASS / MAC METAL GATE PENDING | GCC + Clang `-Werror`, 5/5 Release, ASan/UBSan 5/5, GCC TSan MFR PASS, quality/plan parity PASS, CPU performance baseline recorded. Real Metal parity remains target-Mac-only. |
| 8 | Mac runtime test build | PACKAGE READY / TARGET-MAC PENDING | real Metal 4K/8K, build/sign/install/load/smoke |

## Step 1 verification

- CMake portable tests: 5/5 PASS.
- Bridge regression now verifies null input/output/params, invalid dimensions, NaN drag target, and non-mutation on rejected calls.
- `host-rust/build.rs` emits `cargo:rustc-cfg=catch_panics`; the AE host macro therefore wraps Release `EffectMain` in `catch_unwind`.
- Every exported Metal C ABI function contains both C++ and Objective-C exception barriers.

## Step 2 verification

- Found and fixed a real macOS host compile blocker: `MetalGpuData` was referenced but not defined.
- `MetalGpuData::Drop` now owns `eg_metal_destroy`; setdown only destroys the boxed GPU data, preventing double-free paths.
- `MetalState` tracks active renders and setdown waits for already-entered renders before destroying pipelines/plan buffers.
- Added `tests/test_metal_lifecycle.mm` and `tools/run_metal_lifecycle_macos.command`; Mac preflight is now 13 gated steps.
- Portable regression remains 5/5 PASS. Real Metal lifecycle execution remains part of final target-Mac gate.

## Step 3 verification

- Added an AE abort trampoline that directly invokes the host callback without allocation or Rust panics.
- CPU workers never call back into After Effects; the calling thread polls between bounded row batches.
- `eg_render_frame` returns status 5 for cancellation and the Rust host maps it to `ae::Error::InterruptCancel`.
- Metal checks abort immediately before committing work and after completion; Metal command buffers are not force-cancelled mid-flight.
- Regression test cancels a multi-threaded bicubic render after several host polls. Portable 5/5 PASS; ASan/UBSan PASS.
- Final performance tuning keeps a one-dispatch fast path when no host abort callback is present and uses 2048-row polling batches in the AE CPU path.

## Step 4 verification

- 1×1 identity render passes at 8/16/32 bpc in Bilinear and Bicubic modes.
- 128×128 guide topology and infeasible requested spacing are projected to a valid monotonic grid.
- 8K sampling-plan construction passes without full-frame allocation.
- Negative time and extreme finite wave parameters remain finite/monotonic.
- Core now rejects NaN/Inf guide edits and ignores non-finite wave inputs. Wave phase is computed in double and cycle-wrapped before `sin`.
- Portable suite 5/5 PASS; ASan/UBSan core + bridge PASS.

## Step 5 verification

- SmartFX `PF_LayerDef.origin_x/y` semantics are now represented explicitly in the render ABI.
- The inverse LUT is evaluated in full layer/canvas coordinates and then converted to the local checked-out input-world coordinates.
- `eg_required_source_rect` derives the source checkout from the actual Bilinear/Bicubic sample indices; interpolation support pixels are therefore included exactly.
- SmartPreRender uses that required source rect instead of unconditionally requesting the full source. On planning failure it falls back to full canvas for correctness.
- Portable regression renders a deformed frame once as full-frame and once as a non-zero-origin partial ROI; the ROI result matches the corresponding full-frame crop byte-for-byte.
- Uniform-grid partial worlds with matching non-zero origins preserve exact identity.
- ASan/UBSan bridge regression PASS.
- Remaining target-AE gate: visual alignment in Comp/Layer Viewer with non-square pixels, precomps and upstream buffer-expanding effects.

## Step 6 verification

- Effect match name remains `com.elasticgrid.fx.warp`. Existing parameter variant names are explicitly frozen.
- GridState schema versioning reuses the existing `columns: u16` slot, so the bincode struct field count/order remains identical to v0.8.
- Decoder accepts legacy raw columns 1..128 and current schema v1, and rejects unknown versions.
- Added Rust tests for current roundtrip, v0.8 legacy migration, unknown-version rejection, topology interpolation and wave cache dependency.
- Viewer drag continues to use `ArbitraryDef::set_value`, which marks `CHANGED_VALUE`; no hidden global state is mutated.
- Target-AE runtime still must confirm undo/redo and .aep save-close-reopen behavior.

## Step 7 verification

- GCC Release + `-Werror`: 5/5 PASS.
- Clang Release + `-Werror`: 5/5 PASS.
- Clang ASan + UBSan: 5/5 PASS with leak/error halting enabled.
- GCC ThreadSanitizer: concurrent MFR test PASS. The container's Swift Clang TSan runtime cannot link on Linux because it expects libdispatch symbols, so GCC TSan is the portable race gate here; target-macOS TSan remains in preflight.
- CPU/GPU sampling-plan parity and independent Final-quality reference tests: PASS.
- Static re-audit found and fixed two additional Metal boundary issues: pool-release cleanup could terminate from a `noexcept` destructor if a mutex operation threw, and GPU buffer size/pitch validation was incomplete.
- Host abort polling was tuned to 2048-row batches to retain cancellation while reducing production CPU dispatch overhead.
- Portable CPU regression baseline is recorded in `docs/performance-v0.9.md`; 4K 32-bpc Final is ~17.9 ms and 8K 32-bpc Final is ~68.8 ms in this environment.
- Real Metal image parity and Metal 4K/8K performance cannot be truthfully marked PASS until `tools/preflight_macos.command` runs on the target Mac.


## Step 8 verification

- Added `tests/ae_runtime_smoke.jsx` plus `tools/ae_smoke_test_macos.command` for a real After Effects 32-bpc Final Bicubic render after installation.
- Runtime smoke verifies effect enumeration by stable match name, application to a layer, required parameters, Wave + Mirror setup, and actual frame output.
- Safety gate refuses to run if the current AE project is saved, contains items, or reports dirty state; user work is never used as a smoke-test project.
- `tools/ae_runtime_check_macos.command --smoke` combines installed-bundle/signature/log inspection with the real render smoke.
- One-click build/install instructions now point to this runtime gate.
- Package-side checks can be completed here, but actual Metal execution, AE load, render, viewer alignment, Undo/Redo and `.aep` reopen remain honestly **target-Mac pending**.
- Exact target workflow is documented in `docs/mac-runtime-gate-v0.9.md`.
