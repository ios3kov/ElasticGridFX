#import <Foundation/Foundation.h>
#import <Metal/Metal.h>

#include "bridge/elasticgrid_ffi.h"

// Keep test operations and checks active regardless of compiler build mode.
#ifdef NDEBUG
#undef NDEBUG
#endif
#include <cassert>
#include <cstddef>
#include <cstdint>
#include <iostream>
#include <thread>
#include <vector>

static EgRenderParams params() {
    EgRenderParams p{};
    p.columns = 4;
    p.rows = 4;
    p.tension_radius = 3.0f;
    p.falloff = 2;
    p.elasticity_strength = 1.0f;
    p.min_spacing = 0.005f;
    p.easing_distance = 0.25f;
    p.edge_mode = 1;
    p.quality = 2;
    p.threads = 1;
    return p;
}

int main() {
    @autoreleasepool {
        assert(eg_metal_create(nullptr, nullptr) == nullptr);
        eg_metal_destroy(nullptr);

        id<MTLDevice> device = MTLCreateSystemDefaultDevice();
        if (!device) {
            std::cerr << "Metal lifecycle: no Metal device\n";
            return 5;
        }
        id<MTLCommandQueue> queue = [device newCommandQueue];
        assert(queue != nil);

        constexpr int w = 96;
        constexpr int h = 64;
        constexpr std::ptrdiff_t row_bytes = w * 16;
        const NSUInteger bytes = static_cast<NSUInteger>(row_bytes) * h;
        auto p = params();

        // Repeated setup/setdown catches retained pipelines/buffers and stale state.
        for (int cycle = 0; cycle < 128; ++cycle) {
            void* state = eg_metal_create((__bridge void*)device, (__bridge void*)queue);
            assert(state != nullptr);
            id<MTLBuffer> src = [device newBufferWithLength:bytes options:MTLResourceStorageModeShared];
            id<MTLBuffer> dst = [device newBufferWithLength:bytes options:MTLResourceStorageModeShared];
            assert(src && dst);
            const int rc = eg_metal_render(
                state,
                (__bridge void*)src, row_bytes, w, h,
                (__bridge void*)dst, row_bytes, w, h,
                &p);
            assert(rc == 0);
            eg_metal_destroy(state);
        }

        // MFR-style concurrent renders share one device state and plan pool.
        void* shared = eg_metal_create((__bridge void*)device, (__bridge void*)queue);
        assert(shared != nullptr);
        std::vector<std::thread> workers;
        for (int t = 0; t < 8; ++t) {
            workers.emplace_back([&, t] {
                @autoreleasepool {
                    id<MTLBuffer> src = [device newBufferWithLength:bytes options:MTLResourceStorageModeShared];
                    id<MTLBuffer> dst = [device newBufferWithLength:bytes options:MTLResourceStorageModeShared];
                    assert(src && dst);
                    auto local = p;
                    local.wave_enabled = 1;
                    local.wave_amplitude = 0.01f * float((t % 3) + 1);
                    local.wave_speed = 0.25f;
                    for (int i = 0; i < 64; ++i) {
                        local.time_seconds = float(i) / 24.0f;
                        assert(eg_metal_render(
                            shared,
                            (__bridge void*)src, row_bytes, w, h,
                            (__bridge void*)dst, row_bytes, w, h,
                            &local) == 0);
                    }
                }
            });
        }
        for (auto& wkr : workers) wkr.join();

        // Long shared-state soak exercises plan-buffer reuse without recompiling pipelines.
        id<MTLBuffer> soak_src = [device newBufferWithLength:bytes options:MTLResourceStorageModeShared];
        id<MTLBuffer> soak_dst = [device newBufferWithLength:bytes options:MTLResourceStorageModeShared];
        assert(soak_src && soak_dst);
        for (int i = 0; i < 2048; ++i) {
            auto local = p;
            local.wave_enabled = 1;
            local.wave_amplitude = 0.02f;
            local.wave_frequency = 1.0f + float(i % 7) * 0.25f;
            local.wave_speed = 0.2f;
            local.time_seconds = float(i) / 60.0f;
            local.quality = (i & 1) ? 2 : 1;
            assert(eg_metal_render(
                shared,
                (__bridge void*)soak_src, row_bytes, w, h,
                (__bridge void*)soak_dst, row_bytes, w, h,
                &local) == 0);
        }

        const NSUInteger allocated_before_destroy = device.currentAllocatedSize;
        eg_metal_destroy(shared);
        const NSUInteger allocated_after_destroy = device.currentAllocatedSize;
        std::cout << "Metal allocated bytes before/after shared-state destroy: "
                  << allocated_before_destroy << '/' << allocated_after_destroy << "\n";

        std::cout << "Metal lifecycle: PASS setup_cycles=128 concurrent_renders=512 soak_renders=2048\n";
        return 0;
    }
}
