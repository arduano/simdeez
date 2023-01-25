#![allow(deprecated)]

use super::*;

pub struct Avx2;
impl Simd for Avx2 {
    type Vi16 = I16x16;
    type Vi32 = I32x8;
    type Vf32 = F32x8;
    type Vf64 = F64x4;
    type Vi64 = I64x4;

    unsafe fn mullo_epi16(a: Self::Vi16, b: Self::Vi16) -> Self::Vi16 {
        I16x16(_mm256_mullo_epi16(a.0, b.0))
    }

    unsafe fn castps_pd(a: Self::Vf32) -> Self::Vf64 {
        F64x4(_mm256_castps_pd(a.0))
    }

    unsafe fn castpd_ps(a: Self::Vf64) -> Self::Vf32 {
        F32x8(_mm256_castpd_ps(a.0))
    }

    unsafe fn i32gather_epi32(arr: &[i32], index: Self::Vi32) -> Self::Vi32 {
        I32x8(_mm256_i32gather_epi32(&arr[0] as *const i32, index.0, 4))
    }

    unsafe fn i64gather_epi64(arr: &[i64], index: Self::Vi64) -> Self::Vi64 {
        I64x4(_mm256_i64gather_epi64(&arr[0] as *const i64, index.0, 8))
    }

    unsafe fn i32gather_ps(arr: &[f32], index: Self::Vi32) -> Self::Vf32 {
        F32x8(_mm256_i32gather_ps(&arr[0] as *const f32, index.0, 4))
    }

    unsafe fn maskload_epi32(mem_addr: &i32, mask: Self::Vi32) -> Self::Vi32 {
        I32x8(_mm256_maskload_epi32(mem_addr as *const i32, mask.0))
    }

    unsafe fn maskload_epi64(mem_addr: &i64, mask: Self::Vi64) -> Self::Vi64 {
        I64x4(_mm256_maskload_epi64(mem_addr as *const i64, mask.0))
    }

    unsafe fn maskload_ps(mem_addr: &f32, mask: Self::Vi32) -> Self::Vf32 {
        F32x8(_mm256_maskload_ps(mem_addr as *const f32, mask.0))
    }

    unsafe fn maskload_pd(mem_addr: &f64, mask: Self::Vi64) -> Self::Vf64 {
        F64x4(_mm256_maskload_pd(mem_addr as *const f64, mask.0))
    }

    unsafe fn mullo_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x8(_mm256_mullo_epi32(a.0, b.0))
    }

    unsafe fn mullo_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        let mut result = Self::setzero_epi64();
        result[0] = a[0] * b[0];
        result[1] = a[1] * b[1];
        result[2] = a[2] * b[2];
        result[3] = a[3] * b[3];
        result
    }

    unsafe fn shuffle_epi32(a: Self::Vi32, imm8: i32) -> Self::Vi32 {
        macro_rules! call {
            ($imm8:expr) => {
                I32x8(_mm256_shuffle_epi32(a.0, $imm8))
            };
        }
        constify_imm8!(imm8, call)
    }
}
