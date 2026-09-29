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
PlaneMapResult PlaneWarp::sourceFor(PlanePoint destination) const {
    auto local=transform_.toLocal(destination);
    if(!local) return {PlaneMapStatus::InvalidProjection,std::nullopt};
    // Only absorb floating-point roundoff at the border, never a pixel-wide halo.
    constexpr double border=1e-10;
    if(local->x < -border || local->x > 1+border || local->y < -border || local->y > 1+border)
        return {PlaneMapStatus::OutsidePlane,std::nullopt};
    if(identity_) return {PlaneMapStatus::Mapped,destination};
    const float x=static_cast<float>(std::clamp(local->x,0.0,1.0));
    const float y=static_cast<float>(std::clamp(local->y,0.0,1.0));
    auto source=transform_.toSurface({inverseMapNormalized(x,columns_,easing_,easing_distance_),
                                     inverseMapNormalized(y,rows_,easing_,easing_distance_)});
    if(!source) return {PlaneMapStatus::InvalidProjection,std::nullopt};
    return {PlaneMapStatus::Mapped,source};
}
} // namespace elasticgrid
