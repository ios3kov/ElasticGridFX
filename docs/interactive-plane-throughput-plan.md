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
Native and Windows checks: pending on committed candidate.

User also proposed temporary Preview while dragging, restoring Final on release.
This is useful as an additional interaction design. It needs reliable release /
final-frame invalidation and cache/export isolation; currently investigation only.
The Dev160 candidate preserves selected quality throughout the gesture.
