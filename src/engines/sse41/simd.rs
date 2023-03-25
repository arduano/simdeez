use crate::Simd;

use super::*;
use core::mem;

pub struct Sse41;
impl Simd for Sse41 {
    type Vi8 = I8x16_41;
    type Vi16 = I16x8_41;
    type Vi32 = I32x4_41;
    type Vf32 = F32x4_41;
    type Vf64 = F64x2_41;
    type Vi64 = I64x2_41;

    #[inline]
    fn invoke<R>(f: impl FnOnce() -> R) -> R {
        #[inline]
        #[target_feature(enable = "sse4.1")]
        unsafe fn inner<R>(f: impl FnOnce() -> R) -> R {
            f()
        }

        unsafe { inner(f) }
    }

    #[inline(always)]
    unsafe fn castps_pd(a: Self::Vf32) -> Self::Vf64 {
        F64x2_41(_mm_castps_pd(a.0))
    }

    #[inline(always)]
    unsafe fn castpd_ps(a: Self::Vf64) -> Self::Vf32 {
        F32x4_41(_mm_castpd_ps(a.0))
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
    unsafe fn i64gather_epi64(arr: &[i64], index: Self::Vi64) -> Self::Vi64 {
        let index_as_arr = mem::transmute::<I64x2_41, [i64; 2]>(index);
        I64x2_41(_mm_set_epi64x(
            arr[index_as_arr[1] as usize],
            arr[index_as_arr[0] as usize],
        ))
    }

    #[inline(always)]
    unsafe fn i32gather_ps(arr: &[f32], index: Self::Vi32) -> Self::Vf32 {
        let index_as_arr = mem::transmute::<I32x4_41, [i32; 4]>(index);
        F32x4_41(_mm_set_ps(
            arr[index_as_arr[3] as usize],
            arr[index_as_arr[2] as usize],
            arr[index_as_arr[1] as usize],
            arr[index_as_arr[0] as usize],
        ))
    }

    #[inline(always)]
    unsafe fn maskload_epi32(mem_addr: &i32, mask: Self::Vi32) -> Self::Vi32 {
        let mut result = I32x4_41(_mm_setzero_si128());
        let ptr = mem_addr as *const i32;
        result[0] = if mask[0] != 0 { *ptr } else { 0 };
        result[1] = if mask[1] != 0 { *ptr.offset(1) } else { 0 };
        result[2] = if mask[2] != 0 { *ptr.offset(2) } else { 0 };
        result[3] = if mask[3] != 0 { *ptr.offset(3) } else { 0 };
        result
    }

    #[inline(always)]
    unsafe fn maskload_epi64(mem_addr: &i64, mask: Self::Vi64) -> Self::Vi64 {
        let mut result = I64x2_41(_mm_setzero_si128());
        let ptr = mem_addr as *const i64;
        result[0] = if mask[0] != 0 { *ptr } else { 0 };
        result[1] = if mask[1] != 0 { *ptr.offset(1) } else { 0 };
        result
    }

    #[inline(always)]
    unsafe fn maskload_ps(mem_addr: &f32, mask: Self::Vi32) -> Self::Vf32 {
        let mut result = F32x4_41(_mm_setzero_ps());
        let ptr = mem_addr as *const f32;
        result[0] = if mask[0] != 0 { *ptr } else { 0.0 };
        result[1] = if mask[1] != 0 { *ptr.offset(1) } else { 0.0 };
        result[2] = if mask[2] != 0 { *ptr.offset(2) } else { 0.0 };
        result[3] = if mask[3] != 0 { *ptr.offset(3) } else { 0.0 };
        result
    }

    #[inline(always)]
    unsafe fn maskload_pd(mem_addr: &f64, mask: Self::Vi64) -> Self::Vf64 {
        let mut result = F64x2_41(_mm_setzero_pd());
        let ptr = mem_addr as *const f64;
        result[0] = if mask[0] != 0 { *ptr } else { 0.0 };
        result[1] = if mask[1] != 0 { *ptr.offset(1) } else { 0.0 };
        result
    }

    #[inline(always)]
    unsafe fn shuffle_epi32<const IMM8: i32>(a: Self::Vi32) -> Self::Vi32 {
        I32x4_41(_mm_shuffle_epi32(a.0, IMM8))
    }
}
