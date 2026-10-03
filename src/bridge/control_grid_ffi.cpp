#include "bridge/control_grid_ffi.h"
#include "core/GridModel.h"
#include "core/WarpMath.h"
#include <algorithm>
#include <bit>
#include <cmath>
#include <limits>
#include <vector>

namespace {
constexpr int kMax = 52;  // 50 internal guides, two immutable endpoints.
bool same(float a, float b) noexcept {
    return std::bit_cast<std::uint32_t>(a) == std::bit_cast<std::uint32_t>(b);
}
bool valid(const float* lines, int size) noexcept {
    if (!lines || size < 3 || size > kMax || !same(lines[0], 0.0f) || !same(lines[size-1], 1.0f)) return false;
    for (int i = 1; i < size; ++i) {
        if (!std::isfinite(lines[i]) || !(lines[i] > lines[i-1])) return false;
    }
    return true;
}
std::vector<float> references(int size, int count) {
    const float last = static_cast<float>(size - 1);
    std::vector<float> out{0.0f, last};
    // First distribute existing knots evenly by maximin spacing. Prefixes are
    // nested, so lowering density removes controls, not deformation data.
    while (out.size() < static_cast<std::size_t>(std::min(size, count+2))) {
        float best = -1.0f;
        int chosen = 1;
        for (int i = 1; i < size-1; ++i) {
            const float v = static_cast<float>(i);
            float gap = last;
            for (float selected : out) gap = std::min(gap, std::abs(v-selected));
            if (gap > best) { best = gap; chosen = i; }
        }
        out.push_back(static_cast<float>(chosen));
    }
    std::sort(out.begin(), out.end());
    // Above the saved density retain every old knot and subdivide the widest
    // source interval. Deterministic tie order; no user state or time stored.
    while (out.size() < static_cast<std::size_t>(count+2)) {
        std::size_t chosen = 0;
        float longest = -1.0f;
        for (std::size_t i = 0; i+1 < out.size(); ++i) {
            const float length = out[i+1]-out[i];
            if (length > longest) { longest = length; chosen = i; }
        }
        const float middle = (out[chosen]+out[chosen+1])*0.5f;
        out.insert(out.begin()+static_cast<std::ptrdiff_t>(chosen+1), middle);
    }
    return out;
}
float position(const std::vector<float>& lines, float ref, float easing, float distance) {
    const auto lo = static_cast<std::size_t>(ref);
    if (same(ref, static_cast<float>(lo)) || lo+1 >= lines.size()) return lines[lo];
    const float t = ref-static_cast<float>(lo);
    if (easing <= 0.0f) return lines[lo]+(lines[lo+1]-lines[lo])*t;
    // Invert the existing renderer's map inside the known source segment.
    // This is viewer work only; the render curve/algorithm is never resampled.
    const float source = ref/static_cast<float>(lines.size()-1);
    float left = lines[lo], right = lines[lo+1];
    for (int step = 0; step < 28; ++step) {
        const float mid = (left+right)*0.5f;
        if (same(mid, left) || same(mid, right)) break;
        if (elasticgrid::inverseMapNormalized(mid, lines, easing, distance) < source) left = mid;
        else right = mid;
    }
    return (left+right)*0.5f;
}
float weight(float d, float radius, int falloff) {
    if (d <= 0.0f) return 1.0f;
    if (radius <= 0.0f || d >= radius) return 0.0f;
    const float t = d/radius;
    if (falloff == 2) return std::exp(-4.0f*t*t);
    if (falloff == 3) return 1.0f-t;
    return 1.0f-t*t*(3.0f-2.0f*t);
}
}

