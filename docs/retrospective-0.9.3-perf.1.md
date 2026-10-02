# FSTR Stretch 0.9.3-perf.1 — performance and release retrospective

Date: 2026-10-02. This closes the resumed quality-preserving Render/RAM Preview
cycle and public-release documentation, not the earlier 0.9.3 milestone.
The [older retrospective](retrospective-0.9.3.md) retains its original candidate,
experimental outcomes and historical policy. No retrospective verdict is a new test.

## Scope and identity

- Shipping source: `f611312bd7b76ebe5bc5f2bd8b48b44f50c0c761`.
- Build ID: `EGFX-6147dc406abc596e7f2d1b60`; embedded version 0.9.3 Develop Build 2.
- Package/tag: 0.9.3-perf.1 / v0.9.3-perf.1; release target `cb429e1ffab06cd79cb0873ba8909b2a8369198d`.
- Public outer ZIP SHA-256: `a563f8e14961e19ee0740d5eb063c89e4bbec830ac1053d23fa09de02f2a6d14`.
- Measured host: AE25.6x101, macOS26.6.2, M1 Pro, 16 GiB.
- Candidate package toolchain: Apple Clang21 / Rust1.98.1, arm64 release profile;
  source control and candidate were rebuilt with matching settings for parity.
- Current documentation policy: adopted rules6.0.0 / `bb8b769404ddd5b97462812a4e6b430e8bfefe13`.
  Earlier experiments retain the baseline recorded in their own evidence.
- No new SDK/API, render change, plugin rebuild, install or host run in this closeout.

Confidence is PROVEN, USER-REPORTED, OBSERVED or UNVERIFIED. Experiment outcome
(accepted/rejected) is independent of Test Status and confidence.

## 1. What now works in the verified After Effects scope?

**Exact cached axis sampling — PROVEN; outcome accepted.** The active plane route
was identified before optimizing it: `eg_render_plane_region` and
`eg_render_plane_layer`, rather than the older generic CPU renderer. Eligible
separable mappings reuse exact axis computations and four Bicubic source rows;
general perspective and exceptional cases preserve the original path. Channel
arithmetic order, Final sampling, resolution, alpha and parameter serialization
were retained. External comparisons include finite and exceptional float cases;
matching-toolchain AE comparisons are exact for 40 plane/3D/AEP frames, a 60-frame
sequence and ten queue-chain states. This is bounded fixture proof, not universal
HDR/OCIO certification. [Native record](performance-resume-2026-10-01.md),
[exceptional floats](performance-plane-nonfinite-comparison-2026-10-01.json),
[host matrix](performance-host-matrix-2026-10-02.json).

**Ordinary Render/export improvement — PROVEN in the recorded workload; outcome
accepted.** Five matched MFR-requested-OFF pairs rendered 60 frames at 1080p,
32-bpc, Full, Final to straight RGBA16 PNG. All 300 measured and 60 warmup paired
frames are exact after independent calibrated decode. Startup/export/polling and
uncontrolled ambient/disk-cache activity remain in the total. This is not a
per-callback latency or cold-cache measurement.
[Raw samples and limits](performance-host-render-2026-10-02.json).

### Performance comparison

| Measurement | Before | After | Meaning and limits |
|---|---:|---:|---|
| Native standalone 1080p32 Final deformed region, median | 386.4830 ms | 32.0415 ms | 12.062× native-only; not AE/Preview speed |
| Ordinary AE 60-frame export, median total | 65.870721 s | 48.766815 s | 1.3507× ratio of medians; 25.97% less time; five matched MFR-requested-OFF pairs |
| GUI Spacebar-to-full-cache median observed interval | (11.546,12.924] s | (1.753,6.007] s | Screenshot-bracketed bounds; capture overhead, fixed order and uncontrolled caches; one pair inconclusive |
| Exact first-playable latency / new Full control timing | NOT RUN | NOT RUN | User closed speed testing; no invented measurement |

[Corrected native samples](performance-plane-corrected-comparison-2026-10-01.json),
[all ten Preview intervals](performance-host-preview-intervals-2026-10-02.json).
The interval series' Preview preset cannot be certified retroactively from later
Full readback. No single blanket “12× faster in AE” claim is justified.

**Native interactive workflows — PROVEN/OBSERVED within named cases; outcome
accepted.** Actual guide dragging changed the image, Cmd+Z restored it, GUI
Preview cached 60/60 frames and played at the composition's 30fps. A preserved
user-scene copy retained animated grid keys and selected-frame parity through
Undo and save/reopen. Viewer functional evidence and frame-render labels do not
prove gesture-to-display latency.
[Guide check](performance-host-guide-2026-10-02.json),
[project check](postrelease-project-validation-2026-10-02.json).

**Public downloaded-candidate installation/loading — PROVEN on the existing
Mac; outcome accepted.** Outer ZIP hash matched, quarantine survived both
extractions and reversible reinstall, and AE loaded the expected binary UUID.
The static baseline-project copy saved/reopened with exact selected-frame output.
No security-settings bypass or certificate substitution was used.
[Distribution check](public-install-project-check-2026-10-02.json).

