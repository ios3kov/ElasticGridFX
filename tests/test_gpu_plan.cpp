#include "bridge/elasticgrid_ffi.h"

#include <algorithm>
#include <array>
#include <cassert>
#include <cmath>
#include <cstddef>
#include <cstdint>
#include <iostream>
#include <vector>

static_assert(sizeof(EgGpuLinearSample) == 16);
static_assert(sizeof(EgGpuCubicSample) == 32);

static EgRenderParams params_for(int quality, int edge,
                                 const float* x, int nx, const std::uint8_t* xp,
                                 const float* y, int ny, const std::uint8_t* yp) {
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
    p.elasticity_strength = 1.25f;
    p.min_spacing = 0.003f;
    p.stretch_easing = 0.72f;
    p.easing_distance = 0.18f;
    p.wave_enabled = 1;
    p.wave_amplitude = 0.035f;
    p.wave_frequency = 2.25f;
    p.wave_phase = 0.17f;
    p.wave_speed = -0.3f;
    p.wave_axis = 1;
    p.edge_mode = edge;
    p.quality = quality;
    p.time_seconds = 1.2345f;
    p.threads = 1;
    return p;
}

static void fill_source(std::vector<float>& src, int pitch_pixels, int w, int h) {
    for (int y = 0; y < h; ++y) {
        for (int x = 0; x < w; ++x) {
            float* q = src.data() + (static_cast<std::size_t>(y) * pitch_pixels + x) * 4;
            q[0] = 0.15f + 0.67f * std::sin(0.11f * static_cast<float>(x + 3 * y));
            q[1] = static_cast<float>((x * 17 + y * 13) % 101) / 100.0f;
            q[2] = static_cast<float>((x * 7 + y * 29) % 97) / 96.0f;
            q[3] = 0.2f + 0.8f * static_cast<float>((x + 2 * y) % 19) / 18.0f;
        }
    }
}

static std::array<float,4> load4(const std::vector<float>& src, int pitch_pixels, int x, int y) {
    const float* q = src.data() + (static_cast<std::size_t>(y) * pitch_pixels + x) * 4;
    return {q[0], q[1], q[2], q[3]};
}

static std::array<float,4> lerp4(const std::array<float,4>& a, const std::array<float,4>& b, float t) {
    return {
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
        a[3] + (b[3] - a[3]) * t,
    };
}

static void simulate_linear(const std::vector<float>& src, int src_pitch,
                            std::vector<float>& dst, int dst_pitch,
                            int w, int h,
                            const std::vector<EgGpuLinearSample>& xp,
                            const std::vector<EgGpuLinearSample>& yp) {
    for (int y = 0; y < h; ++y) {
        const auto ys = yp[static_cast<std::size_t>(y)];
        for (int x = 0; x < w; ++x) {
            const auto xs = xp[static_cast<std::size_t>(x)];
            const auto p00 = load4(src, src_pitch, xs.i0, ys.i0);
            const auto p10 = load4(src, src_pitch, xs.i1, ys.i0);
            const auto p01 = load4(src, src_pitch, xs.i0, ys.i1);
            const auto p11 = load4(src, src_pitch, xs.i1, ys.i1);
            const auto a = lerp4(p00, p10, xs.t);
            const auto b = lerp4(p01, p11, xs.t);
            const auto v = lerp4(a, b, ys.t);
            float* out = dst.data() + (static_cast<std::size_t>(y) * dst_pitch + x) * 4;
            for (int c = 0; c < 4; ++c) out[c] = v[c];
        }
    }
}

static void simulate_cubic(const std::vector<float>& src, int src_pitch,
                           std::vector<float>& dst, int dst_pitch,
                           int w, int h,
                           const std::vector<EgGpuCubicSample>& xp,
                           const std::vector<EgGpuCubicSample>& yp) {
    for (int y = 0; y < h; ++y) {
        const auto& ys = yp[static_cast<std::size_t>(y)];
        for (int x = 0; x < w; ++x) {
            const auto& xs = xp[static_cast<std::size_t>(x)];
            float acc[4] = {0,0,0,0};
            for (int ky = 0; ky < 4; ++ky) {
                const float wy = ys.weight[ky];
                for (int kx = 0; kx < 4; ++kx) {
                    const float wgt = wy * xs.weight[kx];
                    const float* q = src.data() +
                        (static_cast<std::size_t>(ys.index[ky]) * src_pitch + xs.index[kx]) * 4;
                    acc[0] += q[0] * wgt;
                    acc[1] += q[1] * wgt;
                    acc[2] += q[2] * wgt;
                    acc[3] += q[3] * wgt;
                }
            }
            float* out = dst.data() + (static_cast<std::size_t>(y) * dst_pitch + x) * 4;
            for (int c = 0; c < 4; ++c) out[c] = acc[c];
        }
    }
}

