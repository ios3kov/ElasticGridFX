#include "core/PlaneRenderer.h"
#include <algorithm>
#include <cmath>
#include <cstring>
#include <limits>
#include <stdexcept>
#include <type_traits>

namespace elasticgrid {
namespace {
template<typename T>
std::uintptr_t endAddress(const T* data, int width, int height, std::ptrdiff_t stride) {
    if(!data || width<=0 || height<=0 || stride<static_cast<std::ptrdiff_t>(width)*4)
        throw std::invalid_argument("invalid dense plane image view");
    auto max=static_cast<std::size_t>(std::numeric_limits<std::ptrdiff_t>::max())/sizeof(T);
    auto row=static_cast<std::size_t>(width)*4;
    if(static_cast<std::size_t>(height-1)>(max-row)/static_cast<std::size_t>(stride))
        throw std::invalid_argument("plane image address overflow");
    auto bytes=(static_cast<std::size_t>(height-1)*static_cast<std::size_t>(stride)+row)*sizeof(T);
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
template<typename Src,typename Dst>
static PlaneRenderReport renderRegion(const Src& src,const Dst& dst,
    std::ptrdiff_t source_stride,std::ptrdiff_t output_stride,
    const PlaneCanvasRegion& region,const PlaneWarp* warp,AbortFn abort,void* refcon) {
    using T=std::remove_cv_t<std::remove_pointer_t<decltype(src.data)>>;
    if(!std::isfinite(region.surface_units_x) || !std::isfinite(region.surface_units_y) ||
       region.surface_units_x<=0 || region.surface_units_y<=0 ||
       !std::isfinite(region.surface_units_x*region.canvas_width) ||
       !std::isfinite(region.surface_units_y*region.canvas_height))
        throw std::invalid_argument("invalid plane raster scale");
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
    auto se=empty?0:endAddress(src.data,src.width,src.height,source_stride);
    auto de=endAddress(dst.data,dst.width,dst.height,output_stride);
    if(!empty && reinterpret_cast<std::uintptr_t>(src.data)<de && reinterpret_cast<std::uintptr_t>(dst.data)<se)
        throw std::invalid_argument("overlapping plane image views");
    const T zero[4]{};
    auto sourcePixel=[&](int x,int y)->const T* {
        if(empty || x<region.source_x || y<region.source_y ||
           x-region.source_x>=src.width || y-region.source_y>=src.height) return zero;
        return src.data+static_cast<std::ptrdiff_t>(y-region.source_y)*source_stride+
               static_cast<std::ptrdiff_t>(x-region.source_x)*4;
    };
    PlaneRenderReport report;report.invalid_plane=!warp;
    for(int y=0;y<dst.height;++y) {
        if(abort && abort(refcon)) throw RenderCancelled{};
        auto out=dst.data+static_cast<std::ptrdiff_t>(y)*output_stride;
        for(int x=0;x<dst.width;++x) {
            PlanePoint q{static_cast<double>(x+region.output_x),static_cast<double>(y+region.output_y)},p=q;
            if(warp) {
                PlanePoint surface{q.x*region.surface_units_x,q.y*region.surface_units_y};
                auto mapped=warp->sourceFor(surface);
                if(mapped.status==PlaneMapStatus::Mapped) {
                    // Keep identity bit-exact despite scale multiplication/division.
                    if(mapped.source->x!=surface.x || mapped.source->y!=surface.y) {
                        p={mapped.source->x/region.surface_units_x,mapped.source->y/region.surface_units_y};
                        if(!std::isfinite(p.x) || !std::isfinite(p.y)) {
                            p=q; ++report.invalid_projection_pixels;
                        }
                    }
                }
                else if(mapped.status==PlaneMapStatus::OutsidePlane) ++report.outside_pixels;
                else ++report.invalid_projection_pixels;
            }
            auto pixel=out+static_cast<std::ptrdiff_t>(x)*4;
            if(p.x==q.x && p.y==q.y) {
                std::memcpy(pixel,sourcePixel(x+region.output_x,y+region.output_y),4*sizeof(T));
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
                if constexpr(std::is_floating_point_v<T>) {
                    pixel[c]=value; // float alpha/HDR/negative values remain unclamped
                } else {
                    constexpr float maximum=std::is_same_v<T,std::uint16_t>?32768.f:255.f;
                    pixel[c]=static_cast<T>(std::clamp(value,0.f,maximum)+.5f);
                }
            }
        }
    }
    return report;
}
PlaneRenderReport renderPlaneRGBAfRegion(const ConstImageRGBAf& src,const ImageRGBAf& dst,
    const PlaneCanvasRegion& region,const PlaneWarp* warp,AbortFn abort,void* refcon) {
    return renderRegion(src,dst,src.row_stride_floats,dst.row_stride_floats,region,warp,abort,refcon);
}
// All strides passed to the shared sampler are element counts, never byte counts.
PlaneRenderReport renderPlaneRGBA8Region(const ConstImageRGBA8& src,const ImageRGBA8& dst,
    const PlaneCanvasRegion& region,const PlaneWarp* warp,AbortFn abort,void* refcon) {
    return renderRegion(src,dst,src.row_stride_values,dst.row_stride_values,region,warp,abort,refcon);
}
PlaneRenderReport renderPlaneRGBA16Region(const ConstImageRGBA16& src,const ImageRGBA16& dst,
    const PlaneCanvasRegion& region,const PlaneWarp* warp,AbortFn abort,void* refcon) {
    return renderRegion(src,dst,src.row_stride_values,dst.row_stride_values,region,warp,abort,refcon);
}
}
