#!/bin/bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BUILD="$ROOT/.metal-lifecycle-macos"
CXX="${CXX:-clang++}"
[[ "$(uname -s)" == "Darwin" ]] || { echo "ERROR: Metal lifecycle test requires macOS."; exit 2; }
xcode-select -p >/dev/null 2>&1 || { echo "ERROR: Xcode Command Line Tools are required."; exit 3; }
rm -rf "$BUILD" && mkdir -p "$BUILD"
{
  echo '#pragma once'
  echo '#include <cstddef>'
  echo 'static const char kElasticGridMetalSource[] = R"EGMETAL('
  cat "$ROOT/src/gpu/warp.metal"
  echo ')EGMETAL";'
  echo 'static constexpr std::size_t kElasticGridMetalSourceLength = sizeof(kElasticGridMetalSource) - 1;'
} > "$BUILD/elasticgrid_metal_source.h"
"$CXX" -std=c++20 -O2 -fobjc-arc -fno-math-errno -ffp-contract=fast \
  -Wall -Wextra -Wpedantic -Werror \
  -I"$ROOT/src" -I"$BUILD" \
  "$ROOT/src/core/GridModel.cpp" "$ROOT/src/core/GridCodec.cpp" \
  "$ROOT/src/core/WarpMath.cpp" "$ROOT/src/core/CpuRenderer.cpp" \
  "$ROOT/src/bridge/elasticgrid_ffi.cpp" "$ROOT/src/gpu/metal_backend.mm" \
  "$ROOT/tests/test_metal_lifecycle.mm" \
  -framework Foundation -framework Metal -o "$BUILD/test_metal_lifecycle"
"$BUILD/test_metal_lifecycle"
