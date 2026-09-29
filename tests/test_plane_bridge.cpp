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
int main(){
    verify<std::uint8_t>(8);verify<std::uint16_t>(16);verify<float>(32);
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
