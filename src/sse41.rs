use super::*;
use core::mem;

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;
use overloads::*;
pub struct Sse41;
impl Simd for Sse41 {
    type Vi32 = I32x4_41;
    type Vf32 = F32x4;
    type Vf64 = F64x2;
    type Vi64 = I64x2_41;

    const VF32_WIDTH: usize = 4;
    const VF64_WIDTH: usize = 2;
    const VI32_WIDTH: usize = 4;
    const VI64_WIDTH: usize = 2;

    #[inline(always)]
    unsafe fn abs_ps(a: Self::Vf32) -> Self::Vf32 {
        let b = _mm_set1_ps(-0.0);
        F32x4(_mm_andnot_ps(a.0, b))
    }
    #[inline(always)]
    unsafe fn abs_pd(a: Self::Vf64) -> Self::Vf64 {
        let b = _mm_set1_pd(-0.0);
        F64x2(_mm_andnot_pd(a.0, b))
    }
    #[inline(always)]
    unsafe fn add_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x4_41(_mm_add_epi32(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn add_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_add_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn add_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        F64x2(_mm_add_pd(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn and_si(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x4_41(_mm_and_si128(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn andnot_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_andnot_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn andnot_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        F64x2(_mm_andnot_pd(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn andnot_si(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x4_41(_mm_andnot_si128(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn blendv_epi32(a: Self::Vi32, b: Self::Vi32, mask: Self::Vi32) -> Self::Vi32 {
        /*        I32x4_41(_mm_castps_si128(_mm_blendv_ps(
            _mm_castsi128_ps(a.0),
            _mm_castsi128_ps(b.0),
            _mm_castsi128_ps(mask.0),
        )))*/
        I32x4_41(_mm_blendv_epi8(a.0, b.0, mask.0))
    }
    #[inline(always)]
    unsafe fn blendv_ps(a: Self::Vf32, b: Self::Vf32, mask: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_blendv_ps(a.0, b.0, mask.0))
    }
    #[inline(always)]
    unsafe fn blendv_pd(a: Self::Vf64, b: Self::Vf64, mask: Self::Vf64) -> Self::Vf64 {
        F64x2(_mm_blendv_pd(a.0, b.0, mask.0))
    }
    #[inline(always)]
    unsafe fn castps_si(a: Self::Vf32) -> Self::Vi32 {
        I32x4_41(_mm_castps_si128(a.0))
    }
    #[inline(always)]
    unsafe fn castsi_ps(a: Self::Vi32) -> Self::Vf32 {
        F32x4(_mm_castsi128_ps(a.0))
    }
    #[inline(always)]
    unsafe fn castsi_pd(a: Self::Vi64) -> Self::Vf64 {
        F64x2(_mm_castsi128_pd(a.0))
    }
    #[inline(always)]
    unsafe fn cmpeq_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x4_41(_mm_cmpeq_epi32(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn cmpge_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_cmpge_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn cmpgt_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x4_41(_mm_cmpgt_epi32(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn cmpgt_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_cmpgt_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn cmplt_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_cmplt_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn cvtepi32_ps(a: Self::Vi32) -> Self::Vf32 {
        F32x4(_mm_cvtepi32_ps(a.0))
    }
    #[inline(always)]
    unsafe fn cvtps_epi32(a: Self::Vf32) -> Self::Vi32 {
        I32x4_41(_mm_cvtps_epi32(a.0))
    }
    #[inline(always)]
    unsafe fn floor_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_floor_ps(a.0))
    }
    #[inline(always)]
    unsafe fn floor_pd(a: Self::Vf64) -> Self::Vf64 {
        F64x2(_mm_floor_pd(a.0))
    }
    #[inline(always)]
    unsafe fn ceil_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_ceil_ps(a.0))
    }
    #[inline(always)]
    unsafe fn ceil_pd(a: Self::Vf64) -> Self::Vf64 {
        F64x2(_mm_ceil_pd(a.0))
    }
    #[inline(always)]
    unsafe fn fastfloor_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_floor_ps(a.0))
    }
    #[inline(always)]
    unsafe fn fmadd_ps(a: Self::Vf32, b: Self::Vf32, c: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_add_ps(_mm_mul_ps(a.0, b.0), c.0))
    }
    #[inline(always)]
    unsafe fn fnmadd_ps(a: Self::Vf32, b: Self::Vf32, c: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_add_ps(
            _mm_mul_ps(_mm_set1_ps(-1.0), _mm_mul_ps(a.0, b.0)),
            c.0,
        ))
    }
    #[inline(always)]
    unsafe fn horizontal_add_ps(a: Self::Vf32) -> f32 {
        let t1 = _mm_hadd_ps(a.0, a.0);
        _mm_cvtss_f32(_mm_hadd_ps(t1, t1))
    }
    #[inline(always)]
    unsafe fn horizontal_add_pd(a: Self::Vf64) -> f64 {
        _mm_cvtsd_f64(_mm_hadd_pd(a.0, a.0))
    }

    #[inline(always)]
    unsafe fn i32gather_epi32(arr: &[i32], index: Self::Vi32) -> Self::Vi32 {
        let index_as_arr = mem::transmute::<I32x4_41, [i32; 4]>(index);
        I32x4_41(_mm_set_epi32(
            arr[index_as_arr[3] as usize],
            arr[index_as_arr[2] as usize],
            arr[index_as_arr[1] as usize],
            arr[index_as_arr[0] as usize],
        ))
    }
    #[inline(always)]
    unsafe fn i32gather_ps(arr: &[f32], index: Self::Vi32) -> Self::Vf32 {
        let index_as_arr = mem::transmute::<I32x4_41, [i32; 4]>(index);
        F32x4(_mm_set_ps(
            arr[index_as_arr[3] as usize],
            arr[index_as_arr[2] as usize],
            arr[index_as_arr[1] as usize],
            arr[index_as_arr[0] as usize],
        ))
    }
    #[inline(always)]
    unsafe fn loadu_ps(a: &f32) -> Self::Vf32 {
        F32x4(_mm_loadu_ps(a as *const f32))
    }
    #[inline(always)]
    unsafe fn loadu_pd(a: &f64) -> Self::Vf64 {
        F64x2(_mm_loadu_pd(a as *const f64))
    }
    #[inline(always)]
    unsafe fn loadu_si(a: &i32) -> Self::Vi32 {
        let m = mem::transmute::<&i32, &__m128i>(a);
        I32x4_41(_mm_loadu_si128(m))
    }
    #[inline(always)]
    unsafe fn storeu_ps(a: &mut f32, b: Self::Vf32) {
        _mm_storeu_ps(a as *mut f32, b.0);
    }
    #[inline(always)]
    unsafe fn max_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x4_41(_mm_max_epi32(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn min_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x4_41(_mm_min_epi32(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn max_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_max_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn min_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_min_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn max_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        F64x2(_mm_max_pd(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn min_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        F64x2(_mm_min_pd(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn mul_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_mul_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn mul_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        F64x2(_mm_mul_pd(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn div_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_div_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn div_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        F64x2(_mm_div_pd(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn mullo_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x4_41(_mm_mullo_epi32(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn or_si(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x4_41(_mm_or_si128(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn or_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_or_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn or_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        F64x2(_mm_or_pd(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn round_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_round_ps(
            a.0,
            _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC,
        ))
    }
    #[inline(always)]
    unsafe fn round_pd(a: Self::Vf64) -> Self::Vf64 {
        F64x2(_mm_round_pd(
            a.0,
            _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC,
        ))
    }

    #[inline(always)]
    unsafe fn set1_epi32(a: i32) -> Self::Vi32 {
        I32x4_41(_mm_set1_epi32(a))
    }
    #[inline(always)]
    unsafe fn set1_ps(a: f32) -> Self::Vf32 {
        F32x4(_mm_set1_ps(a))
    }
    #[inline(always)]
    unsafe fn set1_pd(a: f64) -> Self::Vf64 {
        F64x2(_mm_set1_pd(a))
    }
    #[inline(always)]
    unsafe fn setzero_pd() -> Self::Vf64 {
        F64x2(_mm_setzero_pd())
    }
    #[inline(always)]
    unsafe fn setzero_ps() -> Self::Vf32 {
        F32x4(_mm_setzero_ps())
    }
    #[inline(always)]
    unsafe fn setzero_si() -> Self::Vi32 {
        I32x4_41(_mm_setzero_si128())
    }
    #[inline(always)]
    unsafe fn srai_epi32(a: Self::Vi32, imm8: i32) -> Self::Vi32 {
        macro_rules! call {
            ($imm8:expr) => {
                I32x4_41(_mm_srai_epi32(a.0, $imm8))
            };
        }
        constify_imm8!(imm8, call)
    }

    #[inline(always)]
    unsafe fn sub_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x4_41(_mm_sub_epi32(a.0, b.0))
    }
    #[inline(always)]

    unsafe fn sub_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_sub_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn sqrt_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_sqrt_ps(a.0))
    }
    #[inline(always)]
    unsafe fn rsqrt_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_rsqrt_ps(a.0))
    }
    #[inline(always)]
    unsafe fn sqrt_pd(a: Self::Vf64) -> Self::Vf64 {
        F64x2(_mm_sqrt_pd(a.0))
    }
    #[inline(always)]
    unsafe fn rsqrt_pd(a: Self::Vf64) -> Self::Vf64 {
        F64x2(_mm_div_pd(_mm_set1_pd(1.0), _mm_sqrt_pd(a.0)))
    }
    #[inline(always)]
    unsafe fn xor_si(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x4_41(_mm_xor_si128(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn xor_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_xor_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn xor_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        F64x2(_mm_xor_pd(a.0, b.0))
    }
}
