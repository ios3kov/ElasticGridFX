#!/bin/zsh
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
MANIFEST="$ROOT/host-rust/Cargo.toml"
TARGET="$ROOT/host-rust/target/release"
SOURCE="$TARGET/libelasticgrid_ae.dylib"
DEST_DIR="$HOME/Library/Application Support/AE Hot Loader/implementations/elasticgrid"
DEST="$DEST_DIR/current.dylib"
LABEL="${1:-elasticgrid-dev}"
RUST_TOOLCHAIN="1.98.1"

export MACOSX_DEPLOYMENT_TARGET="11.0"
export AE_HOT_LOADER_IMPL_LABEL="$LABEL"

echo "Verifying ElasticGrid host/state contract..."
python3 "$ROOT/tools/verify_shell_metadata.py" \
  "$ROOT/host-rust/build.rs" \
  "$ROOT/host-rust/shell/ElasticGridShell.cpp"
python3 "$ROOT/tools/verify_hot_reload_state.py" \
  "$ROOT/host-rust/src/lib.rs" \
  "$ROOT/host-rust/shell/ElasticGridShell.cpp"

USE_RUSTUP=0
if command -v rustup >/dev/null 2>&1; then
  rustup toolchain install "$RUST_TOOLCHAIN" --profile minimal >/dev/null
  USE_RUSTUP=1
else
  active_rust="$(rustc --version 2>/dev/null | awk '{print $2}' || true)"
  [[ "$active_rust" == "$RUST_TOOLCHAIN" ]] || {
    echo "ERROR: Rust $RUST_TOOLCHAIN required for hot-reload Runtime ABI compatibility."
    exit 4
  }
  command -v cargo >/dev/null 2>&1 || {
    echo "ERROR: cargo is required."
    exit 4
  }
fi

echo "Building ElasticGrid implementation: $LABEL"
if (( USE_RUSTUP )); then
  cargo +"$RUST_TOOLCHAIN" build --release --locked --manifest-path "$MANIFEST"
else
  cargo build --release --locked --manifest-path "$MANIFEST"
fi

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
