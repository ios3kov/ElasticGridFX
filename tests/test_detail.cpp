#include "bridge/elasticgrid_ffi.h"
#include "bridge/plane_ffi.h"
#include "core/PlaneWarp.h"
#include <type_traits>
#include <algorithm>
#include <array>
#include <bit>
#include <cmath>
#include <cstdint>
#include <iostream>
#include <limits>
#include <stdexcept>
#include <thread>
#include <vector>

static void check(bool value) { if(!value) throw std::runtime_error("detail regression failed"); }
static std::vector<float> curve() {
    std::vector<float> values(elasticgrid::kDetailSamples);
    for(std::size_t i=0;i<values.size();++i) {
        const float q=static_cast<float>(i)/static_cast<float>(values.size()-1);
        values[i]=q+0.03f*std::sin(q*6.283185307f);
    }
    values.front()=0;values.back()=1;return values;
}
static EgRenderParams params(const std::array<float,6>& x,const std::array<float,6>& y,const std::array<std::uint8_t,6>& pins) {
    EgRenderParams p{};p.columns=4;p.rows=4;p.column_lines=x.data();p.row_lines=y.data();
    p.column_line_count=6;p.row_line_count=6;p.column_pins=pins.data();p.row_pins=pins.data();
    p.canvas_width=31;p.canvas_height=23;p.quality=2;p.edge_mode=1;p.threads=1;
    p.elasticity_strength=1;p.falloff=2;p.tension_radius=3;p.min_spacing=0.005f;
    p.stretch_easing=0.5f;p.easing_distance=0.25f;return p;
}
template<class T> static void pixels(int depth) {
    std::array<float,6> x{0,0.12f,0.37f,0.62f,0.91f,1};
    std::array<float,6> y{0,0.09f,0.31f,0.68f,0.86f,1};
    std::array<std::uint8_t,6> pins{1,0,0,0,0,1};auto p=params(x,y,pins);
    const int stride=31*4+8;const std::ptrdiff_t pitch=stride*static_cast<std::ptrdiff_t>(sizeof(T));
    std::vector<T> input(static_cast<std::size_t>(stride*23));
    for(std::size_t i=0;i<input.size();++i) {
        if constexpr(std::is_same_v<T,float>) input[i]=static_cast<float>(i%71)/11.0f-2.0f;
        else input[i]=static_cast<T>((i*37)%(depth==8?256:32769));
    }
    std::vector<T> expected(input.size()),actual(input.size()),changed(input.size());
    EgDetailMaps empty{};auto detail=curve();EgDetailMaps map{detail.data(),257,detail.data(),257};
    for(int quality=1;quality<=2;++quality) {for(int sparse=0;sparse<=1;++sparse) {
        p.quality=quality;
        const auto old=sparse?eg_render_frame_sparse:eg_render_frame;
        check(old(input.data(),pitch,31,23,expected.data(),pitch,31,23,depth,&p)==0);
        check(eg_render_frame_detail(input.data(),pitch,31,23,actual.data(),pitch,31,23,depth,&p,&empty,sparse)==0);
        check(actual==expected);
        check(eg_render_frame_detail(input.data(),pitch,31,23,changed.data(),pitch,31,23,depth,&p,&map,sparse)==0);
        check(changed!=expected);
        for(int repeat=0;repeat<4;++repeat) {
            check(eg_render_frame_detail(input.data(),pitch,31,23,actual.data(),pitch,31,23,depth,&p,&map,sparse)==0);
            check(actual==changed);
        }
        EgDetailMaps invalid{detail.data(),256,nullptr,0};auto saved=actual;
        check(eg_render_frame_detail(input.data(),pitch,31,23,actual.data(),pitch,31,23,depth,&p,&invalid,sparse)!=0);
        check(actual==saved);
        invalid={nullptr,257,nullptr,0};
        check(eg_render_frame_detail(input.data(),pitch,31,23,actual.data(),pitch,31,23,depth,&p,&invalid,sparse)!=0);
        check(actual==saved);
        invalid={nullptr,-1,nullptr,0};
        check(eg_render_frame_detail(input.data(),pitch,31,23,actual.data(),pitch,31,23,depth,&p,&invalid,sparse)!=0);
        check(actual==saved);
    }}
    p.quality=2;
    std::vector<std::thread> workers;
    for(int i=0;i<4;++i) workers.emplace_back([&]{std::vector<T> out(input.size());
        check(eg_render_frame_detail(input.data(),pitch,31,23,out.data(),pitch,31,23,depth,&p,&map,1)==0);check(out==changed);});
    for(auto& worker:workers) worker.join();
    p.abort_fn=[](void*)->std::int32_t{return 1;};
    check(eg_render_frame_detail(input.data(),pitch,31,23,actual.data(),pitch,31,23,depth,&p,&map,1)==5);
    p.abort_fn=nullptr;
    check(eg_render_frame_detail(input.data(),pitch,31,23,actual.data(),pitch,31,23,depth,&p,&map,1)==0);check(actual==changed);
}
int main() {
    auto map=curve();check(elasticgrid::validDetail(map));check(!elasticgrid::identityDetail(map));
    std::vector<float> identity(257);for(std::size_t i=0;i<identity.size();++i) identity[i]=static_cast<float>(i)/256.0f;
    check(elasticgrid::identityDetail(identity));
    for(std::size_t i=0;i<map.size();++i) check(std::abs(elasticgrid::inverseDetail(map[i],map)-static_cast<double>(i)/256.0)<1e-7);
    std::array<float,6> axis{0,0.12f,0.37f,0.62f,0.91f,1};
    for(float easing:{0.0f,0.5f,1.0f}) {
        for(int j=0;j<=50;++j) {
            float q=static_cast<float>(j)/50.0f,point=0,restored=0;
            check(eg_axis_coordinates(axis.data(),6,&q,&point,1,easing,0.25f,1)==0);
            check(eg_axis_coordinates(axis.data(),6,&point,&restored,1,easing,0.25f,0)==0);
            check(std::abs(q-restored)<2e-6f);
        }
    }
    const auto transform=elasticgrid::PlaneTransform::fromCorners({elasticgrid::PlanePoint{0,0},{1,0},{1,1},{0,1}});
    check(transform.has_value());
    auto plane=elasticgrid::PlaneWarp::prepare(*transform,{0,0.5f,1},{0,0.5f,1});
    check(plane.has_value());check(plane->setDetail(map,{}));
    auto point=plane->sourceFor({0.3,0.7});check(point.source.has_value());
    check(std::abs(point.source->x-elasticgrid::inverseDetail(0.3,map))<1e-7);
    auto outside=plane->sourceFor({1.1,0.7});check(outside.status==elasticgrid::PlaneMapStatus::OutsidePlane);
    pixels<std::uint8_t>(8);pixels<std::uint16_t>(16);pixels<float>(32);
    std::cout<<"detail maps: identity, changed pixels, malformed data, cancellation, repeat, MFR, plane and mapping PASS\n";
}
