#!/bin/bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"; OUT="$ROOT/dist/compliance"; mkdir -p "$OUT"
[[ "$(uname -s)" == Darwin ]] || { echo "ERROR: reproducible plugin build must run on macOS."; exit 2; }
command -v cargo >/dev/null || { echo "ERROR: cargo required"; exit 3; }
LOCK="$ROOT/host-rust/Cargo.lock"; [[ -f "$LOCK" ]] || { echo "ERROR: Cargo.lock missing"; exit 5; }
TMP="$(mktemp -d "${TMPDIR:-/tmp}/elasticgrid-repro.XXXXXX")"; trap 'rm -rf "$TMP"' EXIT
SRC="$TMP/src"; CH="$TMP/cargo-home"; A="$TMP/target-a"; B="$TMP/target-b"; mkdir -p "$SRC" "$CH"
tar -C "$ROOT" --exclude='./.git' --exclude='./dist' --exclude='./host-rust/target' --exclude='./.preflight-macos' --exclude='./build' --exclude='./build-*' -cf - . | tar -C "$SRC" -xf -
MANIFEST="$SRC/host-rust/Cargo.toml"; export MACOSX_DEPLOYMENT_TARGET=11.0
CARGO_HOME="$CH" cargo fetch --locked --manifest-path "$MANIFEST"
CARGO_HOME="$CH" CARGO_TARGET_DIR="$A" CARGO_NET_OFFLINE=true cargo build --release --locked --manifest-path "$MANIFEST"
CARGO_HOME="$CH" CARGO_TARGET_DIR="$B" CARGO_NET_OFFLINE=true cargo build --release --locked --manifest-path "$MANIFEST"
FILES=(libelasticgrid_ae.dylib elasticgrid_ae.rsrc elasticgrid_ae_PkgInfo elasticgrid_ae_Info.plist); : > "$OUT/repro-build-sha256.txt"
for f in "${FILES[@]}"; do
  aa="$A/release/$f"; bb="$B/release/$f"; [[ -f "$aa" && -f "$bb" ]] || { echo "ERROR: missing $f"; exit 6; }
  cmp -s "$aa" "$bb" || { echo "ERROR: reproducible-build mismatch $f"; shasum -a 256 "$aa" "$bb"; exit 7; }
  shasum -a 256 "$aa" | sed "s#$aa#$f#" >> "$OUT/repro-build-sha256.txt"
done
CARGO_HOME="$CH" CARGO_TARGET_DIR="$TMP/target-test" CARGO_NET_OFFLINE=true cargo test --release --locked --manifest-path "$MANIFEST"
echo "clean reproducible build gate: PASS"
