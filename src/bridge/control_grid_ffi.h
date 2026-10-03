#pragma once
#include "bridge/elasticgrid_ffi.h"

// UI-only sampling/editing of a retained deformation axis. Display counts
// never enter the render ABI and these functions never access AE or keyframes.
extern "C" {
int eg_control_axis_read(const float* lines, std::int32_t size,
                         std::int32_t visible_count, float easing, float distance,
                         float* positions, float* references, std::int32_t capacity) noexcept;
int eg_control_axis_drag(float* lines, const std::uint8_t* pins, std::int32_t size,
                         float reference, float delta, const EgElasticParams* elastic) noexcept;
int eg_control_axis_drag_live(float* lines,const std::uint8_t* pins,std::int32_t size,
                              float reference,float target,const EgElasticParams* elastic,
                              const EgRenderParams* render,std::int32_t columns) noexcept;
// Retained viewer layout: normalized source references, independent of render
// axes/keys. Reflow chooses references whose current destination is uniform.
int eg_control_axis_reflow(const float* lines, std::int32_t size,
                           std::int32_t visible_count, float easing, float distance,
                           float* normalized_refs, std::int32_t capacity) noexcept;
int eg_control_axis_read_layout(const float* lines, std::int32_t size,
                                const float* normalized_refs, std::int32_t count,
                                float easing, float distance, float* positions,
                                float* references, std::int32_t capacity) noexcept;
}
