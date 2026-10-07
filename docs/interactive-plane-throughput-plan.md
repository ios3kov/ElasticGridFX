# Interactive plane throughput — 2026-10-06

Rules baseline:8.0.0. Repair scope, both platforms; no parameter/default/key,
sampling, resolution or live-feedback change. No release or merge authority.

User comparison on installed Dev156 (loupe source requests disabled): corner
drag remains jerky at Full/Final; adaptive resolution and Preview are visibly
smoother. This excludes loupe requests as the sole explanation of observed
latency, not as a cause of the separate hangs. No quantitative speed claim.
The original whole-Mac RAM incident and AE-only Dev152 incident remain OPEN.

## Design and authority

The general non-axis-aligned plane sampler currently visits rows serially.
Final samples16 taps versus Preview4. Retain its exact arithmetic and source
coordinate conventions. Schedule disjoint16-row strips through the existing
PF_UtilCallbacks.iterate_generic, four strips per synchronous batch. Only large
general/projected jobs use this path; separable axis-cache and small jobs retain
the current path. No private thread pool, extra frame request or image copy.

SDK25.6 AE_EffectCB.h:925–934 defines iterate_generic in the current input's
utility table; AE_EffectCBSuites.h:612–621 contains the equivalent suite API.
after-effects0.4.0 pf/util_callbacks.rs:136–165 wraps the utility table. No new
suite acquisition or compatibility floor. The wrapper lacks a Sync bound and
panic containment; our callback must provide both explicitly.

The caller retains checked-out worlds, owned axes and State until synchronous
iteration returns. A contained job is Sync solely because input is immutable,
workers write distinct rows, and scheduling joins before these borrows expire.
Workers receive no abort callback/refcon and call no host API. Caller polls the
existing AE abort bridge before/after each bounded batch, never from workers.
No retry/fallback after a scheduler error or cancellation; propagate the error.
Invalid dimensions/strides/overlap and unsupported jobs retain serial validation.

## Verification and remaining gates

- Exact whole-frame versus concurrent strips: all four dispatch modes,8/16/32bpc,
  both qualities, all edges, negative/expanded origins, sparse/empty source,
  padded rows and HDR/non-finite values. Include invalid-quad fallback.
- Cancellation before any work and between batches, worker failure propagation,
  callback panic containment and independent concurrent jobs.
- Existing renderer sanitizer matrix, ordinary/probe Rust checks, strict Clippy,
  host contracts; Windows CI on exact committed source after local checks.
- Exact installed artifact: short Full/Final drag +Undo, followed by ordinary
  loupe press/repress and release. No15-second drag or RAM Preview recurrence
  without a separate bounded incident plan. Short responsiveness is not hang
  closure or release acceptance.

Dev160 implementation: shared Legacy/SmartFX path schedules through the existing
utility table.120 ordinary Rust tests and strict Clippy PASS;16 source contracts
PASS. The focused matrix compares576 distinct mode/depth/quality/edge/source/
geometry combinations byte-for-byte, including NaN/HDR/padding; two independent
frames also run concurrently. Caller cancellation and no worker abort PASS.
Existing plane ASan/UBSan matrix PASS. These are algorithm/threading checks,
not actual AE scheduling, drag-speed or incident closure evidence.

Ordinary Dev160 restores the existing loupe, with no diagnostics. The optional
source-disabled build is Dev161; no diagnostic workaround is enabled by default.
Installed ordinary Dev160: source b585977f90bb7ea1ce15086aaea811dafa9c9c1a,
Build ID EGFX-44e5125b553fe875bb72e116. Exact canonical installed files,
permissions/signature and loaded UUID/path PASS. User confirms repeated press
loupe and Undo work, but drag remains jerky at Full/Final: responsiveness FAIL.
This does not close either hang incident. Mac CI37485816445 and Windows
CI37485816125 both PASS on this exact source; not native Windows acceptance.
Private checkpoint: outputs/interactive-plane-dev160/verification.json.

User also proposed temporary Preview while dragging, restoring Final on release.
This is useful as an additional interaction design. It needs reliable release /
final-frame invalidation and cache/export isolation; currently investigation only.
The Dev160 candidate preserves selected quality throughout the gesture.

## Host quality research / isolated census

SDK25.6 AE_Effect.h:2970–2977 /3120 documents PF_InData.quality as the current
HI/LO render quality, not a drag/export discriminator. Downsample fields are
render resolution, not proof of preview purpose. PF_RenderRequest, SmartPreRender
and SmartRender inputs do not expose an interactive-purpose switch. Artisan
PR_RenderContext APIs cannot be presumed available to an ordinary effect.
AE_Effect.h:748–765 warns that FORCE_RERENDER is a last resort with Undo/cache
limitations; it does not classify requests. AEGP_SetStreamValue changes project
state; temporarily changing Render Quality therefore needs separate Undo,
save/recovery and background-export proof. Do not implement a global mouse bit
that changes output or call UI-only AEGP suites from a render worker.

