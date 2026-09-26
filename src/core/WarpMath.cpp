#include "core/WarpMath.h"
#include <algorithm>
#include <cmath>
#include <stdexcept>

namespace elasticgrid {
namespace {
float smoothstep(float t) { return t * t * (3.0f - 2.0f * t); }

float applyEasing(float t, float amount, float distance) {
    if (!std::isfinite(t)) t = 0.0f;
    if (!std::isfinite(amount)) amount = 0.0f;
    if (!std::isfinite(distance)) distance = 0.25f;
    amount = std::clamp(amount, 0.0f, 1.0f);
    distance = std::clamp(distance, 1e-4f, 0.5f);
    if (amount <= 0.0f) return t;

    // Local edge softening: the middle stays almost linear, while the motion near
    // guide lines gets progressively smoother. This is stable and invertible.
    float eased = t;
    if (t < distance) {
        const float x = t / distance;
        eased = distance * smoothstep(x);
    } else if (t > 1.0f - distance) {
        const float x = (t - (1.0f - distance)) / distance;
        eased = (1.0f - distance) + distance * smoothstep(x);
    }
    return std::lerp(t, eased, amount);
}
} // namespace

float inverseMapNormalized(float output_u,
                           const std::vector<float>& destination_lines,
                           float easing,
                           float easing_distance) {
    if (destination_lines.size() < 2) return std::clamp(output_u, 0.0f, 1.0f);
    output_u = std::clamp(output_u, 0.0f, 1.0f);

    auto it = std::upper_bound(destination_lines.begin(), destination_lines.end(), output_u);
    std::size_t seg = 0;
    if (it == destination_lines.begin()) seg = 0;
    else if (it == destination_lines.end()) seg = destination_lines.size() - 2;
    else seg = static_cast<std::size_t>(std::distance(destination_lines.begin(), it) - 1);

    const float a = destination_lines[seg];
    const float b = destination_lines[seg + 1];
    const float denom = std::max(1e-8f, b - a);
    float t = (output_u - a) / denom;
    t = applyEasing(std::clamp(t, 0.0f, 1.0f), easing, easing_distance);

    const float cells = static_cast<float>(destination_lines.size() - 1);
    return (static_cast<float>(seg) + t) / cells;
}

void buildInverseLUTRangeInto(WarpAxisLUT& lut,
                              const std::vector<float>& destination_lines,
                              int output_extent,
                              int canvas_extent,
                              int output_origin,
                              float easing,
                              float easing_distance) {
    if (output_extent <= 0) throw std::invalid_argument("output_extent must be > 0");
    if (canvas_extent <= 0) throw std::invalid_argument("canvas_extent must be > 0");
    lut.source_u.resize(static_cast<std::size_t>(output_extent));
    const float denom = static_cast<float>(std::max(1, canvas_extent - 1));
    for (int i = 0; i < output_extent; ++i) {
        const float u = static_cast<float>(output_origin + i) / denom;
        lut.source_u[static_cast<std::size_t>(i)] =
            inverseMapNormalized(u, destination_lines, easing, easing_distance);
    }
}

WarpAxisLUT buildInverseLUTRange(const std::vector<float>& destination_lines,
                                 int output_extent,
                                 int canvas_extent,
                                 int output_origin,
                                 float easing,
                                 float easing_distance) {
    WarpAxisLUT lut;
    buildInverseLUTRangeInto(lut, destination_lines, output_extent, canvas_extent,
                             output_origin, easing, easing_distance);
    return lut;
}

WarpAxisLUT buildInverseLUT(const std::vector<float>& destination_lines,
                            int output_extent,
                            float easing,
                            float easing_distance) {
    return buildInverseLUTRange(destination_lines, output_extent, output_extent, 0,
                                easing, easing_distance);
}

} // namespace elasticgrid
