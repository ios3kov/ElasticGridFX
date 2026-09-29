# Stage 9: 32-bpc PNG baseline investigation

Status: IN PROGRESS; Stage 10 NOT STARTED. No runtime or installed artifact changes.

Candidate: de314981005606741bc75c517d8bb33798b46a1d,
Build ID EGFX-0fa68430a170b3612e8d00f7. Tooling e3d2066.

Acceptance run EGFX-PLANE-6d7bc14040b34b8795a42be81f6b1caf captured all
34 frames, proved live identity PASS, and completed cleanup CLEAN. The functional
gate remained BLOCKED because the fixed nonflat threshold rejected d32-original.
The structured image was dark, not uniformly blank. Pixel comparisons were not
completed; capture completion is not functional PASS.

Follow-up probe 1c251d5f6cc647acb07f2a83a63ce6a2 used a fresh empty owned
project, the same 128x96 source, zero wave, Existing plane mode, and toggled the
effect enabled state at 8, 16 and 32 bpc. Six PNGs were captured and the owned
project was closed with CLEAN recorded. Installed candidate bytes were verified
before execution; this probe did not repeat live image sampling.

| Depth | Disabled RGB maxima | Enabled vs disabled max absolute difference |
|---|---|---|
| 8 | 0.792157, 1.0, 0.713726 | 0 |
| 16 | 0.791257, 1.0, 0.715663 | 0 |
| 32 | 0.079118, 0.099992, 0.071550 | 0 |

Conclusion limited to this scenario: the approximately tenfold darkening exists
with the effect disabled. ElasticGrid processing is not required to reproduce
it. The exact host color/import/export cause is unresolved. This does not verify
32-bpc HDR correctness or the complete deformation matrix.

Next: investigate host PNG conversion/color behavior and add explicit disabled
reference frames to the acceptance design. Do not silently reduce nonflat
thresholds, normalize away possible defects, or reinterpret this probe as Stage 9
PASS. Native guide/corner dragging, Undo and Redo remain separate acceptance.

Local evidence is retained under outputs/plane-acceptance with the above run
identifiers, including the acceptance ZIP and six diagnostic PNGs. User projects,
preferences, installation and main were not changed.

## Follow-up: native export differential and automated acceptance PASS

Native-white probe 1762851d687842ed876ceff01725cdd0 creates only a solid,
without imported footage or effects. saveFrameToPng writes white as 1.0 at 16 bpc
and 0.099992372 at 32 bpc. Probe 4fe194452d644e0d873d875a61ef748e additionally
renders the same 32-bpc white through Render Queue: RGB is 1.0. Both closed only
their owned project, cleanup true. This localizes the observed scaling to the
saveFrameToPng capture route, not ElasticGrid. An independent first-hand report
describes the same symptom (not Adobe confirmation of internal cause):
https://community.adobe.com/bug-reports-528/saveframetopng-results-in-dark-images-if-project-color-is-set-to-8-bit-depth-1216145

The test fixture now uses Render Queue while preserving project bitsPerChannel.
The host-provided _HIDDEN X-Factor 16 output template is accepted only after
verifying PNG Sequence, RGB + Alpha, Trillions of Colors+, Straight (Unmatted),
no resize/crop. Missing or altered template blocks the test. This is a bounded
test-only dependency, not a production dependency; revisit if AE changes template
availability or offers a reliable supported capture API. Each queue item belongs
to the owned fixture; foreign queue items are rejected. Full/Half resolution is
explicit and checked. Queue items are removed on render error; no retries or host
restart. Comparator thresholds are unchanged.

Run EGFX-PLANE-531656cd2c5243818311ac9b1dcb152e: functional PASS, identity PASS,
pixel PASS, cleanup CLEAN. 34 frames and 26 comparisons; full 128x96 and half
64x48 confirmed. Installed immutable candidate unchanged. This is automated
Stage 9 evidence only; native drag/Undo/Redo and final release gates remain open.

Regression: capture-function mocks cover Full/Half, project-depth preservation,
wrong format/depth/alpha/color/resize/crop rejection, foreign queues and render
failure cleanup. They run through test_plane_smoke_safety.js in existing CI gates.
Local Python plane tests 12/12 PASS; real AE supplies rendering evidence above.
Static scanner exits 1 (review_required): existing workflow pinning/checkout
credential findings and an authentication heuristic in an unchanged test. None
is in the changed fixture/capture tests; the scanner is not a release PASS.
