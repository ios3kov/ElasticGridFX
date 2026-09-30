put('src/bridge/elasticgrid_ffi.cpp', [
    (4,0,r'''#include "bridge/detail_ffi.h"
'''),
    (224,2,r'''                   bool force_sampling_plan = false, const EgDetailMaps* detail = nullptr) {
    if (!p || input_width <= 0 || input_height <= 0 || output_width <= 0 || output_height <= 0 || !eg_detail_valid(detail)) return 1;
'''),
    (237,0,r'''
    const auto detail_x = eg_detail_x(detail), detail_y = eg_detail_y(detail);
    if (!detail_x.empty()) for (float& u : out.x_lut.source_u) u = static_cast<float>(eg::inverseDetail(static_cast<double>(u), detail_x));
    if (!detail_y.empty()) for (float& v : out.y_lut.source_u) v = static_cast<float>(eg::inverseDetail(static_cast<double>(v), detail_y));
'''),
    (261,2,r'''    const bool uniform_x = axis_uniform_exact(out.x_lines) && eg::identityDetail(detail_x);
    const bool uniform_y = axis_uniform_exact(out.y_lines) && eg::identityDetail(detail_y);
'''),
    (303,1,r'''                          const EgRenderParams* p, PreparedBridge& out, const EgDetailMaps* detail = nullptr) {
'''),
    (307,1,r'''                                  &logical, out, true, detail);
'''),
    (498,1,r'''    const EgRenderParams* p, bool sparse, const EgDetailMaps* detail = nullptr) noexcept {
'''),
    (500,1,r'''    if (!eg_detail_valid(detail) || !output_data || !p || output_width <= 0 || output_height <= 0 ||
'''),
    (534,2,r'''            ? prepare_sparse_bridge(input_width, input_height, output_width, output_height, p, prepared, detail)
            : prepare_bridge(input_width, input_height, output_width, output_height, p, prepared, false, detail);
'''),
    (607,0,r'''
// Same validation, cancellation, pixel sampling and sparse semantics as the
// legacy functions; only the owned coordinate map is composed before sampling.
int eg_render_frame_detail(const void* input, std::ptrdiff_t input_pitch,
    std::int32_t iw, std::int32_t ih, void* output, std::ptrdiff_t output_pitch,
    std::int32_t ow, std::int32_t oh, std::int32_t depth,
    const EgRenderParams* params, const EgDetailMaps* detail, std::int32_t sparse) noexcept {
    if (sparse != 0 && sparse != 1) return 1;
    return render_frame_impl(input, input_pitch, iw, ih, output, output_pitch,
                             ow, oh, depth, params, sparse != 0, detail);
}

int eg_axis_coordinates(const float* axis, std::int32_t count,
    const float* queries, float* output, std::int32_t n,
    float easing, float distance, std::int32_t forward) noexcept {
    if (!axis || count < 3 || count > 52 || !queries || !output || n < 1 || n > 52 ||
        (forward != 0 && forward != 1) || !std::isfinite(easing) || !std::isfinite(distance)) return 1;
    for (std::int32_t i = 0; i < count; ++i)
        if (!std::isfinite(axis[i]) || (i && axis[i] <= axis[i-1])) return 1;
    if (axis[0] != 0.0f || axis[count-1] != 1.0f) return 1;
    for (std::int32_t i = 0; i < n; ++i) if (!std::isfinite(queries[i])) return 1;
    try {
        const std::vector<float> values(axis, axis + count);
        for (std::int32_t i = 0; i < n; ++i) {
            const float q = std::clamp(queries[i], 0.0f, 1.0f);
            if (!forward) { output[i] = eg::inverseMapNormalized(q, values, easing, distance); continue; }
            bool exact = false;
            for (std::int32_t j = 0; j < count; ++j) {
                if (float_bits_equal(q, static_cast<float>(j) / static_cast<float>(count-1))) {
                    output[i] = axis[j]; exact = true; break;
                }
            }
            if (exact) continue;
            float lo = 0.0f, hi = 1.0f;
            for (int iteration = 0; iteration < 32; ++iteration) {
                const float mid = lo + (hi-lo)*0.5f;
                if (eg::inverseMapNormalized(mid, values, easing, distance) < q) lo = mid;
                else hi = mid;
            }
            output[i] = lo + (hi-lo)*0.5f;
        }
        return 0;
    } catch (...) { return 3; }
}
'''),
])
put('src/bridge/elasticgrid_ffi.h', [
    (3,0,r'''#include "bridge/detail_ffi.h"
'''),
    (150,0,r'''int eg_render_frame_detail(const void* input, std::ptrdiff_t input_pitch,
    std::int32_t iw, std::int32_t ih, void* output, std::ptrdiff_t output_pitch,
    std::int32_t ow, std::int32_t oh, std::int32_t depth,
    const EgRenderParams* params, const EgDetailMaps* detail, std::int32_t sparse) noexcept;
int eg_axis_coordinates(const float* axis, std::int32_t count,
    const float* queries, float* output, std::int32_t n,
    float easing, float distance, std::int32_t forward) noexcept;

'''),
])
put('src/bridge/plane_ffi.cpp', [
    (58,1,r'''    const double* source_corners,bool regional=false,bool layer=false,const EgDetailMaps* detail=nullptr) noexcept {
'''),
    (61,1,r'''    if(!eg_detail_valid(detail) || !source || !output || !f || quality<0 || quality>1 || edge<0 || edge>2 ||
'''),
    (95,0,r'''        }
        if (warp && detail) {
            const auto x = eg_detail_x(detail), y = eg_detail_y(detail);
            if (!warp->setDetail(std::vector<float>(x.begin(),x.end()), std::vector<float>(y.begin(),y.end()))) return 1;
'''),
    (142,0,r'''
int eg_render_plane_detail(const EgPlaneImage* source,const EgPlaneImage* output,
    std::int32_t depth,const EgPlaneFrame* frame,EgPlaneReport* report,
    std::int32_t quality,std::int32_t edge,const EgDetailMaps* detail,std::int32_t layer) noexcept {
    if (layer != 0 && layer != 1) { if(report) *report={}; return 1; }
    return renderPlane(source,output,depth,frame,report,quality,edge,0,0,nullptr,true,layer != 0,detail);
}
'''),
])
put('src/bridge/plane_ffi.h', [
    (78,0,r'''
extern "C" int eg_render_plane_detail(const EgPlaneImage* source,const EgPlaneImage* output,
    std::int32_t depth,const EgPlaneFrame* frame,EgPlaneReport* report,
    std::int32_t quality,std::int32_t edge,const EgDetailMaps* detail,std::int32_t layer) noexcept;
'''),
])
put('src/core/DetailMap.h', [
    (0,0,r'''#pragma once
#include <algorithm>
#include <bit>
#include <cstdint>
#include <cmath>
#include <cstddef>
#include <span>

namespace elasticgrid {
// Fixed material-coordinate refinement, independent of displayed guide count.
// Empty means the exact identity, preserving the historical sampling path.
inline constexpr std::size_t kDetailSamples = 257;
inline bool validDetail(std::span<const float> values) noexcept {
    if (values.empty()) return true;
    if (values.size() != kDetailSamples || values.front() != 0.0f || values.back() != 1.0f) return false;
    for (std::size_t i = 0; i < values.size(); ++i)
        if (!std::isfinite(values[i]) || (i && values[i] <= values[i-1])) return false;
    return true;
}
inline bool identityDetail(std::span<const float> values) noexcept {
    if (values.empty()) return true;
    for (std::size_t i = 0; i < values.size(); ++i)
        if (std::bit_cast<std::uint32_t>(values[i]) != std::bit_cast<std::uint32_t>(static_cast<float>(i) / static_cast<float>(values.size()-1))) return false;
    return true;
}
// Caller has validated an immutable owned view. Extrapolation preserves layer
// perimeter semantics; bounded Four Corners still handles its own exterior.
inline double inverseDetail(double value, std::span<const float> map) noexcept {
    if (map.empty()) return value;
    const double cells = static_cast<double>(map.size()-1);
    if (value <= 0.0) return value / (cells * static_cast<double>(map[1]));
    if (value >= 1.0) return 1.0 + (value-1.0) / (cells * (1.0-static_cast<double>(map[map.size()-2])));
    auto right = std::upper_bound(map.begin(), map.end(), static_cast<float>(value));
    const std::size_t index = std::clamp(static_cast<std::size_t>(right-map.begin()), std::size_t{1}, map.size()-1);
    const double fraction = (value-static_cast<double>(map[index-1])) / (static_cast<double>(map[index])-static_cast<double>(map[index-1]));
    return (static_cast<double>(index-1)+fraction)/cells;
}
} // namespace elasticgrid
'''),
])
put('src/core/PlaneWarp.cpp', [
    (2,0,r'''#include "core/DetailMap.h"
'''),
    (60,0,r'''bool PlaneWarp::setDetail(std::vector<float> columns, std::vector<float> rows) {
    if (!validDetail(columns) || !validDetail(rows)) return false;
    column_detail_ = std::move(columns); row_detail_ = std::move(rows);
    identity_ = uniform(columns_) && uniform(rows_) && identityDetail(column_detail_) && identityDetail(row_detail_);
    return true;
}
'''),
    (69,2,r'''        auto source=transform_.toSurface({inverseDetail(extendedAxis(local->x,columns_,easing_,easing_distance_),column_detail_),
                                         inverseDetail(extendedAxis(local->y,rows_,easing_,easing_distance_),row_detail_)});
'''),
    (85,2,r'''    PlanePoint normalized{inverseDetail(inverseMapNormalized(x,columns_,easing_,easing_distance_),column_detail_),
                          inverseDetail(inverseMapNormalized(y,rows_,easing_,easing_distance_),row_detail_)};
'''),
])
put('src/core/PlaneWarp.h', [
    (24,0,r'''    bool setDetail(std::vector<float> columns, std::vector<float> rows);
'''),
    (39,0,r'''    std::vector<float> column_detail_, row_detail_;
'''),
])
put('tests/ae_density_invariance.jsx', [
    (0,0,r'''// Load ae_runtime_smoke.jsx for its strict empty-project ownership guard.
// One kind/depth per test-owned session. Loaded Build ID is a separate gate.
function fstrDensityInvariance(config) {
    if (!config || typeof config.run_id !== 'string' || !/^[a-f0-9]{32}$/.test(config.run_id)) throw Error('Invalid run id');
    if (config.depth !== 8 && config.depth !== 16 && config.depth !== 32) throw Error('Invalid depth');
    if (config.kind !== 'text' && config.kind !== 'checker_precomp') throw Error('Invalid kind');
    if (typeof elasticGridCurrentProjectState !== 'function' || typeof elasticGridHasTestProjectOwnership !== 'function') throw Error('Load project guard');
    var state=elasticGridCurrentProjectState();
    if (!elasticGridHasTestProjectOwnership(state.guard,state.project_revision)) throw Error('Requires clean empty unsaved project');
    var root=new Folder(config.folder);
    if (!root.exists || root.name!=='EGFX-density-'+config.run_id || root.getFiles().length!==0) throw Error('Fresh run folder required');
    var p=app.project;
    if (p.renderQueue.numItems!==0) throw Error('Existing queue');
    p.bitsPerChannel=config.depth; p.workingSpace=''; p.linearizeWorkingSpace=false;
    var c=p.items.addComp('__EGFX_DENSITY_'+config.run_id,640,480,1,2,25),l;
    if (config.kind==='text') { l=c.layers.addText('FSTR DENSITY'); }
    else {
        l=c.layers.addSolid([0.3,0.6,0.9],'Density checker',640,480,1,2);
        l.property('ADBE Effect Parade').addProperty('ADBE Checkerboard');
        c.layers.precompose([l.index],'Density source',true); l=c.layer(1);
    }
    var e=l.property('ADBE Effect Parade').addProperty('com.elasticgrid.fx.warp');
    if (!e) throw Error('Effect unavailable');
    var grid=e.property('Grid Positions'),cols=e.property('Columns'),rows=e.property('Rows');
    if (cols.canVaryOverTime || rows.canVaryOverTime || !grid.canVaryOverTime) throw Error('Animation flags');
    grid.addKey(0); grid.addKey(1);
    // Real changing frames without trying to fabricate AE's CUSTOM_VALUE bytes.
    // Nonuniform Grid Positions animation is additionally tested via Rust/FFI
    // and must be exercised in a manually authored native fixture.
    e.property('Wave Amplitude').setValue(15); e.property('Wave Speed').setValue(1);
    c.openInViewer();
    function verifyKeys() {
        if (grid.numKeys!==2 || grid.keyTime(1)!==0 || grid.keyTime(2)!==1 || cols.numKeys!==0 || rows.numKeys!==0)
            throw Error('Count change mutated key topology');
    }
    function capture(label,time) {
        var item=p.renderQueue.items.add(c);
        try {
            var output=item.outputModule(1); output.applyTemplate('_HIDDEN X-Factor 16'); output=item.outputModule(1);
            var s=output.getSettings(GetSettingsFormat.STRING);
            if (s.Format!=='PNG Sequence'||s.Depth!=='Trillions of Colors+'||s.Color!=='Straight (Unmatted)'||s.Resize!=='false'||s.Crop!=='false') throw Error('Output contract');
            output.file=new File(root.fsName+'/'+label+'-[#####].png');
            item.setSettings({'Quality':'Best','Resolution':'Full','Color Depth':'Current Settings','Effects':'Current Settings'});
            item.timeSpanStart=time; item.timeSpanDuration=c.frameDuration; p.renderQueue.render();
            if (item.status!==RQItemStatus.DONE) throw Error('Capture failed '+label);
        } finally { item.remove(); }
    }
    // Run capture in a LATER host turn, after automatic plane initialization.
    // This function only prepares state; it never pretends that setup is PASS.
    function run() {
        if (app.project!==p || p.activeItem!==c || p.renderQueue.numItems!==0) throw Error('Owned context changed');
        var counts=[[1,1],[9,2],[50,50],[4,4]],times=[0,0.48,1];
        for(var ti=0;ti<times.length;ti++) {
            c.time=times[ti]; cols.setValue(4);rows.setValue(4);verifyKeys();capture('t'+ti+'-baseline',times[ti]);
            for(var ci=0;ci<counts.length;ci++) {
                cols.setValue(counts[ci][0]);rows.setValue(counts[ci][1]);verifyKeys();capture('t'+ti+'-c'+ci,times[ti]);
            }
        }
        var target=new File(root.fsName+'/density.aep'); if(target.exists)throw Error('Existing project');p.save(target);
        return 'CAPTURED_NOT_FULL_ACCEPTANCE: compare decoded pairs; verify loaded identity and real Grid Positions animation separately';
    }
    return run;
}
'''),
])
