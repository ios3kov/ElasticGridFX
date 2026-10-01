# Bounded render callback observation

Stage 8 requires fresh render work to be separated from cached playback and PNG
encoding. The RGB8 process-start series cannot prove this: cache state was
uncontrolled and the outputs were not exact. Keep that rejected evidence.

Add an optional Cargo `render-diagnostics` feature, absent from default builds.
Observe the existing Render, SmartPreRender and SmartRender branches with Rust
monotonic timestamps; reuse existing callback arguments and SDK methods. No new
Adobe API, render path, parameter/state change, cache purge or worker thread.

Each process creates a new private temporary directory and exclusive CSV. Record
only selector, frame time numerator/scale, canvas/depth, selected CPU path,
phase endpoints and successful output/completion. Serialize writes on a test-only
mutex, then stop after any write failure. Never
record user project names, source paths or pixels. Cap records at 4096. Refuse
an existing directory/file; logging failure never changes effect results. Missing,
dropped or incomplete observations cannot establish a performance PASS. The
reader rejects malformed identities, phases and sequences; a full-cap file is
explicitly truncated. No-record files do not by themselves prove cache hits.

Feature-specific environment must participate in Build Identity, so an
instrumented and default artifact from the same source cannot share a Build ID.
Keep the diagnostic artifact distinct from the default validation candidate.

Use diagnostic callbacks to prove actual render invocation and attribute phase
cost. Logging perturbs timing; it is not the acceptance benchmark. Repeat final
wall-clock/RAM Preview observations with the uninstrumented exact candidate.
Externally observe range/cache completion, first playable frame and invalidation;
do not substitute cached FPS or a scripting command ID for generation time.

Validation: feature identity isolation, encoder/phase/cap tests, Rust default and
feature suites/Clippy, sealed distinct packages, exact host image identity and
decoded host pixels. Default builds must retain the native sampling contract.

The schema contains 17 fields: sequence, selector, rational frame time, canvas,
depth, route, output/completion flags, five phase endpoints, total and
process-relative callback start. Endpoints are cumulative nanoseconds. Optional
missing phases remain empty; success and output presence are independent.
Phase-delta summaries cover completed output callbacks only. Start/end overlap
is observed wall-time overlap, not proof of CPU parallelism or unique frames.
The current feature is validated for the agreed Unix/macOS target scope.

Source verification: default Rust 61 tests; feature Rust 65 tests and strict
Clippy; complete Python 246/246 (including native owned-process observations). Runtime, sealed package and target
pixel checks remain separate gates. Build/CI outcomes are recorded in the current
checkpoint, not implied by this design.

## Effect-cache identity

Accepted 0.9.3 and initial performance artifacts retained Develop build 1. The
[SDK guide's cache discussion](https://ae-plugins.docsforadobe.dev/effect-details/tips-tricks/#global-performance-cache-consideratons)
says the effect version participates in the Global Performance Cache key. This
is an additional confounder in the rejected RGB8 observations; reuse was not
proved. Continue with Develop build 2 for ordinary performance builds and build 3
for `render-diagnostics`, retaining product version 0.9.3. No global cache purge,
match-name/parameter/state change or image-quality change.

Verified pinned source: `pipl 0.1.1` `AE_Effect_Version` emits the same `pf_version`
into resource `eVER` and `PIPL_VERSION`. `after-effects 0.4.0` EffectMain sets
`PF_OutData.my_version` from that environment value in GlobalSetup. Builds 1/2/3
fit the existing nine-bit build field. Existing API/property only, no new host
call. Verify real packaged resource and generated compile values for both variants;
runtime callback evidence and uninstrumented timing remain required.


## Confirming the intended host route

The first actual observer run reports 60 successful 32-bpc `legacy_cpu` callbacks
for the saved footage fixture; it cannot measure the new plane optimization.
New fixtures must explicitly select Four Corners (bounded full-image rectangle)
or Layer Plane and retain read-back mode/corners/expected route in schema 3.
Historical schema 1/2 remains unchanged/readable. Validate observed output routes,
logical dimensions/depth and complete rational frame-time coverage against the
pinned new fixture. Wrong/missing routes or frame coverage reject attribution;
no-record logs do not prove cache hits. Changing test parameters is declared;
product defaults, quality and renderer selection are not changed.

Schema-3 source validation: Python 248/248 and 12 JSX control-flow suites PASS;
the route validator rejects wrong route/depth/geometry, fractional frame times,
incomplete frame coverage and unbounded frame ranges. Actual saved schema-3 AEP
creation remains blocked by Mac lock. Do not retag the existing schema-1 AEP.

The actual legacy pilot isolates a build difference: original-artifact repeat
and ordinary/observer candidate pair are exact; accepted native source rebuilt
with the current toolchain matches all 60 candidate PNGs. Older-toolchain versus
current-toolchain decoded equality stays FAIL (max 2/65535, no alpha difference).
This is not plane optimization acceptance or evidence of quality loss caused by
the new sampler. Preserve the failure and investigate precise FP/build conditions
if needed; do not relax the quality contract. See [sanitized records](performance-host-diagnostic-2026-10-02.json).
