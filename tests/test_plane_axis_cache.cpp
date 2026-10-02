#include "core/PlaneRenderer.h"
#include <array>
#include <bit>
#include <cassert>
#include <cmath>
#include <cstring>
#include <cstdio>
#include <future>
#include <thread>
#include <type_traits>
#include <vector>
using namespace elasticgrid;

template<typename T>
PlaneRenderReport render(const T* input,int sw,int sh,int ss,T* output,int dw,int dh,int ds,
                         PlaneCanvasRegion region,const PlaneWarp* warp,
                         AbortFn abort=nullptr,void* refcon=nullptr) {
    if constexpr(std::is_same_v<T,float>)
        return renderPlaneRGBAfRegion({input,sw,sh,ss},{output,dw,dh,ds},region,warp,abort,refcon);
    else if constexpr(std::is_same_v<T,std::uint8_t>)
        return renderPlaneRGBA8Region({input,sw,sh,ss},{output,dw,dh,ds},region,warp,abort,refcon);
    else return renderPlaneRGBA16Region({input,sw,sh,ss},{output,dw,dh,ds},region,warp,abort,refcon);
}

template<typename T>
void compare(const PlaneWarp* warp,PlaneCanvasRegion region,int sw,int sh,int dw,int dh) {
    const int ss=sw*4+8,ds=dw*4+12;
    std::vector<T> input(static_cast<std::size_t>(ss)*sh),cached(static_cast<std::size_t>(ds)*dh,T(17)),
                   scalar(cached);
    for(int y=0;y<sh;++y) for(int x=0;x<sw;++x) for(int c=0;c<4;++c) {
        const unsigned value=(static_cast<unsigned>(x)*137+static_cast<unsigned>(y)*79+
                              static_cast<unsigned>(c)*47)%251;
        if constexpr(std::is_same_v<T,float>)
            input[static_cast<std::size_t>(y)*ss+x*4+c]=c==3?float(value)/250:float(value)/31-3;
        else input[static_cast<std::size_t>(y)*ss+x*4+c]=
            static_cast<T>(value*(sizeof(T)==2?128:1));
    }
    if constexpr(std::is_same_v<T,float>) {
        if(!input.empty()) input[0]=std::bit_cast<float>(0x80000000u); // negative zero
        if(input.size()>17) {input[9]=INFINITY;input[17]=std::bit_cast<float>(0x7fc00123u);}
    }
    region.cache_axis_mapping=true;
    const auto a=render(input.data(),sw,sh,ss,cached.data(),dw,dh,ds,region,warp);
    region.cache_axis_mapping=false;
    const auto b=render(input.data(),sw,sh,ss,scalar.data(),dw,dh,ds,region,warp);
    assert(a.invalid_plane==b.invalid_plane && a.outside_pixels==b.outside_pixels &&
           a.invalid_projection_pixels==b.invalid_projection_pixels);
    if(std::memcmp(cached.data(),scalar.data(),cached.size()*sizeof(T))) {
        for(std::size_t i=0;i<cached.size();++i) {
            if(std::memcmp(&cached[i],&scalar[i],sizeof(T))) {
                std::uint32_t actual=0,expected=0;
                std::memcpy(&actual,&cached[i],sizeof(T));
                std::memcpy(&expected,&scalar[i],sizeof(T));
                std::fprintf(stderr,"plane parity: element=%zu depth=%zu cached=%08x general=%08x "
                    "source=%dx%d output=%dx%d origin=%d,%d quality=%d edge=%d separable=%d\n",
                    i,sizeof(T)*8,actual,expected,sw,sh,dw,dh,region.output_x,region.output_y,
                    int(region.quality),int(region.edge),warp && bool(warp->separableAnchor()));
                break;
            }
        }
        assert(false && "cached/general plane pixels must match exactly");
    }
    // Cancellation remains on the calling thread, before any mapping/output.
    struct Abort {std::thread::id owner;int calls=0;int limit;};
    auto callback=[](void* raw)->std::int32_t {
        auto& state=*static_cast<Abort*>(raw);assert(std::this_thread::get_id()==state.owner);
        return ++state.calls>=state.limit;
    };
    for(int limit:{1,3}) {
        std::fill(cached.begin(),cached.end(),T(17));
        Abort state{std::this_thread::get_id(),0,limit};
        region.cache_axis_mapping=true;
        bool cancelled=false;
        try {render(input.data(),sw,sh,ss,cached.data(),dw,dh,ds,region,warp,callback,&state);}
        catch(const RenderCancelled&) {cancelled=true;}
        assert(cancelled && state.calls==limit);
        if(limit==1) for(T value:cached) assert(value==T(17));
        std::fill(cached.begin(),cached.end(),T(17));
        render(input.data(),sw,sh,ss,cached.data(),dw,dh,ds,region,warp);
        assert(!std::memcmp(cached.data(),scalar.data(),cached.size()*sizeof(T)));
    }
}

