#pragma once
#include "core/PlaneTransform.h"
#include <vector>

namespace elasticgrid {
enum class PlaneMapStatus { Mapped, OutsidePlane, InvalidProjection };
struct PlaneMapResult {
    PlaneMapStatus status;
    std::optional<PlanePoint> source; // present only for Mapped
};

// Immutable evaluated grid snapshot. Coordinates are composed before sampling:
// source = H(local inverse grid(H^-1(destination))). No intermediate image.
class PlaneWarp {
public:
    static std::optional<PlaneWarp> prepare(PlaneTransform transform,
        std::vector<float> columns, std::vector<float> rows,
        float easing = 0, float easing_distance = .25f);
    PlaneMapResult sourceFor(PlanePoint destination) const;
    // Project the original rectangular image into the destination plane.
    // Extent is the source last-pixel center in surface units (not buffer size).
    static std::optional<PlaneWarp> prepareProjected(PlaneTransform destination,
        PlanePoint source_extent, std::vector<float> columns, std::vector<float> rows,
        float easing = 0, float easing_distance = .25f);
    // Already-rasterized source: Hsource(grid^-1(Hdestination^-1(pixel))).
    // Source and destination are independent; no intermediate resampling.
    static std::optional<PlaneWarp> prepareBetween(PlaneTransform source,
        PlaneTransform destination, std::vector<float> columns, std::vector<float> rows,
        float easing = 0, float easing_distance = .25f);
    bool projectsSource() const { return source_extent_.has_value() || source_transform_.has_value(); }
private:
    explicit PlaneWarp(PlaneTransform transform) : transform_(transform) {}
    PlaneTransform transform_;
    std::vector<float> columns_, rows_;
    float easing_ = 0, easing_distance_ = .25f;
    bool identity_ = false;
    std::optional<PlanePoint> source_extent_;
    std::optional<PlaneTransform> source_transform_;
};
} // namespace elasticgrid
