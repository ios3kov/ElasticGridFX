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

## Owned fixture readiness — 2026-10-02

Actual 40-frame GUI matrix passes on accepted original, ordinary f611312 and
same-current-toolchain original native source. Original older-toolchain versus
candidate has 29 changed color channels in six frames (max 2/65535, alpha exact).
Same-current-toolchain original source versus candidate is decoded RGBA16 exact
for all 40 frames. Cross-toolchain FAIL remains recorded separately.

The first schema-3 AEP was saved/closed within one host turn. Headless execution
on both original and candidate produced no frames and a BadCallbackParameter
516 warning despite exit 0. After opening only that owned AEP through regular
AE UI, the hidden plane marker reads 1, expression enabled/error-free, mode 2.
The existing tests/ae_native_plane_final.jsx explicitly separates creation and
capture into host turns for the deferred idle binding; the performance fixture
must follow that same contract. This is fixture readiness, not permission to
write hidden expressions manually or bypass the pending-render rejection.

Plan: split creation/finalization into separate guarded native script calls;
leave only the structurally owned synthetic scene open after creation. Match
UUID composition comment, exact imported pattern, one layer/effect and expected
hidden stream names/enabled/error-free marker before applying deformation and
saving. Missing/not-ready binding refuses a PREPARED fixture. Preserve the
failed first AEP and pilot logs; create a fresh workspace. No filter, bit depth,
renderer default, saved plugin state or production API change. Reuse verified
create/openInViewer/comment/expression checks from ae_native_plane_final.jsx
and ae_binding_probe.jsx; existing save/close/output APIs remain unchanged.

