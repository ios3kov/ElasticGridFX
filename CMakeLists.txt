cmake_minimum_required(VERSION 3.20)
project(ElasticGridFXCore VERSION 0.9.4 LANGUAGES CXX)

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
    src/core/PlaneTransform.cpp
    src/core/PlaneWarp.cpp
    src/core/PlaneRenderer.cpp
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
  add_executable(elasticgrid_license_dialog_tests tests/test_license_dialog.cpp)
  target_include_directories(elasticgrid_license_dialog_tests PRIVATE src)
  elasticgrid_enable_test_assertions(elasticgrid_license_dialog_tests)
  add_test(NAME elasticgrid_license_dialog COMMAND elasticgrid_license_dialog_tests)
  if(WIN32)
    add_executable(elasticgrid_native_installer_tests tests/test_native_installer_windows.cpp
      tests/installer_fixture_payload.cpp installer/windows/NativeInstaller.cpp)
    target_include_directories(elasticgrid_native_installer_tests PRIVATE installer installer/windows)
    target_compile_definitions(elasticgrid_native_installer_tests PRIVATE _WIN32_WINNT=0x0A00)
    target_link_libraries(elasticgrid_native_installer_tests PRIVATE advapi32 bcrypt shell32 ole32 uuid)
    elasticgrid_enable_test_assertions(elasticgrid_native_installer_tests)
    add_test(NAME elasticgrid_native_installer COMMAND elasticgrid_native_installer_tests)
    set_tests_properties(elasticgrid_native_installer PROPERTIES TIMEOUT 60)
    add_executable(elasticgrid_license_window_tests tests/test_license_window_win.cpp src/bridge/license_window_win.cpp)
    target_link_libraries(elasticgrid_license_window_tests PRIVATE user32 shell32 ole32)
    elasticgrid_enable_test_assertions(elasticgrid_license_window_tests)
    add_test(NAME elasticgrid_license_window_guards COMMAND elasticgrid_license_window_tests)
    set_tests_properties(elasticgrid_license_window_guards PROPERTIES TIMEOUT 10)
  endif()
  if(APPLE)
    add_executable(elasticgrid_installer_exchange_tests
        tests/test_installer_exchange_macos.cpp installer/macos/AtomicExchange.cpp
        installer/macos/PreparedJournal.cpp installer/macos/ExchangeCoordinator.cpp)
    target_include_directories(elasticgrid_installer_exchange_tests PRIVATE installer/macos)
    target_compile_options(elasticgrid_installer_exchange_tests PRIVATE -Wall -Wextra -Werror)
    elasticgrid_enable_test_assertions(elasticgrid_installer_exchange_tests)
    add_test(NAME elasticgrid_installer_exchange COMMAND elasticgrid_installer_exchange_tests)
    add_executable(elasticgrid_installer_coordinator_tests
        tests/test_installer_coordinator_macos.cpp installer/macos/AtomicExchange.cpp
        installer/macos/PreparedJournal.cpp installer/macos/ExchangeCoordinator.cpp
        installer/macos/PayloadSnapshot.cpp)
    target_include_directories(elasticgrid_installer_coordinator_tests PRIVATE installer/macos)
    target_compile_options(elasticgrid_installer_coordinator_tests PRIVATE -Wall -Wextra -Werror)
    elasticgrid_enable_test_assertions(elasticgrid_installer_coordinator_tests)
    add_test(NAME elasticgrid_installer_coordinator COMMAND elasticgrid_installer_coordinator_tests)
    add_executable(elasticgrid_installer_snapshot_tests
        tests/test_installer_snapshot_macos.cpp installer/macos/PayloadSnapshot.cpp)
    target_include_directories(elasticgrid_installer_snapshot_tests PRIVATE installer/macos)
    target_compile_options(elasticgrid_installer_snapshot_tests PRIVATE -Wall -Wextra -Werror)
    elasticgrid_enable_test_assertions(elasticgrid_installer_snapshot_tests)
    add_test(NAME elasticgrid_installer_snapshot COMMAND elasticgrid_installer_snapshot_tests)
    add_executable(elasticgrid_installer_receipt_tests
        tests/test_installer_receipt_macos.cpp installer/macos/SnapshotReceipt.cpp
        installer/macos/PayloadSnapshot.cpp installer/macos/AtomicExchange.cpp
        installer/macos/PreparedJournal.cpp installer/macos/ExchangeCoordinator.cpp)
    target_include_directories(elasticgrid_installer_receipt_tests PRIVATE installer/macos)
    target_compile_options(elasticgrid_installer_receipt_tests PRIVATE -Wall -Wextra -Werror)
    elasticgrid_enable_test_assertions(elasticgrid_installer_receipt_tests)
    add_test(NAME elasticgrid_installer_receipt COMMAND elasticgrid_installer_receipt_tests)
    add_executable(elasticgrid_installer_fresh_tests
        tests/test_installer_fresh_macos.cpp installer/macos/FreshPublication.cpp
        installer/macos/SnapshotReceipt.cpp installer/macos/PayloadSnapshot.cpp)
    target_include_directories(elasticgrid_installer_fresh_tests PRIVATE installer/macos)
    target_compile_options(elasticgrid_installer_fresh_tests PRIVATE -Wall -Wextra -Werror)
    elasticgrid_enable_test_assertions(elasticgrid_installer_fresh_tests)
    add_test(NAME elasticgrid_installer_fresh COMMAND elasticgrid_installer_fresh_tests)
  endif()
  add_executable(elasticgrid_plane_axis_cache_tests tests/test_plane_axis_cache.cpp)
  target_link_libraries(elasticgrid_plane_axis_cache_tests PRIVATE elasticgrid_core)
  elasticgrid_enable_test_assertions(elasticgrid_plane_axis_cache_tests)
  add_test(NAME elasticgrid_plane_axis_cache COMMAND elasticgrid_plane_axis_cache_tests)
  add_executable(elasticgrid_control_density_tests tests/test_control_density.cpp
      src/bridge/control_grid_ffi.cpp src/bridge/elasticgrid_ffi.cpp)
  target_link_libraries(elasticgrid_control_density_tests PRIVATE elasticgrid_core)
  elasticgrid_enable_test_assertions(elasticgrid_control_density_tests)
  add_test(NAME elasticgrid_control_density COMMAND elasticgrid_control_density_tests)
  add_executable(elasticgrid_live_influence_tests tests/test_live_influence.cpp
      src/bridge/control_grid_ffi.cpp src/bridge/elasticgrid_ffi.cpp)
  target_link_libraries(elasticgrid_live_influence_tests PRIVATE elasticgrid_core)
  elasticgrid_enable_test_assertions(elasticgrid_live_influence_tests)
  add_test(NAME elasticgrid_live_influence COMMAND elasticgrid_live_influence_tests)
  add_executable(elasticgrid_plane_perimeter_tests tests/test_plane_perimeter.cpp src/bridge/plane_ffi.cpp)
  target_link_libraries(elasticgrid_plane_perimeter_tests PRIVATE elasticgrid_core)
  elasticgrid_enable_test_assertions(elasticgrid_plane_perimeter_tests)
  add_test(NAME elasticgrid_plane_perimeter COMMAND elasticgrid_plane_perimeter_tests)
  enable_testing()
  add_executable(elasticgrid_plane_sampling_tests tests/test_plane_sampling.cpp src/bridge/plane_ffi.cpp)
  target_link_libraries(elasticgrid_plane_sampling_tests PRIVATE elasticgrid_core)
  elasticgrid_enable_test_assertions(elasticgrid_plane_sampling_tests)
  add_test(NAME elasticgrid_plane_sampling COMMAND elasticgrid_plane_sampling_tests)
  add_executable(elasticgrid_plane_bridge_tests tests/test_plane_bridge.cpp src/bridge/plane_ffi.cpp)
  target_link_libraries(elasticgrid_plane_bridge_tests PRIVATE elasticgrid_core)
  elasticgrid_enable_test_assertions(elasticgrid_plane_bridge_tests)
  add_test(NAME elasticgrid_plane_bridge COMMAND elasticgrid_plane_bridge_tests)
  add_executable(elasticgrid_plane_expanded_tests tests/test_plane_expanded.cpp)
  target_link_libraries(elasticgrid_plane_expanded_tests PRIVATE elasticgrid_core)
  elasticgrid_enable_test_assertions(elasticgrid_plane_expanded_tests)
  add_test(NAME elasticgrid_plane_expanded COMMAND elasticgrid_plane_expanded_tests)
  add_executable(elasticgrid_plane_tests tests/test_plane.cpp)
  target_link_libraries(elasticgrid_plane_tests PRIVATE elasticgrid_core)
  elasticgrid_enable_test_assertions(elasticgrid_plane_tests)
  add_test(NAME elasticgrid_plane COMMAND elasticgrid_plane_tests)
  add_executable(elasticgrid_plane_warp_tests tests/test_plane_warp.cpp)
  target_link_libraries(elasticgrid_plane_warp_tests PRIVATE elasticgrid_core)
  elasticgrid_enable_test_assertions(elasticgrid_plane_warp_tests)
  add_test(NAME elasticgrid_plane_warp COMMAND elasticgrid_plane_warp_tests)
  add_executable(elasticgrid_plane_render_tests tests/test_plane_render.cpp)
  target_link_libraries(elasticgrid_plane_render_tests PRIVATE elasticgrid_core)
  elasticgrid_enable_test_assertions(elasticgrid_plane_render_tests)
  add_test(NAME elasticgrid_plane_render COMMAND elasticgrid_plane_render_tests)
  add_executable(elasticgrid_plane_depth_tests tests/test_plane_depth.cpp)
  target_link_libraries(elasticgrid_plane_depth_tests PRIVATE elasticgrid_core)
  elasticgrid_enable_test_assertions(elasticgrid_plane_depth_tests)
  add_test(NAME elasticgrid_plane_depth COMMAND elasticgrid_plane_depth_tests)
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

  add_executable(elasticgrid_sparse_tests
      tests/test_sparse.cpp src/bridge/elasticgrid_ffi.cpp)
  target_link_libraries(elasticgrid_sparse_tests PRIVATE elasticgrid_core)
  target_include_directories(elasticgrid_sparse_tests PRIVATE src)
  elasticgrid_enable_test_assertions(elasticgrid_sparse_tests)
  add_test(NAME elasticgrid_sparse COMMAND elasticgrid_sparse_tests)

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
  add_executable(elasticgrid_plane_bench bench/bench_plane.cpp src/bridge/plane_ffi.cpp)
  target_link_libraries(elasticgrid_plane_bench PRIVATE elasticgrid_core)
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
