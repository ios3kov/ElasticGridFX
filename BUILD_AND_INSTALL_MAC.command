#!/bin/bash
set -u
ROOT="$(cd "$(dirname "$0")" && pwd)"
cd "$ROOT"

clear
echo "ElasticGrid FX v0.9 — final Mac preflight build"
echo "Runs sanitizers, quality parity, real Metal 4K/8K benchmarks, builds, signs and installs."
echo

if "$ROOT/tools/build_macos_sdkless.command" --install; then
  echo
  echo "SUCCESS: ElasticGrid.plugin installed."
  echo "Restart After Effects and apply: Effect > ElasticGrid FX > ElasticGrid FX"
  echo "Final runtime gate: fully quit/reopen AE into a NEW EMPTY project, then run:"
  echo "  tools/ae_runtime_check_macos.command --full"
  status=0
else
  status=$?
  echo
  echo "BUILD FAILED (exit $status)."
  echo "Send me the Terminal output or dist/mac/preflight-report.txt."
fi

echo
read -r -p "Press Return to close..." _
exit "$status"
