# Changelog

## 0.9.4 Dev 4 — development, not released

- Affected Lines now evaluates the saved deformation itself. Existing animated
  Tension Radius values remain in their original stream and immediately influence
  old projects; their appearance may change, by explicit user decision. Grid keys,
  scalar keys and grid serialization remain unchanged. Native acceptance pending.
- Shared compact Smoothstep displacement field feeds viewer and render paths;
  dragging solves against the evaluated field, including fractional controls,
  smoothing and waves. Sampling quality modes remain unchanged.
- Cache version and visible Dev label now share the build script's development
  number. Previous Dev1/2/3 evidence is not transferred to this candidate.

## 0.9.4 Dev 1 — development, not released

- Simplify deformation UI: hide Follow Shape, Follow Strength, Smooth Stretch
  and Smooth Width. New effects use Smoothstep, full follow strength, full
  boundary smoothing and 25% smoothing width; retain saved legacy values/keys.
  Host visual/default/legacy acceptance is pending.

- Shared Mac/Windows UI: Grid Positions has no empty disclosure area, with inline
  Reset at the current time; collapsed Wave Animation section and Affected Lines.
  Reset matches Fit Layer dimensions, pill shape and theme
  colors on Mac AE25.6; final inset/panel-width and Windows checks remain pending.
- Hide Min Line Spacing and enable automatic safety spacing for new effects.
  A new appended compatibility flag preserves existing project spacing values
  and keys. New-instance defaults verified on Mac; old-project migration pending.
- Deformation Plane, Falloff, Edge Behavior and Render Quality disallow new
  animation. Existing animated-project compatibility remains pending.
- Preserve existing parameter identities, wave values and render math; resolve
  native-plane hidden streams independently of inserted UI controls.
- Windows validation requires matching per-run completion after cleanup and a
  decoded frame. AE acceptance of these changes is still pending.

- 2026-10-02: reconcile published0.9.3-perf.1 README; add current end-user guide,
  performance/release retrospective and reusable AE know-how transfer. Preserve
  historical evidence and unchanged release assets; no rebuild or new speed runs.

- 2026-10-02: 0.9.3-perf.1 promoted to Latest Release with unchanged assets;
  consciously adopt rules6.0.0, which removes certificate/notarization prerequisites.

- 2026-10-02: user reports clean-environment install and legacy animated
  Columns/Rows checks passed; record USER-REPORTED acceptance and promote
  existing 0.9.3-perf.1 release with the explicit signing exception.

- 2026-10-02: verified browser-downloaded quarantined public package loading and
  static baseline-project save/reopen/render parity; existing Mac scope only.
  See docs/public-install-project-check-2026-10-02.json.

## MFR disposition — 2026-10-02

- Complete six bounded MFR-requested-ON100 reproduction attempts on the original
  failed scene: three exact-control and three retained-candidate runs,360/360
  frames with exact output. No recurrence/new crash report. Close issue21 as
  not reproduced by explicit user decision; preserve original failure/unknown
  cause, with no native fix or general MFR certification claim.
- Restore accelerated candidate atomically and reopen the preserved user scene.

## Postrelease validation — 2026-10-02

- Verify one user-scene copy: selected-frame pixel parity after UI Undo and
  save/reopen, animated keys/settings retained, native GUI Preview functional.
  Preserve source locally and retain unchanged installed candidate. This bounded
  check does not close MFR #21 or certify broad release/migration gates.

## 0.9.3-perf.1 — experimental TEST prerelease published, 2026-10-02

- Publish unchanged tested0.9.3 Develop Build2 by explicit user decision
  without Developer ID/notarization. Omitted gates retain NOT DONE/NOT RUN;
  stable release readiness is not claimed. MFR issue #21 remains open.
- Retain public archive download/hash verification and package/install notes.

## 0.9.3-perf.1 — local validation package preparation, 2026-10-02

- Package unchanged tested0.9.3 Develop Build2 with validation notes, notices
  and exact identity. Verify extracted signature and installed-payload parity.
- Retain MFR issue #21 and Developer ID/notarization/distribution blockers.
  This is a package revision, not a public release or binary version bump.

## Unreleased — resumed performance development

- Complete four stability-only MFR-requested-ON candidate/control × enabled/
  bypass checks: all60 frames complete,120 independently decoded candidate
  frames and120 encoded-exact control pairs. Preserve unreproduced prior crash
  and unresolved cause; no speculative shipping patch. Recheck raw crash identity
  and retain accelerated candidate/verified backup after temporary transactions.
