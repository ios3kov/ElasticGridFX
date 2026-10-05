#import <Foundation/Foundation.h>
#import <Metal/Metal.h>

#include "bridge/elasticgrid_ffi.h"

#include <algorithm>
// Keep allocation/host guards active even in the optimized test executable.
#ifdef NDEBUG
#undef NDEBUG
#endif
#include <cassert>
#include <cmath>
#include <cstddef>
#include <cstdint>
#include <iostream>
#include <vector>

static EgRenderParams make_params(int quality, int edge,
                                  const float* x, int nx, const std::uint8_t* xp,
                                  const float* y, int ny, const std::uint8_t* yp,
                                  bool wave = true) {
    EgRenderParams p{};
    p.columns = nx - 2;
    p.rows = ny - 2;
    p.column_lines = x;
    p.column_line_count = nx;
    p.column_pins = xp;
    p.row_lines = y;
    p.row_line_count = ny;
    p.row_pins = yp;
    p.tension_radius = 4.0f;
    p.falloff = 3;
    p.elasticity_strength = 1.15f;
    p.min_spacing = 0.003f;
    p.stretch_easing = 0.63f;
    p.easing_distance = 0.2f;
    p.wave_enabled = wave ? 1 : 0;
    p.wave_amplitude = wave ? 0.027f : 0.0f;
    p.wave_frequency = 2.4f;
    p.wave_phase = 0.11f;
    p.wave_speed = -0.37f;
    p.wave_axis = 1;
    p.edge_mode = edge;
    p.quality = quality;
    p.time_seconds = 0.731f;
    p.threads = 1;
    return p;
}

static void fill_source(std::vector<float>& data, int pitch_pixels, int w, int h) {
    for (int y = 0; y < h; ++y) {
        for (int x = 0; x < w; ++x) {
            float* q = data.data() + (static_cast<std::size_t>(y) * pitch_pixels + x) * 4;
            // Deliberately asymmetric lanes. This catches accidental channel
            // reordering while still exercising HDR-ish values above 1.0.
            q[0] = -0.15f + 1.35f * (0.5f + 0.5f * std::sin(float(x * 3 + y) * 0.07f));
            q[1] = float((x * 11 + y * 17) % 127) / 83.0f;
            q[2] = float((x * 31 + y * 5) % 113) / 197.0f;
            q[3] = 0.05f + 0.95f * float((x + y * 2) % 23) / 22.0f;
        }
    }
}

static bool compare_active_pixels(const float* cpu, const float* gpu,
                                  int pitch_pixels, int w, int h,
                                  float& max_abs, double& rms) {
    max_abs = 0.0f;
    double sum_sq = 0.0;
    std::size_t n = 0;
    for (int y = 0; y < h; ++y) {
        for (int x = 0; x < w; ++x) {
            for (int c = 0; c < 4; ++c) {
                const std::size_t i = (static_cast<std::size_t>(y) * pitch_pixels + x) * 4 + c;
                const float e = std::abs(cpu[i] - gpu[i]);
                max_abs = std::max(max_abs, e);
                sum_sq += double(e) * double(e);
                ++n;
            }
        }
    }
    rms = std::sqrt(sum_sq / double(std::max<std::size_t>(1, n)));
    return max_abs <= 2.5e-5f && rms <= 3.0e-6;
}

struct Case {
    int sw, sh, dw, dh;
    bool wave;
};

int main() {
    @autoreleasepool {
        id<MTLDevice> device = MTLCreateSystemDefaultDevice();
        if (!device) {
            std::cerr << "Metal parity: no Metal device\n";
            return 5;
        }
        id<MTLCommandQueue> queue = [device newCommandQueue];
        assert(queue != nil);

        void* state = eg_metal_create((__bridge void*)device, (__bridge void*)queue);
        if (!state) {
            std::cerr << "Metal parity: shader/pipeline creation failed\n";
            return 2;
        }

        const float x[] = {0.0f, 0.08f, 0.36f, 0.73f, 1.0f};
        const float y[] = {0.0f, 0.29f, 0.57f, 1.0f};
        const std::uint8_t xp[] = {1, 0, 0, 1, 1};
        const std::uint8_t yp[] = {1, 0, 0, 1};
        const Case cases[] = {
            {257, 149, 239, 137, true},
            {131, 97, 131, 97, false},
            {73, 41, 91, 63, true},
        };

        for (const auto& tc : cases) {
            const int sp = tc.sw + 7;
            const int dp = tc.dw + 11;
            const std::ptrdiff_t src_row_bytes = static_cast<std::ptrdiff_t>(sp) * 16;
            const std::ptrdiff_t dst_row_bytes = static_cast<std::ptrdiff_t>(dp) * 16;
            std::vector<float> src(static_cast<std::size_t>(sp) * tc.sh * 4, 0.0f);
            fill_source(src, sp, tc.sw, tc.sh);

            for (int edge = 1; edge <= 4; ++edge) {
                for (int quality = 1; quality <= 2; ++quality) {
                    auto p = make_params(quality, edge, x, 5, xp, y, 4, yp, tc.wave);
                    std::vector<float> cpu(static_cast<std::size_t>(dp) * tc.dh * 4, 0.0f);
                    // Release builds define NDEBUG: never put the reference
                    // render inside assert, which would skip it completely.
                    const int cpu_rc = eg_render_frame(src.data(), src_row_bytes, tc.sw, tc.sh,
                                           cpu.data(), dst_row_bytes, tc.dw, tc.dh, 32, &p);
                    if (cpu_rc != 0) {
                        std::cerr << "Metal parity: CPU reference failed rc=" << cpu_rc << '\n';
                        eg_metal_destroy(state);
                        return 6;
                    }

                    const NSUInteger src_bytes = static_cast<NSUInteger>(src_row_bytes) * tc.sh;
                    const NSUInteger dst_bytes = static_cast<NSUInteger>(dst_row_bytes) * tc.dh;
                    id<MTLBuffer> src_buffer = [device newBufferWithBytes:src.data()
                                                                   length:src_bytes
                                                                  options:MTLResourceStorageModeShared];
                    id<MTLBuffer> dst_buffer = [device newBufferWithLength:dst_bytes
                                                                   options:MTLResourceStorageModeShared];
                    assert(src_buffer && dst_buffer);

                    const int rc = eg_metal_render(
                        state,
                        (__bridge void*)src_buffer, src_row_bytes, tc.sw, tc.sh,
                        (__bridge void*)dst_buffer, dst_row_bytes, tc.dw, tc.dh,
                        &p);
                    if (rc != 0) {
                        std::cerr << "Metal parity: optimized render failed rc=" << rc << '\n';
                        eg_metal_destroy(state);
                        return 3;
                    }

                    const float* gpu = static_cast<const float*>([dst_buffer contents]);
                    float max_abs = 0.0f;
                    double rms = 0.0;
                    const bool ok = compare_active_pixels(cpu.data(), gpu, dp, tc.dw, tc.dh, max_abs, rms);
                    std::cout << "Metal parity " << tc.sw << 'x' << tc.sh << " -> "
                              << tc.dw << 'x' << tc.dh << " quality=" << quality
                              << " edge=" << edge << " wave=" << tc.wave
                              << " max_abs=" << max_abs << " rms=" << rms << '\n';
                    if (!ok) {
                        std::cerr << "Metal parity tolerance exceeded\n";
                        eg_metal_destroy(state);
                        return 4;
                    }
                }
            }
        }

        eg_metal_destroy(state);
        return 0;
    }
}
