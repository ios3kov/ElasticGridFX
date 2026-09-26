#include "core/GridModel.h"
#include "core/GridCodec.h"
#include "core/WarpMath.h"
#include "core/CpuRenderer.h"
#include <algorithm>
#include <cmath>
#include <cstdlib>
#include <iostream>
#include <limits>
#include <vector>

using namespace elasticgrid;

static void expect(bool v, const char* msg) {
    if (!v) { std::cerr << "FAIL: " << msg << '\n'; std::exit(1); }
}

int main() {
    {
        AxisGrid g(4);
        expect(g.lineCount() == 5, "4 cells => 5 lines");
        expect(std::abs(g.lines()[2] - 0.5f) < 1e-6f, "uniform reset");
    }
    {
        AxisGrid g(8);
        ElasticSettings s;
        s.radius_lines = 3.0f;
        s.strength = 1.0f;
        s.min_spacing = 0.01f;
        expect(g.dragElastic(4, 0.65f, s), "drag accepted");
        expect(std::abs(g.lines()[4] - 0.65f) < 0.03f, "grabbed line follows target");
        for (std::size_t i = 1; i < g.lines().size(); ++i)
            expect(g.lines()[i] > g.lines()[i-1], "monotonic after elastic drag");
        expect(g.lines()[3] > 3.0f/8.0f, "neighbor follows drag");
    }
    {
        AxisGrid g(6);
        g.setPinned(2, true);
        const float before = g.lines()[2];
        ElasticSettings s;
        g.dragElastic(4, 0.8f, s);
        expect(std::abs(g.lines()[2] - before) < 1e-6f, "pinned line stays fixed");
    }
    {
        AxisGrid g(4);
        const float raw_lines[] = {0.0f, 0.20f, 0.58f, 0.82f, 1.0f};
        const std::uint8_t raw_pins[] = {1, 0, 1, 0, 1};
        expect(g.setState(raw_lines, raw_pins, 0.005f), "bulk state import");
        expect(std::abs(g.lines()[2] - 0.58f) < 1e-6f, "bulk state positions");
        expect(g.pins()[2] == 1, "bulk state pins");
    }
    {
        AxisGrid g(4);
        auto lut = buildInverseLUT(g.lines(), 17);
        for (int i = 0; i < 17; ++i) {
            float u = static_cast<float>(i) / 16.0f;
            expect(std::abs(lut.source_u[static_cast<std::size_t>(i)] - u) < 1e-5f,
                   "identity grid yields identity LUT");
        }
    }
    {
        AxisGrid g(4);
        WaveSettings w;
        w.enabled = true;
        w.amplitude = 0.04f;
        w.frequency = 2.0f;
        auto e = g.evaluated(w, 0.25f, 0.005f, true);
        for (std::size_t i = 1; i < e.size(); ++i)
            expect(e[i] > e[i-1], "wave evaluation remains monotonic");
    }
    {
        GridState s{AxisGrid(7), AxisGrid(5)};
        ElasticSettings es; es.radius_lines=2.5f; es.min_spacing=.002f;
        s.columns.dragElastic(3, .51f, es);
        s.rows.dragElastic(2, .31f, es);
        s.columns.setPinned(2, true);
        auto blob = encodeGridState(s);
        GridState d;
        expect(decodeGridState(blob, d), "grid codec roundtrip decodes");
        expect(d.columns.cells() == 7 && d.rows.cells() == 5, "grid codec topology");
        expect(d.columns.pins()[2] == 1, "grid codec pins");
        for (std::size_t i=0;i<s.columns.lines().size();++i)
            expect(std::abs(s.columns.lines()[i]-d.columns.lines()[i]) < 1e-6f, "grid codec column positions");
        blob[blob.size()/2] ^= 0x55;
        expect(!decodeGridState(blob, d), "grid codec rejects corruption");
    }
    {
        GridState a{AxisGrid(4), AxisGrid(4)};
        GridState b{AxisGrid(4), AxisGrid(4)};
        b.columns.setLinePosition(2, .65f, .001f);
        auto m = interpolateGridState(a,b,.5f,.001f);
        expect(std::abs(m.columns.lines()[2]-.575f) < 1e-4f, "grid state interpolation");
    }

    {
        // Topology bounds: 0 -> minimum one cell, oversized -> hard max 128.
        AxisGrid min_grid(0), max_grid(10000);
        expect(min_grid.cells() == 1 && min_grid.lineCount() == 2, "minimum topology clamps to 1 cell");
        expect(max_grid.cells() == 128 && max_grid.lineCount() == 129, "maximum topology clamps to 128 cells");
    }
    {
        // NaN/Inf must not poison the persistent grid state.
        AxisGrid g(4);
        const auto before = g.lines();
        ElasticSettings bad;
        bad.radius_lines = std::numeric_limits<float>::quiet_NaN();
        bad.strength = std::numeric_limits<float>::quiet_NaN();
        bad.min_spacing = std::numeric_limits<float>::quiet_NaN();
        expect(!g.dragElastic(2, std::numeric_limits<float>::quiet_NaN(), bad), "NaN drag rejected");
        expect(g.lines() == before, "NaN drag leaves grid unchanged");
        expect(!g.setLinePosition(2, std::numeric_limits<float>::infinity(), 0.01f), "Inf position rejected");
        expect(g.lines() == before, "Inf position leaves grid unchanged");
    }
    {
        // Extreme finite procedural time/frequency remains stable and monotonic.
        AxisGrid g(128);
        WaveSettings w;
        w.enabled = true;
        w.amplitude = 0.25f;
        w.frequency = 1.0e30f;
        w.phase_cycles = -1.0e30f;
        w.speed_cycles_per_second = 1.0e20f;
        auto e = g.evaluated(w, -1.0e20f, 0.25f, true);
        expect(e.size() == 129, "extreme wave keeps topology");
        for (float v : e) expect(std::isfinite(v), "extreme wave remains finite");
        for (std::size_t i = 1; i < e.size(); ++i) expect(e[i] >= e[i-1], "extreme wave remains monotonic");

        w.amplitude = std::numeric_limits<float>::quiet_NaN();
        auto ignored = g.evaluated(w, 0.0f, 0.01f, true);
        expect(ignored == g.lines(), "NaN wave is ignored instead of poisoning output");
    }
    {
        AxisGrid g(1);
        auto lut = buildInverseLUT(g.lines(), 1, std::numeric_limits<float>::quiet_NaN(),
                                   std::numeric_limits<float>::infinity());
        expect(lut.source_u.size() == 1 && lut.source_u[0] == 0.0f, "1-pixel LUT is valid");
    }
    {
        constexpr int W=32,H=24;
        std::vector<float> src(W*H*4), dst(W*H*4, 0.0f), dst2(W*H*4, 0.0f);
        for (int y=0;y<H;++y) for(int x=0;x<W;++x) {
            auto k=(y*W+x)*4; src[k]=x/31.0f; src[k+1]=y/23.0f; src[k+2]=0.25f; src[k+3]=1.0f;
        }
        AxisGrid gx(4), gy(4);
        auto xl=buildInverseLUT(gx.lines(),W), yl=buildInverseLUT(gy.lines(),H);
        RenderSettings rs; rs.threads=1;
        auto prep=prepareWarpRGBAf(W,H,W,H,xl,yl,rs);
        renderWarpRGBAfPrepared({src.data(),W,H,W*4},{dst.data(),W,H,W*4},prep,rs);
        renderWarpRGBAf({src.data(),W,H,W*4},{dst2.data(),W,H,W*4},xl,yl,rs);
        float max_err=0.0f, parity=0.0f;
        for(std::size_t i=0;i<src.size();++i) {
            max_err=std::max(max_err,std::abs(src[i]-dst[i]));
            parity=std::max(parity,std::abs(dst[i]-dst2[i]));
        }
        expect(max_err < 1e-5f,"identity prepared render");
        expect(parity < 1e-7f,"prepared/non-prepared parity");
    }
    {
        constexpr int W=37,H=29;
        std::vector<float> src(W*H*4), a(W*H*4), b(W*H*4);
        for (int y=0;y<H;++y) for(int x=0;x<W;++x) {
            auto k=(y*W+x)*4; src[k]=std::sin(float(x)*.2f); src[k+1]=std::cos(float(y)*.3f); src[k+2]=float((x+y)%7)/7; src[k+3]=1;
        }
        AxisGrid gx(6), gy(5); ElasticSettings es; es.min_spacing=.001f;
        gx.dragElastic(3,.63f,es); gy.dragElastic(2,.27f,es);
        auto xl=buildInverseLUT(gx.lines(),W,.35f,.2f), yl=buildInverseLUT(gy.lines(),H,.35f,.2f);
        RenderSettings rs; rs.quality=SampleQuality::Bicubic; rs.edge=EdgeMode::Mirror; rs.threads=1;
        auto prep=prepareWarpRGBAf(W,H,W,H,xl,yl,rs);
        renderWarpRGBAfPrepared({src.data(),W,H,W*4},{a.data(),W,H,W*4},prep,rs);
        renderWarpRGBAf({src.data(),W,H,W*4},{b.data(),W,H,W*4},xl,yl,rs);
        float parity=0;
        for(std::size_t i=0;i<a.size();++i) parity=std::max(parity,std::abs(a[i]-b[i]));
        expect(parity < 1e-7f,"bicubic prepared/non-prepared parity");
    }
    std::cout << "elasticgrid_tests: OK\n";
    return 0;
}