API-SOURCE-001 review for the fixture: the community-maintained guide derived from Adobe scripting documentation
[FootageItem.file](https://ae-scripting.docsforadobe.dev/item/footageitem/#footageitemfile)
defines a read-only File/null source, used only for owned-input verification.
[CompItem.openInViewer](https://ae-scripting.docsforadobe.dev/item/compitem/#compitemopeninviewer)
and [Property.expressionError](https://ae-scripting.docsforadobe.dev/property/property/#propertyexpressionerror)
match the existing native-final probe's viewer/dependency-readiness checks.
These are ExtendScript object APIs on the AE 2025/25.6 target, not new PF render
calls. Actual target execution is required separately from mocks/docs.

Two guarded host turns now produced a fresh binding-ready schema-3 AEP in
actual AE 25.6. Its initial original-source diagnostic run validates all 60
plane_region callbacks and straight RGBA16 headers. Original installed payload
was restored and verified after the run. Instrumented time is not acceptance.

Next timing control: build the same accepted native source with the current
toolchain and diagnostics disabled, using research Develop build 5 to separate
its existing AE effect cache identity from accepted 1, candidate 2 and observers
3/4. Only the existing PiPL version literal changes; no native source or Adobe
API change. Retain prior original-source gates and run the complete clean Mac
build/repro/package checks for the new exact source. Validate source/quality
before matched timing; restore the original after every bounded experiment.

Legacy chain harness plan: retain the historical saveFrameToPng failure on both
accepted and candidate artifacts. Add a separate owned empty-project fixture
with creation and capture in different host turns, so Adjustment Layer binding
is proven before nonneutral rendering. Capture the same seven direct states
and three downstream Corner Pin states through the existing guarded straight
RGBA16 Render Queue method; retain requested/read-back times and encoded depth.
Use unchanged smoke pixel assertions plus exact before/after same-toolchain
comparison. No pending-plane fallback, depth reduction or weakened oracle. All
APIs are already exercised in ae_runtime_smoke.jsx and ae_plane_smoke.jsx; this
is harness validation, not a new native/plugin API. Only owned test projects
are closed without saving; foreign/changed structures fail before mutation.

Preview validation plan: open only the pinned owned scene in regular GUI AE,
prove loaded ordinary identity, 32-bpc/full resolution/Final quality and existing
ready binding. Use the normal Preview UI/shortcut and externally observe cache
coverage, first displayed motion and cached playback separately. Retain timestamp
bounds from observations; do not label UI observation intervals precise callback
latencies. Rebuild only the owned scene after a parameter change; no global purge.
The community-maintained [ViewOptions.fastPreview](https://ae-scripting.docsforadobe.dev/other/viewoptions/#viewoptionsfastpreview)
provides FP_OFF (Final Quality) on a Composition viewer, with readback; other
viewer types must fail. [Viewer.type](https://ae-scripting.docsforadobe.dev/other/viewer/#viewertype)
is verified before using this option. These documented ExtendScript reads/options
control the owned test viewer, not PF rendering or a Preview completion API.
Actual playback/cache completion requires the external UI evidence.

Ordinary fixed-AEP series found a cache confounder: original-source warmup
71.648 s followed by a 43.350 s measured run, versus candidate 46.655/45.416 s.
Those exports retain exact image files but cannot establish fresh renderer
throughput. Stop the owned Python measurement driver with normal interrupt;
allow its current AE render to finish and restore the original in finally.
Keep incomplete/cache-sensitive records, not a speed PASS.

Fresh-invalidation plan: prepare six owned copies through regular guarded GUI
AE, one warmup pair and five measured pairs. Each pair uses the identical
hash-checked AEP for both versions; only Wave Phase differs between pairs,
with identical geometry, depth, quality and animation range. Distinct existing
Develop 2/5 versions and never-rendered phase values avoid the observed same-key
reuse without purging user caches. Require encoded equality for every matched
60-frame pair, record all phase values/project hashes, and calculate paired
speedups plus spread. Cross-pair content variation remains an explicit limit.

Source provenance clarification: the scripting guide is maintained by
docsforadobe from Adobe documentation. Adobe's official
[Scripts page](https://helpx.adobe.com/ca/after-effects/desktop/automate-in-after-effects/automate-animation/scripts.html)
links the scripting reference; its modern UXP ViewOptions page describes a
different engine and is not substituted for ExtendScript compatibility proof.
Adobe's [Preview documentation](https://helpx.adobe.com/mena_en/after-effects/desktop/view-and-preview/preview-video-and-audio/previewing.html)
defines the user-visible cache/playback and Fast Preview quality controls.
Actual AE 25.6 readback and external UI behavior remain required.

The uninstrumented original-source Develop-5 control at 98e3839 /
EGFX-a733d95d2018fb8bc321b6d8 passes the complete 20-stage Mac build,
clean reproducibility, signature and extracted-package verification. Its native
src/ diff from accepted 2ccc5f6 is empty. Build identity confirms the same Clang
21 / Rust 1.98.1 / ordinary build settings as f611312. Earlier bootstrap attempts
that updated the stable Rust alias or lacked pinned Clippy were rejected; the
default was restored to installed 1.98.1 and no 1.99 artifact is used.

Original accepted plugin now passes the independent two-turn 10-frame queue
chain fixture at project 32 bpc and straight RGBA16, export scale 1.0, unchanged
smoke assertions. The first report-writing failure remains retained; its
cleanup completed, and the fix serializes primitive readiness before closed
AE objects become invalid and recreates the report File after cleanup.

## Actual unlocked host block — 2026-10-02

Separate ordinary Develop-5 control uses accepted native src/ unchanged, Clang 21
and Rust 1.98.1 matching candidate Develop-2; diagnostic feature is disabled.
Full physical-Mac/package and reproducibility gates PASS. A fixed-AEP ordinary
series is retained INCOMPLETE after cache reuse masked work. Six fresh Wave Phase
variants were saved in AE only after binding readiness; both versions use the
identical AEP inside each pair, with varying phase between pairs and no purge.
Five measured pairs (plus one warmup pair) pass exact mapped image/disk package
identity, exits, complete 60-frame RGBA16 output and byte equality. Medians
65.870721 -> 48.766815 s, ratio 1.3507x, reduction 25.97%; median paired ratio
1.3303x. All 360 distinct pair/warmup baseline PNGs independently decoded at
calibrated straight RGBA16; corresponding candidate files are encoded identical.
Startup/export/polling and ambient/cache limits are explicit.
See [samples](performance-host-render-2026-10-02.json).

The two-turn queue fixture passes unchanged seven pixel assertions on all three
builds, including Adjustment Layer -> ElasticGrid -> Corner Pin, actual project
32 bpc/straight RGBA16. All ten same-toolchain before/after frames decoded exact.
The original saveFrameToPng harness failure is retained, not weakened or converted
to a renderer PASS. See [matrix/chain](performance-host-matrix-2026-10-02.json).

Actual GUI preview at Full/Final/32 bpc/FP_OFF reaches Playing 60 of 60 and
30 fps (realtime) on candidate and control; cached restart, wave invalidation
and Escape interruption with a still-partial cache are observed. SDK setup
reports only readiness/settings, while UI proves the observed lifecycle.
No precise cache-generation/first-playable timing or five matched RAM pairs:
UI/tool delays, incomplete lower-panel control readback and differing viewer
zoom prevent equal-condition speed acceptance. Native custom-canvas click/drag
returns AXError.notImplemented, blocking a real guide gesture; no OS input
injection or SDK parameter mutation substitutes for it. All owned projects
closed without saving, prior Fast Preview restored, original plugin restored
and Adobe processes stopped. See [observations](performance-host-preview-2026-10-02.json).
Stage 8 stays IN PROGRESS; no merge/release or universal 4K speed claim.

Review after the captured host runs added pre-mutation refusal of stale variant
reports and nonfinite/repeated/source-identical phases. Negative guard mocks PASS;
the successful six-phase creation path is unchanged. Host records preserve the
SHA of the script actually executed, separately from this subsequent guard edit.
All 15 JSX suites and fresh Python 248/248 PASS. The first sandboxed Python run
was incomplete on three OS process/image checks; full permitted execution PASS.
Offline scanner review retains existing mutable Actions/checkouts and a test-only
rate-limit heuristic, with no credential finding; no release certification.
