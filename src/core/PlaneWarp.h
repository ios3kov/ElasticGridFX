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
private:
    explicit PlaneWarp(PlaneTransform transform) : transform_(transform) {}
    PlaneTransform transform_;
    std::vector<float> columns_, rows_;
    float easing_ = 0, easing_distance_ = .25f;
    bool identity_ = false;
};
} // namespace elasticgrid
