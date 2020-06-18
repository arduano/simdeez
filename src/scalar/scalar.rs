use super::*;
use core::mem;

use libm::{F32Ext, F64Ext};

pub struct Scalar;
impl Simd for Scalar {
    type Vi16 = I16x1;
    type Vi32 = I32x1;
    type Vf32 = F32x1;
    type Vf64 = F64x1;
    type Vi64 = I64x1;
    const VF32_WIDTH: usize = 1;
    const VF64_WIDTH: usize = 1;
    const VI16_WIDTH: usize = 1;
    const VI32_WIDTH: usize = 1;
    const VI64_WIDTH: usize = 1;

    #[inline(always)]
    unsafe fn abs_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x1(a.0.abs())
    }
    #[inline(always)]
    unsafe fn abs_pd(a: Self::Vf64) -> Self::Vf64 {
        F64x1(a.0.abs())
    }
    #[inline(always)]
    unsafe fn mullo_epi16(a: Self::Vi16, b: Self::Vi16) -> Self::Vi16 {
        a * b
    }
    #[inline(always)]
    unsafe fn andnot_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        (!a) & b
    }
    #[inline(always)]
    unsafe fn andnot_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        (!a) & b
    }
    #[inline(always)]
    unsafe fn andnot_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        (!a) & b
    }
    #[inline(always)]
    unsafe fn andnot_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        (!a) & b
    }
    #[inline(always)]
    unsafe fn blendv_epi32(a: Self::Vi32, b: Self::Vi32, mask: Self::Vi32) -> Self::Vi32 {
        if mask.0 != 0 {
            b
        } else {
            a
        }
    }
    #[inline(always)]
    unsafe fn blendv_epi64(a: Self::Vi64, b: Self::Vi64, mask: Self::Vi64) -> Self::Vi64 {
        if mask.0 != 0 {
            b
        } else {
            a
        }
    }
    #[inline(always)]
    unsafe fn blendv_ps(a: Self::Vf32, b: Self::Vf32, mask: Self::Vf32) -> Self::Vf32 {
        if mask.0 != 0.0 {
            b
        } else {
            a
        }
    }
    #[inline(always)]
    unsafe fn blendv_pd(a: Self::Vf64, b: Self::Vf64, mask: Self::Vf64) -> Self::Vf64 {
        if mask.0 != 0.0 {
            b
        } else {
            a
        }
    }
    #[inline(always)]
    unsafe fn castps_epi32(a: Self::Vf32) -> Self::Vi32 {
        I32x1(mem::transmute::<f32, i32>(a.0))
    }
    #[inline(always)]
    unsafe fn castpd_epi64(a: Self::Vf64) -> Self::Vi64 {
        I64x1(mem::transmute::<f64, i64>(a.0))
    }
    #[inline(always)]
    unsafe fn castepi32_ps(a: Self::Vi32) -> Self::Vf32 {
        F32x1(mem::transmute::<i32, f32>(a.0))
    }
    #[inline(always)]
    unsafe fn castepi64_pd(a: Self::Vi64) -> Self::Vf64 {
        F64x1(mem::transmute::<i64, f64>(a.0))
    }
    #[inline(always)]
    unsafe fn castps_pd(a: Self::Vf32) -> Self::Vf64 {
        F64x1(a.0 as f64)
    }
    #[inline(always)]
    unsafe fn castpd_ps(a: Self::Vf64) -> Self::Vf32 {
        F32x1(a.0 as f32)
    }
    #[inline(always)]
    unsafe fn cmpeq_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        if a.0 == b.0 {
            I32x1(-1)
        } else {
            I32x1(0)
        }
    }
    #[inline(always)]
    unsafe fn cmpneq_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        if a.0 != b.0 {
            I32x1(-1)
        } else {
            I32x1(0)
        }
    }
    #[inline(always)]
    unsafe fn cmpge_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        if a.0 >= b.0 {
            I32x1(-1)
        } else {
            I32x1(0)
        }
    }
    #[inline(always)]
    unsafe fn cmpgt_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        if a.0 > b.0 {
            I32x1(-1)
        } else {
            I32x1(0)
        }
    }
    #[inline(always)]
    unsafe fn cmple_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        if a.0 <= b.0 {
            I32x1(-1)
        } else {
            I32x1(0)
        }
    }
    #[inline(always)]
    unsafe fn cmplt_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        if a.0 < b.0 {
            I32x1(-1)
        } else {
            I32x1(0)
        }
    }
    #[inline(always)]
    unsafe fn cmpeq_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        if a.0 == b.0 {
            I64x1(-1)
        } else {
            I64x1(0)
        }
    }
    #[inline(always)]
    unsafe fn cmpneq_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        if a.0 != b.0 {
            I64x1(-1)
        } else {
            I64x1(0)
        }
    }
    #[inline(always)]
    unsafe fn cmpge_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        if a.0 >= b.0 {
            I64x1(-1)
        } else {
            I64x1(0)
        }
    }
    #[inline(always)]
    unsafe fn cmpgt_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        if a.0 > b.0 {
            I64x1(-1)
        } else {
            I64x1(0)
        }
    }
    #[inline(always)]
    unsafe fn cmple_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        if a.0 <= b.0 {
            I64x1(-1)
        } else {
            I64x1(0)
        }
    }
    #[inline(always)]
    unsafe fn cmplt_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        if a.0 < b.0 {
            I64x1(-1)
        } else {
            I64x1(0)
        }
    }
    #[inline(always)]
    unsafe fn cmpeq_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        if a.0 == b.0 {
            F32x1(mem::transmute::<i32, f32>(-1))
        } else {
            F32x1(0.0)
        }
    }
    #[inline(always)]
    unsafe fn cmpneq_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        if a.0 != b.0 {
            F32x1(mem::transmute::<i32, f32>(-1))
        } else {
            F32x1(0.0)
        }
    }
    #[inline(always)]
    unsafe fn cmpge_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        if a.0 >= b.0 {
            F32x1(mem::transmute::<i32, f32>(-1))
        } else {
            F32x1(0.0)
        }
    }
    #[inline(always)]
    unsafe fn cmpgt_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        if a.0 > b.0 {
            F32x1(mem::transmute::<i32, f32>(-1))
        } else {
            F32x1(0.0)
        }
    }
    #[inline(always)]
    unsafe fn cmple_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        if a.0 <= b.0 {
            F32x1(mem::transmute::<i32, f32>(-1))
        } else {
            F32x1(0.0)
        }
    }
    #[inline(always)]
    unsafe fn cmplt_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        if a.0 < b.0 {
            F32x1(mem::transmute::<i32, f32>(-1))
        } else {
            F32x1(0.0)
        }
    }
    #[inline(always)]
    unsafe fn cmpeq_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        if a.0 == b.0 {
            F64x1(mem::transmute::<i64, f64>(-1))
        } else {
            F64x1(0.0)
        }
    }
    #[inline(always)]
    unsafe fn cmpneq_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        if a.0 != b.0 {
            F64x1(mem::transmute::<i64, f64>(-1))
        } else {
            F64x1(0.0)
        }
    }
    #[inline(always)]
    unsafe fn cmpge_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        if a.0 >= b.0 {
            F64x1(mem::transmute::<i64, f64>(-1))
        } else {
            F64x1(0.0)
        }
    }
    #[inline(always)]
    unsafe fn cmpgt_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        if a.0 > b.0 {
            F64x1(mem::transmute::<i64, f64>(-1))
        } else {
            F64x1(0.0)
        }
    }
    #[inline(always)]
    unsafe fn cmple_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        if a.0 <= b.0 {
            F64x1(mem::transmute::<i64, f64>(-1))
        } else {
            F64x1(0.0)
        }
    }
    #[inline(always)]
    unsafe fn cmplt_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        if a.0 < b.0 {
            F64x1(mem::transmute::<i64, f64>(-1))
        } else {
            F64x1(0.0)
        }
    }   
    #[inline(always)]
    unsafe fn cvtps_epi32(a: Self::Vf32) -> Self::Vi32 {
        I32x1((a.0 + 0.5).floor() as i32)
    }
    #[inline(always)]
    unsafe fn cvtpd_epi64(a: Self::Vf64) -> Self::Vi64 {
        I64x1((a.0 + 0.5).floor() as i64)
    }
    #[inline(always)]
    unsafe fn cvtepi32_ps(a: Self::Vi32) -> Self::Vf32 {
        F32x1(a.0 as f32)
    }
    #[inline(always)]
    unsafe fn cvtepi64_pd(a: Self::Vi64) -> Self::Vf64 {
        F64x1(a.0 as f64)
    }
    #[inline(always)]
    unsafe fn ceil_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x1(a.0.ceil())
    }
    #[inline(always)]
    unsafe fn ceil_pd(a: Self::Vf64) -> Self::Vf64 {
        F64x1(a.0.ceil())
    }
    #[inline(always)]
    unsafe fn floor_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x1(a.0.floor())
    }
    #[inline(always)]
    unsafe fn floor_pd(a: Self::Vf64) -> Self::Vf64 {
        F64x1(a.0.floor())
    }
    #[inline(always)]
    unsafe fn fast_floor_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x1(a.0.floor())
    }
    #[inline(always)]
    unsafe fn fast_floor_pd(a: Self::Vf64) -> Self::Vf64 {
        F64x1(a.0.floor())
    }
    #[inline(always)]
    unsafe fn fast_ceil_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x1(a.0.ceil())
    }
    #[inline(always)]
    unsafe fn fast_round_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x1(a.0.round())
    }
    #[inline(always)]
    unsafe fn fmadd_ps(a: Self::Vf32, b: Self::Vf32, c: Self::Vf32) -> Self::Vf32 {
        a * b + c
    }
    #[inline(always)]
    unsafe fn fnmadd_ps(a: Self::Vf32, b: Self::Vf32, c: Self::Vf32) -> Self::Vf32 {
        F32x1((-(a.0 * b.0)) + c.0)
    }
    #[inline(always)]
    unsafe fn fmadd_pd(a: Self::Vf64, b: Self::Vf64, c: Self::Vf64) -> Self::Vf64 {
        a * b + c
    }
    #[inline(always)]
    unsafe fn fnmadd_pd(a: Self::Vf64, b: Self::Vf64, c: Self::Vf64) -> Self::Vf64 {
        F64x1((-(a.0 * b.0)) + c.0)
    }
    #[inline(always)]
    unsafe fn fmsub_ps(a: Self::Vf32, b: Self::Vf32, c: Self::Vf32) -> Self::Vf32 {
        a * b - c
    }
    #[inline(always)]
    unsafe fn fnmsub_ps(a: Self::Vf32, b: Self::Vf32, c: Self::Vf32) -> Self::Vf32 {
        F32x1((-(a.0 * b.0)) - c.0)
    }
    #[inline(always)]
    unsafe fn fmsub_pd(a: Self::Vf64, b: Self::Vf64, c: Self::Vf64) -> Self::Vf64 {
        a * b - c
    }
    #[inline(always)]
    unsafe fn fnmsub_pd(a: Self::Vf64, b: Self::Vf64, c: Self::Vf64) -> Self::Vf64 {
        F64x1((-(a.0 * b.0)) - c.0)
    }
    #[inline(always)]
    unsafe fn horizontal_add_ps(a: Self::Vf32) -> f32 {
        a.0
    }
    #[inline(always)]
    unsafe fn horizontal_add_pd(a: Self::Vf64) -> f64 {
        a.0
    }
    #[inline(always)]
    unsafe fn i32gather_epi32(arr: &[i32], index: Self::Vi32) -> Self::Vi32 {
        I32x1(arr[index.0 as usize])
    }
    #[inline(always)]
    unsafe fn i64gather_epi64(arr: &[i64], index: Self::Vi64) -> Self::Vi64 {
        I64x1(arr[index.0 as usize])
    }
    #[inline(always)]
    unsafe fn i32gather_ps(arr: &[f32], index: Self::Vi32) -> Self::Vf32 {
        F32x1(arr[index.0 as usize])
    }
    #[inline(always)]
    unsafe fn load_pd(a: &f64) -> Self::Vf64 {
        F64x1(*a)
    }
    #[inline(always)]
    unsafe fn load_ps(a: &f32) -> Self::Vf32 {
        F32x1(*a)
    }
    #[inline(always)]
    unsafe fn load_epi16(a: &i16) -> Self::Vi16 {
        I16x1(*a)
    }
    #[inline(always)]
    unsafe fn load_epi32(a: &i32) -> Self::Vi32 {
        I32x1(*a)
    }
    #[inline(always)]
    unsafe fn load_epi64(a: &i64) -> Self::Vi64 {
        I64x1(*a)
    }
    #[inline(always)]
    unsafe fn loadu_pd(a: &f64) -> Self::Vf64 {
        F64x1(*a)
    }
    #[inline(always)]
    unsafe fn loadu_ps(a: &f32) -> Self::Vf32 {
        F32x1(*a)
    }
    #[inline(always)]
    unsafe fn loadu_epi32(a: &i32) -> Self::Vi32 {
        I32x1(*a)
    }
    #[inline(always)]
    unsafe fn loadu_epi64(a: &i64) -> Self::Vi64 {
        I64x1(*a)
    }
    #[inline(always)]
    unsafe fn maskload_epi32(mem_addr: &i32, mask: Self::Vi32) -> Self::Vi32 {
        if mask.0 != 0 {
            I32x1(*mem_addr)
        } else {
            I32x1(0)
        }
    }
    #[inline(always)]
    unsafe fn maskload_epi64(mem_addr: &i64, mask: Self::Vi64) -> Self::Vi64 {
        if mask.0 != 0 {
            I64x1(*mem_addr)
        } else {
            I64x1(0)
        }
    }
    #[inline(always)]
    unsafe fn maskload_ps(mem_addr: &f32, mask: Self::Vi32) -> Self::Vf32 {
        if mask.0 != 0 {
            F32x1(*mem_addr)
        } else {
            F32x1(0.0)
        }
    }
    #[inline(always)]
    unsafe fn maskload_pd(mem_addr: &f64, mask: Self::Vi64) -> Self::Vf64 {
        if mask.0 != 0 {
            F64x1(*mem_addr)
        } else {
            F64x1(0.0)
        }
    }
    #[inline(always)]
    unsafe fn store_ps(mem_addr: &mut f32, a: Self::Vf32) {
        *mem_addr = a.0;
    }
    #[inline(always)]
    unsafe fn store_pd(mem_addr: &mut f64, a: Self::Vf64) {
        *mem_addr = a.0;
    }
    #[inline(always)]
    unsafe fn store_epi32(mem_addr: &mut i32, a: Self::Vi32) {
        *mem_addr = a.0;
    }
    #[inline(always)]
    unsafe fn store_epi64(mem_addr: &mut i64, a: Self::Vi64) {
        *mem_addr = a.0;
    }
    #[inline(always)]
    unsafe fn storeu_ps(mem_addr: &mut f32, a: Self::Vf32) {
        *mem_addr = a.0;
    }
    #[inline(always)]
    unsafe fn storeu_pd(mem_addr: &mut f64, a: Self::Vf64) {
        *mem_addr = a.0;
    }
    #[inline(always)]
    unsafe fn storeu_epi32(mem_addr: &mut i32, a: Self::Vi32) {
        *mem_addr = a.0;
    }
    #[inline(always)]
    unsafe fn storeu_epi64(mem_addr: &mut i64, a: Self::Vi64) {
        *mem_addr = a.0;
    }
    #[inline(always)]
    unsafe fn maskstore_epi32(mem_addr: &mut i32, mask: Self::Vi32, a: Self::Vi32) {
        if mask.0 != 0 {
            *mem_addr = a.0;
        }
    }
    #[inline(always)]
    unsafe fn maskstore_epi64(mem_addr: &mut i64, mask: Self::Vi64, a: Self::Vi64) {
        if mask.0 != 0 {
            *mem_addr = a.0;
        }
    }
    #[inline(always)]
    unsafe fn maskstore_ps(mem_addr: &mut f32, mask: Self::Vi32, a: Self::Vf32) {
        if mask.0 != 0 {
            *mem_addr = a.0;
        }
    }
    #[inline(always)]
    unsafe fn maskstore_pd(mem_addr: &mut f64, mask: Self::Vi64, a: Self::Vf64) {
        if mask.0 != 0 {
            *mem_addr = a.0;
        }
    }
    #[inline(always)]
    unsafe fn max_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        if a.0 > b.0 {
            a
        } else {
            b
        }
    }
    #[inline(always)]
    unsafe fn min_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        if a.0 < b.0 {
            a
        } else {
            b
        }
    }
    #[inline(always)]
    unsafe fn max_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        if a.0 > b.0 {
            a
        } else {
            b
        }
    }
    #[inline(always)]
    unsafe fn min_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        if a.0 < b.0 {
            a
        } else {
            b
        }
    }
    #[inline(always)]
    unsafe fn max_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        if a.0 > b.0 {
            a
        } else {
            b
        }
    }
    #[inline(always)]
    unsafe fn min_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        if a.0 < b.0 {
            a
        } else {
            b
        }
    }
    #[inline(always)]
    unsafe fn mullo_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        a * b
    }
    #[inline(always)]
    unsafe fn mullo_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        a * b
    }
    #[inline(always)]
    unsafe fn rcp_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x1(1.0 / a.0)
    }
    #[inline(always)]
    unsafe fn round_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x1(a.0.round())
    }
    #[inline(always)]
    unsafe fn round_pd(a: Self::Vf64) -> Self::Vf64 {
        F64x1(a.0.round())
    }
    #[inline(always)]
    unsafe fn set1_epi32(a: i32) -> Self::Vi32 {
        I32x1(a)
    }
    #[inline(always)]
    unsafe fn set1_epi64(a: i64) -> Self::Vi64 {
        I64x1(a)
    }
    #[inline(always)]
    unsafe fn set1_pd(a: f64) -> Self::Vf64 {
        F64x1(a)
    }
    #[inline(always)]
    unsafe fn set1_ps(a: f32) -> Self::Vf32 {
        F32x1(a)
    }
    #[inline(always)]
    unsafe fn setzero_pd() -> Self::Vf64 {
        F64x1(0.0)
    }
    #[inline(always)]
    unsafe fn setzero_ps() -> Self::Vf32 {
        F32x1(0.0)
    }
    #[inline(always)]
    unsafe fn setzero_epi32() -> Self::Vi32 {
        I32x1(0)
    }
    #[inline(always)]
    unsafe fn setzero_epi64() -> Self::Vi64 {
        I64x1(0)
    }
    #[inline(always)]
    unsafe fn srai_epi64(a: Self::Vi64, amt_const: i32) -> Self::Vi64 {
        a >> amt_const
    }
    #[inline(always)]
    unsafe fn srli_epi32(a: Self::Vi32, amt_const: i32) -> Self::Vi32 {
        //Transmute to unsigned so we don't get sign bits in the shift
        I32x1(mem::transmute::<u32, i32>(
            mem::transmute::<i32, u32>(a.0) >> amt_const,
        ))
    }
    #[inline(always)]
    unsafe fn sra_epi32(a: Self::Vi32, amt: i32) -> Self::Vi32 {
        a >> amt
    }
    #[inline(always)]
    unsafe fn srl_epi32(a: Self::Vi32, amt: i32) -> Self::Vi32 {
        I32x1(mem::transmute::<u32, i32>(
            mem::transmute::<i32, u32>(a.0) >> amt,
        ))
    }
    #[inline(always)]
    unsafe fn sll_epi32(a: Self::Vi32, amt: i32) -> Self::Vi32 {
        a << amt
    }
    #[inline(always)]
    unsafe fn sqrt_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x1(a.0.sqrt())
    }
    #[inline(always)]
    unsafe fn rsqrt_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x1(1.0 / a.0.sqrt())
    }
    #[inline(always)]
    unsafe fn sqrt_pd(a: Self::Vf64) -> Self::Vf64 {
        F64x1(a.0.sqrt())
    }
    #[inline(always)]
    unsafe fn rsqrt_pd(a: Self::Vf64) -> Self::Vf64 {
        F64x1(1.0 / a.0.sqrt())
    }
    #[inline(always)]
    unsafe fn shuffle_epi32(a: Self::Vi32, _imm8: i32) -> Self::Vi32 {
        a
    }
    cfg_if::cfg_if! {
        if #[cfg(feature = "sleef")] {
            #[inline(always)]
            unsafe fn sin_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x1(a.0.sin())
            }
            #[inline(always)]
            unsafe fn cos_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x1(a.0.cos())
            }
            #[inline(always)]
            unsafe fn fast_sin_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x1(a.0.sin())
            }
            #[inline(always)]
            unsafe fn fast_cos_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x1(a.0.cos())
            }
            #[inline(always)]
            unsafe fn asin_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x1(a.0.asin())
            }
            #[inline(always)]
            unsafe fn acos_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x1(a.0.acos())
            }
            #[inline(always)]
            unsafe fn fast_asin_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x1(a.0.asin())
            }
            #[inline(always)]
            unsafe fn fast_acos_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x1(a.0.acos())
            }
            #[inline(always)]
            unsafe fn tan_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x1(a.0.tan())
            }
            #[inline(always)]
            unsafe fn fast_tan_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x1(a.0.tan())
            }
            #[inline(always)]
            unsafe fn atan_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x1(a.0.atan())
            }

            #[inline(always)]
            unsafe fn fast_atan_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x1(a.0.atan())
            }
            //hyperbolic
            #[inline(always)]
            unsafe fn sinh_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x1(a.0.sinh())
            }
            #[inline(always)]
            unsafe fn cosh_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x1(a.0.cosh())
            }
            #[inline(always)]
            unsafe fn fast_sinh_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x1(a.0.sinh())
            }
            #[inline(always)]
            unsafe fn fast_cosh_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x1(a.0.cosh())
            }
            #[inline(always)]
            unsafe fn asinh_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x1(a.0.asinh())
            }
            #[inline(always)]
            unsafe fn acosh_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x1(a.0.acosh())
            }
            #[inline(always)]
            unsafe fn tanh_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x1(a.0.tanh())
            }
            #[inline(always)]
            unsafe fn fast_tanh_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x1(a.0.tanh())
            }
            #[inline(always)]
            unsafe fn atanh_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x1(a.0.atanh())
            }
            #[inline(always)]
            unsafe fn atan2_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
                F32x1(a.0.atan2(b.0))
            }
            #[inline(always)]
            unsafe fn fast_atan2_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
                F32x1(a.0.atan2(b.0))
            }
            #[inline(always)]
            unsafe fn ln_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x1(a.0.ln())
            }
            #[inline(always)]
            unsafe fn fast_ln_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x1(a.0.ln())
            }
            #[inline(always)]
            unsafe fn log2_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x1(a.0.log2())
            }
            #[inline(always)]
            unsafe fn log10_ps(a: Self::Vf32) -> Self::Vf32 {
                F32x1(a.0.log10())
            }
            #[inline(always)]
            unsafe fn hypot_ps(a:Self::Vf32,b:Self::Vf32) -> Self::Vf32 {
                F32x1(a.0.hypot(b.0))
            }
            #[inline(always)]
            unsafe fn fast_hypot_ps(a:Self::Vf32,b:Self::Vf32) -> Self::Vf32 {
                F32x1(a.0.hypot(b.0))
            }
            #[inline(always)]
            unsafe fn fmod_ps(a:Self::Vf32,b:Self::Vf32) -> Self::Vf32 {
                F32x1(a.0 % b.0)
            }
        }
    }
}
