# Density candidate verification — 2026-09-30

Initial candidate e32eec9 / tree dceaa93: hosted macOS Rust tests **64/64 PASS**
in run36757856008, job110032856631. Strict Clippy then FAIL: the test-only
legacy dense-render declaration was not exercised. This run remains FAIL.

Follow-up adds a real Rust-to-production-FFI dense legacy/detail equality test
rather than disabling the warning. Additional cases assert exact old v3 bytes,
reject malformed v4 maps and exercise row/combined local controls without
mutating the other axis or another key. No rendering/UI implementation change.
All final tests/build results must be associated with the follow-up commit.

Native AE runtime and installed/loaded identity remain NOT RUN. A full source
build is not a substitute for actual key/frame checks in AE. Main, PR #10's
candidate, release and user's installation remain unchanged.
