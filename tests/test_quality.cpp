#include "core/GridModel.h"
#include "core/WarpMath.h"
#include "core/CpuRenderer.h"

#include <algorithm>
#include <array>
#include <cmath>
#include <cstdlib>
#include <iostream>
#include <vector>

using namespace elasticgrid;

static void fail(const char* msg) {
    std::cerr << "FAIL: " << msg << '\n';
    std::exit(1);
}
static void expect(bool ok, const char* msg) { if (!ok) fail(msg); }

static int wrap_ref(int i, int n) {
    if (n <= 1) return 0;
    i %= n;
    if (i < 0) i += n;
    return i;
}
static int mirror_ref(int i, int n) {
    if (n <= 1) return 0;
    const int period = 2 * n - 2;
    const int x = wrap_ref(i, period);
    return x < n ? x : period - x;
}
static int resolve_ref(int i, int n, EdgeMode edge) {
    switch (edge) {
        case EdgeMode::Clamp: return std::clamp(i, 0, n - 1);
        case EdgeMode::Wrap: return wrap_ref(i, n);
        case EdgeMode::Mirror: return mirror_ref(i, n);
        case EdgeMode::None: return i >= 0 && i < n ? i : -1;
    }
    return std::clamp(i, 0, n - 1);
}
static double cubic_weight_ref(double x) {
    constexpr double a = -0.5;
    x = std::abs(x);
    if (x < 1.0) return (a + 2.0) * x*x*x - (a + 3.0) * x*x + 1.0;
    if (x < 2.0) return a * x*x*x - 5.0*a*x*x + 8.0*a*x - 4.0*a;
    return 0.0;
}

static std::array<double, 4> reference_bicubic(
    const std::vector<float>& src, int sw, int sh,
    double u, double v, EdgeMode edge) {
    const double px = u * double(std::max(0, sw - 1));
    const double py = v * double(std::max(0, sh - 1));
    const int bx = int(std::floor(px));
    const int by = int(std::floor(py));

    double wx[4], wy[4], sx = 0.0, sy = 0.0;
    int ix[4], iy[4];
    for (int k = 0; k < 4; ++k) {
        const int rx = bx + k - 1;
        const int ry = by + k - 1;
        ix[k] = resolve_ref(rx, sw, edge);
        iy[k] = resolve_ref(ry, sh, edge);
        wx[k] = cubic_weight_ref(px - double(rx));
        wy[k] = cubic_weight_ref(py - double(ry));
        sx += wx[k]; sy += wy[k];
    }
    if (std::abs(sx) > 1e-15) for (double& w : wx) w /= sx;
    if (std::abs(sy) > 1e-15) for (double& w : wy) w /= sy;

    std::array<double,4> out{0,0,0,0};
    for (int ky = 0; ky < 4; ++ky) {
        for (int kx = 0; kx < 4; ++kx) {
            const double w = wy[ky] * wx[kx];
            if (iy[ky] < 0 || ix[kx] < 0) continue;
            const float* q = src.data() + (iy[ky] * sw + ix[kx]) * 4;
            for (int c = 0; c < 4; ++c) out[c] += double(q[c]) * w;
        }
    }
    return out;
}

template <typename T>
static std::array<double, 4> reference_bicubic_integer(
    const std::vector<T>& src, int sw, int sh,
    double u, double v, EdgeMode edge) {
    const double px = u * double(std::max(0, sw - 1));
    const double py = v * double(std::max(0, sh - 1));
    const int bx = int(std::floor(px));
    const int by = int(std::floor(py));

    double wx[4], wy[4], sx = 0.0, sy = 0.0;
    int ix[4], iy[4];
    for (int k = 0; k < 4; ++k) {
        const int rx = bx + k - 1;
        const int ry = by + k - 1;
        ix[k] = resolve_ref(rx, sw, edge);
        iy[k] = resolve_ref(ry, sh, edge);
        wx[k] = cubic_weight_ref(px - double(rx));
        wy[k] = cubic_weight_ref(py - double(ry));
        sx += wx[k]; sy += wy[k];
    }
    if (std::abs(sx) > 1e-15) for (double& w : wx) w /= sx;
    if (std::abs(sy) > 1e-15) for (double& w : wy) w /= sy;

    std::array<double,4> out{0,0,0,0};
    for (int ky = 0; ky < 4; ++ky) {
        for (int kx = 0; kx < 4; ++kx) {
            const double w = wy[ky] * wx[kx];
            if (iy[ky] < 0 || ix[kx] < 0) continue;
            const T* q = src.data() + (iy[ky] * sw + ix[kx]) * 4;
            for (int c = 0; c < 4; ++c) out[c] += double(q[c]) * w;
        }
    }
    return out;
}

