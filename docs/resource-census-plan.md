# Corner-drag resource diagnosis — Dev169

Rules8.0.0, Critical native / Development. AI_ENTRYPOINT first; controlled state,
documented API contracts, memory/resource lifetime and the debugging stop criterion
apply. User authorizes the local diagnosis/fix/verification block, AE closure and
FSTR replacement. User additionally authorized the exact temporary exclusion and
return of9 Hot Loader test bundles, and explicitly confirmed Dev169 installer
launch. No push, release, other plug-in changes or other-app closure.

## Question and acceptance

Dev167 failed to freeze native Point dragging and coincided with a confirmed
system memory-pressure event. Its allocation/cache cause remains UNKNOWN. Do not
repeat its DRAW flags experiment. Dev169 is a passive, nondefault diagnostic on
ordinary Dev168 behavior, not a repair or a release candidate. No speed benchmark.

Compare process physical footprint with retained parameter snapshots, copied
loupe pixels, borrowed SmartRender worlds and actual checkin results. A positive
signal must identify which category grows; stable FSTR counters do not prove an
AE cache cause or exclude transient core allocations. Borrowed world scope is
not allocation ownership. Only then design a causal repair; separately validate
the approved frozen-image/live-grid/live-loupe interaction and release commit.

## Instrumentation contract

Feature `resource-census-probe` selects Dev169. Default remains ordinary Dev168.
No render-quality setter, additional frame request, event invalidation, saved
parameter change, host call on workers or retained host pointer is added.
Workers only update scalar atomics. Existing UI callbacks append at most64 records
per UI thread to a fresh0600 private temp file; unchanged samples are omitted and
nonforced records are separated by at least200ms. Log exhaustion is PARTIAL.
Unix millisecond timestamps bind the census and external observer.

Schema1 `resources` rows: snapshot, loupe copy, borrowed input, borrowed output.
Each row: created tokens, dropped tokens, live bytes, peak live bytes, largest
token, last width, height, bpc. Snapshot bytes estimate struct plus grid vector
capacity, not allocator overhead. A cloned snapshot starts another estimated
token. Loupe bytes describe the retained copied pixel vector, limited by the
existing64MiB bound. Input/output bytes are row stride times height while in
callback scope; token drop does not prove that AE freed or evicted those pixels.
`checkins`: pixel success/error, loupe success/error, async polls/nonnull receipts.
Individual relaxed atomics are not a coherent concurrent snapshot; interpret
balances at idle boundaries, never infer a leak from a single racing record.
AE owns pre_render_data and its deletion callback; the census cannot delete it.

## Native execution gate

1. Read current process inventory without starting AE. No concurrent AE validation.
   Inventory foreign test plug-ins and preserve exact files/hashes. If an isolated
   FSTR-only baseline requires temporarily excluding them, obtain that additional
   authorization with a concrete restore plan; none is implied by FSTR installation.
2. Build/package exact clean Dev169, preserve ordinary Dev168 and prior artifacts.
   Validate identity, PiPL, exports, signature, manifest and installer before use.
3. Use a saved disposable copy of the retained32bpc Surface/Full/Final one-effect
   scene. No original project writes, RAM Preview, long drag or adaptive-quality
   substitution. Bind loaded path/UUID/BID and owned PID before a gesture.
