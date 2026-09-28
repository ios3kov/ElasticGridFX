#include "bridge/elasticgrid_ffi.h"

#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <iostream>
#include <limits>
#include <type_traits>
#include <vector>

static EgRenderParams parameters(int quality, int edge, float easing) {
    EgRenderParams p{};
    p.columns = 7;
    p.rows = 11;
    p.min_spacing = 0.005f;
    p.stretch_easing = easing;
    p.easing_distance = 1.0f;
    p.quality = quality;
    p.edge_mode = edge;
    p.threads = 1;
    return p;
}

template <typename T>
static void identity_pixels(int depth) {
    // Non-power-of-two width exposes roundoff hidden by the old 64x32 fixture.
    constexpr int w = 1919, h = 33;
    const std::size_t pitch = static_cast<std::size_t>(w) * 4u + 12u;
    std::vector<T> source(pitch * h, T{7}), output(source.size(), T{7});
    for (int y = 0; y < h; ++y) {
        for (std::size_t x = 0; x < static_cast<std::size_t>(w) * 4u; ++x) {
            const auto i = static_cast<std::size_t>(y) * pitch + x;
            if constexpr (std::is_same_v<T, float>) {
                // Include negative / HDR samples and varying alpha; no 0..1 clamp.
                source[i] = static_cast<float>((i * 7919u) % 1021u) / 128.0f - 2.0f;
            } else {
                const std::size_t range = depth == 8 ? 256u : 32769u;
                source[i] = static_cast<T>((i * 7919u) % range);
            }
        }
    }
    for (int quality : {1, 2}) for (int edge : {1, 2, 3}) {
        for (float easing : {0.0f, 0.25f, 1.0f}) {
            auto p = parameters(quality, edge, easing);
            const auto stride = static_cast<std::ptrdiff_t>(pitch * sizeof(T));
            std::fill(output.begin(), output.end(), T{7});
            assert(eg_render_frame(source.data(), stride, w, h,
                                   output.data(), stride, w, h, depth, &p) == 0);
            assert(std::memcmp(source.data(), output.data(), source.size() * sizeof(T)) == 0);

            // The same contract applies to bottom-up worlds.
            assert(eg_render_frame(source.data() + (h - 1) * pitch, -stride, w, h,
                                   output.data() + (h - 1) * pitch, -stride, w, h, depth, &p) == 0);
            assert(std::memcmp(source.data(), output.data(), source.size() * sizeof(T)) == 0);
        }
    }
}

static void identity_crop() {
    constexpr int w = 1919, h = 33, ox = 317, oy = 7, cw = 117, ch = 13;
    std::vector<float> source(static_cast<std::size_t>(w) * h * 4u);
    std::vector<float> output(static_cast<std::size_t>(cw) * ch * 4u);
    for (std::size_t i = 0; i < source.size(); ++i) {
        source[i] = static_cast<float>((i * 7919u) % 1021u) / 128.0f - 2.0f;
    }
    for (int quality : {1, 2}) for (int edge : {1, 2, 3}) {
        auto p = parameters(quality, edge, 1.0f);
        p.canvas_width = w; p.canvas_height = h;
        p.output_origin_x = ox; p.output_origin_y = oy;
        assert(eg_render_frame(source.data(), w * 16, w, h,
                               output.data(), cw * 16, cw, ch, 32, &p) == 0);
        for (int y = 0; y < ch; ++y) for (int x = 0; x < cw; ++x) for (int c = 0; c < 4; ++c) {
            assert(output[static_cast<std::size_t>((y * cw + x) * 4 + c)] ==
                   source[static_cast<std::size_t>(((y + oy) * w + x + ox) * 4 + c)]);
        }
    }
}

