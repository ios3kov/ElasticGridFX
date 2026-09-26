// ElasticGrid FX - DirectX shader prototype.
// Host supplies source texture + two 1D inverse warp LUTs.
Texture2D<float4> Src : register(t0);
Texture1D<float> XMap : register(t1);
Texture1D<float> YMap : register(t2);
SamplerState LinearSampler : register(s0);
RWTexture2D<float4> Dst : register(u0);

cbuffer Params : register(b0) {
    uint Width;
    uint Height;
    uint Quality; // 0 bilinear; bicubic path will use explicit taps
    uint _pad;
};

[numthreads(16, 16, 1)]
void CSMain(uint3 tid : SV_DispatchThreadID) {
    if (tid.x >= Width || tid.y >= Height) return;
    float u = (Width  > 1) ? (float)tid.x / (float)(Width  - 1) : 0.0;
    float v = (Height > 1) ? (float)tid.y / (float)(Height - 1) : 0.0;
    float su = XMap.SampleLevel(LinearSampler, u, 0);
    float sv = YMap.SampleLevel(LinearSampler, v, 0);
    Dst[tid.xy] = Src.SampleLevel(LinearSampler, float2(su, sv), 0);
}
