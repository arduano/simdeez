use std::fmt::Debug;
#[macro_use]
pub mod macros;

pub mod avx2;
pub mod sse41;
use sse41::*;
use avx2::*;
use macros::*;



#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

trait Simd {
    type Vi32: Copy + Debug;
    type Vf32: Copy + Debug;

    fn get_width_bytes() -> usize;
    unsafe fn abs_ps(a: Self::Vf32) -> Self::Vf32;
    unsafe fn add_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn add_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn and_si(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn andnot_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn blendv_epi32(a: Self::Vi32, b: Self::Vi32, mask: Self::Vi32) -> Self::Vi32;
    unsafe fn blendv_ps(a: Self::Vf32, b: Self::Vf32, mask:Self::Vf32) -> Self::Vf32;
    unsafe fn castps_si(a: Self::Vf32) -> Self::Vi32;
    unsafe fn castsi_ps(a: Self::Vi32) -> Self::Vf32;
    unsafe fn cmpeq_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn cmpge_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn cmpgt_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn cmpgt_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn cmplt_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn cvtepi32_ps(a: Self::Vi32) -> Self::Vf32;
    unsafe fn cvtps_epi32(a: Self::Vf32) -> Self::Vi32;
    unsafe fn floor_ps(a: Self::Vf32) -> Self::Vf32;
    unsafe fn fmadd_ps(a: Self::Vf32, b: Self::Vf32, c: Self::Vf32) -> Self::Vf32;
    unsafe fn fnmadd_ps(a: Self::Vf32, b: Self::Vf32, c: Self::Vf32) -> Self::Vf32;
    unsafe fn i32gather_epi32(arr: &[i32], index: Self::Vi32) -> Self::Vi32;
    unsafe fn i32gather_ps(arr: &[f32], index: Self::Vi32) -> Self::Vf32;
    unsafe fn loadu_ps(a: &f32) -> Self::Vf32;
    unsafe fn max_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn min_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn mul_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn mullo_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn or_si(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn round_ps(a: Self::Vf32) -> Self::Vf32;
    unsafe fn set1_epi32(a: i32) -> Self::Vi32;
    unsafe fn set1_ps(a: f32) -> Self::Vf32;
    unsafe fn setzero_ps() -> Self::Vf32;
    unsafe fn setzero_si() -> Self::Vi32;
    unsafe fn srai_epi32(a: Self::Vi32, imm8: i32) -> Self::Vi32;
    unsafe fn storeu_ps(a: &mut f32, Self::Vf32);
    unsafe fn sub_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn sub_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn xor_si(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
}

#[target_feature(enable="avx2")]
unsafe fn testfunc<S: Simd>() -> S::Vf32 {
    let a = S::set1_ps(1.0);
    a
}

fn main() {
   unsafe { let a = testfunc::<Avx2>();}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
