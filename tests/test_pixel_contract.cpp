#include "bridge/elasticgrid_ffi.h"

#include <algorithm>
#include <array>
#include <cassert>
#include <cmath>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <limits>
#include <vector>

namespace {

EgRenderParams defaults() {
    EgRenderParams p{};
    p.columns = 1;
    p.rows = 1;
    p.tension_radius = 3.0f;
    p.falloff = 1;
    p.elasticity_strength = 1.0f;
    p.min_spacing = 0.005f;
    p.stretch_easing = 0.5f;
    p.easing_distance = 0.25f;
    p.wave_enabled = 0;
    p.wave_frequency = 1.0f;
    p.wave_axis = 1;
    p.edge_mode = 1;
    p.quality = 1;
    p.visualization_enabled = 0;
    p.threads = 1;
    return p;
}

template <typename T>
void fill_argb_pattern(std::vector<T>& data, int width, int height, T maxv) {
    for (int y = 0; y < height; ++y) {
        for (int x = 0; x < width; ++x) {
            const std::size_t i = static_cast<std::size_t>((y * width + x) * 4);
            data[i + 0] = (x & 1) ? T{} : maxv;
            data[i + 1] = static_cast<T>((static_cast<std::uint64_t>(x + 1) * maxv) / (width + 1));
            data[i + 2] = static_cast<T>((static_cast<std::uint64_t>(y + 1) * maxv) / (height + 1));
            data[i + 3] = static_cast<T>(maxv / 3);
        }
    }
}

template <typename T>
void assert_payload_rows_equal(
    const std::vector<T>& a,
    const std::vector<T>& b,
    int width,
    int height,
    std::size_t stride_values) {
    for (int y = 0; y < height; ++y) {
        const auto off = static_cast<std::size_t>(y) * stride_values;
        for (int x = 0; x < width * 4; ++x) {
            assert(a[off + static_cast<std::size_t>(x)] == b[off + static_cast<std::size_t>(x)]);
        }
    }
}

} // namespace