static void exact_sampling_plan() {
    constexpr int w = 1919, h = 33, ox = 317, oy = 7, cw = 117, ch = 13;
    std::vector<EgGpuLinearSample> xl(cw), yl(ch);
    std::vector<EgGpuCubicSample> xc(cw), yc(ch);
    for (int guides : {1, 4, 7, 50}) for (int quality : {1, 2}) for (int edge : {1, 2, 3}) {
        auto p = parameters(quality, edge, 1.0f);
        p.columns = guides; p.rows = guides;
        p.canvas_width = w; p.canvas_height = h;
        p.output_origin_x = ox; p.output_origin_y = oy;
        assert(eg_prepare_gpu_plan(w, h, cw, ch, &p, xl.data(), yl.data(), xc.data(), yc.data()) == 0);
        for (int axis = 0; axis < 2; ++axis) {
            const int count = axis == 0 ? cw : ch;
            const int origin = axis == 0 ? ox : oy;
            for (int i = 0; i < count; ++i) {
                const auto n = static_cast<std::size_t>(i);
                if (quality == 1) {
                    const auto& s = axis == 0 ? xl[n] : yl[n];
                    assert(s.i0 == i + origin && s.i1 == i + origin && s.t == 0.0f);
                } else {
                    const auto& s = axis == 0 ? xc[n] : yc[n];
                    for (int k = 0; k < 4; ++k) {
                        assert(s.index[k] == i + origin);
                        assert(s.weight[k] == (k == 1 ? 1.0f : 0.0f));
                    }
                }
            }
        }
    }
    // One-axis deformation must not disturb the other axis or disappear.
    float x[] = {0.0f, 0.10f, 0.61f, 0.84f, 1.0f};
    std::uint8_t pins[] = {1, 0, 0, 0, 1};
    auto p = parameters(2, 1, 1.0f);
    p.columns = 3; p.column_lines = x; p.column_line_count = 5; p.column_pins = pins;
    p.canvas_width = w; p.canvas_height = h;
    p.output_origin_x = ox; p.output_origin_y = oy;
    assert(eg_prepare_gpu_plan(w, h, cw, ch, &p, nullptr, nullptr, xc.data(), yc.data()) == 0);
    bool changed = false;
    for (int i = 0; i < cw; ++i) if (xc[static_cast<std::size_t>(i)].index[1] != ox + i) changed = true;
    assert(changed);
    for (int i = 0; i < ch; ++i) assert(yc[static_cast<std::size_t>(i)].index[1] == oy + i &&
                                        yc[static_cast<std::size_t>(i)].weight[1] == 1.0f);
}

static void boundary_and_origin_mapping() {
    constexpr int width = 19, height = 9;
    std::vector<EgGpuLinearSample> x(width), y(height);
    for (int edge : {1, 2, 3}) {
        auto p = parameters(1, edge, 1.0f);
        p.canvas_width = 500; p.canvas_height = 40;
        p.input_origin_x = 23; p.input_origin_y = 5;
        // A cropped checkout whose local coordinate is negative.
        assert(eg_prepare_gpu_plan(width, height, width, height, &p,
                                   x.data(), y.data(), nullptr, nullptr) == 0);
        const int expected_x = edge == 1 ? 0 : (edge == 2 ? 15 : 13);
        assert(x[0].i0 == expected_x && x[0].i1 == expected_x && x[0].t == 0.0f);
        // Output-origin addition must not overflow a 32-bit signed integer.
        p.output_origin_x = std::numeric_limits<std::int32_t>::max() - 5;
        p.input_origin_x = 0;
        assert(eg_prepare_gpu_plan(width, height, width, height, &p,
                                   x.data(), y.data(), nullptr, nullptr) == 0);
        const int expected_border = edge == 1 ? 18 : (edge == 2 ? 5 : 5);
        for (const auto& sample : x) assert(sample.i0 == expected_border && sample.i1 == expected_border);
    }
}

int main() {
    identity_pixels<std::uint8_t>(8);
    identity_pixels<std::uint16_t>(16);
    identity_pixels<float>(32);
    identity_crop();
    exact_sampling_plan();
    boundary_and_origin_mapping();
    std::cout << "Identity: exact 8/16/32-bpc pixels, easing, padded/negative stride, crop and CPU/GPU plans PASS\n";
}
