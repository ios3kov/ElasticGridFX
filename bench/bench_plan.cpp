#include "bridge/elasticgrid_ffi.h"
#include <chrono>
#include <cstdint>
#include <iostream>
#include <string>
#include <vector>

int main(int argc, char** argv) {
    int w = argc > 1 ? std::stoi(argv[1]) : 3840;
    int h = argc > 2 ? std::stoi(argv[2]) : 2160;
    int frames = argc > 3 ? std::stoi(argv[3]) : 1000;
    int quality = argc > 4 && std::string(argv[4]) == "bicubic" ? 2 : 1;

    EgRenderParams p{};
    p.columns = 12; p.rows = 8; p.tension_radius = 4.0f; p.falloff = 2;
    p.elasticity_strength = 1.0f; p.min_spacing = 0.002f;
    p.stretch_easing = 0.4f; p.easing_distance = 0.25f;
    p.wave_enabled = 1; p.wave_amplitude = 0.04f; p.wave_frequency = 2.0f;
    p.wave_speed = 0.2f; p.wave_axis = 1; p.edge_mode = 1; p.quality = quality;

    std::vector<float> x(static_cast<std::size_t>(p.columns)+2), y(static_cast<std::size_t>(p.rows)+2);
    std::vector<std::uint8_t> xp(x.size(),0), yp(y.size(),0);
    for (std::size_t i=0;i<x.size();++i) x[i]=float(i)/float(p.columns+1);
    for (std::size_t i=0;i<y.size();++i) y[i]=float(i)/float(p.rows+1);
    xp.front()=xp.back()=1; yp.front()=yp.back()=1;
    EgElasticParams ep{p.tension_radius,p.falloff,p.elasticity_strength,p.min_spacing};
    if (eg_drag_axis(x.data(),xp.data(),static_cast<int>(x.size()),6,0.58f,&ep)!=0) return 2;
    if (eg_drag_axis(y.data(),yp.data(),static_cast<int>(y.size()),4,0.43f,&ep)!=0) return 3;
    p.column_lines=x.data(); p.column_line_count=static_cast<int>(x.size()); p.column_pins=xp.data();
    p.row_lines=y.data(); p.row_line_count=static_cast<int>(y.size()); p.row_pins=yp.data();

    std::vector<EgGpuLinearSample> lx,ly; std::vector<EgGpuCubicSample> cx,cy;
    if (quality==2) { cx.resize(w); cy.resize(h); } else { lx.resize(w); ly.resize(h); }

    auto run=[&](int i){
        p.time_seconds=float(i)/30.0f;
        return eg_prepare_gpu_plan(w,h,w,h,&p,
            quality==1?lx.data():nullptr, quality==1?ly.data():nullptr,
            quality==2?cx.data():nullptr, quality==2?cy.data():nullptr);
    };
    for(int i=0;i<10;++i) if(run(i)!=0) return 4;
    auto t0=std::chrono::steady_clock::now();
    for(int i=0;i<frames;++i) if(run(i)!=0) return 5;
    auto t1=std::chrono::steady_clock::now();
    double ms=std::chrono::duration<double,std::milli>(t1-t0).count()/frames;
    std::cout<<w<<'x'<<h<<" GPU plan "<<(quality==2?"bicubic":"bilinear")<<": "<<ms<<" ms/frame\n";
}
