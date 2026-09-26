# ElasticGrid FX — v1.0 Final Validation

This is the live gate before the next user-facing macOS `.plugin`. A step is marked PASS only after implementation/audit, executable verification, and documentation update. Mac-only behavior is not marked PASS until executed on the target Mac.

| Step | Gate | Status | Evidence |
|---|---|---|---|
| 1 | Fuzzing / malformed input hardening | **PASS** | `elasticgrid_fuzz_smoke_tests`: 180k deterministic randomized operations in Release; scaled sanitizer fuzz passes under ASan+UBSan with leak detection. |
| 2 | Leak / soak | **CPU PASS / MAC METAL SOAK PENDING** | 50k Release CPU/bridge cycles with stable post-warmup RSS; 3k ASan+UBSan+LSan cycles; expanded Metal lifecycle soak wired into Mac preflight |
| 3 | Determinism | **CPU/MFR PASS / MAC METAL PENDING** | exact repeated outputs across bit depths/quality/edges/thread counts + 384 concurrent CPU MFR renders; Metal sequential/concurrent gate added to Mac preflight |
| 4 | Allocation / lock audit | **CPU PASS / MAC METAL COMPILE-RUNTIME PENDING** | 0 steady-state CPU heap allocations after warm-up; no blocking mutex primitives in core/bridge/Metal render hot path; Metal uses atomic lifecycle + fixed CAS plan pool |
| 5 | Full regression | **PORTABLE PASS / RUST+METAL TARGET-MAC PENDING** | GCC strict 9/9; Clang strict 9/9; ASan+UBSan+LSan 9/9; TSan MFR+determinism 2/2; hot-path audit PASS; same-machine CPU perf vs clean v0.9 within noise |
| 6 | Mac runtime gate | **READY / TARGET-MAC EXECUTION PENDING** | Automated Metal lifecycle/parity/determinism/4K/8K + AE load/32-bpc render + temp AEP save/reopen/keyframe roundtrip are wired; viewer drag/Undo/Redo/downsample/PAR remain manual |

## Step 1 — Fuzzing

Added `tests/test_fuzz.cpp` and a CTest target named `elasticgrid_fuzz_smoke`. The harness uses a fixed seed (`0xE1A57C0FFEE12345`) so failures are reproducible.

Coverage in one normal run:

- 50,000 completely random byte blobs (0..2048 bytes) through `decodeGridState`;
- 10,000 mutations/truncations of valid encoded grid states;
- 100,000 randomized `AxisGrid` pin/position/elastic/wave operations including NaN, infinities, denormals and extreme finite values;
- 20,000 randomized bridge cases spanning plan creation, ROI calculation and small 8/16/32-bpc renders.

Invariants checked include finite/monotonic guides, fixed 0/1 boundaries, valid pin values, decode non-mutation on failure, codec roundtrip, valid source rectangles, and bounded public-ABI return codes.

Verification:

- Release CTest: **6/6 PASS** including the full 180k-operation fuzz smoke.
- Clang ASan+UBSan + LeakSanitizer: scaled randomized fuzz smoke **PASS**.
- No crash, hang, sanitizer error, or invariant violation observed.

`EG_FUZZ_SCALE` can scale iteration counts for sanitizer/CI environments while preserving the same deterministic input stream.

## Step 2 — Leak / soak

Added `tests/test_soak.cpp` and CTest target `elasticgrid_soak`. Each cycle performs state construction/serialization/deserialization, sampling-plan construction/destruction, an 8/16/32-bpc frame render, and an elastic guide edit.

Portable verification:

- Release long soak: **50,000 cycles PASS**. RSS after allocator warm-up was ~2.01 MiB at the first sample and ~2.06 MiB at midpoint/end in this Linux environment; no monotonic growth was observed. RSS is diagnostic, not the authoritative leak check.
- Clang ASan+UBSan + LeakSanitizer: **3,000 cycles PASS**, no leak report at process exit.
- Normal CTest soak defaults to 5,000 cycles; `EG_SOAK_CYCLES` raises/lowers the count for CI and deep runs.

Target-Mac Metal gate was strengthened rather than falsely marked PASS here:

- 128 complete `eg_metal_create -> render -> eg_metal_destroy` cycles;
- 8 worker threads × 64 renders on one shared Metal state (512 concurrent/MFR-style renders);
- 2,048 sequential renders on the same state to exercise plan-buffer reuse;
- `MTLDevice.currentAllocatedSize` is logged before/after shared-state destroy for diagnostics. Driver caches make this value non-authoritative, so resource correctness remains enforced by RAII/setdown plus the runtime soak rather than a brittle byte threshold.

The real Metal soak remains **target-Mac pending** until `tools/preflight_macos.command` runs.

## Step 3 — Determinism

Added `tests/test_determinism.cpp` plus a target-Mac `tests/test_metal_determinism.mm`.

Portable verification:

- 8/16/32-bpc renders are byte-identical across repeated runs.
- Bilinear and Final Bicubic are covered for Clamp, Wrap and Mirror.
- CPU worker counts 1, 2 and 4 produce the exact same bytes.
- 12 concurrent frame workers × 32 renders (384 MFR-style renders) match one exact reference buffer.
- CPU-built Metal sampling plans are byte-identical across 256 repeated builds.
- Determinism test passes in Release and under ASan+UBSan+LeakSanitizer.

Target-Mac gate:

- `tools/run_metal_determinism_macos.command` runs the production Metal entrypoint;
- 384 sequential GPU renders across quality/edge combinations must be byte-identical;
- 8 threads × 32 concurrent renders (256 MFR-style Metal renders) must match one exact GPU reference;
- this gate is part of the 16-stage macOS preflight and remains **target-Mac pending**.