template<typename T>
void matrix() {
    const std::array<std::array<PlanePoint,4>,7> quads{{
        {{{0,0},{40,0},{40,30},{0,30}}},
        {{{3.3,4.2},{27.6,4.2},{27.6,26.8},{3.3,26.8}}},
        {{{40,30},{0,30},{0,0},{40,0}}},
        {{{-7,-9},{21,-9},{21,18},{-7,18}}},
        {{{2,2},{37,2.000000001},{37,29},{2,29}}}, // no approximate eligibility
        {{{0,0},{1e-308,0},{1e-308,1e-308},{0,1e-308}}},
        {{{3,4},{38,2},{33,29},{7,25}}}
    }};
    for(const auto& quad:quads) {
        auto transform=PlaneTransform::fromCorners(quad);assert(transform);
        for(bool neutral:{false,true}) for(bool layer:{false,true}) {
            const std::vector<float> columns=neutral?std::vector<float>{0,.25f,.5f,.75f,1}:
                std::vector<float>{0,.08f,.36f,.51f,.9f,1};
            const std::vector<float> rows=neutral?std::vector<float>{0,.5f,1}:
                std::vector<float>{0,.21f,.77f,1};
            auto warp=layer?PlaneWarp::prepareLayer(*transform,columns,rows,.4f,.25f):
                PlaneWarp::prepare(*transform,columns,rows,.4f,.25f);
            assert(warp);
            if(!transform->axisAligned()) assert(!warp->separableAnchor());
            for(auto edge:{EdgeMode::Clamp,EdgeMode::Wrap,EdgeMode::Mirror})
            for(auto quality:{SampleQuality::Bilinear,SampleQuality::Bicubic}) {
                PlaneCanvasRegion region{41,31};region.edge=edge;region.quality=quality;
                compare<T>(&*warp,region,41,31,41,31);
                region.source_x=7;region.source_y=5;region.output_x=-3;region.output_y=-4;
                compare<T>(&*warp,region,19,13,47,39);
                region.surface_units_x=2.4;region.surface_units_y=.75;
                compare<T>(&*warp,region,19,13,17,11); // explicit general-path fallback
                region.surface_units_x=region.surface_units_y=1;
                compare<T>(&*warp,region,0,0,17,11); // empty sparse checkout
            }
        }
        auto projected=PlaneWarp::prepareProjected(*transform,{40,30},{0,.7f,1},{0,.4f,1});
        auto between=PlaneWarp::prepareBetween(*transform,*transform,{0,.7f,1},{0,.4f,1});
        assert(projected && between);
        assert(!projected->separableAnchor() && !between->separableAnchor());
        compare<T>(&*projected,{41,31},41,31,41,31);
        compare<T>(&*between,{41,31},41,31,41,31);
    }
    compare<T>(nullptr,{41,31},41,31,41,31);
}
int main() {
    // Concurrent independent frames exercise local cache ownership like MFR.
    auto a=std::async(std::launch::async,[]{matrix<float>();});
    auto b=std::async(std::launch::async,[]{matrix<std::uint8_t>();});
    auto c=std::async(std::launch::async,[]{matrix<std::uint16_t>();});
    matrix<float>(); a.get();b.get();c.get();
}
