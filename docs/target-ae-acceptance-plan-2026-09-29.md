# Target After Effects functional acceptance plan — 2026-09-29

Baseline candidate under test: immutable fd69988c10b25268eb8cad6ee6ced7f6a28bee9d,
Build ID EGFX-f442513cb6528f14295d6d45. The current updater source is 11c607d.
Authoritative development rules rechecked before this stage.

## Goal

Close only the currently reported functional regression in the real target host:
bright streaks / compact-world handling and Adjustment Layer -> ElasticGrid ->
Corner Pin black output. Do not conflate this with release readiness, 3D plane
delivery or performance approval.

## Controlled target state

- macOS Apple Silicon, observed target app:
  /Applications/Adobe After Effects 2025/Adobe After Effects 2025.app
- Exactly one idle After Effects instance.
- User work saved first. Test requires a new empty, unsaved, non-dirty project.
- Installed plugin must match the complete fd69988 signed manifest before any host
  action. Different/mutated builds refuse execution.
- No install/update, sudo, preferences reset, cache deletion, process termination,
  project close/save, security changes or automatic retry are permitted here.

## Required automated sequence

1. Create a unique private Test Run directory and record runner identity.
2. Verify exact installed payload, signature, target architecture and candidate
   package/manifest.
3. Observe the live loaded plugin image path + UUID and map it to the pinned
   candidate. Disk identity alone is insufficient.
4. Only after live identity PASS, execute the guarded AE smoke in the existing
   empty test project.
5. Capture deterministic patterned frames:
   bypass, identity, static deformation x2, animated deformation x2, reset,
   Adjustment Layer + ElasticGrid before Corner Pin, identity Corner Pin, moved
   Corner Pin.
6. Pixel assertions:
   identity and reset preserve expected pixels; deformation changes enough pixels;
   static wave is stable across time; animated wave changes; adding identity
   Corner Pin does not change or black the frame; moved Corner Pin changes it.
7. Retain frame hashes and the synthetic test PNGs. No stale frame/result can be
   reused. Timeout/crash/missing output is not PASS.
8. Write one sanitized report ZIP. Raw process sample remains local/private.
9. Combined functional status may be PASS only when both live identity and all
   pixel checks PASS on the same run. Release status remains BLOCKED.

## Safety / cleanup

The JSX must refuse saved/occupied/dirty/unknown project state before mutation.
It creates only its own footage/compositions/effects, removes only those objects,
restores project bit depth, never closes the project and never touches user files.
A foreign-project/context change or cleanup error fails the run. Full stack/sample
data is not included in the shared report.

## Mandatory evidence before handoff

- Unit/control-flow negative and positive tests.
- macOS packaging test using the exact immutable fd69988 artifact; no rebuild.
- Exact package SHA-256 + source commit + included-file manifest.
- Existing C++/Rust/sanitizer gates stay green for the branch.
- Actual user target run remains NOT RUN until its returned report is inspected.

## Out of scope for this stage

Render/RAM Preview performance acceptance, guide drag/Undo/Redo, project migration,
8/16/32-bpc host matrix, aerender/MFR/cancellation, physical Metal, and approved
2D/3D perspective-plane implementation. Those remain subsequent gates.
