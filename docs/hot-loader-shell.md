# ElasticGrid — original-parity Hot Loader adapter

The previous Hot Loader branch was based on obsolete main render code and did not include the later `fix/original-gridwarp-parity` work. Real AE exposed that directly: custom guides moved while pixels could remain unchanged, and SmartFX produced seam/line artifacts.

This branch is now rebuilt from the parity tree, then the hardened Hot Loader shell/protocol is re-applied.

Identity remains ElasticGrid FX / `com.elasticgrid.fx.warp`; behavior and visible parameter surface are derived from the supplied GridWarp binary. Effect API 13.29, Protocol ABI 2, StateABI 5, Grid wire v3, Rust 1.98.1.

The first live correctness gate intentionally uses CPU SmartFX with full-source checkout. Metal returns only after real AE passes deformation with no seam artifacts. Every independent test uses the clean-run installer.
