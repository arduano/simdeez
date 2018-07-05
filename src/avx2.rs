use super::*;
use core::mem;
use overloads::*;

pub struct Avx2;
impl Simd for Avx2 {
    type Vi32 = I32x8;
    type Vf32 = F32x8;
    type Vf64 = F64x4;
    type Vi64 = I64x4;

    const VF32_WIDTH: usize = 8;
    const VF64_WIDTH: usize = 4;
    const VI32_WIDTH: usize = 8;
    const VI64_WIDTH: usize = 4;

    #[inline(always)]
    unsafe fn abs_ps(a: Self::Vf32) -> Self::Vf32 {
        let b = _mm256_set1_ps(-0.0);
        F32x8(_mm256_andnot_ps(a.0, b))
    }
    #[inline(always)]
    unsafe fn abs_pd(a: Self::Vf64) -> Self::Vf64 {
        let b = _mm256_set1_pd(-0.0);
        F64x4(_mm256_andnot_pd(a.0, b))
    }
    #[inline(always)]
    unsafe fn add_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x8(_mm256_add_epi32(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn add_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_add_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn add_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        F64x4(_mm256_add_pd(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn and_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x8(_mm256_and_si256(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn and_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        I64x4(_mm256_and_si256(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn andnot_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_andnot_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn andnot_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        F64x4(_mm256_andnot_pd(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn andnot_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x8(_mm256_andnot_si256(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn andnot_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        I64x4(_mm256_andnot_si256(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn blendv_epi32(a: Self::Vi32, b: Self::Vi32, mask: Self::Vi32) -> Self::Vi32 {
        I32x8(_mm256_castps_si256(_mm256_blendv_ps(
            _mm256_castsi256_ps(a.0),
            _mm256_castsi256_ps(b.0),
            _mm256_castsi256_ps(mask.0),
        )))
    }
    #[inline(always)]
    unsafe fn blendv_ps(a: Self::Vf32, b: Self::Vf32, mask: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_blendv_ps(a.0, b.0, mask.0))
    }
    #[inline(always)]
    unsafe fn blendv_pd(a: Self::Vf64, b: Self::Vf64, mask: Self::Vf64) -> Self::Vf64 {
        F64x4(_mm256_blendv_pd(a.0, b.0, mask.0))
    }
    #[inline(always)]
    unsafe fn castps_epi32(a: Self::Vf32) -> Self::Vi32 {
        I32x8(_mm256_castps_si256(a.0))
    }
    #[inline(always)]
    unsafe fn castpd_epi64(a: Self::Vf64) -> Self::Vi64 {
        I64x4(_mm256_castpd_si256(a.0))
    }
    #[inline(always)]
    unsafe fn castepi32_ps(a: Self::Vi32) -> Self::Vf32 {
        F32x8(_mm256_castsi256_ps(a.0))
    }
    #[inline(always)]
    unsafe fn castepi64_pd(a: Self::Vi64) -> Self::Vf64 {
        F64x4(_mm256_castsi256_pd(a.0))
    }
    #[inline(always)]
    unsafe fn cmpeq_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x8(_mm256_cmpeq_epi32(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn cmpneq_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        Self::not_epi32(I32x8(_mm256_cmpeq_epi32(a.0, b.0)))
    }
    #[inline(always)]
    unsafe fn cmpge_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        Self::not_epi32(I32x8(_mm256_cmpgt_epi32(b.0, a.0)))
    }
    #[inline(always)]
    unsafe fn cmpgt_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x8(_mm256_cmpgt_epi32(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn cmple_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        Self::not_epi32(I32x8(_mm256_cmpgt_epi32(a.0, b.0)))
    }
    #[inline(always)]
    unsafe fn cmplt_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x8(_mm256_cmpgt_epi32(b.0, a.0))
    }
    #[inline(always)]
    unsafe fn cmpeq_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_cmp_ps(a.0, b.0, _CMP_EQ_OQ))
    }
    #[inline(always)]
    unsafe fn cmpneq_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_cmp_ps(a.0, b.0, _CMP_NEQ_OQ))
    }
    #[inline(always)]
    unsafe fn cmpge_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_cmp_ps(a.0, b.0, _CMP_GE_OQ))
    }
    #[inline(always)]
    unsafe fn cmpgt_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_cmp_ps(a.0, b.0, _CMP_GT_OQ))
    }
    #[inline(always)]
    unsafe fn cmple_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_cmp_ps(a.0, b.0, _CMP_LE_OQ))
    }
    #[inline(always)]
    unsafe fn cmplt_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_cmp_ps(a.0, b.0, _CMP_LT_OQ))
    }
    #[inline(always)]
    unsafe fn cvtepi32_ps(a: Self::Vi32) -> Self::Vf32 {
        F32x8(_mm256_cvtepi32_ps(a.0))
    }
    #[inline(always)]
    unsafe fn cvtps_epi32(a: Self::Vf32) -> Self::Vi32 {
        I32x8(_mm256_cvtps_epi32(a.0))
    }
    #[inline(always)]
    unsafe fn ceil_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_ceil_ps(a.0))
    }
    #[inline(always)]
    unsafe fn ceil_pd(a: Self::Vf64) -> Self::Vf64 {
        F64x4(_mm256_ceil_pd(a.0))
    }
    #[inline(always)]
    unsafe fn floor_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_floor_ps(a.0))
    }
    #[inline(always)]
    unsafe fn floor_pd(a: Self::Vf64) -> Self::Vf64 {
        F64x4(_mm256_floor_pd(a.0))
    }
    #[inline(always)]
    unsafe fn fastfloor_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_floor_ps(a.0))
    }
    #[inline(always)]
    unsafe fn fmadd_ps(a: Self::Vf32, b: Self::Vf32, c: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_fmadd_ps(a.0, b.0, c.0))
    }
    #[inline(always)]
    unsafe fn fnmadd_ps(a: Self::Vf32, b: Self::Vf32, c: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_fnmadd_ps(a.0, b.0, c.0))
    }
    // TODO: test this
    #[inline(always)]
    unsafe fn horizontal_add_ps(a: Self::Vf32) -> f32 {
        let t1 = _mm256_hadd_ps(a.0, a.0);
        let t2 = _mm256_hadd_ps(t1, t1);
        _mm256_cvtss_f32(_mm256_hadd_ps(t2, t2))
    }
    #[inline(always)]
    unsafe fn horizontal_add_pd(a: Self::Vf64) -> f64 {
        let t1 = F64x4(_mm256_hadd_pd(a.0, a.0));
        t1[0] + t1[2]
    }
    #[inline(always)]
    unsafe fn i32gather_epi32(arr: &[i32], index: Self::Vi32) -> Self::Vi32 {
        I32x8(_mm256_i32gather_epi32(&arr[0] as *const i32, index.0, 4))
    }
    #[inline(always)]
    unsafe fn i32gather_ps(arr: &[f32], index: Self::Vi32) -> Self::Vf32 {
        F32x8(_mm256_i32gather_ps(&arr[0] as *const f32, index.0, 4))
    }
    #[inline(always)]
    unsafe fn loadu_ps(a: &f32) -> Self::Vf32 {
        F32x8(_mm256_loadu_ps(a as *const f32))
    }
    #[inline(always)]
    unsafe fn loadu_pd(a: &f64) -> Self::Vf64 {
        F64x4(_mm256_loadu_pd(a as *const f64))
    }
    #[inline(always)]
    unsafe fn loadu_si(a: &i32) -> Self::Vi32 {
        let m = mem::transmute::<&i32, &__m256i>(a);
        I32x8(_mm256_loadu_si256(m))
    }
    #[inline(always)]
    unsafe fn storeu_ps(a: &mut f32, b: Self::Vf32) {
        _mm256_storeu_ps(a as *mut f32, b.0);
    }
    #[inline(always)]
    unsafe fn storeu_pd(a: &mut f64, b: Self::Vf64) {
        _mm256_storeu_pd(a as *mut f64, b.0);
    }
    #[inline(always)]
    unsafe fn max_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x8(_mm256_max_epi32(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn min_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x8(_mm256_min_epi32(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn max_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_max_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn min_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_min_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn max_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        F64x4(_mm256_max_pd(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn min_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        F64x4(_mm256_min_pd(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn mul_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_mul_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn mul_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        F64x4(_mm256_mul_pd(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn div_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_div_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn div_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        F64x4(_mm256_div_pd(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn mullo_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x8(_mm256_mullo_epi32(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn not_epi32(a: Self::Vi32) -> Self::Vi32 {
        I32x8(_mm256_xor_si256(a.0, _mm256_set1_epi32(-1)))
    }
    #[inline(always)]
    unsafe fn or_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x8(_mm256_or_si256(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn or_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        I64x4(_mm256_or_si256(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn or_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_or_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn or_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        F64x4(_mm256_or_pd(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn round_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_round_ps(
            a.0,
            _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC,
        ))
    }
    #[inline(always)]
    unsafe fn round_pd(a: Self::Vf64) -> Self::Vf64 {
        F64x4(_mm256_round_pd(
            a.0,
            _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC,
        ))
    }
    #[inline(always)]
    unsafe fn set1_epi32(a: i32) -> Self::Vi32 {
        I32x8(_mm256_set1_epi32(a))
    }
    #[inline(always)]
    unsafe fn set1_ps(a: f32) -> Self::Vf32 {
        F32x8(_mm256_set1_ps(a))
    }
    #[inline(always)]
    unsafe fn set1_pd(a: f64) -> Self::Vf64 {
        F64x4(_mm256_set1_pd(a))
    }
    #[inline(always)]
    unsafe fn setzero_pd() -> Self::Vf64 {
        F64x4(_mm256_setzero_pd())
    }
    #[inline(always)]
    unsafe fn setzero_ps() -> Self::Vf32 {
        F32x8(_mm256_setzero_ps())
    }
    #[inline(always)]
    unsafe fn setzero_epi32() -> Self::Vi32 {
        I32x8(_mm256_setzero_si256())
    }
    #[inline(always)]
    unsafe fn setzero_epi64() -> Self::Vi64 {
        I64x4(_mm256_setzero_si256())
    }
    #[inline(always)]
    unsafe fn srai_epi32(a: Self::Vi32, b: i32) -> Self::Vi32 {
        I32x8(_mm256_srai_epi32(a.0, b))
    }
    unsafe fn srli_epi32(a: Self::Vi32, imm8: i32) -> Self::Vi32 {
        I32x8(_mm256_srli_epi32(a.0, imm8))
    }
    unsafe fn slli_epi32(a: Self::Vi32, imm8: i32) -> Self::Vi32 {
        I32x8(_mm256_slli_epi32(a.0, imm8))
    }

    unsafe fn sra_epi32(a: Self::Vi32, count: __m128i) -> Self::Vi32 {
        I32x8(_mm256_sra_epi32(a.0, count))
    }
    unsafe fn srl_epi32(a: Self::Vi32, count: __m128i) -> Self::Vi32 {
        I32x8(_mm256_srl_epi32(a.0, count))
    }
    unsafe fn sll_epi32(a: Self::Vi32, count: __m128i) -> Self::Vi32 {
        I32x8(_mm256_sll_epi32(a.0, count))
    }

    #[inline(always)]
    unsafe fn sub_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x8(_mm256_sub_epi32(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn sub_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_sub_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn sqrt_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_sqrt_ps(a.0))
    }
    #[inline(always)]
    unsafe fn rsqrt_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_rsqrt_ps(a.0))
    }
    #[inline(always)]
    unsafe fn sqrt_pd(a: Self::Vf64) -> Self::Vf64 {
        F64x4(_mm256_sqrt_pd(a.0))
    }
    #[inline(always)]
    unsafe fn rsqrt_pd(a: Self::Vf64) -> Self::Vf64 {
        F64x4(_mm256_div_pd(_mm256_set1_pd(1.0), _mm256_sqrt_pd(a.0)))
    }
    #[inline(always)]
    unsafe fn shuffle_epi32(a: Self::Vi32, imm8: i32) -> I32x8 {
        macro_rules! call {
            ($imm8:expr) => {
                I32x8(_mm256_shuffle_epi32(a.0, $imm8))
            };
        }
        constify_imm8!(imm8, call)
    }
    #[inline(always)]
    unsafe fn shuffle_ps(a: Self::Vf32, b: Self::Vf32, imm8: i32) -> Self::Vf32 {
        macro_rules! call {
            ($imm8:expr) => {
                F32x8(_mm256_shuffle_ps(a.0, b.0, $imm8))
            };
        }
        constify_imm8!(imm8, call)
    }
    #[inline(always)]
    unsafe fn xor_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x8(_mm256_xor_si256(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn xor_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        I64x4(_mm256_xor_si256(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn xor_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_xor_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn xor_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        F64x4(_mm256_xor_pd(a.0, b.0))
    }
}
