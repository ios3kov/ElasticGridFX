#include "bridge/plane_ffi.h"
#include "core/PlaneRenderer.h"
#include <cmath>
#include <stdexcept>
#include <new>

namespace eg = elasticgrid;
static_assert(sizeof(EgPlaneFrame)==152);
static_assert(offsetof(EgPlaneFrame,abort_fn)==136);
static_assert(sizeof(EgPlaneImage)==24);
static_assert(sizeof(EgPlaneReport)==24);

struct EgPlaneGeometry { eg::PlaneTransform transform; };
EgPlaneGeometry* eg_plane_geometry_create(const double* values) noexcept {
    if(!values) return nullptr;
    std::array<eg::PlanePoint,4> corners{};
    for(int i=0;i<4;++i) corners[i]={values[2*i],values[2*i+1]};
    auto transform=eg::PlaneTransform::fromCorners(corners);
    return transform?new(std::nothrow) EgPlaneGeometry{*transform}:nullptr;
}
void eg_plane_geometry_destroy(EgPlaneGeometry* geometry) noexcept {delete geometry;}
int eg_plane_geometry_map(const EgPlaneGeometry* geometry,int inverse,
    double x,double y,double* output) noexcept {
    if(!geometry || !output || (inverse!=0 && inverse!=1)) return 1;
    const auto point=inverse?geometry->transform.toLocal({x,y}):geometry->transform.toSurface({x,y});
    if(!point) return 1;
    output[0]=point->x;output[1]=point->y;
    return 0;
}

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
    return eg_render_plane_sampled(source,output,depth,f,report,1,0);
}
int eg_render_plane_sampled(const EgPlaneImage* source,const EgPlaneImage* output,
    std::int32_t depth,const EgPlaneFrame* f,EgPlaneReport* report,
    std::int32_t quality,std::int32_t edge) noexcept {
    return eg_render_plane_projected(source,output,depth,f,report,quality,edge,
        f?(static_cast<double>(f->canvas_width)-1)*f->surface_units_x:0,
        f?(static_cast<double>(f->canvas_height)-1)*f->surface_units_y:0);
}
static int renderPlane(const EgPlaneImage* source,const EgPlaneImage* output,
    std::int32_t depth,const EgPlaneFrame* f,EgPlaneReport* report,
    std::int32_t quality,std::int32_t edge,double source_extent_x,double source_extent_y,
    const double* source_corners,bool regional=false) noexcept {
    if(!report) return 1;
    *report={};
    if(!source || !output || !f || quality<0 || quality>1 || edge<0 || edge>2 ||
       !std::isfinite(source_extent_x) || !std::isfinite(source_extent_y) ||
       source_extent_x<0 || source_extent_y<0) return 1;
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
        if(regional && transform) {
            warp=eg::PlaneWarp::prepare(*transform,
                {f->columns,f->columns+f->column_count},
                {f->rows,f->rows+f->row_count},f->easing,f->easing_distance);
            if(!warp) return 1;
        } else if(source_corners) {
            for(int i=0;i<4;++i) corners[i]={source_corners[2*i],source_corners[2*i+1]};
            auto source_transform=eg::PlaneTransform::fromCorners(corners);
            if(!transform || !source_transform) return 1;
            warp=eg::PlaneWarp::prepareBetween(*source_transform,*transform,
                {f->columns,f->columns+f->column_count},
                {f->rows,f->rows+f->row_count},f->easing,f->easing_distance);
            if(!warp) return 1;
        } else if(transform) {
            warp=eg::PlaneWarp::prepareProjected(*transform,
                {source_extent_x,source_extent_y},
                {f->columns,f->columns+f->column_count},
                {f->rows,f->rows+f->row_count},f->easing,f->easing_distance);
            if(!warp) return 1;
        }
        const eg::PlaneCanvasRegion region{f->canvas_width,f->canvas_height,
            f->source_x,f->source_y,f->output_x,f->output_y,f->surface_units_x,f->surface_units_y,
            static_cast<eg::EdgeMode>(edge),static_cast<eg::SampleQuality>(quality)};
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

int eg_render_plane_projected(const EgPlaneImage* source,const EgPlaneImage* output,
    std::int32_t depth,const EgPlaneFrame* f,EgPlaneReport* report,
    std::int32_t quality,std::int32_t edge,double source_extent_x,double source_extent_y) noexcept {
    return renderPlane(source,output,depth,f,report,quality,edge,source_extent_x,source_extent_y,nullptr);
}
int eg_render_plane_between(const EgPlaneImage* source,const EgPlaneImage* output,
    std::int32_t depth,const EgPlaneFrame* f,EgPlaneReport* report,
    std::int32_t quality,std::int32_t edge,const double* source_corners) noexcept {
    if(!source_corners) {if(report) *report={};return 1;}
    return renderPlane(source,output,depth,f,report,quality,edge,0,0,source_corners);
}
int eg_render_plane_region(const EgPlaneImage* source,const EgPlaneImage* output,
    std::int32_t depth,const EgPlaneFrame* f,EgPlaneReport* report,
    std::int32_t quality,std::int32_t edge) noexcept {
    return renderPlane(source,output,depth,f,report,quality,edge,0,0,nullptr,true);
}
