#import <Foundation/Foundation.h>
#import <Metal/Metal.h>

#include "bridge/elasticgrid_ffi.h"
#include "elasticgrid_metal_source.h"

#include <algorithm>
#include <array>
#include <atomic>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <new>
#include <limits>

namespace {

struct PlanBuffers {
    __strong id<MTLBuffer> x = nil;
    __strong id<MTLBuffer> y = nil;
    NSUInteger x_capacity = 0;
    NSUInteger y_capacity = 0;
    std::atomic<bool> busy{false};
};

struct MetalState {
    __strong id<MTLDevice> device = nil;
    __strong id<MTLCommandQueue> queue = nil;
    __strong id<MTLComputePipelineState> bilinear = nil;
    __strong id<MTLComputePipelineState> bicubic = nil;

    // One atomic word encodes teardown + active render count. The high bit is
    // the shutdown latch; the lower bits are the number of entered renders.
    // This removes mutex acquisition from every GPU frame while still making
    // setdown race-safe.
    std::atomic<std::uint32_t> lifecycle{0};

    // Fixed lock-free MFR pool. Slots are claimed with one CAS and retain their
    // MTLBuffers across frames, so steady-state GPU render has no malloc and no
    // pool mutex. 64 concurrent frames is far beyond practical AE MFR fan-out.
    std::array<PlanBuffers, 64> plan_pool{};
};

constexpr std::uint32_t kLifecycleShutdown = 0x80000000u;
constexpr std::uint32_t kLifecycleCountMask = 0x7fffffffu;

struct RenderActivity {
    MetalState* state = nullptr;
    bool active = false;

    explicit RenderActivity(MetalState* s) noexcept : state(s) {
        if (!state) return;
        std::uint32_t current = state->lifecycle.load(std::memory_order_acquire);
        for (;;) {
            if ((current & kLifecycleShutdown) != 0) return;
            if ((current & kLifecycleCountMask) == kLifecycleCountMask) return;
            if (state->lifecycle.compare_exchange_weak(
                    current, current + 1u,
                    std::memory_order_acq_rel, std::memory_order_acquire)) {
                active = true;
                return;
            }
        }
    }

    ~RenderActivity() {
        if (!state || !active) return;
        const std::uint32_t previous = state->lifecycle.fetch_sub(1u, std::memory_order_acq_rel);
        if ((previous & kLifecycleShutdown) != 0 && (previous & kLifecycleCountMask) == 1u) {
            state->lifecycle.notify_all();
        }
    }

