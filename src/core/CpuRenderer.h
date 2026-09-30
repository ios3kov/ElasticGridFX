#pragma once
#include "core/GridModel.h"
#include "core/WarpMath.h"
#include <array>
#include <cstddef>
#include <cstdint>
#include <exception>
#include <vector>

namespace elasticgrid {

struct ConstImageRGBAf {
    const float* data = nullptr;
    int width = 0;
    int height = 0;
    std::ptrdiff_t row_stride_floats = 0;
};

struct ImageRGBAf {
    float* data = nullptr;
    int width = 0;
    int height = 0;
    std::ptrdiff_t row_stride_floats = 0;
};


struct ConstImageRGBA8 {
    const std::uint8_t* data = nullptr;
    int width = 0;
    int height = 0;
    std::ptrdiff_t row_stride_values = 0;
};

struct ImageRGBA8 {
    std::uint8_t* data = nullptr;
    int width = 0;
    int height = 0;
    std::ptrdiff_t row_stride_values = 0;
};

struct ConstImageRGBA16 {
    const std::uint16_t* data = nullptr;
    int width = 0;
    int height = 0;
    std::ptrdiff_t row_stride_values = 0;
};

struct ImageRGBA16 {
    std::uint16_t* data = nullptr;
    int width = 0;
    int height = 0;
    std::ptrdiff_t row_stride_values = 0;
};

using AbortFn = std::int32_t (*)(void* refcon);

struct RenderSettings {
    EdgeMode edge = EdgeMode::Clamp;
    SampleQuality quality = SampleQuality::Bilinear;
    unsigned threads = 0; // 0 = auto
    AbortFn abort_fn = nullptr;
    void* abort_refcon = nullptr;
};

struct RenderCancelled final : std::exception {
    ~RenderCancelled() override;
    const char* what() const noexcept override { return "render cancelled"; }
};

struct LinearSample1D {
    int i0 = 0;
    int i1 = 0;
    float t = 0.0f;
    std::uint32_t t_fixed16 = 0; // 0..65536, for fast integer bilinear sampling
};

struct CubicSample1D {
    std::array<int, 4> index{};
    std::array<float, 4> weight{};
};

// Prepared sampling removes floor/modulo/cubic-weight work from the per-pixel
// hot loop. It is intended to be cached per evaluated grid/frame.
struct PreparedWarpRGBAf {
    int src_width = 0;
    int src_height = 0;
    int dst_width = 0;
    int dst_height = 0;
    SampleQuality quality = SampleQuality::Bilinear;
    bool identity = false;
    // Internal CPU-only plan: -1 taps are transparent missing checkout pixels.
    bool transparent_taps = false;
    std::vector<LinearSample1D> x_linear;
    std::vector<LinearSample1D> y_linear;
    std::vector<CubicSample1D> x_cubic;
    std::vector<CubicSample1D> y_cubic;
};

PreparedWarpRGBAf prepareWarpRGBAf(int src_width, int src_height,
                                   int dst_width, int dst_height,
                                   const WarpAxisLUT& x_lut,
                                   const WarpAxisLUT& y_lut,
                                   const RenderSettings& settings = {});

void prepareWarpRGBAfInto(PreparedWarpRGBAf& prepared,
                          int src_width, int src_height,
                          int dst_width, int dst_height,
                          const WarpAxisLUT& x_lut,
                          const WarpAxisLUT& y_lut,
                          const RenderSettings& settings = {});


void renderWarpRGBA8Prepared(const ConstImageRGBA8& src,
                             const ImageRGBA8& dst,
                             const PreparedWarpRGBAf& prepared,
                             const RenderSettings& settings = {});

// After Effects 16-bpc uses 0..32768 rather than the full uint16 range, so
// the default maximum matches AE.
void renderWarpRGBA16Prepared(const ConstImageRGBA16& src,
                              const ImageRGBA16& dst,
                              const PreparedWarpRGBAf& prepared,
                              const RenderSettings& settings = {},
                              std::uint16_t channel_max = 32768);

void renderWarpRGBAfPrepared(const ConstImageRGBAf& src,
                             const ImageRGBAf& dst,
                             const PreparedWarpRGBAf& prepared,
                             const RenderSettings& settings = {});

void renderWarpRGBAf(const ConstImageRGBAf& src,
                     const ImageRGBAf& dst,
                     const WarpAxisLUT& x_lut,
                     const WarpAxisLUT& y_lut,
                     const RenderSettings& settings = {});

} // namespace elasticgrid
