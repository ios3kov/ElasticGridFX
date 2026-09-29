#include "core/PlaneRenderer.h"
#include <cassert>
#include <cmath>
#include <cstring>
#include <stdexcept>
#include <vector>
using namespace elasticgrid;
int main() {
    constexpr int n=9,stride=n*4+8;
    std::vector<float> input(n*stride,-999),output(n*stride,-777);
    for(int y=0;y<n;++y) for(int x=0;x<n;++x) {
        auto p=&input[y*stride+x*4];p[0]=x-4;p[1]=y+2;p[2]=x+y;p[3]=.25f;
    }
    ConstImageRGBAf src{input.data(),n,n,stride};ImageRGBAf dst{output.data(),n,n,stride};
    auto invalid=renderPlaneRGBAf(src,dst,nullptr);assert(invalid.invalid_plane);
    for(int y=0;y<n;++y) {
        assert(!std::memcmp(&input[y*stride],&output[y*stride],n*4*sizeof(float)));
        for(int i=n*4;i<stride;++i) assert(output[y*stride+i]==-777);
    }
    auto plane=PlaneTransform::fromCorners({{{2,2},{6,2},{6,6},{2,6}}});assert(plane);
    auto warp=PlaneWarp::prepare(*plane,{0,.75f,1},{0,.5f,1});assert(warp);
    auto report=renderPlaneRGBAf(src,dst,&*warp);assert(report.outside_pixels==56);
    // Destination (5,4) maps to (4,4): all channels sampled in one cubic pass.
    assert(std::abs(output[4*stride+5*4])<1e-6);
    assert(std::abs(output[4*stride+5*4+1]-6)<1e-6);
    assert(std::abs(output[4*stride+5*4+3]-.25f)<1e-6);
    assert(output[0]==input[0]);
    assert(std::abs(output[4*stride+4*4]+2.f/3)<2e-6); // fractional, negative float sample
    // Compare the new nonseparable sampler to existing Final on a flat full plane.
    auto full=PlaneTransform::fromCorners({{{0,0},{8,0},{8,8},{0,8}}});assert(full);
    auto fullwarp=PlaneWarp::prepare(*full,{0,.75f,1},{0,.25f,1});assert(fullwarp);
    std::vector<float> reference(n*stride,-777);
    for(int y=0;y<n;++y) for(int x=0;x<n;++x)
        input[y*stride+x*4+2]=(x==4 && y==4)?5.f:0.f;
    RenderSettings settings;settings.quality=SampleQuality::Bicubic;settings.threads=1;
    renderWarpRGBAf(src,{reference.data(),n,n,stride},buildInverseLUT({0,.75f,1},n),
                    buildInverseLUT({0,.25f,1},n),settings);
    renderPlaneRGBAf(src,dst,&*fullwarp);
    for(int y=0;y<n;++y) for(int i=0;i<n*4;++i)
        assert(std::abs(output[y*stride+i]-reference[y*stride+i])<1e-5);
    auto identity=PlaneWarp::prepare(*plane,{0,.5f,1},{0,.5f,1});assert(identity);
    renderPlaneRGBAf(src,dst,&*identity);
    for(int y=0;y<n;++y) assert(!std::memcmp(&input[y*stride],&output[y*stride],n*4*sizeof(float)));
    bool rejected=false;
    try { renderPlaneRGBAf(src,{input.data(),n,n,stride},&*warp); }
    catch(const std::invalid_argument&) {rejected=true;} assert(rejected);
    rejected=false;
    try { renderPlaneRGBAf(src,dst,&*warp,[](void*)->std::int32_t{return 1;}); }
    catch(const RenderCancelled&) {rejected=true;} assert(rejected);
    // Invalid plane preserves even nonfinite input bit patterns (no resampling).
    input[0]=NAN;renderPlaneRGBAf(src,dst,nullptr);
    assert(!std::memcmp(input.data(),output.data(),4*sizeof(float)));
}
