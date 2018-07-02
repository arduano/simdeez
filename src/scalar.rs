use super::*;
use overloads::*;
use std::mem;
/// Scalar version of SIMD code will often be much slower than
/// they would be if written by hand, especially if the logic has branches.
/// This is provided for convenience when that performance penalty is not a
/// problem, or doesn't exist.
pub struct Scalar;

impl Simd for Scalar {
    type Vi32 = I32x1;
    type Vf32 = F32x1;

    const WIDTH_BYTES: usize = 4;

    #[inline(always)]
    unsafe fn abs_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x1(a.0.abs())
    }
    #[inline(always)]
    unsafe fn add_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x1(a.0 + b.0)
    }
    #[inline(always)]
    unsafe fn add_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x1(a.0 + b.0)
    }
    #[inline(always)]
    unsafe fn and_si(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x1(a.0 & b.0)
    }
    #[inline(always)]
    unsafe fn andnot_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        let ai = mem::transmute::<f32, i32>(a.0);
        let bi = mem::transmute::<f32, i32>(b.0);

        F32x1(mem::transmute::<i32, f32>((!ai) & bi))
    }
    #[inline(always)]
    unsafe fn andnot_si(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x1((!a.0) & b.0)
    }
    #[inline(always)]
    unsafe fn blendv_epi32(a: Self::Vi32, b: Self::Vi32, mask: Self::Vi32) -> Self::Vi32 {
        I32x1(((!mask.0) & a.0) | (mask.0 & b.0))
    }
    #[inline(always)]
    unsafe fn blendv_ps(a: Self::Vf32, b: Self::Vf32, mask: Self::Vf32) -> Self::Vf32 {
        let ai = mem::transmute::<f32, i32>(a.0);
        let bi = mem::transmute::<f32, i32>(b.0);
        let maski = mem::transmute::<f32, i32>(mask.0);
        F32x1(mem::transmute::<i32, f32>(((!maski) & ai) | (maski & bi)))
    }
    #[inline(always)]
    unsafe fn castps_si(a: Self::Vf32) -> Self::Vi32 {
        I32x1(mem::transmute::<f32, i32>(a.0))
    }
    #[inline(always)]
    unsafe fn castsi_ps(a: Self::Vi32) -> Self::Vf32 {
        F32x1(mem::transmute::<i32, f32>(a.0))
    }
    #[inline(always)]
    unsafe fn ceil_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x1(a.0.ceil())
    }
    #[inline(always)]
    unsafe fn cmpeq_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x1(mem::transmute::<bool, i8>(a.0 == b.0) as i32 * -1)
    }
    #[inline(always)]
    unsafe fn cmpge_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x1(mem::transmute::<i32, f32>(
            mem::transmute::<bool, i8>(a.0 >= b.0) as i32 * -1,
        ))
    }
    #[inline(always)]
    unsafe fn cmpgt_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x1(mem::transmute::<bool, i8>(a.0 > b.0) as i32 * -1)
    }
    #[inline(always)]
    unsafe fn cmpgt_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x1(mem::transmute::<i32, f32>(
            mem::transmute::<bool, i8>(a.0 > b.0) as i32 * -1,
        ))
    }
    #[inline(always)]
    unsafe fn cmplt_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x1(mem::transmute::<i32, f32>(
            mem::transmute::<bool, i8>(a.0 < b.0) as i32 * -1,
        ))
    }
    #[inline(always)]
    unsafe fn cvtepi32_ps(a: Self::Vi32) -> Self::Vf32 {
        F32x1(a.0 as f32)
    }
    #[inline(always)]
    unsafe fn cvtps_epi32(a: Self::Vf32) -> Self::Vi32 {
        I32x1(a.0 as i32)
    }
    #[inline(always)]
    unsafe fn floor_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x1(a.0.floor())
    }
    #[inline(always)]
    unsafe fn fastfloor_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x1(a.0.floor())
    }
    #[inline(always)]
    unsafe fn fmadd_ps(a: Self::Vf32, b: Self::Vf32, c: Self::Vf32) -> Self::Vf32 {
        F32x1(a.0 * b.0 + c.0)
    }
    #[inline(always)]
    unsafe fn fnmadd_ps(a: Self::Vf32, b: Self::Vf32, c: Self::Vf32) -> Self::Vf32 {
        F32x1((-(a.0 * b.0)) + c.0)
    }
    #[inline(always)]
    unsafe fn i32gather_epi32(arr: &[i32], index: Self::Vi32) -> Self::Vi32 {
        I32x1(arr[index.0 as usize])
    }
    #[inline(always)]
    unsafe fn i32gather_ps(arr: &[f32], index: Self::Vi32) -> Self::Vf32 {
        F32x1(arr[index.0 as usize])
    }
    #[inline(always)]
    unsafe fn loadu_ps(a: &f32) -> Self::Vf32 {
        F32x1(*a)
    }
    #[inline(always)]
    unsafe fn loadu_si(a: &i32) -> Self::Vi32 {
        I32x1(*a)
    }
    #[inline(always)]
    unsafe fn storeu_ps(a: &mut f32, b: Self::Vf32) {
        *a = b.0;
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
    unsafe fn mul_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x1(a.0 * b.0)
    }
    #[inline(always)]
    unsafe fn div_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x1(a.0 / b.0)
    }
    #[inline(always)]
    unsafe fn mullo_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x1(a.0 * b.0)
    }
    #[inline(always)]
    unsafe fn or_si(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x1(a.0 | b.0)
    }
    #[inline(always)]
    unsafe fn round_ps(a: Self::Vf32) -> Self::Vf32 {
        F32x1(a.0.round())
    }
    #[inline(always)]
    unsafe fn set1_epi32(a: i32) -> Self::Vi32 {
        I32x1(a)
    }
    #[inline(always)]
    unsafe fn set1_ps(a: f32) -> Self::Vf32 {
        F32x1(a)
    }
    #[inline(always)]
    unsafe fn setzero_ps() -> Self::Vf32 {
        F32x1(0.0)
    }
    #[inline(always)]
    unsafe fn setzero_si() -> Self::Vi32 {
        I32x1(0)
    }
    #[inline(always)]
    unsafe fn srai_epi32(a: Self::Vi32, imm8: i32) -> Self::Vi32 {
        I32x1(a.0 >> imm8)
    }
    #[inline(always)]
    unsafe fn sub_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x1(a.0 - b.0)
    }
    #[inline(always)]
    unsafe fn sub_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        F32x1(a.0 - b.0)
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
    unsafe fn xor_si(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        I32x1(a.0 ^ b.0)
    }
}
