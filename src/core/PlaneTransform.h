#pragma once
#include <array>
#include <optional>

namespace elasticgrid {
struct PlanePoint { double x, y; };

// Geometry only: no AE view/camera, pixel sampling, or outside-plane policy.
// Corner order corresponds to local (0,0), (1,0), (1,1), (0,1).
class PlaneTransform {
public:
    static std::optional<PlaneTransform> fromCorners(const std::array<PlanePoint,4>& corners);
    std::optional<PlanePoint> toSurface(PlanePoint local) const;
    std::optional<PlanePoint> toLocal(PlanePoint surface) const;
    // Exact structural test, never an epsilon approximation of perspective.
    bool axisAligned() const;
private:
    PlaneTransform() = default;
    std::array<double,9> forward_{}, inverse_{};
    PlanePoint origin_{};
    double scale_ = 1;
};
} // namespace elasticgrid
