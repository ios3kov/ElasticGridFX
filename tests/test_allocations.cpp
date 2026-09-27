#include "bridge/elasticgrid_ffi.h"

#include <atomic>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <iostream>
#include <new>
#include <vector>

namespace {
std::atomic<std::uint64_t> g_allocs{0};
std::atomic<std::uint64_t> g_bytes{0};
std::atomic<bool> g_count{false};

void* counted_alloc(std::size_t n) {
    if (g_count.load(std::memory_order_relaxed)) {
        g_allocs.fetch_add(1, std::memory_order_relaxed);
        g_bytes.fetch_add(n, std::memory_order_relaxed);
    }
    if (void* p = std::malloc(n)) return p;
    throw std::bad_alloc();
}

[[noreturn]] void fail(const char* what, std::uint64_t allocs, std::uint64_t bytes) {
    std::cerr << "ALLOCATION AUDIT FAIL: " << what << " allocations=" << allocs << " bytes=" << bytes << '\n';
    std::abort();
}

struct CountWindow {
    std::uint64_t a0=0,b0=0;
    CountWindow() {
        a0=g_allocs.load(std::memory_order_relaxed);
        b0=g_bytes.load(std::memory_order_relaxed);
        g_count.store(true,std::memory_order_release);
    }
    std::pair<std::uint64_t,std::uint64_t> stop() {
        g_count.store(false,std::memory_order_release);
        return {g_allocs.load(std::memory_order_relaxed)-a0,
                g_bytes.load(std::memory_order_relaxed)-b0};
    }
};

EgRenderParams make_params(const float* x,const std::uint8_t* xp,const float* y,const std::uint8_t* yp) {
    EgRenderParams p{};
    p.columns=3;p.rows=2;p.column_lines=x;p.column_line_count=5;p.column_pins=xp;
    p.row_lines=y;p.row_line_count=4;p.row_pins=yp;
    p.tension_radius=3;p.falloff=2;p.elasticity_strength=1;p.min_spacing=.002f;
    p.stretch_easing=.55f;p.easing_distance=.2f;
    p.wave_enabled=1;p.wave_amplitude=.02f;p.wave_frequency=2.2f;p.wave_phase=.1f;p.wave_speed=.35f;p.wave_axis=1;
    p.edge_mode=3;p.quality=2;p.time_seconds=1.25f;p.threads=1;
    return p;
}

void audit_render(int depth,int quality,EgRenderParams p) {
    constexpr int W=384,H=216;
    const std::size_t bpc=depth==8?1u:depth==16?2u:4u;
    const std::ptrdiff_t stride=static_cast<std::ptrdiff_t>(W*4*bpc);
    std::vector<std::uint8_t> src(static_cast<std::size_t>(stride)*H,0x3d);
    std::vector<std::uint8_t> dst(static_cast<std::size_t>(stride)*H,0);
    p.quality=quality;p.threads=1;
    // Warm TLS buffers/capacities for this quality/width before counting.
    for(int i=0;i<3;++i) {
        if(eg_render_frame(src.data(),stride,W,H,dst.data(),stride,W,H,depth,&p)!=0) std::abort();
    }
    CountWindow w;
    for(int i=0;i<200;++i) {
        if(eg_render_frame(src.data(),stride,W,H,dst.data(),stride,W,H,depth,&p)!=0) std::abort();
    }
    auto [a,b]=w.stop();
    if(a!=0) fail("steady-state eg_render_frame",a,b);
}
}

void* operator new(std::size_t n) { return counted_alloc(n); }
void* operator new[](std::size_t n) { return counted_alloc(n); }
void operator delete(void* p) noexcept { std::free(p); }
void operator delete[](void* p) noexcept { std::free(p); }
void operator delete(void* p,std::size_t) noexcept { std::free(p); }
void operator delete[](void* p,std::size_t) noexcept { std::free(p); }

int main() {
    const float x[]={0,.12f,.36f,.70f,1};
    const float y[]={0,.27f,.62f,1};
    const std::uint8_t xp[]={1,0,0,0,1},yp[]={1,0,0,1};
    auto p=make_params(x,xp,y,yp);

    for(int depth:{8,16,32}) for(int quality:{1,2}) audit_render(depth,quality,p);

    // GPU plan preparation should also be allocation-free after TLS warm-up.
    constexpr int W=384,H=216;
    std::vector<EgGpuCubicSample> xc(W),yc(H);
    p.quality=2;
    for(int i=0;i<3;++i) if(eg_prepare_gpu_plan(W,H,W,H,&p,nullptr,nullptr,xc.data(),yc.data())!=0) std::abort();
    CountWindow plan_window;
    for(int i=0;i<500;++i) if(eg_prepare_gpu_plan(W,H,W,H,&p,nullptr,nullptr,xc.data(),yc.data())!=0) std::abort();
    auto [pa,pb]=plan_window.stop();
    if(pa!=0) fail("steady-state eg_prepare_gpu_plan",pa,pb);

    EgRectI32 out{17,11,301,177},source{};
    p.canvas_width=W;p.canvas_height=H;
    for(int i=0;i<3;++i) if(eg_required_source_rect(W,H,out,&p,&source)!=0) std::abort();
    CountWindow roi_window;
    for(int i=0;i<500;++i) if(eg_required_source_rect(W,H,out,&p,&source)!=0) std::abort();
    auto [ra,rb]=roi_window.stop();
    if(ra!=0) fail("steady-state eg_required_source_rect",ra,rb);

    std::cout << "allocation audit PASS: steady-state CPU render/plan/ROI = 0 heap allocations after warm-up\n";
    return 0;
}
