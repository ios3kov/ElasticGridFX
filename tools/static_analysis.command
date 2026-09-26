#!/bin/bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
CXX="${CXX:-clang++}"
FILES=(
  "$ROOT/src/core/GridModel.cpp"
  "$ROOT/src/core/GridCodec.cpp"
  "$ROOT/src/core/WarpMath.cpp"
  "$ROOT/src/core/CpuRenderer.cpp"
  "$ROOT/src/bridge/elasticgrid_ffi.cpp"
)
INCLUDES=( -I"$ROOT/src" )
WARN=(
  -std=c++20 -fsyntax-only -Weverything -Werror
  -Wno-c++98-compat -Wno-c++98-compat-pedantic -Wno-padded
  -Wno-unsafe-buffer-usage -Wno-switch-enum -Wno-covered-switch-default
  -Wno-global-constructors -Wno-exit-time-destructors
  -Wno-documentation-unknown-command -Wno-reserved-identifier
  -Wno-disabled-macro-expansion -Wno-weak-vtables
)

echo "[static] C++ strict syntax / high-warning audit"
for f in "${FILES[@]}"; do "$CXX" "${WARN[@]}" "${INCLUDES[@]}" "$f"; done

echo "[static] Clang Static Analyzer"
for f in "${FILES[@]}"; do "$CXX" --analyze -std=c++20 "${INCLUDES[@]}" "$f" -o /dev/null; done

if command -v clang-tidy >/dev/null 2>&1; then
  echo "[static] clang-tidy"
  for f in "${FILES[@]}"; do clang-tidy "$f" -- -std=c++20 "${INCLUDES[@]}"; done
else
  echo "[static] NOTE: clang-tidy not installed; Clang Static Analyzer + -Weverything already ran."
fi

if command -v cppcheck >/dev/null 2>&1; then
  echo "[static] cppcheck"
  cppcheck --enable=warning,performance,portability --error-exitcode=1 --std=c++20 -I "$ROOT/src" "${FILES[@]}"
else
  echo "[static] NOTE: cppcheck not installed."
fi

if command -v cargo >/dev/null 2>&1; then
  if command -v rustup >/dev/null 2>&1 && ! cargo clippy --version >/dev/null 2>&1; then
    rustup component add clippy
  fi
  echo "[static] cargo clippy -D warnings"
  cargo clippy --all-targets --manifest-path "$ROOT/host-rust/Cargo.toml" -- -D warnings
else
  echo "[static] NOTE: cargo unavailable; Clippy remains target-Mac pending."
fi

echo "static analysis: PASS for all available analyzers"
