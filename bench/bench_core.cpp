#include "core/GridModel.h"
#include "core/WarpMath.h"
#include "core/CpuRenderer.h"
#include <chrono>
#include <iostream>
#include <thread>
#include <vector>

using namespace elasticgrid;

int main(int argc, char** argv) {
    int w = 3840, h = 2160, frames = 5;
    bool bicubic = false;
    if (argc > 1) w = std::stoi(argv[1]);
    if (argc > 2) h = std::stoi(argv[2]);
    if (argc > 3) frames = std::stoi(argv[3]);
    if (argc > 4) bicubic = std::string(argv[4]) == "bicubic";

    std::vector<float> src(static_cast<std::size_t>(w)*h*4, 0.5f);
    std::vector<float> dst(src.size());
    AxisGrid gx(12), gy(8);
    ElasticSettings es; es.radius_lines=4; es.min_spacing=.002f;
    gx.dragElastic(6,.62f,es); gy.dragElastic(4,.42f,es);
    auto xl=buildInverseLUT(gx.lines(),w,.4f,.25f);
    auto yl=buildInverseLUT(gy.lines(),h,.4f,.25f);

    RenderSettings rs; rs.quality = bicubic ? SampleQuality::Bicubic : SampleQuality::Bilinear;
    auto prepared=prepareWarpRGBAf(w,h,w,h,xl,yl,rs);

    // Warm-up outside the measured region.
    renderWarpRGBAfPrepared({src.data(),w,h,w*4},{dst.data(),w,h,w*4},prepared,rs);

    auto t0=std::chrono::steady_clock::now();
    for(int i=0;i<frames;++i)
        renderWarpRGBAfPrepared({src.data(),w,h,w*4},{dst.data(),w,h,w*4},prepared,rs);
    auto t1=std::chrono::steady_clock::now();
    const double ms=std::chrono::duration<double,std::milli>(t1-t0).count()/frames;
    const double mpix=(static_cast<double>(w)*h)/1e6;
    std::cout << w << 'x' << h << " prepared " << (bicubic ? "bicubic" : "bilinear") << " CPU: " << ms << " ms/frame, "
              << mpix/(ms/1000.0) << " MPix/s, hw_threads=" << std::thread::hardware_concurrency() << '\n';
}
