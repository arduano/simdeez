use super::*;
#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
use std::mem;

pub struct Sse2;
impl Simd for Sse2 {
    type Vi32 = __m128i;
    type Vf32 = __m128;

    #[inline(always)]
    unsafe fn set_lane_ps(a: Self::Vf32, value: f32, i: usize) {
        let mut arr = mem::transmute::<__m128, [f32; 4]>(a);
        arr[i] = value;
    }
    #[inline(always)]
    unsafe fn set_lane_epi32(a: Self::Vi32, value: i32, i: usize) {
        let mut arr = mem::transmute::<__m128i, [i32; 4]>(a);
        arr[i] = value;
    }
    #[inline(always)]
    unsafe fn get_lane_ps(a: Self::Vf32, i: usize) -> f32 {
        let mut arr = mem::transmute::<__m128, [f32; 4]>(a);
        arr[i]
    }
    #[inline(always)]
    unsafe fn get_lane_epi32(a: Self::Vi32, i: usize) -> i32 {
        let mut arr = mem::transmute::<__m128i, [i32; 4]>(a);
        arr[i]
    }
    #[inline(always)]
    fn get_width_bytes() -> usize {
        4
    }
    #[inline(always)]
    unsafe fn abs_ps(a: Self::Vf32) -> Self::Vf32 {
        let b = _mm_set1_epi32(0x7fffffff);
        _mm_and_ps(a, _mm_castsi128_ps(b))
    }
    #[inline(always)]
    unsafe fn add_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        _mm_add_epi32(a, b)
    }
    #[inline(always)]
    unsafe fn add_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        _mm_add_ps(a, b)
    }
    #[inline(always)]
    unsafe fn and_si(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        _mm_and_si128(a, b)
    }
    #[inline(always)]
    unsafe fn andnot_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        _mm_andnot_ps(a, b)
    }
    #[inline(always)]
    unsafe fn blendv_epi32(a: Self::Vi32, b: Self::Vi32, mask: Self::Vi32) -> Self::Vi32 {
        _mm_or_si128(_mm_andnot_si128(mask, a), _mm_and_si128(mask, b))
    }
    #[inline(always)]
    unsafe fn blendv_ps(a: Self::Vf32, b: Self::Vf32, mask: Self::Vf32) -> Self::Vf32 {
        _mm_or_ps(_mm_andnot_ps(mask, a), _mm_and_ps(mask, b))
    }
    #[inline(always)]
    unsafe fn castps_si(a: Self::Vf32) -> Self::Vi32 {
        _mm_castps_si128(a)
    }
    #[inline(always)]
    unsafe fn castsi_ps(a: Self::Vi32) -> Self::Vf32 {
        _mm_castsi128_ps(a)
    }
    #[inline(always)]
    unsafe fn cmpeq_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        _mm_cmpeq_epi32(a, b)
    }
    #[inline(always)]
    unsafe fn cmpge_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        _mm_cmp_ps(a, b, _CMP_GE_OQ)
    }
    #[inline(always)]
    unsafe fn cmpgt_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        _mm_cmpgt_epi32(a, b)
    }
    #[inline(always)]
    unsafe fn cmpgt_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        _mm_cmp_ps(a, b, _CMP_GT_OQ)
    }
    #[inline(always)]
    unsafe fn cmplt_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        _mm_cmp_ps(a, b, _CMP_LT_OQ)
    }
    #[inline(always)]
    unsafe fn cvtepi32_ps(a: Self::Vi32) -> Self::Vf32 {
        _mm_cvtepi32_ps(a)
    }
    #[inline(always)]
    unsafe fn cvtps_epi32(a: Self::Vf32) -> Self::Vi32 {
        _mm_cvtps_epi32(a)
    }
    #[inline(always)]
    unsafe fn floor_ps(a: Self::Vf32) -> Self::Vf32 {
        let i = _mm_cvttps_epi32(a);
        let fi = _mm_cvtepi32_ps(i);
        let igx = _mm_cmpgt_ps(fi, a);
        _mm_sub_ps(fi, _mm_and_ps(igx, _mm_set1_ps(1.0)))
    }
    #[inline(always)]
    unsafe fn fastfloor_ps(a: Self::Vf32) -> Self::Vf32 {
        let t = _mm_cvtepi32_ps(_mm_cvttps_epi32(a));
        _mm_sub_ps(t, _mm_and_ps(_mm_cmplt_ps(a, t), _mm_set1_ps(1.0)))
    }
    #[inline(always)]
    unsafe fn fmadd_ps(a: Self::Vf32, b: Self::Vf32, c: Self::Vf32) -> Self::Vf32 {
        _mm_fmadd_ps(a, b, c)
    }
    #[inline(always)]
    unsafe fn fnmadd_ps(a: Self::Vf32, b: Self::Vf32, c: Self::Vf32) -> Self::Vf32 {
        _mm_fnmadd_ps(a, b, c)
    }
    #[inline(always)]
    unsafe fn i32gather_epi32(arr: &[i32], index: Self::Vi32) -> Self::Vi32 {
        let index_as_arr = mem::transmute::<__m128i, &[i32]>(index);
        _mm_set_epi32(
            arr[index_as_arr[0] as usize],
            arr[index_as_arr[1] as usize],
            arr[index_as_arr[2] as usize],
            arr[index_as_arr[3] as usize],
        )
    }
    #[inline(always)]
    unsafe fn i32gather_ps(arr: &[f32], index: Self::Vi32) -> Self::Vf32 {
        let index_as_arr = mem::transmute::<__m128i, &[i32]>(index);
        _mm_set_ps(
            arr[index_as_arr[0] as usize],
            arr[index_as_arr[1] as usize],
            arr[index_as_arr[2] as usize],
            arr[index_as_arr[3] as usize],
        )
    }
    #[inline(always)]
    unsafe fn loadu_ps(a: &f32) -> Self::Vf32 {
        _mm_loadu_ps(a)
    }
    #[inline(always)]
    unsafe fn max_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        _mm_max_ps(a, b)
    }
    #[inline(always)]
    unsafe fn min_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        _mm_min_ps(a, b)
    }
    #[inline(always)]
    unsafe fn mul_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        _mm_mul_ps(a, b)
    }
    #[inline(always)]
    unsafe fn mullo_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        let tmp1 = _mm_mul_epu32(a, b); /* mul 2,0*/
        let tmp2 = _mm_mul_epu32(_mm_srli_si128(a, 4), _mm_srli_si128(b, 4)); /* mul 3,1 */
        _mm_unpacklo_epi32(
            _mm_shuffle_epi32(tmp1, mm_shuffle!(0, 0, 2, 0) as i32),
            _mm_shuffle_epi32(tmp2, mm_shuffle!(0, 0, 2, 0) as i32),
        ) /* shuffle results to [63..0] and pack */
    }
    #[inline(always)]
    unsafe fn or_si(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        _mm_or_si128(a, b)
    }
    #[inline(always)]
    unsafe fn round_ps(a: Self::Vf32) -> Self::Vf32 {
        let v1 = _mm_cmpeq_ps(_mm_setzero_ps(), _mm_setzero_ps());
        let i = _mm_cvttps_epi32(a);
        let a_trunc = _mm_cvtepi32_ps(i); // truncate a
        let rmd = _mm_sub_ps(a, a_trunc); // get remainder
        let rmd2 = _mm_mul_ps(
            rmd,
            _mm_castsi128_ps(_mm_srli_epi32(_mm_castps_si128(v1), 2)),
        ); // mul remainder by near 2 will yield the needed offset
        let rmd2i = _mm_cvttps_epi32(rmd2); // after being truncated of course
        let rmd2_trunc = _mm_cvtepi32_ps(rmd2i);
        _mm_add_ps(a_trunc, rmd2_trunc)
    }
    #[inline(always)]
    unsafe fn set1_epi32(a: i32) -> Self::Vi32 {
        _mm_set1_epi32(a)
    }
    #[inline(always)]
    unsafe fn set1_ps(a: f32) -> Self::Vf32 {
        _mm_set1_ps(a)
    }
    #[inline(always)]
    unsafe fn setzero_ps() -> Self::Vf32 {
        _mm_setzero_ps()
    }
    #[inline(always)]
    unsafe fn setzero_si() -> Self::Vi32 {
        _mm_setzero_si128()
    }
    #[inline(always)]
    unsafe fn srai_epi32(a: Self::Vi32, imm8: i32) -> Self::Vi32 {
        macro_rules! call {
            ($imm8:expr) => {
                _mm_srai_epi32(a, $imm8)
            };
        }
        constify_imm8!(imm8, call)
    }

    #[inline(always)]
    unsafe fn storeu_ps(a: &mut f32, b: Self::Vf32) {
        _mm_storeu_ps(a, b)
    }
    #[inline(always)]
    unsafe fn sub_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        _mm_sub_epi32(a, b)
    }
    #[inline(always)]

    unsafe fn sub_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        _mm_sub_ps(a, b)
    }
    #[inline(always)]
    unsafe fn xor_si(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        _mm_xor_si128(a, b)
    }
}
