# Current development / release status

## Release acceptance — 2026-10-02 — USER-REPORTED

The human replied “проверил. все ок” to both remaining checks: installation on a
clean Mac/separate account and a real legacy project with animated Columns/Rows
(open, output, save). Both are recorded as USER-REPORTED PASS, not independently
reproduced checks; exact environment, project hash and raw evidence were not supplied.
The accepted scope is the unchanged v0.9.3-perf.1 / f611312 /
EGFX-6147dc406abc596e7f2d1b60 package already linked in this conversation.

GitHub release promotion uses the existing tag and immutable archive. Developer ID
and notarization remain omitted by explicit user decision. This publication status
does not certify the full AE-Development-Rules Release Gate or broad HDR/OCIO/GPU
compatibility. Historical NOT RUN / TEST statements below describe earlier checkpoints.
[Human acceptance](release-user-acceptance-2026-10-02.json).


## Latest distribution and saved-project check — 2026-10-02 — scoped PASS

PR #22 merged at f459a249df0cab9c41179548dd8b173ec155e933; shipping source
f611312 and public archive remain unchanged. Browser-downloaded public ZIP matches
its published hash. Quarantine propagated through both ZIP extractions and was
retained during a reversible atomic reinstall. AE25.6 loaded the exact candidate
(UUID verified in memory), with no security-setting changes. That accelerated
public copy remains installed; the previous accelerated copy is preserved.

A copied baseline AEP from EGFX-bd19dee13315abc0b7e6090e opened, saved and
reopened with all checked static parameters preserved. Its selected Full/Final
native8-bpc frame matches the old reference exactly after RGBA16 decoding.
The preserved user source stayed unchanged and was reopened. This is an existing
Mac/user check, not clean-environment certification. No authenticated legacy
fixture with animated Columns/Rows was available; that migration is NOT RUN.
Developer ID/notarization remain omitted by user decision. TEST prerelease
retained; stable release and broad HDR/OCIO certification not claimed.
[Distribution and project record](public-install-project-check-2026-10-02.json).


## Latest MFR disposition — 2026-10-02 — issue21 CLOSED, NOT REPRODUCED

The human explicitly requested one bounded retry and closure if it did not
recur. Three ordinary renders on the exact originally failing control and three
on the retained candidate all completed60/60 frames (360 total), using the
original hash-pinned AEP / AE25.6 / Full Final32-bpc / MFR-requested ON100. No
new aerendercore crash report;300 encoded pairs exact and60 independently
decoded frames match the retained prior reference. Actual callback concurrency
is unmeasured; disk cache and ambient activity uncontrolled. No new speed test.

Issue #21 is verified CLOSED with reason not_planned by the user's later
decision, superseding earlier open/causal-fix closure criteria for disposition.
The original19/60 SIGABRT remains retained, cause UNKNOWN, fix NONE; no general
MFR certification or full Release Gate PASS. Reopen on a new exact-identity
crash with project hash and pre-termination context. Temporary control rollback
PASS; exact accelerated candidate retained, preserved user scene reopened.
[Retry and closure record](mfr-closure-retry-2026-10-02.json).

Earlier open-issue statements below are historical checkpoints.

## Postrelease project check — 2026-10-02 — scoped PASS

One preserved user scene was checked through an owned copy on the exact retained
0.9.3 Develop Build2 / EGFX-6147dc406abc596e7f2d1b60, AE25.6x101. Native8-bpc
sRGB settings and all three animated grid keys survived save/reopen. UI Undo
restored amplitude0 and the prior deformation. Four selected frames rendered
in each of baseline/Undo/reopen phases; all eight decoded straight RGBA16
comparisons are byte-exact. Native GUI Preview played the animation with full
work-area cache and stopped normally; no new FPS/latency or preset certification.
The source snapshot hash stayed unchanged and it is reopened in AE. User AEPs,
raw logs and pixels stay local. MFR #21 and broad release gates remain open.
[Scoped record](postrelease-project-validation-2026-10-02.json).

## Published experimental prerelease — 2026-10-02

The human explicitly directed release without Developer ID/notarization after
being informed of the adopted rules requirement. This later instruction
supersedes that publication hold for this TEST prerelease only; it does not
turn omitted checks into PASS or certify the full Release Gate.

Published v0.9.3-perf.1, prerelease=true / draft=false, target cb429e1:
https://github.com/ios3kov/ElasticGridFX/releases/tag/v0.9.3-perf.1
Public TEST archive SHA-256 a563f8e14961e19ee0740d5eb063c89e4bbec830ac1053d23fa09de02f2a6d14.
Downloaded public asset matches exactly. Original inner ZIP, signed payload
and tested f611312 / EGFX-6147dc406abc596e7f2d1b60 remain unchanged; AE version
0.9.3 Develop Build2. Previous local VALIDATION wrapper remains historical.
No plugin replacement, certificate change or security bypass performed.

