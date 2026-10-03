#pragma once
#include <cstddef>
#include <cstdint>

#ifdef __cplusplus
extern "C" {
#endif

typedef std::int32_t (*EgAbortFn)(void* refcon);

typedef struct EgRenderParams {
    std::int32_t columns;
    std::int32_t rows;
    const float* column_lines;
    std::int32_t column_line_count;
    const std::uint8_t* column_pins;
    const float* row_lines;
    std::int32_t row_line_count;
    const std::uint8_t* row_pins;
    float tension_radius;
    std::int32_t falloff;
    float elasticity_strength;
    float min_spacing;
    float stretch_easing;
    float easing_distance;
    std::int32_t wave_enabled;
    float wave_amplitude;
    float wave_frequency;
    float wave_phase;
    float wave_speed;
    std::int32_t wave_axis;
    std::int32_t edge_mode;
    std::int32_t quality;
    float time_seconds;
    std::uint32_t threads;
    // SmartFX spatial context in rendered-pixel layer coordinates. Zero canvas
    // dimensions mean "use output dimensions" for legacy/full-frame callers.
    std::int32_t canvas_width;
    std::int32_t canvas_height;
    std::int32_t input_origin_x;
    std::int32_t input_origin_y;
    std::int32_t output_origin_x;
    std::int32_t output_origin_y;
    EgAbortFn abort_fn;
    void* abort_refcon;
    // Internal coordinated host/core ABI: new live field, frozen parity off=0.
    std::int32_t live_influence;
} EgRenderParams;

typedef struct EgElasticParams {
    float tension_radius;
    std::int32_t falloff;
    float elasticity_strength;
    float min_spacing;
} EgElasticParams;

// GPU sampling plans intentionally mirror the CPU prepared plan. Keeping all
// edge resolution and interpolation weights on the CPU makes Metal and CPU use
// the same geometry math, avoiding a separate lower-quality GPU algorithm.
typedef struct EgGpuLinearSample {
    std::int32_t i0;
    std::int32_t i1;
    float t;
    float reserved;
} EgGpuLinearSample;

typedef struct EgGpuCubicSample {
    std::int32_t index[4];
    float weight[4];
} EgGpuCubicSample;

// Evaluates immutable destination guides, including wave, using the legacy path.
// Output capacities must cover columns+2 and rows+2; no retained pointers.
int eg_evaluate_grid(const EgRenderParams* params, float* columns, int column_capacity,
                     float* rows, int row_capacity) noexcept;

// Mutates one normalized guide axis in place using the same clean-room
// elasticity/no-crossing implementation used by the render core.
// `line_count` includes the two boundary lines. Returns 0 on success.
int eg_drag_axis(
    float* lines,
    std::uint8_t* pins,
    std::int32_t line_count,
    std::int32_t line_index,
    float target,
    const EgElasticParams* params) noexcept;

// Builds the exact same sampling plan used by the CPU renderer. For bilinear
// quality, x/y_linear must provide dst_width/dst_height elements and cubic
// pointers may be null. For bicubic, x/y_cubic are required and linear
// pointers may be null. Returns 0 on success.
int eg_prepare_gpu_plan(
    std::int32_t input_width,
    std::int32_t input_height,
    std::int32_t output_width,
    std::int32_t output_height,
    const EgRenderParams* params,
    EgGpuLinearSample* x_linear,
    EgGpuLinearSample* y_linear,
    EgGpuCubicSample* x_cubic,
    EgGpuCubicSample* y_cubic) noexcept;


typedef struct EgRectI32 {
    std::int32_t left;
    std::int32_t top;
    std::int32_t right;
    std::int32_t bottom;
} EgRectI32;

// Compute the exact rectangular source checkout needed for an output ROI.
// The result includes interpolation support pixels. Wrap/Mirror may naturally
// expand the rectangle to the full canvas because samples can cross edges.
int eg_required_source_rect(
    std::int32_t canvas_width,
    std::int32_t canvas_height,
    EgRectI32 output_rect,
    const EgRenderParams* params,
    EgRectI32* source_rect) noexcept;

// Returns 0 on success. Pixel buffers are After Effects ARGB on CPU; the core
// treats the four channels generically, so channel ordering is preserved.
int eg_render_frame(
    const void* input_data,
    std::ptrdiff_t input_row_bytes,
    std::int32_t input_width,
    std::int32_t input_height,
    void* output_data,
    std::ptrdiff_t output_row_bytes,
    std::int32_t output_width,
    std::int32_t output_height,
    std::int32_t bit_depth,
    const EgRenderParams* params) noexcept;

// CPU SmartFX sparse-canvas entry. The host must have requested the complete
// logical canvas: pixels absent from its returned compact world are zero.
// Requires positive canvas dimensions; output outside that canvas is zero.
// Edge modes apply to the logical canvas, NOT the compact storage rectangle.
// An empty input (width==0 or height==0) clears output without reading input.
// Existing eg_render_frame and GPU-plan semantics/ABI remain unchanged.
int eg_render_frame_sparse(
    const void* input_data,
    std::ptrdiff_t input_row_bytes,
    std::int32_t input_width,
    std::int32_t input_height,
    void* output_data,
    std::ptrdiff_t output_row_bytes,
    std::int32_t output_width,
    std::int32_t output_height,
    std::int32_t bit_depth,
    const EgRenderParams* params) noexcept;

#ifdef __APPLE__
// Metal backend. AE passes id<MTLDevice>, id<MTLCommandQueue>, and id<MTLBuffer>
// as opaque pointers; keeping them opaque here avoids leaking Objective-C types
// into the portable core/host ABI.
void* eg_metal_create(void* device, void* command_queue) noexcept;
void eg_metal_destroy(void* state) noexcept;

// Optimized Metal entry point used by the AE host. The image buffers stay
// AE-owned on the GPU, and the CPU sampling plan is written directly into the
// pooled shared MTLBuffers (no intermediate Rust Vec + memcpy).
int eg_metal_render(
    void* state,
    void* input_buffer,
    std::ptrdiff_t input_row_bytes,
    std::int32_t input_width,
    std::int32_t input_height,
    void* output_buffer,
    std::ptrdiff_t output_row_bytes,
    std::int32_t output_width,
    std::int32_t output_height,
    const EgRenderParams* params) noexcept;
int eg_metal_render_linear(
    void* state,
    void* input_buffer,
    std::ptrdiff_t input_row_bytes,
    std::int32_t input_width,
    std::int32_t input_height,
    void* output_buffer,
    std::ptrdiff_t output_row_bytes,
    std::int32_t output_width,
    std::int32_t output_height,
    const EgGpuLinearSample* x_plan,
    const EgGpuLinearSample* y_plan) noexcept;
int eg_metal_render_cubic(
    void* state,
    void* input_buffer,
    std::ptrdiff_t input_row_bytes,
    std::int32_t input_width,
    std::int32_t input_height,
    void* output_buffer,
    std::ptrdiff_t output_row_bytes,
    std::int32_t output_width,
    std::int32_t output_height,
    const EgGpuCubicSample* x_plan,
    const EgGpuCubicSample* y_plan) noexcept;
#endif

#ifdef __cplusplus
}
#endif
