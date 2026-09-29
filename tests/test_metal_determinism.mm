#import <Foundation/Foundation.h>
#import <Metal/Metal.h>

#include "bridge/elasticgrid_ffi.h"

#include <atomic>
// Test assertions (including reference renders) must execute in Release too.
// This affects only this test translation unit, not production renderer flags.
#ifdef NDEBUG
#undef NDEBUG
#endif
#include <cassert>
#include <cmath>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <iostream>
#include <thread>
#include <vector>

static EgRenderParams make_params(int quality, int edge) {
    static const float x[] = {0.0f,0.09f,0.31f,0.58f,0.82f,1.0f};
    static const float y[] = {0.0f,0.19f,0.47f,0.76f,1.0f};
    static const std::uint8_t xp[] = {1,0,0,1,0,1};
    static const std::uint8_t yp[] = {1,0,1,0,1};
    EgRenderParams p{};
    p.columns=4; p.rows=3;
    p.column_lines=x; p.column_line_count=6; p.column_pins=xp;
    p.row_lines=y; p.row_line_count=5; p.row_pins=yp;
    p.tension_radius=3.7f; p.falloff=3; p.elasticity_strength=1.1f; p.min_spacing=0.002f;
    p.stretch_easing=0.61f; p.easing_distance=0.19f;
    p.wave_enabled=1; p.wave_amplitude=0.029f; p.wave_frequency=2.4f; p.wave_phase=-0.13f; p.wave_speed=0.37f; p.wave_axis=1;
    p.edge_mode=edge; p.quality=quality; p.time_seconds=1.375f; p.threads=1;
    return p;
}

static void fill(float* dst, int pitch, int w, int h) {
    for (int yy=0;yy<h;++yy) for(int xx=0;xx<w;++xx) {
        float* q=dst+(std::size_t(yy)*pitch+xx)*4;
        q[0]=-0.2f+1.4f*(0.5f+0.5f*std::sin(float(xx*5+yy)*0.041f));
        q[1]=float((xx*13+yy*17)%127)/91.0f;
        q[2]=float((xx*19+yy*7)%109)/131.0f;
        q[3]=0.1f+0.9f*float((xx+yy*3)%31)/30.0f;
    }
}

int main() {
    @autoreleasepool {
        id<MTLDevice> device=MTLCreateSystemDefaultDevice();
        if(!device) { std::cerr<<"Metal determinism: no device\n"; return 5; }
        id<MTLCommandQueue> queue=[device newCommandQueue];
        assert(queue);
        void* state=eg_metal_create((__bridge void*)device,(__bridge void*)queue);
        if(!state) { std::cerr<<"Metal determinism: state create failed\n"; return 2; }

        constexpr int W=191,H=127,P=W+9;
        constexpr std::ptrdiff_t row_bytes=P*16;
        const NSUInteger bytes=static_cast<NSUInteger>(row_bytes)*H;
        id<MTLBuffer> src=[device newBufferWithLength:bytes options:MTLResourceStorageModeShared];
        assert(src);
        fill(static_cast<float*>(src.contents),P,W,H);

        for(int quality: {1,2}) for(int edge:{1,2,3}) {
            auto p=make_params(quality,edge);
            id<MTLBuffer> ref=[device newBufferWithLength:bytes options:MTLResourceStorageModeShared];
            id<MTLBuffer> out=[device newBufferWithLength:bytes options:MTLResourceStorageModeShared];
            assert(ref&&out);
            std::memset(ref.contents,0xA5,bytes);
            assert(eg_metal_render(state,(__bridge void*)src,row_bytes,W,H,
                                   (__bridge void*)ref,row_bytes,W,H,&p)==0);
            for(int i=0;i<64;++i) {
                std::memset(out.contents,0xA5,bytes);
                if(eg_metal_render(state,(__bridge void*)src,row_bytes,W,H,
                                   (__bridge void*)out,row_bytes,W,H,&p)!=0 ||
                   std::memcmp(ref.contents,out.contents,bytes)!=0) {
                    std::cerr<<"Metal sequential determinism failed quality="<<quality<<" edge="<<edge<<" iter="<<i<<"\n";
                    eg_metal_destroy(state); return 3;
                }
            }
        }

        // MFR-style concurrent same-frame renders must match one exact GPU reference.
        auto p=make_params(2,3);
        id<MTLBuffer> reference=[device newBufferWithLength:bytes options:MTLResourceStorageModeShared];
        assert(reference);
        std::memset(reference.contents,0x5A,bytes);
        assert(eg_metal_render(state,(__bridge void*)src,row_bytes,W,H,
                               (__bridge void*)reference,row_bytes,W,H,&p)==0);
        std::vector<std::uint8_t> ref_bytes(bytes);
        std::memcpy(ref_bytes.data(),reference.contents,bytes);

        constexpr int THREADS=8, ITERS=32;
        std::atomic<int> failures{0};
        std::vector<std::thread> workers;
        for(int t=0;t<THREADS;++t) {
            workers.emplace_back([&,t] {
                @autoreleasepool {
                    id<MTLBuffer> out=[device newBufferWithLength:bytes options:MTLResourceStorageModeShared];
                    if(!out) { ++failures; return; }
                    for(int i=0;i<ITERS;++i) {
                        std::memset(out.contents,0x5A,bytes);
                        const int rc=eg_metal_render(state,(__bridge void*)src,row_bytes,W,H,
                                                     (__bridge void*)out,row_bytes,W,H,&p);
                        if(rc!=0 || std::memcmp(ref_bytes.data(),out.contents,bytes)!=0) ++failures;
                    }
                    (void)t;
                }
            });
        }
        for(auto& th:workers) th.join();
        if(failures.load()!=0) {
            std::cerr<<"Metal concurrent determinism failures="<<failures.load()<<"\n";
            eg_metal_destroy(state); return 4;
        }

        eg_metal_destroy(state);
        std::cout<<"Metal determinism: PASS sequential=384 concurrent=256\n";
        return 0;
    }
}