## Step 4 — Allocation / lock audit

The audit found real avoidable hot-path overhead and it was removed rather than merely documented.

### CPU allocations

Before this pass, a steady-state deformed Bicubic bridge render at fixed dimensions performed **18 heap allocations per frame**. The sources were: temporary AxisGrid buffers, evaluated guide arrays, inverse LUTs, prepared sampling plans, and four horizontal bicubic row-cache vectors.

Changes:

- `PreparedBridge` is now thread-local and reuses all frame-preparation storage without sharing mutable state across AE MFR threads.
- `AxisGrid::setState` validates first and then reuses member capacity.
- added `AxisGrid::evaluatedInto`, `buildInverseLUTRangeInto`, and `prepareWarpRGBAfInto`;
- bicubic row scratch is thread-local per worker and only grows when the output width grows.

`tests/test_allocations.cpp` overrides global allocation entry points during a measured window. After warm-up it requires exactly **0 heap allocations** for:

- 200 repeated renders for each of 8/16/32 bpc × Bilinear/Bicubic;
- 500 repeated GPU sampling-plan builds;
- 500 repeated ROI/source-rect plans.

Result: **PASS — 0 allocations / 0 bytes** in every measured steady-state window.

### Lock/contention audit

CPU core and C bridge contain no blocking mutexes. Metal was also refactored:

- per-render lifecycle entry/exit now uses a single atomic state word (shutdown bit + active-render count);
- setdown uses C++20 atomic wait only during rare teardown;
- the dynamic mutex-protected plan pool was replaced with a fixed 64-slot pool claimed by atomic CAS;
- warmed plan buffers remain pooled, so the GPU hot path does not allocate image or plan storage per frame.

`tools/audit_hotpath.command` fails if blocking mutex primitives reappear in core/bridge/Metal hot-path sources and runs the allocation gate.

Portable gate: **PASS**. Metal compile/runtime verification of the atomic implementation remains part of the target-Mac preflight.

Current sampling-plan cost after the refactor is ~0.19 ms at 4K Bicubic and ~0.39 ms at 8K Bicubic in this environment.


## Step 5 — Full regression

Portable regression was rerun after the allocation/lock refactor instead of relying on earlier green results.

Verification:

- GCC Release, `-Werror`: **9/9 PASS**.
- Clang Release, `-Werror`: **9/9 PASS** (fuzz/soak iteration counts scaled for the repeated audit run).
- Clang ASan + UBSan + LeakSanitizer: **9/9 PASS**.
- GCC ThreadSanitizer: MFR + determinism **2/2 PASS**.
- `tools/audit_hotpath.command`: **PASS** — zero steady-state heap allocations after warm-up and no blocking mutex primitives in the audited render hot-path sources.
- All `.command` / shell scripts: `bash -n` **PASS**.
- Quality-reference, CPU/GPU plan parity, deterministic output, fuzz and soak tests are included in the 9-test suite above.

Same-machine CPU performance was compared against an untouched v0.9 tree using alternating runs and median timings:

| Case | Clean v0.9 median | Current median | Delta |
|---|---:|---:|---:|
| 4K 32-bpc Final Bicubic + AE-style abort polling | 19.398 ms | 19.517 ms | +0.6% |
| 4K 16-bpc Final Bicubic + polling | 23.526 ms | 23.634 ms | +0.5% |
| 4K 8-bpc Final Bicubic + polling | 28.577 ms | 28.117 ms | -1.6% |
| 4K 32-bpc Bilinear + polling | 8.641 ms | 8.640 ms | ~0.0% |
| 8K 32-bpc Final Bicubic + polling | 76.737 ms | 77.253 ms | +0.7% |

Those deltas are inside normal run-to-run noise for this shared Linux host; no meaningful CPU performance regression is observed. The raw alternating-run data is stored in `docs/perf-compare-v1.0.json`.

Limitation: `cargo` is not installed in this container, and macOS/Metal are unavailable. Therefore Rust AE-host compile/tests, Metal lifecycle/parity/determinism/performance and real AE runtime are deliberately **not** marked PASS here. They remain mandatory target-Mac gates.


## Step 6 — Mac runtime gate

The final target-Mac gate is fully wired but cannot be executed in this Linux container. No Mac-only result is marked PASS prematurely.

Automated on the target Mac:

1. C++ sanitizer/strict/fuzz/soak/determinism/allocation gates.
2. Metal lifecycle/soak, CPU↔Metal parity, Metal determinism and production-path 4K/8K benchmarks.
3. Rust AE-host Release tests and locked Cargo verification.
4. `.plugin` assembly, PiPL/entrypoint/dependency/signature verification and install.
5. `tools/ae_runtime_check_macos.command --full` from a new empty unsaved AE project:
   - effect enumeration/application by stable match name;
   - real 32-bpc Final Bicubic render;
   - temporary `.aep` save → close → reopen;
   - ordinary parameter values + Wave keyframes persistence;
   - real render after reopen.

Still manual because ExtendScript cannot reliably represent the exact viewer/custom-Arbitrary-data interaction semantics we need to certify:

- Grid Positions guide drag in Comp and Layer viewers;
- Undo / Redo after guide drag;
- 1/2 and 1/4 preview alignment;
- non-square pixel aspect ratio;
- precomp plus upstream buffer-expanding/resizing effect;
- visual equality of Grid Positions keyframes after save/reopen.

The candidate is not release-ready until the automated target-Mac gate and these manual viewer checks pass.
