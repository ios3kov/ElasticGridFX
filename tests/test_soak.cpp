#include "bridge/elasticgrid_ffi.h"
#include "core/GridCodec.h"
#include "core/GridModel.h"

#include <algorithm>
#include <cstdint>
#include <cstdlib>
#include <cstring>
#include <iostream>
#include <numeric>
#include <vector>

#ifdef __linux__
#include <fstream>
#include <unistd.h>
#endif

using namespace elasticgrid;

namespace {

[[noreturn]] void die(const char* msg) {
    std::cerr << "SOAK FAIL: " << msg << '\n';
    std::abort();
}
void require(bool v, const char* msg) { if (!v) die(msg); }

std::size_t rss_bytes() {
#ifdef __linux__
    std::ifstream f("/proc/self/statm");
    std::size_t total = 0, resident = 0;
    if (f >> total >> resident) {
        const long page = sysconf(_SC_PAGESIZE);
        if (page > 0) return resident * static_cast<std::size_t>(page);
    }
#endif
    return 0;
}

EgRenderParams make_params(std::vector<float>& cx,
                           std::vector<std::uint8_t>& px,
                           std::vector<float>& cy,
                           std::vector<std::uint8_t>& py) {
    constexpr int cols = 8, rows = 6;
    cx.resize(cols + 2); cy.resize(rows + 2);
    px.assign(cols + 2, 0); py.assign(rows + 2, 0);
    for (int i=0;i<=cols+1;++i) cx[static_cast<std::size_t>(i)] = float(i)/float(cols+1);
    for (int i=0;i<=rows+1;++i) cy[static_cast<std::size_t>(i)] = float(i)/float(rows+1);
    px.front()=px.back()=1; py.front()=py.back()=1;

    EgRenderParams p{};
    p.columns=cols; p.rows=rows;
    p.column_lines=cx.data(); p.column_line_count=static_cast<int>(cx.size()); p.column_pins=px.data();
    p.row_lines=cy.data(); p.row_line_count=static_cast<int>(cy.size()); p.row_pins=py.data();
    p.tension_radius=3.5f; p.falloff=2; p.elasticity_strength=1.0f; p.min_spacing=0.002f;
    p.stretch_easing=0.55f; p.easing_distance=0.2f;
    p.wave_enabled=1; p.wave_amplitude=0.025f; p.wave_frequency=2.25f; p.wave_phase=0.1f; p.wave_speed=0.35f; p.wave_axis=1;
    p.edge_mode=3; p.quality=2; p.threads=1;
    return p;
}

int cycles_from_env() {
    const char* raw = std::getenv("EG_SOAK_CYCLES");
    if (!raw || !*raw) return 5000;
    char* end=nullptr;
    long v=std::strtol(raw,&end,10);
    if (end==raw) return 5000;
    return static_cast<int>(std::clamp<long>(v, 100, 1000000));
}

void fill_bytes(std::vector<std::uint8_t>& v, std::uint32_t seed) {
    std::uint32_t x=seed;
    for (auto& b:v) { x = x*1664525u + 1013904223u; b=static_cast<std::uint8_t>(x>>24); }
}

} // namespace

int main() {
    constexpr int W=64, H=48;
    const int cycles=cycles_from_env();
    std::vector<float> cx,cy;
    std::vector<std::uint8_t> px,py;
    auto p=make_params(cx,px,cy,py);

    // Warm the allocator, thread runtime, codecs and renderer before RSS samples.
    std::size_t rss_start=0, rss_mid=0, rss_end=0;
    std::uint64_t checksum=0;

    for (int cycle=0; cycle<cycles; ++cycle) {
        p.time_seconds=float(cycle%240)/24.0f;
        p.quality=(cycle&1) ? 2 : 1;
        p.edge_mode=1+(cycle%3);
        p.threads=(cycle%97==0) ? 4u : 1u;

        // State setup/serialize/deserialize/teardown.
        GridState state{AxisGrid(8),AxisGrid(6)};
        ElasticSettings es; es.radius_lines=2.0f+float(cycle%5); es.strength=1.0f; es.min_spacing=0.002f;
        (void)state.columns.dragElastic(2+(cycle%5), 0.2f+0.1f*float(cycle%5), es);
        (void)state.rows.dragElastic(1+(cycle%4), 0.15f+0.12f*float(cycle%4), es);
        auto blob=encodeGridState(state);
        GridState decoded;
        require(decodeGridState(blob,decoded),"codec soak roundtrip");
        checksum += blob[cycle % blob.size()];

        // Plan setup/teardown exercises vector allocations independent of pixel depth.
        if (p.quality==2) {
            std::vector<EgGpuCubicSample> xp(W),yp(H);
            require(eg_prepare_gpu_plan(W,H,W,H,&p,nullptr,nullptr,xp.data(),yp.data())==0,"cubic plan soak");
            checksum += static_cast<std::uint64_t>(xp[cycle%W].index[0]+4);
        } else {
            std::vector<EgGpuLinearSample> xp(W),yp(H);
            require(eg_prepare_gpu_plan(W,H,W,H,&p,xp.data(),yp.data(),nullptr,nullptr)==0,"linear plan soak");
            checksum += static_cast<std::uint64_t>(xp[cycle%W].i0+2);
        }

        const int depth_mode=cycle%3;
        const int bit_depth=depth_mode==0?8:depth_mode==1?16:32;
        const std::size_t bpc=bit_depth==8?1u:bit_depth==16?2u:4u;
        const std::ptrdiff_t stride=static_cast<std::ptrdiff_t>(W*4*bpc + (cycle%4)*bpc);
        std::vector<std::uint8_t> src(static_cast<std::size_t>(stride)*H);
        std::vector<std::uint8_t> dst(static_cast<std::size_t>(stride)*H,0);
        fill_bytes(src,0x12345678u+static_cast<std::uint32_t>(cycle));
        require(eg_render_frame(src.data(),stride,W,H,dst.data(),stride,W,H,bit_depth,&p)==0,"render soak");
        checksum += dst[static_cast<std::size_t>((cycle*131)%dst.size())];

        // Elastic ABI setup/edit/teardown.
        auto lines=cx; auto pins=px;
        EgElasticParams ep{3.0f,2,1.0f,0.002f};
        require(eg_drag_axis(lines.data(),pins.data(),static_cast<int>(lines.size()),1+(cycle%7),
                             0.1f+0.1f*float(cycle%8),&ep)==0,"drag soak");

        if (cycle==std::min(500,cycles/4)) rss_start=rss_bytes();
        if (cycle==cycles/2) rss_mid=rss_bytes();
    }
    rss_end=rss_bytes();

    std::cout << "soak PASS cycles=" << cycles << " checksum=" << checksum;
    if (rss_start && rss_mid && rss_end) {
        auto mib=[](std::size_t b){return double(b)/(1024.0*1024.0);};
        std::cout << " rssMiB[start/mid/end]=" << mib(rss_start) << '/' << mib(rss_mid) << '/' << mib(rss_end);
        // RSS is diagnostic only: allocators/thread runtimes legitimately retain arenas.
        // LeakSanitizer is the authoritative leak gate.
    }
    std::cout << '\n';
    return 0;
}
