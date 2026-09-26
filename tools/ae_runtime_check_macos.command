#!/bin/bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"
DEST="/Library/Application Support/Adobe/Common/Plug-ins/7.0/MediaCore/ElasticGrid.plugin"
LAUNCH=0
SMOKE=0
ROUNDTRIP=0
FULL=0
for arg in "$@"; do
  case "$arg" in
    --launch) LAUNCH=1 ;;
    --smoke) SMOKE=1 ;;
    --project-roundtrip) ROUNDTRIP=1 ;;
    --full) FULL=1; SMOKE=1; ROUNDTRIP=1 ;;
    *) echo "ERROR: unknown argument: $arg" >&2; exit 2 ;;
  esac
done

[[ "$(uname -s)" == "Darwin" ]] || { echo 'ERROR: macOS only'; exit 2; }
[[ -d "$DEST" ]] || { echo "ERROR: plugin not installed: $DEST"; exit 3; }

echo "Installed plugin: $DEST"
codesign --verify --deep --strict --verbose=2 "$DEST"

AE_APP="$(find /Applications -maxdepth 2 -type d -name 'Adobe After Effects*.app' -print 2>/dev/null | sort -V | tail -n 1 || true)"
if [[ -z "$AE_APP" ]]; then
  echo 'NOTE: After Effects app was not auto-detected. Open AE manually.'
else
  echo "After Effects: $AE_APP"
  if [[ "$LAUNCH" == "1" ]]; then
    open "$AE_APP"
    echo 'AE launched. Wait until startup finishes, then run this script again without --launch to inspect loading logs.'
  fi
fi

FOUND=0
while IFS= read -r log; do
  [[ -n "$log" ]] || continue
  FOUND=1
  echo "--- $log"
  grep -i -C 3 'ElasticGrid\|com.elasticgrid.fx.warp' "$log" | tail -n 80 || true
done < <(find "$HOME/Library/Preferences/Adobe/After Effects" -type f \( -iname '*Plugin*Loading*.log' -o -iname '*plugin*.log' \) -print 2>/dev/null | sort -V | tail -n 6)

if [[ "$FOUND" == "0" ]]; then
  echo 'No AE plug-in loading log found yet. Launch After Effects once, then run this script again.'
fi

if [[ "$SMOKE" == "1" ]]; then
  echo
  echo "Running real After Effects render smoke..."
  "$ROOT/ae_smoke_test_macos.command"
fi

if [[ "$ROUNDTRIP" == "1" ]]; then
  echo
  echo "Running After Effects project save/reopen roundtrip..."
  "$ROOT/ae_project_roundtrip_macos.command"
fi

if [[ "$FULL" == "1" ]]; then
  echo
  echo "Automated After Effects runtime gate: PASS"
  echo "Manual viewer-only checks still required: guide drag, Undo/Redo, 1/2 + 1/4 preview, non-square PAR, precomp/upstream resize."
fi
