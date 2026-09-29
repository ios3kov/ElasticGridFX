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

## Native journal result, 2026-09-30

Research source 4df19c61993b6e497056f654a666a1e00d92ac1b, clean,
Build ID EGFX-eac4547855c370c540da1681, AE PID 40389:

- GlobalSetup: main, registration succeeds.
- Opening the owned native-text fixture: SequenceResetup main, 24 streams.
- SequenceResetup worker: no AEGP calls (guard exercised).
- Adding a second temporary effect: SequenceSetup main, Parameter error.
- The add/remove JSX experiment then reported AE internal verification failure
  `child not found in parent` at its cleanup line. No successful lifecycle
  initialization or successful cleanup is claimed; causal attribution needs
  a separate control experiment. The owned project was closed without saving.

Evidence: four private PID-bound journals copied to the local deliverable
`outputs/lifecycle-evidence-4df19c6/`. This is callback evidence, not complete
live-Mach-O acceptance or product PASS. Do not use this lifecycle path for
production automatic binding. Stage 9 remains OPEN; render/picking and direct
3D text integration remain unresolved.

Rollback receipt EGFX-update-dc594d4c00c544d7b2e9ba2b67cfcaca is ROLLED_BACK;
ordinary 1eed79a / EGFX-25e03a7ae1a9304311095c8e payload reverified byte-for-byte.

## Deferred observation experiment

Next hypothesis: parameters are readable after the apply operation returns,
using a registered idle hook, not from SequenceSetup. Setup now only sets an
atomic request; no PF handle survives the callback. One main-thread idle attempt
per process reacquires the active layer and its second effect, restricted to
the named owned text fixture and exact match name. It reads only stream count,
disposes the acquired effect reference, logs once, and never changes parameters
or expressions. Idle sleep scheduling is untouched. No continuous project scan.

Acceptance: identified research candidate; add returns successfully; deferred
journal reports 24 streams; removal in a separate JSX call succeeds; close the
owned fixture without saving and restore ordinary installed payload. Tests must
not reuse Property references across calls. Default build excludes all hooks.
This experiment does not prove expression writes, Undo, non-interactive first
render or a supported production initialization mechanism. Native run pending.
