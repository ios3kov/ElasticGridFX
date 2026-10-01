#include "core/PlaneRenderer.h"
#include <algorithm>
#include <cmath>
#include <cstring>
#include <limits>
#include <stdexcept>
#include <type_traits>
#include <vector>

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
struct Taps { int index[4]; float w[4]; int count; };
struct AxisSample {
    PlaneMapStatus status;
    double source;
    Taps taps;
};
struct HorizontalRow {
    int source_y = -1;
    std::vector<float> pixels;
};
int resolve(std::int64_t index,int extent,EdgeMode edge) {
    if(extent==1) return 0;
    if(edge==EdgeMode::Clamp) return static_cast<int>(std::clamp(index,std::int64_t(0),std::int64_t(extent-1)));
    const std::int64_t period=edge==EdgeMode::Wrap?extent:2LL*(extent-1);
    auto value=index%period;
    if(value<0) value+=period;
    return static_cast<int>(value<extent?value:period-value);
}
Taps taps(double coordinate,int extent,EdgeMode edge,SampleQuality quality) {
    // Reduce before integer conversion, including coordinates far beyond int64.
    double p=coordinate;
    if(extent==1) p=0;
    else if(edge==EdgeMode::Clamp) p=std::clamp(p,0.0,static_cast<double>(extent-1));
    else p=std::fmod(p,edge==EdgeMode::Wrap?double(extent):2.0*(extent-1));
    auto base=static_cast<std::int64_t>(std::floor(p));
    float fraction=static_cast<float>(p-base),sum=0;
    Taps result{};
    if(quality==SampleQuality::Bilinear) {
        result.count=2;
        result.index[0]=resolve(base,extent,edge);result.index[1]=resolve(base+1,extent,edge);
        result.w[0]=1-fraction;result.w[1]=fraction;
        return result;
    }
    result.count=4;
    for(int k=0;k<4;++k) {
        result.index[k]=resolve(base+k-1,extent,edge);
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
    if((region.edge!=EdgeMode::Clamp && region.edge!=EdgeMode::Wrap && region.edge!=EdgeMode::Mirror) ||
       (region.quality!=SampleQuality::Bilinear && region.quality!=SampleQuality::Bicubic))
        throw std::invalid_argument("invalid plane sampling mode");
    if(!std::isfinite(region.surface_units_x) || !std::isfinite(region.surface_units_y) ||
       region.surface_units_x<=0 || region.surface_units_y<=0 ||
       !std::isfinite(region.surface_units_x*region.canvas_width) ||
       !std::isfinite(region.surface_units_y*region.canvas_height))
        throw std::invalid_argument("invalid plane raster scale");
    if(region.canvas_width<=0 || region.canvas_height<=0 ||
       src.width<0 || src.height<0 ||
       ((src.width==0)!=(src.height==0)))
        throw std::invalid_argument("invalid plane canvas region");
    const bool empty=src.width==0;
    auto se=empty?0:endAddress(src.data,src.width,src.height,source_stride);
    auto de=endAddress(dst.data,dst.width,dst.height,output_stride);
    if(!empty && reinterpret_cast<std::uintptr_t>(src.data)<de && reinterpret_cast<std::uintptr_t>(dst.data)<se)
        throw std::invalid_argument("overlapping plane image views");
    const T zero[4]{};
    auto sourcePixel=[&](int x,int y)->const T* {
        const auto sx=static_cast<std::int64_t>(x)-region.source_x;
        const auto sy=static_cast<std::int64_t>(y)-region.source_y;
        if(empty || x<0 || y<0 || x>=region.canvas_width || y>=region.canvas_height ||
           sx<0 || sy<0 || sx>=src.width || sy>=src.height) return zero;
        return src.data+static_cast<std::ptrdiff_t>(sy)*source_stride+
               static_cast<std::ptrdiff_t>(sx)*4;
    };
    // Cache only structurally separable transforms. Use the original sourceFor
    // and tap functions, preserving their precision and arithmetic order.
    // Non-unit raster scales retain the general path: its joint identity check
    // deliberately avoids a multiply/divide roundtrip on neutral pixels.
    if(abort && abort(refcon)) throw RenderCancelled{};
    auto anchor=warp && region.cache_axis_mapping &&
        region.surface_units_x==1 && region.surface_units_y==1
        ? warp->separableAnchor() : std::nullopt;
    std::vector<AxisSample> x_samples,y_samples;
    if(anchor) {
        x_samples.reserve(static_cast<std::size_t>(dst.width));
        y_samples.reserve(static_cast<std::size_t>(dst.height));
        for(int x=0;x<dst.width;++x) {
            const auto qx=static_cast<std::int64_t>(x)+region.output_x;
            auto mapped=warp->sourceFor({static_cast<double>(qx),anchor->y});
            const double coordinate=mapped.source?mapped.source->x:static_cast<double>(qx);
            x_samples.push_back({mapped.status,coordinate,
                taps(coordinate,region.canvas_width,region.edge,region.quality)});
        }
        for(int y=0;y<dst.height;++y) {
            const auto qy=static_cast<std::int64_t>(y)+region.output_y;
            auto mapped=warp->sourceFor({anchor->x,static_cast<double>(qy)});
            const double coordinate=mapped.source?mapped.source->y:static_cast<double>(qy);
            y_samples.push_back({mapped.status,coordinate,
                taps(coordinate,region.canvas_height,region.edge,region.quality)});
        }
        // Extremely scaled geometry may fail at an intermediate projection.
        // Retain the original combined validity/outside precedence in that case.
        const auto invalid=[](const AxisSample& sample) {
            return sample.status==PlaneMapStatus::InvalidProjection;
        };
        if(std::any_of(x_samples.begin(),x_samples.end(),invalid) ||
           std::any_of(y_samples.begin(),y_samples.end(),invalid)) anchor.reset();
    }
    std::array<HorizontalRow,4> row_cache;
    auto horizontalRow=[&](int source_y,const Taps& required) -> const float* {
        for(auto& row:row_cache) if(row.source_y==source_y) return row.pixels.data();
        auto available=std::find_if(row_cache.begin(),row_cache.end(),[&](const HorizontalRow& row) {
            return std::find(required.index,required.index+required.count,row.source_y)==required.index+required.count;
        });
        // At most four distinct source rows are required; a cache miss must have
        // a victim outside this set. Do not invalidate any previously returned row.
        if(available==row_cache.end()) throw std::logic_error("plane row cache invariant");
        auto& row=*available;
        row.pixels.resize(static_cast<std::size_t>(dst.width)*4);
        for(int x=0;x<dst.width;++x) {
            const auto& xs=x_samples[static_cast<std::size_t>(x)].taps;
            float* pixel=row.pixels.data()+static_cast<std::ptrdiff_t>(x)*4;
            pixel[0]=pixel[1]=pixel[2]=pixel[3]=0;
            for(int i=0;i<xs.count;++i) {
                const T* sample=sourcePixel(xs.index[i],source_y);
                for(int c=0;c<4;++c) pixel[c]+=sample[c]*xs.w[i];
            }
        }
        row.source_y=source_y;
        return row.pixels.data();
    };
    PlaneRenderReport report;report.invalid_plane=!warp;
    for(int y=0;y<dst.height;++y) {
        if(y>0 && abort && abort(refcon)) throw RenderCancelled{};
        auto out=dst.data+static_cast<std::ptrdiff_t>(y)*output_stride;
        const float* cached_rows[4]{};
        bool rows_ready=false;
        for(int x=0;x<dst.width;++x) {
            const auto qx=static_cast<std::int64_t>(x)+region.output_x;
            const auto qy=static_cast<std::int64_t>(y)+region.output_y;
            auto pixel=out+static_cast<std::ptrdiff_t>(x)*4;
            // Expanded host worlds are storage, not a larger deformation canvas.
            // Never stretch a logical edge into these pixels, even for bad planes.
            if(qx<0 || qy<0 || qx>=region.canvas_width || qy>=region.canvas_height) {
                std::memcpy(pixel,zero,4*sizeof(T));
                continue;
            }
            PlanePoint q{static_cast<double>(qx),static_cast<double>(qy)},p=q;
            if(anchor) {
                const auto& xs=x_samples[static_cast<std::size_t>(x)];
                const auto& ys=y_samples[static_cast<std::size_t>(y)];
                if(xs.status==PlaneMapStatus::OutsidePlane || ys.status==PlaneMapStatus::OutsidePlane)
                    ++report.outside_pixels;
                else if(xs.status==PlaneMapStatus::InvalidProjection || ys.status==PlaneMapStatus::InvalidProjection)
                    ++report.invalid_projection_pixels;
                else p={xs.source,ys.source};
            } else if(warp) {
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
                else if(mapped.status==PlaneMapStatus::OutsidePlane) {
                    ++report.outside_pixels;
                    if(warp->projectsSource()) {
                        std::memcpy(pixel,zero,4*sizeof(T));
                        continue;
                    }
                }
                else {
                    ++report.invalid_projection_pixels;
                    if(warp->projectsSource()) {
                        std::memcpy(pixel,zero,4*sizeof(T));
                        continue;
                    }
                }
            }
            if(p.x==q.x && p.y==q.y) {
                std::memcpy(pixel,sourcePixel(static_cast<int>(qx),static_cast<int>(qy)),4*sizeof(T));
                continue;
            }
            const auto xs=anchor?x_samples[static_cast<std::size_t>(x)].taps:
                taps(p.x,region.canvas_width,region.edge,region.quality);
            const auto ys=anchor?y_samples[static_cast<std::size_t>(y)].taps:
                taps(p.y,region.canvas_height,region.edge,region.quality);
            const T* samples[4][4]{};
            if(anchor) {
                if(!rows_ready) {
                    for(int j=0;j<ys.count;++j) cached_rows[j]=horizontalRow(ys.index[j],ys);
                    rows_ready=true;
                }
            } else for(int j=0;j<ys.count;++j) for(int i=0;i<xs.count;++i)
                samples[j][i]=sourcePixel(xs.index[i],ys.index[j]);
            // Interleave independent channels so the compiler can vectorize
            // RGBA without reassociating any one channel's sums or tap order.
            float values[4]{};
            for(int j=0;j<ys.count;++j) {
                float horizontal[4]{};
                if(anchor) std::memcpy(horizontal,cached_rows[j]+static_cast<std::ptrdiff_t>(x)*4,sizeof(horizontal));
                else for(int i=0;i<xs.count;++i) for(int c=0;c<4;++c)
                    horizontal[c]+=samples[j][i][c]*xs.w[i];
                for(int c=0;c<4;++c) values[c]+=horizontal[c]*ys.w[j];
            }
            for(int c=0;c<4;++c) {
                const float value=values[c];
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
