#include "core/PlaneRenderer.h"
#include "bridge/plane_ffi.h"
#include <array>
#include <cassert>
#include <cstring>
#include <limits>
#include <type_traits>
#include <vector>
using namespace elasticgrid;

template<typename T>
PlaneRenderReport render(const T* src,int sw,int sh,int ss,T* dst,int dw,int dh,int ds,
                         PlaneCanvasRegion region,const PlaneWarp* warp,AbortFn abort=nullptr) {
    if constexpr(std::is_same_v<T,float>)
        return renderPlaneRGBAfRegion({src,sw,sh,ss},{dst,dw,dh,ds},region,warp,abort);
    else if constexpr(std::is_same_v<T,std::uint8_t>)
        return renderPlaneRGBA8Region({src,sw,sh,ss},{dst,dw,dh,ds},region,warp,abort);
    else
        return renderPlaneRGBA16Region({src,sw,sh,ss},{dst,dw,dh,ds},region,warp,abort);
}

template<typename T> void verify() {
    constexpr int n=9,expanded=13,stride=expanded*4+8;
    std::vector<T> dense(n*n*4),storage(expanded*stride,T(231));
    std::vector<T> reference(n*n*4),output(expanded*stride,T(231));
    for(int y=0;y<n;++y) for(int x=0;x<n;++x) for(int c=0;c<4;++c) {
        T value;
        if constexpr(std::is_same_v<T,float>) value=(x-y+c)*.75f;
        else value=T((x+y+c)*7);
        dense[(y*n+x)*4+c]=value;
        storage[(y+2)*stride+(x+2)*4+c]=value;
    }
    auto h=PlaneTransform::fromCorners({{{0,0},{8,1},{7,8},{1,7}}});assert(h);
    auto moved=PlaneWarp::prepare(*h,{0,.75f,1},{0,.3f,1});assert(moved);
    auto identity=PlaneWarp::prepare(*h,{0,.5f,1},{0,.5f,1});assert(identity);
    for(const PlaneWarp* warp: std::array<const PlaneWarp*,3>{&*moved,&*identity,nullptr}) {
        render(dense.data(),n,n,n*4,reference.data(),n,n,n*4,{n,n},warp);
        render(storage.data(),expanded,expanded,stride,output.data(),expanded,expanded,stride,
               {n,n,-2,-2,-2,-2},warp);
        for(int y=0;y<expanded;++y) for(int x=0;x<expanded;++x) {
            const auto pixel=&output[y*stride+x*4];
            if(x>=2 && y>=2 && x<n+2 && y<n+2)
                assert(!std::memcmp(pixel,&reference[((y-2)*n+x-2)*4],4*sizeof(T)));
            else for(int c=0;c<4;++c) assert(pixel[c]==0);
        }
        for(int y=0;y<expanded;++y) for(int x=expanded*4;x<stride;++x)
            assert(output[y*stride+x]==231); // row padding is not output
        // Origins near either signed limit must not overflow into valid pixels.
        for(int origin: {std::numeric_limits<int>::min(),std::numeric_limits<int>::max()}) {
            render(storage.data(),expanded,expanded,stride,output.data(),expanded,expanded,stride,
                   {n,n,origin,origin,origin,origin},warp);
            for(int y=0;y<expanded;++y) for(int x=0;x<expanded*4;++x)
                assert(output[y*stride+x]==0);
            render(storage.data(),expanded,expanded,stride,output.data(),n,n,stride,
                   {n,n,origin,origin,0,0},warp);
            for(int y=0;y<n;++y) for(int x=0;x<n*4;++x) assert(output[y*stride+x]==0);
        }
    }
    bool cancelled=false;
    try {render(dense.data(),n,n,n*4,output.data(),expanded,expanded,stride,
                {n,n,0,0,-100,-100},&*moved,[](void*)->std::int32_t{return 1;});}
    catch(const RenderCancelled&) {cancelled=true;}
    assert(cancelled); // even a wholly transparent expanded world is cancellable
}
// Comp can move real image pixels beyond source bounds, with unchanged sampling.
// Layer retains its clipping. Neutral Comp must not invent edge pixels.
template<typename T> void compDestination(int depth) {
    constexpr int n=9,w=25,h=25,stride=w*4+8;
    std::vector<T> input(n*n*4,T(17)),comp(h*stride,T(231)),layer(h*stride,T(231));
    float cols[]={0,.8f,1},rows[]={0,.5f,1};
    EgPlaneFrame frame{{-8,-8,17,-8,17,17,-8,17},cols,rows,3,3,1,1,n,n,0,0,-8,-8,0,.25f,nullptr,nullptr};
    EgPlaneImage src{input.data(),n*4*std::ptrdiff_t(sizeof(T)),n,n};
    EgPlaneImage dst{comp.data(),stride*std::ptrdiff_t(sizeof(T)),w,h};
    EgPlaneReport report{};
    for(int edge=0;edge<3;edge++)for(int quality=0;quality<2;quality++){
        assert(eg_render_plane_comp(&src,&dst,depth,&frame,&report,quality,edge)==0);
        dst.pixels=layer.data();
        assert(eg_render_plane_layer(&src,&dst,depth,&frame,&report,quality,edge)==0);
        bool outside=false;
        for(int y=0;y<h;y++)for(int x=0;x<w;x++){
            const bool inside=x>=8 && y>=8 && x<17 && y<17;
            for(int c=0;c<4;c++){
                if(inside)assert(comp[y*stride+x*4+c]==layer[y*stride+x*4+c]);
                else {assert(layer[y*stride+x*4+c]==0);outside|=comp[y*stride+x*4+c]!=0;}
            }
        }
        assert(outside);
        dst.pixels=comp.data();
        for(int y=0;y<h;y++)for(int x=w*4;x<stride;x++)assert(comp[y*stride+x]==T(231));
    }
    // None renders only samples from the real source, never repeated edge colors.
    assert(eg_render_plane_comp(&src,&dst,depth,&frame,&report,1,3)==0);
    for(int c=0;c<4;c++)assert(comp[c]==0 && comp[(h-1)*stride+(w-1)*4+c]==0);
    bool moved_outside=false;
    for(int y=8;y<17;y++)for(int x=17;x<w;x++)moved_outside|=comp[y*stride+x*4]!=0;
    assert(moved_outside); // transparent edges must not reintroduce layer clipping
    cols[1]=.5f;
    assert(eg_render_plane_comp(&src,&dst,depth,&frame,&report,1,3)==0);
    for(int y=0;y<h;y++)for(int x=0;x<w;x++)for(int c=0;c<4;c++)
        assert(comp[y*stride+x*4+c]==(x>=8 && y>=8 && x<17 && y<17?T(17):T(0)));
}
int main() { verify<std::uint8_t>();verify<std::uint16_t>();verify<float>();
compDestination<std::uint8_t>(8);compDestination<std::uint16_t>(16);compDestination<float>(32); }
