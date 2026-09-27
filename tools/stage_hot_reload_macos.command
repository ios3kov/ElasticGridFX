#!/bin/zsh
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
MANIFEST="$ROOT/host-rust/Cargo.toml"
TARGET="$ROOT/host-rust/target/release"
SOURCE="$TARGET/libelasticgrid_ae.dylib"
DEST_DIR="$HOME/Library/Application Support/AE Hot Loader/implementations/elasticgrid"
DEST="$DEST_DIR/current.dylib"
LABEL="${1:-elasticgrid-dev}"

export MACOSX_DEPLOYMENT_TARGET="11.0"
export AE_HOT_LOADER_IMPL_LABEL="$LABEL"

echo "Building ElasticGrid implementation: $LABEL"
cargo build --release --locked --manifest-path "$MANIFEST"

[[ -f "$SOURCE" ]] || { echo "ERROR: missing implementation: $SOURCE"; exit 2; }

mkdir -p "$DEST_DIR"
TMP="$DEST_DIR/current.tmp.dylib"
rm -f "$TMP"
cp "$SOURCE" "$TMP"
codesign --force --sign - "$TMP"
xattr -d com.apple.quarantine "$TMP" 2>/dev/null || true
mv -f "$TMP" "$DEST"

echo
echo "Staged ElasticGrid implementation:"
echo "  $DEST"
echo "  label=$LABEL"
echo
echo "Keep After Effects open and click Window → AE Hot Loader → Reload Plugins."
