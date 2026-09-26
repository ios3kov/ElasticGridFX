// ElasticGrid FX - CUDA driver-compatible kernel source prototype.
// Build integration should prefer CUDA Driver API in the AE adapter.
extern "C" __global__ void elasticgrid_warp_bilinear(
    cudaTextureObject_t src,
    float4* dst,
    int width,
    int height,
    int dst_pitch_pixels,
    const float* xmap,
    const float* ymap)
{
    int x = blockIdx.x * blockDim.x + threadIdx.x;
    int y = blockIdx.y * blockDim.y + threadIdx.y;
    if (x >= width || y >= height) return;
    float su = xmap[x];
    float sv = ymap[y];
    dst[y * dst_pitch_pixels + x] = tex2D<float4>(src, su, sv);
}
