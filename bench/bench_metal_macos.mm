#import <Foundation/Foundation.h>
#import <Metal/Metal.h>

#include "bridge/elasticgrid_ffi.h"

#include <chrono>
#include <cmath>
#include <cstddef>
#include <cstdint>
#include <iostream>
#include <string>
#include <vector>

static EgRenderParams make_params(int quality,
                                  const float* x, int nx, const std::uint8_t* xp,
                                  const float* y, int ny, const std::uint8_t* yp) {
    EgRenderParams p{};
    p.columns = nx - 1;
    p.rows = ny - 1;
    p.column_lines = x;
    p.column_line_count = nx;
    p.column_pins = xp;
    p.row_lines = y;
    p.row_line_count = ny;
    p.row_pins = yp;
    p.tension_radius = 4.0f;
    p.falloff = 2;
    p.elasticity_strength = 1.0f;
    p.min_spacing = 0.002f;
    p.stretch_easing = 0.4f;
    p.easing_distance = 0.25f;
    p.wave_enabled = 1;
    p.wave_amplitude = 0.04f;
    p.wave_frequency = 2.0f;
    p.wave_phase = 0.0f;
    p.wave_speed = 0.2f;
    p.wave_axis = 1;
    p.edge_mode = 1;
    p.quality = quality;
    p.time_seconds = 0.0f;
    p.threads = 0;
    return p;
}

static int run_case(id<MTLDevice> device,
                    void* state,
                    int w,
                    int h,
                    int frames,
                    int quality) {
    const std::ptrdiff_t row_bytes = static_cast<std::ptrdiff_t>(w) * 16;
    const NSUInteger bytes = static_cast<NSUInteger>(row_bytes) * static_cast<NSUInteger>(h);
    id<MTLBuffer> src = [device newBufferWithLength:bytes options:MTLResourceStorageModeShared];
    id<MTLBuffer> dst = [device newBufferWithLength:bytes options:MTLResourceStorageModeShared];
    if (!src || !dst) {
        std::cerr << "Metal benchmark: failed to allocate " << w << 'x' << h
                  << " input/output buffers (" << (double(bytes) * 2.0 / (1024.0 * 1024.0)) << " MiB)\n";
        return 6;
    }

    // Deterministic non-zero float4 data. This initialization is outside the
    // timed region and does not affect the render measurement.
    float* pixels = static_cast<float*>([src contents]);
    const std::size_t values = static_cast<std::size_t>(w) * h * 4u;
    for (std::size_t i = 0; i < values; ++i) {
        pixels[i] = float((i * 17u) % 251u) / 250.0f;
    }

    constexpr int columns = 12;
    constexpr int rows = 8;
    std::vector<float> x(columns + 1), y(rows + 1);
    std::vector<std::uint8_t> xp(x.size(), 0), yp(y.size(), 0);
    for (int i = 0; i <= columns; ++i) x[i] = float(i) / float(columns);
    for (int i = 0; i <= rows; ++i) y[i] = float(i) / float(rows);
    xp.front() = xp.back() = 1;
    yp.front() = yp.back() = 1;
    EgElasticParams ep{4.0f, 2, 1.0f, 0.002f};
    if (eg_drag_axis(x.data(), xp.data(), int(x.size()), 6, 0.58f, &ep) != 0 ||
        eg_drag_axis(y.data(), yp.data(), int(y.size()), 4, 0.43f, &ep) != 0) {
        return 7;
    }

    EgRenderParams p = make_params(quality, x.data(), int(x.size()), xp.data(), y.data(), int(y.size()), yp.data());

    // IMPORTANT: use the exact production entry point used by the AE host.
    auto one_frame = [&](int frame) -> int {
        p.time_seconds = float(frame) / 30.0f;
        return eg_metal_render(
            state,
            (__bridge void*)src, row_bytes, w, h,
            (__bridge void*)dst, row_bytes, w, h,
            &p);
    };

    for (int i = 0; i < 2; ++i) {
        if (one_frame(i) != 0) return 8;
    }

    const auto t0 = std::chrono::steady_clock::now();
    for (int i = 0; i < frames; ++i) {
        if (one_frame(i) != 0) return 9;
    }
    const auto t1 = std::chrono::steady_clock::now();

    const double ms = std::chrono::duration<double, std::milli>(t1 - t0).count() / frames;
    const double mpix = double(w) * double(h) / 1.0e6;
    std::cout << w << 'x' << h << " Metal 32bpc production-path "
              << (quality == 2 ? "bicubic-final" : "bilinear-preview")
              << ": " << ms << " ms/frame, " << mpix / (ms / 1000.0) << " MPix/s\n";
    return 0;
}

int main(int argc, char** argv) {
    @autoreleasepool {
        int w = 3840;
        int h = 2160;
        int frames = 10;
        if (argc > 1) w = std::stoi(argv[1]);
        if (argc > 2) h = std::stoi(argv[2]);
        if (argc > 3) frames = std::stoi(argv[3]);
        if (w <= 0 || h <= 0 || frames <= 0) return 2;

        id<MTLDevice> device = MTLCreateSystemDefaultDevice();
        if (!device) {
            std::cerr << "Metal benchmark: no Metal device\n";
            return 3;
        }
        id<MTLCommandQueue> queue = [device newCommandQueue];
        if (!queue) return 4;
        void* state = eg_metal_create((__bridge void*)device, (__bridge void*)queue);
        if (!state) return 5;

        std::cout << "Metal device: " << [[device name] UTF8String] << '\n';
        int rc = run_case(device, state, w, h, frames, 1);
        if (rc == 0) rc = run_case(device, state, w, h, frames, 2);

        eg_metal_destroy(state);
        return rc;
    }
}