    explicit operator bool() const noexcept { return active; }
};

PlanBuffers* acquire_plan_buffers(MetalState* state, NSUInteger x_bytes, NSUInteger y_bytes) noexcept {
    if (!state) return nullptr;
    @try {
        for (auto& slot : state->plan_pool) {
            bool expected = false;
            if (!slot.busy.compare_exchange_strong(
                    expected, true, std::memory_order_acq_rel, std::memory_order_relaxed)) {
                continue;
            }
            if (slot.x_capacity < x_bytes) {
                slot.x = [state->device newBufferWithLength:x_bytes options:MTLResourceStorageModeShared];
                slot.x_capacity = slot.x ? x_bytes : 0;
            }
            if (slot.y_capacity < y_bytes) {
                slot.y = [state->device newBufferWithLength:y_bytes options:MTLResourceStorageModeShared];
                slot.y_capacity = slot.y ? y_bytes : 0;
            }
            if (!slot.x || !slot.y) {
                slot.busy.store(false, std::memory_order_release);
                return nullptr;
            }
            return &slot;
        }
        return nullptr;
    } @catch (NSException* exception) {
        (void)exception;
        return nullptr;
    }
}

void release_plan_buffers(MetalState* state, PlanBuffers* slot) noexcept {
    (void)state;
    if (!slot) return;
    slot->busy.store(false, std::memory_order_release);
}

struct PlanLease {
    MetalState* state = nullptr;
    PlanBuffers* slot = nullptr;
    ~PlanLease() { release_plan_buffers(state, slot); }
};

struct MetalWarpParams {
    std::int32_t src_pitch_pixels;
    std::int32_t dst_pitch_pixels;
    std::int32_t src_width;
    std::int32_t src_height;
    std::int32_t dst_width;
    std::int32_t dst_height;
    std::int32_t reserved0;
    std::int32_t reserved1;
};
static_assert(sizeof(MetalWarpParams) == 32);
static_assert(sizeof(EgGpuLinearSample) == 16);
static_assert(sizeof(EgGpuCubicSample) == 32);

id<MTLComputePipelineState> make_pipeline(id<MTLDevice> device,
                                          id<MTLLibrary> library,
                                          NSString* name) {
    id<MTLFunction> fn = [library newFunctionWithName:name];
    if (!fn) return nil;
    NSError* error = nil;
    id<MTLComputePipelineState> pipeline = [device newComputePipelineStateWithFunction:fn error:&error];
    return pipeline;
}

bool valid_render_args(MetalState* state,
                       id<MTLBuffer> input,
                       std::ptrdiff_t input_row_bytes,
                       std::int32_t input_width,
                       std::int32_t input_height,
                       id<MTLBuffer> output,
                       std::ptrdiff_t output_row_bytes,
                       std::int32_t output_width,
                       std::int32_t output_height) noexcept {
    if (!(state && state->device && state->queue && input && output &&
          input_width > 0 && input_height > 0 && output_width > 0 && output_height > 0 &&
          input_row_bytes > 0 && output_row_bytes > 0 &&
          (input_row_bytes % 16) == 0 && (output_row_bytes % 16) == 0)) {
        return false;
    }

    const auto src_pitch = input_row_bytes / 16;
    const auto dst_pitch = output_row_bytes / 16;
    if (src_pitch > std::numeric_limits<std::int32_t>::max() ||
        dst_pitch > std::numeric_limits<std::int32_t>::max()) {
        return false;
    }
    if (src_pitch < input_width || dst_pitch < output_width) return false;

    auto buffer_fits = [](std::ptrdiff_t row_bytes, std::int32_t height, NSUInteger length) noexcept {
        const auto rb = static_cast<unsigned long long>(row_bytes);
        const auto h = static_cast<unsigned long long>(height);
        if (h != 0 && rb > std::numeric_limits<unsigned long long>::max() / h) return false;
        return rb * h <= static_cast<unsigned long long>(length);
    };
    return buffer_fits(input_row_bytes, input_height, [input length]) &&
           buffer_fits(output_row_bytes, output_height, [output length]);
}

int dispatch_impl(MetalState* state,
                  id<MTLBuffer> input,
                  std::ptrdiff_t input_row_bytes,
                  std::int32_t input_width,
                  std::int32_t input_height,
                  id<MTLBuffer> output,
                  std::ptrdiff_t output_row_bytes,
                  std::int32_t output_width,
                  std::int32_t output_height,
                  PlanBuffers* plan_buffers,
                  id<MTLComputePipelineState> pipeline) noexcept {
    if (!valid_render_args(state, input, input_row_bytes, input_width, input_height,
                           output, output_row_bytes, output_width, output_height) ||
        !plan_buffers || !plan_buffers->x || !plan_buffers->y || !pipeline) {
        return 1;
    }

    @autoreleasepool {
        const MetalWarpParams params{
            static_cast<std::int32_t>(input_row_bytes / 16),
            static_cast<std::int32_t>(output_row_bytes / 16),
            input_width,
            input_height,
            output_width,
            output_height,
            0,
            0,
        };

        id<MTLCommandBuffer> command_buffer = [state->queue commandBuffer];
        if (!command_buffer) return 3;
        id<MTLComputeCommandEncoder> encoder = [command_buffer computeCommandEncoder];
        if (!encoder) return 3;

        [encoder setComputePipelineState:pipeline];
        [encoder setBuffer:input offset:0 atIndex:0];
        [encoder setBuffer:output offset:0 atIndex:1];
        [encoder setBuffer:plan_buffers->x offset:0 atIndex:2];
        [encoder setBuffer:plan_buffers->y offset:0 atIndex:3];
        [encoder setBytes:&params length:sizeof(params) atIndex:4];

        const NSUInteger width = std::max<NSUInteger>(1, pipeline.threadExecutionWidth);
        const NSUInteger max_threads = std::max<NSUInteger>(width, pipeline.maxTotalThreadsPerThreadgroup);
        const NSUInteger height = std::max<NSUInteger>(1, std::min<NSUInteger>(16, max_threads / width));
        const MTLSize threads_per_group = MTLSizeMake(width, height, 1);
        const MTLSize groups = MTLSizeMake(
            (static_cast<NSUInteger>(output_width) + width - 1) / width,
            (static_cast<NSUInteger>(output_height) + height - 1) / height,
            1);

        [encoder dispatchThreadgroups:groups threadsPerThreadgroup:threads_per_group];
        [encoder endEncoding];
        [command_buffer commit];

        // AE may consume the output world as soon as this selector returns.
        // Wait for the single command buffer so correctness is deterministic;
        // there is still no CPU image readback/upload in this path.
        [command_buffer waitUntilCompleted];
        return command_buffer.status == MTLCommandBufferStatusCompleted ? 0 : 4;
    }
}

template <typename Sample>
int render_explicit_plan(MetalState* state,
                         id<MTLBuffer> input,
                         std::ptrdiff_t input_row_bytes,
                         std::int32_t input_width,
                         std::int32_t input_height,
                         id<MTLBuffer> output,
                         std::ptrdiff_t output_row_bytes,
                         std::int32_t output_width,
                         std::int32_t output_height,
                         const Sample* x_plan,
                         const Sample* y_plan,
                         id<MTLComputePipelineState> pipeline) noexcept {
    if (!valid_render_args(state, input, input_row_bytes, input_width, input_height,
                           output, output_row_bytes, output_width, output_height) ||
        !x_plan || !y_plan || !pipeline) {
        return 1;
    }

    @autoreleasepool {
        const NSUInteger x_bytes = static_cast<NSUInteger>(output_width) * sizeof(Sample);
        const NSUInteger y_bytes = static_cast<NSUInteger>(output_height) * sizeof(Sample);
        PlanBuffers* plan_buffers = acquire_plan_buffers(state, x_bytes, y_bytes);
        if (!plan_buffers) return 2;
        PlanLease lease{state, plan_buffers};
        std::memcpy([plan_buffers->x contents], x_plan, x_bytes);
        std::memcpy([plan_buffers->y contents], y_plan, y_bytes);
        return dispatch_impl(state, input, input_row_bytes, input_width, input_height,
                             output, output_row_bytes, output_width, output_height,
                             plan_buffers, pipeline);
    }
}

int render_direct_plan(MetalState* state,
                       id<MTLBuffer> input,
                       std::ptrdiff_t input_row_bytes,
                       std::int32_t input_width,
                       std::int32_t input_height,
                       id<MTLBuffer> output,
                       std::ptrdiff_t output_row_bytes,
                       std::int32_t output_width,
                       std::int32_t output_height,
                       const EgRenderParams* render_params) noexcept {
    if (!valid_render_args(state, input, input_row_bytes, input_width, input_height,
                           output, output_row_bytes, output_width, output_height) ||
        !render_params) {
        return 1;
    }

    @autoreleasepool {
        const bool cubic = render_params->quality == 2;
        const NSUInteger sample_size = cubic ? sizeof(EgGpuCubicSample) : sizeof(EgGpuLinearSample);
        const NSUInteger x_bytes = static_cast<NSUInteger>(output_width) * sample_size;
        const NSUInteger y_bytes = static_cast<NSUInteger>(output_height) * sample_size;
        PlanBuffers* plan_buffers = acquire_plan_buffers(state, x_bytes, y_bytes);
        if (!plan_buffers) return 2;
        PlanLease lease{state, plan_buffers};

        int prep = 0;
        if (cubic) {
            prep = eg_prepare_gpu_plan(
                input_width, input_height, output_width, output_height, render_params,
                nullptr, nullptr,
                static_cast<EgGpuCubicSample*>([plan_buffers->x contents]),
                static_cast<EgGpuCubicSample*>([plan_buffers->y contents]));
        } else {
            prep = eg_prepare_gpu_plan(
                input_width, input_height, output_width, output_height, render_params,
                static_cast<EgGpuLinearSample*>([plan_buffers->x contents]),
                static_cast<EgGpuLinearSample*>([plan_buffers->y contents]),
                nullptr, nullptr);
        }
        if (prep != 0) return prep == 1 || prep == 4 ? 1 : 5;

        return dispatch_impl(state, input, input_row_bytes, input_width, input_height,
                             output, output_row_bytes, output_width, output_height,
                             plan_buffers, cubic ? state->bicubic : state->bilinear);
    }
}

} // namespace

