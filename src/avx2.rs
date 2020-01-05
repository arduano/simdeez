use super::*;
use crate::overloads::*;
use core::mem;

pub struct Avx2;
impl Simd for Avx2 {
    type Vi16 = I16x16;
    type Vi32 = I32x8;
    type Vf32 = F32x8;
    type Vf64 = F64x4;
    type Vi64 = I64x4;

    const VF32_WIDTH: usize = 8;
    const VF64_WIDTH: usize = 4;
    const VI16_WIDTH: usize = 16;
    const VI32_WIDTH: usize = 8;
    const VI64_WIDTH: usize = 4;

    #[inline(always)]
    unsafe fn abs_ps(a: Self::Vf32) -> Self::Vf32 {
        let b = _mm256_set1_ps(-0.0f32);
        F32x8(_mm256_andnot_ps(b, a.0))
    }
    #[inline(always)]
    unsafe fn abs_pd(a: Self::Vf64) -> Self::Vf64 {
        let b = _mm256_set1_pd(-0.0f64);
        F64x4(_mm256_andnot_pd(b, a.0))
    }
    #[inline(always)]
    unsafe fn add_epi16(a: Self::Vi16, b: Self::Vi16) -> Self::Vi16 {
        I16x16(_mm256_add_epi16(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn sub_epi16(a: Self::Vi16, b: Self::Vi16) -> Self::Vi16 {
        I16x16(_mm256_sub_epi16(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn mullo_epi16(a: Self::Vi16, b: Self::Vi16) -> Self::Vi16 {
        I16x16(_mm256_mullo_epi16(a.0, b.0))
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
    unsafe fn blendv_epi64(a: Self::Vi64, b: Self::Vi64, mask: Self::Vi64) -> Self::Vi64 {
        I64x4(_mm256_castpd_si256(_mm256_blendv_pd(
            _mm256_castsi256_pd(a.0),
            _mm256_castsi256_pd(b.0),
            _mm256_castsi256_pd(mask.0),
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
    unsafe fn castps_pd(a: Self::Vf32) -> Self::Vf64 {
        F64x4(_mm256_castps_pd(a.0))
    }
    #[inline(always)]
    unsafe fn castpd_ps(a: Self::Vf64) -> Self::Vf32 {
        F32x8(_mm256_castpd_ps(a.0))
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
    unsafe fn cmpeq_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        I64x4(_mm256_cmpeq_epi64(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn cmpneq_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        Self::not_epi64(I64x4(_mm256_cmpeq_epi64(a.0, b.0)))
    }
    #[inline(always)]
    unsafe fn cmpge_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        Self::not_epi64(I64x4(_mm256_cmpgt_epi64(b.0, a.0)))
    }
    #[inline(always)]
    unsafe fn cmpgt_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        I64x4(_mm256_cmpgt_epi64(a.0, b.0))
    }
    #[inline(always)]
    unsafe fn cmple_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        Self::not_epi64(I64x4(_mm256_cmpgt_epi64(a.0, b.0)))
    }
    #[inline(always)]
    unsafe fn cmplt_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        I64x4(_mm256_cmpgt_epi64(b.0, a.0))
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
    unsafe fn cmpeq_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        F64x4(_mm256_cmp_pd(a.0, b.0, _CMP_EQ_OQ))
    }
    #[inline(always)]
    unsafe fn cmpneq_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        F64x4(_mm256_cmp_pd(a.0, b.0, _CMP_NEQ_OQ))
    }
    #[inline(always)]
    unsafe fn cmpge_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        F64x4(_mm256_cmp_pd(a.0, b.0, _CMP_GE_OQ))
    }
    #[inline(always)]
    unsafe fn cmpgt_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        F64x4(_mm256_cmp_pd(a.0, b.0, _CMP_GT_OQ))
    }
    #[inline(always)]
    unsafe fn cmple_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        F64x4(_mm256_cmp_pd(a.0, b.0, _CMP_LE_OQ))
    }
    #[inline(always)]
    unsafe fn cmplt_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        F64x4(_mm256_cmp_pd(a.0, b.0, _CMP_LT_OQ))
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
    unsafe fn fast_ceil_ps(a: Self::Vf32) -> Self::Vf32 {
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
    unsafe fn fast_floor_ps(a: Self::Vf32) -> Self::Vf32 {
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
    #[inline(always)]
    unsafe fn fmadd_pd(a: Self::Vf64, b: Self::Vf64, c: Self::Vf64) -> Self::Vf64 {
        F64x4(_mm256_fmadd_pd(a.0, b.0, c.0))
    }
    #[inline(always)]
    unsafe fn fnmadd_pd(a: Self::Vf64, b: Self::Vf64, c: Self::Vf64) -> Self::Vf64 {
        F64x4(_mm256_fnmadd_pd(a.0, b.0, c.0))
    }
    #[inline(always)]
    unsafe fn fmsub_ps(a: Self::Vf32, b: Self::Vf32, c: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_fmsub_ps(a.0, b.0, c.0))
    }
    #[inline(always)]
    unsafe fn fnmsub_ps(a: Self::Vf32, b: Self::Vf32, c: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_fnmsub_ps(a.0, b.0, c.0))
    }
    #[inline(always)]
    unsafe fn fmsub_pd(a: Self::Vf64, b: Self::Vf64, c: Self::Vf64) -> Self::Vf64 {
        F64x4(_mm256_fmsub_pd(a.0, b.0, c.0))
    }
    #[inline(always)]
    unsafe fn fnmsub_pd(a: Self::Vf64, b: Self::Vf64, c: Self::Vf64) -> Self::Vf64 {
        F64x4(_mm256_fnmsub_pd(a.0, b.0, c.0))
    }
    #[inline(always)]
    unsafe fn horizontal_add_ps(a: Self::Vf32) -> f32 {
        let mut vlow = _mm256_castps256_ps128(a.0);
        let vhigh = _mm256_extractf128_ps(a.0, 1);
        vlow = _mm_add_ps(vlow, vhigh);
        let mut shuf = _mm_movehdup_ps(vlow);
        let mut sums = _mm_add_ps(vlow, shuf);
        shuf = _mm_movehl_ps(shuf, sums);
        sums = _mm_add_ss(sums, shuf);
        _mm_cvtss_f32(sums)
    }
    #[inline(always)]
    unsafe fn horizontal_add_pd(a: Self::Vf64) -> f64 {
        let mut vlow = _mm256_castpd256_pd128(a.0);
        let vhigh = _mm256_extractf128_pd(a.0, 1);
        vlow = _mm_add_pd(vlow, vhigh);
        let high64 = _mm_unpackhi_pd(vlow, vlow);
        _mm_cvtsd_f64(_mm_add_sd(vlow, high64))
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
    unsafe fn load_ps(a: &f32) -> Self::Vf32 {
        F32x8(_mm256_load_ps(a as *const f32))
    }
    #[inline(always)]
    unsafe fn load_pd(a: &f64) -> Self::Vf64 {
        F64x4(_mm256_load_pd(a as *const f64))
    }
    #[inline(always)]
    unsafe fn load_epi32(a: &i32) -> Self::Vi32 {
        let m = mem::transmute::<&i32, &__m256i>(a);
        I32x8(_mm256_load_si256(m))
    }
    #[inline(always)]
    unsafe fn load_epi64(a: &i64) -> Self::Vi64 {
        let m = mem::transmute::<&i64, &__m256i>(a);
        I64x4(_mm256_load_si256(m))
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
    unsafe fn loadu_epi32(a: &i32) -> Self::Vi32 {
        let m = mem::transmute::<&i32, &__m256i>(a);
        I32x8(_mm256_loadu_si256(m))
    }
    #[inline(always)]
    unsafe fn loadu_epi64(a: &i64) -> Self::Vi64 {
        let m = mem::transmute::<&i64, &__m256i>(a);
        I64x4(_mm256_loadu_si256(m))
    }
    #[inline(always)]
    unsafe fn maskload_epi32(mem_addr: &i32, mask: Self::Vi32) -> Self::Vi32 {
        I32x8(_mm256_maskload_epi32(mem_addr as *const i32, mask.0))
    }
    #[inline(always)]
    unsafe fn maskload_epi64(mem_addr: &i64, mask: Self::Vi64) -> Self::Vi64 {
        I64x4(_mm256_maskload_epi64(mem_addr as *const i64, mask.0))
    }
    #[inline(always)]
    unsafe fn maskload_ps(mem_addr: &f32, mask: Self::Vi32) -> Self::Vf32 {
        F32x8(_mm256_maskload_ps(mem_addr as *const f32, mask.0))
    }
    #[inline(always)]
    unsafe fn maskload_pd(mem_addr: &f64, mask: Self::Vi64) -> Self::Vf64 {
        F64x4(_mm256_maskload_pd(mem_addr as *const f64, mask.0))
    }
    #[inline(always)]
    unsafe fn store_ps(mem_addr: &mut f32, a: Self::Vf32) {
        _mm256_store_ps(mem_addr as *mut f32, a.0);
    }
    #[inline(always)]
    unsafe fn store_pd(mem_addr: &mut f64, a: Self::Vf64) {
        _mm256_store_pd(mem_addr as *mut f64, a.0);
    }
    #[inline(always)]
    unsafe fn store_epi32(mem_addr: &mut i32, a: Self::Vi32) {
        let mem_addr_256 = mem::transmute::<&mut i32, &mut __m256i>(mem_addr);
        _mm256_store_si256(mem_addr_256, a.0);
    }
    #[inline(always)]
    unsafe fn store_epi64(mem_addr: &mut i64, a: Self::Vi64) {
        let mem_addr_256 = mem::transmute::<&mut i64, &mut __m256i>(mem_addr);
        _mm256_store_si256(mem_addr_256, a.0);
    }
    #[inline(always)]
    unsafe fn storeu_ps(mem_addr: &mut f32, a: Self::Vf32) {
        _mm256_storeu_ps(mem_addr as *mut f32, a.0);
    }
    #[inline(always)]
    unsafe fn storeu_pd(mem_addr: &mut f64, a: Self::Vf64) {
        _mm256_storeu_pd(mem_addr as *mut f64, a.0);
    }
    #[inline(always)]
    unsafe fn storeu_epi32(mem_addr: &mut i32, a: Self::Vi32) {
        let mem_addr_256 = mem::transmute::<&mut i32, &mut __m256i>(mem_addr);
        _mm256_storeu_si256(mem_addr_256, a.0);
    }
    #[inline(always)]
    unsafe fn storeu_epi64(mem_addr: &mut i64, a: Self::Vi64) {
        let mem_addr_256 = mem::transmute::<&mut i64, &mut __m256i>(mem_addr);
        _mm256_storeu_si256(mem_addr_256, a.0);
    }
    #[inline(always)]
    unsafe fn maskstore_epi32(mem_addr: &mut i32, mask: Self::Vi32, a: Self::Vi32) {
        _mm256_maskstore_epi32(mem_addr as *mut i32, mask.0, a.0)
    }
    #[inline(always)]
    unsafe fn maskstore_epi64(mem_addr: &mut i64, mask: Self::Vi64, a: Self::Vi64) {
        _mm256_maskstore_epi64(mem_addr as *mut i64, mask.0, a.0)
    }
    #[inline(always)]
    unsafe fn maskstore_ps(mem_addr: &mut f32, mask: Self::Vi32, a: Self::Vf32) {
        _mm256_maskstore_ps(mem_addr as *mut f32, mask.0, a.0)
    }
    #[inline(always)]
    unsafe fn maskstore_pd(mem_addr: &mut f64, mask: Self::Vi64, a: Self::Vf64) {
        _mm256_maskstore_pd(mem_addr as *mut f64, mask.0, a.0)
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
    unsafe fn not_epi64(a: Self::Vi64) -> Self::Vi64 {
        I64x4(_mm256_xor_si256(a.0, _mm256_set1_epi64x(-1)))
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
    unsafe fn rcp_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_rcp_ps(a.0))
    }
    #[inline(always)]
    unsafe fn round_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x8(_mm256_round_ps(
            a.0,
            _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC,
        ))
    }
    #[inline(always)]
    unsafe fn fast_round_ps(a: Self::Vf32) -> Self::Vf32 {
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
    unsafe fn set1_epi64(a: i64) -> Self::Vi64 {
        I64x4(_mm256_set1_epi64x(a))
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
    unsafe fn srai_epi32(a: Self::Vi32, amt_const: i32) -> Self::Vi32 {
        I32x8(_mm256_srai_epi32(a.0, amt_const))
    }
    #[inline(always)]
    unsafe fn srai_epi64(a: Self::Vi64, amt_const: i32) -> Self::Vi64 {
        // instruction does not exist. Split into 32-bit shifts
        if amt_const <= 32 {
            let bb = _mm_set_epi32(0, 0, 0, amt_const);
            let sra = _mm256_sra_epi32(a.0, bb); // a >> b signed dwords
            let srl = _mm256_srl_epi64(a.0, bb); // a >> b unsigned qwords
            let mask = _mm256_setr_epi32(0, -1, 0, -1, 0, -1, 0, -1); // mask for signed high part
            Self::blendv_epi64(I64x4(srl), I64x4(sra), I64x4(mask))
        } else {
            // b > 32
            let bm32 = _mm_set_epi32(0, 0, 0, amt_const - 32);
            let sign = _mm256_srai_epi32(a.0, 31); // sign of a
            let sra2 = _mm256_sra_epi32(a.0, bm32); // a >> (b-32) signed dwords
            let sra3 = _mm256_srli_epi64(sra2, 32); // a >> (b-32) >> 32 (second shift unsigned qword)
            let mask = _mm256_setr_epi32(0, -1, 0, -1, 0, -1, 0, -1); // mask for high part containing only sign
            Self::blendv_epi64(I64x4(sra3), I64x4(sign), I64x4(mask))
        }
    }
    #[inline(always)]
    unsafe fn srli_epi32(a: Self::Vi32, amt_const: i32) -> Self::Vi32 {
        I32x8(_mm256_srli_epi32(a.0, amt_const))
    }
    #[inline(always)]
    unsafe fn slli_epi32(a: Self::Vi32, amt_const: i32) -> Self::Vi32 {
        I32x8(_mm256_slli_epi32(a.0, amt_const))
    }

    #[inline(always)]
    unsafe fn sra_epi32(a: Self::Vi32, amt: i32) -> Self::Vi32 {
        I32x8(_mm256_sra_epi32(a.0, _mm_set1_epi32(amt)))
    }
    #[inline(always)]
    unsafe fn srl_epi32(a: Self::Vi32, amt: i32) -> Self::Vi32 {
        I32x8(_mm256_srl_epi32(a.0, _mm_set1_epi32(amt)))
    }
    #[inline(always)]
    unsafe fn sll_epi32(a: Self::Vi32, amt: i32) -> Self::Vi32 {
        I32x8(_mm256_sll_epi32(a.0, _mm_set1_epi32(amt)))
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
    unsafe fn sub_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        F64x4(_mm256_sub_pd(a.0, b.0))
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

    cfg_if::cfg_if! {
        if #[cfg(feature = "sleef")]
        {

            #[inline(always)]
            unsafe fn sin_ps(a: Self::Vf32) -> Self::Vf32 {
                let sin1 = F32x4(sleef_sys::Sleef_sinf4_u10sse4(_mm_loadu_ps(&a[0])));
                let sin2 = F32x4(sleef_sys::Sleef_sinf4_u10sse4(_mm_loadu_ps(&a[4])));
                F32x8(_mm256_loadu2_m128(&sin2[0], &sin1[0]))
            }

            #[inline(always)]
            unsafe fn cos_ps(a: Self::Vf32) -> Self::Vf32 {
                let cos1 = F32x4(sleef_sys::Sleef_cosf4_u10sse4(_mm_loadu_ps(&a[0])));
                let cos2 = F32x4(sleef_sys::Sleef_cosf4_u10sse4(_mm_loadu_ps(&a[4])));
                F32x8(_mm256_loadu2_m128(&cos2[0], &cos1[0]))
            }

            #[inline(always)]
            unsafe fn fast_sin_ps(a: Self::Vf32) -> Self::Vf32 {
                let sin1 = F32x4(sleef_sys::Sleef_sinf4_u35sse4(_mm_loadu_ps(&a[0])));
                let sin2 = F32x4(sleef_sys::Sleef_sinf4_u35sse4(_mm_loadu_ps(&a[4])));
                F32x8(_mm256_loadu2_m128(&sin2[0], &sin1[0]))
            }

            #[inline(always)]
            unsafe fn fast_cos_ps(a: Self::Vf32) -> Self::Vf32 {
                let cos1 = F32x4(sleef_sys::Sleef_cosf4_u35sse4(_mm_loadu_ps(&a[0])));
                let cos2 = F32x4(sleef_sys::Sleef_cosf4_u35sse4(_mm_loadu_ps(&a[4])));
                F32x8(_mm256_loadu2_m128(&cos2[0], &cos1[0]))
            }
            //
            #[inline(always)]
            unsafe fn asin_ps(a: Self::Vf32) -> Self::Vf32 {
                let sin1 = F32x4(sleef_sys::Sleef_asinf4_u10sse4(_mm_loadu_ps(&a[0])));
                let sin2 = F32x4(sleef_sys::Sleef_asinf4_u10sse4(_mm_loadu_ps(&a[4])));
                F32x8(_mm256_loadu2_m128(&sin2[0], &sin1[0]))
            }

            #[inline(always)]
            unsafe fn acos_ps(a: Self::Vf32) -> Self::Vf32 {
                let cos1 = F32x4(sleef_sys::Sleef_acosf4_u10sse4(_mm_loadu_ps(&a[0])));
                let cos2 = F32x4(sleef_sys::Sleef_acosf4_u10sse4(_mm_loadu_ps(&a[4])));
                F32x8(_mm256_loadu2_m128(&cos2[0], &cos1[0]))
            }

            #[inline(always)]
            unsafe fn fast_asin_ps(a: Self::Vf32) -> Self::Vf32 {
                let sin1 = F32x4(sleef_sys::Sleef_asinf4_u35sse4(_mm_loadu_ps(&a[0])));
                let sin2 = F32x4(sleef_sys::Sleef_asinf4_u35sse4(_mm_loadu_ps(&a[4])));
                F32x8(_mm256_loadu2_m128(&sin2[0], &sin1[0]))
            }

            #[inline(always)]
            unsafe fn fast_acos_ps(a: Self::Vf32) -> Self::Vf32 {
                let cos1 = F32x4(sleef_sys::Sleef_acosf4_u35sse4(_mm_loadu_ps(&a[0])));
                let cos2 = F32x4(sleef_sys::Sleef_acosf4_u35sse4(_mm_loadu_ps(&a[4])));
                F32x8(_mm256_loadu2_m128(&cos2[0], &cos1[0]))
            }
            #[inline(always)]
            unsafe fn tan_ps(a: Self::Vf32) -> Self::Vf32 {
                let tan1 = F32x4(sleef_sys::Sleef_tanf4_u10sse4(_mm_loadu_ps(&a[0])));
                let tan2 = F32x4(sleef_sys::Sleef_tanf4_u10sse4(_mm_loadu_ps(&a[4])));
                F32x8(_mm256_loadu2_m128(&tan2[0], &tan1[0]))
            }

            #[inline(always)]
            unsafe fn fast_tan_ps(a: Self::Vf32) -> Self::Vf32 {
                let tan1 = F32x4(sleef_sys::Sleef_tanf4_u35sse4(_mm_loadu_ps(&a[0])));
                let tan2 = F32x4(sleef_sys::Sleef_tanf4_u35sse4(_mm_loadu_ps(&a[4])));
                F32x8(_mm256_loadu2_m128(&tan2[0], &tan1[0]))
            }
            #[inline(always)]
            unsafe fn atan_ps(a: Self::Vf32) -> Self::Vf32 {
                let atan1 = F32x4(sleef_sys::Sleef_atanf4_u10sse4(_mm_loadu_ps(&a[0])));
                let atan2 = F32x4(sleef_sys::Sleef_atanf4_u10sse4(_mm_loadu_ps(&a[4])));
                F32x8(_mm256_loadu2_m128(&atan2[0], &atan1[0]))
            }

            #[inline(always)]
            unsafe fn fast_atan_ps(a: Self::Vf32) -> Self::Vf32 {
                let atan1 = F32x4(sleef_sys::Sleef_atanf4_u35sse4(_mm_loadu_ps(&a[0])));
                let atan2 = F32x4(sleef_sys::Sleef_atanf4_u35sse4(_mm_loadu_ps(&a[4])));
                F32x8(_mm256_loadu2_m128(&atan2[0], &atan1[0]))
            }
            //hyperbolic

            #[inline(always)]
            unsafe fn sinh_ps(a: Self::Vf32) -> Self::Vf32 {
                let sin1 = F32x4(sleef_sys::Sleef_sinhf4_u10sse4(_mm_loadu_ps(&a[0])));
                let sin2 = F32x4(sleef_sys::Sleef_sinhf4_u10sse4(_mm_loadu_ps(&a[4])));
                F32x8(_mm256_loadu2_m128(&sin2[0], &sin1[0]))
            }

            #[inline(always)]
            unsafe fn cosh_ps(a: Self::Vf32) -> Self::Vf32 {
                let cos1 = F32x4(sleef_sys::Sleef_coshf4_u10sse4(_mm_loadu_ps(&a[0])));
                let cos2 = F32x4(sleef_sys::Sleef_coshf4_u10sse4(_mm_loadu_ps(&a[4])));
                F32x8(_mm256_loadu2_m128(&cos2[0], &cos1[0]))
            }

            #[inline(always)]
            unsafe fn fast_sinh_ps(a: Self::Vf32) -> Self::Vf32 {
                let sin1 = F32x4(sleef_sys::Sleef_sinhf4_u35sse4(_mm_loadu_ps(&a[0])));
                let sin2 = F32x4(sleef_sys::Sleef_sinhf4_u35sse4(_mm_loadu_ps(&a[4])));
                F32x8(_mm256_loadu2_m128(&sin2[0], &sin1[0]))
            }

            #[inline(always)]
            unsafe fn fast_cosh_ps(a: Self::Vf32) -> Self::Vf32 {
                let cos1 = F32x4(sleef_sys::Sleef_coshf4_u35sse4(_mm_loadu_ps(&a[0])));
                let cos2 = F32x4(sleef_sys::Sleef_coshf4_u35sse4(_mm_loadu_ps(&a[4])));
                F32x8(_mm256_loadu2_m128(&cos2[0], &cos1[0]))
            }
            //
            #[inline(always)]
            unsafe fn asinh_ps(a: Self::Vf32) -> Self::Vf32 {
                let sin1 = F32x4(sleef_sys::Sleef_asinhf4_u10sse4(_mm_loadu_ps(&a[0])));
                let sin2 = F32x4(sleef_sys::Sleef_asinhf4_u10sse4(_mm_loadu_ps(&a[4])));
                F32x8(_mm256_loadu2_m128(&sin2[0], &sin1[0]))
            }

            #[inline(always)]
            unsafe fn acosh_ps(a: Self::Vf32) -> Self::Vf32 {
                let cos1 = F32x4(sleef_sys::Sleef_acoshf4_u10sse4(_mm_loadu_ps(&a[0])));
                let cos2 = F32x4(sleef_sys::Sleef_acoshf4_u10sse4(_mm_loadu_ps(&a[4])));
                F32x8(_mm256_loadu2_m128(&cos2[0], &cos1[0]))
            }


            #[inline(always)]
            unsafe fn tanh_ps(a: Self::Vf32) -> Self::Vf32 {
                let tan1 = F32x4(sleef_sys::Sleef_tanhf4_u10sse4(_mm_loadu_ps(&a[0])));
                let tan2 = F32x4(sleef_sys::Sleef_tanhf4_u10sse4(_mm_loadu_ps(&a[4])));
                F32x8(_mm256_loadu2_m128(&tan2[0], &tan1[0]))
            }

            #[inline(always)]
            unsafe fn fast_tanh_ps(a: Self::Vf32) -> Self::Vf32 {
                let tan1 = F32x4(sleef_sys::Sleef_tanhf4_u35sse4(_mm_loadu_ps(&a[0])));
                let tan2 = F32x4(sleef_sys::Sleef_tanhf4_u35sse4(_mm_loadu_ps(&a[4])));
                F32x8(_mm256_loadu2_m128(&tan2[0], &tan1[0]))
            }
            #[inline(always)]
            unsafe fn atanh_ps(a: Self::Vf32) -> Self::Vf32 {
                let atan1 = F32x4(sleef_sys::Sleef_atanhf4_u10sse4(_mm_loadu_ps(&a[0])));
                let atan2 = F32x4(sleef_sys::Sleef_atanhf4_u10sse4(_mm_loadu_ps(&a[4])));
                F32x8(_mm256_loadu2_m128(&atan2[0], &atan1[0]))
            }
            #[inline(always)]
            unsafe fn atan2_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
                let atan1 = F32x4(sleef_sys::Sleef_atan2f4_u10sse4(_mm_loadu_ps(&a[0]),_mm_loadu_ps(&b[0])));
                let atan2 = F32x4(sleef_sys::Sleef_atan2f4_u10sse4(_mm_loadu_ps(&a[4]),_mm_loadu_ps(&b[4])));
                F32x8(_mm256_loadu2_m128(&atan2[0], &atan1[0]))
            }

            #[inline(always)]
            unsafe fn fast_atan2_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
                let atan1 = F32x4(sleef_sys::Sleef_atan2f4_u35sse4(_mm_loadu_ps(&a[0]),_mm_loadu_ps(&b[0])));
                let atan2 = F32x4(sleef_sys::Sleef_atan2f4_u35sse4(_mm_loadu_ps(&a[4]),_mm_loadu_ps(&b[4])));
                F32x8(_mm256_loadu2_m128(&atan2[0], &atan1[0]))
            }
            #[inline(always)]
            unsafe fn ln_ps(a: Self::Vf32) -> Self::Vf32 {
                let log1 = F32x4(sleef_sys::Sleef_logf4_u10sse4(_mm_loadu_ps(&a[0])));
                let log2 = F32x4(sleef_sys::Sleef_logf4_u10sse4(_mm_loadu_ps(&a[4])));
                F32x8(_mm256_loadu2_m128(&log2[0], &log1[0]))
            }
            #[inline(always)]
            unsafe fn fast_ln_ps(a: Self::Vf32) -> Self::Vf32 {
                let log1 = F32x4(sleef_sys::Sleef_logf4_u35sse4(_mm_loadu_ps(&a[0])));
                let log2 = F32x4(sleef_sys::Sleef_logf4_u35sse4(_mm_loadu_ps(&a[4])));
                F32x8(_mm256_loadu2_m128(&log2[0], &log1[0]))
            }
            #[inline(always)]
            unsafe fn log2_ps(a: Self::Vf32) -> Self::Vf32 {
                let log1 = F32x4(sleef_sys::Sleef_log2f4_u10sse4(_mm_loadu_ps(&a[0])));
                let log2 = F32x4(sleef_sys::Sleef_log2f4_u10sse4(_mm_loadu_ps(&a[4])));
                F32x8(_mm256_loadu2_m128(&log2[0], &log1[0]))
            }
            #[inline(always)]
            unsafe fn log10_ps(a: Self::Vf32) -> Self::Vf32 {
                let log1 = F32x4(sleef_sys::Sleef_log10f4_u10sse4(_mm_loadu_ps(&a[0])));
                let log2 = F32x4(sleef_sys::Sleef_log10f4_u10sse4(_mm_loadu_ps(&a[4])));
                F32x8(_mm256_loadu2_m128(&log2[0], &log1[0]))
            }
            #[inline(always)]
            unsafe fn hypot_ps(a:Self::Vf32,b:Self::Vf32) -> Self::Vf32 {
                let hypot1 = F32x4(sleef_sys::Sleef_hypotf4_u05sse4(_mm_loadu_ps(&a[0]),_mm_loadu_ps(&b[0])));
                let hypot2 = F32x4(sleef_sys::Sleef_hypotf4_u05sse4(_mm_loadu_ps(&a[4]),_mm_loadu_ps(&b[4])));
                F32x8(_mm256_loadu2_m128(&hypot2[0], &hypot1[0]))
            }
            #[inline(always)]
            unsafe fn fast_hypot_ps(a:Self::Vf32,b:Self::Vf32) -> Self::Vf32 {
                let hypot1 = F32x4(sleef_sys::Sleef_hypotf4_u35sse4(_mm_loadu_ps(&a[0]),_mm_loadu_ps(&b[0])));
                let hypot2 = F32x4(sleef_sys::Sleef_hypotf4_u35sse4(_mm_loadu_ps(&a[4]),_mm_loadu_ps(&b[4])));
                F32x8(_mm256_loadu2_m128(&hypot2[0], &hypot1[0]))
            }
            #[inline(always)]
            unsafe fn fmod_ps(a:Self::Vf32,b:Self::Vf32) -> Self::Vf32 {
                let fmod1 = F32x4(sleef_sys::Sleef_fmodf4_sse4(_mm_loadu_ps(&a[0]),_mm_loadu_ps(&b[0])));
                let fmod2 = F32x4(sleef_sys::Sleef_fmodf4_sse4(_mm_loadu_ps(&a[4]),_mm_loadu_ps(&b[4])));
                F32x8(_mm256_loadu2_m128(&fmod2[0], &fmod1[0]))
            }
        }
    }
}
