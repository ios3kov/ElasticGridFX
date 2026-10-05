#include "bridge/elasticgrid_ffi.h"
#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <future>
#include <iostream>
#include <limits>
#include <type_traits>
#include <vector>

#ifdef EGFX_BASELINE
#define eg_render_frame_sparse eg_render_frame
#endif

namespace {
EgRenderParams params(int quality, int edge, bool deformed) {
    EgRenderParams p{};
    p.columns = p.rows = 4;
    p.quality = quality; p.edge_mode = edge; p.threads = 1;
    p.min_spacing = 0.005f; p.stretch_easing = 0.7f; p.easing_distance = 0.4f;
    p.canvas_width = 71; p.canvas_height = 43;
    if (deformed) {
        p.wave_enabled = 1; p.wave_amplitude = 0.11f;
        p.wave_frequency = 1.3f; p.wave_phase = 35.0f; p.wave_axis = 1;
    }
    return p;
}

template<class T> T value(int x, int y, int c) {
    if constexpr (std::is_same_v<T, float>) {
        return c == 0 ? 0.8f : static_cast<float>((x * 41 + y * 17 + c * 11) % 97) / 13.0f - 2.0f;
    } else {
        const unsigned maximum = sizeof(T) == 1 ? 255 : 32768;
        return static_cast<T>(c == 0 ? maximum : (x * 41u + y * 17u + c * 11u) % maximum);
    }
}

template<class T>
void fixture(int iw, int ih, int ix, int iy, int quality, int edge, bool deformed,
             bool negative_stride, bool expanded_output, unsigned threads = 1) {
    constexpr int w = 71, h = 43;
    auto p = params(quality, edge, deformed);
    p.threads = threads;
    const int ox = expanded_output ? -5 : 7, oy = expanded_output ? -4 : 3;
    const int ow = expanded_output ? w + 10 : 49, oh = expanded_output ? h + 8 : 29;
    const int depth = sizeof(T) == 1 ? 8 : sizeof(T) == 2 ? 16 : 32;
    const auto is = static_cast<std::size_t>(iw * 4 + 12);
    const auto os = static_cast<std::size_t>(ow * 4 + 16);
    const T sentinel = T{7};
    std::vector<T> full(static_cast<std::size_t>(w * h * 4), T{}), crop(is * ih, sentinel);
    std::vector<T> dense(full.size()), output(os * oh, sentinel);
    for (int y = 0; y < ih; ++y) for (int x = 0; x < iw; ++x) for (int c = 0; c < 4; ++c) {
        const auto v = value<T>(x, y, c);
        crop[static_cast<std::size_t>(negative_stride ? ih-1-y : y) * is + x*4+c] = v;
        if (x+ix >= 0 && x+ix < w && y+iy >= 0 && y+iy < h)
            full[static_cast<std::size_t>(((y+iy)*w+x+ix)*4+c)] = v;
    }
    assert(eg_render_frame(full.data(), w*4*sizeof(T), w, h,
                          dense.data(), w*4*sizeof(T), w, h, depth, &p) == 0);
    p.input_origin_x = ix; p.input_origin_y = iy;
    p.output_origin_x = ox; p.output_origin_y = oy;
    auto input = crop.data() + (negative_stride ? (ih-1)*is : 0);
    auto out = output.data() + (negative_stride ? (oh-1)*os : 0);
    const auto stride_sign = negative_stride ? -1 : 1;
    assert(eg_render_frame_sparse(input, stride_sign*static_cast<std::ptrdiff_t>(is*sizeof(T)), iw, ih,
           out, stride_sign*static_cast<std::ptrdiff_t>(os*sizeof(T)), ow, oh, depth, &p) == 0);
    std::size_t changed = 0;
    double max_error = 0;
    for (int y = 0; y < oh; ++y) for (int x = 0; x < ow; ++x) for (int c = 0; c < 4; ++c) {
        const T expected = x+ox >= 0 && x+ox < w && y+oy >= 0 && y+oy < h
            ? dense[static_cast<std::size_t>(((y+oy)*w+x+ox)*4+c)] : T{};
        const T actual = output[static_cast<std::size_t>(negative_stride ? oh-1-y : y)*os+x*4+c];
        const double err = std::abs(static_cast<double>(expected) - static_cast<double>(actual));
        const double tolerance = std::is_same_v<T, float> && deformed ? 2.5e-5 : 0.0;
        changed += err > tolerance;
        max_error = std::max(err, max_error);
    }
    if (changed) {
        std::cerr << "sparse mismatch: depth=" << depth << " quality=" << quality
                  << " crop=" << iw << 'x' << ih << " origin=" << ix << ',' << iy
                  << " changed=" << changed << " max=" << max_error << '\n';
    }
    assert(changed == 0);
    for (int y = 0; y < oh; ++y) for (auto k = static_cast<std::size_t>(ow*4); k < os; ++k)
        assert(output[static_cast<std::size_t>(y)*os+k] == sentinel);
}

int abort_now(void*) { return 1; }
void validation_and_empty() {
    auto p = params(2, 1, true);
    std::vector<float> out(12*7, 8.0f), input(4, 0.75f);
    assert(eg_render_frame_sparse(nullptr, 0, 0, 0, out.data(), 12*sizeof(float), 2, 7, 32, &p) == 0);
    for (int y = 0; y < 7; ++y) {
        for (int c = 0; c < 8; ++c) assert(out[y*12+c] == 0.0f);
        for (int c = 8; c < 12; ++c) assert(out[y*12+c] == 8.0f);
    }
    p.abort_fn = abort_now;
    std::fill(out.begin(), out.end(), 8.0f);
    assert(eg_render_frame_sparse(nullptr, 0, 0, 0, out.data(), 48, 2, 7, 32, &p) == 5);
    assert(std::all_of(out.begin(), out.end(), [](float v) { return v == 8.0f; }));
    assert(eg_render_frame_sparse(input.data(), 16, 1, 1, out.data(), 48, 2, 7, 32, &p) == 5);
    p.abort_fn = nullptr;
    assert(eg_render_frame_sparse(nullptr, 16, 1, 1, out.data(), 48, 2, 7, 32, &p) == 1);
    assert(eg_render_frame_sparse(nullptr, 0, -1, 0, out.data(), 48, 2, 7, 32, &p) == 1);
    assert(eg_render_frame_sparse(nullptr, 0, 0, 0, out.data(), 48, 2, 7, 24, &p) == 2);
    const auto huge = std::numeric_limits<std::ptrdiff_t>::max() / 4 * 4;
    assert(eg_render_frame_sparse(input.data(), huge, 1, 3, out.data(), 48, 2, 7, 32, &p) == 1);
    p.canvas_width = 0;
    assert(eg_render_frame_sparse(input.data(), 16, 1, 1, out.data(), 48, 2, 7, 32, &p) == 1);
}
} // namespace

