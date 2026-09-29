#pragma once
#include "core/CpuRenderer.h"
#include "core/PlaneWarp.h"

namespace elasticgrid {
struct PlaneRenderReport {
    bool invalid_plane = false;
    std::size_t outside_pixels = 0, invalid_projection_pixels = 0;
};
struct PlaneCanvasRegion {
    int canvas_width, canvas_height;
    int source_x = 0, source_y = 0, output_x = 0, output_y = 0;
};
// Stored rectangles must lie inside the logical canvas. Missing source pixels
// are transparent black, never a stretched checkout edge. A 0x0 source is valid.
PlaneRenderReport renderPlaneRGBAfRegion(const ConstImageRGBAf& src, const ImageRGBAf& dst,
    const PlaneCanvasRegion& region, const PlaneWarp* warp,
    AbortFn abort = nullptr, void* abort_refcon = nullptr);
// Dense, same-canvas float reference path. Integer pixel centers in plane surface
// coordinates. Final Catmull-Rom only, clamp at canvas edges. Nonoverlapping views.
// Null warp means invalid plane: exact pass-through with diagnostic flag.
PlaneRenderReport renderPlaneRGBAf(const ConstImageRGBAf& src, const ImageRGBAf& dst,
                                   const PlaneWarp* warp, AbortFn abort = nullptr,
                                   void* abort_refcon = nullptr);
}
