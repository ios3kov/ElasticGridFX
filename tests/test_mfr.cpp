#include "bridge/elasticgrid_ffi.h"

#include <algorithm>
#include <atomic>
#include <cmath>
#include <cstdint>
#include <cstdlib>
#include <iostream>
#include <thread>
#include <vector>

static void expect(bool ok, const char* msg) {
    if (!ok) { std::cerr << "FAIL: " << msg << '\n'; std::exit(1); }
}

int main() {
    constexpr int W = 128, H = 96;
    const float x[] = {0.0f, 0.11f, 0.37f, 0.68f, 1.0f};
    const float y[] = {0.0f, 0.24f, 0.61f, 1.0f};
    const std::uint8_t xp[] = {1,0,0,0,1};
    const std::uint8_t yp[] = {1,0,0,1};

    EgRenderParams p{};
    p.columns = 3; p.rows = 2;
    p.column_lines = x; p.column_line_count = 5; p.column_pins = xp;
    p.row_lines = y; p.row_line_count = 4; p.row_pins = yp;
    p.tension_radius = 3.0f; p.falloff = 2; p.elasticity_strength = 1.0f;
    p.min_spacing = 0.002f; p.stretch_easing = 0.55f; p.easing_distance = 0.2f;
    p.wave_enabled = 1; p.wave_amplitude = 0.02f; p.wave_frequency = 2.2f;
    p.wave_phase = 0.1f; p.wave_speed = 0.35f; p.wave_axis = 1;
    p.edge_mode = 3; p.quality = 2; p.time_seconds = 1.25f;
    // One worker inside each frame: the test stresses frame-level concurrency,
    // matching AE MFR without hiding races behind nested worker pools.
    p.threads = 1;

    std::vector<float> src(W * H * 4);
    for (int yy = 0; yy < H; ++yy) for (int xx = 0; xx < W; ++xx) {
        float* q = src.data() + (yy * W + xx) * 4;
        q[0] = std::sin(float(xx + yy) * 0.05f);
        q[1] = float((xx * 7 + yy * 11) % 97) / 96.0f;
        q[2] = float((xx * 17 + yy * 3) % 89) / 88.0f;
        q[3] = 0.2f + float((xx + yy) % 13) / 16.0f;
    }

    std::vector<float> reference(W * H * 4, 0.0f);
    expect(eg_render_frame(src.data(), W * 16, W, H,
                           reference.data(), W * 16, W, H, 32, &p) == 0,
           "reference render");

    constexpr int THREADS = 8;
    constexpr int ITERS = 24;
    std::atomic<int> failures{0};
    std::vector<std::thread> workers;
    for (int t = 0; t < THREADS; ++t) {
        workers.emplace_back([&, t] {
            (void)t;
            for (int i = 0; i < ITERS; ++i) {
                std::vector<float> out(W * H * 4, 0.0f);
                if (eg_render_frame(src.data(), W * 16, W, H,
                                    out.data(), W * 16, W, H, 32, &p) != 0) {
                    ++failures;
                    continue;
                }
                float max_err = 0.0f;
                for (std::size_t k = 0; k < out.size(); ++k)
                    max_err = std::max(max_err, std::abs(out[k] - reference[k]));
                if (max_err != 0.0f) ++failures;
            }
        });
    }
    for (auto& th : workers) th.join();
    expect(failures.load() == 0, "concurrent renders deterministic");

    std::cout << "elasticgrid_mfr_tests: OK\n";
    return 0;
}
