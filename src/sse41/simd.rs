#![allow(deprecated)]
use super::*;
use core::mem;

pub struct Sse41;
impl Simd for Sse41 {
    type Vi16 = I16x8_41;
    type Vi32 = I32x4_41;
    type Vf32 = F32x4_41;
    type Vf64 = F64x2_41;
    type Vi64 = I64x2_41;

    unsafe fn mullo_epi16(a: Self::Vi16, b: Self::Vi16) -> Self::Vi16 {
        I16x8_41(_mm_mullo_epi16(a.0, b.0))
    }

    unsafe fn castps_pd(a: Self::Vf32) -> Self::Vf64 {
        F64x2_41(_mm_castps_pd(a.0))
    }

    unsafe fn castpd_ps(a: Self::Vf64) -> Self::Vf32 {
        F32x4_41(_mm_castpd_ps(a.0))
    }

    unsafe fn i32gather_epi32(arr: &[i32], index: Self::Vi32) -> Self::Vi32 {
        let index_as_arr = mem::transmute::<I32x4_41, [i32; 4]>(index);
        I32x4_41(_mm_set_epi32(
            arr[index_as_arr[3] as usize],
            arr[index_as_arr[2] as usize],
            arr[index_as_arr[1] as usize],
            arr[index_as_arr[0] as usize],
        ))
    }

    unsafe fn i64gather_epi64(arr: &[i64], index: Self::Vi64) -> Self::Vi64 {
        let index_as_arr = mem::transmute::<I64x2_41, [i64; 2]>(index);
        I64x2_41(_mm_set_epi64x(
            arr[index_as_arr[1] as usize],
            arr[index_as_arr[0] as usize],
        ))
    }

    unsafe fn i32gather_ps(arr: &[f32], index: Self::Vi32) -> Self::Vf32 {
        let index_as_arr = mem::transmute::<I32x4_41, [i32; 4]>(index);
        F32x4_41(_mm_set_ps(
            arr[index_as_arr[3] as usize],
            arr[index_as_arr[2] as usize],
            arr[index_as_arr[1] as usize],
            arr[index_as_arr[0] as usize],
        ))
    }

    unsafe fn maskload_epi32(mem_addr: &i32, mask: Self::Vi32) -> Self::Vi32 {
        let mut result = I32x4_41(_mm_setzero_si128());
        let ptr = mem_addr as *const i32;
        result[0] = if mask[0] != 0 { *ptr } else { 0 };
        result[1] = if mask[1] != 0 { *ptr.offset(1) } else { 0 };
        result[2] = if mask[2] != 0 { *ptr.offset(2) } else { 0 };
        result[3] = if mask[3] != 0 { *ptr.offset(3) } else { 0 };
        result
    }

    unsafe fn maskload_epi64(mem_addr: &i64, mask: Self::Vi64) -> Self::Vi64 {
        let mut result = I64x2_41(_mm_setzero_si128());
        let ptr = mem_addr as *const i64;
        result[0] = if mask[0] != 0 { *ptr } else { 0 };
        result[1] = if mask[1] != 0 { *ptr.offset(1) } else { 0 };
        result
    }

    unsafe fn maskload_ps(mem_addr: &f32, mask: Self::Vi32) -> Self::Vf32 {
        let mut result = F32x4_41(_mm_setzero_ps());
        let ptr = mem_addr as *const f32;
        result[0] = if mask[0] != 0 { *ptr } else { 0.0 };
        result[1] = if mask[1] != 0 { *ptr.offset(1) } else { 0.0 };
        result[2] = if mask[2] != 0 { *ptr.offset(2) } else { 0.0 };
        result[3] = if mask[3] != 0 { *ptr.offset(3) } else { 0.0 };
        result
    }

    unsafe fn maskload_pd(mem_addr: &f64, mask: Self::Vi64) -> Self::Vf64 {
        let mut result = F64x2_41(_mm_setzero_pd());
        let ptr = mem_addr as *const f64;
        result[0] = if mask[0] != 0 { *ptr } else { 0.0 };
        result[1] = if mask[1] != 0 { *ptr.offset(1) } else { 0.0 };
        result
    }

    unsafe fn mullo_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x4_41(_mm_mullo_epi32(a.0, b.0))
    }

    unsafe fn mullo_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        let mut result = Self::setzero_epi64();
        result[0] = a[0] * b[0];
        result[1] = a[1] * b[1];
        result
    }

    unsafe fn shuffle_epi32(a: Self::Vi32, imm8: i32) -> Self::Vi32 {
        macro_rules! call {
            ($imm8:expr) => {
                I32x4_41(_mm_shuffle_epi32(a.0, $imm8))
            };
        }
        constify_imm8!(imm8, call)
    }
}