Dev162 interactive-quality-probe is local-only/nondefault. It only counts the
existing Legacy, SmartPreRender and SmartRender callback quality/resolution in
18 atomic bins. UI gesture start/end take snapshots and write at most eight
small private temporary records. No extra frame request, parameter setter,
sampler change, retained host pointer or worker file I/O. Loupe remains restored.
Bins for each stage: HI full/reduced/unknown, LO full/reduced/unknown. This is a
process-wide overlapping-callback census; use an owned one-effect scene. Counts
are not unique frames and do not prove a generic export classifier.122 probe
and120 ordinary Rust checks, strict probe Clippy and16 host contracts PASS.
The initial probe compile failed because generated BUILD_ID is feature-gated;
the probe now reads the always-present diagnostic marker on the UI thread.
Separate review confirms render workers perform atomic counts only, no UI API,
pointer retention or file access. Skill scanner excludes Rust: its empty finding
list is NOT ASSESSED for this implementation. Pending: exact probe build identity
and a short native gesture census, no long drag or RAM Preview.
Automatic Preview substitution remains NOT IMPLEMENTED pending safe separation.

Dev162 installed from clean local1f90454666f0327b5f495d0dc216af362fc139a9,
BID EGFX-59ae18a6c89d2e8e727b577d. Release/package/installed/loaded-image checks
PASS; ordinary loupe retained. One-effect graph confirmed on a separate saved
project copy,32bpc Full/Final. No new-source CI/publication. Initial AE opening
transport timeout is followed by completed script result and responsive Cua.
Point.value script read fails with AE invalid numeric result even after opening;
cause UNCONFIRMED. Metadata/scalar/key-count guards pass without that getter.
The first helper gesture refused before input (foreground guard); the second
moved the layer rather than a corner and was immediately undone, visually restored.
No census file or valid corner observation yet. Requested one <=1-second human
corner drag/Undo because the bounded helper did not reliably target the grip.
No smoothness, release or hang-closure claim. Detailed private verification:
outputs/interactive-quality-dev162/verification.json.

Installer relaunch is separately reproduced: Close terminated PID51830, then one
Cua getAXState on the closed binding launched PID51890. Source success path exits.
No installation repeated in the probe. Read-only process checks now replace all
closed-installer UI queries. Evidence retained privately at
outputs/interactive-plane-dev160/installer-relaunch-probe.jsonl.

## Dev162 census result / next exact optimization

User completed the bounded corner gesture. Private record binds PID54753 and
BID EGFX-59ae18a6c89d2e8e727b577d, corner1, release1067ms: Legacy0,
SmartPreRender11 HI/full and SmartRender11 HI/full; all LO/reduced/unknown bins0.
These are overlapping stage callbacks, not22 frames or a speed benchmark. Native
screenshot shows Top Right changed to1308,447 and no error dialog, Final/Full
retained. Undo restoration is NOT CONFIRMED by this observation. Host LO is not
an automatic drag signal in this scene; do not use it as an export classifier.

Next scoped optimization: prepare immutable inverse-axis segment invariants once
per PlaneWarp snapshot instead of recomputing source endpoints, spans and Hermite
tangents for every pixel. Retain the original per-sample operation order, float
rounding, upper_bound segment choice, endpoint handling and Hermite blend. No
approximate LUT, quality switch, new host API, shared mutable cache or extra frame.
Both platforms share this C++ path. Verify exact bits against the retained
uncached inverseMapNormalized across valid/near-degenerate axes, boundary and
random coordinates, easing/range cases, then existing full-image/sanitizer and
Rust/host contracts. Native smoothness remains pending, both incidents OPEN.

Prepared-axis experiment REJECTED before integration: release O3 fast and O2
fast each match6,483,600 values, but ASan/O1 fast finds a one-ULP difference;
endpoint-expression and constrained-contraction repair attempts also fail exact
parity. No such code is installed or retained in product source. Private patches
and failing logs retained. Replacement scope: factor repeated source row/column
address validation out of the16-tap lookup; no change to map/weights/color math.
Verify old-address/new-address exact whole-image equality, both samplers, all
three depths, edges, sparse/empty checkouts, origins, HDR/NaN/padding and MFR.

