#!/bin/zsh
set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"
PYTHON="${PYTHON:-python3}"

exec "$PYTHON" "$ROOT/tools/performance_baseline_094.py" "$@"
