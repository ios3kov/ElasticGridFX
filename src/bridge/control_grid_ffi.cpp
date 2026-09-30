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