Developer ID NOT DONE, notarization NOT RUN by user decision; quarantined
Gatekeeper/clean installation and final broad release gates remain unpassed.
MFR issue #21 stays open; no cause/fix claim. Stable release not certified.
[Current release notes](release-0.9.3-perf.1.md) and
[verified publication](prerelease-publication-0.9.3-perf.1.json).

### Historical preparation hold — superseded for this TEST prerelease


## Release preparation — 2026-10-02 — local package ready; public release BLOCKED

PR #20 merged as cb429e1ffab06cd79cb0873ba8909b2a8369198d, with identical
Git tree to reviewed f78a5de (all seven hosted checks PASS). The human then
authorized proposed test-release preparation. Critical native / Release
preparation; adopted rules5.1.0 and Release sections26/28 apply.

Prepared local package revision **0.9.3-perf.1** around unchanged tested
0.9.3 Develop Build2 / f611312 / EGFX-6147dc406abc596e7f2d1b60. Wrapper,
inner ZIP, extracted payload/signature and installed-payload identity PASS.
No new native build, installation or host timing series. The package revision
is explicitly separate from embedded plugin version.

Public distribution BLOCKED: Developer ID Application unavailable;
notarization and quarantined clean download/install NOT RUN; final applicable
release regression/migration scope INCOMPLETE. The available local Apple
Development identity is not a substitute. Intermittent MFR issue #21 stays
open with no causal finding/fix. No public GitHub Release or tag created.
Next: obtain Developer ID/notarization access, then sign a new fixed candidate
and verify it separately without transferring previous artifact PASS.
[Package notes](release-0.9.3-perf.1.md) and
[hashes/packaging checks](validation-package-0.9.3-perf.1.json).


## Completed bounded MFR isolation — 2026-10-02

Direct human “делай” applied to the remaining MFR investigation. AI_ENTRYPOINT
read first; adopted rules5.1.0, Critical native diagnostics / Development.
Four planned runs (ordinary candidate/control × enabled/bypassed effect) all
PASS60/60 at requested MFR ON100, Full/Final/32-bpc/straight RGBA16 PNG on the
same-source guarded/hash-pinned AEP copies. Actual callback concurrency is not
measured. All120 candidate frames independently decode; all120 control/candidate
encoded pairs are exact. Enabled60 match the retained same-phase ordinary
candidate and are60 unique frames; bypass60 are one static sequence, distinct.

The original19/60 control crash did not recur in any of these four runs. Raw
crash hash/UUID rechecked: BEE WorkQueue General Render Thread,36 recorded
threads, no ElasticGrid frame on their stacks, generic abort() message only.
This does not exonerate the plugin or identify AE/Camera Raw as the cause.
No shipping fix is justified; no MFR disablement or hidden CPU budget change.
The planned isolation is complete, cause remains unresolved. Further retries
require a new pre-termination callback/error signal or a minimal reproducer.
Speed acceptance stays closed; this is stability evidence only.

Final ordinary f611312 / EGFX-6147dc406abc596e7f2d1b60 is retained installed,
receipt EGFX-update-2186fd3402f54285a391e33f378a75f0. Temporary candidate9a11d806
and control16ef537d transactions are ROLLED_BACK; final payload/package/signature,
sole installed plugin and original backup independently PASS; Adobe hosts stopped.
The source AEP hash is unchanged. No user project, shared settings, permissions,
main or release change. Source/CI checkpoint2236e8d66a29d9cfcb9b747001b53ea339f83e5d
all seven reported hosted checks PASS; this new docs checkpoint has separate CI.
[Four runs, pixel validation, crash review and final identity](mfr-stability-isolation-2026-10-02.json).

### Prior speed-acceptance checkpoint (historical installed receipt)

## Current checkpoint — 2026-10-02 — speed accepted; MFR reliability open

The human explicitly said “считай тесты на скорость мы прошли”: speed scope is
**USER_ACCEPTED**, with no further timing runs planned. This changes the acceptance
scope; it does not fabricate a Test: PASS for missing exact latency or MFR timing.
Quality remains unchanged. Five fresh candidate Full-preset Preview runs all
reach60/60 at30fps; the control was opened but not timed because the human closed
speed testing. All slow/broad intervals remain retained; no new Full comparison.
The human selected Preview Resolution Full, observed natively and preserved
across the control restart; Skip0/Full preset readback now PASS.

Both owned Full scenes are CLOSED and prior FP_ADAPTIVE_RESOLUTION restored and
checked. Control receipt f772d45f is ROLLED_BACK. Under the later direct decision
“возвращать потом старый не надо”, ordinary f611312 / EGFX-6147dc406abc596e7f2d1b60
is now installed and intentionally retained, receipt
EGFX-update-9a11d806b48c4a9abe48ab36c97f1897. Complete payload/package, binary
7a1c2d7c96039cf3bbad3437a90519563394817d5fe4d641e22d1d9a9d3e84d1,
signature, sole installed payload and original backup independently PASS.
Adobe hosts stopped at verification. This is a development/validation artifact.

