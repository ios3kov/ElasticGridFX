#!/bin/bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
# Read-only inspection is the default. No sudo, deletion, or implicit install.
exec python3 "$ROOT/tools/install_candidate.py" "$@"