extern "C" void* eg_metal_create(void* device_ptr, void* command_queue_ptr) noexcept {
    @try {
        try {
            if (!device_ptr || !command_queue_ptr) return nullptr;
            @autoreleasepool {
                id<MTLDevice> device = (__bridge id<MTLDevice>)device_ptr;
                id<MTLCommandQueue> queue = (__bridge id<MTLCommandQueue>)command_queue_ptr;
                if (!device || !queue || queue.device != device) return nullptr;

                NSString* source = [[NSString alloc] initWithBytes:kElasticGridMetalSource
                                                           length:kElasticGridMetalSourceLength
                                                         encoding:NSUTF8StringEncoding];
                if (!source) return nullptr;
                NSError* compile_error = nil;
                id<MTLLibrary> library = [device newLibraryWithSource:source options:nil error:&compile_error];
                if (!library) return nullptr;

                auto* state = new (std::nothrow) MetalState();
                if (!state) return nullptr;
                state->device = device;
                state->queue = queue;
                state->bilinear = make_pipeline(device, library, @"elasticgrid_warp_bilinear");
                state->bicubic = make_pipeline(device, library, @"elasticgrid_warp_bicubic");
                if (!state->bilinear || !state->bicubic) {
                    delete state;
                    return nullptr;
                }
                return state;
            }
        } catch (...) {
            return nullptr;
        }
    } @catch (NSException* exception) {
        (void)exception;
        return nullptr;
    }
}