Rules reread AI_ENTRYPOINT first and consciously updated to5.1.0 / immutable
v5.1.0 peeled54fa9966fd4eab10f35f1fbc8aa18f94ff42925b, after changes/errata review.
Critical native risk; Development plus scoped Validation; quality/testing/
diagnostics overlays apply. No new discovery, reference audit, API or release.
Previous exact head e8ee90c6fb448a0e3a8230e02339c0ab684121a1 has all seven reported
hosted checks PASS; new docs checkpoint requires its own CI.
[Human decisions, Full observations, cleanup and installed identity](performance-user-acceptance-2026-10-02.json).

Remaining independent risk: ordinary MFR control SIGABRT with19/60 outputs.
It remains an unresolved host incident; diagnostic successes and speed acceptance
do not fix it. Bounded source/ownership review is complete: cache objects are call-local,
C++ render exceptions return status, and normal SmartRender Result errors reach
pixel checkin. This static review does not establish the crash cause or a fix.
Next distinguishing signal is effect-enabled/bypass stability isolation or an
exact-identity callback/error signal before termination; no unchanged timing loop. Public release remains unaccepted; no
main merge, public release or cache purge.

### Earlier host checkpoints (historical; superseded final installed choice)

Completed guide checkpoint: ordinary candidate f611312 / EGFX-6147dc406abc596e7f2d1b60
passed an actual native guide edit and Undo in owned AE at Full/Final/32bpc.
The scene preview-2de67dc7e8b64314977ca836bfc20bec is CLOSED; prior
FP_ADAPTIVE_RESOLUTION restored/read back before closure. Receipt
EGFX-update-4b2302ad9b9b480cbef0b90d1995c435 is ROLLED_BACK and the original
payload/signature independently inspected. The human explicitly authorized
bounded standard CGEvent input only in AE and granted both local helpers via
normal System Settings after action-time confirmations. Initial refused or
unconfirmed delivery attempts remain excluded. No permission bypass was used.
[Gesture, current Preview settings and scope](performance-host-guide-2026-10-02.json).

Latest completed host block, 2026-10-02: unlocked AE 25.6x101 on M1 Pro / 16 GiB.
Five fresh matched ordinary MFR-requested-OFF 1080p/32-bpc/Full/Final 60-frame aerender pairs:
median total time **65.870721 -> 48.766815 s** (1.3507x ratio of medians,
25.97% less time; paired-ratio median 1.3303x). All 300 measured frames and
60 warmup frames are encoded byte-identical; independent calibrated RGBA16
decode PASS. Startup/PNG export/polling are included; ambient activity and
OS/disk caches remain uncontrolled. Wave Phase varies between pairs, and the
same hash-pinned AEP is used within each pair. This accepts only that scoped
total-time comparison, not cold-cache/per-frame or RAM Preview latency.
[Samples, identities, distributions and limits](performance-host-render-2026-10-02.json).

All 40 GUI plane/3D/AEP frames pass functional assertions on original, ordinary
f611312 and same-current-toolchain original native source. Same-toolchain
before/after decoded RGBA16 is exact across all 40, the 60-frame plane sequence,
and all ten queue-chain states including Adjustment Layer -> ElasticGrid ->
Corner Pin. Queue assertions pass on all three builds at actual 32 bpc with
straight RGBA16 export. Older-artifact strict FAIL remains retained (29 changed
color channels in six matrix frames, max 2/65535, alpha exact); the unchanged
saveFrameToPng harness failure is retained separately from the successful queue
fixture. [Matrix and chain evidence](performance-host-matrix-2026-10-02.json).

Actual GUI RAM Preview: full 60-frame green range, Info "30 fps (realtime)"
on candidate and same-toolchain control; cached restart, wave invalidation and
Escape cancellation before full cache completion PASS. Full/Final/32-bpc and
FP_OFF are read back through guarded SDK setup; completion comes from UI
observations. The initial lifecycle series had different viewer zooms and is
retained as functional evidence only. A new ten-run series matches 75.4% zoom,
Full/Final/32 bpc and five integer fresh phases. Screenshot-bracketed median
Spacebar-to-full-cache intervals are **(11.546,12.924] -> (1.753,6.007] s**.
Four paired bounds show improvement; phase317 overlaps and is inconclusive.
The slow candidate and broad first-control interval remain included. This is
an observed comparison with capture overhead, fixed run order and uncontrolled
ambient/cache state, not observer-free latency acceptance. Exact first-playable
latency remains open. Later current preset readback observes Skip0 and Resolution
Auto; it does not retroactively certify the interval series or a Full override.
[All ten observations, bounds and identities](performance-host-preview-intervals-2026-10-02.json).
Actual candidate guide edit/fresh-image and Undo now PASS by native UI observation.
Routed authorized CGEvent moved an internal guide; the pattern changed, then
Cmd+Z restored both and Info reported Undo / Change Effect Value. The single
Frame Render Time66ms label is not gesture-to-display latency. Quantitative
guide response and first-playable gates remain open. Current Spacebar preset
readback: Cache Before Playback off, Skip0, Resolution Auto, frame-rate label(30).
[Functional gesture and current settings](performance-host-guide-2026-10-02.json).

