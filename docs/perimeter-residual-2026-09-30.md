# Native 3D text: undeformed antialiased perimeter

Stage 10/10 corrective render work, baseline 951f4a7 /
EGFX-5a6dbf03b692020f0a89957e. No merge or public release authorized here.

## Reproduction / root cause

User's text fixture exported with effect disabled/enabled to
`outputs/FSTR-Stretch-perimeter/baseline-{off,on}.png`. At x=498,
y=683..709, RGBA is unchanged (including repeated 112/255 coverage), while
the neighbouring letter moves. This confirms a rendered residual, not overlay.
Native text bounds are fractional. PlaneWarp's strict outside-quad test passes
these antialiased perimeter pixels through because native Layer Plane currently
uses the bounded Four Corners region renderer.

## Requirements and scope

- Native automatic text Layer Plane maps all input coverage, including pixels
  outside vector bounds. Extend end-cell inverse mapping continuously beyond
  normalized bounds instead of preserving an undeformed fringe.
- Do not enlarge or round the UI grid, erase alpha, add a fixed pixel halo, or
  change neutral output. Keep neutral mapping bit-exact.
- Four Corners remains a bounded perspective deformation region, with unchanged
  exterior. Raster layer-local path, persistent parameters and UI remain intact.
- Keep Final Bicubic and 8/16/32-bit behavior, sparse origins and cancellation.

Acceptance: independent perimeter mapping/sampling tests across all edges and
corners, fractional/rotated/projective bounds, cropped buffers, neutral parity;
existing core/bridge tests, Rust tests/Clippy; clean artifact; exact-identity AE
before/after on original repro and perimeter variants. No PASS until executed.

Initial baseline script tried reading hidden point streams through JSX and got
`invalid numeric result`; the guarded capture still exported both frames from
a duplicate. It did not change the user's original effect enable state.
Saved original checkpoint: `outputs/FSTR-Stretch-perimeter/user-repro.aep`.

## Implementation and local verification

Added an additive `eg_render_plane_layer` bridge entry, selected only for the
automatic comp-space native Layer Plane. Its inverse mapping extends the first
and last cells continuously past vector bounds. Public Four Corners still uses
`eg_render_plane_region`; frame ABI and saved parameters are unchanged.

- CMake Release / CTest: **19/19 PASS**, including new independent perimeter
  oracle (8/16/32-bit, Draft/Final, all sides/corners, rotated/projective mapping,
  cropped-output parity, stride guards, bit-exact neutral, bounded-region parity).
- Rust: **42/42 PASS**.
- Clippy: **PASS** with the existing two after-effects-macro allowances only.
- Static code audit: review_required, the same 26 baseline findings (20 unpinned
  actions, 5 retained checkout credentials, 1 rate-limit heuristic); none in the
  changed render code. This is not release certification.
- Initial checkpoint, superseded by the installed verification below: further
  live AE exports / installation were **NOT RUN for the new build**. The
  original live reproduction is preserved. A second AE process under LLDB
  appeared during verification; both bundle-ID and full-path DoScript targeting
  now fail with -600. Do not terminate the independently-owned debugger session
  or replace a binary while either AE process is running.

## Installed candidate / original-symptom verification

After user authorization, the debugger process exited independently. Main AE
was checkpointed and quit through DoScript addressed to its exact PID; baseline
exports at Z=0/90/180/270 were preserved before replacement. Candidate e1269d2,
Build EGFX-4b2ef86487d7553a4700e6a4, installed rollback-safely with receipt
EGFX-update-9cf8a9e85f5544998e6c4f753f162b2c. Live identity PASS in AE PID 56880.

The first reopened-project export reused old AE cache and still showed the
fringe. Fresh owned duplicates rendered correctly. After ALL_CACHES purge,
the original repro export matched the fresh duplicate byte-for-byte. At x=498,
y=683..705, all previous fringe alpha is now zero. Fresh exports at all four
rotations were visually inspected; no residual perimeter lines observed.
Artifacts: outputs/FSTR-Stretch-perimeter/after-purged.png, side-0-fresh.png
through side-3-fresh.png; ready-to-check.aep saved and opened for the user.
Host verification covers this 8-bit native-text fixture. Higher depths and
projective corner coverage are core-test evidence, not broad host certification.
Before distributing a new public version, bump the effect version to invalidate
old caches; this local test installation explicitly purged them. No release.

## User acceptance

2026-09-30: following the manual checklist (400% perimeter inspection while
moving guides, 3D layer/grid alignment, Grid Positions animation), the user
replied “фиксируем”. Recorded as user acceptance of the installed correction,
separate from the automated evidence above. Perimeter defect closed for this
candidate. No additional merge, push or public release performed by this
confirmation. Release preparation still requires an effect-version bump for
cache invalidation and verification of that exact release artifact.
