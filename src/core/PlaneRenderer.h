#pragma once
#include "core/CpuRenderer.h"
#include "core/PlaneWarp.h"

namespace elasticgrid {
struct PlaneRenderReport {
    bool invalid_plane = false;
    std::size_t outside_pixels = 0, invalid_projection_pixels = 0;
};
// Dense, same-canvas float reference path. Integer pixel centers in plane surface
// coordinates. Final Catmull-Rom only, clamp at canvas edges. Nonoverlapping views.
// Null warp means invalid plane: exact pass-through with diagnostic flag.
PlaneRenderReport renderPlaneRGBAf(const ConstImageRGBAf& src, const ImageRGBAf& dst,
                                   const PlaneWarp* warp, AbortFn abort = nullptr,
                                   void* abort_refcon = nullptr);
}
