#!/bin/bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")" && pwd)"
echo "ElasticGrid FX — build-only candidate validation"
echo "This legacy launcher no longer installs or replaces plugins automatically."
exec "$ROOT/tools/build_macos_sdkless.command" "$@"
