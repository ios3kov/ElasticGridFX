#!/bin/bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
# Keep generated output inside the ignored/export-excluded test workspace.
BUILD="$ROOT/.preflight-macos/metal-parity"
[[ ! -L "$ROOT/.preflight-macos" ]] || { echo "ERROR: refusing symlinked preflight workspace" >&2; exit 4; }
CXX="${CXX:-clang++}"

if [[ "$(uname -s)" != "Darwin" ]]; then
  echo "ERROR: Metal parity test must run on macOS."
  exit 2
fi
if ! xcode-select -p >/dev/null 2>&1; then
  echo "ERROR: Xcode Command Line Tools are required."
  exit 3
fi

rm -rf "$BUILD"
mkdir -p "$BUILD"
{
  echo '#pragma once'
  echo '#include <cstddef>'
  echo 'static const char kElasticGridMetalSource[] = R"EGMETAL('
  cat "$ROOT/src/gpu/warp.metal"
  echo ')EGMETAL";'
  echo 'static constexpr std::size_t kElasticGridMetalSourceLength = sizeof(kElasticGridMetalSource) - 1;'
} > "$BUILD/elasticgrid_metal_source.h"

"$CXX" -std=c++20 -O3 -DNDEBUG -fobjc-arc -fno-math-errno -ffp-contract=fast \
  -Wall -Wextra -Wpedantic -Werror \
  -I"$ROOT/src" -I"$BUILD" \
  "$ROOT/src/core/GridModel.cpp" \
  "$ROOT/src/core/GridCodec.cpp" \
  "$ROOT/src/core/WarpMath.cpp" \
  "$ROOT/src/core/CpuRenderer.cpp" \
  "$ROOT/src/bridge/elasticgrid_ffi.cpp" \
  "$ROOT/src/gpu/metal_backend.mm" \
  "$ROOT/tests/test_metal_parity.mm" \
  -framework Foundation -framework Metal \
  -o "$BUILD/test_metal_parity"

if [[ "${ELASTICGRID_METAL_COMPILE_ONLY:-0}" == "1" ]]; then
  echo "Metal parity: COMPILE PASS — runtime skipped because this macOS host exposes no Metal device"
  exit 0
fi

"$BUILD/test_metal_parity"
