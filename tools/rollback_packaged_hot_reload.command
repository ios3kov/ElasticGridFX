#!/bin/zsh
set -euo pipefail

DEST_DIR="$HOME/Library/Application Support/AE Hot Loader/implementations/elasticgrid"

if ! pgrep -x "After Effects" >/dev/null 2>&1; then
  echo "ERROR: After Effects must already be running."
  exit 3
fi

rm -f "$DEST_DIR/current.dylib" "$DEST_DIR/current.tmp.dylib"

echo "ElasticGrid external candidate removed."
echo "Keep AE open and click Window → AE Hot Loader → Reload Plugins."
echo "Expected: ElasticGrid reloads its bundled elasticgrid-default implementation."