int main() {
    constexpr int sw = 73, sh = 41, dw = 67, dh = 37;
    constexpr int sp = sw + 5, dp = dw + 7;
    const float x[] = {0.0f, 0.12f, 0.49f, 0.78f, 1.0f};
    const float y[] = {0.0f, 0.21f, 0.69f, 1.0f};
    const std::uint8_t xpins[] = {1, 0, 1, 0, 1};
    const std::uint8_t ypins[] = {1, 0, 0, 1};

    std::vector<float> src(static_cast<std::size_t>(sp) * sh * 4, -99.0f);
    fill_source(src, sp, sw, sh);

    for (int edge = 1; edge <= 3; ++edge) {
        for (int quality = 1; quality <= 2; ++quality) {
            auto p = params_for(quality, edge, x, 5, xpins, y, 4, ypins);
            std::vector<float> cpu(static_cast<std::size_t>(dp) * dh * 4, -77.0f);
            std::vector<float> simulated(cpu.size(), -88.0f);

            const int rc = eg_render_frame(
                src.data(), static_cast<std::ptrdiff_t>(sp * 4 * sizeof(float)), sw, sh,
                cpu.data(), static_cast<std::ptrdiff_t>(dp * 4 * sizeof(float)), dw, dh,
                32, &p);
            assert(rc == 0);

            if (quality == 1) {
                std::vector<EgGpuLinearSample> xplan(dw), yplan(dh);
                assert(eg_prepare_gpu_plan(sw, sh, dw, dh, &p,
                                           xplan.data(), yplan.data(), nullptr, nullptr) == 0);
                simulate_linear(src, sp, simulated, dp, dw, dh, xplan, yplan);
            } else {
                std::vector<EgGpuCubicSample> xplan(dw), yplan(dh);
                assert(eg_prepare_gpu_plan(sw, sh, dw, dh, &p,
                                           nullptr, nullptr, xplan.data(), yplan.data()) == 0);
                simulate_cubic(src, sp, simulated, dp, dw, dh, xplan, yplan);
            }

            float max_error = 0.0f;
            for (int yy = 0; yy < dh; ++yy) {
                for (int xx = 0; xx < dw; ++xx) {
                    for (int c = 0; c < 4; ++c) {
                        const std::size_t i = (static_cast<std::size_t>(yy) * dp + xx) * 4 + c;
                        max_error = std::max(max_error, std::abs(cpu[i] - simulated[i]));
                    }
                }
            }
            if (max_error > 2.0e-6f) {
                std::cerr << "plan parity failed quality=" << quality << " edge=" << edge
                          << " max_error=" << max_error << '\n';
                for (int yy = 0; yy < dh; ++yy) {
                    for (int xx = 0; xx < dw; ++xx) {
                        const std::size_t i = (static_cast<std::size_t>(yy) * dp + xx) * 4;
                        if (std::abs(cpu[i] - simulated[i]) > 1.0e-4f) {
                            std::cerr << "first pixel " << xx << "," << yy << " cpu=" << cpu[i] << " sim=" << simulated[i] << '\n';
                            return 2;
                        }
                    }
                }
                return 2;
            }
        }
    }

    // Validate output-buffer requirements for each plan type.
    auto p = params_for(1, 1, x, 5, xpins, y, 4, ypins);
    assert(eg_prepare_gpu_plan(sw, sh, dw, dh, &p, nullptr, nullptr, nullptr, nullptr) == 1);
    p.quality = 2;
    assert(eg_prepare_gpu_plan(sw, sh, dw, dh, &p, nullptr, nullptr, nullptr, nullptr) == 1);

    return 0;
}
