# Stage 8 research — Render / RAM Preview performance

Date: 2026-09-29
Current production stage: 7 of 10. This is parallel research/preparation only.
No performance implementation is approved before the Stage 7 target pixel baseline
is accepted. Authoritative quality contract: performance-quality-contract.md.

## Sources checked

Adobe automated rendering:
https://helpx.adobe.com/after-effects/using/automated-rendering-network-rendering.html

Relevant documented aerender capabilities:
- render a named composition or current render queue;
- choose start/end frames and output;
- print version/build information;
- run as a new render process by default;
- `-reuse` can request use of an already-running After Effects instance;
- progress/error output is available for retained timing/evidence.

Adobe Multi-Frame Rendering:
https://helpx.adobe.com/after-effects/desktop/render-and-export/multi-frame-rendering/multi-frame-rendering.html

Relevant behavior:
- MFR affects both Preview and Export;
- performance depends on CPU cores, RAM and GPU/resources;
- third-party effects need actual MFR compatibility; a flag alone is not proof.

After Effects Scripting Guide:
https://ae-scripting.docsforadobe.dev/general/application/
https://ae-scripting.docsforadobe.dev/other/viewoptions/

Useful observability/control:
- `app.memoryInUse` reports current AE memory;
- `app.setMultiFrameRenderingConfig()` can configure MFR for the next render,
  then AE restores the previous UI setting;
- `ViewOptions.fastPreview` exposes viewer Fast Preview state, so the harness
  can assert Final-quality viewer mode instead of silently accepting Draft/
  Adaptive/Fast Draft;
- `app.executeCommand(id)` can trigger menu commands not otherwise exposed;
  however command lookup by label is documented as unreliable across language
  packages. No stable dedicated `ramPreview()` scripting method was found.

The scripting guide is community-maintained from Adobe documentation. Any
release-critical behavior found only there must be confirmed on the target AE.

## Decision: do not automate RAM Preview via menu-command magic

A hard-coded Preview menu command ID or localized `findMenuCommandId()` would
make benchmark validity depend on UI language/version/focus. That violates the
controlled/reproducible testing requirement.

Therefore:
- do NOT use `executeCommand()` as the release oracle for RAM Preview;
- do use it only in a disposable exploratory fixture if a later target-host
  investigation proves one exact AE-version command contract and we retain a
  fallback/validation check;
- prefer direct host/plugin instrumentation plus externally observed preview
  completion/cache state for final RAM Preview profiling.

## Stage 8 measurement architecture

### A. Render Queue / aerender — fully automatable

Use a dedicated synthetic performance project stored entirely under a unique
test workspace. Do not touch user projects/preferences.

Fixture matrix:
- 1920x1080 and 3840x2160;
- 32 bpc Final Bicubic first, then 8/16 bpc parity runs;
- static deformed grid and animated wave;
- full-frame and sparse/transparent-content cases;
- fixed frame rate/range;
- MFR on/off controlled explicitly where supported.

Record for every run:
- plugin candidate Build ID and package hash;
- AE/aerender version/build;
- macOS/hardware summary;
- composition geometry/frame rate/bit depth;
- MFR state/CPU percentage;
- cold/warm classification;
- wall time, per-frame progress times when available;
- process CPU time and peak resident memory from macOS process metrics;
- output frame hashes;
- stdout/stderr and exact command line without sensitive paths.

Protocol:
1. Warmup runs not counted.
2. At least five measured runs per condition.
3. Alternate baseline/candidate order serially.
4. No concurrent CI/profile jobs on the same target.
5. Compute median, p50/p95 and spread.
6. A >5% reproducible slowdown is an investigation gate, not permission to
   lower quality.
7. Any output mismatch outside existing pixel tolerance invalidates the speed
   comparison.

### B. Interactive fresh-frame latency — automatable in target AE

Extend the guarded target runner after Stage 7:
- synthetic project only;
- verify active viewer is Composition and Fast Preview is FP_OFF;
- set a known frame/time/parameter state;
- capture timing around explicit frame renders already used by the acceptance
  runner;
- measure first frame and warmed repeated frames;
- change one guide/wave parameter and measure cache invalidation/fresh frame;
- record `app.memoryInUse` before/after as supporting data.

