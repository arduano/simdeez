use super::*;
#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
use std::mem;

pub struct Avx2;
impl Simd for Avx2 {
    type Vi32 = __m256i;
    type Vf32 = __m256;
    const WIDTH_BYTES: usize = 8 * 4;
    #[inline(always)]
    unsafe fn set_lane_ps(a: &mut Self::Vf32, value: f32, i: usize) {
        let arr = mem::transmute::<&mut __m256, &mut [f32; 8]>(a);
        arr[i] = value;
    }
    #[inline(always)]
    unsafe fn set_lane_epi32(a: &mut Self::Vi32, value: i32, i: usize) {
        let arr = mem::transmute::<&mut __m256i, &mut [i32; 8]>(a);
        arr[i] = value;
    }
    #[inline(always)]
    unsafe fn get_lane_ps(a: Self::Vf32, i: usize) -> f32 {
        let arr = mem::transmute::<__m256, [f32; 8]>(a);
        arr[i]
    }
    #[inline(always)]
    unsafe fn get_lane_epi32(a: Self::Vi32, i: usize) -> i32 {
        let arr = mem::transmute::<__m256i, [i32; 8]>(a);
        arr[i]
    }
    #[inline(always)]
    unsafe fn abs_ps(a: Self::Vf32) -> Self::Vf32 {
        let b = _mm256_set1_epi32(0x7fffffff);
        _mm256_and_ps(a, _mm256_castsi256_ps(b))
    }
    #[inline(always)]
    unsafe fn add_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        _mm256_add_epi32(a, b)
    }
    #[inline(always)]
    unsafe fn add_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        _mm256_add_ps(a, b)
    }
    #[inline(always)]
    unsafe fn and_si(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        _mm256_and_si256(a, b)
    }
    #[inline(always)]
    unsafe fn andnot_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        _mm256_andnot_ps(a, b)
    }
    #[inline(always)]
    unsafe fn andnot_si(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        _mm256_andnot_si256(a, b)
    }
    #[inline(always)]
    unsafe fn blendv_epi32(a: Self::Vi32, b: Self::Vi32, mask: Self::Vi32) -> Self::Vi32 {
        _mm256_or_si256(_mm256_andnot_si256(mask, a), _mm256_and_si256(mask, b))
    }
    #[inline(always)]
    unsafe fn blendv_ps(a: Self::Vf32, b: Self::Vf32, mask: Self::Vf32) -> Self::Vf32 {
        _mm256_blendv_ps(a, b, mask)
    }
    #[inline(always)]
    unsafe fn castps_si(a: Self::Vf32) -> Self::Vi32 {
        _mm256_castps_si256(a)
    }
    #[inline(always)]
    unsafe fn castsi_ps(a: Self::Vi32) -> Self::Vf32 {
        _mm256_castsi256_ps(a)
    }
    #[inline(always)]
    unsafe fn cmpeq_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        _mm256_cmpeq_epi32(a, b)
    }
    #[inline(always)]
    unsafe fn cmpge_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        _mm256_cmp_ps(a, b, _CMP_GE_OQ)
    }
    #[inline(always)]
    unsafe fn cmpgt_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        _mm256_cmpgt_epi32(a, b)
    }
    #[inline(always)]
    unsafe fn cmpgt_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        _mm256_cmp_ps(a, b, _CMP_GT_OQ)
    }
    #[inline(always)]
    unsafe fn cmplt_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        _mm256_cmp_ps(a, b, _CMP_LT_OQ)
    }
    #[inline(always)]
    unsafe fn cvtepi32_ps(a: Self::Vi32) -> Self::Vf32 {
        _mm256_cvtepi32_ps(a)
    }
    #[inline(always)]
    unsafe fn cvtps_epi32(a: Self::Vf32) -> Self::Vi32 {
        _mm256_cvtps_epi32(a)
    }
    #[inline(always)]
    unsafe fn ceil_ps(a: Self::Vf32) -> Self::Vf32 {
        _mm256_ceil_ps(a)
    }
    #[inline(always)]
    unsafe fn floor_ps(a: Self::Vf32) -> Self::Vf32 {
        _mm256_floor_ps(a)
    }
    #[inline(always)]
    unsafe fn fastfloor_ps(a: Self::Vf32) -> Self::Vf32 {
        _mm256_floor_ps(a)
    }
    #[inline(always)]
    unsafe fn fmadd_ps(a: Self::Vf32, b: Self::Vf32, c: Self::Vf32) -> Self::Vf32 {
        _mm256_fmadd_ps(a, b, c)
    }
    #[inline(always)]
    unsafe fn fnmadd_ps(a: Self::Vf32, b: Self::Vf32, c: Self::Vf32) -> Self::Vf32 {
        _mm256_fnmadd_ps(a, b, c)
    }
    #[inline(always)]
    unsafe fn i32gather_epi32(arr: &[i32], index: Self::Vi32) -> Self::Vi32 {
        _mm256_i32gather_epi32(&arr[0] as *const i32, index, 4)
    }
    #[inline(always)]
    unsafe fn i32gather_ps(arr: &[f32], index: Self::Vi32) -> Self::Vf32 {
        _mm256_i32gather_ps(&arr[0] as *const f32, index, 4)
    }
    #[inline(always)]
    unsafe fn loadu_ps(a: &f32) -> Self::Vf32 {
        _mm256_loadu_ps(a as *const f32)
    }
    #[inline(always)]
    unsafe fn loadu_si(a: &i32) -> Self::Vi32 {
        let m = mem::transmute::<&i32, &__m256i>(a);
        _mm256_loadu_si256(m)
    }
    #[inline(always)]
    unsafe fn storeu_ps(a: &mut f32, b: Self::Vf32) {
        _mm256_storeu_ps(a as *mut f32, b)
    }
    #[inline(always)]
    unsafe fn max_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        _mm256_max_ps(a, b)
    }
    #[inline(always)]
    unsafe fn min_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        _mm256_min_ps(a, b)
    }
    #[inline(always)]
    unsafe fn mul_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        _mm256_mul_ps(a, b)
    }
    #[inline(always)]
    unsafe fn div_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        _mm256_div_ps(a, b)
    }
    #[inline(always)]
    unsafe fn mullo_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        _mm256_mullo_epi32(a, b)
    }
    #[inline(always)]
    unsafe fn or_si(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        _mm256_or_si256(a, b)
    }
    #[inline(always)]
    unsafe fn round_ps(a: Self::Vf32) -> Self::Vf32 {
        _mm256_round_ps(a, _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC)
    }
    #[inline(always)]
    unsafe fn set1_epi32(a: i32) -> Self::Vi32 {
        _mm256_set1_epi32(a)
    }
    #[inline(always)]
    unsafe fn set1_ps(a: f32) -> Self::Vf32 {
        _mm256_set1_ps(a)
    }
    #[inline(always)]
    unsafe fn setzero_ps() -> Self::Vf32 {
        _mm256_setzero_ps()
    }
    #[inline(always)]
    unsafe fn setzero_si() -> Self::Vi32 {
        _mm256_setzero_si256()
    }
    #[inline(always)]
    unsafe fn srai_epi32(a: Self::Vi32, b: i32) -> Self::Vi32 {
        _mm256_srai_epi32(a, b)
    }
    #[inline(always)]
    unsafe fn sub_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        _mm256_sub_epi32(a, b)
    }
    #[inline(always)]
    unsafe fn sub_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        _mm256_sub_ps(a, b)
    }
    #[inline(always)]
    unsafe fn xor_si(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        _mm256_xor_si256(a, b)
    }
}
