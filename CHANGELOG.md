# Changelog

## v1.0.0-dev — final validation cycle

- Fixed the macOS bundle-metadata gap exposed by the hosted gate: pinned `pipl 0.1.1` emits the PiPL `.rsrc` but not the `*_PkgInfo` / `*_Info.plist` files expected by the bundle script. The project build script now deterministically generates those two metadata files locally without changing the pinned dependency.

- Reproducible-build gate hardening: the two clean offline Release builds now reuse the same canonical `CARGO_TARGET_DIR` after a full target wipe. This keeps build settings identical instead of accidentally testing Cargo/rustc path variance between `target-a` and `target-b`. On a real mismatch the gate now prints hashes, first differing byte offsets, and Mach-O UUIDs for the plugin dylib.

- Fixed malformed compile-only insertion in the Metal helper scripts and added a mandatory CI `bash -n` audit for every `.command`/`.sh` script.

- GitHub-hosted macOS runner has no MTLDevice. Added a CI-only Metal compile mode: shader is compiled by Apple's `metal/metallib` tools and lifecycle/parity/determinism/benchmark binaries are built, while hardware execution remains mandatory and unskipped on the physical target Mac.

- macOS Clippy cleanup: removed redundant Rust casts and reduced the float-slider helper argument count by grouping valid/slider ranges.
- Rust 1.98 reports `clippy::drop_non_drop` and `clippy::question_mark` from inside the pinned `after-effects 0.4.0` item-macro expansion. Those two known upstream style lints are the only command-line Clippy exceptions; every other warning remains `-D warnings`.

- First real macOS Rust-host compile found and fixed a plugin-trait ABI/API mismatch: `handle_command` now uses the required mutable receiver (`&mut self`).
- Registered the cfg names expanded by `after-effects 0.4.0` (`does_dialog`, `with_premiere`, `threaded_rendering`, `catch_panics`) with rustc check-cfg so mandatory Clippy can remain at `-D warnings` on modern Rust.

- macOS hosted-runner static analysis: excluded Apple Clang's environment-only `-Wpoison-system-directories` diagnostic while retaining `-Werror` for source diagnostics.

- Added GitHub CI for GCC/Clang regression, portable C++ static analysis, ASan/UBSan/leak checks, and GCC TSan. Rust Clippy is target-correctly kept in the mandatory macOS source gate because the pinned after-effects host crate is not a Linux host crate.

- Code-freeze step 4 reproducible-build gate added: clean temporary source snapshot, isolated Cargo registry, locked fetch, two independent offline Release builds, byte-for-byte comparison of dylib/PiPL/PkgInfo/plist, plus offline locked tests. Portable C++ static core reproduces byte-for-byte here; macOS host result remains target-Mac pending.
- Code-freeze step 3 dependency/license policy complete in source: exact-pinned direct Rust dependencies, explicit Rust 1.85 MSRV, automated target-Mac Cargo.lock generation, RustSec vulnerability gate, CycloneDX 1.5 SBOM, dependency/license inventory and compliance hashes.
- Documented `RUSTSEC-2025-0141` for `bincode` as an informational/unmaintained advisory; it remains visible in release compliance output and is not silently ignored. Full target-Mac lock/audit/SBOM remains pending until Cargo runs there.
- Code-freeze step 1 static analysis complete for portable C++: Clang high-warning audit and Clang Static Analyzer pass after fixes.
- Replaced naked exact float comparisons in identity/uniform fast paths with explicit bit-exact comparisons; removed signed array-index conversions and weak-vtable noise; added defensive enum fallbacks.
- Added `tools/static_analysis.command`; target-Mac preflight now makes `cargo clippy --all-targets -- -D warnings` mandatory and runs `clang-tidy`/`cppcheck` when available.
- Rust Clippy remains target-Mac pending because Rust/Cargo are unavailable in this Linux container.
- Code-freeze step 2 manual unsafe/FFI audit complete in source: pinned 64-bit ABI layout on C++ and Rust sides, hardened extreme ROI subtraction, bounded hostile GridState vector deserialization, removed production arbitrary-data allocation `unwrap()`, documented Metal `Send/Sync` safety, and corrected Metal runtime error classification.
- Added hostile project-state + ABI-layout regression tests; portable C++ suite and static analyzer remain green. Rust compilation of these edits remains a mandatory target-Mac gate.
- Final validation item 5 portable regression complete.
- GCC Release strict (`-Werror`) full suite: 9/9 PASS.
- Clang Release strict (`-Werror`) full suite: 9/9 PASS.
- Clang ASan+UBSan+LeakSanitizer full suite: 9/9 PASS.
- GCC ThreadSanitizer: MFR + determinism 2/2 PASS.
- Hot-path allocation/lock audit and shell-script syntax audit PASS.
- Same-machine alternating performance comparison against clean v0.9 shows no meaningful CPU regression; measured medians range from ~1.6% faster to ~0.7% slower across the selected 4K/8K cases.
- Added raw comparison data (`docs/perf-compare-v1.0.json`) and portable verification record (`docs/verification-v1.0.txt`).
- Rust AE-host compile/tests and all Metal/After Effects runtime claims remain target-Mac pending because cargo/macOS are unavailable in this container.
- Final validation item 6 target-Mac runtime gate prepared: added safe temporary `.aep` save/close/reopen roundtrip automation with parameter + Wave-keyframe persistence checks and a 32-bpc render after reopen.
- `ae_runtime_check_macos.command --full` now chains load/log inspection, real render smoke and project roundtrip. Viewer guide drag, Undo/Redo, downsample/PAR and upstream-resize checks remain explicit manual gates.

