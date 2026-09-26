# ElasticGrid FX — v1.0 Code Freeze Checklist

| Step | Gate | Status | Evidence |
|---|---|---|---|
| 1 | Static analysis | **C++ PASS / RUST CLIPPY TARGET-MAC PENDING** | Clang high-warning + Static Analyzer clean after fixes; Mac preflight mandates Clippy. |
| 2 | Manual unsafe / FFI audit | **CODE PASS / RUST TARGET-MAC COMPILE PENDING** | ABI layouts frozen; hostile GridState allocation bounded; ROI arithmetic hardened; panic/error paths reviewed. |
| 3 | Dependencies / licenses / SBOM | **SOURCE/POLICY PASS / TARGET-MAC LOCK+AUDIT+SBOM PENDING** | Direct versions pinned; Cargo.lock/RustSec/CycloneDX gate added. |
| 4 | Clean reproducible build | **PORTABLE CLEAN BUILD PASS / HOSTED-MAC REPRO RERUN PENDING** | Portable static core reproduced byte-for-byte; Mac verifier now performs two clean offline builds at one canonical target path and reports binary-level mismatch diagnostics. |
| 5 | Final regression after fixes | **PORTABLE PASS / TARGET-MAC METAL+RUST PENDING** | Release 9/9; ASan/UBSan/LSan PASS; GCC TSan MFR+determinism PASS; final performance smoke recorded. |
| 6 | Code freeze | **FROZEN** | Aggregate code hash `e8d043c7bbab6d7674887558c8c80d6afb55d02ba1e3e9a3cf087236074ddeb3`; see `code-freeze-manifest-v1.0.txt`. |
| 7 | Mac/AE runtime gate | **TARGET-MAC PENDING** | Metal/Rust/plugin/AE integration must run on the user's Mac. |

## Step 1 — Static analysis
Clang `-Weverything` focused audit and Clang Static Analyzer cover all portable core/bridge translation units. Fixes include explicit invalid-enum fallbacks, bit-exact float identity comparisons, warning-clean array indexing, and an out-of-line `RenderCancelled` destructor. `tools/static_analysis.command` also runs clang-tidy/cppcheck when installed and makes `cargo clippy --all-targets -- -D warnings` mandatory on Mac.

## Step 2 — Manual unsafe / FFI audit
The Rust/C++ render ABI is pinned with size/offset assertions, extreme ROI subtraction is done in 64-bit before narrowing, hostile serialized grid vectors are bounded before allocation, pin bytes must be canonical, production arbitrary-data initialization propagates allocation errors instead of unwrapping, Metal `Send/Sync` invariants are documented, and runtime GPU failures are classified as internal failures rather than user-parameter errors.

## Step 3 — Dependencies / licenses / SBOM
Direct dependencies are exact-pinned. `dependency_audit_macos.command` resolves/fixes the target-Mac lockfile, runs RustSec, generates CycloneDX 1.5 + license/dependency inventories and hashes them. `RUSTSEC-2025-0141` (bincode unmaintained) is explicitly recorded as informational rather than hidden. Full transitive evidence is target-Mac pending because Cargo is unavailable here.

## Step 4 — Clean reproducible build
`repro_build_macos.command` creates a clean source snapshot, uses a fresh isolated Cargo registry, fetches only the frozen lock graph, and makes two independent locked/offline Release builds. The target directory is fully deleted between builds but recreated at the same canonical path, so byte-for-byte comparison measures deterministic rebuilding under identical settings rather than path-dependent Cargo/rustc metadata. The gate compares unsigned dylib/PiPL/PkgInfo/plist outputs and reruns locked tests offline. Any mismatch records hashes, initial differing byte offsets, and Mach-O UUIDs for the dylib. The portable C++ static library was independently reproduced byte-for-byte from two clean copies in this environment.

## Step 5 — Final regression

Post-hardening portable regression is green: 9/9 Release tests, ASan/UBSan/LeakSanitizer, GCC TSan MFR/determinism, fuzz, soak, determinism, allocation audit and Final Bicubic reference checks. See `final-regression-v1.0.md`.

## Step 6 — Code freeze

Functional source is frozen with aggregate hash `e8d043c7bbab6d7674887558c8c80d6afb55d02ba1e3e9a3cf087236074ddeb3`. No functional code changes are allowed before the target-Mac/After Effects gate unless that gate reveals a blocker. The exact per-file hashes are in `code-freeze-manifest-v1.0.txt`. Shipping metadata remains at 0.9.0 until the target-Mac gate passes; a release-only version bump is allowed after PASS. `Cargo.lock` is intentionally generated/frozen by the target-Mac dependency gate because Cargo is unavailable in the Linux validation container.

## Target-Mac validation amendments

Code freeze permits only changes required by a failed target-Mac gate. First hosted-Mac Rust compile exposed two such blockers: the AE host trait requires `handle_command(&mut self, ...)`, and modern rustc check-cfg requires explicit registration of cfg names emitted by the pinned `after-effects 0.4.0` macro. These are compatibility/build fixes only; render algorithms and quality paths are unchanged.

The hosted-Mac reproducibility run later reached step 20/20 and found that the verifier itself compared artifacts produced under two different `CARGO_TARGET_DIR` paths. Because Cargo/rustc build metadata can be path-sensitive, the verifier was hardened to do two fully clean builds at the same target path. This changes only validation methodology, not plugin rendering or runtime behavior.
