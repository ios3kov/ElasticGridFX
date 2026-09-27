#!/bin/zsh
set -euo pipefail

ROOT="${0:A:h:h}"
MANIFEST="$ROOT/host-rust/Cargo.toml"
TARGET="$ROOT/host-rust/target/release"
DEST_DIR="$HOME/Library/Application Support/AE Hot Loader/implementations/elasticgrid"
DEST="$DEST_DIR/current.dylib"

export PATH="$HOME/.cargo/bin:$PATH"
export MACOSX_DEPLOYMENT_TARGET="11.0"

echo "[1/3] Build ElasticGrid implementation"
cargo build --release --locked --manifest-path "$MANIFEST"

SOURCE="$TARGET/libelasticgrid_ae.dylib"
[[ -f "$SOURCE" ]] || { echo "ERROR: missing $SOURCE"; exit 2; }

echo "[2/3] Stage signed implementation"
mkdir -p "$DEST_DIR"
TMP="$DEST_DIR/current.tmp.dylib"
rm -f "$TMP"
cp "$SOURCE" "$TMP"
codesign --force --sign - "$TMP"
mv -f "$TMP" "$DEST"
xattr -d com.apple.quarantine "$DEST" 2>/dev/null || true

echo "[3/3] Ready"
echo "  $DEST"
echo
echo "Keep After Effects open and click Reload Plugins in Window → AE Hot Loader."
