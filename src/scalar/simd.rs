use super::*;

pub struct Scalar;
impl Simd for Scalar {
    type Vi16 = I16x1;
    type Vi32 = I32x1;
    type Vf32 = F32x1;
    type Vf64 = F64x1;
    type Vi64 = I64x1;

    unsafe fn mullo_epi16(a: Self::Vi16, b: Self::Vi16) -> Self::Vi16 {
        a * b
    }

    unsafe fn castps_pd(a: Self::Vf32) -> Self::Vf64 {
        F64x1(a.0 as f64)
    }

    unsafe fn castpd_ps(a: Self::Vf64) -> Self::Vf32 {
        F32x1(a.0 as f32)
    }

    unsafe fn i32gather_epi32(arr: &[i32], index: Self::Vi32) -> Self::Vi32 {
        I32x1(arr[index.0 as usize])
    }

    unsafe fn i64gather_epi64(arr: &[i64], index: Self::Vi64) -> Self::Vi64 {
        I64x1(arr[index.0 as usize])
    }

    unsafe fn i32gather_ps(arr: &[f32], index: Self::Vi32) -> Self::Vf32 {
        F32x1(arr[index.0 as usize])
    }

    unsafe fn maskload_epi32(mem_addr: &i32, mask: Self::Vi32) -> Self::Vi32 {
        if mask.0 != 0 {
            I32x1(*mem_addr)
        } else {
            I32x1(0)
        }
    }

    unsafe fn maskload_epi64(mem_addr: &i64, mask: Self::Vi64) -> Self::Vi64 {
        if mask.0 != 0 {
            I64x1(*mem_addr)
        } else {
            I64x1(0)
        }
    }

    unsafe fn maskload_ps(mem_addr: &f32, mask: Self::Vi32) -> Self::Vf32 {
        if mask.0 != 0 {
            F32x1(*mem_addr)
        } else {
            F32x1(0.0)
        }
    }

    unsafe fn maskload_pd(mem_addr: &f64, mask: Self::Vi64) -> Self::Vf64 {
        if mask.0 != 0 {
            F64x1(*mem_addr)
        } else {
            F64x1(0.0)
        }
    }

    unsafe fn mullo_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        a * b
    }

    unsafe fn mullo_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        a * b
    }

    unsafe fn shuffle_epi32(a: Self::Vi32, _imm8: i32) -> Self::Vi32 {
        a
    }
}
