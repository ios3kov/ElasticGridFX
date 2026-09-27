#include "bridge/elasticgrid_ffi.h"
#include "core/GridModel.h"
#include "core/WarpMath.h"
#include "core/CpuRenderer.h"

#include <algorithm>
#include <array>
#include <cstring>
#include <bit>
#include <cmath>
#include <cstddef>
#include <cstdint>
#include <limits>
#include <span>
#include <thread>
#include <type_traits>
#include <vector>

namespace eg = elasticgrid;

static_assert(sizeof(void*) == 8, "ElasticGrid AE host ABI requires 64-bit pointers");
static_assert(sizeof(EgRenderParams) == 160, "EgRenderParams ABI drift");
static_assert(offsetof(EgRenderParams, abort_fn) == 144, "EgRenderParams::abort_fn ABI drift");
static_assert(offsetof(EgRenderParams, abort_refcon) == 152, "EgRenderParams::abort_refcon ABI drift");
static_assert(sizeof(EgElasticParams) == 16, "EgElasticParams ABI drift");
static_assert(sizeof(EgRectI32) == 16, "EgRectI32 ABI drift");
static_assert(sizeof(EgGpuLinearSample) == 16, "EgGpuLinearSample ABI drift");
static_assert(sizeof(EgGpuCubicSample) == 32, "EgGpuCubicSample ABI drift");