- Final validation item 1 complete: deterministic fuzzing.
- Added `elasticgrid_fuzz_smoke_tests` covering malformed/random GridState payloads, guide edits, NaN/Inf/extreme values, sampling-plan creation, ROI planning, and small 8/16/32-bpc renders.
- Normal fuzz smoke executes 180,000 deterministic randomized operations from a fixed reproducible seed.
- Added `EG_FUZZ_SCALE` so sanitizer/CI runs can keep broad randomized coverage without excessive runtime.
- Fuzz smoke passes in Release and under Clang ASan+UBSan with leak detection enabled.
- Failed GridState decodes are explicitly verified to leave the destination state unchanged.
- Documentation updated with the v1.0 final-validation gate.
- Final validation item 2 CPU-side complete: leak / soak.
- Added `elasticgrid_soak_tests`: repeated GridState setup/codec, sampling-plan allocation, 8/16/32-bpc render, elastic edit and teardown.
- 50,000 Release soak cycles keep Linux RSS effectively flat after warm-up (~2.01 MiB -> ~2.06 MiB in this environment).
- 3,000 soak cycles pass Clang ASan+UBSan with LeakSanitizer enabled; no leak report.
- Expanded target-Mac Metal lifecycle gate to 128 setup/setdown cycles, 512 concurrent renders and 2,048 pooled shared-state renders; real GPU-memory result remains target-Mac pending.
- Mac preflight now includes fuzz and CPU soak gates (15 total stages).
- Final validation item 3 portable complete: determinism.
- Added `elasticgrid_determinism_tests`: byte-for-byte output equality for 8/16/32 bpc, Bilinear/Bicubic, Clamp/Wrap/Mirror, and 1/2/4 CPU worker counts.
- Added 384 concurrent same-frame MFR renders; all match one exact reference buffer.
- Added repeated CPU->GPU sampling-plan determinism checks (256 plan rebuilds).
- Added a target-Mac Metal determinism gate: 384 sequential GPU renders across quality/edge combinations plus 256 concurrent MFR-style GPU renders must be byte-identical.
- Mac preflight expanded to 16 stages.
- Final validation item 4 portable complete: allocation / lock audit.
- Refactored render preparation to reuse thread-local AxisGrid, evaluated-line, LUT and prepared sampling-plan buffers.
- Added in-place `evaluatedInto`, `buildInverseLUTRangeInto` and `prepareWarpRGBAfInto` APIs to reuse capacity.
- Bicubic horizontal row caches are now thread-local/reused instead of allocating four row buffers per worker on every frame.
- Measured steady-state CPU Final path dropped from 18 heap allocations/frame to **0** after warm-up for fixed geometry.
- Added `elasticgrid_allocation_audit_tests`, which gates 8/16/32-bpc Bilinear/Bicubic render, GPU-plan build and ROI planning at zero steady-state heap allocations.
- Replaced Metal render-entry lifecycle mutex with a shutdown/count atomic protocol; replaced the dynamic mutex-protected plan pool with a fixed 64-slot CAS pool.
- Added `tools/audit_hotpath.command`; render hot-path source now contains no blocking mutex primitives.
- 4K/8K Bicubic sampling-plan build remains ~0.19/~0.39 ms in this environment.
- Mac preflight expanded to 17 stages.

## v0.9.0-dev — final hardening

