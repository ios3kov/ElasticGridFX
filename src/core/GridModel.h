#pragma once
#include <cstddef>
#include <cstdint>
#include <span>
#include <vector>

namespace elasticgrid {

enum class FalloffProfile : std::uint8_t { Smoothstep, Gaussian, Linear };
enum class WaveAxis : std::uint8_t { Both, ColumnsOnly, RowsOnly };
enum class EdgeMode : std::uint8_t { Clamp, Wrap, Mirror };
enum class SampleQuality : std::uint8_t { Bilinear, Bicubic };

struct ElasticSettings {
    float strength = 1.0f;      // 0..2
    float radius_lines = 3.0f;  // influence radius in guide-line units
    float min_spacing = 0.005f; // normalized axis spacing
    FalloffProfile falloff = FalloffProfile::Smoothstep;
};

struct WaveSettings {
    bool enabled = false;
    float amplitude = 0.0f;     // 0..1; original scales to 40% of uniform cell spacing
    float frequency = 1.0f;
    float phase_degrees = 0.0f;
    float speed_cycles_per_second = 0.0f;
    WaveAxis axis = WaveAxis::Both;
};

class AxisGrid {
public:
    explicit AxisGrid(std::size_t cells = 4);

    void reset(std::size_t cells);
    [[nodiscard]] std::size_t cells() const noexcept;
    [[nodiscard]] std::size_t lineCount() const noexcept;
    [[nodiscard]] const std::vector<float>& lines() const noexcept;
    [[nodiscard]] const std::vector<std::uint8_t>& pins() const noexcept;

    bool setPinned(std::size_t line, bool pinned);
    bool setLinePosition(std::size_t line, float position, float min_spacing);
    bool dragElastic(std::size_t line, float target, const ElasticSettings& settings);
    bool setState(std::span<const float> lines,
                  std::span<const std::uint8_t> pins,
                  float min_spacing = 0.0f);

    [[nodiscard]] std::vector<float> evaluated(const WaveSettings& wave,
                                               float time_seconds,
                                               float min_spacing,
                                               bool apply_wave) const;
    void evaluatedInto(std::vector<float>& out,
                       const WaveSettings& wave,
                       float time_seconds,
                       float min_spacing,
                       bool apply_wave) const;

    static void enforceMonotonic(std::vector<float>& values,
                                 float min_spacing,
                                 const std::vector<std::uint8_t>* pins = nullptr);

private:
    std::vector<float> lines_;
    std::vector<std::uint8_t> pins_;
};

struct GridState {
    AxisGrid columns{4};
    AxisGrid rows{4};
};

} // namespace elasticgrid
