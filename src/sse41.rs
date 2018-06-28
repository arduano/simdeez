use super::*;
use std::mem;

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

pub struct Sse41;
impl Simd for Sse41 {
    type Vi32 = __m128i;
    type Vf32 = __m128;

    const WIDTH_BYTES: usize = 4 * 4;
    unsafe fn set_lane_ps(a: Self::Vf32, value: f32, i: usize) {
        let mut arr = mem::transmute::<__m128, [f32; 4]>(a);
        arr[i] = value;
    }
    unsafe fn set_lane_epi32(a: Self::Vi32, value: i32, i: usize) {
        let mut arr = mem::transmute::<__m128i, [i32; 4]>(a);
        arr[i] = value;
    }
    #[inline(always)]
    unsafe fn get_lane_ps(a: Self::Vf32, i: usize) -> f32 {
        let arr = mem::transmute::<__m128, [f32; 4]>(a);
        arr[i]
    }
    #[inline(always)]
    unsafe fn get_lane_epi32(a: Self::Vi32, i: usize) -> i32 {
        let arr = mem::transmute::<__m128i, [i32; 4]>(a);
        arr[i]
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
        _mm_blendv_ps(a, b, mask)
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
        _mm_floor_ps(a)
    }
    #[inline(always)]
    unsafe fn fastfloor_ps(a: Self::Vf32) -> Self::Vf32 {
        _mm_floor_ps(a)
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
        _mm_loadu_ps(a as *const f32)
    }
    #[inline(always)]
    unsafe fn storeu_ps(a: &mut f32, b: Self::Vf32) {
        _mm_storeu_ps(a as *mut f32, b)
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
        _mm_mullo_epi32(a, b)
    }
    #[inline(always)]
    unsafe fn or_si(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        _mm_or_si128(a, b)
    }
    #[inline(always)]
    unsafe fn round_ps(a: Self::Vf32) -> Self::Vf32 {
        _mm_round_ps(a, _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC)
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