int main() {
    constexpr int w = 11;
    constexpr int h = 7;

    {
        // ARGB32 / 8 bpc identity is byte-exact, including hidden RGB under
        // alpha zero. The effect must never zero RGB merely because alpha is 0.
        std::vector<std::uint8_t> src(w * h * 4);
        std::vector<std::uint8_t> dst(src.size(), 0xA5);
        fill_argb_pattern(src, w, h, static_cast<std::uint8_t>(255));
        auto p = defaults();
        assert(eg_render_frame(src.data(), w * 4, w, h,
                               dst.data(), w * 4, w, h, 8, &p) == 0);
        assert(src == dst);
        assert(dst[0] == 0);
        assert(dst[1] != 0 || dst[2] != 0 || dst[3] != 0);
    }

    {
        // After Effects 16 bpc uses PF_MAX_CHAN16 == 32768, not uint16 max.
        std::vector<std::uint16_t> src(w * h * 4);
        std::vector<std::uint16_t> dst(src.size(), 0);
        fill_argb_pattern(src, w, h, static_cast<std::uint16_t>(32768));
        auto p = defaults();
        assert(eg_render_frame(src.data(), w * 8, w, h,
                               dst.data(), w * 8, w, h, 16, &p) == 0);
        assert(src == dst);
        for (auto v : dst) assert(v <= 32768);
    }

    {
        // 32 bpc float identity must preserve HDR / extended-range samples,
        // negative values, transparent RGB and IEEE non-finite payloads.
        std::vector<float> src(w * h * 4);
        for (int y = 0; y < h; ++y) {
            for (int x = 0; x < w; ++x) {
                const auto i = static_cast<std::size_t>((y * w + x) * 4);
                src[i + 0] = (x & 1) ? 0.0f : 1.0f;
                src[i + 1] = -0.75f + 0.11f * static_cast<float>(x);
                src[i + 2] = 1.25f + 0.07f * static_cast<float>(y);
                src[i + 3] = 2.0f;
            }
        }
        src[5] = std::numeric_limits<float>::infinity();
        src[10] = -std::numeric_limits<float>::infinity();
        src[15] = std::numeric_limits<float>::quiet_NaN();

        std::vector<float> dst(src.size(), 0.0f);
        auto p = defaults();
        assert(eg_render_frame(src.data(), w * 16, w, h,
                               dst.data(), w * 16, w, h, 32, &p) == 0);

        assert(std::memcmp(src.data(), dst.data(), src.size() * sizeof(float)) == 0);
    }

    {
        // A deformed 32f frame must not clamp finite extended range into 0..1.
        std::vector<float> src(w * h * 4);
        for (int y = 0; y < h; ++y) {
            for (int x = 0; x < w; ++x) {
                const auto i = static_cast<std::size_t>((y * w + x) * 4);
                src[i + 0] = 1.0f;
                src[i + 1] = -1.0f + 0.15f * static_cast<float>(x);
                src[i + 2] = 1.2f + 0.08f * static_cast<float>(x + y);
                src[i + 3] = 2.5f - 0.04f * static_cast<float>(y);
            }
        }
        std::vector<float> dst(src.size(), 0.0f);
        float x_lines[] = {0.0f, 0.63f, 1.0f};
        float y_lines[] = {0.0f, 0.42f, 1.0f};
        std::uint8_t pins[] = {1, 0, 1};
        auto p = defaults();
        p.column_lines = x_lines;
        p.column_line_count = 3;
        p.column_pins = pins;
        p.row_lines = y_lines;
        p.row_line_count = 3;
        p.row_pins = pins;
        assert(eg_render_frame(src.data(), w * 16, w, h,
                               dst.data(), w * 16, w, h, 32, &p) == 0);
        assert(std::any_of(dst.begin(), dst.end(), [](float v) { return std::isfinite(v) && v < 0.0f; }));
        assert(std::any_of(dst.begin(), dst.end(), [](float v) { return std::isfinite(v) && v > 1.0f; }));
    }

    {
        // All-transparent input with non-zero RGB stays transparent after a
        // deformation while hidden RGB remains sampleable.
        std::vector<float> src(w * h * 4);
        for (int y = 0; y < h; ++y) {
            for (int x = 0; x < w; ++x) {
                const auto i = static_cast<std::size_t>((y * w + x) * 4);
                src[i + 0] = 0.0f;
                src[i + 1] = 0.2f + 0.03f * static_cast<float>(x);
                src[i + 2] = 0.4f + 0.02f * static_cast<float>(y);
                src[i + 3] = 0.6f;
            }
        }
        std::vector<float> dst(src.size(), 0.0f);
        float x_lines[] = {0.0f, 0.58f, 1.0f};
        float y_lines[] = {0.0f, 0.47f, 1.0f};
        std::uint8_t pins[] = {1, 0, 1};
        auto p = defaults();
        p.column_lines = x_lines; p.column_line_count = 3; p.column_pins = pins;
        p.row_lines = y_lines; p.row_line_count = 3; p.row_pins = pins;
        assert(eg_render_frame(src.data(), w * 16, w, h,
                               dst.data(), w * 16, w, h, 32, &p) == 0);
        for (int i = 0; i < w*h; ++i) assert(std::abs(dst[i*4+0]) < 1.0e-7f);
        assert(std::any_of(dst.begin()+1, dst.end(), [](float v) { return v > 0.1f; }));
    }

    {
        // Positive padded rowbytes are legal. The renderer must touch only the
        // pixel payload and leave row padding / outer guards unchanged.
        constexpr std::size_t pad = 13;
        const std::size_t stride = static_cast<std::size_t>(w * 4) + pad;
        const std::uint8_t guard = 0xD7;
        std::vector<std::uint8_t> src(stride * h, guard);
        std::vector<std::uint8_t> dst(stride * h, guard);
        for (int y = 0; y < h; ++y) {
            auto* row = src.data() + static_cast<std::size_t>(y) * stride;
            for (int x = 0; x < w*4; ++x) row[x] = static_cast<std::uint8_t>((17*x + 31*y) & 255);
        }
        auto p = defaults();
        assert(eg_render_frame(src.data(), static_cast<std::ptrdiff_t>(stride), w, h,
                               dst.data(), static_cast<std::ptrdiff_t>(stride), w, h, 8, &p) == 0);
        assert_payload_rows_equal(src, dst, w, h, stride);
        for (int y = 0; y < h; ++y) {
            const auto off = static_cast<std::size_t>(y) * stride + static_cast<std::size_t>(w*4);
            for (std::size_t x = 0; x < pad; ++x) assert(dst[off+x] == guard);
        }
    }

    {
        // Undersized/misaligned rowbytes are rejected instead of risking an
        // out-of-bounds read or write.
        std::vector<std::uint16_t> src(w*h*4, 0), dst(src.size(), 0);
        auto p = defaults();
        assert(eg_render_frame(src.data(), w*8 - 2, w, h,
                               dst.data(), w*8, w, h, 16, &p) != 0);
        assert(eg_render_frame(src.data(), w*8, w, h,
                               dst.data(), w*8 - 2, w, h, 16, &p) != 0);
    }

    return 0;
}
