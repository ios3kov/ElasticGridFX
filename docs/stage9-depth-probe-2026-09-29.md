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