namespace {
eg::FalloffProfile falloff_from_i32(std::int32_t v) noexcept {
    switch (v) {
        case 1: return eg::FalloffProfile::Linear;
        case 3: return eg::FalloffProfile::Gaussian;
        case 4: return eg::FalloffProfile::Cosine;
        default: return eg::FalloffProfile::Smoothstep;
    }
}

eg::WaveAxis wave_axis_from_i32(std::int32_t v) noexcept {
    switch (v) {
        case 2: return eg::WaveAxis::ColumnsOnly;
        case 3: return eg::WaveAxis::RowsOnly;
        default: return eg::WaveAxis::Both;
    }
}

eg::EdgeMode edge_from_i32(std::int32_t v) noexcept {
    switch (v) {
        case 2: return eg::EdgeMode::Wrap;
        case 3: return eg::EdgeMode::Mirror;
        default: return eg::EdgeMode::Clamp;
    }
}

eg::SampleQuality quality_from_i32(std::int32_t v) noexcept {
    return v == 2 ? eg::SampleQuality::Bicubic : eg::SampleQuality::Bilinear;
}

unsigned effective_threads(std::uint32_t requested) noexcept {
    if (requested != 0) return std::clamp(requested, 1u, 16u);
    const unsigned hw = std::max(1u, std::thread::hardware_concurrency());
    // AE may render several frames concurrently. Keep per-frame CPU fan-out
    // intentionally small so Multi-Frame Rendering is not starved. On macOS
    // the renderer uses GCD, so these are work chunks rather than new threads.
    return std::min(4u, hw);
}

float finite_or(float value, float fallback) noexcept {
    return std::isfinite(value) ? value : fallback;
}

bool valid_row_bytes(std::ptrdiff_t row_bytes, std::int32_t width, std::size_t bytes_per_channel) noexcept {
    if (row_bytes == 0 || width <= 0 || bytes_per_channel == 0) return false;
    if (row_bytes == std::numeric_limits<std::ptrdiff_t>::min()) return false;
    const auto magnitude = static_cast<std::size_t>(row_bytes < 0 ? -row_bytes : row_bytes);
    const auto min_bytes = static_cast<std::size_t>(width) * 4u * bytes_per_channel;
    return magnitude >= min_bytes && (magnitude % bytes_per_channel) == 0;
}

eg::ElasticSettings elastic_from_params(const EgElasticParams& p) noexcept {
    eg::ElasticSettings out;
    out.radius_lines = std::clamp(finite_or(p.tension_radius, 3.0f), 0.0f, 32.0f);
    out.strength = std::clamp(finite_or(p.elasticity_strength, 1.0f), 0.0f, 2.0f);
    out.min_spacing = std::clamp(finite_or(p.min_spacing, 0.005f), 0.0f, 0.25f);
    out.falloff = falloff_from_i32(p.falloff);
    return out;
}

bool load_axis(eg::AxisGrid& axis,
               const float* lines,
               const std::uint8_t* pins,
               std::int32_t line_count,
               float min_spacing) {
    if (!lines || line_count != static_cast<std::int32_t>(axis.lineCount())) return false;
    std::vector<std::uint8_t> default_pins;
    if (!pins) {
        default_pins.assign(static_cast<std::size_t>(line_count), 0);
        default_pins.front() = 1;
        default_pins.back() = 1;
        pins = default_pins.data();
    }
    return axis.setState(
        std::span<const float>(lines, static_cast<std::size_t>(line_count)),
        std::span<const std::uint8_t>(pins, static_cast<std::size_t>(line_count)),
        min_spacing);
}


bool float_bits_equal(float a, float b) noexcept {
    return std::bit_cast<std::uint32_t>(a) == std::bit_cast<std::uint32_t>(b);
}

bool axis_uniform_exact(const std::vector<float>& lines) noexcept {
    if (lines.size() < 2) return false;
    const float cells = static_cast<float>(lines.size() - 1);
    for (std::size_t i = 0; i < lines.size(); ++i) {
        const float expected = static_cast<float>(i) / cells;
        if (!float_bits_equal(lines[i], expected)) return false;
    }
    return true;
}


bool is_mesh_params(const EgRenderParams* p) noexcept {
    if (!p || p->columns < 1 || p->columns > 128 || p->rows < 1 || p->rows > 128) return false;
    const std::int64_t count =
        (static_cast<std::int64_t>(p->columns) + 1) *
        (static_cast<std::int64_t>(p->rows) + 1);
    if (count <= 0 || count > 16641) return false;
    return p->column_lines && p->row_lines &&
        p->column_line_count == count &&
        p->row_line_count == count;
}

bool validate_mesh(const EgRenderParams* p) noexcept {
    if (!is_mesh_params(p)) return false;
    const int columns = p->columns;
    const int rows = p->rows;
    const int stride = columns + 1;
    for (int row = 0; row <= rows; ++row) {
        for (int column = 0; column <= columns; ++column) {
            const int i = row * stride + column;
            const float x = p->column_lines[i];
            const float y = p->row_lines[i];
            if (!std::isfinite(x) || !std::isfinite(y) ||
                x < 0.0f || x > 1.0f || y < 0.0f || y > 1.0f) {
                return false;
            }
            if (column == 0 && x != 0.0f) return false;
            if (column == columns && x != 1.0f) return false;
            if (row == 0 && y != 0.0f) return false;
            if (row == rows && y != 1.0f) return false;
            if (column > 0 && x <= p->column_lines[i - 1]) return false;
            if (row > 0 && y <= p->row_lines[i - stride]) return false;
        }
    }
    return true;
}

struct MeshVec2 {
    float x = 0.0f;
    float y = 0.0f;
};

struct PreparedMesh {
    std::vector<float> source_u;
    std::vector<float> source_v;
    std::vector<std::uint8_t> covered;
    std::vector<float> evaluated_x;
    std::vector<float> evaluated_y;
};

PreparedMesh& reusable_mesh_state() {
    thread_local PreparedMesh state;
    return state;
}

float triangle_edge(const MeshVec2& a, const MeshVec2& b, const MeshVec2& p) noexcept {
    return (p.x - a.x) * (b.y - a.y) - (p.y - a.y) * (b.x - a.x);
}

void raster_mesh_triangle(
    PreparedMesh& out,
    int output_width,
    int output_height,
    MeshVec2 a,
    MeshVec2 b,
    MeshVec2 c,
    MeshVec2 sa,
    MeshVec2 sb,
    MeshVec2 sc) {

    const float area = triangle_edge(a, b, c);
    if (!std::isfinite(area) || std::abs(area) < 1.0e-8f) return;

    const float min_xf = std::min({a.x, b.x, c.x});
    const float max_xf = std::max({a.x, b.x, c.x});
    const float min_yf = std::min({a.y, b.y, c.y});
    const float max_yf = std::max({a.y, b.y, c.y});
    const int x0 = std::clamp(static_cast<int>(std::floor(min_xf)) - 1, 0, output_width - 1);
    const int x1 = std::clamp(static_cast<int>(std::ceil(max_xf)) + 1, 0, output_width - 1);
    const int y0 = std::clamp(static_cast<int>(std::floor(min_yf)) - 1, 0, output_height - 1);
    const int y1 = std::clamp(static_cast<int>(std::ceil(max_yf)) + 1, 0, output_height - 1);

    constexpr float epsilon = 2.0e-4f;
    for (int y = y0; y <= y1; ++y) {
        for (int x = x0; x <= x1; ++x) {
            const MeshVec2 p{static_cast<float>(x), static_cast<float>(y)};
            const float w0 = triangle_edge(b, c, p) / area;
            const float w1 = triangle_edge(c, a, p) / area;
            const float w2 = triangle_edge(a, b, p) / area;
            if (w0 < -epsilon || w1 < -epsilon || w2 < -epsilon ||
                w0 > 1.0f + epsilon || w1 > 1.0f + epsilon || w2 > 1.0f + epsilon) {
                continue;
            }
            const std::size_t i =
                static_cast<std::size_t>(y) * static_cast<std::size_t>(output_width) +
                static_cast<std::size_t>(x);
            out.source_u[i] = sa.x * w0 + sb.x * w1 + sc.x * w2;
            out.source_v[i] = sa.y * w0 + sb.y * w1 + sc.y * w2;
            out.covered[i] = 1;
        }
    }
}

bool prepare_mesh_map(
    const EgRenderParams* p,
    int output_width,
    int output_height,
    PreparedMesh& out) {

    if (!validate_mesh(p) || output_width <= 0 || output_height <= 0) return false;
    const int columns = p->columns;
    const int rows = p->rows;
    const int stride = columns + 1;
    const int canvas_width = p->canvas_width > 0 ? p->canvas_width : output_width;
    const int canvas_height = p->canvas_height > 0 ? p->canvas_height : output_height;
    if (canvas_width <= 0 || canvas_height <= 0) return false;

    const std::size_t pixel_count =
        static_cast<std::size_t>(output_width) * static_cast<std::size_t>(output_height);
    const std::size_t node_count =
        static_cast<std::size_t>(columns + 1) * static_cast<std::size_t>(rows + 1);
    out.source_u.resize(pixel_count);
    out.source_v.resize(pixel_count);
    out.covered.assign(pixel_count, 0);
    out.evaluated_x.assign(p->column_lines, p->column_lines + node_count);
    out.evaluated_y.assign(p->row_lines, p->row_lines + node_count);

    // Preserve the old Wave semantics on the new mesh: X and Y coordinates
    // are evaluated independently, but now every intersection remains a real
    // 2D control point.
    if (p->wave_enabled != 0) {
        constexpr double kTwoPi = 6.283185307179586476925286766559;
        const float amplitude = std::clamp(finite_or(p->wave_amplitude, 0.0f), 0.0f, 0.25f);
        const float frequency = std::max(0.0f, finite_or(p->wave_frequency, 1.0f));
        const float phase = finite_or(p->wave_phase, 0.0f);
        const float speed = finite_or(p->wave_speed, 0.0f);
        const float time = finite_or(p->time_seconds, 0.0f);
        const bool move_x = p->wave_axis != 3;
        const bool move_y = p->wave_axis != 2;
        for (int row = 1; row < rows; ++row) {
            for (int column = 1; column < columns; ++column) {
                const int i = row * stride + column;
                const bool pinned =
                    (p->column_pins && p->column_pins[i] != 0) ||
                    (p->row_pins && p->row_pins[i] != 0);
                if (pinned) continue;
                if (move_x) {
                    const double cycles =
                        static_cast<double>(frequency) * static_cast<double>(p->column_lines[i]) +
                        static_cast<double>(phase) +
                        static_cast<double>(speed) * static_cast<double>(time);
                    out.evaluated_x[i] += amplitude * static_cast<float>(
                        std::sin(kTwoPi * std::remainder(cycles, 1.0)));
                }
                if (move_y) {
                    const double cycles =
                        static_cast<double>(frequency) * static_cast<double>(p->row_lines[i]) +
                        static_cast<double>(phase) +
                        static_cast<double>(speed) * static_cast<double>(time);
                    out.evaluated_y[i] += amplitude * static_cast<float>(
                        std::sin(kTwoPi * std::remainder(cycles, 1.0)));
                }
            }
        }

        const float min_spacing = std::clamp(finite_or(p->min_spacing, 0.005f), 0.0f, 0.25f);
        const float sx = std::min(min_spacing, 0.45f / static_cast<float>(columns));
        const float sy = std::min(min_spacing, 0.45f / static_cast<float>(rows));
        for (int pass = 0; pass < 2; ++pass) {
            for (int row = 0; row <= rows; ++row) {
                for (int column = 1; column < columns; ++column) {
                    const int i = row * stride + column;
                    out.evaluated_x[i] = std::max(out.evaluated_x[i], out.evaluated_x[i - 1] + sx);
                }
                for (int column = columns - 1; column >= 1; --column) {
                    const int i = row * stride + column;
                    out.evaluated_x[i] = std::min(out.evaluated_x[i], out.evaluated_x[i + 1] - sx);
                }
            }
            for (int column = 0; column <= columns; ++column) {
                for (int row = 1; row < rows; ++row) {
                    const int i = row * stride + column;
                    out.evaluated_y[i] = std::max(out.evaluated_y[i], out.evaluated_y[i - stride] + sy);
                }
                for (int row = rows - 1; row >= 1; --row) {
                    const int i = row * stride + column;
                    out.evaluated_y[i] = std::min(out.evaluated_y[i], out.evaluated_y[i + stride] - sy);
                }
            }
        }
    }

    const float canvas_x = static_cast<float>(std::max(1, canvas_width - 1));
    const float canvas_y = static_cast<float>(std::max(1, canvas_height - 1));
    auto dst = [&](int column, int row) -> MeshVec2 {
        const int i = row * stride + column;
        return {
            out.evaluated_x[i] * canvas_x - static_cast<float>(p->output_origin_x),
            out.evaluated_y[i] * canvas_y - static_cast<float>(p->output_origin_y),
        };
    };
    auto src = [&](int column, int row) -> MeshVec2 {
        return {
            static_cast<float>(column) / static_cast<float>(columns),
            static_cast<float>(row) / static_cast<float>(rows),
        };
    };

    for (int row = 0; row < rows; ++row) {
        for (int column = 0; column < columns; ++column) {
            const MeshVec2 d00 = dst(column, row);
            const MeshVec2 d10 = dst(column + 1, row);
            const MeshVec2 d01 = dst(column, row + 1);
            const MeshVec2 d11 = dst(column + 1, row + 1);
            const MeshVec2 s00 = src(column, row);
            const MeshVec2 s10 = src(column + 1, row);
            const MeshVec2 s01 = src(column, row + 1);
            const MeshVec2 s11 = src(column + 1, row + 1);

            // Fixed diagonal gives a deterministic piecewise-affine 2D mesh.
            raster_mesh_triangle(out, output_width, output_height, d00, d10, d11, s00, s10, s11);
            raster_mesh_triangle(out, output_width, output_height, d00, d11, d01, s00, s11, s01);
        }
    }

    // Shared triangle edges can miss a pixel under extreme floating-point
    // positions. Fill only those rare gaps with the undeformed full-canvas map
    // instead of emitting uninitialized pixels.
    const float full_x_denom = static_cast<float>(std::max(1, canvas_width - 1));
    const float full_y_denom = static_cast<float>(std::max(1, canvas_height - 1));
    for (int y = 0; y < output_height; ++y) {
        for (int x = 0; x < output_width; ++x) {
            const std::size_t i =
                static_cast<std::size_t>(y) * static_cast<std::size_t>(output_width) +
                static_cast<std::size_t>(x);
            if (out.covered[i]) continue;
            out.source_u[i] = std::clamp(
                static_cast<float>(p->output_origin_x + x) / full_x_denom, 0.0f, 1.0f);
            out.source_v[i] = std::clamp(
                static_cast<float>(p->output_origin_y + y) / full_y_denom, 0.0f, 1.0f);
        }
    }
    return true;
}

int wrap_mesh_index(int i, int n) noexcept {
    if (n <= 1) return 0;
    i %= n;
    if (i < 0) i += n;
    return i;
}

int mirror_mesh_index(int i, int n) noexcept {
    if (n <= 1) return 0;
    const int period = 2 * n - 2;
    const int x = wrap_mesh_index(i, period);
    return x < n ? x : period - x;
}

int resolve_mesh_index(int i, int n, int edge_mode) noexcept {
    if (edge_mode == 2) return wrap_mesh_index(i, n);
    if (edge_mode == 3) return mirror_mesh_index(i, n);
    return std::clamp(i, 0, n - 1);
}

float mesh_cubic_weight(float x) noexcept {
    constexpr float a = -0.5f;
    x = std::abs(x);
    if (x < 1.0f) return (a + 2.0f) * x*x*x - (a + 3.0f) * x*x + 1.0f;
    if (x < 2.0f) return a * x*x*x - 5.0f*a*x*x + 8.0f*a*x - 4.0f*a;
    return 0.0f;
}

template <typename T>
const T* mesh_pixel(
    const void* data,
    std::ptrdiff_t row_bytes,
    int x,
    int y) noexcept {
    const auto* row =
        static_cast<const std::byte*>(data) +
        static_cast<std::ptrdiff_t>(y) * row_bytes;
    return reinterpret_cast<const T*>(row) + static_cast<std::ptrdiff_t>(x) * 4;
}

template <typename T>
T* mesh_pixel_mut(
    void* data,
    std::ptrdiff_t row_bytes,
    int x,
    int y) noexcept {
    auto* row =
        static_cast<std::byte*>(data) +
        static_cast<std::ptrdiff_t>(y) * row_bytes;
    return reinterpret_cast<T*>(row) + static_cast<std::ptrdiff_t>(x) * 4;
}

template <typename T>
T mesh_store(float value, float channel_max) noexcept {
    if constexpr (std::is_same_v<T, float>) {
        return value;
    } else {
        value = std::clamp(value, 0.0f, channel_max);
        return static_cast<T>(std::lround(value));
    }
}

template <typename T>
void sample_mesh_bilinear(
    const void* input_data,
    std::ptrdiff_t input_row_bytes,
    int input_width,
    int input_height,
    float x,
    float y,
    int edge_mode,
    float channel_max,
    T* dst) noexcept {

    const int x0_raw = static_cast<int>(std::floor(x));
    const int y0_raw = static_cast<int>(std::floor(y));
    const float tx = x - static_cast<float>(x0_raw);
    const float ty = y - static_cast<float>(y0_raw);
    const int x0 = resolve_mesh_index(x0_raw, input_width, edge_mode);
    const int x1 = resolve_mesh_index(x0_raw + 1, input_width, edge_mode);
    const int y0 = resolve_mesh_index(y0_raw, input_height, edge_mode);
    const int y1 = resolve_mesh_index(y0_raw + 1, input_height, edge_mode);
    const T* p00 = mesh_pixel<T>(input_data, input_row_bytes, x0, y0);
    const T* p10 = mesh_pixel<T>(input_data, input_row_bytes, x1, y0);
    const T* p01 = mesh_pixel<T>(input_data, input_row_bytes, x0, y1);
    const T* p11 = mesh_pixel<T>(input_data, input_row_bytes, x1, y1);

    for (int c = 0; c < 4; ++c) {
        const float a = static_cast<float>(p00[c]) +
            (static_cast<float>(p10[c]) - static_cast<float>(p00[c])) * tx;
        const float b = static_cast<float>(p01[c]) +
            (static_cast<float>(p11[c]) - static_cast<float>(p01[c])) * tx;
        dst[c] = mesh_store<T>(a + (b - a) * ty, channel_max);
    }
}

template <typename T>
void sample_mesh_bicubic(
    const void* input_data,
    std::ptrdiff_t input_row_bytes,
    int input_width,
    int input_height,
    float x,
    float y,
    int edge_mode,
    float channel_max,
    T* dst) noexcept {

    const int bx = static_cast<int>(std::floor(x));
    const int by = static_cast<int>(std::floor(y));
    float acc[4] = {0, 0, 0, 0};
    float sum = 0.0f;
    for (int ky = -1; ky <= 2; ++ky) {
        const int sy = resolve_mesh_index(by + ky, input_height, edge_mode);
        const float wy = mesh_cubic_weight(y - static_cast<float>(by + ky));
        for (int kx = -1; kx <= 2; ++kx) {
            const int sx = resolve_mesh_index(bx + kx, input_width, edge_mode);
            const float wx = mesh_cubic_weight(x - static_cast<float>(bx + kx));
            const float w = wx * wy;
            const T* q = mesh_pixel<T>(input_data, input_row_bytes, sx, sy);
            for (int c = 0; c < 4; ++c) acc[c] += static_cast<float>(q[c]) * w;
            sum += w;
        }
    }
    if (std::abs(sum) > 1.0e-8f) {
        for (float& v : acc) v /= sum;
    }
    for (int c = 0; c < 4; ++c) dst[c] = mesh_store<T>(acc[c], channel_max);
}

template <typename T>
int render_mesh_typed(
    const void* input_data,
    std::ptrdiff_t input_row_bytes,
    int input_width,
    int input_height,
    void* output_data,
    std::ptrdiff_t output_row_bytes,
    int output_width,
    int output_height,
    const EgRenderParams* p,
    float channel_max) {

    PreparedMesh& map = reusable_mesh_state();
    if (!prepare_mesh_map(p, output_width, output_height, map)) return 4;

    if (p->abort_fn && p->abort_fn(p->abort_refcon) != 0) return 5;

    const int canvas_width = p->canvas_width > 0 ? p->canvas_width : output_width;
    const int canvas_height = p->canvas_height > 0 ? p->canvas_height : output_height;
    const float canvas_x = static_cast<float>(std::max(1, canvas_width - 1));
    const float canvas_y = static_cast<float>(std::max(1, canvas_height - 1));
    const bool cubic = p->quality == 2;

    auto render_rows = [&](int y0, int y1) {
        for (int y = y0; y < y1; ++y) {
            for (int x = 0; x < output_width; ++x) {
                const std::size_t i =
                    static_cast<std::size_t>(y) * static_cast<std::size_t>(output_width) +
                    static_cast<std::size_t>(x);
                const float source_x =
                    map.source_u[i] * canvas_x - static_cast<float>(p->input_origin_x);
                const float source_y =
                    map.source_v[i] * canvas_y - static_cast<float>(p->input_origin_y);
                T* dst = mesh_pixel_mut<T>(output_data, output_row_bytes, x, y);
                if (cubic) {
                    sample_mesh_bicubic<T>(
                        input_data, input_row_bytes, input_width, input_height,
                        source_x, source_y, p->edge_mode, channel_max, dst);
                } else {
                    sample_mesh_bilinear<T>(
                        input_data, input_row_bytes, input_width, input_height,
                        source_x, source_y, p->edge_mode, channel_max, dst);
                }
            }
        }
    };

    const unsigned count =
        std::max(1u, std::min<unsigned>(
            effective_threads(p->threads),
            static_cast<unsigned>(std::max(1, output_height))));
    if (count == 1 || output_height < 64) {
        render_rows(0, output_height);
    } else {
        std::vector<std::thread> workers;
        workers.reserve(count);
        const int rows_per =
            (output_height + static_cast<int>(count) - 1) / static_cast<int>(count);
        for (unsigned t = 0; t < count; ++t) {
            const int y0 = static_cast<int>(t) * rows_per;
            const int y1 = std::min(output_height, y0 + rows_per);
            if (y0 >= y1) break;
            workers.emplace_back(render_rows, y0, y1);
        }
        for (auto& worker : workers) worker.join();
    }

    if (p->abort_fn && p->abort_fn(p->abort_refcon) != 0) return 5;
    return 0;
}

int render_mesh_frame(
    const void* input_data,
    std::ptrdiff_t input_row_bytes,
    int input_width,
    int input_height,
    void* output_data,
    std::ptrdiff_t output_row_bytes,
    int output_width,
    int output_height,
    int bit_depth,
    const EgRenderParams* p) {

    if (bit_depth == 8) {
        return render_mesh_typed<std::uint8_t>(
            input_data, input_row_bytes, input_width, input_height,
            output_data, output_row_bytes, output_width, output_height, p, 255.0f);
    }
    if (bit_depth == 16) {
        return render_mesh_typed<std::uint16_t>(
            input_data, input_row_bytes, input_width, input_height,
            output_data, output_row_bytes, output_width, output_height, p, 32768.0f);
    }
    if (bit_depth == 32) {
        return render_mesh_typed<float>(
            input_data, input_row_bytes, input_width, input_height,
            output_data, output_row_bytes, output_width, output_height, p, 1.0f);
    }
    return 2;
}

struct PreparedBridge {
    eg::RenderSettings settings;
    eg::PreparedWarpRGBAf plan;
    eg::AxisGrid gx{4};
    eg::AxisGrid gy{4};
    std::vector<float> x_lines;
    std::vector<float> y_lines;
    eg::WarpAxisLUT x_lut;
    eg::WarpAxisLUT y_lut;
};

PreparedBridge& reusable_bridge_state() {
    // Each AE render thread keeps its own reusable preparation buffers. This
    // removes steady-state per-frame heap traffic without introducing locks or
    // cross-frame mutable state.
    thread_local PreparedBridge state;
    return state;
}

int prepare_bridge(std::int32_t input_width,
                   std::int32_t input_height,
                   std::int32_t output_width,
                   std::int32_t output_height,
                   const EgRenderParams* p,
                   PreparedBridge& out,
                   bool force_sampling_plan = false) {
    if (!p || input_width <= 0 || input_height <= 0 || output_width <= 0 || output_height <= 0) {
        return 1;
    }

    const int columns = std::clamp<int>(p->columns, 1, 128);
    const int rows = std::clamp<int>(p->rows, 1, 128);

    out.gx.reset(static_cast<std::size_t>(columns));
    out.gy.reset(static_cast<std::size_t>(rows));

    const EgElasticParams elastic_params{
        p->tension_radius,
        p->falloff,
        p->elasticity_strength,
        p->min_spacing,
    };
    const auto elastic = elastic_from_params(elastic_params);
    if (p->column_lines &&
        !load_axis(out.gx, p->column_lines, p->column_pins, p->column_line_count, elastic.min_spacing)) {
        return 4;
    }
    if (p->row_lines &&
        !load_axis(out.gy, p->row_lines, p->row_pins, p->row_line_count, elastic.min_spacing)) {
        return 4;
    }

    eg::WaveSettings wave;
    wave.enabled = p->wave_enabled != 0;
    wave.amplitude = std::clamp(finite_or(p->wave_amplitude, 0.0f), 0.0f, 0.25f);
    wave.frequency = std::max(0.0f, finite_or(p->wave_frequency, 1.0f));
    wave.phase_cycles = finite_or(p->wave_phase, 0.0f);
    wave.speed_cycles_per_second = finite_or(p->wave_speed, 0.0f);
    wave.axis = wave_axis_from_i32(p->wave_axis);

    const float min_spacing = elastic.min_spacing;
    const bool wave_x = wave.axis != eg::WaveAxis::RowsOnly;
    const bool wave_y = wave.axis != eg::WaveAxis::ColumnsOnly;
    const float time_seconds = finite_or(p->time_seconds, 0.0f);
    out.gx.evaluatedInto(out.x_lines, wave, time_seconds, min_spacing, wave_x);
    out.gy.evaluatedInto(out.y_lines, wave, time_seconds, min_spacing, wave_y);

    const float easing = std::clamp(finite_or(p->stretch_easing, 0.0f), 0.0f, 1.0f);
    const float easing_distance = std::clamp(finite_or(p->easing_distance, 0.25f), 0.0f, 1.0f);
    const int canvas_width = p->canvas_width > 0 ? p->canvas_width : output_width;
    const int canvas_height = p->canvas_height > 0 ? p->canvas_height : output_height;
    eg::buildInverseLUTRangeInto(out.x_lut, out.x_lines, output_width, canvas_width,
                                 p->output_origin_x, easing, easing_distance);
    eg::buildInverseLUTRangeInto(out.y_lut, out.y_lines, output_height, canvas_height,
                                 p->output_origin_y, easing, easing_distance);

    // Convert full-layer normalized source coordinates into the local checked-out
    // input-world coordinates. This is the key SmartFX ROI/origin correction.
    const float canvas_x_denom = static_cast<float>(std::max(1, canvas_width - 1));
    const float canvas_y_denom = static_cast<float>(std::max(1, canvas_height - 1));
    const float input_x_denom = static_cast<float>(std::max(1, input_width - 1));
    const float input_y_denom = static_cast<float>(std::max(1, input_height - 1));
    for (float& u : out.x_lut.source_u) {
        const float layer_x = u * canvas_x_denom;
        u = (layer_x - static_cast<float>(p->input_origin_x)) / input_x_denom;
    }
    for (float& v : out.y_lut.source_u) {
        const float layer_y = v * canvas_y_denom;
        v = (layer_y - static_cast<float>(p->input_origin_y)) / input_y_denom;
    }

    out.settings.edge = edge_from_i32(p->edge_mode);
    out.settings.quality = quality_from_i32(p->quality);
    out.settings.threads = effective_threads(p->threads);
    out.settings.abort_fn = p->abort_fn;
    out.settings.abort_refcon = p->abort_refcon;

    // Exact semantic identity: no approximation/tolerance is used here. The
    // grid must be exactly uniform after evaluation, easing must be exactly
    // disabled, and source/output sizes must match. CPU rendering can then
    // skip sampling-plan construction entirely and perform an exact row copy.
    const bool semantic_identity =
        input_width == output_width && input_height == output_height &&
        p->input_origin_x == p->output_origin_x && p->input_origin_y == p->output_origin_y &&
        float_bits_equal(easing, 0.0f) && axis_uniform_exact(out.x_lines) && axis_uniform_exact(out.y_lines);
    if (semantic_identity && !force_sampling_plan) {
        out.plan.src_width = input_width;
        out.plan.src_height = input_height;
        out.plan.dst_width = output_width;
        out.plan.dst_height = output_height;
        out.plan.quality = out.settings.quality;
        out.plan.identity = true;
        return 0;
    }

    eg::prepareWarpRGBAfInto(
        out.plan, input_width, input_height,
        output_width, output_height,
        out.x_lut, out.y_lut, out.settings);
    return 0;
}

} // namespace

