#include "bridge/plane_ffi.h"
#include "core/PlaneRenderer.h"
#include <cmath>
#include <stdexcept>

namespace eg = elasticgrid;
static_assert(sizeof(EgPlaneFrame)==152);
static_assert(offsetof(EgPlaneFrame,abort_fn)==136);
static_assert(sizeof(EgPlaneImage)==24);
static_assert(sizeof(EgPlaneReport)==24);

namespace {
bool axisValid(const float* values,int count) {
    if(!values || count<2 || count>52 || values[0]!=0 || values[count-1]!=1) return false;
    for(int i=0;i<count;++i)
        if(!std::isfinite(values[i]) || (i && values[i]<=values[i-1])) return false;
    return true;
}
bool viewValid(const EgPlaneImage& image,int bytes,bool allowEmpty) {
    if(allowEmpty && image.width==0 && image.height==0) return true;
    return image.pixels && image.width>0 && image.height>0 && image.row_bytes>0 &&
        image.row_bytes>=static_cast<std::ptrdiff_t>(image.width)*4*bytes &&
        image.row_bytes%bytes==0 && reinterpret_cast<std::uintptr_t>(image.pixels)%bytes==0;
}
}
int eg_render_plane(const EgPlaneImage* source,const EgPlaneImage* output,
    std::int32_t depth,const EgPlaneFrame* f,EgPlaneReport* report) noexcept {
    if(!report) return 1;
    *report={};
    if(!source || !output || !f) return 1;
    const int bytes=depth==8?1:depth==16?2:depth==32?4:0;
    if(!bytes) return 2;
    if(!viewValid(*source,bytes,true) || !viewValid(*output,bytes,false) ||
       !axisValid(f->columns,f->column_count) || !axisValid(f->rows,f->row_count) ||
       !std::isfinite(f->easing) || f->easing<0 || f->easing>1 ||
       !std::isfinite(f->easing_distance) || f->easing_distance<0 || f->easing_distance>1) return 1;
    try {
        std::array<eg::PlanePoint,4> corners{};
        for(int i=0;i<4;++i) corners[i]={f->corners[i*2],f->corners[i*2+1]};
        auto transform=eg::PlaneTransform::fromCorners(corners);
        std::optional<eg::PlaneWarp> warp;
        if(transform) {
            warp=eg::PlaneWarp::prepare(*transform,{f->columns,f->columns+f->column_count},
                {f->rows,f->rows+f->row_count},f->easing,f->easing_distance);
            if(!warp) return 1;
        }
        const eg::PlaneCanvasRegion region{f->canvas_width,f->canvas_height,
            f->source_x,f->source_y,f->output_x,f->output_y,f->surface_units_x,f->surface_units_y};
        const auto* mapping=warp?&*warp:nullptr;
        const auto ss=source->row_bytes/bytes,ds=output->row_bytes/bytes;
        eg::PlaneRenderReport result;
        if(depth==8) result=eg::renderPlaneRGBA8Region(
            {static_cast<const std::uint8_t*>(source->pixels),source->width,source->height,ss},
            {static_cast<std::uint8_t*>(output->pixels),output->width,output->height,ds},
            region,mapping,f->abort_fn,f->abort_refcon);
        else if(depth==16) result=eg::renderPlaneRGBA16Region(
            {static_cast<const std::uint16_t*>(source->pixels),source->width,source->height,ss},
            {static_cast<std::uint16_t*>(output->pixels),output->width,output->height,ds},
            region,mapping,f->abort_fn,f->abort_refcon);
        else result=eg::renderPlaneRGBAfRegion(
            {static_cast<const float*>(source->pixels),source->width,source->height,ss},
            {static_cast<float*>(output->pixels),output->width,output->height,ds},
            region,mapping,f->abort_fn,f->abort_refcon);
        *report={result.invalid_plane?1:0,0,result.outside_pixels,result.invalid_projection_pixels};
        return 0;
    } catch(const eg::RenderCancelled&) { return 5; }
      catch(const std::invalid_argument&) { return 1; }
      catch(...) { return 3; }
}
