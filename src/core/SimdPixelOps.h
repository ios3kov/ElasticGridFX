#pragma once

#include <cstdint>
#include <cstring>

#if defined(__aarch64__) && defined(__ARM_NEON)
  #include <arm_neon.h>
  #define ELASTICGRID_SIMD_PIXELS 1
#elif defined(__x86_64__) || defined(_M_X64)
  #include <immintrin.h>
  #define ELASTICGRID_SIMD_PIXELS 1
#else
  #define ELASTICGRID_SIMD_PIXELS 0
#endif

namespace elasticgrid::simd {

#if defined(__aarch64__) && defined(__ARM_NEON)
using Vec4f = float32x4_t;

inline Vec4f zero() noexcept { return vdupq_n_f32(0.0f); }
inline Vec4f load4(const std::uint8_t* p) noexcept {
    std::uint32_t packed = 0;
    std::memcpy(&packed, p, sizeof(packed));
    const uint8x8_t bytes = vreinterpret_u8_u32(vdup_n_u32(packed));
    return vcvtq_f32_u32(vmovl_u16(vget_low_u16(vmovl_u8(bytes))));
}
inline Vec4f load4(const std::uint16_t* p) noexcept {
    return vcvtq_f32_u32(vmovl_u16(vld1_u16(p)));
}
inline Vec4f load4(const float* p) noexcept { return vld1q_f32(p); }
inline Vec4f lerp(Vec4f a, Vec4f b, float t) noexcept {
    return vmlaq_n_f32(a, vsubq_f32(b, a), t);
}
inline Vec4f madd(Vec4f acc, Vec4f v, float scalar) noexcept {
    return vmlaq_n_f32(acc, v, scalar);
}
inline void store4(std::uint8_t* p, Vec4f v, float) noexcept {
    v = vmaxq_f32(vdupq_n_f32(0.0f), vminq_f32(vdupq_n_f32(255.0f), v));
    const uint16x4_t u16 = vqmovn_u32(vcvtnq_u32_f32(v));
    const uint8x8_t u8 = vqmovn_u16(vcombine_u16(u16, u16));
    const std::uint32_t packed = vget_lane_u32(vreinterpret_u32_u8(u8), 0);
    std::memcpy(p, &packed, sizeof(packed));
}
inline void store4(std::uint16_t* p, Vec4f v, float maxv) noexcept {
    v = vmaxq_f32(vdupq_n_f32(0.0f), vminq_f32(vdupq_n_f32(maxv), v));
    vst1_u16(p, vqmovn_u32(vcvtnq_u32_f32(v)));
}
inline void store4(float* p, Vec4f v) noexcept { vst1q_f32(p, v); }

#elif defined(__x86_64__) || defined(_M_X64)
using Vec4f = __m128;

inline Vec4f zero() noexcept { return _mm_setzero_ps(); }
inline Vec4f load4(const std::uint8_t* p) noexcept {
    std::uint32_t packed = 0;
    std::memcpy(&packed, p, sizeof(packed));
    const __m128i v8 = _mm_cvtsi32_si128(static_cast<int>(packed));
    const __m128i z = _mm_setzero_si128();
    return _mm_cvtepi32_ps(_mm_unpacklo_epi16(_mm_unpacklo_epi8(v8, z), z));
}
inline Vec4f load4(const std::uint16_t* p) noexcept {
    std::uint64_t packed = 0;
    std::memcpy(&packed, p, sizeof(packed));
    const __m128i v16 = _mm_cvtsi64_si128(static_cast<long long>(packed));
    return _mm_cvtepi32_ps(_mm_unpacklo_epi16(v16, _mm_setzero_si128()));
}
inline Vec4f load4(const float* p) noexcept { return _mm_loadu_ps(p); }
inline Vec4f lerp(Vec4f a, Vec4f b, float t) noexcept {
    return _mm_add_ps(a, _mm_mul_ps(_mm_sub_ps(b, a), _mm_set1_ps(t)));
}
inline Vec4f madd(Vec4f acc, Vec4f v, float scalar) noexcept {
    return _mm_add_ps(acc, _mm_mul_ps(v, _mm_set1_ps(scalar)));
}
inline void store4(std::uint8_t* p, Vec4f v, float) noexcept {
    v = _mm_max_ps(_mm_setzero_ps(), _mm_min_ps(_mm_set1_ps(255.0f), v));
    alignas(16) std::int32_t tmp[4];
    _mm_store_si128(reinterpret_cast<__m128i*>(tmp), _mm_cvtps_epi32(v));
    p[0u]=static_cast<std::uint8_t>(tmp[0u]); p[1u]=static_cast<std::uint8_t>(tmp[1u]);
    p[2u]=static_cast<std::uint8_t>(tmp[2u]); p[3u]=static_cast<std::uint8_t>(tmp[3u]);
}
inline void store4(std::uint16_t* p, Vec4f v, float maxv) noexcept {
    v = _mm_max_ps(_mm_setzero_ps(), _mm_min_ps(_mm_set1_ps(maxv), v));
    alignas(16) std::int32_t tmp[4];
    _mm_store_si128(reinterpret_cast<__m128i*>(tmp), _mm_cvtps_epi32(v));
    p[0u]=static_cast<std::uint16_t>(tmp[0u]); p[1u]=static_cast<std::uint16_t>(tmp[1u]);
    p[2u]=static_cast<std::uint16_t>(tmp[2u]); p[3u]=static_cast<std::uint16_t>(tmp[3u]);
}
inline void store4(float* p, Vec4f v) noexcept { _mm_storeu_ps(p, v); }
#endif

} // namespace elasticgrid::simd
