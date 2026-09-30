#include "bridge/control_grid_ffi.h"
#include "core/WarpMath.h"
#include <algorithm>
#include <array>
#include <cassert>
#include <cmath>
#include <cstring>
#include <iostream>
#include <limits>
#include <vector>

template<class T> bool exact(const std::vector<T>& a,const std::vector<T>& b) {
    return a.size()==b.size() && std::memcmp(a.data(),b.data(),a.size()*sizeof(T))==0;
}
static void sample_and_drag() {
    for (int saved_count : {1,4,7,19,50}) {
        std::vector<float> saved(static_cast<std::size_t>(saved_count+2));
        for (std::size_t i=0;i<saved.size();++i) {
            const float u=static_cast<float>(i)/static_cast<float>(saved.size()-1);
            saved[i]=u + 0.09f*std::sin(6.2831853f*u);
        }
        saved.front()=0.0f;saved.back()=1.0f;
        const auto before=saved;
        std::vector<std::uint8_t> pins(saved.size(),0); pins.front()=1;pins.back()=1;
        std::vector<float> previous_refs;
        for (int count=1;count<=50;++count) {
            std::array<float,52> positions{},refs{};
            assert(eg_control_axis_read(saved.data(),saved_count+2,count,0.0f,0.25f,
                                       positions.data(),refs.data(),52)==0);
            assert(exact(saved,before));
            for(float ref:previous_refs) assert(std::find(refs.begin(),refs.begin()+count+2,ref)!=refs.begin()+count+2);
            previous_refs.assign(refs.begin(),refs.begin()+count+2);
            for(int i=1;i<count+2;++i) assert(positions[static_cast<std::size_t>(i)]>positions[static_cast<std::size_t>(i-1)]);
            if(count==saved_count) for(int i=0;i<count+2;++i) assert(positions[static_cast<std::size_t>(i)]==saved[static_cast<std::size_t>(i)]);
            for(int i=1;i<=count;++i) for(float radius:{0.0f,1.0f,3.0f,20.0f}) {
                auto changed=saved;
                const EgElasticParams e{radius,2,1.0f,0.00001f};
                assert(eg_control_axis_drag(changed.data(),pins.data(),saved_count+2,refs[static_cast<std::size_t>(i)],0.0f,&e)==0);
                assert(exact(changed,saved));
                assert(eg_control_axis_drag(changed.data(),pins.data(),saved_count+2,refs[static_cast<std::size_t>(i)],0.0001f,&e)==0);
                assert(!exact(changed,saved));
                assert(changed.front()==0.0f && changed.back()==1.0f);
                for(std::size_t j=1;j<changed.size();++j) assert(changed[j]>changed[j-1]);
            }
            // Control positions follow the actual inverse warp under easing.
            for(float ease:{0.25f,1.0f}) {
                assert(eg_control_axis_read(saved.data(),saved_count+2,count,ease,0.5f,
                                           positions.data(),refs.data(),52)==0);
                for(int i=1;i<=count;++i) {
                    const float got=elasticgrid::inverseMapNormalized(positions[static_cast<std::size_t>(i)],saved,ease,0.5f);
                    const float expected=refs[static_cast<std::size_t>(i)]/static_cast<float>(saved_count+1);
                    assert(std::abs(got-expected)<2e-6f);
                }
            }
        }
    }
    // A near-collapsed retained cell can place several virtual handles on one
    // float/pixel; increasing density must not turn that into a host error.
    for (float middle : {0.000001f,0.999999f}) {
        const std::array<float,3> narrow{0.0f,middle,1.0f};
        std::array<float,52> shown{},ref{};
        for (int count=1;count<=50;++count) {
            assert(eg_control_axis_read(narrow.data(),3,count,1.0f,.5f,shown.data(),ref.data(),52)==0);
            for (int i=1;i<count+2;++i) assert(shown[static_cast<std::size_t>(i)]>=shown[static_cast<std::size_t>(i-1)]);
        }
    }
    std::vector<float> lines{0.0f,0.12f,0.4f,0.65f,0.92f,1.0f};
    std::vector<std::uint8_t> pins{1,0,0,0,0,1};
    for(int i=1;i<=4;++i) for(int falloff:{1,2,3,4}) {
        auto old=lines,next=lines; auto p=pins;
        const EgElasticParams e{3.0f,falloff,1.0f,0.005f};
        assert(eg_drag_axis(old.data(),p.data(),6,i,old[static_cast<std::size_t>(i)]+0.03f,&e)==0);
        assert(eg_control_axis_drag(next.data(),pins.data(),6,static_cast<float>(i),0.03f,&e)==0);
        assert(exact(old,next));
    }
    std::array<float,52> out{},refs{};out.fill(-99.0f);refs.fill(-99.0f);
    const auto sentinel=out;
    assert(eg_control_axis_read(nullptr,6,4,0,0.25f,out.data(),refs.data(),52)!=0);
    assert(eg_control_axis_read(lines.data(),6,51,0,0.25f,out.data(),refs.data(),52)!=0);
    assert(eg_control_axis_read(lines.data(),6,50,0,0.25f,out.data(),refs.data(),6)!=0);
    assert(out==sentinel && refs==sentinel);
    const auto before=lines; const EgElasticParams e{3.0f,2,1.0f,0.005f};
    assert(eg_control_axis_drag(lines.data(),pins.data(),6,std::numeric_limits<float>::quiet_NaN(),0.1f,&e)!=0);
    assert(eg_control_axis_drag(lines.data(),pins.data(),6,1.5f,std::numeric_limits<float>::infinity(),&e)!=0);
    assert(exact(lines,before));
}

