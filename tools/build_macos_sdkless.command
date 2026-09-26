#!/bin/bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
MANIFEST="$ROOT/host-rust/Cargo.toml"
TARGET="$ROOT/host-rust/target/release"
DIST="$ROOT/dist/mac"
BUNDLE="$DIST/ElasticGrid.plugin"
REPORT="$DIST/preflight-report.txt"
INSTALL=0

if [[ "${1:-}" == "--install" ]]; then
  INSTALL=1
fi

if [[ "$(uname -s)" != "Darwin" ]]; then
  echo "ERROR: this script must be run on macOS."
  exit 2
fi

if ! xcode-select -p >/dev/null 2>&1; then
  echo "Xcode Command Line Tools are required. Starting installer..."
  xcode-select --install || true
  echo "Re-run this script after installation finishes."
  exit 3
fi

if ! command -v cargo >/dev/null 2>&1; then
  echo "Rust is not installed. Installing the official minimal toolchain..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
  # shellcheck disable=SC1090
  source "$HOME/.cargo/env"
fi

if command -v rustup >/dev/null 2>&1 && ! cargo clippy --version >/dev/null 2>&1; then
  echo "Installing Rust Clippy component for the mandatory static-analysis gate..."
  rustup component add clippy
fi

export MACOSX_DEPLOYMENT_TARGET="11.0"
mkdir -p "$DIST"
: > "$REPORT"

echo "[1/6] Running audit/debug/parity/performance preflight..."
set -o pipefail
"$ROOT/tools/preflight_macos.command" 2>&1 | tee -a "$REPORT"

echo "[2/6] Building ElasticGrid FX v0.9 (native $(uname -m))..." | tee -a "$REPORT"
cargo build --release --locked --manifest-path "$MANIFEST" 2>&1 | tee -a "$REPORT"

echo "[3/6] Creating After Effects .plugin bundle..." | tee -a "$REPORT"
rm -rf "$BUNDLE"
mkdir -p "$BUNDLE/Contents/MacOS" "$BUNDLE/Contents/Resources"

cp "$TARGET/libelasticgrid_ae.dylib" "$BUNDLE/Contents/MacOS/ElasticGrid"
cp "$TARGET/elasticgrid_ae.rsrc" "$BUNDLE/Contents/Resources/ElasticGrid.rsrc"
cp "$TARGET/elasticgrid_ae_PkgInfo" "$BUNDLE/Contents/PkgInfo"
cp "$TARGET/elasticgrid_ae_Info.plist" "$BUNDLE/Contents/Info.plist"

/usr/libexec/PlistBuddy -c 'Set :CFBundleIdentifier com.elasticgrid.fx' "$BUNDLE/Contents/Info.plist" >/dev/null
/usr/libexec/PlistBuddy -c 'Add :CFBundleExecutable string ElasticGrid' "$BUNDLE/Contents/Info.plist" >/dev/null 2>&1 || \
  /usr/libexec/PlistBuddy -c 'Set :CFBundleExecutable ElasticGrid' "$BUNDLE/Contents/Info.plist" >/dev/null
/usr/libexec/PlistBuddy -c 'Add :CFBundleName string ElasticGrid FX' "$BUNDLE/Contents/Info.plist" >/dev/null 2>&1 || \
  /usr/libexec/PlistBuddy -c 'Set :CFBundleName ElasticGrid FX' "$BUNDLE/Contents/Info.plist" >/dev/null
/usr/libexec/PlistBuddy -c 'Add :CFBundleShortVersionString string 0.9.0' "$BUNDLE/Contents/Info.plist" >/dev/null 2>&1 || \
  /usr/libexec/PlistBuddy -c 'Set :CFBundleShortVersionString 0.9.0' "$BUNDLE/Contents/Info.plist" >/dev/null
/usr/libexec/PlistBuddy -c 'Add :CFBundleVersion string 9' "$BUNDLE/Contents/Info.plist" >/dev/null 2>&1 || \
  /usr/libexec/PlistBuddy -c 'Set :CFBundleVersion 9' "$BUNDLE/Contents/Info.plist" >/dev/null

xattr -cr "$BUNDLE" || true
codesign --force --deep --sign - "$BUNDLE"

echo "[4/6] Verifying bundle/entrypoints/signature..." | tee -a "$REPORT"
"$ROOT/tools/verify_bundle_macos.command" "$BUNDLE" 2>&1 | tee -a "$REPORT"

printf '[artifact] sha256\n' | tee -a "$REPORT"
shasum -a 256 "$BUNDLE/Contents/MacOS/ElasticGrid" "$BUNDLE/Contents/Resources/ElasticGrid.rsrc" | tee -a "$REPORT"

if [[ "$INSTALL" == "1" ]]; then
  echo "[5/6] Installing into Adobe MediaCore..." | tee -a "$REPORT"
  DEST="/Library/Application Support/Adobe/Common/Plug-ins/7.0/MediaCore"
  sudo mkdir -p "$DEST"
  sudo rm -rf "$DEST/ElasticGrid.plugin"
  sudo cp -R "$BUNDLE" "$DEST/ElasticGrid.plugin"
  sudo xattr -cr "$DEST/ElasticGrid.plugin" || true
  codesign --verify --deep --strict "$DEST/ElasticGrid.plugin"
  echo "Installed: $DEST/ElasticGrid.plugin" | tee -a "$REPORT"
else
  echo "[5/6] Install skipped." | tee -a "$REPORT"
  echo "Build ready: $BUNDLE" | tee -a "$REPORT"
fi

echo "[6/6] Final report" | tee -a "$REPORT"
echo "preflight: PASS" | tee -a "$REPORT"
echo "bundle: PASS" | tee -a "$REPORT"
echo "report: $REPORT" | tee -a "$REPORT"
if [[ "$INSTALL" == "1" ]]; then
  echo "Next: fully quit/reopen After Effects into a NEW EMPTY project, then run tools/ae_runtime_check_macos.command --smoke." | tee -a "$REPORT"
else
  echo "To build + install: $0 --install" | tee -a "$REPORT"
fi
