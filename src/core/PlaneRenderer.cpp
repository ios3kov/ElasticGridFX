#include "core/PlaneRenderer.h"
#include <algorithm>
#include <cmath>
#include <cstring>
#include <limits>
#include <stdexcept>

namespace elasticgrid {
namespace {
std::uintptr_t endAddress(const void* data, int width, int height, std::ptrdiff_t stride) {
    if(!data || width<=0 || height<=0 || stride<static_cast<std::ptrdiff_t>(width)*4)
        throw std::invalid_argument("invalid dense plane image view");
    auto max=static_cast<std::size_t>(std::numeric_limits<std::ptrdiff_t>::max())/sizeof(float);
    auto row=static_cast<std::size_t>(width)*4;
    if(static_cast<std::size_t>(height-1)>(max-row)/static_cast<std::size_t>(stride))
        throw std::invalid_argument("plane image address overflow");
    auto bytes=(static_cast<std::size_t>(height-1)*static_cast<std::size_t>(stride)+row)*sizeof(float);
    auto start=reinterpret_cast<std::uintptr_t>(data);
    if(bytes>std::numeric_limits<std::uintptr_t>::max()-start)
        throw std::invalid_argument("plane image address overflow");
    return start+bytes;
}
float weight(float x) {
    x=std::abs(x);
    if(x<1) return 1.5f*x*x*x-2.5f*x*x+1;
    if(x<2) return -.5f*x*x*x+2.5f*x*x-4*x+2;
    return 0;
}
struct Taps { int index[4]; float w[4]; };
Taps taps(double coordinate,int extent) {
    // Clamp in double before conversion: no huge-coordinate integer conversion.
    double p=std::clamp(coordinate,0.0,static_cast<double>(extent-1));
    int base=static_cast<int>(std::floor(p));
    float fraction=static_cast<float>(p-base),sum=0;
    Taps result{};
    for(int k=0;k<4;++k) {
        auto raw=static_cast<long long>(base)+k-1;
        result.index[k]=static_cast<int>(std::clamp(raw,0LL,static_cast<long long>(extent-1)));
        result.w[k]=weight(fraction-static_cast<float>(k-1));sum+=result.w[k];
    }
    for(float& w:result.w) w/=sum;
    return result;
}
}
PlaneRenderReport renderPlaneRGBAf(const ConstImageRGBAf& src,const ImageRGBAf& dst,
                                   const PlaneWarp* warp,AbortFn abort,void* refcon) {
    if(src.width!=dst.width || src.height!=dst.height)
        throw std::invalid_argument("plane renderer requires same canvas");
    return renderPlaneRGBAfRegion(src,dst,{src.width,src.height},warp,abort,refcon);
}
PlaneRenderReport renderPlaneRGBAfRegion(const ConstImageRGBAf& src,const ImageRGBAf& dst,
    const PlaneCanvasRegion& region,const PlaneWarp* warp,AbortFn abort,void* refcon) {
    auto fits=[](int origin,int extent,int canvas) {
        return canvas>0 && origin>=0 && extent>=0 && origin<=canvas && extent<=canvas-origin;
    };
    if(!fits(region.source_x,src.width,region.canvas_width) ||
       !fits(region.source_y,src.height,region.canvas_height) ||
       !fits(region.output_x,dst.width,region.canvas_width) ||
       !fits(region.output_y,dst.height,region.canvas_height) ||
       ((src.width==0)!=(src.height==0)))
        throw std::invalid_argument("invalid plane canvas region");
    const bool empty=src.width==0;
    auto se=empty?0:endAddress(src.data,src.width,src.height,src.row_stride_floats);
    auto de=endAddress(dst.data,dst.width,dst.height,dst.row_stride_floats);
    if(!empty && reinterpret_cast<std::uintptr_t>(src.data)<de && reinterpret_cast<std::uintptr_t>(dst.data)<se)
        throw std::invalid_argument("overlapping plane image views");
    const float zero[4]{};
    auto sourcePixel=[&](int x,int y)->const float* {
        if(empty || x<region.source_x || y<region.source_y ||
           x-region.source_x>=src.width || y-region.source_y>=src.height) return zero;
        return src.data+static_cast<std::ptrdiff_t>(y-region.source_y)*src.row_stride_floats+
               static_cast<std::ptrdiff_t>(x-region.source_x)*4;
    };
    PlaneRenderReport report;report.invalid_plane=!warp;
    for(int y=0;y<dst.height;++y) {
        if(abort && abort(refcon)) throw RenderCancelled{};
        auto out=dst.data+static_cast<std::ptrdiff_t>(y)*dst.row_stride_floats;
        for(int x=0;x<dst.width;++x) {
            PlanePoint q{static_cast<double>(x+region.output_x),static_cast<double>(y+region.output_y)},p=q;
            if(warp) {
                auto mapped=warp->sourceFor(q);
                if(mapped.status==PlaneMapStatus::Mapped) p=*mapped.source;
                else if(mapped.status==PlaneMapStatus::OutsidePlane) ++report.outside_pixels;
                else ++report.invalid_projection_pixels;
            }
            auto pixel=out+static_cast<std::ptrdiff_t>(x)*4;
            if(p.x==q.x && p.y==q.y) {
                std::memcpy(pixel,sourcePixel(x+region.output_x,y+region.output_y),4*sizeof(float));
                continue;
            }
            auto xs=taps(p.x,region.canvas_width),ys=taps(p.y,region.canvas_height);
            for(int c=0;c<4;++c) {
                float value=0;
                for(int j=0;j<4;++j) {
                    float horizontal=0;
                    for(int i=0;i<4;++i) horizontal+=sourcePixel(xs.index[i],ys.index[j])[c]*xs.w[i];
                    value+=horizontal*ys.w[j];
                }
                pixel[c]=value; // preserve float alpha/HDR/negative values, no clamp
            }
        }
    }
    return report;
}
}
