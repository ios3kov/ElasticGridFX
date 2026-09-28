# Runtime acceptance tooling plan — 2026-09-28

Baseline: 37bc8bc924e07f11eef25c989490aec8ac0180ef; branch fix/final-validation.
Authoritative rules unchanged: FSTR-Line blob a1760fde8763f789b50b91c20407938b4fcaea4a.
Scope: install/test tooling, not deformation algorithms, host ABI or serialized state.

Before implementation, required acceptance:

1. No install/overwrite/delete on a default or inspection command. Replace the two
   unconditional sudo/rm paths with one inspected, explicit test-install operation.
   Require verified signed package/payload, native target, stopped Adobe hosts,
   conflict discovery and explicit existing test scope. Never replace an existing
   different build or remove old/user plugins; same verified build is idempotent.
   Refuse symlinks, scan errors, races detected before publication and corrupted
   packages. Retain diagnostics on failure. Tests use isolated fixtures only.
2. Replace flat-solid/nonempty-file smoke with a deterministic patterned input and
   disabled/identity/static-wave/time-varying/reset renders. Compare decoded pixels,
   not ZIP/PNG bytes; reject pass-through, flat/blank output, stale files and stale
   results. Use a unique run ID, per-run directory, explicit terminal result and
   bounded wait. Never accept the expected disk Build ID as observed loaded identity.
   Unknown AE project state refuses execution. Cleanup only test-owned objects;
   failed/foreign-context cleanup never becomes PASS. Retain all outputs.
3. Run portable negative/positive regression tests, C++ regression, exact-head CI
   and macOS source/package checks. Real AE and physical Metal remain BLOCKED in
   this Linux environment; mocked control flow/image fixtures prove tooling only.
   No user installation, main merge or release in this stage.

Installation is deliberately create-only. Upgrade/removal of existing builds is
not automated without an independently authorized reversible migration plan.
Actual host versions, custom plugin search paths and loaded Build ID still need
an authorized target runtime. This stage must not claim to resolve the reported
no-deformation issue without reproducing it inside AE.
