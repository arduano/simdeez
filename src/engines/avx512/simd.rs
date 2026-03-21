#![allow(deprecated)]

use super::*;
use crate::Simd;

pub struct Avx512;
impl Simd for Avx512 {
    type Vi8 = I8x64;
    type Vi16 = I16x32;
    type Vi32 = I32x16;
    type Vf32 = F32x16;
    type Vf64 = F64x8;
    type Vi64 = I64x8;

    #[inline]
    fn invoke<R>(f: impl FnOnce() -> R) -> R {
        #[inline]
        #[target_feature(enable = "avx512f", enable = "avx512bw", enable = "avx512dq")]
        unsafe fn inner<R>(f: impl FnOnce() -> R) -> R {
            f()
        }

        unsafe { inner(f) }
    }

    #[inline(always)]
    unsafe fn castps_pd(a: Self::Vf32) -> Self::Vf64 {
        F64x8(_mm512_castps_pd(a.0))
    }

    #[inline(always)]
    unsafe fn castpd_ps(a: Self::Vf64) -> Self::Vf32 {
        F32x16(_mm512_castpd_ps(a.0))
    }

    #[inline(always)]
    unsafe fn i32gather_epi32(arr: &[i32], index: Self::Vi32) -> Self::Vi32 {
        I32x16(_mm512_i32gather_epi32::<4>(
            index.0,
            arr.as_ptr() as *const i32,
        ))
    }

    #[inline(always)]
    unsafe fn i64gather_epi64(arr: &[i64], index: Self::Vi64) -> Self::Vi64 {
        I64x8(_mm512_i64gather_epi64::<8>(
            index.0,
            arr.as_ptr() as *const i64,
        ))
    }

    #[inline(always)]
    unsafe fn i32gather_ps(arr: &[f32], index: Self::Vi32) -> Self::Vf32 {
        F32x16(_mm512_i32gather_ps::<4>(
            index.0,
            arr.as_ptr() as *const f32,
        ))
    }

    #[inline(always)]
    unsafe fn maskload_epi32(mem_addr: &i32, mask: Self::Vi32) -> Self::Vi32 {
        let kmask = _mm512_cmpneq_epi32_mask(mask.0, _mm512_setzero_si512());
        I32x16(_mm512_maskz_loadu_epi32(kmask, mem_addr as *const i32))
    }

    #[inline(always)]
    unsafe fn maskload_epi64(mem_addr: &i64, mask: Self::Vi64) -> Self::Vi64 {
        let kmask = _mm512_cmpneq_epi64_mask(mask.0, _mm512_setzero_si512());
        I64x8(_mm512_maskz_loadu_epi64(kmask, mem_addr as *const i64))
    }

    #[inline(always)]
    unsafe fn maskload_ps(mem_addr: &f32, mask: Self::Vi32) -> Self::Vf32 {
        let kmask = _mm512_cmpneq_epi32_mask(mask.0, _mm512_setzero_si512());
        F32x16(_mm512_maskz_loadu_ps(kmask, mem_addr as *const f32))
    }

    #[inline(always)]
    unsafe fn maskload_pd(mem_addr: &f64, mask: Self::Vi64) -> Self::Vf64 {
        let kmask = _mm512_cmpneq_epi64_mask(mask.0, _mm512_setzero_si512());
        F64x8(_mm512_maskz_loadu_pd(kmask, mem_addr as *const f64))
    }

    #[inline(always)]
    unsafe fn shuffle_epi32<const IMM8: i32>(a: Self::Vi32) -> Self::Vi32 {
        I32x16(_mm512_shuffle_epi32::<IMM8>(a.0))
    }
}
