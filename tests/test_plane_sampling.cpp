#include "bridge/plane_ffi.h"
#include "core/PlaneRenderer.h"
#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstring>
#include <type_traits>
#include <vector>
using namespace elasticgrid;

// Independent scalar oracle: unbounded taps, edge resolution after weighting.
static int indexAt(int i,int n,int edge) {
    if(edge==3) return i<0 || i>=n?-1:i;
    if(n==1) return 0;
    if(edge==0) return std::clamp(i,0,n-1);
    if(edge==1) {while(i<0)i+=n;while(i>=n)i-=n;return i;}
    while(i<0 || i>=n) i=i<0?-i:2*(n-1)-i;
    return i;
}
static double kernel(double t,int quality) {
    t=std::abs(t);
    if(quality==0) return std::max(0.0,1-t);
    if(t<1) return 1.5*t*t*t-2.5*t*t+1;
    return t<2?-.5*t*t*t+2.5*t*t-4*t+2:0;
}
template<typename T> void run(int depth,bool projected) {
    constexpr int w=13,h=11,stride=w*4+8;
    std::vector<T> source(h*stride),out(h*stride,T(99)),reference(h*stride,T(99));
    for(int y=0;y<h;++y) for(int x=0;x<w;++x) for(int c=0;c<4;++c) {
        double v=((x*23+y*17+c*13)%103)/102.0;
        if constexpr(std::is_same_v<T,float>) source[y*stride+x*4+c]=float(v*4-1);
        else source[y*stride+x*4+c]=T(v*(depth==8?255:32768));
    }
    // Partly off-canvas skewed plane makes mapped samples cross every edge.
    std::array<PlanePoint,4> corners{{{-8,-6},{20,-3},{16,18},{-4,14}}};
    auto transform=PlaneTransform::fromCorners(corners);assert(transform);
    float columns[]={0,.88f,1},rows[]={0,.12f,1};
    auto warp=projected?PlaneWarp::prepareProjected(*transform,{w-1,h-1},{0,.88f,1},{0,.12f,1}):
                        PlaneWarp::prepare(*transform,{0,.88f,1},{0,.12f,1});assert(warp);
    EgPlaneFrame frame{{-8,-6,20,-3,16,18,-4,14},columns,rows,3,3,1,1,w,h,0,0,0,0,0,.25f,nullptr,nullptr};
    EgPlaneImage src{source.data(),stride*sizeof(T),w,h},dst{out.data(),stride*sizeof(T),w,h};
    EgPlaneReport report{};
    std::vector<T> variants[8];
    int outside=0;
    for(int quality=0;quality<2;++quality) for(int edge=0;edge<4;++edge) {
        if(projected) assert(eg_render_plane_sampled(&src,&dst,depth,&frame,&report,quality,edge)==0);
        else {
            PlaneCanvasRegion region{w,h};region.quality=static_cast<SampleQuality>(quality);region.edge=static_cast<EdgeMode>(edge);
            if constexpr(std::is_same_v<T,float>) renderPlaneRGBAfRegion({source.data(),w,h,stride},{out.data(),w,h,stride},region,&*warp);
            else if constexpr(std::is_same_v<T,std::uint8_t>) renderPlaneRGBA8Region({source.data(),w,h,stride},{out.data(),w,h,stride},region,&*warp);
            else renderPlaneRGBA16Region({source.data(),w,h,stride},{out.data(),w,h,stride},region,&*warp);
        }
        for(int y=0;y<h;++y) for(int x=0;x<w;++x) {
            auto mapping=warp->sourceFor({double(x),double(y)});
            assert(mapping.status==PlaneMapStatus::Mapped);
            auto p=*mapping.source;
            if(p.x<0 || p.y<0 || p.x>w-1 || p.y>h-1) ++outside;
            if(edge==0) {p.x=std::clamp(p.x,0.0,double(w-1));p.y=std::clamp(p.y,0.0,double(h-1));}
            const int bx=int(std::floor(p.x)),by=int(std::floor(p.y));
            for(int c=0;c<4;++c) {
                double value=0;
                for(int j=-1;j<=2;++j) for(int i=-1;i<=2;++i) {
                    const int ix=indexAt(bx+i,w,edge),iy=indexAt(by+j,h,edge);
                    if(ix>=0 && iy>=0) value+=source[iy*stride+ix*4+c]*
                        kernel(p.x-bx-i,quality)*kernel(p.y-by-j,quality);
                }
                if constexpr(!std::is_same_v<T,float>) value=std::floor(std::clamp(value,0.0,depth==8?255.0:32768.0)+.5);
                assert(std::abs(double(out[y*stride+x*4+c])-value)<(depth==32?2e-5:1.01));
            }
        }
        for(int y=0;y<h;++y) for(int x=w*4;x<stride;++x) assert(out[y*stride+x]==99);
        variants[quality*4+edge]=out;
    }
    if(!projected) {
        assert(outside>0);
        for(int i=0;i<8;++i) for(int j=i+1;j<8;++j) assert(variants[i]!=variants[j]);
    }
    auto unchanged=out;
    for(auto bad: {-1,2,100}) {
        assert(eg_render_plane_sampled(&src,&dst,depth,&frame,&report,bad,0)==1);
        assert(out==unchanged);
    }
    for(auto bad: {-1,4,100}) assert(eg_render_plane_sampled(&src,&dst,depth,&frame,&report,1,bad)==1);
    assert(out==unchanged);
    // Sparse checkout is zero outside storage, not wrap/mirror at checkout edges.
    std::vector<T> compact(3*4*4),full(h*stride,0);
    for(int y=0;y<3;++y) for(int x=0;x<4;++x) for(int c=0;c<4;++c)
        full[(y+2)*stride+(x+3)*4+c]=compact[y*16+x*4+c]=source[(y+2)*stride+(x+3)*4+c];
    EgPlaneImage sparse{compact.data(),16*sizeof(T),4,3},dense{full.data(),stride*sizeof(T),w,h};
    for(int quality=0;quality<2;++quality) for(int edge=0;edge<4;++edge) {
        frame.source_x=frame.source_y=0;
        assert(eg_render_plane_sampled(&dense,&dst,depth,&frame,&report,quality,edge)==0);
        reference=out;frame.source_x=3;frame.source_y=2;
        assert(eg_render_plane_sampled(&sparse,&dst,depth,&frame,&report,quality,edge)==0);
        assert(out==reference);
    }
}
int main(){for(bool projected:{false,true}){run<std::uint8_t>(8,projected);run<std::uint16_t>(16,projected);run<float>(32,projected);}}