4. Check global memory headroom first. External guard validates ordinary AE path
   and process birth. Current loaded idle baseline must be<=1.25GiB; absolute
   limit1.5GiB or growth+256MiB. A limit stops the test, with no gesture or blind retry.
   The first observation stopped during loading, before any gesture or registered
   fixture result: initial602331256bytes rose past the256MiB growth bound.
   Only the GlobalSetup zero census was delivered, so it cannot attribute growth.
   Follow-up separates loading from loaded idle: optional `--loading` keeps a
   absolute loaded-idle cap until setup completes and memory varies<=8MiB over
   >=1second. Only then ARMED establishes the loaded idle baseline for the same
   +256MiB/1.5GiB gesture limits. No gesture before ARMED; cap failure stops again.
   Fixture setup omits an unnecessary complete app.effects catalog traversal.
   Other installed third-party components remain loaded; absence of Hot Loader
   is not a fully clean third-party environment claim.
   The separate loading run completed setup and exact loaded identity, then hit
   the initial1GiB cap at1124494760bytes. Four UI census records show snapshots
   3created/3dropped/live0, worlds2created/2dropped/live0, two successful pixel
   checkins, no loupe copy/request yet. Largest borrowed world33177600bytes,
   1920x1080x32bpc; the second world113x64 is a host thumbnail, not a speed test.
   These idle-boundary records do not establish cache ownership or the drag cause.
   Evidence-based plan revision: loading/idle cap1.25GiB accommodates the observed
   initial frame; total limit stays1.5GiB, growth stays256MiB, duration stays45sec.
   No quality reduction or new graphics build. Stop again if this cap fails.
   PID3572 subsequently armed at1070084744bytes, then reached1740308152bytes
   at8.989seconds without any mouse gesture. Expected exact-PID stop/sample/kill
   PASS. Last delivered UI census: snapshots9/9/live0, worlds8/8/live0,
   eight successful pixel checkins, no loupe copy/poll/receipt. The stopped
   process sample includes active FSTR rendering; zero last-UI scopes do not
   exclude subsequent worker activity or untracked/host allocations.
   A Cua getApp lookup timed out in that interval; its screenshot never ran.
   A separate empty AE PID3656 appeared afterward, was proven unsaved with zero
   items and closed by exact PID/path/birth. All9 foreign bundles restored.
   Read-only preference inspection finds Cache Frames When Idle enabled with
   an8000ms delay. Adobe documents automatic idle rendering after that delay:
   [Multi-Frame Rendering / Speculative Preview](https://helpx.adobe.com/after-effects/desktop/render-and-export/multi-frame-rendering/multi-frame-rendering.html).
   This is a new testable confound, not proof of the original hang's cause.
   Next bounded comparison: same saved fixture and exact Dev169, no Cua/AX or
   mouse input in either run, first leave idle caching on, then temporarily
   disable only that preference in the second disposable process. Each run
   lasts20seconds under the unchanged guard bounds. Read back the runtime
   setting, suppress preference saving in the off process, verify the original
   on value on disk after exit, and return the9 bundles after each run. Never
   persist the off setting as a product fix or change MFR/quality/memory caps.
   No native gesture until this distinction and current ownership are resolved.
5. Arm the guard before any action. One<=1-second corner gesture only after READY
   (ARMED when using the loading phase) and exact post-open identity;
   maximum observation45seconds. On a limit the owned AE is stopped, sampled for
   one second and killed; it is also killed at the deadline. The saved fixture
   makes this authorized disposal safe. Recheck process identity before each
   signal and verify exit afterward. No Photoshop/Telegram or other-app signals.
6. The guard is best effort, not a guarantee against system stalls, allocations
   between200ms polls or OS/permission failure. A failed identity/read terminates
   observation without signaling an unverified process. Stop interaction at once.
   Never leave a human gesture running after observer expiry. Preserve any guard
   exception, check host state without launching it, and do not retry blindly.
7. Retain private logs/sample/project hashes, classify counters/footprint/checkins
   and stop on a limit/error. No raw project/log upload. A subsequent test needs a
   new discriminating hypothesis, not repetition of the failed Dev167 experiment.

## Offline evidence

Ordinary Rust120, probe Rust122, strict probe Clippy,16 host contracts and6 external
guard tests PASS. Initial probe test compilation failed for a missing cfg field;
repaired and rerun, original failure log retained. Guard tests cover independent
limits, PID/path/birth mismatch, target system struct layout, sample/log failure
cleanup and armed versus observe-only deadline. Tests do not prove native memory
growth, host frame validity or recovery. C++ renderer unchanged; no new C++ verdict.
Rust/C++ are outside the general static scanner's supported scope: NOT ASSESSED
by that scanner; manual ownership/worker/error-path review is recorded above.
Follow-up loading guard10 tests PASS, including tighter loading cap, marker plus
stable idle transition and unchanged post-load growth bound. Native first observer
PASS for expected stop/sample/kill and exact-process exit; gesture NOT RUN and
memory attribution INCONCLUSIVE. Source2b53747 / EGFX-376c92a761bbecece5122c43 /
loaded UUID3826B5EF-8479-3D2B-A9E4-ADF619F5F8A2 / PID1992 exact identity PASS.
AE-only disposal verified; all9 Hot Loader bundles restored byte/mode-exact.

Private local evidence: outputs/resource-census-*.log,
outputs/resource-guard-tests.log, outputs/resource-census-foreign-inventory.json.

## Idle-cache discrimination result — 2026-10-07

Same source2b53747 / BID EGFX-376c92a761bbecece5122c43 / installed UUID and saved
32bpc Full/Final Surface fixture. No mouse, Cua/AX, RAM playback or quality change.
The UI Deformation Plane value3 denotes Surface; hidden saved mode2 is preserved
through the display permutation. The loupe preparation's saved-mode2|4 check is
correct, not a stale menu-order defect.

| Observation | Enabled / PID6028 | Runtime disabled / PID6756 |
|---|---|---|
| Setup / post-open exact loaded identity | PASS | PASS |
| External bound | Absolute limit at8.996seconds | No limit for20seconds |
| Memory | Stable1081471792bytes then1703626704bytes | Peak1088548432bytes, end989884872bytes |
| Last delivered census | Snapshot4/3/live607; input/output3/2/live33177600 each | Snapshot2/2/live0; input/output1/1/live0 |
| Pixel checkins | 2success/0error; render active at last UI row | 1success/0error;113x64 thumbnail |
| Loupe copies/polls/receipts | 0 | 0 |
| Exact-PID exit /9-bundle return | PASS / PASS | PASS / PASS |

Cold-start empty-project preflight timed out before AE UI initialization. A
sample then showed startup complete; one bounded read-only retry proved the same
PID empty before opening the fixture. The off run waited for cold initialization
before its empty-project check. Neither observation began on an unproved project.

Conclusion: AX lookup is unnecessary for the observed approximately9-second
idle burst. Enabled8000ms speculative caching is a supported explanation for this
burst; disabling it removes that burst in the bounded follow-up. The off run's
full viewer frame may be served by the now-warm host cache (only a thumbnail
crossed FSTR); do not promote this comparison to fresh full-frame/MFR stability,
leak elimination or the original hang's root cause. UI-only logs may lag workers.
Stopped-process stack repetitions do not measure a stall duration.

The off setting was confined to the disposable AE process using the documented
[Preferences API](https://ae-scripting.docsforadobe.dev/other/preferences/)
with machine-independent storage;
[saving preferences on quit](https://ae-scripting.docsforadobe.dev/general/application/#appsetsavepreferencesonquit) was
disabled. Original Cache Frames When Idle=01 and the entire original preference
file SHA256 verified unchanged on disk after exact-process exit. No disk
preference file overwrite, cache deletion or permanent disable.
All9 excluded bundles returned, AE absent, no other-app signals. Raw logs and
saved projects stay private under outputs/resource-census-dev169/idle-cache-*-no-ax.

Next gate: keep background caching controlled for a single-frame/owned-corner
experiment, independently prove a fresh full-size render miss, and observe the
gesture without startup/idle/AX confounds under the existing bounds. Do not change
the plugin's quality, remove AE abort checks, retry failed DRAW flags, or implement
renderer-side output reuse under changed parameters. The approved freeze contract
requires transient corner state outside canonical streams until release. Native
Point hit priority and old-key/UI compatibility remain unresolved; no gesture or
product fix is accepted by this diagnostic result.