extern "C" void eg_metal_destroy(void* opaque) noexcept {
    @try {
        try {
            @autoreleasepool {
                auto* state = static_cast<MetalState*>(opaque);
                if (!state) return;
                state->lifecycle.fetch_or(kLifecycleShutdown, std::memory_order_acq_rel);
                std::uint32_t current = state->lifecycle.load(std::memory_order_acquire);
                while ((current & kLifecycleCountMask) != 0) {
                    state->lifecycle.wait(current, std::memory_order_acquire);
                    current = state->lifecycle.load(std::memory_order_acquire);
                }
                delete state;
            }
        } catch (...) {
            // Never allow cleanup failures to cross the host ABI.
        }
    } @catch (NSException* exception) {
        (void)exception;
    }
}

extern "C" int eg_metal_render(
    void* opaque,
    void* input_buffer,
    std::ptrdiff_t input_row_bytes,
    std::int32_t input_width,
    std::int32_t input_height,
    void* output_buffer,
    std::ptrdiff_t output_row_bytes,
    std::int32_t output_width,
    std::int32_t output_height,
    const EgRenderParams* params) noexcept {
    @try {
        try {
            auto* state = static_cast<MetalState*>(opaque);
            RenderActivity activity(state);
            if (!activity) return 7;
            return render_direct_plan(
                state,
                (__bridge id<MTLBuffer>)input_buffer,
                input_row_bytes,
                input_width,
                input_height,
                (__bridge id<MTLBuffer>)output_buffer,
                output_row_bytes,
                output_width,
                output_height,
                params);
        } catch (...) {
            return 6;
        }
    } @catch (NSException* exception) {
        (void)exception;
        return 6;
    }
}

extern "C" int eg_metal_render_linear(
    void* opaque,
    void* input_buffer,
    std::ptrdiff_t input_row_bytes,
    std::int32_t input_width,
    std::int32_t input_height,
    void* output_buffer,
    std::ptrdiff_t output_row_bytes,
    std::int32_t output_width,
    std::int32_t output_height,
    const EgGpuLinearSample* x_plan,
    const EgGpuLinearSample* y_plan) noexcept {
    @try {
        try {
            auto* state = static_cast<MetalState*>(opaque);
            RenderActivity activity(state);
            if (!activity) return 7;
            return render_explicit_plan(
                state,
                (__bridge id<MTLBuffer>)input_buffer,
                input_row_bytes,
                input_width,
                input_height,
                (__bridge id<MTLBuffer>)output_buffer,
                output_row_bytes,
                output_width,
                output_height,
                x_plan,
                y_plan,
                state ? state->bilinear : nil);
        } catch (...) {
            return 6;
        }
    } @catch (NSException* exception) {
        (void)exception;
        return 6;
    }
}

extern "C" int eg_metal_render_cubic(
    void* opaque,
    void* input_buffer,
    std::ptrdiff_t input_row_bytes,
    std::int32_t input_width,
    std::int32_t input_height,
    void* output_buffer,
    std::ptrdiff_t output_row_bytes,
    std::int32_t output_width,
    std::int32_t output_height,
    const EgGpuCubicSample* x_plan,
    const EgGpuCubicSample* y_plan) noexcept {
    @try {
        try {
            auto* state = static_cast<MetalState*>(opaque);
            RenderActivity activity(state);
            if (!activity) return 7;
            return render_explicit_plan(
                state,
                (__bridge id<MTLBuffer>)input_buffer,
                input_row_bytes,
                input_width,
                input_height,
                (__bridge id<MTLBuffer>)output_buffer,
                output_row_bytes,
                output_width,
                output_height,
                x_plan,
                y_plan,
                state ? state->bicubic : nil);
        } catch (...) {
            return 6;
        }
    } @catch (NSException* exception) {
        (void)exception;
        return 6;
    }
}
