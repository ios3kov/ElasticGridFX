#include "core/PlaneWarp.h"
#include "core/WarpMath.h"
#include <algorithm>
#include <cmath>
#include <utility>

namespace elasticgrid {
namespace {
bool validAxis(const std::vector<float>& axis) {
    if(axis.size()<2 || axis.front()!=0 || axis.back()!=1) return false;
    for(std::size_t i=0;i<axis.size();++i)
        if(!std::isfinite(axis[i]) || (i && axis[i]<=axis[i-1])) return false;
    return true;
}
bool uniform(const std::vector<float>& axis) {
    for(std::size_t i=0;i<axis.size();++i)
        if(axis[i]!=static_cast<float>(i)/static_cast<float>(axis.size()-1)) return false;
    return true;
}
double extendedAxis(double p,const std::vector<float>& axis,float easing,float distance) {
    const double cell=1.0/static_cast<double>(axis.size()-1);
    // Endpoint Hermite tangents equal the adjacent cell slope as well.
    if(p<0) return p*cell/axis[1];
    if(p>1) return 1+(p-1)*cell/(1-axis[axis.size()-2]);
    return inverseMapNormalized(static_cast<float>(p),axis,easing,distance);
}
}
std::optional<PlaneWarp> PlaneWarp::prepare(PlaneTransform transform,
    std::vector<float> columns,std::vector<float> rows,float easing,float easing_distance) {
    if(!validAxis(columns) || !validAxis(rows) || !std::isfinite(easing) ||
       !std::isfinite(easing_distance) || easing<0 || easing>1 ||
       easing_distance<0 || easing_distance>1) return std::nullopt;
    PlaneWarp warp(transform);
    warp.identity_=uniform(columns) && uniform(rows);
    warp.columns_=std::move(columns); warp.rows_=std::move(rows);
    warp.easing_=easing; warp.easing_distance_=easing_distance;
    return warp;
}
std::optional<PlaneWarp> PlaneWarp::prepareLayer(PlaneTransform transform,
    std::vector<float> columns,std::vector<float> rows,float easing,float easing_distance) {
    auto warp=prepare(transform,std::move(columns),std::move(rows),easing,easing_distance);
    if(warp) warp->extend_layer_=true;
    return warp;
}
std::optional<PlaneWarp> PlaneWarp::prepareProjected(PlaneTransform destination,
    PlanePoint source_extent,std::vector<float> columns,std::vector<float> rows,
    float easing,float easing_distance) {
    if(!std::isfinite(source_extent.x) || !std::isfinite(source_extent.y) ||
       source_extent.x<0 || source_extent.y<0) return std::nullopt;
    auto warp=prepare(destination,std::move(columns),std::move(rows),easing,easing_distance);
    if(warp) warp->source_extent_=source_extent;
    return warp;
}
std::optional<PlaneWarp> PlaneWarp::prepareBetween(PlaneTransform source,
    PlaneTransform destination,std::vector<float> columns,std::vector<float> rows,
    float easing,float easing_distance) {
    auto warp=prepare(destination,std::move(columns),std::move(rows),easing,easing_distance);
    if(warp) warp->source_transform_=source;
    return warp;
}
PlaneMapResult PlaneWarp::sourceFor(PlanePoint destination) const {
    auto local=transform_.toLocal(destination);
    if(!local) return {PlaneMapStatus::InvalidProjection,std::nullopt};
    // Only absorb floating-point roundoff at the border, never a pixel-wide halo.
    constexpr double border=1e-10;
    if(!extend_layer_ && (local->x < -border || local->x > 1+border || local->y < -border || local->y > 1+border))
        return {PlaneMapStatus::OutsidePlane,std::nullopt};
    if(identity_ && !projectsSource()) return {PlaneMapStatus::Mapped,destination};
    if(extend_layer_) {
        auto source=transform_.toSurface({extendedAxis(local->x,columns_,easing_,easing_distance_),
                                         extendedAxis(local->y,rows_,easing_,easing_distance_)});
        return source ? PlaneMapResult{PlaneMapStatus::Mapped,source}
                      : PlaneMapResult{PlaneMapStatus::InvalidProjection,std::nullopt};
    }
    if(source_extent_ && identity_) return {PlaneMapStatus::Mapped,PlanePoint{
        std::clamp(local->x,0.0,1.0)*source_extent_->x,
        std::clamp(local->y,0.0,1.0)*source_extent_->y}};
    if(source_transform_ && identity_) {
        auto source=source_transform_->toSurface({std::clamp(local->x,0.0,1.0),
                                                 std::clamp(local->y,0.0,1.0)});
        return source ? PlaneMapResult{PlaneMapStatus::Mapped,source}
                      : PlaneMapResult{PlaneMapStatus::InvalidProjection,std::nullopt};
    }
    const float x=static_cast<float>(std::clamp(local->x,0.0,1.0));
    const float y=static_cast<float>(std::clamp(local->y,0.0,1.0));
    PlanePoint normalized{inverseMapNormalized(x,columns_,easing_,easing_distance_),
                          inverseMapNormalized(y,rows_,easing_,easing_distance_)};
    if(source_extent_) return {PlaneMapStatus::Mapped,PlanePoint{
        normalized.x*source_extent_->x,normalized.y*source_extent_->y}};
    auto source=(source_transform_ ? *source_transform_ : transform_).toSurface(normalized);
    if(!source) return {PlaneMapStatus::InvalidProjection,std::nullopt};
    return {PlaneMapStatus::Mapped,source};
}
} // namespace elasticgrid