int eg_drag_axis(
    float* lines,
    std::uint8_t* pins,
    std::int32_t line_count,
    std::int32_t line_index,
    float target,
    const EgElasticParams* p) noexcept {
    if (!lines || !pins || !p || line_count < 2 || line_count > 129 ||
        line_index <= 0 || line_index >= line_count - 1 || !std::isfinite(target)) {
        return 1;
    }
    try {
        eg::AxisGrid axis(static_cast<std::size_t>(line_count - 1));
        const auto elastic = elastic_from_params(*p);
        if (!axis.setState(
                std::span<const float>(lines, static_cast<std::size_t>(line_count)),
                std::span<const std::uint8_t>(pins, static_cast<std::size_t>(line_count)),
                elastic.min_spacing)) {
            return 2;
        }
        if (!axis.dragElastic(static_cast<std::size_t>(line_index), target, elastic)) return 3;
        std::copy(axis.lines().begin(), axis.lines().end(), lines);
        std::copy(axis.pins().begin(), axis.pins().end(), pins);
        return 0;
    } catch (...) {
        return 4;
    }
}

int eg_prepare_gpu_plan(
    std::int32_t input_width,
    std::int32_t input_height,
    std::int32_t output_width,
    std::int32_t output_height,
    const EgRenderParams* p,
    EgGpuLinearSample* x_linear,
    EgGpuLinearSample* y_linear,
    EgGpuCubicSample* x_cubic,
    EgGpuCubicSample* y_cubic) noexcept {
    try {
        if (is_mesh_params(p)) return 6; // true 2D mesh currently uses the CPU path
        PreparedBridge& prepared = reusable_bridge_state();
        const int rc = prepare_bridge(input_width, input_height, output_width, output_height, p, prepared, true);
        if (rc != 0) return rc;

        if (prepared.settings.quality == eg::SampleQuality::Bilinear) {
            if (!x_linear || !y_linear) return 1;
            for (std::int32_t x = 0; x < output_width; ++x) {
                const auto& s = prepared.plan.x_linear[static_cast<std::size_t>(x)];
                x_linear[x] = EgGpuLinearSample{static_cast<std::int32_t>(s.i0), static_cast<std::int32_t>(s.i1), s.t, 0.0f};
            }
            for (std::int32_t y = 0; y < output_height; ++y) {
                const auto& s = prepared.plan.y_linear[static_cast<std::size_t>(y)];
                y_linear[y] = EgGpuLinearSample{static_cast<std::int32_t>(s.i0), static_cast<std::int32_t>(s.i1), s.t, 0.0f};
            }
        } else {
            if (!x_cubic || !y_cubic) return 1;
            for (std::int32_t x = 0; x < output_width; ++x) {
                const auto& s = prepared.plan.x_cubic[static_cast<std::size_t>(x)];
                EgGpuCubicSample o{};
                for (std::size_t i = 0; i < 4u; ++i) { o.index[i] = s.index[i]; o.weight[i] = s.weight[i]; }
                x_cubic[x] = o;
            }
            for (std::int32_t y = 0; y < output_height; ++y) {
                const auto& s = prepared.plan.y_cubic[static_cast<std::size_t>(y)];
                EgGpuCubicSample o{};
                for (std::size_t i = 0; i < 4u; ++i) { o.index[i] = s.index[i]; o.weight[i] = s.weight[i]; }
                y_cubic[y] = o;
            }
        }
        return 0;
    } catch (...) {
        return 3;
    }
}

