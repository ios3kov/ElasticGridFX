#pragma once
#include "bridge/elasticgrid_ffi.h"

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
