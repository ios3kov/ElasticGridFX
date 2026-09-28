#include "bridge/elasticgrid_ffi.h"
#include "core/GridCodec.h"
#include "core/GridModel.h"

#include <algorithm>
#include <bit>
#include <cassert>
#include <cmath>
#include <cstdint>
#include <cstring>
#include <cstdlib>
#include <iostream>
#include <limits>
#include <random>
#include <vector>

using namespace elasticgrid;

namespace {

[[noreturn]] void fail(const char* msg) {
    std::cerr << "FUZZ FAIL: " << msg << '\n';
    std::abort();
}

void require(bool v, const char* msg) {
    if (!v) fail(msg);
}

bool axis_ok(const AxisGrid& axis) {
    const auto& l = axis.lines();
    const auto& p = axis.pins();
    if (l.size() < 2 || l.size() != p.size()) return false;
    if (p.front() != 1 || p.back() != 1) return false;
    if (l.front() != 0.0f || l.back() != 1.0f) return false;
    for (std::size_t i = 0; i < l.size(); ++i) {
        if (!std::isfinite(l[i]) || l[i] < 0.0f || l[i] > 1.0f) return false;
        if (p[i] > 1) return false;
        if (i && l[i] < l[i - 1]) return false;
    }
    return true;
}

float weird_float(std::mt19937_64& rng) {
    switch (rng() % 20) {
        case 0: return std::numeric_limits<float>::quiet_NaN();
        case 1: return std::numeric_limits<float>::infinity();
        case 2: return -std::numeric_limits<float>::infinity();
        case 3: return std::numeric_limits<float>::max();
        case 4: return -std::numeric_limits<float>::max();
        case 5: return std::numeric_limits<float>::denorm_min();
        default: {
            std::uniform_real_distribution<float> d(-10.0f, 10.0f);
            return d(rng);
        }
    }
}

EgRenderParams base_params(std::vector<float>& cx, std::vector<std::uint8_t>& px,
                           std::vector<float>& cy, std::vector<std::uint8_t>& py,
                           int cols, int rows) {
    cx.resize(static_cast<std::size_t>(cols + 2));
    cy.resize(static_cast<std::size_t>(rows + 2));
    px.assign(static_cast<std::size_t>(cols + 2), 0);
    py.assign(static_cast<std::size_t>(rows + 2), 0);
    for (int i = 0; i <= cols + 1; ++i) cx[static_cast<std::size_t>(i)] = static_cast<float>(i) / static_cast<float>(cols + 1);
    for (int i = 0; i <= rows + 1; ++i) cy[static_cast<std::size_t>(i)] = static_cast<float>(i) / static_cast<float>(rows + 1);
    px.front() = px.back() = 1;
    py.front() = py.back() = 1;

    EgRenderParams p{};
    p.columns = cols;
    p.rows = rows;
    p.column_lines = cx.data();
    p.column_line_count = static_cast<std::int32_t>(cx.size());
    p.column_pins = px.data();
    p.row_lines = cy.data();
    p.row_line_count = static_cast<std::int32_t>(cy.size());
    p.row_pins = py.data();
    p.tension_radius = 3.0f;
    p.falloff = 2;
    p.elasticity_strength = 1.0f;
    p.min_spacing = 0.005f;
    p.stretch_easing = 0.5f;
    p.easing_distance = 0.25f;
    p.wave_enabled = 0;
    p.wave_amplitude = 0.0f;
    p.wave_frequency = 1.0f;
    p.wave_phase = 0.0f;
    p.wave_speed = 0.0f;
    p.wave_axis = 1;
    p.edge_mode = 1;
    p.quality = 2;
    p.time_seconds = 0.0f;
    p.threads = 2;
    return p;
}

int scaled_count(int base) {
    const char* raw = std::getenv("EG_FUZZ_SCALE");
    if (!raw || !*raw) return base;
    char* end = nullptr;
    const double v = std::strtod(raw, &end);
    if (end == raw || !std::isfinite(v)) return base;
    const double scale = std::clamp(v, 0.01, 100.0);
    return std::max(1, static_cast<int>(static_cast<double>(base) * scale));
}

void fuzz_codec(std::mt19937_64& rng, int random_iters, int mutation_iters) {
    std::uniform_int_distribution<int> len_dist(0, 2048);
    for (int it = 0; it < random_iters; ++it) {
        const int n = len_dist(rng);
        std::vector<std::uint8_t> blob(static_cast<std::size_t>(n));
        for (auto& b : blob) b = static_cast<std::uint8_t>(rng());
        GridState sentinel{AxisGrid(3), AxisGrid(5)};
        const auto before = encodeGridState(sentinel);
        GridState out = sentinel;
        const bool ok = decodeGridState(blob, out);
        if (ok) {
            require(axis_ok(out.columns), "decoded columns invariant");
            require(axis_ok(out.rows), "decoded rows invariant");
            const auto re = encodeGridState(out);
            GridState round;
            require(decodeGridState(re, round), "decoded state must re-encode/decode");
        } else {
            require(encodeGridState(out) == before, "failed decode must leave destination untouched");
        }
    }

    // Mutate known-good payloads too, including NaN/Inf bit patterns and truncation.
    for (int it = 0; it < mutation_iters; ++it) {
        GridState s{AxisGrid(1 + rng() % 128), AxisGrid(1 + rng() % 128)};
        auto blob = encodeGridState(s);
        const int mutations = 1 + static_cast<int>(rng() % 8);
        for (int m = 0; m < mutations && !blob.empty(); ++m) {
            const std::size_t i = static_cast<std::size_t>(rng() % blob.size());
            blob[i] ^= static_cast<std::uint8_t>(1u << (rng() % 8));
        }
        if ((rng() & 7u) == 0 && blob.size() > 4) blob.resize(static_cast<std::size_t>(rng() % blob.size()));
        GridState out;
        if (decodeGridState(blob, out)) {
            require(axis_ok(out.columns) && axis_ok(out.rows), "mutated accepted blob invariant");
        }
    }
}

void fuzz_axis(std::mt19937_64& rng, int iters) {
    for (int it = 0; it < iters; ++it) {
        AxisGrid g(static_cast<std::size_t>(rng() % 1000));
        require(axis_ok(g), "reset invariant");

        ElasticSettings e;
        e.strength = weird_float(rng);
        e.radius_lines = weird_float(rng);
        e.min_spacing = weird_float(rng);
        e.falloff = static_cast<FalloffProfile>(rng() & 0xffu);

        const std::size_t idx = static_cast<std::size_t>(rng() % (g.lineCount() + 3));
        (void)g.setPinned(idx, (rng() & 1u) != 0);
        (void)g.setLinePosition(idx, weird_float(rng), weird_float(rng));
        (void)g.dragElastic(idx, weird_float(rng), e);
        require(axis_ok(g), "axis operation invariant");

        WaveSettings w;
        w.enabled = (rng() & 1u) != 0;
        w.amplitude = weird_float(rng);
        w.frequency = weird_float(rng);
        w.phase_degrees = weird_float(rng);
        w.speed_cycles_per_second = weird_float(rng);
        w.axis = static_cast<WaveAxis>(rng() & 0xffu);
        auto eval = g.evaluated(w, weird_float(rng), weird_float(rng), (rng() & 1u) != 0);
        require(eval.size() == g.lineCount(), "evaluated topology invariant");
        for (float v : eval) require(std::isfinite(v), "evaluated finite invariant");
        require(eval.front() == 0.0f && eval.back() == 1.0f, "evaluated boundaries invariant");
        for (std::size_t i = 1; i < eval.size(); ++i) require(eval[i] >= eval[i - 1], "evaluated monotonic invariant");
    }
}

void fuzz_bridge(std::mt19937_64& rng, int iters) {
    for (int it = 0; it < iters; ++it) {
        const int cols = 1 + static_cast<int>(rng() % 16);
        const int rows = 1 + static_cast<int>(rng() % 16);
        std::vector<float> cx, cy;
        std::vector<std::uint8_t> px, py;
        auto p = base_params(cx, px, cy, py, cols, rows);

        p.tension_radius = weird_float(rng);
        p.elasticity_strength = weird_float(rng);
        p.min_spacing = weird_float(rng);
        p.stretch_easing = weird_float(rng);
        p.easing_distance = weird_float(rng);
        p.wave_enabled = static_cast<int>(rng() & 1u);
        p.wave_amplitude = weird_float(rng);
        p.wave_frequency = weird_float(rng);
        p.wave_phase = weird_float(rng);
        p.wave_speed = weird_float(rng);
        p.time_seconds = weird_float(rng);
        p.falloff = static_cast<int>(rng());
        p.wave_axis = static_cast<int>(rng());
        p.edge_mode = static_cast<int>(rng());
        p.quality = static_cast<int>(rng());
        p.threads = static_cast<std::uint32_t>(rng());

        const int w = 1 + static_cast<int>(rng() % 48);
        const int h = 1 + static_cast<int>(rng() % 48);
        p.canvas_width = (rng() & 3u) ? w : 1 + static_cast<int>(rng() % 64);
        p.canvas_height = (rng() & 3u) ? h : 1 + static_cast<int>(rng() % 64);
        p.input_origin_x = static_cast<int>(rng() % 41) - 20;
        p.input_origin_y = static_cast<int>(rng() % 41) - 20;
        p.output_origin_x = static_cast<int>(rng() % 41) - 20;
        p.output_origin_y = static_cast<int>(rng() % 41) - 20;

        // Occasionally poison a guide. This must be rejected, not crash.
        if ((rng() % 20) == 0 && cx.size() > 2) cx[1] = std::numeric_limits<float>::quiet_NaN();
        if ((rng() % 20) == 0 && cy.size() > 2) cy[1] = std::numeric_limits<float>::infinity();
        if ((rng() % 30) == 0) p.column_line_count += 1;
        if ((rng() % 30) == 0) p.row_line_count -= 1;

        std::vector<EgGpuCubicSample> x_c(static_cast<std::size_t>(w));
        std::vector<EgGpuCubicSample> y_c(static_cast<std::size_t>(h));
        std::vector<EgGpuLinearSample> x_l(static_cast<std::size_t>(w));
        std::vector<EgGpuLinearSample> y_l(static_cast<std::size_t>(h));
        const int rc_plan = eg_prepare_gpu_plan(w, h, w, h, &p, x_l.data(), y_l.data(), x_c.data(), y_c.data());
        require(rc_plan >= 0 && rc_plan <= 4, "plan return code bounded");

        EgRectI32 out_rect{0, 0, w, h}, source{};
        const int rc_rect = eg_required_source_rect(std::max(1, p.canvas_width), std::max(1, p.canvas_height), out_rect, &p, &source);
        require(rc_rect >= 0 && rc_rect <= 4, "rect return code bounded");
        if (rc_rect == 0) {
            require(source.left >= 0 && source.top >= 0, "source rect nonnegative");
            require(source.right > source.left && source.bottom > source.top, "source rect nonempty");
            require(source.right <= std::max(1, p.canvas_width) && source.bottom <= std::max(1, p.canvas_height), "source rect bounds");
        }

        const int depth_choice = static_cast<int>(rng() % 3);
        const int bit_depth = depth_choice == 0 ? 8 : depth_choice == 1 ? 16 : 32;
        const std::size_t bpc = bit_depth == 8 ? 1u : bit_depth == 16 ? 2u : 4u;
        const std::size_t stride = static_cast<std::size_t>(w) * 4u * bpc + (rng() % 4) * bpc;
        std::vector<std::uint8_t> in(stride * static_cast<std::size_t>(h));
        std::vector<std::uint8_t> out(stride * static_cast<std::size_t>(h), 0xCD);
        for (auto& b : in) b = static_cast<std::uint8_t>(rng());
        const int rc_render = eg_render_frame(in.data(), static_cast<std::ptrdiff_t>(stride), w, h,
                                              out.data(), static_cast<std::ptrdiff_t>(stride), w, h,
                                              bit_depth, &p);
        require(rc_render >= 0 && rc_render <= 5, "render return code bounded");
    }

    // Explicit null/bad-boundary API cases.
    EgRenderParams p{};
    require(eg_prepare_gpu_plan(1, 1, 1, 1, nullptr, nullptr, nullptr, nullptr, nullptr) != 0, "null params rejected");
    require(eg_render_frame(nullptr, 0, 1, 1, nullptr, 0, 1, 1, 32, &p) != 0, "null buffers rejected");
    require(eg_required_source_rect(1, 1, {0,0,1,1}, nullptr, nullptr) != 0, "null rect args rejected");
}

} // namespace

int main() {
    constexpr std::uint64_t kSeed = 0xE1A57C0FFEE12345ULL;
    std::mt19937_64 rng(kSeed);
    const int codec_random = scaled_count(50000);
    const int codec_mut = scaled_count(10000);
    const int axis_iters = scaled_count(100000);
    const int bridge_iters = scaled_count(20000);
    fuzz_codec(rng, codec_random, codec_mut);
    fuzz_axis(rng, axis_iters);
    fuzz_bridge(rng, bridge_iters);
    std::cout << "fuzz-smoke PASS seed=0x" << std::hex << kSeed << std::dec
              << " codec=" << (codec_random + codec_mut)
              << " axis=" << axis_iters
              << " bridge=" << bridge_iters << "\n";
    return 0;
}
