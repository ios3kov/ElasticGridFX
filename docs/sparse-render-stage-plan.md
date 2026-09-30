# Transparent checkout correction — stage plan

Date: 2026-09-28. Baseline a871596; same native renderer as installed 6d3b846.
Rules rechecked: FSTR-Line DEVELOPMENT_RULES blob a1760fde8763f789b50b91c20407938b4fcaea4a.

## Acceptance fixed before implementation

Reproduce a mismatch between a compact source rectangle and its equivalent
zero-filled full logical canvas. Correct sparse CPU sampling without changing
legacy dense/ROI ABI semantics or emitting negative indices to the existing
Metal kernels. No full-canvas image allocation is allowed in the sparse path.

For 8/16/32 bpc and both qualities, sparse output must match the full zero-filled
reference (integer equality; float tolerance max 2.5e-5, identity exact).
Include opaque crop edges, negative/HDR float, positive/negative crop origins,
1-pixel input axes, padded/negative strides, output ROI and outside-canvas pixels,
repeat sparse/dense use of thread-local caches, cancellation and concurrency.
Keep Final Catmull-Rom and existing precision; no resolution/bit-depth fallback.

Use a separately named C ABI entry for sparse SmartFX semantics. Keep the frozen
160-byte EgRenderParams and existing eg_render_frame/eg_prepare_gpu_plan contract
unchanged. Stage the core implementation before host wiring; the new entry alone
cannot close the user's AE streak/Corner Pin or hardware GPU acceptance gates.
The host must prove the coordinate convention and full logical-source request
before opting in; an arbitrary missing ROI is not necessarily transparent.

Checks: baseline negative reproduction, strict GCC/Clang Release, ASan/UBSan,
existing tests including allocation/mfr/soak, new sparse allocation/concurrency
checks, serialized before/after benchmark sanity. Exact-commit CI source checks
must be read; macOS/AE checks unavailable here stay NOT RUN. No new installer,
user-machine mutation, main merge or production release in this stage.