Ordinary MFR-requested-ON series: INCOMPLETE after three measured matched pairs
and one warmup pair. The fourth control process produced only 19/60 frames;
launcher exit0 was correctly rejected. Its OS crash report matches the exact
control image/process: SIGABRT, Adobe BEE render-task termination frames; root
cause remains unresolved. All 240 complete-pair frames and the 19 partial files
decode exact. A separate instrumented ON pair covers/decodes all 60 frames
exactly, with overlapping plane_region callbacks; it does not establish ordinary
MFR throughput or resolve the crash. A further bounded test of the exact failing
AEP with the ordinary control and external5/15/30-frame sampling completed all60
frames, independently decoded exact against the ordinary candidate. Failure was
not reproduced under these diagnostic conditions; root cause remains unresolved.
PNGIO compression/filter symbols occur in all three external samples, identifying
export work but not its wall-time share. No five-pair ON aggregate is reported.
A further one-variable diagnostic requests50% CPU with the same failing AEP,
ordinary control and external sample points: all60 decoded frames exact.
Both sampled100% and50% complete; neither localizes the intermittent cause or
accepts a lower-budget fix. Original plugin restored after this transaction.
[Retained samples, sanitized crash and diagnostic pair](performance-host-mfr-2026-10-02.json).

The original installed 2ccc5f6 / EGFX-bd19dee13315abc0b7e6090e payload is restored
and verified after every bounded transaction; Adobe hosts stopped. Two new
preview sessions close with prior Fast Preview read back/restored. An interrupted
predecessor ended without an observed shutdown; its cleanup was not read back,
its cause is unresolved, and it is excluded from the ten-run series. Its plugin
transaction is rolled back/verified. The source
control 98e3839 / EGFX-a733d95d2018fb8bc321b6d8 has unchanged accepted native src/
and the same Clang 21 / Rust 1.98.1 / release settings as the candidate, a separate
Develop-5 cache key and no enabled observer. Full physical-Mac 20-stage build,
reproducibility, signature and extracted package PASS. Immutable f611312
candidate remains unchanged. Diagnostic callback medians 348.734/33.595 ms
are instrumented attribution, not final Render/RAM Preview timings.