extern "C" int eg_control_axis_read(const float* lines, std::int32_t size,
                                     std::int32_t count, float easing, float distance,
                                     float* positions, float* refs, std::int32_t capacity) noexcept {
    try {
        if (!valid(lines,size) || count < 1 || count > 50 || capacity < count+2 ||
            !positions || !refs || !std::isfinite(easing) || !std::isfinite(distance)) return 1;
        const auto indices = references(size,count);
        const std::vector<float> saved(lines, lines+size);
        std::vector<float> result; result.reserve(indices.size());
        for (float ref : indices) result.push_back(position(saved, ref, easing, distance));
        // Do not publish a partial output if a numerical edge case is invalid.
        // Closely spaced virtual controls may round to the same float/pixel.
        // That is a display overlap, not malformed retained deformation.
        for (std::size_t i=0;i<result.size();++i) {
            if (!std::isfinite(result[i]) || (i>0 && result[i]<result[i-1])) return 4;
        }
        std::copy(result.begin(),result.end(),positions);
        std::copy(indices.begin(),indices.end(),refs);
        return 0;
    } catch (...) { return 4; }
}

extern "C" int eg_control_axis_reflow(const float* lines, std::int32_t size,
                                      std::int32_t count, float easing, float distance,
                                      float* refs, std::int32_t capacity) noexcept {
    try {
        if (!valid(lines,size) || count < 1 || count > 50 || capacity < count+2 ||
            !refs || !std::isfinite(easing) || !std::isfinite(distance)) return 1;
        const std::vector<float> saved(lines,lines+size);
        std::vector<float> result; result.reserve(count+2);
        for (int i=0;i<count+2;++i) {
            const float destination = static_cast<float>(i)/static_cast<float>(count+1);
            result.push_back(i==0 ? 0.0f : i==count+1 ? 1.0f :
                elasticgrid::inverseMapNormalized(destination,saved,easing,distance));
        }
        for (std::size_t i=0;i<result.size();++i)
            if (!std::isfinite(result[i]) || (i>0 && result[i]<=result[i-1])) return 4;
        std::copy(result.begin(),result.end(),refs);
        return 0;
    } catch (...) { return 4; }
}

extern "C" int eg_control_axis_read_layout(const float* lines, std::int32_t size,
                                           const float* normalized, std::int32_t count,
                                           float easing, float distance, float* positions,
                                           float* refs, std::int32_t capacity) noexcept {
    try {
        if (!valid(lines,size) || !normalized || count<3 || count>52 || capacity<count ||
            !positions || !refs || !std::isfinite(easing) || !std::isfinite(distance) ||
            normalized[0]!=0.0f || normalized[count-1]!=1.0f) return 1;
        const std::vector<float> saved(lines,lines+size);
        std::vector<float> result, references; result.reserve(count); references.reserve(count);
        for (int i=0;i<count;++i) {
            if (!std::isfinite(normalized[i]) || normalized[i]<0.0f || normalized[i]>1.0f ||
                (i>0 && normalized[i]<=normalized[i-1])) return 1;
            const float ref = normalized[i]*static_cast<float>(size-1);
            references.push_back(ref); result.push_back(position(saved,ref,easing,distance));
        }
        for (std::size_t i=0;i<result.size();++i)
            if (!std::isfinite(result[i]) || (i>0 && result[i]<result[i-1])) return 4;
        std::copy(result.begin(),result.end(),positions);
        std::copy(references.begin(),references.end(),refs);
        return 0;
    } catch (...) { return 4; }
}

