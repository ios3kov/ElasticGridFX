#pragma once
#include <algorithm>
#include <bit>
#include <cstdint>
#include <cmath>
#include <cstddef>
#include <span>

namespace elasticgrid {
// Fixed material-coordinate refinement, independent of displayed guide count.
// Empty means the exact identity, preserving the historical sampling path.
inline constexpr std::size_t kDetailSamples = 257;
inline bool validDetail(std::span<const float> values) noexcept {
    if (values.empty()) return true;
    if (values.size() != kDetailSamples || values.front() != 0.0f || values.back() != 1.0f) return false;
    for (std::size_t i = 0; i < values.size(); ++i)
        if (!std::isfinite(values[i]) || (i && values[i] <= values[i-1])) return false;
    return true;
}
inline bool identityDetail(std::span<const float> values) noexcept {
    if (values.empty()) return true;
    for (std::size_t i = 0; i < values.size(); ++i)
        if (std::bit_cast<std::uint32_t>(values[i]) != std::bit_cast<std::uint32_t>(static_cast<float>(i) / static_cast<float>(values.size()-1))) return false;
    return true;
}
// Caller has validated an immutable owned view. Extrapolation preserves layer
// perimeter semantics; bounded Four Corners still handles its own exterior.
inline double inverseDetail(double value, std::span<const float> map) noexcept {
    if (map.empty()) return value;
    const double cells = static_cast<double>(map.size()-1);
    if (value <= 0.0) return value / (cells * static_cast<double>(map[1]));
    if (value >= 1.0) return 1.0 + (value-1.0) / (cells * (1.0-static_cast<double>(map[map.size()-2])));
    auto right = std::upper_bound(map.begin(), map.end(), static_cast<float>(value));
    const std::size_t index = std::clamp(static_cast<std::size_t>(right-map.begin()), std::size_t{1}, map.size()-1);
    const double fraction = (value-static_cast<double>(map[index-1])) / (static_cast<double>(map[index])-static_cast<double>(map[index-1]));
    return (static_cast<double>(index-1)+fraction)/cells;
}
} // namespace elasticgrid