static long long quantize_ref(double v, double maxv) {
    v = std::clamp(v, 0.0, maxv);
    return static_cast<long long>(std::nearbyint(v));
}

int main() {
    constexpr int SW = 19, SH = 13, DW = 23, DH = 17;
    std::vector<float> src(SW * SH * 4);
    for (int y = 0; y < SH; ++y) {
        for (int x = 0; x < SW; ++x) {
            float* p = src.data() + (y * SW + x) * 4;
            p[0] = -0.2f + 1.4f * std::sin(0.17f * float(x + 2*y));
            p[1] = 0.03f * float((7*x + 11*y) % 31);
            p[2] = 0.07f * float((13*x + 5*y) % 17);
            p[3] = 0.1f + 0.9f * float((x + 3*y) % 9) / 8.0f;
        }
    }

    AxisGrid gx(5), gy(4);
    ElasticSettings es; es.radius_lines = 2.75f; es.strength = 1.2f; es.min_spacing = 0.002f;
    expect(gx.dragElastic(2, 0.31f, es), "x drag");
    expect(gx.dragElastic(4, 0.88f, es), "x drag 2");
    expect(gy.dragElastic(2, 0.66f, es), "y drag");
    const auto xl = buildInverseLUT(gx.lines(), DW, 0.61f, 0.19f);
    const auto yl = buildInverseLUT(gy.lines(), DH, 0.61f, 0.19f);

    for (EdgeMode edge : {EdgeMode::Clamp, EdgeMode::Wrap, EdgeMode::Mirror, EdgeMode::None}) {
        RenderSettings rs;
        rs.quality = SampleQuality::Bicubic;
        rs.edge = edge;
        rs.threads = 1;
        std::vector<float> dst(DW * DH * 4, 0.0f);
        auto prep = prepareWarpRGBAf(SW, SH, DW, DH, xl, yl, rs);
        renderWarpRGBAfPrepared({src.data(), SW, SH, SW*4}, {dst.data(), DW, DH, DW*4}, prep, rs);

        double max_err = 0.0;
        for (int y = 0; y < DH; ++y) {
            for (int x = 0; x < DW; ++x) {
                const auto ref = reference_bicubic(src, SW, SH,
                    xl.source_u[static_cast<std::size_t>(x)],
                    yl.source_u[static_cast<std::size_t>(y)], edge);
                const float* got = dst.data() + (y * DW + x) * 4;
                for (int c = 0; c < 4; ++c) {
                    max_err = std::max(max_err, std::abs(double(got[c]) - ref[c]));
                    expect(std::isfinite(got[c]), "final bicubic finite");
                }
            }
        }
        if (max_err > 3.0e-6) {
            std::cerr << "bicubic reference max_err=" << max_err << '\n';
            return 2;
        }
    }

    // Integer final-quality regression against an independent double-precision
    // bicubic reference. SIMD implementations may differ by at most one LSB
    // because production weights are stored as float.
    std::vector<std::uint8_t> src8(SW * SH * 4);
    std::vector<std::uint16_t> src16(SW * SH * 4);
    for (int y = 0; y < SH; ++y) {
        for (int x = 0; x < SW; ++x) {
            const int i = (y * SW + x) * 4;
            src8[i+0] = static_cast<std::uint8_t>((17*x + 29*y + 11) & 255);
            src8[i+1] = static_cast<std::uint8_t>((71*x + 13*y + 37) & 255);
            src8[i+2] = static_cast<std::uint8_t>((19*x + 83*y + 101) & 255);
            src8[i+3] = static_cast<std::uint8_t>(32 + ((7*x + 5*y) % 224));
            src16[i+0] = static_cast<std::uint16_t>((997*x + 1597*y + 431) % 32769);
            src16[i+1] = static_cast<std::uint16_t>((1877*x + 719*y + 2203) % 32769);
            src16[i+2] = static_cast<std::uint16_t>((313*x + 2671*y + 7919) % 32769);
            src16[i+3] = static_cast<std::uint16_t>(4096 + ((541*x + 829*y) % 28673));
        }
    }
    for (EdgeMode edge : {EdgeMode::Clamp, EdgeMode::Wrap, EdgeMode::Mirror, EdgeMode::None}) {
        RenderSettings rs; rs.quality = SampleQuality::Bicubic; rs.edge = edge; rs.threads = 1;
        auto prep = prepareWarpRGBAf(SW, SH, DW, DH, xl, yl, rs);
        std::vector<std::uint8_t> dst8(DW * DH * 4, 0);
        std::vector<std::uint16_t> dst16(DW * DH * 4, 0);
        renderWarpRGBA8Prepared({src8.data(), SW, SH, SW*4}, {dst8.data(), DW, DH, DW*4}, prep, rs);
        renderWarpRGBA16Prepared({src16.data(), SW, SH, SW*4}, {dst16.data(), DW, DH, DW*4}, prep, rs, 32768);
        int max_err8 = 0, max_err16 = 0;
        for (int y = 0; y < DH; ++y) {
            for (int x = 0; x < DW; ++x) {
                const double u = xl.source_u[static_cast<std::size_t>(x)];
                const double v = yl.source_u[static_cast<std::size_t>(y)];
                const auto ref8 = reference_bicubic_integer(src8, SW, SH, u, v, edge);
                const auto ref16 = reference_bicubic_integer(src16, SW, SH, u, v, edge);
                for (int c = 0; c < 4; ++c) {
                    const int got8 = dst8[(y*DW+x)*4+c];
                    const int got16 = dst16[(y*DW+x)*4+c];
                    max_err8 = std::max(max_err8, std::abs(got8 - int(quantize_ref(ref8[c], 255.0))));
                    max_err16 = std::max(max_err16, std::abs(got16 - int(quantize_ref(ref16[c], 32768.0))));
                }
            }
        }
        if (max_err8 > 1 || max_err16 > 1) {
            std::cerr << "integer bicubic reference errors: 8=" << max_err8
                      << " 16=" << max_err16 << '\n';
            return 3;
        }
    }

    // Constant-field invariance: normalized cubic weights must not introduce
    // brightness/alpha drift, including at all edge modes.
    std::vector<float> constant(SW * SH * 4);
    for (int i = 0; i < SW * SH; ++i) {
        constant[i*4+0] = -0.125f;
        constant[i*4+1] = 0.25f;
        constant[i*4+2] = 1.75f;
        constant[i*4+3] = 0.625f;
    }
    for (EdgeMode edge : {EdgeMode::Clamp, EdgeMode::Wrap, EdgeMode::Mirror}) {
        RenderSettings rs; rs.quality = SampleQuality::Bicubic; rs.edge = edge; rs.threads = 1;
        std::vector<float> dst(DW * DH * 4, 0.0f);
        auto prep = prepareWarpRGBAf(SW, SH, DW, DH, xl, yl, rs);
        renderWarpRGBAfPrepared({constant.data(), SW, SH, SW*4}, {dst.data(), DW, DH, DW*4}, prep, rs);
        const float expected[4] = {-0.125f, 0.25f, 1.75f, 0.625f};
        for (int i = 0; i < DW * DH; ++i)
            for (int c = 0; c < 4; ++c)
                expect(std::abs(dst[i*4+c] - expected[c]) <= 2.0e-6f, "constant field invariant");
    }

    // Exact identity must be byte-for-byte for 32f because the prepared path
    // intentionally bypasses interpolation in this case.
    AxisGrid ux(4), uy(4);
    const auto ixl = buildInverseLUT(ux.lines(), SW, 0.0f, 0.25f);
    const auto iyl = buildInverseLUT(uy.lines(), SH, 0.0f, 0.25f);
    RenderSettings identity_rs; identity_rs.quality = SampleQuality::Bicubic; identity_rs.edge = EdgeMode::Mirror; identity_rs.threads = 1;
    std::vector<float> identity(src.size(), 0.0f);
    auto identity_plan = prepareWarpRGBAf(SW, SH, SW, SH, ixl, iyl, identity_rs);
    renderWarpRGBAfPrepared({src.data(), SW, SH, SW*4}, {identity.data(), SW, SH, SW*4}, identity_plan, identity_rs);
    expect(identity == src, "32f identity exact");

    std::cout << "elasticgrid_quality_tests: OK\n";
    return 0;
}
