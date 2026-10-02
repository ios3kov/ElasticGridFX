#!/bin/bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BUILD="$ROOT/.preflight-macos"
CXX="${CXX:-clang++}"
COMMON=(
  -std=c++20 -I"$ROOT/src" -Wall -Wextra -Wpedantic -Werror
  "$ROOT/src/core/GridModel.cpp"
  "$ROOT/src/core/GridCodec.cpp"
  "$ROOT/src/core/WarpMath.cpp"
  "$ROOT/src/core/CpuRenderer.cpp"
)
BRIDGE=( "$ROOT/src/bridge/elasticgrid_ffi.cpp" )
PLANE=( "$ROOT/src/core/PlaneTransform.cpp" "$ROOT/src/core/PlaneWarp.cpp" "$ROOT/src/core/PlaneRenderer.cpp" )
SAN=( -O1 -g -fno-omit-frame-pointer -fsanitize=address,undefined )
TSAN=( -O1 -g -fno-omit-frame-pointer -fsanitize=thread )

if [[ "$(uname -s)" != "Darwin" ]]; then
  echo "ERROR: macOS preflight must run on macOS."
  exit 2
fi
if ! xcode-select -p >/dev/null 2>&1; then
  echo "ERROR: Xcode Command Line Tools are required."
  exit 3
fi
if ! command -v cargo >/dev/null 2>&1; then
  echo "ERROR: Rust/cargo must be installed before preflight."
  exit 4
fi

rm -rf "$BUILD"
mkdir -p "$BUILD"

run_san() {
  ASAN_OPTIONS=detect_leaks=0:halt_on_error=1 UBSAN_OPTIONS=halt_on_error=1 "$@"
}

echo "[preflight 0/20] Machine/toolchain"
echo "date: $(date -u '+%Y-%m-%dT%H:%M:%SZ')"
echo "arch: $(uname -m)"
sw_vers || true
"$CXX" --version | head -n 2
rustc --version
cargo --version
system_profiler SPDisplaysDataType 2>/dev/null | grep -E 'Chipset Model:|Metal Support:' | sed 's/^ *//' || true

echo "[preflight 1/20] Static analysis — Clang analyzer/high warnings + cargo clippy..."
"$ROOT/tools/static_analysis.command"

echo "[preflight 2/20] Core tests — ASan + UBSan..."
"$CXX" "${SAN[@]}" "${COMMON[@]}" "$ROOT/tests/test_core.cpp" -o "$BUILD/test_core_san"
run_san "$BUILD/test_core_san"

echo "[preflight] Plane axis cache pixel parity/cancellation/MFR — ASan + UBSan..."
"$CXX" "${SAN[@]}" "${COMMON[@]}" "${PLANE[@]}" "$ROOT/tests/test_plane_axis_cache.cpp" -o "$BUILD/test_plane_axis_cache_san"
run_san "$BUILD/test_plane_axis_cache_san"

echo "[preflight] Plane independent-frame caches — ThreadSanitizer..."
"$CXX" "${TSAN[@]}" "${COMMON[@]}" "${PLANE[@]}" "$ROOT/tests/test_plane_axis_cache.cpp" -o "$BUILD/test_plane_axis_cache_tsan"
TSAN_OPTIONS=halt_on_error=1 "$BUILD/test_plane_axis_cache_tsan"

echo "[preflight 3/20] Bridge tests — ASan + UBSan..."
"$CXX" "${SAN[@]}" "${COMMON[@]}" "${BRIDGE[@]}" "$ROOT/tests/test_bridge.cpp" -o "$BUILD/test_bridge_san"
run_san "$BUILD/test_bridge_san"

echo "[preflight] Exact identity / no-seam regression — ASan + UBSan..."
"$CXX" "${SAN[@]}" "${COMMON[@]}" "${BRIDGE[@]}" "$ROOT/tests/test_identity.cpp" -o "$BUILD/test_identity_san"
run_san "$BUILD/test_identity_san"

echo "[preflight] Sparse/full-canvas equivalence — ASan + UBSan..."
"$CXX" "${SAN[@]}" "${COMMON[@]}" "${BRIDGE[@]}" "$ROOT/tests/test_sparse.cpp" -o "$BUILD/test_sparse_san"
run_san "$BUILD/test_sparse_san"

