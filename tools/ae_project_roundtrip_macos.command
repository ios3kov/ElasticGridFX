#!/bin/bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
JSX="$ROOT/tests/ae_project_roundtrip.jsx"
DEST="/Library/Application Support/Adobe/Common/Plug-ins/7.0/MediaCore/ElasticGrid.plugin"
fail() { echo "ERROR: $*" >&2; exit 1; }

[[ "$(uname -s)" == "Darwin" ]] || fail "macOS only"
[[ -d "$DEST" ]] || fail "plugin is not installed: $DEST"
[[ -f "$JSX" ]] || fail "project roundtrip JSX missing: $JSX"
AE_APP="$(find /Applications -maxdepth 2 -type d -name 'Adobe After Effects*.app' -print 2>/dev/null | sort -V | tail -n 1 || true)"
[[ -n "$AE_APP" ]] || fail "After Effects was not found in /Applications"
APP_ID="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIdentifier' "$AE_APP/Contents/Info.plist" 2>/dev/null || true)"
[[ -n "$APP_ID" ]] || fail "could not read After Effects bundle identifier"

echo "Project roundtrip: save -> close -> reopen -> verify params/keyframes -> 32-bpc render"
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
STATUS=$?
set -e
[[ "$STATUS" -eq 0 ]] || { echo "ERROR: AppleScript could not control After Effects. Allow Terminal automation access and retry." >&2; exit 30; }
RESULT="$(echo "$RESULT" | tr -d '[:space:]')"
case "$RESULT" in
  0) echo "After Effects project roundtrip: PASS" ;;
  40) echo "ERROR: roundtrip refuses to touch an existing/dirty AE project. Open a new empty unsaved project." >&2; exit 40 ;;
  41) echo "ERROR: effect could not be applied by match name." >&2; exit 41 ;;
  42) echo "ERROR: required runtime parameters are missing." >&2; exit 42 ;;
  43) echo "ERROR: temporary AEP save failed." >&2; exit 43 ;;
  44) echo "ERROR: temporary AEP reopen failed." >&2; exit 44 ;;
  45) echo "ERROR: saved comp/effect was not restored." >&2; exit 45 ;;
  46) echo "ERROR: saved parameter values/keyframes changed after reopen." >&2; exit 46 ;;
  47) echo "ERROR: reopened project could not produce a 32-bpc frame." >&2; exit 47 ;;
  *) echo "ERROR: unexpected project roundtrip result: ${RESULT:-<empty>}" >&2; exit 49 ;;
esac
