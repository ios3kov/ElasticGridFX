#include <metal_stdlib>
using namespace metal;

// After Effects GPU worlds are BGRA128 float4 buffers. The warp is channel
// agnostic, so keeping float4 untouched preserves AE's BGRA channel order.
struct LinearSample {
    int i0;
    int i1;
    float t;
    float reserved;
};

struct CubicSample {
    int4 index;
    float4 weight;
};

struct WarpParams {
    int src_pitch_pixels;
    int dst_pitch_pixels;
    int src_width;
    int src_height;
    int dst_width;
    int dst_height;
    int reserved0;
    int reserved1;
};

// Transparent plan taps use -1; never form an out-of-bounds GPU load.
inline float4 source_pixel(device const float4* src, int x, int y, constant WarpParams& p) {
    if(x<0 || y<0 || x>=p.src_width || y>=p.src_height) return float4(0.0f);
    return src[y*p.src_pitch_pixels+x];
}

kernel void elasticgrid_warp_bilinear(
    device const float4* src [[buffer(0)]],
    device float4* dst [[buffer(1)]],
    device const LinearSample* x_plan [[buffer(2)]],
    device const LinearSample* y_plan [[buffer(3)]],
    constant WarpParams& p [[buffer(4)]],
    uint2 gid [[thread_position_in_grid]])
{
    if (gid.x >= uint(p.dst_width) || gid.y >= uint(p.dst_height)) return;

    const LinearSample xs = x_plan[gid.x];
    const LinearSample ys = y_plan[gid.y];

    const float4 p00 = source_pixel(src,xs.i0,ys.i0,p);
    const float4 p10 = source_pixel(src,xs.i1,ys.i0,p);
    const float4 p01 = source_pixel(src,xs.i0,ys.i1,p);
    const float4 p11 = source_pixel(src,xs.i1,ys.i1,p);

    const float4 top = mix(p00, p10, xs.t);
    const float4 bottom = mix(p01, p11, xs.t);
    dst[int(gid.y) * p.dst_pitch_pixels + int(gid.x)] = mix(top, bottom, ys.t);
}

kernel void elasticgrid_warp_bicubic(
    device const float4* src [[buffer(0)]],
    device float4* dst [[buffer(1)]],
    device const CubicSample* x_plan [[buffer(2)]],
    device const CubicSample* y_plan [[buffer(3)]],
    constant WarpParams& p [[buffer(4)]],
    uint2 gid [[thread_position_in_grid]])
{
    if (gid.x >= uint(p.dst_width) || gid.y >= uint(p.dst_height)) return;

    const CubicSample xs = x_plan[gid.x];
    const CubicSample ys = y_plan[gid.y];
    float4 acc = float4(0.0f);

    // Same sample order and precomputed Catmull-Rom weights as the CPU path.
    for (int ky = 0; ky < 4; ++ky) {
        const float wy = ys.weight[ky];
        for (int kx = 0; kx < 4; ++kx) {
            acc += source_pixel(src,xs.index[kx],ys.index[ky],p) * (wy * xs.weight[kx]);
        }
    }

    dst[int(gid.y) * p.dst_pitch_pixels + int(gid.x)] = acc;
}
