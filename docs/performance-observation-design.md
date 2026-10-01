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
