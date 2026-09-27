#!/bin/bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BUNDLE="${1:-$ROOT/dist/mac/ElasticGrid.plugin}"
BIN="$BUNDLE/Contents/MacOS/ElasticGrid"
IMPL="$BUNDLE/Contents/Frameworks/libelasticgrid_impl.dylib"
RSRC="$BUNDLE/Contents/Resources/ElasticGrid.rsrc"
PLIST="$BUNDLE/Contents/Info.plist"

fail() { echo "ERROR: $*" >&2; exit 1; }

[[ "$(uname -s)" == "Darwin" ]] || fail "bundle verification must run on macOS"
[[ -d "$BUNDLE" ]] || fail "bundle missing: $BUNDLE"
[[ -f "$BIN" ]] || fail "shell binary missing: $BIN"
[[ -f "$IMPL" ]] || fail "implementation dylib missing: $IMPL"
[[ -f "$RSRC" ]] || fail "PiPL resource missing: $RSRC"
[[ -f "$PLIST" ]] || fail "Info.plist missing: $PLIST"

printf '[bundle] plist\n'
plutil -lint "$PLIST"
/usr/libexec/PlistBuddy -c 'Print :CFBundleIdentifier' "$PLIST"
/usr/libexec/PlistBuddy -c 'Print :CFBundleExecutable' "$PLIST"

printf '[bundle] shell binary\n'
file "$BIN"
if command -v lipo >/dev/null 2>&1; then
  lipo -archs "$BIN" || true
fi

printf '[bundle] shell AE/hot-reload exports\n'
SHELL_SYMS="$(nm -gU "$BIN")"
for sym in EffectMain PluginDataEntryFunction2 AEHotLoader_ShellReload; do
  echo "$SHELL_SYMS" | grep -Eq "[[:space:]]_${sym}$" || fail "${sym} export missing from shell"
done
echo "$SHELL_SYMS" | grep -E '_EffectMain$|_PluginDataEntryFunction2$|_AEHotLoader_ShellReload$'

printf '[bundle] implementation ABI exports\n'
IMPL_SYMS="$(nm -gU "$IMPL")"
for sym in EffectMain AEHotLoader_ImplementationABI AEHotLoader_ImplementationStateABI AEHotLoader_ImplementationKey AEHotLoader_ImplementationLabel; do
  echo "$IMPL_SYMS" | grep -Eq "[[:space:]]_${sym}$" || fail "${sym} export missing from implementation"
done
echo "$IMPL_SYMS" | grep -E '_EffectMain$|_AEHotLoader_Implementation'

printf '[bundle] dynamic dependencies\n'
otool -L "$BIN"
otool -L "$IMPL"

printf '[bundle] code signature\n'
codesign --verify --deep --strict --verbose=2 "$BUNDLE"
codesign -dv --verbose=2 "$BUNDLE" 2>&1 | grep -E 'Identifier=|TeamIdentifier=|Signature=' || true

printf '[bundle] PiPL sanity\n'
strings "$RSRC" | grep -F 'ElasticGrid FX' >/dev/null || fail "ElasticGrid FX name not found in PiPL resource"
strings "$RSRC" | grep -F 'com.elasticgrid.fx.warp' >/dev/null || fail "match name not found in PiPL resource"

printf '[bundle] hashes\n'
shasum -a 256 "$BIN" "$IMPL" "$RSRC" "$PLIST"

echo 'bundle verification: PASS'
