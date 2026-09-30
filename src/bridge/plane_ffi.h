#pragma once
#include "bridge/elasticgrid_ffi.h"

// UI-owned immutable geometry. Create once per event, destroy on every exit.
// No host calls, no mutable shared state. Null creation means invalid corners.
struct EgPlaneGeometry;
extern "C" EgPlaneGeometry* eg_plane_geometry_create(const double* corners) noexcept;
extern "C" void eg_plane_geometry_destroy(EgPlaneGeometry* geometry) noexcept;
// inverse=0 local->surface, inverse=1 surface->local. No output on failure.
extern "C" int eg_plane_geometry_map(const EgPlaneGeometry* geometry, int inverse,
    double x, double y, double* output_xy) noexcept;

// Separate additive ABI: existing EgRenderParams and project IDs are untouched.
// Host supplies an immutable, already evaluated (including wave) guide snapshot.
// Corners are TL,TR,BR,BL in surface coordinates; row bytes are positive bytes.
struct EgPlaneFrame {
    double corners[8];
    const float* columns;
    const float* rows;
    std::int32_t column_count, row_count;
    double surface_units_x, surface_units_y;
    std::int32_t canvas_width, canvas_height;
    std::int32_t source_x, source_y, output_x, output_y;
    float easing, easing_distance;
    EgAbortFn abort_fn;
    void* abort_refcon;
};
struct EgPlaneImage {
    void* pixels;
    std::ptrdiff_t row_bytes;
    std::int32_t width, height;
};
struct EgPlaneReport {
    std::int32_t invalid_plane;
    std::int32_t reserved;
    std::uint64_t outside_pixels, invalid_projection_pixels;
};

// Final Catmull-Rom / logical-canvas clamp only; no silent substitution for other
// quality/edge modes. Does not acquire host suites or retain pointers after return.
// Invalid corners: exact pass-through + invalid_plane=1. Invalid grid/view: error.
// Return codes match existing bridge: 0 success,1 bad input,2 depth,3 failure,5 abort.
// Any nonzero code invalidates the entire output, including a partial aborted frame.
// Report is reset on entry and published only on success; nonnull required.
extern "C" int eg_render_plane(const EgPlaneImage* source, const EgPlaneImage* output,
    std::int32_t bit_depth, const EgPlaneFrame* frame, EgPlaneReport* report) noexcept;

// Additive entry point, same frame layout. quality: 0 Draft Bilinear, 1 Final
// Catmull-Rom; edge: 0 Clamp, 1 Wrap, 2 Mirror. Unknown values return bad input.
extern "C" int eg_render_plane_sampled(const EgPlaneImage* source, const EgPlaneImage* output,
    std::int32_t bit_depth, const EgPlaneFrame* frame, EgPlaneReport* report,
    std::int32_t quality, std::int32_t edge) noexcept;

// Explicit source endpoint in surface coordinates, before raster rounding.
// AE supplies (full layer dimension - 1) * downsample. The old entry points
// retain raster endpoint semantics for existing callers; frame ABI is unchanged.
extern "C" int eg_render_plane_projected(const EgPlaneImage* source, const EgPlaneImage* output,
    std::int32_t bit_depth, const EgPlaneFrame* frame, EgPlaneReport* report,
    std::int32_t quality, std::int32_t edge, double source_extent_x, double source_extent_y) noexcept;

// Already projected input. source_corners contains eight surface coordinates,
// TL/TR/BR/BL. Both quads must be valid; bad projections return 1 without writes.
// Does not retain pointers or call host APIs. Existing frame ABI is unchanged.
extern "C" int eg_render_plane_between(const EgPlaneImage* source, const EgPlaneImage* output,
    std::int32_t bit_depth, const EgPlaneFrame* frame, EgPlaneReport* report,
    std::int32_t quality, std::int32_t edge, const double* source_corners) noexcept;

// Perspective deformation region, not image projection. Neutral guides and
// pixels outside the quad preserve input exactly. Same frame ABI and sampler.
extern "C" int eg_render_plane_region(const EgPlaneImage* source, const EgPlaneImage* output,
    std::int32_t bit_depth, const EgPlaneFrame* frame, EgPlaneReport* report,
    std::int32_t quality, std::int32_t edge) noexcept;

// Automatic native layer plane: deform coverage beyond fractional vector bounds.
// Same ABI/sampler; unlike Four Corners, no unchanged outside-quad fringe.
extern "C" int eg_render_plane_layer(const EgPlaneImage* source, const EgPlaneImage* output,
    std::int32_t bit_depth, const EgPlaneFrame* frame, EgPlaneReport* report,
    std::int32_t quality, std::int32_t edge) noexcept;

extern "C" int eg_render_plane_detail(const EgPlaneImage* source,const EgPlaneImage* output,
    std::int32_t depth,const EgPlaneFrame* frame,EgPlaneReport* report,
    std::int32_t quality,std::int32_t edge,const EgDetailMaps* detail,std::int32_t layer) noexcept;