Previous tooling checkpoint c62bb7422547bfd911fb27be0375a52d55bccb7e: all eight CI
checks PASS. CI hardening checkpoint c4efa51f0c0f2d5bd8e2900c83ca8169851503af:
all eight exact-head hosted checks PASS. New fixture/evidence checkpoint
8747979b7023793da2789b9d4a92f6cac0074c1a also all eight PASS, including macOS.
Attribution checkpoint 6988fa7dfb3ba4b760fe6e9e60ac6b09a646697b:
all seven applicable hosted checks PASS, including macOS. Current additions: 15 JSX guard/control-flow suites and a fresh physical-Mac
Python 248/248 run PASS. Live exact-head hosted checks are linked in
[draft PR #20](https://github.com/ios3kov/ElasticGridFX/pull/20). Offline code scan retains
one review candidate after pinning all 23 Actions references and disabling five
unused checkout-credential defaults. The remaining rate-limit heuristic points
to a package-verification test assertion, manually confirmed as a false positive;
no credential findings. [CI hardening evidence](ci-action-pins-2026-10-02.md).
This scanner does not assess release readiness.
Stage 8 remains IN PROGRESS. Existing observer spans show about 33.3 s outside
SmartRender on both versions; this includes pre-render/logging/host/export work,
not a directly identified bottleneck. [Attribution limits](performance-host-attribution-2026-10-02.json).
The timing/latency followups listed at this earlier checkpoint are superseded by
the direct speed acceptance above. Retain their actual results and limits.
Functional guide edit/Undo is complete; MFR stability remains independently open.

The user explicitly resumed extremely fast Render / RAM Preview with unchanged
Final quality. This supersedes the earlier skip for new work; it does not change
the historical 0.9.3 acceptance or approve a new artifact.

Branch: `perf/plane-render-throughput`, baseline `2e3d066`.
The current plane renderer is measured independently from the older separable
CPU path. Implementation caches exact axis mappings/taps and four horizontal
rows only for structurally axis-aligned, nonprojected planes at unit raster
scale. Other geometries keep the general mapping. Per-channel tap/arithmetic
order, Final Catmull-Rom, integer rounding, float/HDR, C ABI and saved data remain
unchanged. Independent-frame caches are call-local, with no retained source data.

Corrected source `41283e3d0935160da669d7ec69bc4ae01d51f264`: native
baseline/candidate pixels are byte-identical in 31 finite scenes and 16 additional
exceptional-float arm64/x86-Rosetta checks. Five alternating pairs show 12.06x
median improvement at 1080p/32 bpc and 12.84x at 4K/32 bpc for eligible deformed
planes. Perspective: 1.26x. No target-AE speed claim follows from these timings.

Corrected code passed strict host-math C++ 21/21, ASan+UBSan 21/21 and TSan.
Its complete physical-Mac 20-stage preflight, Rust 61/61/Clippy, real Metal,
dependency audit, two clean reproducible builds, signed bundle and extracted ZIP
checks PASS. Linux GCC/Clang/static/ASan+UBSan/TSan and two regression CI jobs PASS;
hosted macOS source gate PASS. All eight checks at source 41283e3 PASS.

### Historical initial corrected-artifact host block

Validation artifact: Build `EGFX-ce45a845413a943552df0258`, source `41283e3`, ZIP
SHA-256 `a3c001599dc615996a666eac2a6976b2576adf4a0d5ccd4f85cc1daf86dbf362`.
User explicitly authorized temporary replacement. Candidate 41283e3 was temporarily
installed for test; exact old `2ccc5f6 / EGFX-bd19dee13315abc0b7e6090e` inode,
content/modes/xattrs were retained and verified in a transaction backup. Receipt
`EGFX-update-b2a4807ee046479ab986a4bd9b1a972b`. Loaded GUI-AE and aerender image identity PASS for the exact candidate. The
unchanged legacy PNG smoke passed five ordinary pixel checks but did not produce
the Adjustment Layer chain frame; the full matrix remains BLOCKED pending baseline
comparison. An immutable animated 1080p/32-bpc/Final 60-frame AEP was prepared in
a controlled workspace. Its alternating aerender series retained nine successful
runs but stopped on a changed process key at run 9; five measured pairs were
not obtained. Decoded RGB8 equality FAIL (max 1/255), including a baseline repeat.
No target speed claim is accepted. Exact external native checks on the same
pattern pass all 16 cases; this is not AE checkout proof. New fixtures now require
verified straight RGBA16 with recorded color state and encoded-header validation.
Python 240/240 at tooling checkpoint 1fcb0f3 and all 12 JSX control-flow files PASS; mocks are not host proof.
Original installed payload is restored; Adobe hosts stopped. GUI continuation
is waiting for the user to unlock the Mac. RAM Preview remains NOT RUN. No cache purge, main merge
or public release. Initial aefeb4b artifact is withheld after
its x86 NaN-payload FAIL; corrected tests retain exact equality.
See [the resumed performance record](performance-resume-2026-10-01.md).

### Optional observation block — actual legacy host route verified, 2026-10-02

All eight hosted checks at exact tooling checkpoint `1fcb0f3` PASS. Optional
Cargo `render-diagnostics` now records bounded callback phase endpoints and
process-relative starts; default builds do not enable it. No new Adobe API or
render/parameter/state decision was introduced. Feature Rust 65/65, strict
Clippy and Python 246/246 PASS. Active Cargo features now participate in Build Identity, keeping
instrumented and ordinary artifacts distinct. The reader rejects malformed,
partial and capped logs as performance evidence. Diagnostic logging perturbs
execution; final timing must use the default artifact.

Ordinary checkpoint `dabf4d1` complete physical-Mac 20-stage preflight, default
Rust 61/61, reproducible builds and sealed/extracted bundle checks PASS. That
artifact is retained separately. Inspection confirms its resource effect version
matches accepted build 1; cached frames are an additional uncontrolled confounder,
not an established cause of the rejected host series. The next source checkpoint
uses existing PiPL Develop build 2 (ordinary) / 3 (observer), without purging user
cache or changing saved parameters. See [verified source contracts](performance-observation-design.md#effect-cache-identity).
Checkpoint `f611312bd7b76ebe5bc5f2bd8b48b44f50c0c761` has all eight exact-source
CI checks PASS. Its ordinary physical-Mac 20-stage/reproducibility/signature/ZIP
gate PASS: Build `EGFX-6147dc406abc596e7f2d1b60`, branded ZIP SHA-256
`b67947316a1b871050a7cb45b40502f7d3550d1074734e0547e3954fa0b3a25a`, eVER 301058.
Distinct diagnostic Build `EGFX-50e7279b47205ab9285ac7d9`, branded ZIP SHA-256
`d24fde69be2d19c3f5ba2c1bed0f9f6d48ed81f49d91d79e6a1ae45d52f4ae2a`, eVER 301059,
passes feature Rust 65/Clippy/signature/sealed extracted package checks.

Actual AE 25.6 headless pilots pin loaded payload/image UUID and verify all 60
1920x1080 straight RGBA16 PNG headers. The observer records 60 fresh completed
32-bpc `legacy_cpu` outputs plus 60 SmartPreRender callbacks. Sampling median
2.199 ms is instrumented attribution, not acceptance timing. This historical
footage fixture does not exercise the optimized plane sampler. CLI Format/Channels
override attempts failed as read-only and produced no frames; exit 0 was rejected.
The existing `_HIDDEN X-Factor 16` output template actually produced RGBA16.

Accepted old-toolchain artifact versus f611312 decoded RGBA16 equality remains
FAIL: up to 2/65535, 44,689 changed channels across 53/60 frames, unchanged alpha.
Original repeat and ordinary/diagnostic candidate pairs match all 60 complete
PNG files. A local research clone retains every accepted `src/` native/bridge
byte and only adds observation/feature identity/effect build 4. Rebuilt with the
current toolchain, Build `EGFX-70cb973b561335f81da67803` produces 60 PNGs exactly
matching ordinary f611312; fresh legacy callbacks and loaded image are verified.
This isolates a build/toolchain difference for this workload; the exact FP cause
is unresolved, and the original-artifact FAIL is retained. No tolerance change.
Public sanitized evidence: [host diagnostic](performance-host-diagnostic-2026-10-02.json).

New schema-3 fixtures explicitly select/read back Four Corners and geometry or
Layer Plane fallback. Callback validation requires actual declared route,
dimensions/depth and complete frame coverage. Python 248/248 and all 12 JSX
control-flow suites PASS; mocks do not prove AE execution. Original installation
is fully restored after each transaction. The Mac remains locked; actual new
plane fixture/matrix and RAM Preview are BLOCKED / NOT RUN. See [design](performance-observation-design.md).

### Resume checkpoint — AI-STATE-001

- Goal: quality-preserving Render/RAM Preview acceleration, requested by the user
  on 2026-10-01; [quality contract](performance-quality-contract.md) covers this
  scope. Stage 8 exit requires actual host timings and unchanged output.
- Rules: v5.0.0 candidate / `b27f45467e0a9152fc82c1072438dfed07f0c36e`, refreshed
  at the user's explicit request, with AI_ENTRYPOINT read first. Stable published
  standard remains v4.0.0. Applicable native Critical risk; Development work and
  limited Validation artifact, no Release request.
- Components: existing Rust `after-effects 0.4.0` SDK-less host and C++ plane
  sampler, Apple Silicon/macOS 26.6.2 physical checks; AE 2025 / 25.6 target scope.
  No new SDK/host API use; reuse current integration evidence only for unchanged
  contracts. New API use requires source/signature/version verification.
- Branch: `perf/plane-render-throughput`; native sampler checkpoint `41283e3`,
  observation `dabf4d1`, cache-version/artifact checkpoint `f611312`. Current
  tooling adds schema-3 route attribution and retains sanitized actual host evidence.
  Sampling and saved-data behavior remain unchanged. Preserve immutable artifact
  identity above; documentation/tooling commits do not retag those packages.
  Recheck HEAD/tree before dependent actions.

| Confirmed scope / permission | Source | Still applies |
|---|---|---|
| Continue this repository under central rules | Original user request, 2026-10-01 | Source edits and relevant checks |
| Extremely fast Render/RAM Preview, no quality loss | User clarification, 2026-10-01; quality contract | No filter/precision/resolution downgrade |
| Push performance branch and open draft PR | Explicit user answer, 2026-10-01 | [PR #20](https://github.com/ios3kov/ElasticGridFX/pull/20), ongoing branch updates |
| Reread current AI_ENTRYPOINT and continue | User instruction, 2026-10-01 | Adopt current v5.0.0 candidate baseline for continuing work |
| Temporary installed-plugin replacement | Explicit user message, 2026-10-01 | Bounded validation candidates, preserve original for rollback |
| Close projects without saving in After Effects | Explicit user message and AE-only clarification, 2026-10-01 | AE project closure allowed; does not extend to other apps |
| Finish development continuously with stage statuses | Explicit user instruction, 2026-10-01 | Continue applicable development/validation; report each completed block, then continue |

Temporary candidate installation is authorized; original is restored between experiments. Controlled host
validation continues on synthetic projects; merge, release and cache purge
are outside the current authorization. Closing AE projects without saving is
explicitly authorized. The task-state record points to the conversation; it is
not independent authorization.

Next: after Mac unlock, compare the missing legacy chain capture on the original
plugin, run the existing straight-RGBA16 plane/3D/AEP matrix, and create a real
schema-3 Four Corners AEP. Require observed `plane_region` frame coverage before
attributing optimization results; keep same-toolchain source isolation separate
from accepted older-artifact compatibility.
Accept Render timings only after pixel parity; RAM Preview needs externally
observed cache-build/playback measurements. Independent source/CI/documentation work continues alongside host validation. Stage 8 remains IN PROGRESS.

Canonical rules now point to the user-selected AE-Development-Rules baseline.
Earlier project evidence and the accepted 0.9.3 runtime candidate below remain
historical evidence for those exact artifacts.

## Final Stage 10 acceptance — 2026-10-01

**Stage 10 COMPLETE for the agreed macOS Apple Silicon / AE 25.6 development scope.**

Exact accepted candidate: `2ccc5f674b8b94b534d4c3ad6abdec3524e3624f`,
Build `EGFX-bd19dee13315abc0b7e6090e`, FSTR Stretch 0.9.3 development,
ZIP SHA-256 `1448a5231fc561e6b10491d1b9d1839de22f21a11266a43e2990244a25176ebc`.

Automated gates for this exact source/package are PASS: Rust 61/61, C++ 20/20,
Python 239/239, strict Clippy, PR CI and full macOS source/build/signature/package
gate. The user loaded this exact candidate in target AE and confirmed the final
About, functional behavior, Layer Plane alignment, RAM Preview and three
cancel→rerender cycles. #7, #8, #9, #13 and #14 are closed as scoped evidence.

See [Stage 10 final acceptance](stage10-final-acceptance-2026-10-01.md).

At this accepted checkpoint Stage 8 optimization was **SKIPPED BY USER**, not PASS.
It is resumed above for new development. Windows/Intel, other AE
versions, broad HDR/OCIO and physical GPU execution remain outside the accepted
scope.

After acceptance, PR #10 → #12 → #15 were merged to `main`; PR #16 merged the
Stage 10 documentation. The plugin tree at the PR #15 merge matched the exact
accepted `2ccc5f6` tree; later merged differences were documentation/checksums only.
0.9.3 is still **not published**.

The §25.1 technical retrospective is now recorded in
[retrospective-0.9.3.md](retrospective-0.9.3.md). Under the current macOS public
distribution rule, release readiness remains **BLOCKED** on Developer ID signing,
notarization/applicable stapling and a Gatekeeper-clean quarantined download/install
test. The accepted development artifact remains ad-hoc signed; no security bypass is
an acceptable release step.

Everything below is preserved historical development context unless explicitly
marked current.

## About footer candidate gate failure and correction — 2026-10-01

Candidate source `e4f1ed10c650f9f70e606ed97f34d4dc16042ae4` removed the redundant
standalone `fstr.tech` footer, but the mandatory first-application and macOS
source gates **FAILED** before handoff. The failure is preserved as evidence and
is not relabeled as PASS.

Failed runs:
- push first-application regression **36853623602** — FAIL in Rust host contracts;
- PR first-application regression **36853629081** — same FAIL;
- macOS source gate **36853623609** — FAIL during full preflight;
- PR CI **36853629066** — PASS for portable C++/sanitizer/static-analysis scope only.

Root cause: research-only `host-rust/src/lifecycle_probe.rs` still referenced
the removed generated constant `build_identity::ABOUT`. The production About
path had already moved to `ABOUT_BYTES`, while internal diagnostics use
`DIAGNOSTIC`. This compile error is unrelated to rendering or the copyright
byte itself, but it blocks delivery under the development rules.

Correction: the lifecycle research journal now uses
`build_identity::DIAGNOSTIC`, matching the intended separation between
user-facing About bytes and internal provenance. No renderer, parameters,
serialization, Fit Layer, grid-density, package naming or project data behavior
changes in this correction.

The previous failed runs remain failures. A new candidate must pass the complete
required regression and macOS source/package gates before it can be handed off.
Target-AE verification is still required for the final About rendering. No merge
or release is authorized.

## About footer cleanup — 2026-10-01

Per user review, the standalone `fstr.tech` footer is redundant and removed.
The final visible copy is now:

```text
FSTR Stretch
Version 0.9.3

Professional mesh deformation for Adobe After Effects

© 2026 FSTR.tech. All rights reserved
```

The single-byte copyright encoding correction remains unchanged. This source
change creates a new candidate identity, so automated gates must pass again
before handoff. No merge or release is authorized.

## About copyright encoding correction — 2026-10-01

The user opened the newly branded 0.9.3 About dialog and the new text appeared,
but the copyright glyph rendered as `¬©`. This directly identifies an encoding
mismatch in the visible dialog: the Rust wrapper copied UTF-8 bytes `C2 A9`
into AE's legacy `A_char[256]` return-message buffer, and the host displayed
both bytes as characters. Adobe's own About dialog on the same host displays a
normal copyright glyph.

The correction keeps the real `©` symbol and writes the About field as exact
legacy bytes with a **single `0xA9` copyright byte**. The visible copy is also
updated per user request to remove terminal periods:

```text
FSTR Stretch
Version 0.9.3

Professional mesh deformation for Adobe After Effects

© 2026 FSTR.tech. All rights reserved
```

The internal BuildIdentity/diagnostic path remains unchanged and separate.
This source change creates another new candidate identity, so the prior package
and Build ID must not be relabeled. Mandatory automated gates and a final
target-AE About check are required again. No merge or release is authorized.

## Target-AE acceptance and About branding — 2026-10-01

The user loaded the exact 0.9.3 candidate `ae0c6c47d3318f3e1b7cb0edb4290f6ce04b0dd7`
(Build ID `EGFX-2ac782234af13eae3b01f67d`), supplied an About screenshot
matching those identifiers, and after the requested manual checks reported that
the rest works. Record this as **USER-REPORTED acceptance of the requested
manual scenario**, not as full platform/compatibility certification.

The next source change is presentation-only: the user-facing About dialog is
approved as:

```text
FSTR Stretch
Version 0.9.3

Professional mesh deformation for Adobe After Effects

© 2026 FSTR.tech. All rights reserved
fstr.tech
```

Build ID, commit, target and source-state text are removed from the visible
About dialog, but provenance remains in `BuildIdentity.json` and the
noninteractive diagnostic string so support/package verification can still
identify the exact binary.

Because this changes source after the accepted `ae0c6c4` artifact, any plugin
built from the new commit is a **new candidate** with a new commit/Build ID.
The old artifact hashes and host PASS must not be transferred to it; mandatory
automation/package checks and target-AE handoff must be repeated. Version
0.9.3 remains a development version and is not a published release.

The remaining Layer Plane overlay/index-span alignment item in #14 stays open
until its real AE viewer mapping is measured and resolved or explicitly
accepted. No merge or release is authorized.

## Development: Fit Layer boundary correction — 2026-09-30

Stage 10, branch `fix/fit-layer-coordinates`, stacked on PR #12. The source
corrects Fit to (0,0)/(W,0)/(W,H)/(0,H), with the existing corner order and full
source-layer dimensions. It skips writes to already-fitted corners, including
repeat Fit. It does not write Grid Positions/counts or migrate stored keys.
Rendering, layer/comp projections and the density implementation are unchanged.
The extra Layer Plane overlay/index-span audit still requires native AE
alignment evidence; a global change to raster coordinates was not made.

Version 0.9.3 development; not released. The source also derives Finder version
from validated BuildIdentity before signing (#13), and the native verifier
checks both version fields on the named and extracted bundle. This is not an
in-place edit of a prior candidate. Exact final CI/artifact evidence is in the
correction PR; live target AE is NOT RUN in this Linux session.

The user reported the density candidate 59a73ea working, then supplied the Fit
coordinate mismatch. That historical density acceptance is not a new-candidate
host PASS. See [coordinate acceptance](fit-layer-coordinate-fix-2026-09-30.md).

## Historical density checkpoint

## Development: animation-preserving control density — 2026-09-30

Stage 10 remains incomplete. Branch `fix/grid-density-preserve-animation`,
based on 7f72201 / PR #10. User supersedes the reset-on-count-change policy.
Columns/Rows now select visible controls over the retained animated deformation;
count changes do not write Grid Positions or resize/reset render snapshots.
The default stored lattice and wire format remain unchanged. New handles edit
that retained curve through interpolation/elastic influence, not through a
lossy conversion of old keys. Fewer visible lines do not discard hidden detail.

Source implements the requirement; exact native AE UI/key metadata checks are
NOT RUN here. See [acceptance and evidence](grid-density-preserve-animation-2026-09-30.md).
Version is 0.9.2 development; the AE effect version is bumped to invalidate old
render caches. This is not a published release. `FSTR Stretch.plugin` is retained.

## Historical static-count/package checkpoint

## Development: static counts and branded test package — 2026-09-30

Stage 10 remains incomplete. Branch `fix/first-application-516`, draft PR #10.
No merge or release is authorized by this development step.

- Source `9199c088b2e31d3c552521d71f4f0d2d5663bb24` makes Columns and Rows
  manual-only setup controls; Grid Positions remains animatable.
- This packaging checkpoint emits **FSTR Stretch.plugin** in the user-facing ZIP.
  Internal executable, effect match name, parameter IDs and rendering remain unchanged.
- Exact automated results, artifact commit/Build ID/hash and host-verification
  boundaries are recorded in PR #10. New package acceptance in AE is NOT RUN.

## Last user-tested candidate (not the new static-count package)

Commit `94d6706db5539d9f3372adf0efc8edeeaa38a622`;
Build ID `EGFX-e967ab9a87fe4e44340d0f46`;
ZIP SHA-256 `99ca836d73cdadb5b3efc793fd16fd6f47f5ceca2ae8944e7612ac4836912b30`.

About identity was shown by the user. First addition, text/Checkerboard-precomp
visible deformation, save/restart, RAM Preview and cancellation/re-render were
confirmed by the user in issues #9, #7 and #8. These are USER-REPORTED results,
not a new instrumented payload/PID or pixel-matrix verification. No result is
transferred to a later Build ID or differently packaged artifact.

## Remaining gates

The latest-head automated regression and signed/extracted package checks must
pass before a test handoff. Target AE must confirm static editable counts,
continued Grid Positions animation, Undo/Redo, fresh addition and save/reopen.
Old projects that already keyframe Columns/Rows are NOT VERIFIED; preserve
originals and test copies. No migration script deletes their keys.

Full evidence reconciliation for #9, #7 and #8 remains open. Stage 8 optimization
stays SKIPPED BY USER, not PASS; there is no performance claim. GPU, Windows/Intel,
non-square 3D UI and broader HDR/OCIO certification remain outside this scope.
The requested bundle filename is a packaging change, not a new effect identity.

## References and preserved history

- [Static counts and compatibility](static-grid-counts-2026-09-30.md)
- [Packaging decision, checks, installation and rollback](branded-package-2026-09-30.md)
- [First-application investigation](first-application-516-2026-09-30.md)
- [Unmodified previous status/history](status-0.9.1-history.md)
- [Published 0.9.1 record](release-0.9.1.md)

The last published release remains e1848d5 / EGFX-879b31e5a95527a385827c24,
not the installed development candidate or latest source. Its historic PASS
records do not close the subsequently reported first-application defect or
certify the new static-count/renamed package.
