#!/bin/bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BUNDLE="$ROOT/dist/mac/ElasticGrid.plugin"
DEST="/Library/Application Support/Adobe/Common/Plug-ins/7.0/MediaCore"
if [[ ! -d "$BUNDLE" ]]; then
  echo "ElasticGrid.plugin not found. Run tools/build_macos_sdkless.command first."
  exit 2
fi
sudo mkdir -p "$DEST"
sudo rm -rf "$DEST/ElasticGrid.plugin"
sudo cp -R "$BUNDLE" "$DEST/ElasticGrid.plugin"
sudo xattr -cr "$DEST/ElasticGrid.plugin" || true
echo "Installed. Restart After Effects."
