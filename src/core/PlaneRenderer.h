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
    // Surface units per rendered pixel. Caller supplies the same coordinate basis
    // as plane corners (e.g. full-resolution pixels or PAR-corrected units).
    // Positive finite diagonal scale only; camera projection is NOT applied here.
    double surface_units_x = 1, surface_units_y = 1;
    EdgeMode edge = EdgeMode::Clamp;
    SampleQuality quality = SampleQuality::Bicubic;
};
// Stored rectangles may extend beyond the logical canvas, including negative
// origins. Output outside the canvas and missing source pixels are transparent
// black, never stretched checkout edges. Source storage outside the canvas is
// ignored. A 0x0 source is valid; strides are positive and views must not overlap.
// Report pixel counters refer only to output pixels inside the logical canvas.
PlaneRenderReport renderPlaneRGBAfRegion(const ConstImageRGBAf& src, const ImageRGBAf& dst,
    const PlaneCanvasRegion& region, const PlaneWarp* warp,
    AbortFn abort = nullptr, void* abort_refcon = nullptr);
// Integer sampling rounds once after both cubic axes; clamps to 255 / AE 32768.
// Pass-through pixels retain their original bits, just like the existing renderer.
PlaneRenderReport renderPlaneRGBA8Region(const ConstImageRGBA8& src, const ImageRGBA8& dst,
    const PlaneCanvasRegion& region, const PlaneWarp* warp,
    AbortFn abort = nullptr, void* abort_refcon = nullptr);
PlaneRenderReport renderPlaneRGBA16Region(const ConstImageRGBA16& src, const ImageRGBA16& dst,
    const PlaneCanvasRegion& region, const PlaneWarp* warp,
    AbortFn abort = nullptr, void* abort_refcon = nullptr);
// Dense, same-canvas float reference path. Integer pixel centers in plane surface
// coordinates. Final Catmull-Rom only, clamp at canvas edges. Nonoverlapping views.
// Null warp means invalid plane: exact pass-through with diagnostic flag.
PlaneRenderReport renderPlaneRGBAf(const ConstImageRGBAf& src, const ImageRGBAf& dst,
                                   const PlaneWarp* warp, AbortFn abort = nullptr,
                                   void* abort_refcon = nullptr);
}