- Add a concise speed-comparison table with scope/Full-preset measurement limits.


- Close additional speed testing by direct human acceptance; preserve missing
  technical results and independent MFR incident. Record five candidate Full
  Preview runs and Full/Skip0 readback; retain accelerated candidate installed
  by explicit authorization, independently verify payload/signature/backup.
- Consciously update rules baseline to published5.1.0 / v5.1.0 after entrypoint,
  change and errata review; preserve historical evidence provenance.


- Verify actual candidate guide displacement, fresh image and Undo restoration
  in owned AE; record current Spacebar preset and preserve input/timing limits.
  Close owned scene, restore prior Fast Preview and original signed plugin.
  Retain60 exact decoded frames from the same-failing-AEP MFR diagnostic;
  intermittent crash cause and ordinary five-pair ON acceptance stay open.
  Retain one requested50%-CPU diagnostic with60 exact frames; both sampled
  100%/50% succeed, so no causal fix or new throughput claim.

- Record ten fresh-phase equal-zoom RAM Preview UI intervals and preserve all
  outliers; separate observed cache-fill bounds from first-playable/guide gates.
  Add guarded invalidation timestamps and reject nonnumeric phase inputs.
- Retain incomplete ordinary MFR-ON series and matched control crash diagnostics;
  verify240 complete-pair/19 partial/60 separate diagnostic RGBA16 frames exact.
  Keep ordinary MFR throughput and crash root cause unresolved.

- Pin all 23 GitHub Actions references to verified official commit identities;
  disable five unused checkout-credential defaults. Keep existing action majors,
  workflow behavior and validation gates unchanged.

- Record five fresh matched target-AE total-time pairs at Full/Final/32 bpc:
  median 65.87 -> 48.77 s for 60 frames; all 360 pair/warmup RGBA16 outputs exact.
  Keep total export time separate from cold/per-frame/RAM Preview acceptance.
- Add guarded two-turn RGBA16 queue-chain, fresh-phase and ordinary GUI preview
  fixtures. Verify ten chain frames exact, full-cache 30-fps playback, wave
  invalidation and Escape cancellation in actual AE; retain blocked guide-drag
  and incomplete quantitative RAM Preview timing explicitly.

- Prepare performance fixtures in separate guarded creation/finalization host
  turns and require actual automatic plane-binding readiness before saving.
  Refuse foreign/changed scenes and retain blocked preparation evidence.
- Verify actual 40-frame GUI plane/3D/AEP matrix and 60-frame plane-region RGBA16
  equality against original native source built with the same toolchain;
  retain older-artifact strict failures and separate diagnostic phase timings.


- Cache exact per-axis plane mapping/taps and horizontal sampling rows for
  eligible axis-aligned planes; retain general projective sampling elsewhere.
  Keep Final Bicubic, per-channel arithmetic order and saved/FFI compatibility.
- Add a plane-specific benchmark and byte-exact optimized/reference checks for
  all depths, sparse/expanded worlds, cancellation and independent frames.
- Preserve historical NaN payload selection on x86 by recomputing exceptional
  float channels with the original scalar sampling loop; retain finite HDR caches.
- Run new cache tests in sanitizer/TSan gates and trigger Rust host regression
  for all relevant native/host source changes. AE/RAM Preview speed acceptance
  remains separate from standalone renderer measurements.
- Require verified straight RGBA16 and explicit color context for new host
  performance fixtures; retain schema-1 historical fixtures. Verify actual PNG
  encoded depth/channels and preserve rejected render-process identity evidence.
  Do not accept incomplete/dithered RGB8 observations as target speed proof.
- Add optional, bounded render callback observations for phase attribution and
  fresh-render verification. Default builds do not log or add observer locks.
  Pin active Cargo features into Build Identity so diagnostic artifacts cannot
  share an identity with normal builds. Actual headless AE identity/legacy callback
  observations pass; they do not measure the optimized plane route.
- Separate ordinary and diagnostic AE effect Develop builds (2/3) from the
  accepted build 1 cache identity, without purging caches or changing saved data.
  Verify existing PiPL/GlobalSetup version propagation and fresh host callbacks.
- Declare Four Corners or footage Layer Plane performance workloads explicitly;
  record read-back geometry in schema 3 and reject wrong callback routes,
  dimensions/depth or incomplete frame coverage. Keep historical fixtures intact.