echo "[preflight 4/20] CPU/GPU sampling-plan parity — ASan + UBSan..."
"$CXX" "${SAN[@]}" "${COMMON[@]}" "${BRIDGE[@]}" "$ROOT/tests/test_gpu_plan.cpp" -o "$BUILD/test_gpu_plan_san"
run_san "$BUILD/test_gpu_plan_san"

echo "[preflight 5/20] Final-quality reference tests — ASan + UBSan..."
"$CXX" "${SAN[@]}" "${COMMON[@]}" "$ROOT/tests/test_quality.cpp" -o "$BUILD/test_quality_san"
run_san "$BUILD/test_quality_san"

echo "[preflight 6/20] MFR concurrency stress — ASan + UBSan..."
"$CXX" "${SAN[@]}" "${COMMON[@]}" "${BRIDGE[@]}" "$ROOT/tests/test_mfr.cpp" -o "$BUILD/test_mfr_san"
run_san "$BUILD/test_mfr_san"

echo "[preflight 7/20] Deterministic fuzz smoke — ASan + UBSan..."
"$CXX" "${SAN[@]}" "${COMMON[@]}" "${BRIDGE[@]}" "$ROOT/tests/test_fuzz.cpp" -o "$BUILD/test_fuzz_san"
EG_FUZZ_SCALE=0.1 run_san "$BUILD/test_fuzz_san"

echo "[preflight 8/20] CPU setup/render/teardown soak — ASan + UBSan..."
"$CXX" "${SAN[@]}" "${COMMON[@]}" "${BRIDGE[@]}" "$ROOT/tests/test_soak.cpp" -o "$BUILD/test_soak_san"
EG_SOAK_CYCLES=5000 run_san "$BUILD/test_soak_san"

echo "[preflight 9/20] MFR concurrency stress — ThreadSanitizer..."
"$CXX" "${TSAN[@]}" "${COMMON[@]}" "${BRIDGE[@]}" "$ROOT/tests/test_mfr.cpp" -o "$BUILD/test_mfr_tsan"
TSAN_OPTIONS=halt_on_error=1 "$BUILD/test_mfr_tsan"

echo "[preflight 10/20] Allocation / lock hot-path audit..."
"$ROOT/tools/audit_hotpath.command"

echo "[preflight 11/20] Strict Release compile — -Werror, no fast-math..."
"$CXX" -O3 -DNDEBUG -fno-math-errno -ffp-contract=fast -Werror \
  "${COMMON[@]}" "${BRIDGE[@]}" "$ROOT/bench/bench_bridge.cpp" \
  -o "$BUILD/bench_bridge"

echo "[preflight 12/20] CPU 4K/8K performance smoke — Preview + Final..."
for depth in 8 16 32; do
  "$BUILD/bench_bridge" 3840 2160 4 "$depth" bilinear deformed 0 poll
  "$BUILD/bench_bridge" 3840 2160 3 "$depth" bicubic deformed 0 poll
done
"$BUILD/bench_bridge" 7680 4320 2 32 bilinear deformed 0 poll
"$BUILD/bench_bridge" 7680 4320 2 32 bicubic deformed 0 poll
"$BUILD/bench_bridge" 3840 2160 5 32 bicubic identity 0 poll

echo "[preflight 13/20] Metal lifecycle / repeated setup-setdown / MFR pool..."
"$ROOT/tools/run_metal_lifecycle_macos.command"

echo "[preflight 14/20] Real Metal CPU/GPU image parity — production path..."
"$ROOT/tools/run_metal_parity_macos.command"

echo "[preflight 15/20] Real Metal determinism — sequential + MFR..."
"$ROOT/tools/run_metal_determinism_macos.command"

echo "[preflight 16/20] Real Metal 4K/8K benchmark — production AE path..."
"$ROOT/tools/run_metal_bench_macos.command"

echo "[preflight 17/20] Dependency/license/RustSec audit + target-Mac SBOM..."
"$ROOT/tools/dependency_audit_macos.command"

echo "[preflight 18/20] AE host Release compile + Rust state tests (locked)..."
cargo test --release --locked --manifest-path "$ROOT/host-rust/Cargo.toml"

echo "[preflight 19/20] Cargo offline reproducibility check..."
cargo fetch --locked --manifest-path "$ROOT/host-rust/Cargo.toml"
CARGO_NET_OFFLINE=true cargo test --release --locked --manifest-path "$ROOT/host-rust/Cargo.toml"

echo "[preflight 20/20] Clean isolated reproducible plugin build..."
"$ROOT/tools/repro_build_macos.command"

echo "macOS preflight: PASS"
