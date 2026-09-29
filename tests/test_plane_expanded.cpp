#include "core/PlaneRenderer.h"
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
int main() { verify<std::uint8_t>();verify<std::uint16_t>();verify<float>(); }
