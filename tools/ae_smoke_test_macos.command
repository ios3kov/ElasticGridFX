#!/bin/bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
JSX="$ROOT/tests/ae_runtime_smoke.jsx"
PNG="/tmp/ElasticGridFX-v0.9-smoke.png"
DEST="/Library/Application Support/Adobe/Common/Plug-ins/7.0/MediaCore/ElasticGrid.plugin"

fail() { echo "ERROR: $*" >&2; exit 1; }

[[ "$(uname -s)" == "Darwin" ]] || fail "macOS only"
[[ -d "$DEST" ]] || fail "plugin is not installed: $DEST"
[[ -f "$JSX" ]] || fail "runtime JSX missing: $JSX"

AE_APP="$(find /Applications -maxdepth 2 -type d -name 'Adobe After Effects*.app' -print 2>/dev/null | sort -V | tail -n 1 || true)"
[[ -n "$AE_APP" ]] || fail "After Effects was not found in /Applications"
PLIST="$AE_APP/Contents/Info.plist"
[[ -f "$PLIST" ]] || fail "After Effects Info.plist missing: $PLIST"
APP_ID="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIdentifier' "$PLIST" 2>/dev/null || true)"
[[ -n "$APP_ID" ]] || fail "could not read After Effects bundle identifier"

echo "After Effects: $AE_APP"
echo "Bundle id: $APP_ID"
echo "Smoke: enumerate effect -> apply by matchName -> 32-bpc Final Bicubic render -> PNG"
rm -f "$PNG"

set +e
RESULT="$(osascript <<APPLESCRIPT
set jsxText to read POSIX file "$JSX" as «class utf8»
tell application id "$APP_ID"
    activate
    set resultCode to DoScript jsxText
end tell
return resultCode
APPLESCRIPT
)"
OSA_STATUS=$?
set -e

if [[ "$OSA_STATUS" -ne 0 ]]; then
  echo "ERROR: AppleScript could not control After Effects." >&2
  echo "macOS may ask you to allow Terminal to control After Effects in System Settings > Privacy & Security > Automation." >&2
  exit 30
fi

RESULT="$(echo "$RESULT" | tr -d '[:space:]')"
case "$RESULT" in
  0)
    ;;
  20)
    echo "ERROR: runtime smoke refuses to touch an existing/dirty After Effects project." >&2
    echo "Open a new empty unsaved project (0 items), then run this command again." >&2
    exit 20
    ;;
  21)
    echo "ERROR: ElasticGrid is not present in app.effects. Fully quit and restart After Effects after installation." >&2
    exit 21
    ;;
  22)
    echo "ERROR: After Effects sees the effect but could not apply it by match name." >&2
    exit 22
    ;;
  23)
    echo "ERROR: Effect parameters are incomplete or not addressable in the runtime host." >&2
    exit 23
    ;;
  24)
    echo "ERROR: this After Effects build does not expose saveFrameToPng for the automated smoke render." >&2
    exit 24
    ;;
  25)
    echo "ERROR: Final 32-bpc frame render did not produce the smoke PNG." >&2
    echo "If AE reports script file-access restrictions, enable Allow Scripts To Write Files And Access Network and retry." >&2
    exit 25
    ;;
  *)
    echo "ERROR: unexpected After Effects smoke result: ${RESULT:-<empty>}" >&2
    exit 29
    ;;
esac

[[ -s "$PNG" ]] || fail "After Effects returned success but PNG is missing/empty"
file "$PNG" || true
BYTES="$(stat -f '%z' "$PNG")"
echo "Rendered smoke frame: $BYTES bytes"
rm -f "$PNG"
echo "After Effects runtime smoke: PASS"
