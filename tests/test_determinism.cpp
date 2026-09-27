#include "bridge/elasticgrid_ffi.h"

#include <algorithm>
#include <atomic>
#include <cstdint>
#include <cstdlib>
#include <cstring>
#include <iostream>
#include <thread>
#include <vector>

namespace {
[[noreturn]] void fail(const char* msg) {
    std::cerr << "DETERMINISM FAIL: " << msg << '\n';
    std::abort();
}
void require(bool v, const char* msg) { if (!v) fail(msg); }

std::uint64_t fnv1a64(const void* data, std::size_t n) {
    const auto* p = static_cast<const std::uint8_t*>(data);
    std::uint64_t h = 1469598103934665603ull;
    for (std::size_t i = 0; i < n; ++i) { h ^= p[i]; h *= 1099511628211ull; }
    return h;
}

EgRenderParams params(const float* x, const std::uint8_t* xp,
                      const float* y, const std::uint8_t* yp) {
    EgRenderParams p{};
    p.columns = 4; p.rows = 3;
    p.column_lines = x; p.column_line_count = 6; p.column_pins = xp;
    p.row_lines = y; p.row_line_count = 5; p.row_pins = yp;
    p.tension_radius = 3.7f; p.falloff = 3; p.elasticity_strength = 1.13f;
    p.min_spacing = 0.002f; p.stretch_easing = 0.61f; p.easing_distance = 0.19f;
    p.wave_enabled = 1; p.wave_amplitude = 0.031f; p.wave_frequency = 2.35f;
    p.wave_phase = -0.17f; p.wave_speed = 0.42f; p.wave_axis = 1;
    p.edge_mode = 3; p.quality = 2; p.time_seconds = -2.375f;
    p.threads = 1;
    return p;
}

void fill(std::vector<std::uint8_t>& bytes, std::uint32_t seed) {
    std::uint32_t x = seed;
    for (auto& b : bytes) { x = x * 1664525u + 1013904223u; b = static_cast<std::uint8_t>(x >> 24); }
}

std::vector<std::uint8_t> render_once(const std::vector<std::uint8_t>& src,
                                      std::ptrdiff_t stride,
                                      int w, int h, int depth,
                                      EgRenderParams p) {
    std::vector<std::uint8_t> out(src.size(), 0xA5);
    const int rc = eg_render_frame(src.data(), stride, w, h,
                                   out.data(), stride, w, h, depth, &p);
    require(rc == 0, "render failed");
    return out;
}

void check_render_determinism() {
    constexpr int W = 96, H = 72;
    const float x[] = {0.0f, 0.09f, 0.31f, 0.53f, 0.79f, 1.0f};
    const float y[] = {0.0f, 0.17f, 0.48f, 0.76f, 1.0f};
    const std::uint8_t xp[] = {1,0,1,0,0,1};
    const std::uint8_t yp[] = {1,0,0,1,1};

    for (int depth : {8,16,32}) {
        const std::size_t bpc = depth == 8 ? 1u : depth == 16 ? 2u : 4u;
        const std::ptrdiff_t stride = static_cast<std::ptrdiff_t>(W * 4 * bpc + 8 * bpc);
        std::vector<std::uint8_t> src(static_cast<std::size_t>(stride) * H);
        fill(src, 0xC001D00Du + static_cast<std::uint32_t>(depth));

        for (int quality : {1,2}) for (int edge : {1,2,3}) {
            auto p = params(x,xp,y,yp);
            p.quality = quality; p.edge_mode = edge; p.threads = 1;
            const auto ref = render_once(src,stride,W,H,depth,p);
            const std::uint64_t ref_hash = fnv1a64(ref.data(),ref.size());

            // Same thread count, many repeats.
            for (int i=0;i<32;++i) {
                auto out = render_once(src,stride,W,H,depth,p);
                require(out == ref, "repeat render changed bytes");
                require(fnv1a64(out.data(),out.size()) == ref_hash, "repeat render changed hash");
            }

            // Scheduling must not change a pixel: 1/2/4 worker chunks are exact.
            for (unsigned threads : {1u,2u,4u}) {
                p.threads = threads;
                for (int i=0;i<8;++i) {
                    auto out = render_once(src,stride,W,H,depth,p);
                    require(out == ref, "worker-count changed output bytes");
                }
            }
        }
    }
}

void check_plan_determinism() {
    constexpr int W=113,H=79;
    const float x[] = {0.0f,0.08f,0.32f,0.55f,0.81f,1.0f};
    const float y[] = {0.0f,0.21f,0.46f,0.73f,1.0f};
    const std::uint8_t xp[] = {1,0,0,1,0,1};
    const std::uint8_t yp[] = {1,0,1,0,1};

    for (int q : {1,2}) {
        auto p=params(x,xp,y,yp); p.quality=q;
        if (q==1) {
            std::vector<EgGpuLinearSample> xr(W),yr(H), x(W),y(H);
            require(eg_prepare_gpu_plan(W,H,W,H,&p,xr.data(),yr.data(),nullptr,nullptr)==0,"reference linear plan");
            for(int i=0;i<128;++i) {
                require(eg_prepare_gpu_plan(W,H,W,H,&p,x.data(),y.data(),nullptr,nullptr)==0,"linear plan repeat");
                require(std::memcmp(x.data(),xr.data(),x.size()*sizeof(x[0]))==0,"linear x plan nondeterministic");
                require(std::memcmp(y.data(),yr.data(),y.size()*sizeof(y[0]))==0,"linear y plan nondeterministic");
            }
        } else {
            std::vector<EgGpuCubicSample> xr(W),yr(H), x(W),y(H);
            require(eg_prepare_gpu_plan(W,H,W,H,&p,nullptr,nullptr,xr.data(),yr.data())==0,"reference cubic plan");
            for(int i=0;i<128;++i) {
                require(eg_prepare_gpu_plan(W,H,W,H,&p,nullptr,nullptr,x.data(),y.data())==0,"cubic plan repeat");
                require(std::memcmp(x.data(),xr.data(),x.size()*sizeof(x[0]))==0,"cubic x plan nondeterministic");
                require(std::memcmp(y.data(),yr.data(),y.size()*sizeof(y[0]))==0,"cubic y plan nondeterministic");
            }
        }
    }
}

void check_mfr_determinism() {
    constexpr int W=128,H=80;
    const float x[] = {0.0f,0.09f,0.28f,0.57f,0.83f,1.0f};
    const float y[] = {0.0f,0.18f,0.51f,0.77f,1.0f};
    const std::uint8_t xp[] = {1,0,0,1,0,1};
    const std::uint8_t yp[] = {1,0,0,0,1};
    auto p=params(x,xp,y,yp); p.quality=2; p.edge_mode=3; p.threads=1;
    const std::ptrdiff_t stride=W*16;
    std::vector<std::uint8_t> src(static_cast<std::size_t>(stride)*H);
    fill(src,0xFACEB00Cu);
    const auto ref=render_once(src,stride,W,H,32,p);
    const auto ref_hash=fnv1a64(ref.data(),ref.size());

    constexpr int THREADS=12, ITERS=32;
    std::atomic<int> failures{0};
    std::vector<std::thread> pool;
    for(int t=0;t<THREADS;++t) {
        pool.emplace_back([&,t] {
            for(int i=0;i<ITERS;++i) {
                auto local=p;
                // same semantic time/params, but distinct stack objects.
                local.threads=1;
                auto out=render_once(src,stride,W,H,32,local);
                if (fnv1a64(out.data(),out.size()) != ref_hash || out != ref) ++failures;
            }
            (void)t;
        });
    }
    for(auto& th:pool) th.join();
    require(failures.load()==0,"MFR concurrent output nondeterministic");
}
}

int main() {
    check_plan_determinism();
    check_render_determinism();
    check_mfr_determinism();
    std::cout << "determinism PASS: plans=256 repeats, CPU 8/16/32 exact across 1/2/4 threads, MFR=384 renders\n";
    return 0;
}
