# Automatic 3D initialization: read-only lifecycle experiment

Goal: establish whether effect streams are available during SequenceSetup and
SequenceResetup on the main thread. This is not permission to mutate parameters
in these callbacks and is not a production binding implementation.

Opt-in RUSTFLAGS: --cfg fstr_lifecycle_probe. Build identity records RUSTFLAGS.
Default build excludes the module and global field entirely. The experiment
registers an AEGP ID once at GlobalSetup, then reads parameter stream count only
on the main thread. Worker callbacks do not acquire any suite. Effect handles
are explicitly disposed even if reading fails. No expressions, project values,
or render buffers are written. Latest four records appear in About,
headed LIFECYCLE RESEARCH ONLY. No polling or render-path diagnostics.

Acceptance: clean identified research candidate; owned fixture; observe setup
and resetup records; return to original installed 1eed79a after the experiment.
Unknown or failed callback evidence is not a product PASS. Automatic 3D and
render/picking integration remain unresolved. No binding button is planned.

First native attempt: clean candidate 8052b37, EGFX-378815573231494e31310901
installed reversibly, owned fixture opened. About could not be accessed with
available native UI automation (canvas click returns AXError.notImplemented).
No callback record was observed: lifecycle result is BLOCKED, not PASS.
Receipt EGFX-update-a7ede1027c394dca960dc07f00338ec1 is ROLLED_BACK;
original EGFX-25e03a7ae1a9304311095c8e installed payload reverified.
Default Rust tests 24/24, host contracts 9/9 PASS; opt-in compilation PASS.
Scanner remains review_required (existing workflow/test findings).

Follow-up instrumentation also writes at most four private create-new temporary
files fstr-lifecycle-PID-INDEX.txt, containing Build ID and callback result only.
It never overwrites an existing path and makes no render-path file calls.
This reporting revision needs a new clean research candidate/native run; the
previous experiment cannot verify it. Automatic binding remains unimplemented.