int eg_required_source_rect(
    std::int32_t canvas_width,
    std::int32_t canvas_height,
    EgRectI32 output_rect,
    const EgRenderParams* p,
    EgRectI32* source_rect) noexcept {
    if (!p || !source_rect || canvas_width <= 0 || canvas_height <= 0 ||
        output_rect.right <= output_rect.left || output_rect.bottom <= output_rect.top) {
        return 1;
    }
    try {
        if (is_mesh_params(p)) {
            if (!validate_mesh(p)) return 4;
            // A moved 2D node can pull source pixels from any neighboring cell.
            // Request the full canvas for correctness until a mesh-aware ROI
            // hull is introduced.
            source_rect->left = 0;
            source_rect->top = 0;
            source_rect->right = canvas_width;
            source_rect->bottom = canvas_height;
            return 0;
        }

        EgRenderParams local = *p;
        local.canvas_width = canvas_width;
        local.canvas_height = canvas_height;
        local.input_origin_x = 0;
        local.input_origin_y = 0;
        local.output_origin_x = output_rect.left;
        local.output_origin_y = output_rect.top;
        local.abort_fn = nullptr;
        local.abort_refcon = nullptr;
        const std::int64_t ow64 = static_cast<std::int64_t>(output_rect.right) - static_cast<std::int64_t>(output_rect.left);
        const std::int64_t oh64 = static_cast<std::int64_t>(output_rect.bottom) - static_cast<std::int64_t>(output_rect.top);
        if (ow64 <= 0 || oh64 <= 0 ||
            ow64 > std::numeric_limits<std::int32_t>::max() ||
            oh64 > std::numeric_limits<std::int32_t>::max()) {
            return 1;
        }
        const int ow = static_cast<int>(ow64);
        const int oh = static_cast<int>(oh64);
        PreparedBridge& prepared = reusable_bridge_state();
        const int rc = prepare_bridge(canvas_width, canvas_height, ow, oh, &local, prepared, true);
        if (rc != 0) return rc;

        int min_x = canvas_width - 1, max_x = 0, min_y = canvas_height - 1, max_y = 0;
        if (prepared.settings.quality == eg::SampleQuality::Bilinear) {
            for (const auto& q : prepared.plan.x_linear) { min_x = std::min({min_x, q.i0, q.i1}); max_x = std::max({max_x, q.i0, q.i1}); }
            for (const auto& q : prepared.plan.y_linear) { min_y = std::min({min_y, q.i0, q.i1}); max_y = std::max({max_y, q.i0, q.i1}); }
        } else {
            for (const auto& q : prepared.plan.x_cubic) for (std::size_t i=0;i<4u;++i) { min_x = std::min(min_x, q.index[i]); max_x = std::max(max_x, q.index[i]); }
            for (const auto& q : prepared.plan.y_cubic) for (std::size_t i=0;i<4u;++i) { min_y = std::min(min_y, q.index[i]); max_y = std::max(max_y, q.index[i]); }
        }
        source_rect->left = std::clamp(min_x, 0, canvas_width - 1);
        source_rect->top = std::clamp(min_y, 0, canvas_height - 1);
        source_rect->right = std::clamp(max_x + 1, 1, canvas_width);
        source_rect->bottom = std::clamp(max_y + 1, 1, canvas_height);
        return 0;
    } catch (...) {
        return 3;
    }
}