**Human acceptance — USER-REPORTED; outcome accepted.** The user closed the
speed scope and later reported clean-environment installation plus a legacy
animated Columns/Rows project passed. Those latter environments and raw fixtures
were not independently supplied. Keep these distinct from instrumented PASS.
[Speed decision](performance-user-acceptance-2026-10-02.json),
[release decision](release-user-acceptance-2026-10-02.json).

## 2. What is unreliable, unavailable or should not be repeated?

- **Original optimization on nonfinite floats — PROVEN; outcome rejected.**
  Negative-zero/NaN/Inf handling exposed that an apparent neutral-axis shortcut
  was not byte-equivalent. The corrected candidate preserves the required mapping
  and exceptional arithmetic; no approximate fast path was accepted.
  [Failure/correction history](performance-resume-2026-10-01.md).
- **Old-toolchain artifact as strict parity oracle — OBSERVED; outcome rejected
  for isolating the optimization.** Six matrix frames differed in 29 color channels
  (maximum 2/65535; alpha exact). Rebuilding unchanged source with the current
  toolchain made all checked before/after outputs exact. The original mismatch
  remains retained; matching-toolchain proof does not erase it or prove its cause.
  [Matrix record](performance-host-matrix-2026-10-02.json).
- **Launcher exit0 as render completion — PROVEN; outcome rejected.** One control
  aborted after 19/60 outputs. Expected frame count, decoded output and process/image
  identity must all be checked; a launcher result alone cannot certify a render.
  [MFR record](performance-host-mfr-2026-10-02.json).
- **MFR crash cause — UNVERIFIED.** Four enabled/bypass isolation runs and six
  requested ordinary retries completed, but intermittent failure did not recur.
  Issue21 was closed by the user's bounded-retry decision, not by a causal fix.
  No new MFR CPU cap, MFR disablement, shipping patch or broad stability claim.
  [Isolation](mfr-stability-isolation-2026-10-02.json),
  [closure](mfr-closure-retry-2026-10-02.json).
- **Addressed automated gesture delivery — OBSERVED; outcome rejected as proof
  of an edit.** Only the later authorized foreground/bounds-limited routed helper
  produced observed native displacement and Undo. Failed/unconfirmed attempts
  stay excluded. Helper permissions do not belong to the shipping plugin or user
  installation workflow. [Guide evidence](performance-host-guide-2026-10-02.json).
- **Generic PNG/export instrumentation as render throughput — OBSERVED; outcome
  rejected.** PNG compression appeared in samples, but stack presence gives no
  wall-time share. Instrumented callbacks and native benchmarks provide attribution,
  not ordinary Render/Preview acceptance. Exact first-playable/guide latency,
  ordinary completed-five-pair MFR-ON timing and broad compatibility remain unverified.

## 3. New AE know-how obtained

Use the real selected route, exact loaded identity and a host-authored/hash-pinned
fixture as the starting point. Separate native sampling, callback attribution,
ordinary export, cache fill and onscreen playback. Export module precision,
straight/premultiplied alpha, working space and decoder calibration belong to
pixel-comparison identity. Full viewer resolution does not establish Preview Full;
AE display labels are observations with scope, not measurement oracles.

Keep render scratch storage call-local under SmartFX/MFR. Static exception/checkin
review supports the ownership argument but cannot exonerate a plugin after a
host abort. Preserve failing runs and raw diagnostics locally; publish sanitized
identity/coverage records without user projects or proprietary media.

## 4. Which decisions gave the best result, and why?

1. Optimize the actual plane path, preserving arithmetic, instead of reducing
   fidelity: the accepted candidate improves measured export and preserves pixels.
2. Compare unchanged source and optimized source under the same toolchain, while
   retaining original-artifact differences: this separates evidence scopes honestly.
3. Use native UI observations plus independent output decoding: each answers a
   different question and prevents instrumentation from becoming a false PASS.
4. Bound crash investigation and stop on the user's speed acceptance: unreproduced
   failure is recorded without speculative fixes or endless timing loops.
5. Promote the unchanged verified asset: package revision, embedded version,
   source identity and release state remain distinct. Rules6.0.0 allow ad-hoc
   distribution; publication is not full compatibility certification.
   [Publication](release-publication-rules6-2026-10-02.json).

## 5. What must be reused or promoted to the common engineering base?

[Reusable project know-how and transfer record](AE_ENGINEERING_KNOWHOW.md)
extracts the general validation patterns. The prepared central AE-Development-Rules
contribution contains generalized notes with scope, confidence and immutable
source references; it is published for review in central PR16 after direct human authorization; it adds no new mandatory policy or automatic baseline upgrade.
The existing `tools/render_observation.py`, `tools/live_identity.py`,
`tools/perf_fixture_runner.py` and regression fixtures already implement portions
of these patterns. Reuse requires reviewing the new host/product assumptions,
not copying this product's acceptance verdicts.

## Documentation closeout

The [user guide](USER_GUIDE.md) covers installation, effect location, verified
compatibility, controls, animation, limitations, troubleshooting, update and
uninstall. README and current release records now distinguish the published
version from immutable historical TEST archive text. Source, binary, assets,
installed plugin and prior host evidence are unchanged. Documentation validation
is recorded in [current status](current-status.md); it is not a new AE runtime PASS.
