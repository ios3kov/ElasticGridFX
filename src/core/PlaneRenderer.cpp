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
    auto se=endAddress(src.data,src.width,src.height,src.row_stride_floats);
    auto de=endAddress(dst.data,dst.width,dst.height,dst.row_stride_floats);
    if(reinterpret_cast<std::uintptr_t>(src.data)<de && reinterpret_cast<std::uintptr_t>(dst.data)<se)
        throw std::invalid_argument("overlapping plane image views");
    PlaneRenderReport report;report.invalid_plane=!warp;
    for(int y=0;y<dst.height;++y) {
        if(abort && abort(refcon)) throw RenderCancelled{};
        auto out=dst.data+static_cast<std::ptrdiff_t>(y)*dst.row_stride_floats;
        for(int x=0;x<dst.width;++x) {
            PlanePoint q{static_cast<double>(x),static_cast<double>(y)},p=q;
            if(warp) {
                auto mapped=warp->sourceFor(q);
                if(mapped.status==PlaneMapStatus::Mapped) p=*mapped.source;
                else if(mapped.status==PlaneMapStatus::OutsidePlane) ++report.outside_pixels;
                else ++report.invalid_projection_pixels;
            }
            auto pixel=out+static_cast<std::ptrdiff_t>(x)*4;
            if(p.x==q.x && p.y==q.y) {
                std::memcpy(pixel,src.data+static_cast<std::ptrdiff_t>(y)*src.row_stride_floats+
                            static_cast<std::ptrdiff_t>(x)*4,4*sizeof(float));
                continue;
            }
            auto xs=taps(p.x,src.width),ys=taps(p.y,src.height);
            for(int c=0;c<4;++c) {
                float value=0;
                for(int j=0;j<4;++j) {
                    float horizontal=0;
                    auto row=src.data+static_cast<std::ptrdiff_t>(ys.index[j])*src.row_stride_floats;
                    for(int i=0;i<4;++i) horizontal+=row[static_cast<std::ptrdiff_t>(xs.index[i])*4+c]*xs.w[i];
                    value+=horizontal*ys.w[j];
                }
                pixel[c]=value; // preserve float alpha/HDR/negative values, no clamp
            }
        }
    }
    return report;
}
}