- Item 1 complete: FFI / crash-safety.
- Enabled `catch_panics` for Release AE entrypoints so Rust panics cannot unwind across the After Effects C ABI.
- Added Objective-C exception and C++ exception containment around all exported Metal ABI functions.
- Added malformed/null FFI regression coverage; portable suite remains 5/5 PASS.
- Documentation now tracks the eight-item final hardening plan explicitly.
- Item 2 code complete: Metal lifecycle.
- Added missing `MetalGpuData` owner type (this was a real macOS compile blocker in v0.8).
- Native Metal state is now RAII-owned by Rust GPU data and destroyed exactly once on setdown/drop.
- Added teardown barrier that waits for already-entered GPU renders before releasing pipelines/buffer pools.
- Added repeated setup/setdown + concurrent MFR Metal lifecycle test to the Mac preflight.
- Item 3 complete: render cancel / abort.
- Added AE abort callback bridge with no host callback calls from worker threads.
- CPU rendering is split into bounded row batches and returns a dedicated cancellation code mapped to `PF_Interrupt_CANCEL`.
- Metal polls AE before dispatch and after command completion; Metal work itself is not unsafely cancelled mid-command-buffer.
- Added cancellation regression test; ASan/UBSan remains clean.
- Item 4 complete: edge-case hardening.
- Hardened core numeric handling for NaN/Inf drag/easing/min-spacing/wave inputs.
- Wave phase now evaluates in double and wraps cycles before `sin`, avoiding overflow at very large finite time/frequency values.
- Added 1x1 renders for 8/16/32 bpc, max 128x128 topology, negative-time/extreme-wave, and 8K sampling-plan tests.
- Item 5 code complete: AE spatial correctness.
- Fixed a major SmartFX ROI bug: checked-out subregions are no longer treated as if they were full frames.
- Added full-layer canvas dimensions plus input/output world origins to the render ABI.
- Added ROI-aware inverse LUT generation and exact source-rect computation including interpolation support pixels.
- SmartPreRender now requests only the source rectangle actually needed for the requested output ROI, with a conservative full-canvas fallback on any planning error.
- Result/max-result bounds are no longer inherited from potentially smaller input alpha bounds.
- Added partial-ROI vs full-frame pixel-exact regression and non-zero-origin identity regression.
- Item 6 code complete: project/keyframe compatibility.
- Added backward-readable GridState schema marker without changing the historical six-field serialized layout.
- v0.9 reads v0.8 plain-column GridState data and rejects unknown future schema versions rather than misreading them.
- Added Rust migration/roundtrip tests to the Mac cargo preflight.
- Froze existing parameter variant names because host parameter IDs derive from those names.
- Bumped development/plugin bundle version to 0.9.0.
- Item 7 portable audit complete: GCC/Clang `-Werror` 5/5, ASan/UBSan 5/5, GCC TSan MFR PASS, quality/plan parity PASS.
- Reworked CPU abort scheduling: no-callback callers keep a single dispatch; AE cancellation polling now uses 2048-row batches to minimize hot-path overhead.
- Hardened Metal pool cleanup so a `noexcept` lease destructor cannot terminate the host on a mutex exception.
- Added Metal row-pitch/buffer-size/overflow validation and command-queue/device consistency checks.
- Recorded v0.9 portable CPU performance baseline; real Metal parity/performance remains a target-Mac gate.
- Item 8 package pipeline ready: added safe automated After Effects runtime smoke with a real 32-bpc Final Bicubic frame render.
- Runtime smoke refuses to touch existing/dirty AE projects and must run from an empty unsaved project.
- Added `ae_runtime_check_macos.command --smoke` and `docs/mac-runtime-gate-v0.9.md`; real target-Mac/AE execution remains pending until run by the user.

## v0.8.0 — final Mac preflight candidate

- Metal benchmark now measures the exact production `eg_metal_render` path used by the AE plugin, not a synthetic GPU kernel.
- Added 4K and 8K measured output with cold/warm GPU initialization split.
- Performance targets are now expressed as CPU-relative parity targets rather than unrealistic fixed millisecond budgets.
- Added production-path Metal/CPU image parity test with byte-exact 8-bit identity coverage and tolerance-bounded warped coverage.
- Added row-by-row diff metrics and finite-output validation.
- Mac preflight now runs real Metal lifecycle, parity and 4K/8K performance as mandatory gates.
- Added `docs/macos-final-preflight-v0.8.md`.

## v0.7.0 — production Metal renderer

- Replaced the previous CPU-fallback GPU callback with a real Metal compute renderer.
- `metal_backend.mm` now owns an `MTLDevice`, command queue, runtime-compiled `warp.metal` pipeline, and reusable buffer pools.
- Added Bilinear/Bicubic sampling, Clamp/Wrap/Mirror edges, 8/16/32-bpc conversions and exact identity path in Metal.
- Added CPU-side sampling-plan ABI so Metal uses the same warped source-coordinate plan as the CPU path.
- Added real `GpuDeviceSetup` / `GpuDeviceSetdown` handling and passes AE GPU device context into the backend.
- Added `tools/test_metal_lifecycle.command` and `tools/bench_metal.command` to the Mac preflight.
- Added source-level Metal lifecycle and allocation audits; real device execution remains a macOS gate.
- Added `docs/performance-v0.9.md` placeholder for measured GPU results.

## v0.6.0 — performance + stress validation

- Added AVX2/NEON SIMD bulk pixel conversion helpers with scalar fallback and runtime CPU detection.
- Added 4K/8K randomized high-resolution stress tests with ROI equivalence checks.
- Added stronger MFR determinism tests: 256 concurrent same-frame renders + 120 mixed-state concurrent renders.
- Added 4K/8K benchmark executable and `tools/benchmark.command`.
- Added `tools/profile.command` for Linux perf profiling.
- Added `docs/performance-audit-v0.6.md`, `docs/performance-targets.md`, and `docs/verification-v0.6.txt`.
