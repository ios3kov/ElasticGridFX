#include "bridge/elasticgrid_ffi.h"
#include <algorithm>
#include <atomic>
#include <cassert>
#include <cmath>
#include <cstdint>
#include <limits>
#include <vector>

template <typename T>
static void fill_pattern(std::vector<T>& src, int w, int h, T maxv) {
    for (int y = 0; y < h; ++y) {
        for (int x = 0; x < w; ++x) {
            const auto i = static_cast<std::size_t>((y * w + x) * 4);
            src[i + 0] = maxv;
            src[i + 1] = static_cast<T>((static_cast<std::uint64_t>(x) * maxv) / (w - 1));
            src[i + 2] = static_cast<T>((static_cast<std::uint64_t>(y) * maxv) / (h - 1));
            src[i + 3] = static_cast<T>(maxv / 4);
        }
    }
}

static EgRenderParams defaults() {
    EgRenderParams p{};
    p.columns = 4;
    p.rows = 4;
    p.tension_radius = 3.0f;
    p.falloff = 2;
    p.elasticity_strength = 1.0f;
    p.min_spacing = 0.005f;
    p.stretch_easing = 0.0f;
    p.easing_distance = 0.25f;
    p.wave_enabled = 0;
    p.wave_frequency = 1.0f;
    p.wave_axis = 1;
    p.edge_mode = 1;
    p.quality = 1;
    p.threads = 1;
    return p;
}

static std::int32_t cancel_after_two(void* refcon) {
    auto* calls = static_cast<std::atomic<int>*>(refcon);
    return calls->fetch_add(1, std::memory_order_relaxed) >= 1 ? 1 : 0;
}

