#!/bin/bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$ROOT/dist/compliance"
mkdir -p "$OUT"

[[ "$(uname -s)" == Darwin ]] || { echo "ERROR: reproducible plugin build must run on macOS."; exit 2; }
command -v cargo >/dev/null || { echo "ERROR: cargo required"; exit 3; }

LOCK="$ROOT/host-rust/Cargo.lock"
[[ -f "$LOCK" ]] || { echo "ERROR: Cargo.lock missing"; exit 5; }

TMP="$(mktemp -d "${TMPDIR:-/tmp}/elasticgrid-repro.XXXXXX")"
trap 'rm -rf "$TMP"' EXIT

SRC="$TMP/src"
CH="$TMP/cargo-home"
TARGET="$TMP/target"
FIRST="$TMP/first-build"
mkdir -p "$SRC" "$CH" "$FIRST"

tar -C "$ROOT" \
  --exclude='./.git' \
  --exclude='./dist' \
  --exclude='./host-rust/target' \
  --exclude='./.preflight-macos' \
  --exclude='./build' \
  --exclude='./build-*' \
  -cf - . | tar -C "$SRC" -xf -

# Export the clean source identity, then validate the copied bytes on every build.
python3 "$ROOT/tools/build_identity.py" snapshot --root "$ROOT" --out "$TMP/source-record.json"
export ELASTICGRID_SOURCE_RECORD="$TMP/source-record.json"
MANIFEST="$SRC/host-rust/Cargo.toml"
export MACOSX_DEPLOYMENT_TARGET=11.0
export CARGO_INCREMENTAL=0

CARGO_HOME="$CH" cargo fetch --locked --manifest-path "$MANIFEST"

FILES=(
  libelasticgrid_ae.dylib
  elasticgrid_ae.rsrc
  elasticgrid_ae_PkgInfo
  elasticgrid_ae_Info.plist
)

echo "[repro 1/3] First clean offline Release build"
CARGO_HOME="$CH" CARGO_TARGET_DIR="$TARGET" CARGO_NET_OFFLINE=true \
  cargo build --release --locked --manifest-path "$MANIFEST"

for f in "${FILES[@]}"; do
  src="$TARGET/release/$f"
  [[ -f "$src" ]] || { echo "ERROR: missing first-build $f"; exit 6; }
  cp "$src" "$FIRST/$f"
done

# Rebuild from scratch at the same canonical target path. Cargo/rustc do not
# promise that artifacts are path-independent when CARGO_TARGET_DIR changes,
# so changing the build directory would test path variance rather than
# deterministic rebuilds under identical settings.
rm -rf "$TARGET"

echo "[repro 2/3] Second clean offline Release build"
CARGO_HOME="$CH" CARGO_TARGET_DIR="$TARGET" CARGO_NET_OFFLINE=true \
  cargo build --release --locked --manifest-path "$MANIFEST"

: > "$OUT/repro-build-sha256.txt"
for f in "${FILES[@]}"; do
  aa="$FIRST/$f"
  bb="$TARGET/release/$f"
  [[ -f "$aa" && -f "$bb" ]] || { echo "ERROR: missing second-build $f"; exit 6; }

  if ! cmp -s "$aa" "$bb"; then
    echo "ERROR: reproducible-build mismatch $f"
    shasum -a 256 "$aa" "$bb" || true
    echo "First differing byte positions (decimal):"
    cmp -l "$aa" "$bb" | head -32 || true
    if [[ "$f" == "libelasticgrid_ae.dylib" ]] && command -v dwarfdump >/dev/null 2>&1; then
      echo "Mach-O UUIDs:"
      dwarfdump --uuid "$aa" "$bb" || true
    fi
    exit 7
  fi

  shasum -a 256 "$aa" | sed "s#$aa#$f#" >> "$OUT/repro-build-sha256.txt"
done

echo "[repro 3/3] Locked offline tests"
CARGO_HOME="$CH" CARGO_TARGET_DIR="$TMP/target-test" CARGO_NET_OFFLINE=true CARGO_INCREMENTAL=0 \
  cargo test --release --locked --manifest-path "$MANIFEST"

echo "clean reproducible build gate: PASS"
