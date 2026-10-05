#include "core/GridModel.h"
#include <algorithm>
#include <array>
#include <cmath>
#include <limits>

namespace elasticgrid {
namespace {
constexpr float kPi = 3.14159265358979323846f;

float clamp01(float x) { return std::clamp(x, 0.0f, 1.0f); }

float falloffWeight(float distance, float radius, FalloffProfile p) {
    if (distance <= 0.0f) return 1.0f;
    if (radius <= 0.0f || distance >= radius) return 0.0f;
    const float t = clamp01(distance / radius);
    switch (p) {
        case FalloffProfile::Linear:
            return 1.0f - t;
        case FalloffProfile::Smoothstep: {
            const float s = t * t * (3.0f - 2.0f * t);
            return 1.0f - s;
        }
        case FalloffProfile::Gaussian:
            // Original GridWarp: exp(-4*t^2).
            return std::exp(-4.0f * t * t);
        default:
            return 0.0f;
    }
}
} // namespace

AxisGrid::AxisGrid(std::size_t cells) { reset(cells); }

void AxisGrid::reset(std::size_t cells) {
    cells = std::clamp<std::size_t>(cells, 1, 128);
    lines_.resize(cells + 1);
    pins_.assign(cells + 1, 0);
    for (std::size_t i = 0; i <= cells; ++i) {
        lines_[i] = static_cast<float>(i) / static_cast<float>(cells);
    }
    pins_.front() = 1;
    pins_.back() = 1;
}

std::size_t AxisGrid::cells() const noexcept { return lines_.size() - 1; }
std::size_t AxisGrid::lineCount() const noexcept { return lines_.size(); }
const std::vector<float>& AxisGrid::lines() const noexcept { return lines_; }
const std::vector<std::uint8_t>& AxisGrid::pins() const noexcept { return pins_; }

bool AxisGrid::setPinned(std::size_t line, bool pinned) {
    if (line >= pins_.size() || line == 0 || line + 1 == pins_.size()) return false;
    pins_[line] = pinned ? 1 : 0;
    return true;
}

void AxisGrid::enforceMonotonic(std::vector<float>& values,
                                float min_spacing,
                                const std::vector<std::uint8_t>* pins) {
    if (values.size() < 2) return;
    const std::size_t n = values.size();
    // Original GridWarp caps requested minimum spacing at half the uniform
    // segment spacing and never lets it reach zero.
    const float max_feasible = 0.5f / static_cast<float>(n - 1);
    if (!std::isfinite(min_spacing)) min_spacing = 0.0f;
    min_spacing = std::max(1.0e-6f, std::min(min_spacing, max_feasible));

    values.front() = 0.0f;
    values.back() = 1.0f;

    // Clamp all values to the feasible envelope first.
    for (std::size_t i = 1; i + 1 < n; ++i) {
        const float lo = static_cast<float>(i) * min_spacing;
        const float hi = 1.0f - static_cast<float>((n - 1) - i) * min_spacing;
        values[i] = std::clamp(values[i], lo, hi);
    }

    // Forward/backward projection. Pinned values are treated as stronger anchors,
    // but we still preserve monotonicity if a caller provides an impossible state.
    for (std::size_t i = 1; i < n; ++i) {
        const float lo = values[i - 1] + min_spacing;
        if (values[i] < lo) values[i] = lo;
    }
    for (std::size_t i = n - 1; i-- > 0;) {
        const float hi = values[i + 1] - min_spacing;
        if (values[i] > hi) values[i] = hi;
    }

    if (pins && pins->size() == n) {
        // A second local pass keeps neighborhoods around pinned guides stable.
        for (std::size_t anchor = 1; anchor + 1 < n; ++anchor) {
            if (!(*pins)[anchor]) continue;
            for (std::size_t i = anchor; i-- > 0;) {
                const float hi = values[i + 1] - min_spacing;
                if (values[i] > hi) values[i] = hi;
            }
            for (std::size_t i = anchor + 1; i < n; ++i) {
                const float lo = values[i - 1] + min_spacing;
                if (values[i] < lo) values[i] = lo;
            }
        }
    }

    // Final defensive projection for legacy/internal-pin states. Canonical
    // GridWarp parity states have no internal pins, so this is normally a no-op.
    for (std::size_t i = 1; i + 1 < n; ++i) values[i] = clamp01(values[i]);
    values.front() = 0.0f;
    values.back() = 1.0f;
    for (std::size_t i = 1; i < n; ++i) {
        const float lo = values[i - 1] + min_spacing;
        if (values[i] < lo) values[i] = lo;
    }
    values.back() = 1.0f;
    for (std::size_t i = n - 1; i-- > 0;) {
        const float hi = values[i + 1] - min_spacing;
        if (values[i] > hi) values[i] = hi;
    }
    values.front() = 0.0f;
    values.back() = 1.0f;
}

bool AxisGrid::setLinePosition(std::size_t line, float position, float min_spacing) {
    if (line == 0 || line + 1 >= lines_.size() || pins_[line] || !std::isfinite(position)) return false;
    auto candidate = lines_;
    candidate[line] = clamp01(position);
    enforceMonotonic(candidate, min_spacing, &pins_);
    lines_.swap(candidate);
    return true;
}

bool AxisGrid::setState(std::span<const float> lines,
                        std::span<const std::uint8_t> pins,
                        float min_spacing) {
    if (lines.size() < 2 || lines.size() > 129 || pins.size() != lines.size()) return false;
    if (!std::isfinite(lines.front()) || !std::isfinite(lines.back())) return false;
    for (float v : lines) if (!std::isfinite(v)) return false;

    // Validate first, then reuse the member buffers. This preserves the old
    // non-mutation-on-invalid-input contract while avoiding two allocations on
    // every render-frame state load once capacity has been warmed.
    lines_.assign(lines.begin(), lines.end());
    pins_.assign(pins.begin(), pins.end());
    pins_.front() = 1;
    pins_.back() = 1;
    for (auto& pin : pins_) pin = pin ? 1 : 0;

    enforceMonotonic(lines_, min_spacing, &pins_);
    return std::abs(lines_.front()) <= 1e-6f && std::abs(lines_.back() - 1.0f) <= 1e-6f;
}

bool AxisGrid::dragElastic(std::size_t line, float target, const ElasticSettings& settings) {
    if (line == 0 || line + 1 >= lines_.size() || pins_[line] || !std::isfinite(target)) return false;

    auto candidate = lines_;
    // Original uses the raw normalized pointer coordinate. Values outside
    // 0..1 still influence neighboring guides before the spacing projection
    // constrains the final grid back inside the image.
    const float delta = target - lines_[line];
    const float radius = std::max(0.0f, std::isfinite(settings.radius_lines) ? settings.radius_lines : 3.0f);
    const float strength = std::clamp(std::isfinite(settings.strength) ? settings.strength : 1.0f, 0.0f, 2.0f);

    for (std::size_t i = 1; i + 1 < candidate.size(); ++i) {
        // Kept only for compatibility with our older saved states. Original
        // GridWarp never sets internal pins, so canonical parity behavior is unchanged.
        if (pins_[i] && i != line) continue;
        const float d = std::abs(static_cast<float>(i) - static_cast<float>(line));
        // The original applies Elasticity Strength to every moved guide,
        // including the grabbed guide itself (weight=1 at distance zero).
        const float w = falloffWeight(d, radius, settings.falloff);
        candidate[i] += delta * w * strength;
    }

    enforceMonotonic(candidate, settings.min_spacing, &pins_);
    lines_.swap(candidate);
    return true;
}

bool AxisGrid::influencedInto(std::vector<float>& out,float radius,float spacing) const {
    if(!std::isfinite(radius) || !std::isfinite(spacing)) return false;
    radius=std::clamp(radius,0.0f,20.0f);
    out=lines_;
    if(radius<=1.0f) return true;
    std::array<double,129> displacement{};
    bool neutral=true;
    for(std::size_t i=1;i+1<lines_.size();++i){
        const float uniform=static_cast<float>(i)/static_cast<float>(lines_.size()-1);
        displacement[i]=static_cast<double>(lines_[i])-static_cast<double>(uniform);
        neutral=neutral && displacement[i]==0.0;
    }
    if(neutral) return true;
    for(std::size_t i=1;i+1<lines_.size();++i){
        if(pins_[i]) continue;
        double sum=0.0,total=0.0;
        for(std::size_t j=0;j<lines_.size();++j){
            const float w=falloffWeight(std::abs(static_cast<float>(i)-static_cast<float>(j)),
                                        radius,FalloffProfile::Smoothstep);
            const double weight=static_cast<double>(w);
            sum+=weight*displacement[j]; total+=weight;
        }
        const float uniform=static_cast<float>(i)/static_cast<float>(lines_.size()-1);
        out[i]=static_cast<float>(static_cast<double>(uniform)+sum/total);
    }
    enforceMonotonic(out,spacing,&pins_);
    return true;
}

void AxisGrid::evaluatedInto(std::vector<float>& out,
                             const WaveSettings& wave,
                             float time_seconds,
                             float min_spacing,
                             bool apply_wave) const {
    out.assign(lines_.begin(), lines_.end());
    if (!wave.enabled || !apply_wave || !std::isfinite(wave.amplitude) ||
        !std::isfinite(wave.frequency) || !std::isfinite(wave.phase_degrees) ||
        !std::isfinite(wave.speed_cycles_per_second) || !std::isfinite(time_seconds) ||
        std::abs(wave.amplitude) < 1e-8f) {
        return;
    }

    for (std::size_t i = 1; i + 1 < out.size(); ++i) {
        if (pins_[i]) continue;
        // Evaluate cycles in double and wrap before sin(). This remains stable
        // for very large finite times/frequencies and avoids float overflow.
        const double phase =
            2.0 * static_cast<double>(kPi) *
                (static_cast<double>(wave.frequency) * static_cast<double>(lines_[i]) +
                 static_cast<double>(wave.speed_cycles_per_second) * static_cast<double>(time_seconds)) +
            static_cast<double>(wave.phase_degrees) * (static_cast<double>(kPi) / 180.0);
        const float uniform_spacing = 1.0f / static_cast<float>(lines_.size() - 1);
        const float displacement = wave.amplitude * 0.4f * uniform_spacing;
        out[i] += displacement * static_cast<float>(std::sin(phase));
    }
    enforceMonotonic(out, min_spacing, &pins_);
}

std::vector<float> AxisGrid::evaluated(const WaveSettings& wave,
                                       float time_seconds,
                                       float min_spacing,
                                       bool apply_wave) const {
    std::vector<float> out;
    evaluatedInto(out, wave, time_seconds, min_spacing, apply_wave);
    return out;
}

} // namespace elasticgrid
