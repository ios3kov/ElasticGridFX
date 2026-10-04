# Windows update parity and CPU optimization — 2026-10-04

## Scope and authority

Latest user instruction resumes Windows implementation now: all implemented Mac
product features must exist on Windows. Later clarification explicitly removes
speed measurements/performance benchmarking from this task. Preserve the same
optimized CPU rendering and quality; run correctness/build checks. Local-only
restriction persists: no push/PR/CI dispatch/publication without new authority.
The current installed Mac Dev52 remains untouched by this Windows source work.

## Feature/source reconciliation

| Feature | Windows implementation | Evidence boundary |
|---|---|---|
| Current-time Grid Positions Reset, keys/Undo | Shared Rust reset_grid and native single-row Drawbot UI; IDs/order unchanged | Existing shared tests; Windows AE not run |
| Uniform density changes preserving deformation | Shared control_grid/C++ field and codec | Shared density and influence tests; Windows AE not run |
| Affected Lines live influence including legacy animation | Shared range_feedback + control_grid; old streams retained | Shared numeric/serialization tests; Windows interaction not run |
| Automatic spacing, legacy stream policy | Shared spacing and default flag | Shared source; native migration Windows not run |
| Simplified hidden controls, static choices, Wave group | Same ParamsSetup on Mac/Windows | Host contract tests; Windows panel layout not run |
| Show Grid default off, optional output pixels | Shared show_grid CPU path, no Mac-only gating | Shared implementation; Windows playback/export not run |
| Popup widths and inline Reset | Shared custom layout/native popup captions | Mac runtime historical; Windows width/keyboard inspection pending |
| About-only version | Shared hidden VersionRow and About command | About/build identity tests |
| License/About/Support/Close window | New Windows native DialogBoxIndirectParamW implementation | Wire-template test locally; MSVC/native guard tests and Windows AE not run |
| Pointer interaction | Shared Adobe custom-UI events; Windows SDK Hand/Pan fallback | Functional interaction retained; Mac closed-hand appearance is platform-specific |
| Safe installer | Mac frontend/pkg also unfinished | Separate U8 obligation; not claimed complete on either platform |
| Marketplace activation | SDKs/test entitlements missing on both platforms | BLOCKED; no fake activation |

Windows now declares IDoDialog in PiPL and GlobalSetup and registers License...
in ParamsSetup, reusing the Rust dialog dispatcher and headless guard. The native
window uses standard Unicode Win32 dialog controls, a default Close button,
Escape/system-close handling, About0.9.4, and a fixed public Support URL on user
click only. Native entry refuses absent/foreign/thread-mismatched active owner;
C++ exceptions are contained at the Rust/C boundary. Support initializes/balances
COM where appropriate, checks ShellExecute failure and displays a fallback URL.
Links are system user32/shell32/ole32, not a new bundled marketplace dependency.
No project/key/credential or renderer mutation is introduced.

## CPU optimization audit (source, not speed claims)

Both host builds compile the same CpuRenderer.cpp and PlaneRenderer.cpp at
release optimization. They use the same SmartFX checked-out snapshot, region
selection, canvas/alpha rules and depth/quality/edge options. Ordinary preview
and render use the shared CPU path, rather than a simplified Windows renderer.

- PlaneRenderer.h defaults cache_axis_mapping=true; PlaneRenderer.cpp caches
  structurally separable mapping and sampling taps per X/Y axis and reuses four
  horizontal sampled rows. Nonseparable/invalid mappings retain safe general
  fallback. These paths have no Apple-only guards.
- Undeformed pixels are copied exactly, retaining float special values.
- RGBA loops interleave independent channels for compiler vectorization without
  reordering per-channel tap sums. Sampling/depth/axis-cache correctness checks
  exercise cached versus uncached results.
- SimdPixelOps.h supports both Apple-arm64 NEON and x64 SSE using _M_X64 for
  MSVC. CpuRenderer uses this four-channel SIMD implementation for 8/16/32-bit
  paths. Windows x64 is not forced into the non-SIMD scalar fallback.
- MFR and immutable SmartRender data flags are declared for both platforms.
- Release C++ uses opt_level(3), ffp-contract=fast where supported, without
  fast-math. Final bicubic quality is retained; no resolution/depth downgrade.
- Legacy row scheduling differs: Mac uses GCD; Windows uses std::thread batches.
  Both are parallel implementations. The overhead difference is not measured
  and no new thread pool is invented without a correctness/design need.
- Metal implementation is Mac-specific, not a Windows backend. Native-plane
  SmartPreRender disables GPU use; the current accepted update CPU optimization
  does not require porting Metal. No Direct3D/GPU acceleration is claimed.

Conclusion: the accepted CPU optimization is present in the Windows source
path. This establishes implementation parity, not a measured speed guarantee.
Performance tests/benchmarks are NOT REQUESTED by latest explicit instruction;
no synthetic number or old Mac timing is relabeled as Windows evidence.

## Verification and next Windows build

Local Mac checks: dialog wire-template layout/parser, shared host contract,
Rust tests/Clippy and selected Release core correctness checks. These do not
compile the Win32 bridge or prove its native visuals/host behavior.
No Windows OS, MSVC compiler, Windows SDK or installed Windows Rust target is
available here. Exact new AEX compilation and Windows AE UI remain NOT RUN.
Existing older AEX/CI is historical and must not be handed out as this update.

Windows CI already builds the same source and CTest scope with benchmarks OFF.
The new conditional Windows native guard test checks null input and worker/no
owned-window rejection with a timeout; it does not substitute for an AE dialog
click test. New source must pass exact-head MSVC/Rust/PE/PiPL gates before giving
an updated AEX to the user. Actual host acceptance focuses on License/About/
Support/Close, keys/projects/Undo and rendering correctness; speed measurements
are excluded. Remote publication/build requires lifting the local-only boundary.


## Subsequent Windows CI checkpoint

User authorized branch publication for the Windows build. First run37189091016
passed24 Windows core tests but exposed Reset continuation ABI mismatch. The
private state now uses SDK A_intptr_t, retaining Mac/Windows semantics and all
existing streams. Source cbb7468 passed run37189319759:24 CTest cases,86 Rust
tests, Clippy, optimized release build and exact PE/PiPL packaging gates. Native
License null/worker guards now PASS on Windows; actual AE dialog interaction is
still NOT RUN. BuildID EGFX-376f97bdd493cd2fbce61edd, version0.9.4 Dev56. Archive,
AEX hash, embedded identity and source reconstruction checked after download.
The automatically triggered repeat Mac workflows were cancelled to avoid
benchmark repetition; no passing Mac gate is claimed for these runs. Speed
measurements remain excluded. This is a Windows validation candidate, not final
AE-host acceptance or a commercial release. Prior local-only notes describe the
earlier checkpoint and are superseded only for this authorized branch build.