int eg_render_frame(
    const void* input_data,
    std::ptrdiff_t input_row_bytes,
    std::int32_t input_width,
    std::int32_t input_height,
    void* output_data,
    std::ptrdiff_t output_row_bytes,
    std::int32_t output_width,
    std::int32_t output_height,
    std::int32_t bit_depth,
    const EgRenderParams* p) noexcept {

    if (!input_data || !output_data || !p ||
        input_width <= 0 || input_height <= 0 ||
        output_width <= 0 || output_height <= 0) {
        return 1;
    }

    const std::size_t bytes_per_channel = bit_depth == 8 ? 1u : bit_depth == 16 ? 2u : bit_depth == 32 ? 4u : 0u;
    if (bytes_per_channel == 0) return 2;
    if (!valid_row_bytes(input_row_bytes, input_width, bytes_per_channel) ||
        !valid_row_bytes(output_row_bytes, output_width, bytes_per_channel)) {
        return 1;
    }

    try {
        if (is_mesh_params(p)) {
            return render_mesh_frame(
                input_data, input_row_bytes, input_width, input_height,
                output_data, output_row_bytes, output_width, output_height,
                bit_depth, p);
        }

        PreparedBridge& prepared = reusable_bridge_state();
        const int prep_rc = prepare_bridge(
            input_width, input_height, output_width, output_height, p, prepared);
        if (prep_rc != 0) return prep_rc;

        if (bit_depth == 32) {
            eg::ConstImageRGBAf src{
                static_cast<const float*>(input_data), input_width, input_height,
                input_row_bytes / static_cast<std::ptrdiff_t>(sizeof(float))};
            eg::ImageRGBAf dst{
                static_cast<float*>(output_data), output_width, output_height,
                output_row_bytes / static_cast<std::ptrdiff_t>(sizeof(float))};
            eg::renderWarpRGBAfPrepared(src, dst, prepared.plan, prepared.settings);
            return 0;
        }
        if (bit_depth == 16) {
            eg::ConstImageRGBA16 src{
                static_cast<const std::uint16_t*>(input_data), input_width, input_height,
                input_row_bytes / static_cast<std::ptrdiff_t>(sizeof(std::uint16_t))};
            eg::ImageRGBA16 dst{
                static_cast<std::uint16_t*>(output_data), output_width, output_height,
                output_row_bytes / static_cast<std::ptrdiff_t>(sizeof(std::uint16_t))};
            eg::renderWarpRGBA16Prepared(src, dst, prepared.plan, prepared.settings, 32768);
            return 0;
        }
        if (bit_depth == 8) {
            eg::ConstImageRGBA8 src{
                static_cast<const std::uint8_t*>(input_data), input_width, input_height,
                input_row_bytes};
            eg::ImageRGBA8 dst{
                static_cast<std::uint8_t*>(output_data), output_width, output_height,
                output_row_bytes};
            eg::renderWarpRGBA8Prepared(src, dst, prepared.plan, prepared.settings);
            return 0;
        }
        return 2;
    } catch (const eg::RenderCancelled&) {
        return 5;
    } catch (...) {
        return 3;
    }
}
