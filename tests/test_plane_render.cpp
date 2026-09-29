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
    // Compact checkout equals a zero-filled canvas, including fractional border taps.
    std::fill(input.begin(),input.end(),0);
    std::vector<float> compact(3*16,0),tile(4*20,-777);
    for(int y=0;y<3;++y) for(int x=0;x<3;++x) for(int c=0;c<4;++c) {
        float v=c==3?.5f:static_cast<float>(x-y+c)*2;
        compact[y*16+x*4+c]=v; input[(y+2)*stride+(x+3)*4+c]=v;
    }
    for(const PlaneWarp* selected:std::array<const PlaneWarp*,3>{&*fullwarp,&*identity,nullptr}) {
        renderPlaneRGBAf(src,dst,selected);
        renderPlaneRGBAfRegion({compact.data(),3,3,16},{reference.data(),n,n,stride},
                               {n,n,3,2,0,0},selected);
        for(int y=0;y<n;++y)
            assert(!std::memcmp(&output[y*stride],&reference[y*stride],n*4*sizeof(float)));
        renderPlaneRGBAfRegion({compact.data(),3,3,16},{tile.data(),4,4,20},
                               {n,n,3,2,2,3},selected);
        for(int y=0;y<4;++y) {
            assert(!std::memcmp(&tile[y*20],&output[(y+3)*stride+2*4],16*sizeof(float)));
            for(int i=16;i<20;++i) assert(tile[y*20+i]==-777);
        }
    }
    renderPlaneRGBAfRegion({nullptr,0,0,0},dst,{n,n},&*fullwarp);
    for(int y=0;y<n;++y) for(int i=0;i<n*4;++i) assert(output[y*stride+i]==0);
    rejected=false;
    try {renderPlaneRGBAfRegion(src,dst,{0,n,1,0,0,0},nullptr);}
    catch(const std::invalid_argument&) {rejected=true;} assert(rejected);
    // Same surface under anisotropic raster scale (downsample/PAR basis).
    for(auto scale:std::array<PlanePoint,3>{{{2,2},{2.4,3},{.75,1.25}}}) {
        auto scaledPlane=PlaneTransform::fromCorners({{{0,0},{8*scale.x,0},
            {8*scale.x,8*scale.y},{0,8*scale.y}}});assert(scaledPlane);
        auto scaledWarp=PlaneWarp::prepare(*scaledPlane,{0,.75f,1},{0,.25f,1});assert(scaledWarp);
        renderPlaneRGBAf(src,dst,&*fullwarp);
        renderPlaneRGBAfRegion(src,{reference.data(),n,n,stride},
            {n,n,0,0,0,0,scale.x,scale.y},&*scaledWarp);
        for(int y=0;y<n;++y) for(int i=0;i<n*4;++i)
            assert(std::abs(output[y*stride+i]-reference[y*stride+i])<1e-5);
        auto unchanged=PlaneWarp::prepare(*scaledPlane,{0,.5f,1},{0,.5f,1});assert(unchanged);
        renderPlaneRGBAfRegion(src,dst,{n,n,0,0,0,0,scale.x,scale.y},&*unchanged);
        for(int y=0;y<n;++y) assert(!std::memcmp(&input[y*stride],&output[y*stride],n*4*sizeof(float)));
    }
    for(double bad:std::array<double,4>{0,-1,INFINITY,NAN}) {
        rejected=false;
        try {renderPlaneRGBAfRegion(src,dst,{n,n,0,0,0,0,bad,1},nullptr);}
        catch(const std::invalid_argument&) {rejected=true;} assert(rejected);
    }
}