- Retain the strict RGBA16 failure against the accepted older-toolchain artifact.
  Same-current-toolchain accepted native source matches the candidate's 60 PNGs;
  this isolates a build difference on the legacy workload, without accepting
  plane output, target performance or RAM Preview.
- Adopt the user-selected central AE-Development-Rules baseline and resume
  Stage 8 without rewriting prior acceptance evidence.

## 0.9.1

- Refresh plane controls automatically when the owning layer switches to 3D.
- Deform antialiased native-text perimeter pixels with the automatic layer plane;
  preserve bounded Four Corners behavior and Grid Positions animation.
- Increment the AE effect version so upgraded projects invalidate old render caches.

## v1.0.0-dev — final validation cycle

- Target-Mac bootstrap hardening: the one-click build now detects Rust older than 1.85 or missing Clippy and installs/activates a user-local stable toolchain automatically. Dependency audit now fails if the committed `Cargo.lock` is missing instead of silently regenerating the graph.

- Tightened the hosted macOS gate after freezing dependencies: CI now verifies and consumes the committed `host-rust/Cargo.lock` with `--locked` instead of regenerating it.

- Froze the exact Cargo dependency graph validated by hosted macOS gate #34: committed `host-rust/Cargo.lock` from the green artifact, SHA-256 `5d77f2ce76302850bd390de5f44d34e772b3d451b706fab257e08fff998d957e`.

- Hosted macOS source gate #33 passed at `f3bca10`: all 20 preflight stages completed, including Clippy, sanitizers, TSan, dependency/license/RustSec/SBOM checks, locked Rust tests, two clean reproducible Release builds, bundle verification/signing, and artifact upload. Hardware Metal execution and real After Effects runtime remain physical-Mac gates.

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

- Metal benchmark now measures the exact production `eg_metal_render` path used by the AE host.
- Added required 8K Metal benchmark alongside 4K.
- Added strict Metal compile warnings (`-Werror`).
- Expanded Mac preflight to 12 gated steps with machine/toolchain capture.
- Added locked Cargo dependency verification.
- Fixed missing PiPL Support URL build-time metadata required by the Rust AE entrypoint.
- Fixed AE frame caching for procedural Wave animation with dynamic `NON_PARAM_VARY`; static warps can still cache normally.
- Added bundle validation: plist, AE entrypoints, PiPL strings, dylib dependencies, code signature and hashes.
- Added post-launch AE runtime/log checker.
- Build writes `dist/mac/preflight-report.txt` for reproducible diagnostics.
- Kept Final Catmull-Rom Bicubic and CPU/Metal parity thresholds unchanged.

## v0.7.0 — performance hardening

- Zero-copy AE GPU image path for Metal.
- MFR-safe Metal plan-buffer pool.
- Guaranteed SmartFX input check-in on error paths.
- MFR concurrency stress + sanitizers.
- CPU bicubic hot-path optimization without sampling-quality reduction.

## v1.0 final regression — 2026-09-27

- Re-ran the complete 9-target portable Release regression: PASS.
- Re-ran all targets under ASan/UBSan/LeakSanitizer: PASS with no leak report.
- Re-ran MFR and determinism under GCC ThreadSanitizer: PASS.
- Documented the container-specific Clang/Swift TSan libdispatch linker limitation; this is not a source failure.
- Re-ran a final 4K/8K CPU performance smoke after all hardening changes. The shared validation host is noisy, so target-Mac Metal measurements remain authoritative.
- Added `docs/final-regression-v1.0.md`.


## v1.0 code freeze — 2026-09-27

- Completed final portable regression after all hardening work.
- Froze functional source with aggregate hash `e8d043c7bbab6d7674887558c8c80d6afb55d02ba1e3e9a3cf087236074ddeb3`.
- Added `docs/code-freeze-manifest-v1.0.txt` with per-file SHA-256 hashes.
- Functional changes are blocked until target-Mac/After Effects runtime validation reveals a blocker.
- `Cargo.lock` remains a target-Mac generated/frozen artifact because Cargo is unavailable in the Linux validation container.


## v1.0 target-Mac gate ready — 2026-09-27

- Revalidated shell syntax for all build/install/runtime `.command` scripts.
- Reviewed the full runtime orchestration: build/install → Metal lifecycle/parity/determinism/bench → bundle verification → AE 32-bpc Final render → project save/reopen/keyframe persistence.
- Added `docs/mac-runtime-ready-v1.0.md`.
- Actual Rust/Metal/After Effects execution remains target-Mac pending.
