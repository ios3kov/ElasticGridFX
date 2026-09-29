#include "core/PlaneRenderer.h"
#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstring>
#include <limits>
#include <stdexcept>
#include <type_traits>
#include <vector>
using namespace elasticgrid;
template<typename T>
void run() {
    constexpr int n=9,stride=40,maximum=sizeof(T)==1?255:32768;
    std::vector<T> input(n*stride,0),out(n*stride,7),reference(n*stride,7);
    auto render=[&](const T* data,int width,int height,int pitch,T* dest,
                    PlaneCanvasRegion region,const PlaneWarp* warp) {
        if constexpr(sizeof(T)==1)
            return renderPlaneRGBA8Region({data,width,height,pitch},{dest,n,n,stride},region,warp);
        else
            return renderPlaneRGBA16Region({data,width,height,pitch},{dest,n,n,stride},region,warp);
    };
    auto plane=PlaneTransform::fromCorners({{{0,0},{8,0},{8,8},{0,8}}});assert(plane);
    auto warp=PlaneWarp::prepare(*plane,{0,.75f,1},{0,.25f,1});assert(warp);
    for(int y=0;y<n;++y) for(int x=0;x<n;++x) for(int c=0;c<4;++c)
        input[y*stride+x*4+c]=static_cast<T>((x+y+c)%3==0?maximum:0);
    auto fallback=render(input.data(),n,n,stride,out.data(),{n,n},nullptr);
    assert(fallback.invalid_plane);
    for(int y=0;y<n;++y) assert(!std::memcmp(&out[y*stride],&input[y*stride],n*4*sizeof(T)));
    RenderSettings settings;settings.quality=SampleQuality::Bicubic;settings.threads=1;
    auto plan=prepareWarpRGBAf(n,n,n,n,buildInverseLUT({0,.75f,1},n),buildInverseLUT({0,.25f,1},n),settings);
    if constexpr(sizeof(T)==1)
        renderWarpRGBA8Prepared({input.data(),n,n,stride},{reference.data(),n,n,stride},plan,settings);
    else
        renderWarpRGBA16Prepared({input.data(),n,n,stride},{reference.data(),n,n,stride},plan,settings);
    render(input.data(),n,n,stride,out.data(),{n,n},&*warp);
    for(int y=0;y<n;++y) {
        for(int i=0;i<n*4;++i) {
            assert(out[y*stride+i]<=maximum);
            assert(std::abs(int(out[y*stride+i])-int(reference[y*stride+i]))<=1);
        }
        for(int i=n*4;i<stride;++i) assert(out[y*stride+i]==7);
    }
    std::vector<T> compact(3*16,0);
    std::fill(input.begin(),input.end(),0);
    for(int y=0;y<3;++y) for(int x=0;x<3;++x) for(int c=0;c<4;++c) {
        compact[y*16+x*4+c]=static_cast<T>(maximum/(c+1));
        input[(y+2)*stride+(x+3)*4+c]=compact[y*16+x*4+c];
    }
    render(input.data(),n,n,stride,reference.data(),{n,n},&*warp);
    render(compact.data(),3,3,16,out.data(),{n,n,3,2},&*warp);
    for(int y=0;y<n;++y) assert(!std::memcmp(&out[y*stride],&reference[y*stride],n*4*sizeof(T)));
    render(nullptr,0,0,0,out.data(),{n,n},&*warp);
    for(int y=0;y<n;++y) for(int i=0;i<n*4;++i) assert(out[y*stride+i]==0);
    bool rejected=false;
    try {render(input.data(),n,n,1,out.data(),{n,n},&*warp);}
    catch(const std::invalid_argument&) {rejected=true;} assert(rejected);
    auto small=PlaneTransform::fromCorners({{{2,2},{6,2},{6,6},{2,6}}});assert(small);
    auto smallwarp=PlaneWarp::prepare(*small,{0,.75f,1},{0,.5f,1});assert(smallwarp);
    input[0]=std::numeric_limits<T>::max(); // even unusual input is preserved outside
    auto result=render(input.data(),n,n,stride,out.data(),{n,n},&*smallwarp);
    assert(result.outside_pixels==56 && out[0]==input[0]);
}
int main() {run<std::uint8_t>();run<std::uint16_t>();}
