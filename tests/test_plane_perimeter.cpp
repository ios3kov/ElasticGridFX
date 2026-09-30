#include "bridge/plane_ffi.h"
#include "core/PlaneWarp.h"
#include <algorithm>
#include <array>
#include <cassert>
#include <cmath>
#include <cstdint>
#include <type_traits>
#include <vector>
using namespace elasticgrid;

// Independent two-cell inverse, including both exterior intervals.
static double axis(double u,double guide) {
    return u<=guide ? .5*u/guide : .5+.5*(u-guide)/(1-guide);
}
static double kernel(double x) {
    x=std::abs(x);
    return x<1 ? 1.5*x*x*x-2.5*x*x+1 : x<2 ? -.5*x*x*x+2.5*x*x-4*x+2 : 0;
}
template<class T> void pixels(int depth) {
    constexpr int n=32,stride=n*4+8;
    const double maximum=std::is_same_v<T,float>?1:std::is_same_v<T,std::uint16_t>?32768:255;
    std::vector<T> input(n*stride),output(n*stride,T(17));
    for(int y=0;y<n;y++)for(int x=0;x<n;x++)for(int c=0;c<4;c++)
        input[y*stride+x*4+c]=T(maximum*((x*7+y*11+c*3)%19)/20.);
    float columns[]={0,.72f,1},rows[]={0,.31f,1};
    EgPlaneFrame f{{3.3,4.2,27.6,4.2,27.6,26.8,3.3,26.8},columns,rows,3,3,1,1,n,n,0,0,0,0,0,.25f,nullptr,nullptr};
    EgPlaneImage src{input.data(),stride*std::ptrdiff_t(sizeof(T)),n,n};
    EgPlaneImage dst{output.data(),stride*std::ptrdiff_t(sizeof(T)),n,n};EgPlaneReport report{};
    for(int quality=0;quality<2;quality++) {
        assert(eg_render_plane_layer(&src,&dst,depth,&f,&report,quality,0)==0);
        assert(report.outside_pixels==0 && report.invalid_projection_pixels==0);
        int changed[4]{};
        for(int y=0;y<n;y++)for(int x=0;x<n;x++) {
            double sx=std::clamp(3.3+24.3*axis((x-3.3)/24.3,columns[1]),0.,31.);
            double sy=std::clamp(4.2+22.6*axis((y-4.2)/22.6,rows[1]),0.,31.);
            int ix=int(std::floor(sx)),iy=int(std::floor(sy));double fx=sx-ix,fy=sy-iy;
            for(int c=0;c<4;c++) {
                double expected=0;
                const int lo=quality?-1:0,hi=quality?2:1;
                for(int j=lo;j<=hi;j++)for(int i=lo;i<=hi;i++) {
                    double wx=quality?kernel(fx-i):(i?fx:1-fx);
                    double wy=quality?kernel(fy-j):(j?fy:1-fy);
                    expected+=input[std::clamp(iy+j,0,n-1)*stride+std::clamp(ix+i,0,n-1)*4+c]*wx*wy;
                }
                if constexpr(!std::is_floating_point_v<T>)expected=std::floor(std::clamp(expected,0.,maximum)+.5);
                const double tolerance=std::is_floating_point_v<T>?2e-5:1;
                assert(std::abs(double(output[y*stride+x*4+c])-expected)<=tolerance);
            }
            if(output[y*stride+x*4+3]!=input[y*stride+x*4+3]){
                if(x==3)++changed[0];if(x==28)++changed[1];
                if(y==4)++changed[2];if(y==27)++changed[3];
            }
        }
        for(int count:changed)assert(count>20); // all four outside perimeter lines warped
        for(int y=0;y<n;y++)for(int k=n*4;k<stride;k++)assert(output[y*stride+k]==T(17));
        // Cropped output must match the same portion of the full-canvas result.
        std::vector<T> tile(9*11*4);EgPlaneImage roi{tile.data(),11*4*std::ptrdiff_t(sizeof(T)),11,9};
        f.output_x=1;f.output_y=2;
        assert(eg_render_plane_layer(&src,&roi,depth,&f,&report,quality,0)==0);
        for(int y=0;y<9;y++)for(int x=0;x<11*4;x++)assert(tile[y*44+x]==output[(y+2)*stride+4+x]);
        f.output_x=f.output_y=0;
    }
    columns[1]=rows[1]=.5f;
    assert(eg_render_plane_layer(&src,&dst,depth,&f,&report,1,0)==0);
    for(int y=0;y<n;y++)for(int x=0;x<n*4;x++)assert(input[y*stride+x]==output[y*stride+x]);
    // Bounded Four Corners retains its explicit unchanged exterior contract.
    columns[1]=.72f;rows[1]=.31f;
    assert(eg_render_plane_region(&src,&dst,depth,&f,&report,1,0)==0);
    for(int y=0;y<n;y++)for(int x=0;x<4;x++)for(int c=0;c<4;c++)
        assert(output[y*stride+x*4+c]==input[y*stride+x*4+c]);
}
int main(){
    for(auto quad:std::array<std::array<PlanePoint,4>,3>{{
        {{{3.3,4.2},{27.6,4.2},{27.6,26.8},{3.3,26.8}}},
        {{{20,1},{31,17},{15,29},{4,13}}},
        {{{7,4},{29,9},{22,30},{2,23}}}
    }}){
        auto h=PlaneTransform::fromCorners(quad);assert(h);
        auto w=PlaneWarp::prepareLayer(*h,{0,.75f,1},{0,.25f,1});assert(w);
        auto bounded=PlaneWarp::prepare(*h,{0,.75f,1},{0,.25f,1});assert(bounded);
        for(double u:{-.01,0.,.2,.7,1.,1.01})for(double v:{-.01,0.,.3,.8,1.,1.01}){
            auto p=h->toSurface({u,v});assert(p);auto m=w->sourceFor(*p);
            auto expected=h->toSurface({axis(u,.75),axis(v,.25)});assert(expected);
            assert(m.status==PlaneMapStatus::Mapped);
            assert(std::hypot(m.source->x-expected->x,m.source->y-expected->y)<1e-4);
            if(u<0||u>1||v<0||v>1)assert(bounded->sourceFor(*p).status==PlaneMapStatus::OutsidePlane);
        }
    }
    pixels<std::uint8_t>(8);pixels<std::uint16_t>(16);pixels<float>(32);
}
