put('tests/test_density_contract.py', [
    (0,0,r'''"""Wiring regressions; these do not replace native AE key-stream checks."""
from pathlib import Path
import unittest
ROOT = Path(__file__).resolve().parents[1]
HOST = (ROOT/'host-rust/src/lib.rs').read_text()
UI = (ROOT/'host-rust/src/ui.rs').read_text()
BINDING = (ROOT/'host-rust/src/binding_probe.rs').read_text()

class DensityContract(unittest.TestCase):
    def test_counts_never_rewrite_grid_keys(self):
        self.assertNotIn('sync_grid_topology', HOST)
        self.assertNotIn('.resized(', HOST)
        handler = HOST.split('if params.index(Params::Columns) == Some(param_index)', 1)[1].split('ae::Command::UpdateParamsUi', 1)[0]
        for forbidden in ('GridState', 'set_value', 'set_value_changed', 'checkout', 'aegp::'):
            self.assertNotIn(forbidden, handler)
        self.assertIn('ForceRerender', handler)
    def test_render_snapshot_does_not_read_display_counts(self):
        for start, end in [('fn smart_render_snapshot(', '#[repr(C)]'),
                           ('pub(crate) fn grid_snapshot(', 'pub(crate) fn elastic_params(')]:
            text = HOST.split(start, 1)[1].split(end, 1)[0]
            for forbidden in ('Params::Columns', 'Params::Rows', 'topology(', '.resized('):
                self.assertNotIn(forbidden, text)
            self.assertIn('guide_density::render_grid', text)
    def test_pending_identity_is_independent_of_display_counts(self):
        text = BINDING.split('fn pending_frame_identity(', 1)[1].split('pub fn add_params(', 1)[0]
        self.assertNotIn('Params::Columns', text)
        self.assertNotIn('Params::Rows', text)
        self.assertIn('params.checkout(Params::GridState)', text)
    def test_all_viewer_paths_share_readonly_projection(self):
        text = UI.split('fn displayed_grid(', 1)[1].split('thread_local!', 1)[0]
        self.assertIn('guide_density::view_grid', text)
        self.assertNotIn('set_value', text)
        # Drawing, picking and cursor all use the same displayed data.
        self.assertEqual(UI.count('displayed_grid('), 4)
        self.assertIn('let base=evaluated_control_base(in_data,params,&plane)?;', UI)
        self.assertIn('guide_density::drag_control(&mut grid,&base,request,&elastic)', UI)
    def test_drag_cannot_switch_target_when_counts_change(self):
        self.assertIn('event.continue_refcon(2)', UI)
        self.assertIn('event.continue_refcon(3)', UI)
        self.assertIn('guide_density::drag_control', UI)
        self.assertIn('if changed {', UI)
    def test_old_wire_stays_readable_and_counts_are_not_keyed(self):
        self.assertIn('const GRID_WIRE_VERSION: u16 = 3;', HOST)
        self.assertIn('const GRID_DETAIL_WIRE_VERSION: u16 = 4;', HOST)
        self.assertIn('if detailed {8} else {6}', HOST)
        self.assertEqual(HOST.count('ae::ParamFlag::CANNOT_TIME_VARY'), 2)

if __name__ == '__main__': unittest.main()
'''),
])
put('tests/test_density_safety.js', [
    (0,0,r''''use strict';
const assert=require('node:assert/strict'), fs=require('node:fs'), vm=require('node:vm'), path=require('node:path');
const source=fs.readFileSync(path.join(__dirname,'ae_density_invariance.jsx'),'utf8');
let touches=0;
const c={app:{},Folder:function(){touches++;throw Error('Unexpected filesystem access');},
    elasticGridCurrentProjectState:()=>({guard:'OCCUPIED',project_revision:'2'}),
    elasticGridHasTestProjectOwnership:()=>false};
vm.createContext(c);vm.runInContext(source,c);
const config={run_id:'a'.repeat(32),depth:8,kind:'text',folder:'/tmp/not-owned'};
for (const value of [null,{}, {...config,run_id:'../escape'},{...config,depth:24},{...config,kind:'unknown'}])
    assert.throws(()=>c.fstrDensityInvariance(value));
assert.throws(()=>c.fstrDensityInvariance(config),/clean empty unsaved/);
assert.equal(touches,0);
for(const token of ['app.quit(','.purge(','beginSuppressDialogs(','DO_NOT_SAVE_CHANGES','scheduleTask('])
    assert.ok(!source.includes(token),token);
assert.ok(source.includes('CAPTURED_NOT_FULL_ACCEPTANCE'));
console.log('PASS density fixture loading/argument/ownership safety (NOT AE)');
'''),
])
put('tests/test_detail.cpp', [
    (0,0,r'''#include "bridge/elasticgrid_ffi.h"
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
'''),
])
put('tests/test_first_application_contract.py', [
    (17,1,r'''        for token in ['checked_float(params,id)',
'''),
    (19,1,r'''                      'Params::WaveAmplitude',
'''),
    (22,0,r'''        self.assertNotIn('Params::Columns',proof)
        self.assertNotIn('Params::Rows',proof)
'''),
    (29,1,r''''''),
])
put('tests/test_host_contract.py', [
    (82,1,r'''    def test_count_change_redraws_without_rewriting_grid_keys(self):
'''),
    (87,1,r'''        self.assertNotIn('sync_grid_topology', SOURCE)
        count_handler = handler.split('if params.index(Params::Columns)', 1)[1]
        self.assertNotIn('set_value', count_handler)
        self.assertNotIn('GridState', count_handler)
        self.assertIn('ForceRerender', count_handler)
'''),
    (119,1,r'''        self.assertIn('render_sparse(input.as_ref(), &mut output, &p, &snapshot.grid)?;', smart)
'''),
])
