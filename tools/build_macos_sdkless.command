#!/bin/bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
MANIFEST="$ROOT/host-rust/Cargo.toml"
TARGET="$ROOT/host-rust/target/release"
DIST="$ROOT/dist/mac"
BUNDLE="$DIST/ElasticGrid.plugin"
REPORT="$DIST/preflight-report.txt"
if [[ "$#" -ne 0 ]]; then
  echo "ERROR: build-only command; implicit installation has been removed." >&2
  echo "Inspect the exact package with tools/install_macos.command --help." >&2
  exit 2
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

command -v python3 >/dev/null 2>&1 || {
  echo "ERROR: python3 is required for toolchain version checks."
  exit 4
}

rust_meets_minimum() {
  local version="$1"
  python3 - "$version" <<'PYVER'
import re, sys
m = re.match(r"^(\d+)\.(\d+)\.(\d+)", sys.argv[1])
if not m:
    raise SystemExit(1)
v = tuple(map(int, m.groups()))
raise SystemExit(0 if v >= (1, 85, 0) else 1)
PYVER
}

install_user_rust() {
  echo "Installing/updating user-local stable Rust (>=1.85 required)..."
  command -v curl >/dev/null 2>&1 || {
    echo "ERROR: curl is required to install Rust."
    exit 4
  }
  if ! command -v rustup >/dev/null 2>&1; then
    RUSTUP_INIT_SKIP_PATH_CHECK=yes curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
      RUSTUP_INIT_SKIP_PATH_CHECK=yes sh -s -- -y --profile minimal --default-toolchain stable
  fi
  export PATH="$HOME/.cargo/bin:$PATH"
  # shellcheck disable=SC1090
  [[ -f "$HOME/.cargo/env" ]] && source "$HOME/.cargo/env"
  rustup toolchain install stable --profile minimal >/dev/null
  rustup default stable >/dev/null
  rustup component add clippy --toolchain stable >/dev/null
  hash -r
}

NEED_USER_RUST=0
if ! command -v cargo >/dev/null 2>&1 || ! command -v rustc >/dev/null 2>&1; then
  NEED_USER_RUST=1
else
  RUST_VERSION="$(rustc --version | awk '{print $2}')"
  if ! rust_meets_minimum "$RUST_VERSION"; then
    echo "Existing Rust $RUST_VERSION is too old; need >=1.85."
    NEED_USER_RUST=1
  elif ! cargo clippy --version >/dev/null 2>&1; then
    echo "Existing Rust is missing Clippy."
    NEED_USER_RUST=1
  fi
fi

if [[ "$NEED_USER_RUST" == "1" ]]; then
  install_user_rust
fi

export PATH="$HOME/.cargo/bin:$PATH"
# shellcheck disable=SC1090
[[ -f "$HOME/.cargo/env" ]] && source "$HOME/.cargo/env"

RUST_VERSION="$(rustc --version | awk '{print $2}')"
rust_meets_minimum "$RUST_VERSION" || {
  echo "ERROR: Rust >=1.85 required, active rustc is $RUST_VERSION"
  exit 4
}
cargo clippy --version >/dev/null 2>&1 || {
  echo "ERROR: Clippy unavailable after Rust bootstrap."
  exit 4
}

cargo --version
rustc --version

export MACOSX_DEPLOYMENT_TARGET="11.0"
mkdir -p "$DIST"
: > "$REPORT"

echo "[1/6] Running audit/debug/parity/performance preflight..."
set -o pipefail
"$ROOT/tools/preflight_macos.command" 2>&1 | tee -a "$REPORT"

echo "[2/6] Building ElasticGrid FX v0.9 (native $(uname -m))..." | tee -a "$REPORT"
cargo build --release --locked --manifest-path "$MANIFEST" --message-format=json-render-diagnostics \
  2>&1 | tee "$DIST/host-build.jsonl" | tee -a "$REPORT"

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

python3 "$ROOT/tools/build_identity.py" stamp --root "$ROOT" --bundle "$BUNDLE" --cargo-log "$DIST/host-build.jsonl"

xattr -cr "$BUNDLE" || true
codesign --force --deep --sign - "$BUNDLE"

echo "[4/6] Verifying bundle/entrypoints/signature..." | tee -a "$REPORT"
"$ROOT/tools/verify_bundle_macos.command" "$BUNDLE" 2>&1 | tee -a "$REPORT"

# Hash/sign first; archive and manifest refer to these exact bytes, never a rebuild.
python3 "$ROOT/tools/build_identity.py" seal --bundle "$BUNDLE" \
  --package "$DIST/ElasticGrid.plugin.zip" --out "$DIST/ElasticGrid.artifact.json"
python3 "$ROOT/tools/build_identity.py" verify --bundle "$BUNDLE" \
  --package "$DIST/ElasticGrid.plugin.zip" --manifest "$DIST/ElasticGrid.artifact.json"
printf '[artifact] signed payload + package manifest verified (AE runtime NOT RUN)\n' | tee -a "$REPORT"
shasum -a 256 "$DIST/ElasticGrid.plugin.zip" "$DIST/ElasticGrid.artifact.json" | tee -a "$REPORT"
printf '[artifact] sha256\n' | tee -a "$REPORT"
shasum -a 256 "$BUNDLE/Contents/MacOS/ElasticGrid" "$BUNDLE/Contents/Resources/ElasticGrid.rsrc" | tee -a "$REPORT"

echo "[5/6] No installation performed. Candidate requires separate test authorization." | tee -a "$REPORT"

echo "[6/6] Final report" | tee -a "$REPORT"
echo "preflight: PASS" | tee -a "$REPORT"
echo "bundle: PASS" | tee -a "$REPORT"
echo "report: $REPORT" | tee -a "$REPORT"
echo "Inspect candidate: tools/install_macos.command (read-only). AE runtime remains NOT RUN." | tee -a "$REPORT"
