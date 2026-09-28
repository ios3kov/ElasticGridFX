#include "core/WarpMath.h"
#include <algorithm>
#include <cmath>
#include <cstdint>
#include <stdexcept>

namespace elasticgrid {
namespace {
constexpr float kEpsilon = 1.0e-8f;
constexpr float kTangentEpsilon = 1.0e-4f;

float smoothstep(float t) {
    t = std::clamp(t, 0.0f, 1.0f);
    return t * t * (3.0f - 2.0f * t);
}

float clamp_tangent(float tangent, float current_slope) {
    const float limit = 3.0f * std::abs(current_slope) + kTangentEpsilon;
    if (std::abs(tangent) <= limit) return tangent;
    return std::copysign(limit, tangent);
}
} // namespace

float inverseMapNormalized(float output_u,
                           const std::vector<float>& destination_lines,
                           float easing,
                           float easing_distance) {
    if (destination_lines.size() < 2) return std::clamp(output_u, 0.0f, 1.0f);
    output_u = std::clamp(output_u, 0.0f, 1.0f);

    if (output_u <= destination_lines.front()) return 0.0f;
    if (output_u >= destination_lines.back()) return 1.0f;

    auto it = std::upper_bound(destination_lines.begin(), destination_lines.end(), output_u);
    std::size_t seg = 0;
    if (it == destination_lines.begin()) {
        seg = 0;
    } else if (it == destination_lines.end()) {
        seg = destination_lines.size() - 2;
    } else {
        seg = static_cast<std::size_t>(std::distance(destination_lines.begin(), it) - 1);
    }

    const std::size_t point_count = destination_lines.size();
    const float cells = static_cast<float>(point_count - 1);
    const float d0 = destination_lines[seg];
    const float d1 = destination_lines[seg + 1];
    const float destination_span = d1 - d0;
    const float t = destination_span > kEpsilon
        ? std::clamp((output_u - d0) / destination_span, 0.0f, 1.0f)
        : 0.0f;

    const float s0 = static_cast<float>(seg) / cells;
    const float s1 = static_cast<float>(seg + 1) / cells;
    const float linear = s0 + (s1 - s0) * t;

    easing = std::clamp(std::isfinite(easing) ? easing : 0.0f, 0.0f, 1.0f);
    easing_distance = std::clamp(
        std::isfinite(easing_distance) ? easing_distance : 0.25f, 0.0f, 1.0f);
    if (easing <= 0.0f || destination_span <= kEpsilon) return linear;

    const float current_slope = (s1 - s0) / destination_span;

    const std::size_t prev = seg > 0 ? seg - 1 : seg;
    const std::size_t next = (seg + 2 < point_count) ? seg + 2 : seg + 1;

    const float prev_destination_span = d1 - destination_lines[prev];
    float left_tangent = current_slope;
    if (std::abs(prev_destination_span) > kEpsilon) {
        const float prev_source = static_cast<float>(prev) / cells;
        left_tangent = (s1 - prev_source) / prev_destination_span;
    }

    const float next_destination_span = destination_lines[next] - d0;
    float right_tangent = current_slope;
    if (std::abs(next_destination_span) > kEpsilon) {
        const float next_source = static_cast<float>(next) / cells;
        right_tangent = (next_source - s0) / next_destination_span;
    }

    left_tangent = clamp_tangent(left_tangent, current_slope);
    right_tangent = clamp_tangent(right_tangent, current_slope);

    const float t2 = t * t;
    const float t3 = t2 * t;
    const float h00 = 2.0f * t3 - 3.0f * t2 + 1.0f;
    const float h10 = t3 - 2.0f * t2 + t;
    const float h01 = 3.0f * t2 - 2.0f * t3;
    const float h11 = t3 - t2;
    const float hermite =
        h00 * s0 +
        h10 * destination_span * left_tangent +
        h01 * s1 +
        h11 * destination_span * right_tangent;

    // Original GridWarp applies Hermite only near each segment boundary.
    // Easing Distance is the total normalized zone size, split equally
    // between the two ends of the segment.
    const float half_distance = easing_distance * 0.5f;
    float distance_weight = 0.0f;
    if (half_distance > 1.0e-6f) {
        const float nearest_edge = std::min(t, 1.0f - t);
        const float edge_amount = std::clamp(1.0f - nearest_edge / half_distance, 0.0f, 1.0f);
        distance_weight = smoothstep(edge_amount);
    }

    const float blend = easing * distance_weight;
    return linear + (hermite - linear) * blend;
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
        const float u = static_cast<float>(static_cast<std::int64_t>(output_origin) + i) / denom;
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