extern "C" int eg_control_axis_drag(float* lines, const std::uint8_t* pins, std::int32_t size,
                                     float ref, float delta, const EgElasticParams* e) noexcept {
    try {
        if (!valid(lines,size) || !pins || !e || !std::isfinite(ref) ||
            ref <= 0.0f || ref >= static_cast<float>(size-1) || !std::isfinite(delta) ||
            !std::isfinite(e->tension_radius) || !std::isfinite(e->elasticity_strength) ||
            !std::isfinite(e->min_spacing)) return 1;
        if (pins[0] != 1 || pins[size-1] != 1) return 1;
        for (int i=1;i<size-1;++i) if (pins[i] != 0) return 1;
        if (std::abs(delta) <= 0.0f || e->elasticity_strength <= 0.0f) return 0;
        // Exact stored handles retain the established drag behavior.
        const auto lo = static_cast<std::size_t>(ref);
        if (same(ref,static_cast<float>(lo))) {
            std::vector<std::uint8_t> owned_pins(pins,pins+size);
            return eg_drag_axis(lines,owned_pins.data(),size,static_cast<int>(lo),lines[lo]+delta,e);
        }
        const auto n = static_cast<std::size_t>(size);
        const auto hi = lo+1;
        const float t = ref-static_cast<float>(lo);
        const float radius = std::clamp(e->tension_radius,0.0f,20.0f);
        const float strength = std::clamp(e->elasticity_strength,0.0f,2.0f);
        std::vector<float> weights(n,0.0f);
        for (std::size_t i=1;i+1<n;++i) weights[i] = weight(std::abs(static_cast<float>(i)-ref),radius,e->falloff);
        // An inserted handle controls its surrounding retained knots. Keep it
        // usable even with zero radius; never silently create a new lattice.
        if (lo > 0) weights[lo] = std::max(weights[lo],1.0f-t);
        if (hi+1 < n) weights[hi] = std::max(weights[hi],t);
        const float response = weights[lo]*(1.0f-t)+weights[hi]*t;
        if (!(response > 0.0f)) return 4;
        std::vector<float> candidate(lines,lines+size);
        for (std::size_t i=1;i+1<n;++i) candidate[i] += delta*strength*weights[i]/response;
        for (float v:candidate) if (!std::isfinite(v)) return 1;
        elasticgrid::AxisGrid::enforceMonotonic(candidate,e->min_spacing);
        if (!valid(candidate.data(),size)) return 4;
        std::copy(candidate.begin(),candidate.end(),lines);
        return 0;
    } catch (...) { return 4; }
}

extern "C" int eg_control_axis_drag_live(float* lines,const std::uint8_t* pins,std::int32_t size,
                                         float ref,float target,const EgElasticParams* e,
                                         const EgRenderParams* render,std::int32_t columns) noexcept {
    try{
        if(!valid(lines,size) || !pins || !e || !render || render->live_influence!=1 ||
           (columns!=0 && columns!=1) || !std::isfinite(target) || !std::isfinite(ref) ||
           ref<=0 || ref>=size-1 || !std::isfinite(e->elasticity_strength)) return 1;
        if((columns ? render->column_line_count : render->row_line_count)!=size) return 1;
        // Every trial starts from the stored, projected axis. No hidden overshoot
        // accumulates and no animated state is published before all checks pass.
        const std::vector<float> base(lines,lines+size);
        auto sample=[&](const std::vector<float>& axis,float& result){
            auto p=*render;
            if(columns){p.column_lines=axis.data();p.column_pins=pins;}
            else {p.row_lines=axis.data();p.row_pins=pins;}
            std::vector<float> x(52),y(52);
            int rc=eg_evaluate_grid(&p,x.data(),52,y.data(),52);
            if(rc) return rc;
            auto& evaluated=columns ? x : y;
            evaluated.resize(static_cast<std::size_t>(size));
            result=position(evaluated,ref,p.stretch_easing,p.easing_distance);
            return std::isfinite(result) ? 0 : 4;
        };
        float current=0;
        if(int rc=sample(base,current)) return rc;
        if(e->elasticity_strength<=0) return 0;
        target=std::clamp(target,0.0f,1.0f);
        if(std::abs(target-current)<2e-7f) return 0;
        const float sign=target>current ? 1.0f : -1.0f;
        auto best=base;
        float bestError=std::abs(target-current), low=0, high=1;
        for(int step=0;step<29;++step){
            const float amount=step==0 ? high : (low+high)*0.5f;
            auto trial=base;
            if(int rc=eg_control_axis_drag(trial.data(),pins,size,ref,sign*amount,e)) return rc;
            float value=0;
            if(int rc=sample(trial,value)) return rc;
            const float error=std::abs(target-value);
            if(error<bestError){bestError=error;best=std::move(trial);}
            if(bestError<2e-7f) break;
            if(sign*(value-target)<0) low=amount; else high=amount;
            if(step==0 && low==high) break; // Unreachable target: closest projected state.
        }
        if(!valid(best.data(),size)) return 4;
        std::copy(best.begin(),best.end(),lines);
        return 0;
    }catch(...){return 4;}
}