This measures interactive render latency, not full RAM Preview playback.

### C. RAM Preview — target-host instrumentation required

Final evidence must measure:
- time to first playable frame;
- time to build/cache a fixed range;
- invalidation/rebuild after parameter change;
- cached playback behavior separately from render-build time;
- memory growth/peak.

Because there is no stable direct scripting Preview API, the preferred design is:
1. add diagnostics-only timing counters around ElasticGrid render callbacks;
2. observe AE preview lifecycle/cache completion externally on the target host;
3. keep instrumentation compile-time/test-only or otherwise proven negligible;
4. repeat with the uninstrumented candidate for wall-clock confirmation;
5. never claim cached playback FPS as renderer throughput.

Do not purge the user's global RAM/disk caches automatically. Cold-cache
experiments must use a dedicated test session/workspace or explicit user
permission for the exact purge operation.

## Optimization hypotheses to measure, not assume

Investigate in order only if profiling attributes cost there:
1. full-canvas checkout cost versus minimum proven source ROI;
2. repeated grid/LUT preparation across unchanged frames;
3. bicubic row-cache efficiency and TLS memory growth;
4. per-frame allocations;
5. GCD/thread fan-out versus AE MFR oversubscription;
6. redundant copies / format conversions;
7. CPU cache/memory bandwidth;
8. Metal upload/dispatch/readback only after physical GPU correctness gate.

The current disabled AE GPU dispatch remains disabled until real host parity,
lifecycle and cancellation behavior are verified.

## Acceptance for Stage 8 completion

Stage 8 is complete only when:
- real target AE Render/aerender and RAM Preview baselines exist;
- bottlenecks are measured;
- applied optimizations show repeatable improvement outside noise;
- Final Bicubic/bit-depth/alpha/HDR correctness is unchanged;
- no hidden lower resolution/filter/precision path is used;
- peak memory and responsiveness do not regress unacceptably;
- raw timings and environment metadata are retained;
- documentation records Before -> Change -> After.

Research alone does not advance Stage 8 to PASS.

## Prepared benchmark tooling

Source-only benchmark tooling is now available for the future Stage 8 target run:
`tools/aerender_benchmark.py` and `tools/perf_compare.py`.

The harness:
- refuses to run without a pinned fixture manifest and project hash;
- requires the fixture project to live inside an explicit controlled workspace;
- never edits the AEP, preferences or caches;
- creates a fresh unique output directory per aerender run and never deletes old
  output;
- requires Final Bicubic and 8/16/32-bpc fixture metadata;
- observes the actual runtime `ElasticGridBuildID=...` marker from aerender;
- uses macOS `/usr/bin/time -l` plus independent wall clock;
- records peak RSS, user/sys time, output hashes and retained local raw logs;
- requires at least five measured samples after optional warmups;
- comparison refuses different fixture/MFR settings and never upgrades timing
  evidence into a quality claim.

CI exercises parser/safety/comparison contracts only. Actual aerender measurements
remain NOT RUN until Stage 7 passes and a controlled target fixture is generated.

## Synthetic fixture generator

The benchmark never needs a user project. `tools/perf_fixture_runner.py` plus
`tests/perf_fixture.jsx` prepare a dedicated 2-second synthetic project in a
unique workspace, but only when the currently open AE project is empty, unsaved
and clean.

The fixture uses:
- 1920x1080 or 3840x2160 at 30 fps;
- explicit 8/16/32-bpc project depth;
- Final Bicubic ElasticGrid with 8x8 guides and static or animated deformation;
- documented Render Queue `items.add(comp)` / template APIs;
- exact local templates `Best Settings` and `PNG Sequence`; absence is BLOCKED,
  never silently substituted;
- a render-queue item pinned to 60 frames and a PNG sequence output inside the
  fixture workspace;
- `Project.save(File)` to create `EGFX_PERF.aep` without a save dialog.

After the AEP is saved, the JSX closes only the synthetic project it owns and
creates a fresh empty project, restoring the previous bit depth. Mock safety
tests cover saved/dirty/occupied projects, missing effect/templates and cleanup
failure. Actual target AE fixture generation remains NOT RUN until Stage 7 passes.
