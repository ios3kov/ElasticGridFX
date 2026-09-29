# Automatic 3D initialization: read-only lifecycle experiment

Goal: establish whether effect streams are available during SequenceSetup and
SequenceResetup on the main thread. This is not permission to mutate parameters
in these callbacks and is not a production binding implementation.

Opt-in RUSTFLAGS: --cfg fstr_lifecycle_probe. Build identity records RUSTFLAGS.
Default build excludes the module and global field entirely. The experiment
registers an AEGP ID once at GlobalSetup, then reads parameter stream count only
on the main thread. Worker callbacks do not acquire any suite. Effect handles
are explicitly disposed even if reading fails. No expressions, project values,
files or render buffers are written. Latest four records appear only in About,
headed LIFECYCLE RESEARCH ONLY. No polling or render-path diagnostics.

Acceptance: clean identified research candidate; owned fixture; observe setup
and resetup records; return to original installed 1eed79a after the experiment.
Unknown or failed callback evidence is not a product PASS. Automatic 3D and
render/picking integration remain unresolved. No binding button is planned.
