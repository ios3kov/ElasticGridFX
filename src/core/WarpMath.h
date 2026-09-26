#pragma once
#include <vector>

namespace elasticgrid {

struct WarpAxisLUT {
    std::vector<float> source_u;
};

// Build inverse output->source mapping. This is the hot data used by renderers.
WarpAxisLUT buildInverseLUT(const std::vector<float>& destination_lines,
                            int output_extent,
                            float easing = 0.0f,
                            float easing_distance = 0.25f);

// Build an inverse LUT for a sub-rectangle of a larger layer/canvas.
// output_origin is the first output pixel in layer coordinates; canvas_extent
// is the full rendered layer extent at the current downsample factor.
WarpAxisLUT buildInverseLUTRange(const std::vector<float>& destination_lines,
                                 int output_extent,
                                 int canvas_extent,
                                 int output_origin,
                                 float easing = 0.0f,
                                 float easing_distance = 0.25f);

void buildInverseLUTRangeInto(WarpAxisLUT& lut,
                              const std::vector<float>& destination_lines,
                              int output_extent,
                              int canvas_extent,
                              int output_origin,
                              float easing = 0.0f,
                              float easing_distance = 0.25f);

float inverseMapNormalized(float output_u,
                           const std::vector<float>& destination_lines,
                           float easing = 0.0f,
                           float easing_distance = 0.25f);

} // namespace elasticgrid
