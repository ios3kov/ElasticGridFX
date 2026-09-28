# ElasticGrid — original-parity Hot Loader adapter

The previous Hot Loader branch was based on obsolete main render code and did not include the later `fix/original-gridwarp-parity` work. Real AE exposed that directly: custom guides moved while pixels could remain unchanged, and SmartFX produced seam/line artifacts.

This branch is now rebuilt from the parity tree, then the hardened Hot Loader shell/protocol is re-applied.

Identity remains ElasticGrid FX / `com.elasticgrid.fx.warp`; behavior and visible parameter surface are derived from the supplied GridWarp binary. Effect API 13.29, Protocol ABI 2, StateABI 5, Grid wire v3, Rust 1.98.1.

The first live correctness gate intentionally uses CPU SmartFX with full-source checkout. Metal returns only after real AE passes deformation with no seam artifacts. Every independent test uses the clean-run installer.


## Live status — 2026-09-28

The original-parity clean live gate is **FAILED** in real After Effects 25.6.

Observed:
- the effect loads;
- the custom grid UI is visible and draggable;
- rendered pixels do not follow the dragged guides correctly;
- bright vertical/horizontal render artifacts appear.

This means the current implementation is not accepted as a GridWarp parity baseline even though CI is green.

No further user manual build is allowed until the automatic/research phase is complete. The next stage is binary decomposition + Adobe SDK/open-source comparison + deterministic render/UI regression harness + code audit/debugging/profiling. A new user test package is produced only after those gates are exhausted.
