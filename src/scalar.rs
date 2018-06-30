//TODO there is a bug in here somewhere, manifesting with extremely large indexes coming
//into the gather instruction from 2d simplex noise
//
use super::*;
use std::mem;
pub struct Scalar;

impl Simd for Scalar {
    type Vi32 = i32;
    type Vf32 = f32;

    const WIDTH_BYTES: usize = 4;

    unsafe fn set_lane_epi32(a: &mut Self::Vi32, value: i32, i: usize) {
        *a = value;
    }
    unsafe fn set_lane_ps(a: &mut Self::Vf32, value: f32, i: usize) {
        *a = value;
    }
    unsafe fn get_lane_epi32(a: Self::Vi32, i: usize) -> i32 {
        a
    }
    unsafe fn get_lane_ps(a: Self::Vf32, i: usize) -> f32 {
        a
    }
    unsafe fn abs_ps(a: Self::Vf32) -> Self::Vf32 {
        a.abs()
    }
    unsafe fn add_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        a + b
    }
    unsafe fn add_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        a + b
    }
    unsafe fn and_si(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        a & b
    }
    unsafe fn andnot_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        let ai = mem::transmute::<f32, i32>(a);
        let bi = mem::transmute::<f32, i32>(b);

        mem::transmute::<i32, f32>((!ai) & bi)
    }
    unsafe fn andnot_si(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        (!a) & b
    }
    // TODO Would an if statement perform better?
    unsafe fn blendv_epi32(a: Self::Vi32, b: Self::Vi32, mask: Self::Vi32) -> Self::Vi32 {
        ((!mask) & a) | (mask & b)
    }
    unsafe fn blendv_ps(a: Self::Vf32, b: Self::Vf32, mask: Self::Vf32) -> Self::Vf32 {
        let ai = mem::transmute::<f32, i32>(a);
        let bi = mem::transmute::<f32, i32>(b);
        let maski = mem::transmute::<f32, i32>(mask);
        mem::transmute::<i32, f32>(((!maski) & ai) | (maski & bi))
    }
    unsafe fn castps_si(a: Self::Vf32) -> Self::Vi32 {
        mem::transmute::<f32, i32>(a)
    }
    unsafe fn castsi_ps(a: Self::Vi32) -> Self::Vf32 {
        mem::transmute::<i32, f32>(a)
    }
    unsafe fn ceil_ps(a: Self::Vf32) -> Self::Vf32 {
        a.ceil()
    }
    unsafe fn cmpeq_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        if  a == b { -1 } else { 0 }
    }
    unsafe fn cmpge_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        if a >= b { mem::transmute::<i32,f32>(-1) } else { mem::transmute::<i32,f32>(0) }
    }
    unsafe fn cmpgt_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        if a > b { -1 } else { 0 } 
    }
    unsafe fn cmpgt_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        if a > b { mem::transmute::<i32,f32>(-1) } else { mem::transmute::<i32,f32>(0) }
    }
    unsafe fn cmplt_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        if a < b { mem::transmute::<i32,f32>(-1) } else { mem::transmute::<i32,f32>(0) }
    }
    unsafe fn cvtepi32_ps(a: Self::Vi32) -> Self::Vf32 {
        a as f32
    }
    unsafe fn cvtps_epi32(a: Self::Vf32) -> Self::Vi32 {
        a as i32
    }
    unsafe fn floor_ps(a: Self::Vf32) -> Self::Vf32 {
        a.floor()
    }
    unsafe fn fastfloor_ps(a: Self::Vf32) -> Self::Vf32 {
        a.floor()
    }
    unsafe fn fmadd_ps(a: Self::Vf32, b: Self::Vf32, c: Self::Vf32) -> Self::Vf32 {
        a * b + c
    }
    unsafe fn fnmadd_ps(a: Self::Vf32, b: Self::Vf32, c: Self::Vf32) -> Self::Vf32 {
        (-(a * b)) + c
    }
    unsafe fn i32gather_epi32(arr: &[i32], index: Self::Vi32) -> Self::Vi32 {
        arr[index as usize]
    }
    unsafe fn i32gather_ps(arr: &[f32], index: Self::Vi32) -> Self::Vf32 {
        arr[index as usize]
    }
    unsafe fn loadu_ps(a: &f32) -> Self::Vf32 {
        *a
    }
    unsafe fn loadu_si(a: &i32) -> Self::Vi32 {
        *a
    }
    unsafe fn storeu_ps(a: &mut f32, b: Self::Vf32) {
        *a = b;
    }
    unsafe fn max_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        if a > b {
            a
        } else {
            b
        }
    }
    unsafe fn min_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        if a < b {
            a
        } else {
            b
        }
    }
    unsafe fn mul_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        a * b
    }
    unsafe fn div_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        a / b
    }
    unsafe fn mullo_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        a * b
    }
    unsafe fn or_si(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        a | b
    }
    unsafe fn round_ps(a: Self::Vf32) -> Self::Vf32 {
        a.round()
    }
    unsafe fn set1_epi32(a: i32) -> Self::Vi32 {
        a
    }
    unsafe fn set1_ps(a: f32) -> Self::Vf32 {
        a
    }
    unsafe fn setzero_ps() -> Self::Vf32 {
        0.0
    }
    unsafe fn setzero_si() -> Self::Vi32 {
        0
    }
    unsafe fn srai_epi32(a: Self::Vi32, imm8: i32) -> Self::Vi32 {
        a << imm8
    }
    unsafe fn sub_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        a - b
    }
    unsafe fn sub_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        a - b
    }
    unsafe fn xor_si(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        a ^ b
    }
}