Dev164 address factoring implemented without map/filter arithmetic change.
The internal cache_sample_addresses switch retains the old per-tap lookup as a
reference, never an AE/saved parameter. Existing matrix now compares both address
paths separately, expanded destinations included. Production O3/fast full matrix
and isolated address O1/fast ASan/UBSan PASS; standard preflight O1 all-cache
ASan/UBSan PASS;28 CTest,120 ordinary Rust, strict Clippy and16 host contracts PASS.
First address harness missed CpuRenderer.cpp's RenderCancelled definition; linker
failure retained, dependencies corrected before test execution. All-cache O1/fast
shows one-ULP axis-cache/general difference, reproduced independently on unchanged
HEAD renderer +HEAD test. This older config-specific parity limit remains open;
no false claim that this nonstandard sanitizer configuration passes all caches.
C++ scanner scope unsupported/empty, NOT ASSESSED; separate manual pointer/lifetime
review PASS. Build/installed smoothness pending; no quantitative speed claim.

Dev164 built/installed/loaded PASS: source99f6e2b8f2fc5ba3f031bb5a4eda5d1de29474e8,
BID EGFX-26e3006115f77318033d8d3a; ordinary features default/native-plane only.
Native installer completed and exited, exact canonical files and signature PASS.
PID58561 loaded matching path/UUID72BF227D-5DB5-3DA6-B5B0-0B6F39138683.
Cold opening transport timeout retained; later registration/Cua confirms completed
owned scene opening, Final/Full32bpc and selected visible grid. No repeated
installer launch. User short drag/Undo/loupe response still PENDING. This addresses
redundant sample lookup checks only; automatic Preview substitution NOT IMPLEMENTED.

## Deferred corner contract — user decision2026-10-07

User reports Dev164 only slightly smoother, still jerky: responsiveness FAIL.
Approved replacement: while moving a Surface/Perspective corner, keep the last
image, move grid/loupe, recalculate selected-quality image on release; internal
line manipulation remains live. No change to animation keys or render sampling.
SDK25.6 AE_EffectUI.h:505-510 and official guide PF_EventExtra document
NEVER_UPDATE to defer comp render while clicking/dragging, ALWAYS_UPDATE to
request comp render, UPDATE_NOW after invalidation. Native Point grips consume
CLICK/DRAG before custom UI, so whether DRAW/AdjustCursor responses affect that
native loop is UNKNOWN. Validate this narrowly before promotion.

Local nondefault deferred-corner-probe (Dev167) uses existing scoped loupe gesture
owner/window/time to set UI-only NEVER_UPDATE during the held corner; release
consumes one transition before invalidating/requesting update. No renderer branch,
quality setter, saved stream, OS input hook, extra frame or worker UI API. Native
Point edits and Undo remain owned by existing handlers. Keep the bounded quality
census for host observation. Other owners/times/windows do not receive release
updates; close/new/deactivated contexts clear local state. Internal guide drags
are not corner gestures. Default remains ordinary Dev164 until native validation.
Verify lifecycle/flag state tests, Rust/Clippy/host contracts, exact artifact and
short native drag/release/Undo/loupe; no long drag/RAM Preview. Both hang incidents
remain OPEN. Source/publish/Windows CI scope unchanged, local work only.


Dev167 candidate checks PASS:125 probe +120 ordinary Rust tests, strict probe
Clippy,16 host contracts, warning-free release and bundle/PiPL/exports/signature/
archive/manifest/native installer. Exact sourcef3943d5,
BID EGFX-412cf851b1695fc8cfc7c32c; probe stays nondefault and uninstalled.
User confirms concurrent AE Hot Loader validation2026-10-07; native session not
modified or closed. Frozen image/grid/loupe/release behavior UNKNOWN, native test
BLOCKED until that validation ends. No product promotion or incident closure.


Dev167 native experiment REJECTED2026-10-07: image continues rerendering; user
reports multi-application stall. Exact census124 HI/full SmartRender plus124
SmartPreRender across3 releases. JetsamEvent lists PID91860 AE largestProcess;
system memory-pressure event confirmed, allocation/cache cause UNKNOWN. AE absent
when collection starts, no current hang sample. No repeat, no promotion. Ordinary
Dev168 recovery excludes probe and census, not a fix for baseline hang. Approved
freeze contract remains OPEN; research owned transient corner geometry / single
release commit offline before any next native test. Preserve stream IDs/keys,
selected quality, internal-line interactivity and loupe.


Ordinary Dev168 recovery installed PASS, source57151ca /
EGFX-09791ccfe60cf0457f6663e0, features default/native-plane only. Exact canonical
payload/signature and installer exit verified; AE left closed, no trial. Renderer
and loupe unchanged, no root-cause repair claim. Next diagnostic plan must establish
baseline memory before activation, distinguish host-owned frame retention from
plugin-owned copies/snapshots, record source/output dimensions and checkout/checkin
outcomes, and define an early stop before another user interaction. Offline source
review finds a single <=64MiB loupe copy per UI thread, bounded gesture work,
SmartRender pixel checkin outside result closure and per-call renderer row caches;
these observations are not live leak/cleanup proof. Do not repeat the failed flag
approach or change AE preferences/caches/foreign plugins to mask the failure.
