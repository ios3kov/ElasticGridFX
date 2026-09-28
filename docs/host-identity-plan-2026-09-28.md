# Stage plan — 2026-09-28
Baseline e1aa77e79cea7a4b1c63e7285871338d8ec76224; branch fix/final-validation.
Rules rechecked: FSTR-Line DEVELOPMENT_RULES blob a1760fde8763f789b50b91c20407938b4fcaea4a.

1. Host metadata: fix visible Falloff labels to match current numeric FFI behavior,
   bound guide/tension/frequency UI to actual core limits and expose degree phase.
   Retain all 17 Params identifiers, order, types, existing popup ordinals and
   wire schema. Keep the unused legacy wave checkbox serialized but hidden;
   amplitude=0 disables waves as on this branch. No algorithm changes.
   Retain new-instance default ordinal 2 to avoid an unrelated default change.
   Tests: before/after source-contract checks, core regression, Mac Rust tests
   including exact expected falloff output for all four saved ordinals.
2. Build identity: generate deterministic commit/source-state/source-hash,
   target/toolchain/settings-aware Build ID; expose it in About and the existing
   custom control (no new serialized parameter). Package the matching metadata,
   hash all signed payload files and final ZIP. Reject dirty final packaging,
   mismatched/stale metadata and modified source snapshots. Test identity
   invalidation, repo-less snapshot, git worktrees, tampering and manifests.
   Repro build must consume the same verified source identity, not an arbitrary
   env commit string. Real loaded AE identity remains BLOCKED without AE.
3. Check exact resulting GitHub commit with Linux CI and Mac compile/package gate.
   Source tests or presence of generated metadata are not AE runtime evidence.
   Do not merge, install, publish release, clear user preferences or kill AE.
