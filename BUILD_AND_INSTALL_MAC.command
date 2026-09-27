#!/bin/bash
set -u
ROOT="$(cd "$(dirname "$0")" && pwd)"
cd "$ROOT"

clear
echo "ElasticGrid FX v1.0 candidate — final Mac/After Effects gate"
echo "Runs sanitizers, quality parity, real Metal 4K/8K benchmarks, builds, signs and installs."
echo

if "$ROOT/tools/build_macos_sdkless.command" --install; then
  echo
  echo "SUCCESS: ElasticGrid.plugin installed."
  echo "Fully quit/reopen After Effects into a NEW EMPTY project."
  echo "Then run the automated runtime gate:"
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
