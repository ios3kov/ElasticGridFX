#!/bin/bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
# Old --full/--launch shortcuts could report PASS without verifying loaded code.
# The runner requires the exact test app/bundle and does not promote pixels to a full gate.
for arg in "$@"; do
  case "$arg" in
    --full|--launch|--smoke|--project-roundtrip)
      echo "ERROR: legacy gate shortcuts are disabled. Use --help for explicit test execution." >&2
      exit 2 ;;
  esac
done
exec python3 "$ROOT/tools/ae_smoke_runner.py" "$@"