int main() {
    constexpr int w = 64;
    constexpr int h = 32;

    {
        // Direct-guide ABI: grabbed line follows the pointer, neighbors move,
        // endpoints remain pinned and ordering is preserved.
        float lines[] = {0.0f, 0.25f, 0.5f, 0.75f, 1.0f};
        std::uint8_t pins[] = {1, 0, 0, 0, 1};
        EgElasticParams ep{};
        ep.tension_radius = 3.0f;
        ep.falloff = 2;
        ep.elasticity_strength = 1.0f;
        ep.min_spacing = 0.01f;
        assert(eg_drag_axis(lines, pins, 5, 2, 0.65f, &ep) == 0);
        assert(std::abs(lines[2] - 0.65f) < 0.03f);
        assert(lines[1] > 0.25f);
        assert(lines[0] == 0.0f && lines[4] == 1.0f);
        for (int i = 1; i < 5; ++i) assert(lines[i] > lines[i - 1]);

        // Extreme drags must still preserve ordering and requested spacing.
        assert(eg_drag_axis(lines, pins, 5, 2, 0.999f, &ep) == 0);
        for (int i = 1; i < 5; ++i) assert(lines[i] - lines[i - 1] >= 0.0099f);
        assert(eg_drag_axis(lines, pins, 5, 2, 0.001f, &ep) == 0);
        for (int i = 1; i < 5; ++i) assert(lines[i] - lines[i - 1] >= 0.0099f);
    }

    {
        std::vector<std::uint8_t> src(w * h * 4), dst(src.size(), 0);
        fill_pattern(src, w, h, static_cast<std::uint8_t>(255));
        auto p = defaults();
        assert(eg_render_frame(src.data(), w * 4, w, h, dst.data(), w * 4, w, h, 8, &p) == 0);
        assert(src == dst);
        p.wave_enabled = 1;
        p.wave_amplitude = 0.05f;
        assert(eg_render_frame(src.data(), w * 4, w, h, dst.data(), w * 4, w, h, 8, &p) == 0);
        assert(src != dst);
    }

    {
        // A host-supplied guide state must drive the actual render, rather
        // than being a UI-only state blob.
        std::vector<std::uint8_t> src(w * h * 4), dst(src.size(), 0);
        fill_pattern(src, w, h, static_cast<std::uint8_t>(255));
        auto p = defaults();
        float x[] = {0.0f, 0.18f, 0.62f, 0.83f, 1.0f};
        float y[] = {0.0f, 0.25f, 0.5f, 0.75f, 1.0f};
        std::uint8_t xp[] = {1, 0, 0, 0, 1};
        std::uint8_t yp[] = {1, 0, 0, 0, 1};
        p.column_lines = x;
        p.column_line_count = 5;
        p.column_pins = xp;
        p.row_lines = y;
        p.row_line_count = 5;
        p.row_pins = yp;
        assert(eg_render_frame(src.data(), w * 4, w, h, dst.data(), w * 4, w, h, 8, &p) == 0);
        assert(src != dst);
    }

    {
        // True 2D mesh regression: move one interior intersection in both X
        // and Y. This must locally deform the image; a separable guide-only
        // implementation cannot satisfy this state representation.
        std::vector<std::uint8_t> src(w * h * 4), dst(src.size(), 0);
        fill_pattern(src, w, h, static_cast<std::uint8_t>(255));
        auto p = defaults();
        constexpr int columns = 4, rows = 4, stride = columns + 1;
        std::vector<float> px((columns + 1) * (rows + 1));
        std::vector<float> py(px.size());
        std::vector<std::uint8_t> pins(px.size(), 0);
        for (int row = 0; row <= rows; ++row) {
            for (int column = 0; column <= columns; ++column) {
                const int i = row * stride + column;
                px[static_cast<std::size_t>(i)] = static_cast<float>(column) / columns;
                py[static_cast<std::size_t>(i)] = static_cast<float>(row) / rows;
                pins[static_cast<std::size_t>(i)] =
                    static_cast<std::uint8_t>(column == 0 || column == columns || row == 0 || row == rows);
            }
        }
        const int center = 2 * stride + 2;
        px[static_cast<std::size_t>(center)] = 0.62f;
        py[static_cast<std::size_t>(center)] = 0.38f;
        p.column_lines = px.data(); p.column_line_count = static_cast<std::int32_t>(px.size()); p.column_pins = pins.data();
        p.row_lines = py.data(); p.row_line_count = static_cast<std::int32_t>(py.size()); p.row_pins = pins.data();
        p.canvas_width = w; p.canvas_height = h;
        assert(eg_render_frame(src.data(), w * 4, w, h, dst.data(), w * 4, w, h, 8, &p) == 0);
        assert(src != dst);

        const EgRectI32 roi{8, 4, 40, 24};
        EgRectI32 required{};
        assert(eg_required_source_rect(w, h, roi, &p, &required) == 0);
        assert(required.left == 0 && required.top == 0 && required.right == w && required.bottom == h);
        assert(eg_prepare_gpu_plan(w, h, w, h, &p, nullptr, nullptr, nullptr, nullptr) == 6);
    }

    {
        std::vector<std::uint16_t> src(w * h * 4), dst(src.size(), 0);
        fill_pattern(src, w, h, static_cast<std::uint16_t>(32768));
        auto p = defaults();
        assert(eg_render_frame(src.data(), w * 8, w, h, dst.data(), w * 8, w, h, 16, &p) == 0);
        assert(src == dst);
    }

    {
        std::vector<float> src(w * h * 4), dst(src.size(), 0.0f);
        for (int y = 0; y < h; ++y) for (int x = 0; x < w; ++x) {
            const auto i = static_cast<std::size_t>((y * w + x) * 4);
            src[i] = 1.0f; src[i+1] = float(x)/(w-1); src[i+2] = float(y)/(h-1); src[i+3] = .25f;
        }
        auto p = defaults();
        assert(eg_render_frame(src.data(), w * 16, w, h, dst.data(), w * 16, w, h, 32, &p) == 0);
        for (std::size_t i = 0; i < src.size(); ++i) assert(std::abs(src[i] - dst[i]) < 1e-6f);
    }


    {
        // Negative row stride is legal in AE worlds. Point data at the logical
        // first row and step backwards through the physical allocation.
        constexpr std::ptrdiff_t row = w * 4;
        std::vector<std::uint8_t> src(w * h * 4), dst(src.size(), 0);
        fill_pattern(src, w, h, static_cast<std::uint8_t>(255));
        auto p = defaults();
        const auto* src_top = src.data() + (h - 1) * row;
        auto* dst_top = dst.data() + (h - 1) * row;
        assert(eg_render_frame(src_top, -row, w, h, dst_top, -row, w, h, 8, &p) == 0);
        assert(src == dst);
    }

    {
        // Bicubic identity should preserve integer pixels exactly after final
        // rounding, and exercises the SIMD bicubic path where available.
        std::vector<std::uint8_t> src(w * h * 4), dst(src.size(), 0);
        fill_pattern(src, w, h, static_cast<std::uint8_t>(255));
        auto p = defaults();
        p.quality = 2;
        p.edge_mode = 3;
        assert(eg_render_frame(src.data(), w * 4, w, h, dst.data(), w * 4, w, h, 8, &p) == 0);
        assert(src == dst);
    }

    {
        // Thread partitioning must not change results.
        std::vector<std::uint8_t> src(w * h * 4), a(src.size(), 0), b(src.size(), 0);
        fill_pattern(src, w, h, static_cast<std::uint8_t>(255));
        auto p = defaults();
        p.wave_enabled = 1;
        p.wave_amplitude = 0.05f;
        p.wave_frequency = 2.0f;
        p.threads = 1;
        assert(eg_render_frame(src.data(), w * 4, w, h, a.data(), w * 4, w, h, 8, &p) == 0);
        p.threads = 4;
        assert(eg_render_frame(src.data(), w * 4, w, h, b.data(), w * 4, w, h, 8, &p) == 0);
        assert(a == b);
    }

    {
        std::vector<std::uint8_t> src(w * h * 4), dst(src.size());
        auto p = defaults();
        assert(eg_render_frame(src.data(), w * 4 - 1, w, h, dst.data(), w * 4, w, h, 8, &p) == 1);
        assert(eg_render_frame(src.data(), w * 4, w, h, dst.data(), w * 4, w, h, 12, &p) == 2);
        p.wave_enabled = 1;
        p.wave_amplitude = std::numeric_limits<float>::quiet_NaN();
        p.time_seconds = std::numeric_limits<float>::infinity();
        assert(eg_render_frame(src.data(), w * 4, w, h, dst.data(), w * 4, w, h, 8, &p) == 0);
    }

    {
        // SmartFX partial-ROI render must match the same crop from a full-frame
        // render. This guards origin_x/y and cache-tile correctness.
        constexpr int cw = 96, ch = 64;
        std::vector<std::uint8_t> full_src(cw * ch * 4), full_dst(cw * ch * 4, 0);
        fill_pattern(full_src, cw, ch, static_cast<std::uint8_t>(255));
        float x[] = {0.0f, 0.12f, 0.43f, 0.79f, 1.0f};
        float y[] = {0.0f, 0.21f, 0.58f, 0.84f, 1.0f};
        std::uint8_t pin[] = {1,0,0,0,1};
        auto full_p = defaults();
        full_p.quality = 2;
        full_p.column_lines=x; full_p.column_line_count=5; full_p.column_pins=pin;
        full_p.row_lines=y; full_p.row_line_count=5; full_p.row_pins=pin;
        full_p.canvas_width=cw; full_p.canvas_height=ch;
        assert(eg_render_frame(full_src.data(), cw*4, cw, ch, full_dst.data(), cw*4, cw, ch, 8, &full_p) == 0);

        const EgRectI32 out_roi{19, 11, 71, 49};
        EgRectI32 src_roi{};
        assert(eg_required_source_rect(cw, ch, out_roi, &full_p, &src_roi) == 0);
        assert(src_roi.left >= 0 && src_roi.top >= 0 && src_roi.right <= cw && src_roi.bottom <= ch);
        const int sw = src_roi.right-src_roi.left, sh = src_roi.bottom-src_roi.top;
        const int ow = out_roi.right-out_roi.left, oh = out_roi.bottom-out_roi.top;
        std::vector<std::uint8_t> crop_src(static_cast<std::size_t>(sw)*sh*4);
        std::vector<std::uint8_t> crop_dst(static_cast<std::size_t>(ow)*oh*4, 0);
        for (int yy=0; yy<sh; ++yy) {
            std::copy_n(full_src.data()+((src_roi.top+yy)*cw+src_roi.left)*4,
                        sw*4, crop_src.data()+yy*sw*4);
        }
        auto crop_p=full_p;
        crop_p.input_origin_x=src_roi.left; crop_p.input_origin_y=src_roi.top;
        crop_p.output_origin_x=out_roi.left; crop_p.output_origin_y=out_roi.top;
        assert(eg_render_frame(crop_src.data(), sw*4, sw, sh,
                               crop_dst.data(), ow*4, ow, oh, 8, &crop_p) == 0);
        for (int yy=0; yy<oh; ++yy) for (int xx=0; xx<ow; ++xx) for (int c=0;c<4;++c) {
            const auto a=crop_dst[(yy*ow+xx)*4+c];
            const auto b=full_dst[((out_roi.top+yy)*cw+(out_roi.left+xx))*4+c];
            assert(a==b);
        }
    }
    {
        // Uniform grid with matching non-zero input/output origins is an exact
        // identity even when the SmartFX worlds are subregions of the layer.
        constexpr int rw=23, rh=17;
        std::vector<std::uint8_t> src(rw*rh*4), dst(src.size(),0);
        fill_pattern(src,rw,rh,static_cast<std::uint8_t>(255));
        auto p=defaults(); p.canvas_width=100; p.canvas_height=80;
        p.input_origin_x=p.output_origin_x=31; p.input_origin_y=p.output_origin_y=9;
        assert(eg_render_frame(src.data(),rw*4,rw,rh,dst.data(),rw*4,rw,rh,8,&p)==0);
        assert(src==dst);
    }

    {
        // 1x1 source/output is a valid degenerate image for all bit depths and qualities.
        for (int quality = 1; quality <= 2; ++quality) {
            auto p = defaults(); p.quality = quality;
            std::uint8_t s8[4] = {255, 17, 29, 41}, d8[4] = {};
            assert(eg_render_frame(s8, 4, 1, 1, d8, 4, 1, 1, 8, &p) == 0);
            for (int i=0;i<4;++i) assert(s8[i] == d8[i]);
            std::uint16_t s16[4] = {32768, 17, 29, 41}, d16[4] = {};
            assert(eg_render_frame(s16, 8, 1, 1, d16, 8, 1, 1, 16, &p) == 0);
            for (int i=0;i<4;++i) assert(s16[i] == d16[i]);
            float s32[4] = {1.25f, -0.5f, 0.25f, 2.0f}, d32[4] = {};
            assert(eg_render_frame(s32, 16, 1, 1, d32, 16, 1, 1, 32, &p) == 0);
            for (int i=0;i<4;++i) assert(s32[i] == d32[i]);
        }
    }
    {
        // Maximum supported 128x128 topology, extreme spacing, and negative time.
        std::vector<float> lines(129);
        std::vector<std::uint8_t> pins(129, 0);
        for (int i=0;i<=128;++i) lines[static_cast<std::size_t>(i)] = float(i) / 128.0f;
        pins.front() = pins.back() = 1;
        auto p = defaults();
        p.columns = 128; p.rows = 128;
        p.column_lines = lines.data(); p.column_line_count = 129; p.column_pins = pins.data();
        p.row_lines = lines.data(); p.row_line_count = 129; p.row_pins = pins.data();
        p.min_spacing = 0.25f; // core clamps to feasible 1/128
        p.wave_enabled = 1; p.wave_amplitude = 0.25f; p.wave_frequency = 20.0f;
        p.wave_speed = -10.0f; p.time_seconds = -12345.75f; p.quality = 2;
        std::vector<float> src(64 * 64 * 4, 0.5f), dst(src.size(), 0.0f);
        assert(eg_render_frame(src.data(), 64*16, 64, 64, dst.data(), 64*16, 64, 64, 32, &p) == 0);
        for (float v : dst) assert(std::isfinite(v));
    }
    {
        // 8K plan creation is a cheap size/overflow gate without allocating a full 8K image.
        auto p = defaults(); p.quality = 2;
        std::vector<EgGpuCubicSample> xp(7680), yp(4320);
        assert(eg_prepare_gpu_plan(7680, 4320, 7680, 4320, &p,
                                   nullptr, nullptr, xp.data(), yp.data()) == 0);
        assert(xp.front().index[0] >= 0 && yp.front().index[0] >= 0);
    }

    {
        // FFI boundary must reject malformed/null inputs without throwing or mutating host memory.
        auto p = defaults();
        std::vector<std::uint8_t> src(w * h * 4, 17), dst(w * h * 4, 23);
        const auto before = dst;
        assert(eg_render_frame(nullptr, w * 4, w, h, dst.data(), w * 4, w, h, 8, &p) == 1);
        assert(dst == before);
        assert(eg_render_frame(src.data(), w * 4, w, h, nullptr, w * 4, w, h, 8, &p) == 1);
        assert(eg_render_frame(src.data(), w * 4, 0, h, dst.data(), w * 4, w, h, 8, &p) == 1);
        assert(eg_render_frame(src.data(), w * 4, w, h, dst.data(), w * 4, w, h, 8, nullptr) == 1);

        float lines[] = {0.0f, 0.5f, 1.0f};
        std::uint8_t pins[] = {1, 0, 1};
        EgElasticParams ep{3.0f, 2, 1.0f, 0.01f};
        const float original_mid = lines[1];
        assert(eg_drag_axis(nullptr, pins, 3, 1, 0.6f, &ep) == 1);
        assert(eg_drag_axis(lines, nullptr, 3, 1, 0.6f, &ep) == 1);
        assert(eg_drag_axis(lines, pins, 3, 1, std::numeric_limits<float>::quiet_NaN(), &ep) == 1);
        assert(lines[1] == original_mid);
    }

    {
        // Hostile ROI endpoints must not trigger signed-overflow UB before
        // dimension validation. The impossible rectangle is rejected.
        auto p = defaults();
        EgRectI32 source{};
        const EgRectI32 hostile{
            std::numeric_limits<std::int32_t>::min(),
            std::numeric_limits<std::int32_t>::min(),
            std::numeric_limits<std::int32_t>::max(),
            std::numeric_limits<std::int32_t>::max()};
        assert(eg_required_source_rect(64, 64, hostile, &p, &source) == 1);
    }

    {
        // CPU renderer must poll cancellation on the host thread between row
        // batches and return the dedicated cancellation status.
        constexpr int cw = 1024;
        constexpr int ch = 2304;
        std::vector<float> src(static_cast<std::size_t>(cw) * ch * 4, 0.25f);
        std::vector<float> dst(src.size(), 0.0f);
        auto p = defaults();
        p.quality = 2;
        p.wave_enabled = 1;
        p.wave_amplitude = 0.04f;
        p.threads = 4;
        std::atomic<int> polls{0};
        p.abort_fn = cancel_after_two;
        p.abort_refcon = &polls;
        const int rc = eg_render_frame(src.data(), cw * 16, cw, ch,
                                       dst.data(), cw * 16, cw, ch, 32, &p);
        assert(rc == 5);
        assert(polls.load(std::memory_order_relaxed) >= 2);
    }

    return 0;
}
