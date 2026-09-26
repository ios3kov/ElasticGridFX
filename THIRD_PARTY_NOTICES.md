# Third-party build/runtime components

ElasticGrid FX is an independent clean-room implementation. No GridWarp source code is included.

Direct Rust dependencies are exact-pinned for the v1.0 freeze candidate:
- `after-effects = 0.4.0` — Apache-2.0 OR BSD-3-Clause OR MIT OR Zlib
- `pipl = 0.1.1` — MIT OR Apache-2.0
- `serde = 1.0.229` — MIT OR Apache-2.0
- `cc = 1.5.1` — MIT OR Apache-2.0
- `bincode = 2.0.1` — MIT

RustSec lists `RUSTSEC-2025-0141` for `bincode` as informational/unmaintained. It is kept visible in the compliance report; known vulnerabilities are not silently waived.

`tools/dependency_audit_macos.command` produces the authoritative target-Mac `Cargo.lock`, CycloneDX 1.5 `dist/compliance/sbom.cdx.json`, `dependencies.tsv`, `licenses-summary.txt`, `cargo-tree.txt`, `cargo-audit.json`, and SHA-256 hashes.
