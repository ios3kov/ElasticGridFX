cmake_minimum_required(VERSION 3.20)
project(ElasticGridFXCore VERSION 0.9.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

option(ELASTICGRID_BUILD_TESTS "Build core tests" ON)
option(ELASTICGRID_BUILD_BENCH "Build microbenchmark" ON)

# CMake is only for standalone core verification/benchmarking.
# The After Effects macOS plugin itself is built by host-rust/Cargo.toml and
# tools/build_macos_sdkless.command, with no Adobe SDK and no CMake dependency.
add_library(elasticgrid_core
    src/core/GridModel.cpp
    src/core/GridCodec.cpp
    src/core/WarpMath.cpp
    src/core/CpuRenderer.cpp
)
target_include_directories(elasticgrid_core PUBLIC src)
target_compile_features(elasticgrid_core PUBLIC cxx_std_20)

if(MSVC)
  target_compile_options(elasticgrid_core PRIVATE /O2 /permissive- /W4)
else()
  target_compile_options(elasticgrid_core PRIVATE -O3 -Wall -Wextra -Wpedantic)
endif()


function(elasticgrid_enable_test_assertions target)
  # CMake defines NDEBUG in Release, which turns assert(expr) into a no-op and
  # would silently skip side-effecting test calls. Force assertions back on for
  # every verification target, including Release/CI builds.
  if(MSVC)
    target_compile_options(${target} PRIVATE /UNDEBUG)
  else()
    target_compile_options(${target} PRIVATE -UNDEBUG)
  endif()
endfunction()

if(ELASTICGRID_BUILD_TESTS)
  enable_testing()
  add_executable(elasticgrid_tests tests/test_core.cpp)
  target_link_libraries(elasticgrid_tests PRIVATE elasticgrid_core)
  elasticgrid_enable_test_assertions(elasticgrid_tests)
  add_test(NAME elasticgrid_core COMMAND elasticgrid_tests)

  add_executable(elasticgrid_bridge_tests
      tests/test_bridge.cpp
      src/bridge/elasticgrid_ffi.cpp
  )
  target_link_libraries(elasticgrid_bridge_tests PRIVATE elasticgrid_core)
  target_include_directories(elasticgrid_bridge_tests PRIVATE src)
  elasticgrid_enable_test_assertions(elasticgrid_bridge_tests)
  add_test(NAME elasticgrid_bridge COMMAND elasticgrid_bridge_tests)

  add_executable(elasticgrid_identity_tests
      tests/test_identity.cpp
      src/bridge/elasticgrid_ffi.cpp
  )
  target_link_libraries(elasticgrid_identity_tests PRIVATE elasticgrid_core)
  target_include_directories(elasticgrid_identity_tests PRIVATE src)
  elasticgrid_enable_test_assertions(elasticgrid_identity_tests)
  add_test(NAME elasticgrid_identity COMMAND elasticgrid_identity_tests)

  add_executable(elasticgrid_gpu_plan_tests
      tests/test_gpu_plan.cpp
      src/bridge/elasticgrid_ffi.cpp
  )
  target_link_libraries(elasticgrid_gpu_plan_tests PRIVATE elasticgrid_core)
  target_include_directories(elasticgrid_gpu_plan_tests PRIVATE src)
  elasticgrid_enable_test_assertions(elasticgrid_gpu_plan_tests)
  add_test(NAME elasticgrid_gpu_plan COMMAND elasticgrid_gpu_plan_tests)

  add_executable(elasticgrid_quality_tests tests/test_quality.cpp)
  target_link_libraries(elasticgrid_quality_tests PRIVATE elasticgrid_core)
  elasticgrid_enable_test_assertions(elasticgrid_quality_tests)
  add_test(NAME elasticgrid_quality COMMAND elasticgrid_quality_tests)

  add_executable(elasticgrid_mfr_tests
      tests/test_mfr.cpp
      src/bridge/elasticgrid_ffi.cpp
  )
  target_link_libraries(elasticgrid_mfr_tests PRIVATE elasticgrid_core)
  target_include_directories(elasticgrid_mfr_tests PRIVATE src)
  elasticgrid_enable_test_assertions(elasticgrid_mfr_tests)
  add_test(NAME elasticgrid_mfr COMMAND elasticgrid_mfr_tests)

  add_executable(elasticgrid_fuzz_smoke_tests
      tests/test_fuzz.cpp
      src/bridge/elasticgrid_ffi.cpp
  )
  target_link_libraries(elasticgrid_fuzz_smoke_tests PRIVATE elasticgrid_core)
  target_include_directories(elasticgrid_fuzz_smoke_tests PRIVATE src)
  elasticgrid_enable_test_assertions(elasticgrid_fuzz_smoke_tests)
  add_test(NAME elasticgrid_fuzz_smoke COMMAND elasticgrid_fuzz_smoke_tests)

  add_executable(elasticgrid_soak_tests
      tests/test_soak.cpp
      src/bridge/elasticgrid_ffi.cpp
  )
  target_link_libraries(elasticgrid_soak_tests PRIVATE elasticgrid_core)
  target_include_directories(elasticgrid_soak_tests PRIVATE src)
  elasticgrid_enable_test_assertions(elasticgrid_soak_tests)
  add_test(NAME elasticgrid_soak COMMAND elasticgrid_soak_tests)

  add_executable(elasticgrid_determinism_tests
      tests/test_determinism.cpp
      src/bridge/elasticgrid_ffi.cpp
  )
  target_link_libraries(elasticgrid_determinism_tests PRIVATE elasticgrid_core)
  target_include_directories(elasticgrid_determinism_tests PRIVATE src)
  elasticgrid_enable_test_assertions(elasticgrid_determinism_tests)
  add_test(NAME elasticgrid_determinism COMMAND elasticgrid_determinism_tests)

  add_executable(elasticgrid_allocation_audit_tests
      tests/test_allocations.cpp
      src/bridge/elasticgrid_ffi.cpp
  )
  target_link_libraries(elasticgrid_allocation_audit_tests PRIVATE elasticgrid_core)
  target_include_directories(elasticgrid_allocation_audit_tests PRIVATE src)
  elasticgrid_enable_test_assertions(elasticgrid_allocation_audit_tests)
  add_test(NAME elasticgrid_allocation_audit COMMAND elasticgrid_allocation_audit_tests)
endif()

if(ELASTICGRID_BUILD_BENCH)
  add_executable(elasticgrid_bench bench/bench_core.cpp)
  target_link_libraries(elasticgrid_bench PRIVATE elasticgrid_core)

  add_executable(elasticgrid_bridge_bench
      bench/bench_bridge.cpp
      src/bridge/elasticgrid_ffi.cpp
  )
  target_link_libraries(elasticgrid_bridge_bench PRIVATE elasticgrid_core)
  target_include_directories(elasticgrid_bridge_bench PRIVATE src)

  add_executable(elasticgrid_plan_bench
      bench/bench_plan.cpp
      src/bridge/elasticgrid_ffi.cpp
  )
  target_link_libraries(elasticgrid_plan_bench PRIVATE elasticgrid_core)
  target_include_directories(elasticgrid_plan_bench PRIVATE src)
endif()
