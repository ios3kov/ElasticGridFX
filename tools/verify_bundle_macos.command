#!/bin/bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BUNDLE="${1:-$ROOT/dist/mac/ElasticGrid.plugin}"
BIN="$BUNDLE/Contents/MacOS/ElasticGrid"
RSRC="$BUNDLE/Contents/Resources/ElasticGrid.rsrc"
IDENTITY="$BUNDLE/Contents/Resources/BuildIdentity.txt"
PLIST="$BUNDLE/Contents/Info.plist"

fail() { echo "ERROR: $*" >&2; exit 1; }

[[ "$(uname -s)" == "Darwin" ]] || fail "bundle verification must run on macOS"
[[ -d "$BUNDLE" ]] || fail "bundle missing: $BUNDLE"
[[ -f "$BIN" ]] || fail "binary missing: $BIN"
[[ -f "$RSRC" ]] || fail "PiPL resource missing: $RSRC"
[[ -f "$IDENTITY" ]] || fail "BuildIdentity missing: $IDENTITY"
[[ -f "$PLIST" ]] || fail "Info.plist missing: $PLIST"

printf '[bundle] plist\n'
plutil -lint "$PLIST"
/usr/libexec/PlistBuddy -c 'Print :CFBundleIdentifier' "$PLIST"
/usr/libexec/PlistBuddy -c 'Print :CFBundleExecutable' "$PLIST"
BUILD_ID="$(sed -n 's/^build_id=//p' "$IDENTITY")"
GIT_COMMIT="$(sed -n 's/^git_commit=//p' "$IDENTITY")"
GIT_STATE="$(sed -n 's/^git_state=//p' "$IDENTITY")"
[[ -n "$BUILD_ID" && -n "$GIT_COMMIT" && -n "$GIT_STATE" ]] || fail "BuildIdentity fields missing"
[[ "$(/usr/libexec/PlistBuddy -c 'Print :ElasticGridBuildID' "$PLIST")" == "$BUILD_ID" ]] || fail "Build ID plist mismatch"
[[ "$(/usr/libexec/PlistBuddy -c 'Print :ElasticGridGitCommit' "$PLIST")" == "$GIT_COMMIT" ]] || fail "Git commit plist mismatch"
[[ "$(/usr/libexec/PlistBuddy -c 'Print :ElasticGridGitState' "$PLIST")" == "$GIT_STATE" ]] || fail "Git state plist mismatch"
printf '[bundle] build identity\n'
cat "$IDENTITY"

printf '[bundle] binary\n'
file "$BIN"
if command -v lipo >/dev/null 2>&1; then
  lipo -archs "$BIN" || true
fi

printf '[bundle] exported AE entry points\n'
SYMS="$(nm -gU "$BIN")"
echo "$SYMS" | grep -Eq '[[:space:]]_EffectMain$' || fail "EffectMain export missing"
echo "$SYMS" | grep -Eq '[[:space:]]_PluginDataEntryFunction2$' || fail "PluginDataEntryFunction2 export missing"
echo "$SYMS" | grep -E '_EffectMain$|_PluginDataEntryFunction2$'

printf '[bundle] dynamic dependencies\n'
otool -L "$BIN"

printf '[bundle] code signature\n'
codesign --verify --deep --strict --verbose=2 "$BUNDLE"
codesign -dv --verbose=2 "$BUNDLE" 2>&1 | grep -E 'Identifier=|TeamIdentifier=|Signature=' || true

printf '[bundle] PiPL sanity\n'
strings "$RSRC" | grep -F 'ElasticGrid FX' >/dev/null || fail "ElasticGrid FX name not found in PiPL resource"
strings "$RSRC" | grep -F 'com.elasticgrid.fx.warp' >/dev/null || fail "match name not found in PiPL resource"

printf '[bundle] hashes\n'
shasum -a 256 "$BIN" "$RSRC" "$IDENTITY" "$PLIST"

echo 'bundle verification: PASS'