template<class T> void pixels(int depth) {
    constexpr int W=27,H=19,STRIDE=W*4+8;
    std::vector<T> source(STRIDE*H),baseline(STRIDE*H),after(STRIDE*H);
    for(std::size_t i=0;i<source.size();++i) source[i]=static_cast<T>((i*13)%251);
    if constexpr(std::is_same_v<T,float>) for(auto& x:source) x=x/31.0f-2.0f;
    std::vector<float> columns{0,.13f,.45f,.59f,.91f,1},rows{0,.19f,.38f,.64f,.88f,1};
    std::vector<std::uint8_t> pins{1,0,0,0,0,1};
    EgRenderParams p{};p.columns=4;p.rows=4;p.column_lines=columns.data();p.row_lines=rows.data();
    p.column_line_count=6;p.row_line_count=6;p.column_pins=pins.data();p.row_pins=pins.data();
    p.min_spacing=.005f;p.canvas_width=W;p.canvas_height=H;p.threads=1;
    for(int quality:{1,2}) for(int edge:{1,2,3}) for(float wave:{0.0f,.17f}) for(float easing:{0.0f,.8f}) {
        p.quality=quality;p.edge_mode=edge;p.wave_enabled=1;p.wave_amplitude=wave;
        p.wave_frequency=2;p.wave_phase=23;p.wave_speed=.25f;p.wave_axis=1;
        p.stretch_easing=easing;p.easing_distance=.25f;p.time_seconds=.73f;
        for(bool sparse:{false,true}) {
            const auto render=sparse?eg_render_frame_sparse:eg_render_frame;
            p.input_origin_x=sparse?3:0;p.input_origin_y=sparse?2:0;
            const int iw=sparse?W-7:W,ih=sparse?H-5:H;
            std::fill(baseline.begin(),baseline.end(),T{});
            assert(render(source.data(),STRIDE*sizeof(T),iw,ih,baseline.data(),STRIDE*sizeof(T),W,H,depth,&p)==0);
            for(int count:{1,50,4,7,19,1,50,4}) {
                std::array<float,52> shown{},refs{};
                assert(eg_control_axis_read(columns.data(),6,count,easing,.25f,shown.data(),refs.data(),52)==0);
                assert(eg_control_axis_read(rows.data(),6,51-count,easing,.25f,shown.data(),refs.data(),52)==0);
                std::fill(after.begin(),after.end(),T{});
                assert(render(source.data(),STRIDE*sizeof(T),iw,ih,after.data(),STRIDE*sizeof(T),W,H,depth,&p)==0);
                assert(exact(baseline,after));
            }
        }
    }
}
int main(){sample_and_drag();pixels<std::uint8_t>(8);pixels<std::uint16_t>(16);pixels<float>(32);
    std::cout<<"control density: nested layouts, usable drags, exact 8/16/32bpc render invariance PASS\n";}
