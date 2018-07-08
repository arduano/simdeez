use super::*;
use core::mem;

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
    unsafe fn and_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x4_41(_mm_and_si128(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn and_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        I64x2_41(_mm_and_si128(a.0, b.0))
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
    unsafe fn andnot_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x4_41(_mm_andnot_si128(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn andnot_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        I64x2_41(_mm_andnot_si128(a.0, b.0))
    }
    // TODO is this equivalent?
    #[inline(always)]
    unsafe fn blendv_epi32(a: Self::Vi32, b: Self::Vi32, mask: Self::Vi32) -> Self::Vi32 {
        I32x4_41(_mm_or_si128(
            _mm_andnot_si128(mask.0, a.0),
            _mm_and_si128(mask.0, b.0),
        ))
    }
    #[inline(always)]
    unsafe fn blendv_epi64(a: Self::Vi64, b: Self::Vi64, mask: Self::Vi64) -> Self::Vi64 {
        I64x2_41(_mm_or_si128(
            _mm_andnot_si128(mask.0, a.0),
            _mm_and_si128(mask.0, b.0),
        ))
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
    unsafe fn castps_epi32(a: Self::Vf32) -> Self::Vi32 {
        I32x4_41(_mm_castps_si128(a.0))
    }
    #[inline(always)]
    unsafe fn castpd_epi64(a: Self::Vf64) -> Self::Vi64 {
        I64x2_41(_mm_castpd_si128(a.0))
    }
    #[inline(always)]
    unsafe fn castepi32_ps(a: Self::Vi32) -> Self::Vf32 {
        F32x4(_mm_castsi128_ps(a.0))
    }
    #[inline(always)]
    unsafe fn castepi64_pd(a: Self::Vi64) -> Self::Vf64 {
        F64x2(_mm_castsi128_pd(a.0))
    }
    #[inline(always)]
    unsafe fn castepi32_epi64(a: Self::Vi32) -> Self::Vi64 {
        I64x2_41(a.0)
    }
    #[inline(always)]
    unsafe fn castepi64_epi32(a: Self::Vi64) -> Self::Vi32 {
        I32x4_41(a.0)
    }
    #[inline(always)]
    unsafe fn castps_pd(a: Self::Vf32) -> Self::Vf64 {
        F64x2(_mm_castps_pd(a.0))
    }
    #[inline(always)]
    unsafe fn castpd_ps(a: Self::Vf64) -> Self::Vf32 {
        F32x4(_mm_castpd_ps(a.0))
    }
    #[inline(always)]
    unsafe fn cmpeq_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x4_41(_mm_cmpeq_epi32(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn cmpneq_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        Self::not_epi32(I32x4_41(_mm_cmpeq_epi32(a.0, b.0)))
    }
    #[inline(always)]
    unsafe fn cmpge_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        Self::not_epi32(I32x4_41(_mm_cmpgt_epi32(b.0, a.0)))
    }
    #[inline(always)]
    unsafe fn cmpgt_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x4_41(_mm_cmpgt_epi32(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn cmple_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        Self::not_epi32(I32x4_41(_mm_cmplt_epi32(b.0, a.0)))
    }
    #[inline(always)]
    unsafe fn cmplt_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x4_41(_mm_cmplt_epi32(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn cmpeq_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        I64x2_41(_mm_cmpeq_epi64(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn cmpneq_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        Self::not_epi64(I64x2_41(_mm_cmpeq_epi64(a.0, b.0)))
    }
    #[inline(always)]
    unsafe fn cmpge_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        Self::not_epi64(I64x2_41(_mm_cmpgt_epi64(b.0, a.0)))
    }
    #[inline(always)]
    unsafe fn cmpgt_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        I64x2_41(_mm_cmpgt_epi64(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn cmple_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        Self::not_epi64(I64x2_41(_mm_cmpgt_epi64(a.0, b.0)))
    }
    #[inline(always)]
    unsafe fn cmplt_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        I64x2_41(_mm_cmpgt_epi64(b.0, a.0))
    }
    #[inline(always)]
    unsafe fn cmpeq_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_cmpeq_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn cmpneq_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_cmpneq_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn cmpge_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_cmpge_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn cmpgt_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_cmpgt_ps(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn cmple_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_cmple_ps(a.0, b.0))
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
    unsafe fn load_pd(a: &f64) -> Self::Vf64 {
        F64x2(_mm_load_pd(a as *const f64))
    }
    #[inline(always)]
    unsafe fn load_ps(a: &f32) -> Self::Vf32 {
        F32x4(_mm_load_ps(a as *const f32))
    }
    #[inline(always)]
    unsafe fn load_epi32(a: &i32) -> Self::Vi32 {
        let m = mem::transmute::<&i32, &__m128i>(a);
        I32x4_41(_mm_load_si128(m))
    }
    #[inline(always)]
    unsafe fn load_epi64(a: &i64) -> Self::Vi64 {
        let m = mem::transmute::<&i64, &__m128i>(a);
        I64x2_41(_mm_load_si128(m))
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
    unsafe fn loadu_epi32(a: &i32) -> Self::Vi32 {
        let m = mem::transmute::<&i32, &__m128i>(a);
        I32x4_41(_mm_loadu_si128(m))
    }
    #[inline(always)]
    unsafe fn loadu_epi64(a: &i64) -> Self::Vi64 {
        let m = mem::transmute::<&i64, &__m128i>(a);
        I64x2_41(_mm_loadu_si128(m))
    }
    #[inline(always)]
    unsafe fn maskload_epi32(mem_addr: &i32, mask: Self::Vi32) -> Self::Vi32 {
        let high_mask = Self::cmplt_epi32(mask, Self::setzero_epi32());
        let data = Self::loadu_epi32(mem_addr).0;
        I32x4_41(_mm_and_si128(data, high_mask.0))
    }
    #[inline(always)]
    unsafe fn maskload_epi64(mem_addr: &i64, mask: Self::Vi64) -> Self::Vi64 {
        let high_mask = Self::cmplt_epi64(mask, Self::setzero_epi64());
        let data = Self::loadu_epi64(mem_addr).0;
        I64x2_41(_mm_and_si128(data, high_mask.0))
    }
    #[inline(always)]
    unsafe fn maskload_ps(mem_addr: &f32, mask: Self::Vi32) -> Self::Vf32 {
        let data = Self::loadu_ps(mem_addr).0;
        F32x4(_mm_and_ps(data, _mm_castsi128_ps(mask.0)))
    }
    #[inline(always)]
    unsafe fn maskload_pd(mem_addr: &f64, mask: Self::Vi64) -> Self::Vf64 {
        let data = Self::loadu_pd(mem_addr).0;
        F64x2(_mm_and_pd(data, _mm_castsi128_pd(mask.0)))
    }
    #[inline(always)]
    unsafe fn store_ps(mem_addr: &mut f32, a: Self::Vf32) {
        _mm_store_ps(mem_addr as *mut f32, a.0);
    }
    #[inline(always)]
    unsafe fn store_pd(mem_addr: &mut f64, a: Self::Vf64) {
        _mm_store_pd(mem_addr as *mut f64, a.0);
    }
    #[inline(always)]
    unsafe fn store_epi32(mem_addr: &mut i32, a: Self::Vi32) {
        let mem_addr_128 = mem::transmute::<&mut i32, &mut __m128i>(mem_addr);
        _mm_store_si128(mem_addr_128, a.0);
    }
    #[inline(always)]
    unsafe fn store_epi64(mem_addr: &mut i64, a: Self::Vi64) {
        let mem_addr_128 = mem::transmute::<&mut i64, &mut __m128i>(mem_addr);
        _mm_store_si128(mem_addr_128, a.0);
    }
    #[inline(always)]
    unsafe fn storeu_ps(mem_addr: &mut f32, a: Self::Vf32) {
        _mm_storeu_ps(mem_addr as *mut f32, a.0);
    }
    #[inline(always)]
    unsafe fn storeu_pd(mem_addr: &mut f64, a: Self::Vf64) {
        _mm_storeu_pd(mem_addr as *mut f64, a.0);
    }
    #[inline(always)]
    unsafe fn storeu_epi32(mem_addr: &mut i32, a: Self::Vi32) {
        let mem_addr_128 = mem::transmute::<&mut i32, &mut __m128i>(mem_addr);
        _mm_storeu_si128(mem_addr_128, a.0);
    }
    #[inline(always)]
    unsafe fn storeu_epi64(mem_addr: &mut i64, a: Self::Vi64) {
        let mem_addr_128 = mem::transmute::<&mut i64, &mut __m128i>(mem_addr);
        _mm_storeu_si128(mem_addr_128, a.0);
    }
    #[inline(always)]
    unsafe fn maskstore_epi32(mem_addr: &mut i32, mask: Self::Vi32, a: Self::Vi32) {
        let data = Self::loadu_epi32(mem_addr);
        let result = Self::blendv_epi32(a, data, mask);
        Self::storeu_epi32(mem_addr, result);
    }
    #[inline(always)]
    unsafe fn maskstore_epi64(mem_addr: &mut i64, mask: Self::Vi64, a: Self::Vi64) {
        let data = Self::loadu_epi64(mem_addr);
        let result = Self::blendv_epi64(a, data, mask);
        Self::storeu_epi64(mem_addr, result);
    }
    #[inline(always)]
    unsafe fn maskstore_ps(mem_addr: &mut f32, mask: Self::Vi32, a: Self::Vf32) {
        let data = Self::loadu_ps(mem_addr);
        let result = Self::blendv_ps(a, data, Self::castepi32_ps(mask));
        Self::storeu_ps(mem_addr, result);
    }
    #[inline(always)]
    unsafe fn maskstore_pd(mem_addr: &mut f64, mask: Self::Vi64, a: Self::Vf64) {
        let data = Self::loadu_pd(mem_addr);
        let result = Self::blendv_pd(a, data, Self::castepi64_pd(mask));
        Self::storeu_pd(mem_addr, result);
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
    unsafe fn not_epi32(a: Self::Vi32) -> Self::Vi32 {
        I32x4_41(_mm_xor_si128(a.0, _mm_set1_epi32(-1)))
    }
    #[inline(always)]
    unsafe fn not_epi64(a: Self::Vi64) -> Self::Vi64 {
        I64x2_41(_mm_xor_si128(a.0, Self::set1_epi64(-1).0))
    }
    #[inline(always)]
    unsafe fn or_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x4_41(_mm_or_si128(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn or_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        I64x2_41(_mm_or_si128(a.0, b.0))
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
    unsafe fn rcp_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x4(_mm_rcp_ps(a.0))
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
    unsafe fn set1_epi64(a: i64) -> Self::Vi64 {
        let x = _mm_cvtsi64_si128(a);
        I64x2_41(_mm_unpacklo_epi64(x, x))
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
    unsafe fn setzero_epi32() -> Self::Vi32 {
        I32x4_41(_mm_setzero_si128())
    }
    #[inline(always)]
    unsafe fn setzero_epi64() -> Self::Vi64 {
        I64x2_41(_mm_setzero_si128())
    }
    #[inline(always)]
    unsafe fn srai_epi32(a: Self::Vi32, amt_const: i32) -> Self::Vi32 {
        macro_rules! call {
            ($amt_const:expr) => {
                I32x4_41(_mm_srai_epi32(a.0, $amt_const))
            };
        }
        constify_imm8!(amt_const, call)
    }
    unsafe fn srli_epi32(a: Self::Vi32, amt_const: i32) -> Self::Vi32 {
        macro_rules! call {
            ($amt_const:expr) => {
                I32x4_41(_mm_srli_epi32(a.0, $amt_const))
            };
        }
        constify_imm8!(amt_const, call)
    }
    unsafe fn slli_epi32(a: Self::Vi32, amt_const: i32) -> Self::Vi32 {
        macro_rules! call {
            ($amt_const:expr) => {
                I32x4_41(_mm_slli_epi32(a.0, $amt_const))
            };
        }
        constify_imm8!(amt_const, call)
    }

    #[inline(always)]
    unsafe fn sra_epi32(a: Self::Vi32, amt: i32) -> Self::Vi32 {
        I32x4_41(_mm_sra_epi32(a.0, _mm_set1_epi32(amt)))
    }
    #[inline(always)]
    unsafe fn srl_epi32(a: Self::Vi32, amt: i32) -> Self::Vi32 {
        I32x4_41(_mm_srl_epi32(a.0, _mm_set1_epi32(amt)))
    }
    #[inline(always)]
    unsafe fn sll_epi32(a: Self::Vi32, amt: i32) -> Self::Vi32 {
        I32x4_41(_mm_sll_epi32(a.0, _mm_set1_epi32(amt)))
    }

    #[inline(always)]
    unsafe fn sub_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x4_41(_mm_sub_epi32(a.0, b.0))
    }
    #[inline(always)]
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
    unsafe fn shuffle_epi32(a: Self::Vi32, imm8: i32) -> Self::Vi32 {
        macro_rules! call {
            ($imm8:expr) => {
                I32x4_41(_mm_shuffle_epi32(a.0, $imm8))
            };
        }
        constify_imm8!(imm8, call)
    }
    #[inline(always)]
    unsafe fn shuffle_ps(a: Self::Vf32, b: Self::Vf32, imm8: i32) -> Self::Vf32 {
        macro_rules! call {
            ($imm8:expr) => {
                F32x4(_mm_shuffle_ps(a.0, b.0, $imm8))
            };
        }
        constify_imm8!(imm8, call)
    }
    #[inline(always)]
    unsafe fn unpackhi_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x4_41(_mm_unpackhi_epi32(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn unpacklo_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x4_41(_mm_unpacklo_epi32(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn unpackhi_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        I64x2_41(_mm_unpackhi_epi64(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn unpacklo_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        I64x2_41(_mm_unpacklo_epi64(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn unpackhi_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        F64x2(_mm_unpackhi_pd(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn unpacklo_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        F64x2(_mm_unpacklo_pd(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn xor_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x4_41(_mm_xor_si128(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn xor_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        I64x2_41(_mm_xor_si128(a.0, b.0))
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
