#!/bin/bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"; HOST="$ROOT/host-rust"; MANIFEST="$HOST/Cargo.toml"; LOCK="$HOST/Cargo.lock"; OUT="$ROOT/dist/compliance"
mkdir -p "$OUT"
[[ "$(uname -s)" == Darwin ]] || { echo "ERROR: dependency freeze must run on target macOS."; exit 2; }
command -v cargo >/dev/null || { echo "ERROR: cargo required"; exit 3; }; command -v python3 >/dev/null || { echo "ERROR: python3 required"; exit 4; }
if [[ ! -f "$LOCK" ]]; then cargo generate-lockfile --manifest-path "$MANIFEST"; fi
cargo metadata --locked --format-version 1 --manifest-path "$MANIFEST" > "$OUT/cargo-metadata.json"
cargo tree --locked --manifest-path "$MANIFEST" > "$OUT/cargo-tree.txt"
python3 "$ROOT/tools/generate_sbom.py" "$OUT/cargo-metadata.json" "$OUT"
if ! cargo audit --version >/dev/null 2>&1; then echo "[deps] Installing cargo-audit..."; cargo install cargo-audit --locked; fi
set +e; (cd "$HOST" && cargo audit --json > "$OUT/cargo-audit.json"); rc=$?; set -e
python3 "$ROOT/tools/check_cargo_audit_json.py" "$OUT/cargo-audit.json"
printf '%s\n' "$rc" > "$OUT/cargo-audit-exit-code.txt"
shasum -a 256 "$LOCK" "$OUT/cargo-metadata.json" "$OUT/sbom.cdx.json" "$OUT/dependencies.tsv" "$OUT/cargo-audit.json" > "$OUT/compliance-sha256.txt"
echo "dependency/license/SBOM gate: PASS"
