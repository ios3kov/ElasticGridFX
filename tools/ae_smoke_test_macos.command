#!/bin/bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
# Default prepares only. Explicit --execute-in-test-ae is required to contact AE.
exec python3 "$ROOT/tools/ae_smoke_runner.py" "$@"
