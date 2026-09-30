#include "bridge/plane_ffi.h"
#include "core/PlaneRenderer.h"
#include <cassert>
#include <cmath>
#include <cstring>
#include <vector>
using namespace elasticgrid;

template<typename T> void verify(int depth) {
    constexpr int n=9,stride=40;
    std::vector<T> source(n*stride),out(n*stride,T(77)),expected(n*stride,T(77));
    for(int i=0;i<n*stride;++i) source[i]=T(i%101);
    float columns[]={0,.75f,1},rows[]={0,.4f,1};
    EgPlaneFrame frame{{0,0,8,0,8,8,0,8},columns,rows,3,3,1,1,n,n,0,0,0,0,0,.25f,nullptr,nullptr};
    EgPlaneImage src{source.data(),stride*sizeof(T),n,n},dst{out.data(),stride*sizeof(T),n,n};
    EgPlaneReport report{1,1,9,9};
    auto transform=PlaneTransform::fromCorners({{{0,0},{8,0},{8,8},{0,8}}});assert(transform);
    auto warp=PlaneWarp::prepare(*transform,{0,.75f,1},{0,.4f,1});assert(warp);
    if constexpr(std::is_same_v<T,float>) renderPlaneRGBAfRegion(
        {source.data(),n,n,stride},{expected.data(),n,n,stride},{n,n},&*warp);
    else if constexpr(std::is_same_v<T,std::uint8_t>) renderPlaneRGBA8Region(
        {source.data(),n,n,stride},{expected.data(),n,n,stride},{n,n},&*warp);
    else renderPlaneRGBA16Region({source.data(),n,n,stride},{expected.data(),n,n,stride},{n,n},&*warp);
    assert(eg_render_plane(&src,&dst,depth,&frame,&report)==0);
    assert(!std::memcmp(out.data(),expected.data(),out.size()*sizeof(T)));
    assert(report.invalid_plane==0 && report.reserved==0);
    const double inputPlane[]={0,0,8,0,8,8,0,8};
    assert(eg_render_plane_between(&src,&dst,depth,&frame,&report,1,0,inputPlane)==0);
    assert(!std::memcmp(out.data(),expected.data(),out.size()*sizeof(T)));
    auto beforeBad=out;
    const double badPlane[]={0,0,0,0,0,0,0,0};
    assert(eg_render_plane_between(&src,&dst,depth,&frame,&report,1,0,badPlane)==1 && out==beforeBad);
    assert(eg_render_plane_between(&src,&dst,depth,&frame,&report,1,0,nullptr)==1 && out==beforeBad);
    frame.corners[2]=0;frame.corners[3]=0; // collapsed plane preserves input
    assert(eg_render_plane(&src,&dst,depth,&frame,&report)==0 && report.invalid_plane==1);
    for(int y=0;y<n;++y) assert(!std::memcmp(&out[y*stride],&source[y*stride],n*4*sizeof(T)));
    auto unchanged=out;
    frame.column_count=53;
    assert(eg_render_plane(&src,&dst,depth,&frame,&report)==1 && out==unchanged);
    assert(report.invalid_plane==0 && report.outside_pixels==0);
    frame.column_count=3;columns[1]=1;
    assert(eg_render_plane(&src,&dst,depth,&frame,&report)==1 && out==unchanged);
    columns[1]=.75f;
    assert(eg_render_plane(&src,&dst,24,&frame,&report)==2);
    assert(eg_render_plane(&src,&src,depth,&frame,&report)==1);
    auto bad=dst;bad.row_bytes=-bad.row_bytes;
    assert(eg_render_plane(&src,&bad,depth,&frame,&report)==1);
    frame.abort_fn=[](void*)->std::int32_t{return 1;};
    assert(eg_render_plane(&src,&dst,depth,&frame,&report)==5 && out==unchanged);
    frame.abort_fn=nullptr;
    EgPlaneImage empty{nullptr,0,0,0};
    assert(eg_render_plane(&empty,&dst,depth,&frame,&report)==0);
    for(int y=0;y<n;++y) for(int x=0;x<n*4;++x) assert(out[y*stride+x]==0);
    assert(eg_render_plane(nullptr,&dst,depth,&frame,&report)==1);
}
template<typename T> void verifyRegion(int depth) {
    constexpr int n=9,stride=40;
    std::vector<T> source(n*stride),out(n*stride,T(77));
    for(int i=0;i<n*stride;++i) source[i]=T(i%101);
    float columns[]={0,.5f,1},rows[]={0,.5f,1};
    EgPlaneFrame frame{{1,1,7,2,6,7,2,6},columns,rows,3,3,1,1,n,n,0,0,0,0,0,.25f,nullptr,nullptr};
    EgPlaneImage src{source.data(),stride*sizeof(T),n,n},dst{out.data(),stride*sizeof(T),n,n};
    EgPlaneReport report{};
    for(int quality=0;quality<2;++quality) for(int edge=0;edge<3;++edge) {
        assert(eg_render_plane_region(&src,&dst,depth,&frame,&report,quality,edge)==0);
        for(int y=0;y<n;++y) {
            assert(!std::memcmp(&out[y*stride],&source[y*stride],n*4*sizeof(T)));
            for(int i=n*4;i<stride;++i)assert(out[y*stride+i]==T(77));
        }
    }
    columns[1]=.75f;
    assert(eg_render_plane_region(&src,&dst,depth,&frame,&report,1,0)==0);
    auto plane=PlaneTransform::fromCorners({{{1,1},{7,2},{6,7},{2,6}}});assert(plane);
    bool changed=false;
    for(int y=0;y<n;++y) for(int x=0;x<n;++x) {
        auto local=plane->toLocal({double(x),double(y)});
        bool same=!std::memcmp(&out[y*stride+x*4],&source[y*stride+x*4],4*sizeof(T));
        if(!local || local->x<0 || local->x>1 || local->y<0 || local->y>1) assert(same);
        else changed|=!same;
    }
    assert(changed);
}
int main(){
    verifyRegion<std::uint8_t>(8);verifyRegion<std::uint16_t>(16);verifyRegion<float>(32);
    verify<std::uint8_t>(8);verify<std::uint16_t>(16);verify<float>(32);
    // AE scales the full-resolution endpoint (127 -> 63.5), not
    // the rounded raster endpoint (64 - 1). A fit plane must stay identity.
    {
        constexpr int w=64,h=48;
        std::vector<float> src(w*h*4),dst(w*h*4);
        for(int i=0;i<w*h*4;++i)src[i]=float(i%97)/96;
        float axis[]={0,1};
        EgPlaneFrame frame{{0,0,63.5,0,63.5,47.5,0,47.5},axis,axis,2,2,1,1,w,h,0,0,0,0,0,.25f,nullptr,nullptr};
        EgPlaneImage a{src.data(),w*4*sizeof(float),w,h},b{dst.data(),w*4*sizeof(float),w,h};
        EgPlaneReport report{};
        for(int quality=0;quality<2;++quality)for(int edge=0;edge<3;++edge){
            assert(eg_render_plane_projected(&a,&b,32,&frame,&report,quality,edge,63.5,47.5)==0);
            // Float bicubic arithmetic may round by a few ULPs; the original
            // half-pixel compression differs by orders of magnitude more.
            for(std::size_t i=0;i<src.size();++i)assert(std::abs(src[i]-dst[i])<2e-6f);
        }
    }
    // A reduced/translated quad must project the source, not leave it stationary.
    // Float markers include negative/HDR values; outside destination is transparent.
    {
        constexpr int n=9;
        std::vector<float> src(n*n*4),dst(n*n*4,-99);
        for(int y=0;y<n;y++)for(int x=0;x<n;x++){
            auto k=(y*n+x)*4;src[k]=float(x)-2;src[k+1]=float(y);src[k+2]=2;src[k+3]=1;
        }
        float axis[]={0,1};
        EgPlaneFrame frame{{2,2,6,2,6,6,2,6},axis,axis,2,2,1,1,n,n,0,0,0,0,0,.25f,nullptr,nullptr};
        EgPlaneImage a{src.data(),n*4*sizeof(float),n,n},b{dst.data(),n*4*sizeof(float),n,n};
        EgPlaneReport report{};
        assert(eg_render_plane(&a,&b,32,&frame,&report)==0);
        for(int y=0;y<n;y++)for(int x=0;x<n;x++){
            auto k=(y*n+x)*4;
            if(x<2||x>6||y<2||y>6)for(int c=0;c<4;c++)assert(dst[k+c]==0);
            else {assert(std::abs(dst[k]-(2*(x-2)-2))<1e-6);assert(std::abs(dst[k+1]-2*(y-2))<1e-6);assert(dst[k+2]==2&&dst[k+3]==1);}
        }
        assert(report.outside_pixels==56);
    }
    const double corners[]={10,20,180,35,130,160,-15,115};
    auto* geometry=eg_plane_geometry_create(corners);assert(geometry);
    for(int y=0;y<=20;++y) for(int x=0;x<=20;++x) {
        double surface[2],local[2];
        assert(eg_plane_geometry_map(geometry,0,x/20.,y/20.,surface)==0);
        assert(eg_plane_geometry_map(geometry,1,surface[0],surface[1],local)==0);
        assert(std::abs(local[0]-x/20.)<1e-10 && std::abs(local[1]-y/20.)<1e-10);
    }
    double output[]={7,8};
    assert(eg_plane_geometry_map(geometry,2,0,0,output)==1 && output[0]==7 && output[1]==8);
    assert(eg_plane_geometry_map(nullptr,0,0,0,output)==1);
    eg_plane_geometry_destroy(geometry);eg_plane_geometry_destroy(nullptr);
    const double invalid[8]{};
    assert(eg_plane_geometry_create(invalid)==nullptr);
    assert(eg_plane_geometry_create(nullptr)==nullptr);
}
