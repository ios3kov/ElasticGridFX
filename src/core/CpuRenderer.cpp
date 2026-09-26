#include "core/CpuRenderer.h"
#include "core/SimdPixelOps.h"
#include <algorithm>
#include <array>
#include <bit>
#include <cstdint>
#include <cmath>
#include <cstring>
#include <stdexcept>
#include <thread>
#include <vector>
#include <type_traits>

#if defined(__APPLE__)
#include <dispatch/dispatch.h>
#endif

namespace elasticgrid {
namespace {
int wrapIndex(int i, int n) {
    if (n <= 1) return 0;
    i %= n;
    if (i < 0) i += n;
    return i;
}

int mirrorIndex(int i, int n) {
    if (n <= 1) return 0;
    const int period = 2 * n - 2;
    int x = wrapIndex(i, period);
    return x < n ? x : period - x;
}

int resolveIndex(int i, int n, EdgeMode mode) {
    switch (mode) {
        case EdgeMode::Clamp: return std::clamp(i, 0, n - 1);
        case EdgeMode::Wrap: return wrapIndex(i, n);
        case EdgeMode::Mirror: return mirrorIndex(i, n);
        default: return std::clamp(i, 0, n - 1);
    }
}


bool floatBitsEqual(float a, float b) noexcept {
    return std::bit_cast<std::uint32_t>(a) == std::bit_cast<std::uint32_t>(b);
}

float cubicWeight(float x) {
    constexpr float a = -0.5f; // Catmull-Rom / Keys
    x = std::abs(x);
    if (x < 1.0f) return (a + 2.0f) * x*x*x - (a + 3.0f) * x*x + 1.0f;
    if (x < 2.0f) return a * x*x*x - 5.0f*a*x*x + 8.0f*a*x - 4.0f*a;
    return 0.0f;
}

LinearSample1D makeLinear(float normalized, int extent, EdgeMode edge) {
    const float p = normalized * static_cast<float>(std::max(0, extent - 1));
    const float f = std::floor(p);
    const int i = static_cast<int>(f);
    const float t = p - f;
    const auto tf = static_cast<std::uint32_t>(std::clamp(t, 0.0f, 1.0f) * 65536.0f + 0.5f);
    return {resolveIndex(i, extent, edge), resolveIndex(i + 1, extent, edge), t, tf};
}

CubicSample1D makeCubic(float normalized, int extent, EdgeMode edge) {
    const float p = normalized * static_cast<float>(std::max(0, extent - 1));
    const int base = static_cast<int>(std::floor(p));
    CubicSample1D s;
    float sum = 0.0f;
    for (int k = 0; k < 4; ++k) {
        const int raw = base + (k - 1);
        s.index[static_cast<std::size_t>(k)] = resolveIndex(raw, extent, edge);
        s.weight[static_cast<std::size_t>(k)] = cubicWeight(p - static_cast<float>(raw));
        sum += s.weight[static_cast<std::size_t>(k)];
    }
    if (std::abs(sum) > 1e-8f) for (float& w : s.weight) w /= sum;
    return s;
}

bool axisIsIdentity(const WarpAxisLUT& lut, int extent) {
    if (extent <= 0 || static_cast<int>(lut.source_u.size()) != extent) return false;
    const float denom = static_cast<float>(std::max(1, extent - 1));
    for (int i = 0; i < extent; ++i) {
        const float expected = static_cast<float>(i) / denom;
        if (!floatBitsEqual(lut.source_u[static_cast<std::size_t>(i)], expected)) return false;
    }
    return true;
}

bool abortRequested(const RenderSettings& settings) noexcept {
    return settings.abort_fn && settings.abort_fn(settings.abort_refcon) != 0;
}

template <typename Fn>
void parallelRows(int height, const RenderSettings& settings, Fn&& fn) {
    if (height <= 0) return;
    if (abortRequested(settings)) throw RenderCancelled{};

    auto run_range = [&](int batch0, int batch1) {
        const int batch_height = batch1 - batch0;
        unsigned count = settings.threads ? settings.threads : std::thread::hardware_concurrency();
        count = std::max(1u, std::min<unsigned>(count, static_cast<unsigned>(batch_height)));
        if (count == 1) {
            fn(batch0, batch1);
            return;
        }

        const int rows_per = (batch_height + static_cast<int>(count) - 1) / static_cast<int>(count);
#if defined(__APPLE__)
        using F = std::remove_reference_t<Fn>;
        struct Context { F* fn; int batch0; int batch1; int rows_per; };
        Context ctx{&fn, batch0, batch1, rows_per};
        dispatch_apply_f(
            count,
            dispatch_get_global_queue(QOS_CLASS_USER_INITIATED, 0),
            &ctx,
            [](void* raw, std::size_t t) {
                auto& c = *static_cast<Context*>(raw);
                const int y0 = c.batch0 + static_cast<int>(t) * c.rows_per;
                const int y1 = std::min(c.batch1, y0 + c.rows_per);
                if (y0 < y1) (*c.fn)(y0, y1);
            });
#else
        std::vector<std::thread> workers;
        workers.reserve(count);
        for (unsigned t = 0; t < count; ++t) {
            const int y0 = batch0 + static_cast<int>(t) * rows_per;
            const int y1 = std::min(batch1, y0 + rows_per);
            if (y0 >= y1) break;
            workers.emplace_back(fn, y0, y1);
        }
        for (auto& th : workers) th.join();
#endif
    };

    // Benchmarks, tests and non-host callers that do not supply an abort
    // callback keep the lowest-overhead one-dispatch path. In After Effects,
    // return to the calling thread periodically so the host abort callback can
    // be polled without ever calling AE from worker threads.
    if (!settings.abort_fn) {
        run_range(0, height);
        return;
    }

    constexpr int kAbortBatchRows = 2048;
    for (int batch0 = 0; batch0 < height; batch0 += kAbortBatchRows) {
        run_range(batch0, std::min(height, batch0 + kAbortBatchRows));
        if (abortRequested(settings)) throw RenderCancelled{};
    }
}

[[maybe_unused]] inline void lerp4(const float* a, const float* b, float t, float* out) noexcept {
    out[0] = a[0] + (b[0] - a[0]) * t;
    out[1] = a[1] + (b[1] - a[1]) * t;
    out[2] = a[2] + (b[2] - a[2]) * t;
    out[3] = a[3] + (b[3] - a[3]) * t;
}

struct CachedHorizontalRow {
    int source_y = -1;
    std::vector<float> pixels;
};

using BicubicRowCache = std::array<CachedHorizontalRow, 4>;

BicubicRowCache& reusableBicubicRowCache(std::size_t values_per_row) {
    // On macOS CPU rendering is dispatched through GCD. A TLS cache means each
    // worker pays for row scratch only on first use / width growth, not every
    // frame, while avoiding shared locks between MFR frames.
    thread_local BicubicRowCache cache;
    for (auto& slot : cache) {
        slot.source_y = -1;
        slot.pixels.resize(values_per_row);
    }
    return cache;
}

} // namespace

RenderCancelled::~RenderCancelled() = default;

void prepareWarpRGBAfInto(PreparedWarpRGBAf& p,
                          int src_width, int src_height,
                          int dst_width, int dst_height,
                          const WarpAxisLUT& x_lut,
                          const WarpAxisLUT& y_lut,
                          const RenderSettings& settings) {
    if (src_width <= 0 || src_height <= 0 || dst_width <= 0 || dst_height <= 0)
        throw std::invalid_argument("invalid dimensions");
    if (static_cast<int>(x_lut.source_u.size()) != dst_width ||
        static_cast<int>(y_lut.source_u.size()) != dst_height)
        throw std::invalid_argument("LUT dimensions must match output");

    p.src_width = src_width; p.src_height = src_height;
    p.dst_width = dst_width; p.dst_height = dst_height;
    p.quality = settings.quality;
    p.identity = src_width == dst_width && src_height == dst_height &&
                 axisIsIdentity(x_lut, dst_width) && axisIsIdentity(y_lut, dst_height);

    if (settings.quality == SampleQuality::Bilinear) {
        p.x_cubic.clear(); p.y_cubic.clear();
        p.x_linear.resize(static_cast<std::size_t>(dst_width));
        p.y_linear.resize(static_cast<std::size_t>(dst_height));
        for (int i = 0; i < dst_width; ++i)
            p.x_linear[static_cast<std::size_t>(i)] = makeLinear(x_lut.source_u[static_cast<std::size_t>(i)], src_width, settings.edge);
        for (int i = 0; i < dst_height; ++i)
            p.y_linear[static_cast<std::size_t>(i)] = makeLinear(y_lut.source_u[static_cast<std::size_t>(i)], src_height, settings.edge);
    } else {
        p.x_linear.clear(); p.y_linear.clear();
        p.x_cubic.resize(static_cast<std::size_t>(dst_width));
        p.y_cubic.resize(static_cast<std::size_t>(dst_height));
        for (int i = 0; i < dst_width; ++i)
            p.x_cubic[static_cast<std::size_t>(i)] = makeCubic(x_lut.source_u[static_cast<std::size_t>(i)], src_width, settings.edge);
        for (int i = 0; i < dst_height; ++i)
            p.y_cubic[static_cast<std::size_t>(i)] = makeCubic(y_lut.source_u[static_cast<std::size_t>(i)], src_height, settings.edge);
    }
}

PreparedWarpRGBAf prepareWarpRGBAf(int src_width, int src_height,
                                   int dst_width, int dst_height,
                                   const WarpAxisLUT& x_lut,
                                   const WarpAxisLUT& y_lut,
                                   const RenderSettings& settings) {
    PreparedWarpRGBAf p;
    prepareWarpRGBAfInto(p, src_width, src_height, dst_width, dst_height, x_lut, y_lut, settings);
    return p;
}

void renderWarpRGBAfPrepared(const ConstImageRGBAf& src,
                             const ImageRGBAf& dst,
                             const PreparedWarpRGBAf& p,
                             const RenderSettings& settings) {
    if (!src.data || !dst.data || src.width <= 0 || src.height <= 0 || dst.width <= 0 || dst.height <= 0)
        throw std::invalid_argument("invalid image view");
    if (src.width != p.src_width || src.height != p.src_height ||
        dst.width != p.dst_width || dst.height != p.dst_height || settings.quality != p.quality)
        throw std::invalid_argument("prepared warp does not match image/settings");

    if (p.identity) {
        if (src.data == dst.data && src.row_stride_floats == dst.row_stride_floats) return;
        auto copy_rows = [&](int y0, int y1) {
            const std::size_t bytes = static_cast<std::size_t>(dst.width) * 4u * sizeof(float);
            for (int y = y0; y < y1; ++y) {
                const float* in = src.data + static_cast<std::ptrdiff_t>(y) * src.row_stride_floats;
                float* out = dst.data + static_cast<std::ptrdiff_t>(y) * dst.row_stride_floats;
                std::memmove(out, in, bytes);
            }
        };
        parallelRows(dst.height, settings, copy_rows);
        return;
    }

    if (p.quality == SampleQuality::Bilinear) {
        auto render_rows = [&](int y0, int y1) {
            for (int y = y0; y < y1; ++y) {
                const auto ys = p.y_linear[static_cast<std::size_t>(y)];
                const float* r0 = src.data + static_cast<std::ptrdiff_t>(ys.i0) * src.row_stride_floats;
                const float* r1 = src.data + static_cast<std::ptrdiff_t>(ys.i1) * src.row_stride_floats;
                float* out = dst.data + static_cast<std::ptrdiff_t>(y) * dst.row_stride_floats;
                for (int x = 0; x < dst.width; ++x) {
                    const auto xs = p.x_linear[static_cast<std::size_t>(x)];
                    const float* p00 = r0 + xs.i0 * 4;
                    const float* p10 = r0 + xs.i1 * 4;
                    const float* p01 = r1 + xs.i0 * 4;
                    const float* p11 = r1 + xs.i1 * 4;
#if ELASTICGRID_SIMD_PIXELS
                    const auto a00 = simd::load4(p00);
                    const auto a10 = simd::load4(p10);
                    const auto a01 = simd::load4(p01);
                    const auto a11 = simd::load4(p11);
                    const auto top = simd::lerp(a00, a10, xs.t);
                    const auto bottom = simd::lerp(a01, a11, xs.t);
                    simd::store4(out + x * 4, simd::lerp(top, bottom, ys.t));
#else
                    float a[4], b[4];
                    lerp4(p00, p10, xs.t, a);
                    lerp4(p01, p11, xs.t, b);
                    lerp4(a, b, ys.t, out + x * 4);
#endif
                }
            }
        };
        parallelRows(dst.height, settings, render_rows);
        return;
    }

    auto render_rows = [&](int y0, int y1) {
        // Bicubic is separable. Cache horizontally-resampled source rows so
        // adjacent output rows reuse 3/4 of the expensive horizontal work.
        // Each parallel worker owns its cache, so this remains MFR/thread safe.
        auto& cache = reusableBicubicRowCache(static_cast<std::size_t>(dst.width) * 4u);
        auto horizontal_row = [&](int source_y, const std::array<int,4>& required) -> const float* {
            for (auto& slot : cache) {
                if (slot.source_y == source_y) return slot.pixels.data();
            }
            // Never evict a row that is required by the current output row;
            // pointers returned earlier in this same iteration must stay valid.
            std::size_t victim = 0;
            for (; victim < cache.size(); ++victim) {
                if (std::find(required.begin(), required.end(), cache[victim].source_y) == required.end()) break;
            }
            if (victim == cache.size()) victim = 0; // all required means this should have been a cache hit
            auto& slot = cache[victim];
            const float* source = src.data + static_cast<std::ptrdiff_t>(source_y) * src.row_stride_floats;
            float* row_out = slot.pixels.data();
            for (int x = 0; x < dst.width; ++x) {
                const auto& xs = p.x_cubic[static_cast<std::size_t>(x)];
#if ELASTICGRID_SIMD_PIXELS
                auto accv = simd::zero();
                for (std::size_t kx = 0; kx < 4u; ++kx)
                    accv = simd::madd(accv, simd::load4(source + xs.index[kx] * 4), xs.weight[kx]);
                simd::store4(row_out + x * 4, accv);
#else
                float* d = row_out + x * 4;
                d[0]=d[1]=d[2]=d[3]=0.0f;
                for (std::size_t kx = 0; kx < 4u; ++kx) {
                    const float w = xs.weight[kx];
                    const float* q = source + xs.index[kx] * 4;
                    d[0]+=q[0]*w; d[1]+=q[1]*w; d[2]+=q[2]*w; d[3]+=q[3]*w;
                }
#endif
            }
            slot.source_y = source_y;
            return slot.pixels.data();
        };

        for (int y = y0; y < y1; ++y) {
            const auto& ys = p.y_cubic[static_cast<std::size_t>(y)];
            const std::array<int,4> required{ys.index[0], ys.index[1], ys.index[2], ys.index[3]};
            const float* rows[4] = {
                horizontal_row(ys.index[0], required), horizontal_row(ys.index[1], required),
                horizontal_row(ys.index[2], required), horizontal_row(ys.index[3], required)
            };
            float* out = dst.data + static_cast<std::ptrdiff_t>(y) * dst.row_stride_floats;
            for (int x = 0; x < dst.width; ++x) {
#if ELASTICGRID_SIMD_PIXELS
                auto accv = simd::zero();
                accv = simd::madd(accv, simd::load4(rows[0] + x * 4), ys.weight[0]);
                accv = simd::madd(accv, simd::load4(rows[1] + x * 4), ys.weight[1]);
                accv = simd::madd(accv, simd::load4(rows[2] + x * 4), ys.weight[2]);
                accv = simd::madd(accv, simd::load4(rows[3] + x * 4), ys.weight[3]);
                simd::store4(out + x * 4, accv);
#else
                float* d = out + x * 4;
                d[0]=d[1]=d[2]=d[3]=0.0f;
                for (int ky = 0; ky < 4; ++ky) {
                    const float w = ys.weight[ky];
                    const float* q = rows[ky] + x * 4;
                    d[0]+=q[0]*w; d[1]+=q[1]*w; d[2]+=q[2]*w; d[3]+=q[3]*w;
                }
#endif
            }
        }
    };
    parallelRows(dst.height, settings, render_rows);
}


template <typename T, typename Src, typename Dst>
static void renderIntegerPreparedImpl(const Src& src,
                                      const Dst& dst,
                                      const PreparedWarpRGBAf& p,
                                      const RenderSettings& settings,
                                      T channel_max) {
    if (!src.data || !dst.data || src.width <= 0 || src.height <= 0 || dst.width <= 0 || dst.height <= 0)
        throw std::invalid_argument("invalid integer image view");
    if (src.width != p.src_width || src.height != p.src_height ||
        dst.width != p.dst_width || dst.height != p.dst_height || settings.quality != p.quality)
        throw std::invalid_argument("prepared warp does not match integer image/settings");

    if (p.identity) {
        if (src.data == dst.data && src.row_stride_values == dst.row_stride_values) return;
        auto copy_rows = [&](int y0, int y1) {
            const std::size_t bytes = static_cast<std::size_t>(dst.width) * 4u * sizeof(T);
            for (int y = y0; y < y1; ++y) {
                const T* in = src.data + static_cast<std::ptrdiff_t>(y) * src.row_stride_values;
                T* out = dst.data + static_cast<std::ptrdiff_t>(y) * dst.row_stride_values;
                std::memmove(out, in, bytes);
            }
        };
        parallelRows(dst.height, settings, copy_rows);
        return;
    }

    [[maybe_unused]] auto store = [channel_max](float v) -> T {
        v = std::clamp(v, 0.0f, static_cast<float>(channel_max));
        return static_cast<T>(v + 0.5f);
    };

    if (p.quality == SampleQuality::Bilinear) {
        auto render_rows = [&](int y0, int y1) {
            for (int y = y0; y < y1; ++y) {
                const auto ys = p.y_linear[static_cast<std::size_t>(y)];
                const T* r0 = src.data + static_cast<std::ptrdiff_t>(ys.i0) * src.row_stride_values;
                const T* r1 = src.data + static_cast<std::ptrdiff_t>(ys.i1) * src.row_stride_values;
                T* out = dst.data + static_cast<std::ptrdiff_t>(y) * dst.row_stride_values;
                for (int x = 0; x < dst.width; ++x) {
                    const auto xs = p.x_linear[static_cast<std::size_t>(x)];
                    const T* p00 = r0 + xs.i0 * 4;
                    const T* p10 = r0 + xs.i1 * 4;
                    const T* p01 = r1 + xs.i0 * 4;
                    const T* p11 = r1 + xs.i1 * 4;
#if ELASTICGRID_SIMD_PIXELS
                    if constexpr (std::is_same_v<T, std::uint8_t> || std::is_same_v<T, std::uint16_t>) {
                        const auto a00 = simd::load4(p00);
                        const auto a10 = simd::load4(p10);
                        const auto a01 = simd::load4(p01);
                        const auto a11 = simd::load4(p11);
                        const auto top = simd::lerp(a00, a10, xs.t);
                        const auto bottom = simd::lerp(a01, a11, xs.t);
                        const auto value = simd::lerp(top, bottom, ys.t);
                        simd::store4(out + x * 4, value, static_cast<float>(channel_max));
                        continue;
                    }
#endif
                    constexpr std::uint32_t kOne = 65536u;
                    constexpr std::uint32_t kHalf = 32768u;
                    const std::uint32_t wx1 = xs.t_fixed16;
                    const std::uint32_t wx0 = kOne - wx1;
                    const std::uint32_t wy1 = ys.t_fixed16;
                    const std::uint32_t wy0 = kOne - wy1;
                    for (int c = 0; c < 4; ++c) {
                        const std::uint32_t top =
                            (static_cast<std::uint32_t>(p00[c]) * wx0 +
                             static_cast<std::uint32_t>(p10[c]) * wx1 + kHalf) >> 16;
                        const std::uint32_t bottom =
                            (static_cast<std::uint32_t>(p01[c]) * wx0 +
                             static_cast<std::uint32_t>(p11[c]) * wx1 + kHalf) >> 16;
                        const std::uint32_t value =
                            (top * wy0 + bottom * wy1 + kHalf) >> 16;
                        out[x * 4 + c] = static_cast<T>(std::min<std::uint32_t>(value, channel_max));
                    }
                }
            }
        };
        parallelRows(dst.height, settings, render_rows);
        return;
    }

    auto render_rows = [&](int y0, int y1) {
        // Same separable cubic math as the float path. Horizontal rows are
        // cached as float so repeated vertical samples avoid reloading and
        // reconverting 8/16-bit source pixels. Final quantization happens once.
        auto& cache = reusableBicubicRowCache(static_cast<std::size_t>(dst.width) * 4u);

        auto horizontal_row = [&](int source_y, const std::array<int,4>& required) -> const float* {
            for (auto& slot : cache) {
                if (slot.source_y == source_y) return slot.pixels.data();
            }
            std::size_t victim = 0;
            for (; victim < cache.size(); ++victim) {
                if (std::find(required.begin(), required.end(), cache[victim].source_y) == required.end()) break;
            }
            if (victim == cache.size()) victim = 0;
            auto& slot = cache[victim];
            const T* source = src.data + static_cast<std::ptrdiff_t>(source_y) * src.row_stride_values;
            float* row_out = slot.pixels.data();
            for (int x = 0; x < dst.width; ++x) {
                const auto& xs = p.x_cubic[static_cast<std::size_t>(x)];
#if ELASTICGRID_SIMD_PIXELS
                auto accv = simd::zero();
                for (std::size_t kx = 0; kx < 4u; ++kx)
                    accv = simd::madd(accv, simd::load4(source + xs.index[kx] * 4), xs.weight[kx]);
                simd::store4(row_out + x * 4, accv);
#else
                float* d = row_out + x * 4;
                d[0]=d[1]=d[2]=d[3]=0.0f;
                for (std::size_t kx = 0; kx < 4u; ++kx) {
                    const float w = xs.weight[kx];
                    const T* q = source + xs.index[kx] * 4;
                    d[0]+=static_cast<float>(q[0])*w; d[1]+=static_cast<float>(q[1])*w;
                    d[2]+=static_cast<float>(q[2])*w; d[3]+=static_cast<float>(q[3])*w;
                }
#endif
            }
            slot.source_y = source_y;
            return slot.pixels.data();
        };

        for (int y = y0; y < y1; ++y) {
            const auto& ys = p.y_cubic[static_cast<std::size_t>(y)];
            const std::array<int,4> required{ys.index[0], ys.index[1], ys.index[2], ys.index[3]};
            const float* rows[4] = {
                horizontal_row(ys.index[0], required), horizontal_row(ys.index[1], required),
                horizontal_row(ys.index[2], required), horizontal_row(ys.index[3], required)
            };
            T* out = dst.data + static_cast<std::ptrdiff_t>(y) * dst.row_stride_values;
            for (int x = 0; x < dst.width; ++x) {
#if ELASTICGRID_SIMD_PIXELS
                auto accv = simd::zero();
                accv = simd::madd(accv, simd::load4(rows[0] + x * 4), ys.weight[0]);
                accv = simd::madd(accv, simd::load4(rows[1] + x * 4), ys.weight[1]);
                accv = simd::madd(accv, simd::load4(rows[2] + x * 4), ys.weight[2]);
                accv = simd::madd(accv, simd::load4(rows[3] + x * 4), ys.weight[3]);
                simd::store4(out + x * 4, accv, static_cast<float>(channel_max));
#else
                float acc[4] = {0,0,0,0};
                for (int ky = 0; ky < 4; ++ky) {
                    const float w = ys.weight[ky];
                    const float* q = rows[ky] + x * 4;
                    acc[0]+=q[0]*w; acc[1]+=q[1]*w; acc[2]+=q[2]*w; acc[3]+=q[3]*w;
                }
                for (int c=0;c<4;++c) out[x*4+c]=store(acc[c]);
#endif
            }
        }
    };
    parallelRows(dst.height, settings, render_rows);
}

void renderWarpRGBA8Prepared(const ConstImageRGBA8& src,
                             const ImageRGBA8& dst,
                             const PreparedWarpRGBAf& prepared,
                             const RenderSettings& settings) {
    renderIntegerPreparedImpl<std::uint8_t>(src, dst, prepared, settings, 255);
}

void renderWarpRGBA16Prepared(const ConstImageRGBA16& src,
                              const ImageRGBA16& dst,
                              const PreparedWarpRGBAf& prepared,
                              const RenderSettings& settings,
                              std::uint16_t channel_max) {
    renderIntegerPreparedImpl<std::uint16_t>(src, dst, prepared, settings, channel_max);
}

void renderWarpRGBAf(const ConstImageRGBAf& src,
                     const ImageRGBAf& dst,
                     const WarpAxisLUT& x_lut,
                     const WarpAxisLUT& y_lut,
                     const RenderSettings& settings) {
    auto prepared = prepareWarpRGBAf(src.width, src.height, dst.width, dst.height,
                                    x_lut, y_lut, settings);
    renderWarpRGBAfPrepared(src, dst, prepared, settings);
}

} // namespace elasticgrid
