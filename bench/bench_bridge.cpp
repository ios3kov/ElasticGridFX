#include "bridge/elasticgrid_ffi.h"
#include <chrono>
#include <cstdint>
#include <iostream>
#include <string>
#include <vector>

namespace {
std::int32_t never_abort(void*) noexcept { return 0; }
}

int main(int argc, char** argv) {
    int w = 3840, h = 2160, frames = 10, depth = 8, quality = 1;
    bool identity = false;
    bool poll_abort = false;
    bool visualization = false;
    unsigned threads = 0;
    if (argc > 1) w = std::stoi(argv[1]);
    if (argc > 2) h = std::stoi(argv[2]);
    if (argc > 3) frames = std::stoi(argv[3]);
    if (argc > 4) depth = std::stoi(argv[4]);
    if (argc > 5) quality = std::string(argv[5]) == "bicubic" ? 2 : 1;
    if (argc > 6) identity = std::string(argv[6]) == "identity";
    if (argc > 7) threads = static_cast<unsigned>(std::stoul(argv[7]));
    if (argc > 8) poll_abort = std::string(argv[8]) == "poll";
    if (argc > 9) visualization = std::string(argv[9]) == "viz";

    const std::size_t bpc = depth == 8 ? 1u : depth == 16 ? 2u : depth == 32 ? 4u : 0u;
    if (!bpc) return 2;
    std::vector<std::uint8_t> src(static_cast<std::size_t>(w) * h * 4u * bpc, 0x40);
    std::vector<std::uint8_t> dst(src.size());

    EgRenderParams p{};
    p.columns = 12; p.rows = 8; p.tension_radius = 4.0f; p.falloff = 2;
    p.elasticity_strength = 1.0f; p.min_spacing = 0.002f;
    p.stretch_easing = 0.4f; p.easing_distance = 0.25f;
    p.wave_enabled = 1; p.wave_amplitude = 0.04f; p.wave_frequency = 2.0f;
    p.wave_speed = 0.2f; p.wave_axis = 1; p.edge_mode = 1; p.quality = quality;
    p.threads = threads;
    if (visualization) {
        p.visualization_enabled = 1;
        p.column_stroke_argb[0] = 255;
        p.column_stroke_argb[1] = 0;
        p.column_stroke_argb[2] = 96;
        p.column_stroke_argb[3] = 255;
        p.row_stroke_argb[0] = 255;
        p.row_stroke_argb[1] = 0;
        p.row_stroke_argb[2] = 96;
        p.row_stroke_argb[3] = 255;
        p.visualization_stroke_width = 2.0f;
        p.visualization_opacity = 1.0f;
        p.visualization_canvas_width = w;
        p.visualization_canvas_height = h;
    }
    if (poll_abort) {
        p.abort_fn = &never_abort;
        p.abort_refcon = nullptr;
    }

    // Benchmark the real host path. In deformation mode build a mildly
    // deformed guide state through the same public drag ABI used by the AE UI.
    // In identity mode keep a uniform grid with wave/easing disabled so the
    // exact no-op fast path can be measured separately.
    std::vector<float> x(static_cast<std::size_t>(p.columns) + 2u);
    std::vector<float> y(static_cast<std::size_t>(p.rows) + 2u);
    std::vector<std::uint8_t> xp(x.size(), 0), yp(y.size(), 0);
    for (std::size_t i = 0; i < x.size(); ++i) x[i] = static_cast<float>(i) / static_cast<float>(p.columns + 1);
    for (std::size_t i = 0; i < y.size(); ++i) y[i] = static_cast<float>(i) / static_cast<float>(p.rows + 1);
    xp.front() = xp.back() = 1;
    yp.front() = yp.back() = 1;
    if (!identity) {
        EgElasticParams ep{p.tension_radius, p.falloff, p.elasticity_strength, p.min_spacing};
        if (eg_drag_axis(x.data(), xp.data(), static_cast<std::int32_t>(x.size()), 6, 0.58f, &ep) != 0) return 5;
        if (eg_drag_axis(y.data(), yp.data(), static_cast<std::int32_t>(y.size()), 4, 0.43f, &ep) != 0) return 6;
    } else {
        p.wave_enabled = 0;
        p.wave_amplitude = 0.0f;
        p.stretch_easing = 0.0f;
    }
    p.column_lines = x.data();
    p.column_line_count = static_cast<std::int32_t>(x.size());
    p.column_pins = xp.data();
    p.row_lines = y.data();
    p.row_line_count = static_cast<std::int32_t>(y.size());
    p.row_pins = yp.data();

    const auto row = static_cast<std::ptrdiff_t>(w * 4u * bpc);
    for (int i = 0; i < 2; ++i) {
        p.time_seconds = static_cast<float>(i) / 30.0f;
        if (eg_render_frame(src.data(), row, w, h, dst.data(), row, w, h, depth, &p) != 0) return 3;
    }

    const auto t0 = std::chrono::steady_clock::now();
    for (int i = 0; i < frames; ++i) {
        p.time_seconds = static_cast<float>(i) / 30.0f;
        if (eg_render_frame(src.data(), row, w, h, dst.data(), row, w, h, depth, &p) != 0) return 4;
    }
    const auto t1 = std::chrono::steady_clock::now();
    const double ms = std::chrono::duration<double, std::milli>(t1 - t0).count() / frames;
    const double mpix = static_cast<double>(w) * h / 1e6;
    std::cout << w << 'x' << h << " bridge " << depth << "bpc "
              << (quality == 2 ? "bicubic" : "bilinear")
              << (identity ? " identity" : " deformed")
              << (visualization ? " visualization" : "")
              << (poll_abort ? " abort-poll" : "") << ": " << ms
              << " ms/frame, " << mpix / (ms / 1000.0) << " MPix/s, threads=" << threads << "\n";
}
