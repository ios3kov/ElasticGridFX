#include "core/GridModel.h"
#include "bridge/control_grid_ffi.h"
#include <cassert>
#include <cmath>
#include <cstring>
#include <limits>
#include <vector>
#include <iostream>
using elasticgrid::AxisGrid;
bool exact(const std::vector<float>& a,const std::vector<float>& b){
    return a.size()==b.size() && !std::memcmp(a.data(),b.data(),a.size()*sizeof(float));
}
EgRenderParams params(std::vector<float>& lines,std::vector<uint8_t>& pins,float radius){
    EgRenderParams p{};
    p.columns=p.rows=static_cast<int>(lines.size())-2;
    p.column_lines=p.row_lines=lines.data(); p.column_pins=p.row_pins=pins.data();
    p.column_line_count=p.row_line_count=static_cast<int>(lines.size());
    p.tension_radius=radius;p.falloff=1;p.elasticity_strength=1;p.live_influence=1;
    return p;
}
std::vector<float> evaluated(EgRenderParams p){
    std::vector<float> x(p.columns+2),y(p.rows+2);
    assert(!eg_evaluate_grid(&p,x.data(),static_cast<int>(x.size()),y.data(),static_cast<int>(y.size())));
    return x;
}
float displayed(EgRenderParams p,float ref){
    auto axis=evaluated(p);
    const float normalized[]={0,ref/static_cast<float>(axis.size()-1),1};
    float positions[3],refs[3];
    assert(!eg_control_axis_read_layout(axis.data(),static_cast<int>(axis.size()),normalized,3,
        p.stretch_easing,p.easing_distance,positions,refs,3));
    return positions[1];
}
template<class T> void sampling(int depth){
    AxisGrid seed(10); auto lines=seed.lines();auto pins=seed.pins();lines[5]+=.04f;
    constexpr int width=29,height=23;
    std::vector<T> source(width*height*4),actual(source.size()),reference(source.size());
    for(size_t i=0;i<source.size();++i){
        if constexpr(std::is_same_v<T,float>) source[i]=static_cast<float>(i%127)/17.f-3.f;
        else source[i]=static_cast<T>((i*31)%251);
    }
    const auto saved=lines;
    for(int quality:{1,2}) for(int edge:{1,2,3}) for(float wave:{0.f,.1f}){
        auto p=params(lines,pins,3);p.quality=quality;p.edge_mode=edge;
        p.stretch_easing=.7f;p.easing_distance=.25f;
        p.wave_enabled=1;p.wave_amplitude=wave;p.wave_frequency=1;p.wave_axis=1;
        const auto axis=evaluated(p);
        auto expected=p;expected.live_influence=0;expected.wave_enabled=0;
        expected.column_lines=expected.row_lines=axis.data();
        const auto stride=static_cast<std::ptrdiff_t>(width*4*sizeof(T));
        assert(!eg_render_frame(source.data(),stride,width,height,actual.data(),stride,width,height,depth,&p));
        assert(!eg_render_frame(source.data(),stride,width,height,reference.data(),stride,width,height,depth,&expected));
        assert(!std::memcmp(actual.data(),reference.data(),actual.size()*sizeof(T)));
        assert(exact(lines,saved));
    }
}
int main(){
    for(int count=1;count<=50;++count){
        AxisGrid axis(static_cast<size_t>(count+1));
        const auto neutral=axis.lines();
        std::vector<float> out;
        for(float radius:{0.f,1.f,3.f,20.f}){
            assert(axis.influencedInto(out,radius,0)); assert(exact(out,neutral));
        }
        auto deformed=neutral;
        for(size_t i=1;i+1<deformed.size();++i) deformed[i]+=0.15f/(count+1)*std::sin(static_cast<float>(i));
        const auto retainedPins=axis.pins();
        assert(axis.setState(deformed,retainedPins));
        const auto stored=axis.lines();
        for(float radius:{0.f,1.f,1.01f,2.f,3.f,10.f,20.f}){
            assert(axis.influencedInto(out,radius,0)); assert(exact(axis.lines(),stored));
            assert(out.front()==0 && out.back()==1);
            for(size_t i=1;i<out.size();++i) assert(std::isfinite(out[i]) && out[i]>out[i-1]);
            if(radius<=1) assert(exact(out,stored));
        }
    }
    AxisGrid seed(10);
    auto lines=seed.lines();auto pins=seed.pins(); lines[5]+=.04f;
    const auto original=lines;
    auto p=params(lines,pins,3);
    auto smooth=evaluated(p);
    assert(std::abs(smooth[5]-(.5f+.04f/3))<1e-7f);
    assert(std::abs(smooth[4]-(.4f+.04f*(20.f/27)/3))<1e-7f);
    assert(smooth[4]>original[4] && smooth[5]<original[5]);
    p.tension_radius=0;assert(exact(evaluated(p),original));
    p.tension_radius=3;
    p.live_influence=0;assert(exact(evaluated(p),original)); p.live_influence=1;
    // Changing animated radius evaluates saved data without rewriting either axis.
    for(float radius:{0.f,3.f,10.f,3.f,0.f}){p.tension_radius=radius;evaluated(p);assert(exact(lines,original));}
    for(float radius:{0.f,3.f,20.f}) for(float ref:{1.f,4.3f,5.f,8.5f})
        for(float ease:{0.f,1.f}) for(float wave:{0.f,.1f}){
            lines=original; p=params(lines,pins,radius);
            p.stretch_easing=ease;p.easing_distance=.25f;
            p.wave_enabled=1;p.wave_amplitude=wave;p.wave_frequency=1;p.wave_axis=1;
            const float start=displayed(p,ref);
            const float target=start+.015f;
            const EgElasticParams elastic{radius,1,1,0};
            assert(!eg_control_axis_drag_live(lines.data(),pins.data(),static_cast<int>(lines.size()),
                ref,target,&elastic,&p,1));
            assert(std::abs(displayed(p,ref)-target)<2e-6f);
            const auto saved=lines;
            assert(!eg_control_axis_drag_live(lines.data(),pins.data(),static_cast<int>(lines.size()),
                ref,displayed(p,ref),&elastic,&p,1));
            assert(exact(lines,saved));
        }
    lines=original;p=params(lines,pins,3);
    const EgElasticParams elastic{3,1,1,0};
    assert(!eg_control_axis_drag_live(lines.data(),pins.data(),static_cast<int>(lines.size()),5,100,&elastic,&p,1));
    const float saturated=displayed(p,5);
    assert(!eg_control_axis_drag_live(lines.data(),pins.data(),static_cast<int>(lines.size()),5,saturated-.01f,&elastic,&p,1));
    assert(displayed(p,5)<saturated-.009f);
    const auto saved=lines;
    assert(eg_control_axis_drag_live(lines.data(),pins.data(),static_cast<int>(lines.size()),5,
        std::numeric_limits<float>::quiet_NaN(),&elastic,&p,1)!=0);
    assert(exact(lines,saved));
    sampling<uint8_t>(8);sampling<uint16_t>(16);sampling<float>(32);
    std::cout<<"PASS: immediate saved-field response, exact neutral, immutable axes and live fractional/eased/wave drag\n";
}
