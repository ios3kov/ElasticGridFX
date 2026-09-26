#!/bin/bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BUILD="$ROOT/.hotpath-audit"
CXX="${CXX:-clang++}"
rm -rf "$BUILD"; mkdir -p "$BUILD"

if grep -R -nE 'std::mutex|lock_guard|unique_lock|condition_variable' \
    "$ROOT/src/core" "$ROOT/src/bridge" "$ROOT/src/gpu/metal_backend.mm"; then
  echo "ERROR: blocking mutex primitive found in render hot-path sources."
  exit 2
fi

"$CXX" -std=c++20 -O3 -DNDEBUG -Wall -Wextra -Wpedantic -Werror \
  -I"$ROOT/src" \
  "$ROOT/src/core/GridModel.cpp" "$ROOT/src/core/GridCodec.cpp" \
  "$ROOT/src/core/WarpMath.cpp" "$ROOT/src/core/CpuRenderer.cpp" \
  "$ROOT/src/bridge/elasticgrid_ffi.cpp" "$ROOT/tests/test_allocations.cpp" \
  -o "$BUILD/test_allocations"
"$BUILD/test_allocations"

echo "hot-path audit: PASS (no blocking mutex primitives; steady-state CPU heap allocations = 0)"