int main() {
    // First fixture intentionally fails against the legacy local-edge behavior.
    fixture<float>(20, 11, 19, 13, 2, 1, true, false, false);
    for (int q : {1, 2}) for (int edge : {1, 2, 3, 4}) for (bool deform : {false, true}) {
        for (bool neg : {false, true}) for (bool expanded : {false, true}) {
            fixture<std::uint8_t>(20, 11, 19, 13, q, edge, deform, neg, expanded);
            fixture<std::uint16_t>(20, 11, 19, 13, q, edge, deform, neg, expanded);
            fixture<float>(20, 11, 19, 13, q, edge, deform, neg, expanded);
        }
        for (auto origin : {-7, 0, 70, 99}) {
            fixture<float>(9, 5, origin, -2, q, edge, deform, false, true);
            fixture<float>(1, 1, origin, 10, q, edge, deform, false, true);
        }
    }
    validation_and_empty();
    // Same TLS repeatedly alternates dense reference and sparse rendering;
    // independent threads exercise cross-frame cache isolation.
    std::vector<std::future<void>> tasks;
    for (int worker = 0; worker < 4; ++worker) tasks.push_back(std::async(std::launch::async, [worker] {
        for (int i = 0; i < 15; ++i)
            fixture<float>(20, 11, 19+worker, 13, 2, 1, true, i%2 != 0, false, 2);
    }));
    for (auto& task : tasks) task.get();
    std::cout << "PASS sparse/full logical-canvas equivalence; depth/stride/ROI/edge/empty/cancel/concurrency\n";
}
