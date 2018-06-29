use std::fmt::Debug;
#[macro_use]
pub mod macros;

pub mod avx2;
pub mod scalar;
pub mod sse2;
pub mod sse41;
use avx2::*;
use macros::*;
use scalar::*;
use sse2::*;
use sse41::*;

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

pub trait Simd {
    type Vi32: Copy + Debug;
    type Vf32: Copy + Debug;

    const WIDTH_BYTES: usize;

    unsafe fn set_lane_epi32(a: &mut Self::Vi32, value: i32, i: usize);
    unsafe fn set_lane_ps(a: &mut Self::Vf32, value: f32, i: usize);
    unsafe fn get_lane_epi32(a: Self::Vi32, i: usize) -> i32;
    unsafe fn get_lane_ps(a: Self::Vf32, i: usize) -> f32;
    unsafe fn abs_ps(a: Self::Vf32) -> Self::Vf32;
    unsafe fn add_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn add_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn and_si(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn andnot_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn andnot_si(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn blendv_epi32(a: Self::Vi32, b: Self::Vi32, mask: Self::Vi32) -> Self::Vi32;
    unsafe fn blendv_ps(a: Self::Vf32, b: Self::Vf32, mask: Self::Vf32) -> Self::Vf32;
    unsafe fn castps_si(a: Self::Vf32) -> Self::Vi32;
    unsafe fn castsi_ps(a: Self::Vi32) -> Self::Vf32;
    unsafe fn ceil_ps(a: Self::Vf32) -> Self::Vf32;
    unsafe fn cmpeq_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn cmpge_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn cmpgt_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn cmpgt_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn cmplt_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn cvtepi32_ps(a: Self::Vi32) -> Self::Vf32;
    unsafe fn cvtps_epi32(a: Self::Vf32) -> Self::Vi32;
    unsafe fn floor_ps(a: Self::Vf32) -> Self::Vf32;
    // Only works on fp values that can be representined by int values
    unsafe fn fastfloor_ps(a: Self::Vf32) -> Self::Vf32;
    unsafe fn fmadd_ps(a: Self::Vf32, b: Self::Vf32, c: Self::Vf32) -> Self::Vf32;
    unsafe fn fnmadd_ps(a: Self::Vf32, b: Self::Vf32, c: Self::Vf32) -> Self::Vf32;
    unsafe fn i32gather_epi32(arr: &[i32], index: Self::Vi32) -> Self::Vi32;
    unsafe fn i32gather_ps(arr: &[f32], index: Self::Vi32) -> Self::Vf32;
    unsafe fn loadu_ps(a: &f32) -> Self::Vf32;
    unsafe fn storeu_ps(a: &mut f32, b: Self::Vf32);
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
    unsafe fn sub_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn sub_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn xor_si(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
}

// The target_feature attributes ensure that the compiler emits the appropriate instructions on
// a per function basis.

#[cfg(test)]
mod tests {
    use super::*;
    // If using runtime feature detection, you will want to be sure this inlines
    #[inline(always)]
    unsafe fn sample<S: Simd>() -> f32 {
        let a = S::set1_ps(1.5);
        let b = S::set1_ps(2.5);
        // function names mirror the intel intrinsics, minus the _mm_ part
        let mut c = S::add_ps(a, b);
        // If your SIMD instruction set doesn't have floor, SIMDEEZ handles it for you
        c = S::floor_ps(c);
        // You can get the width of the instruction set you are working with
        let width = S::WIDTH_BYTES / 4;
        // And set or get individual lanes with ease
        S::get_lane_ps(c, width - 1)
    }

    // Make an sse2 version of sample
    #[target_feature(enable = "sse2")]
    unsafe fn sample_sse2() -> f32 {
        sample::<Sse2>()
    }

    // Make an avx2 version of sample
    #[target_feature(enable = "avx2")]
    unsafe fn sample_avx2() -> f32 {
        sample::<Avx2>()
    }
    #[target_feature(enable = "sse4.1")]
    unsafe fn sample_sse41() -> f32 {
        sample::<Sse41>()
    }
    unsafe fn sample_scalar() -> f32 {
        sample::<Scalar>()
    }

    #[inline(always)]
    unsafe fn setlanetest<S:Simd>() -> f32 {
        let mut a = S::set1_ps(1.0);
        S::set_lane_ps(&mut a,5.0,0);
        S::get_lane_ps(a,0)
    }
    unsafe fn setlanetest_scalar() -> f32 {
        setlanetest::<Scalar>()
    }   
    unsafe fn setlanetest_avx2() -> f32 {
        setlanetest::<Avx2>()
    }



    #[test]
    fn consistency() {
        unsafe {
        assert_eq!(sample_sse2(), sample_sse41());
        assert_eq!(sample_sse41(), sample_avx2());
        assert_eq!(sample_avx2(), sample_scalar());    
        }
    }
    #[test] 
    fn setlane() {
        unsafe {
            assert_eq!(setlanetest_avx2(),5.0);
            assert_eq!(setlanetest_scalar(),5.0);
        }
    }
}
