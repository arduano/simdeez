//! A library that abstracts over SIMD instruction sets, including ones with differing widths.
//! SIMDeez is designed to allow you to write a function one time and produce SSE2, SSE41, and AVX2 versions of the function.
//! You can either have the version you want chosen at compile time with `cfg` attributes, or at runtime with
//! `target_feature` attributes and using the built in `is_x86_feature_detected!' macro.
//!
//! SIMDeez is currently in Beta, if there are intrinsics you need that are not currently implemented, create an issue
//! and I'll add them. PRs to add more intrinsics are welcome. Currently things are well fleshed out for i32, i64, f32, and f64 types.
//!
//! As Rust stabilizes support for Neon and AVX-512 I plan to add those as well.
//!
//! Refer to the excellent [Intel Intrinsics Guide](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#) for documentation on these functions.
//!
//! # Features
//!
//! * SSE2, SSE41, and AVX2
//! * Can use used with compile time or run time selection
//! * No runtime overhead
//! * Uses familiar intel intrinsic naming conventions, easy to port.
//!   * `_mm_add_ps(a,b)` becomes `add_ps(a,b)`
//! * Fills in missing intrinsics in older APIs with fast SIMD workarounds.
//!   * ceil, floor, round,blend, etc
//! * Can be used by `#[no_std]` projects
//! * Operator overloading: `let sum = va + vb` or `s *= s`
//! * Extract or set a single lane with the index operator: `let v1 = v[1];`
//!
//! # Compared to stdsimd
//!
//! * SIMDeez can abstract over differing simd widths. stdsimd does not
//! * SIMDeez builds on stable rust now, stdsimd does not
//!
//! # Compared to Faster
//!
//! * SIMDeez can be used with runtime selection, Faster cannot.
//! * SIMDeez has faster fallbacks for some functions
//! * SIMDeez does not currently work with iterators, Faster does.
//! * SIMDeez uses more idiomatic intrinsic syntax while Faster uses more idomatic Rust syntax
//! * SIMDeez can be used by `#[no_std]` projects
//! * SIMDeez builds on stable rust now, Faster does not.
//!
//! All of the above could change! Faster seems to generally have the same
//! performance as long as you don't run into some of the slower fallback functions.
//!
//!
//! # Example
//!
//! ```rust
//!     use simdeez::*;
//!     use simdeez::sse2::*;
//!     use simdeez::sse41::*;
//!     use simdeez::avx2::*;
//!     // If using runtime feature detection, you will want to be sure this inlines
//!     // so you can leverage target_feature attributes
//!     #[inline(always)]
//!     unsafe fn distance<S: Simd>(
//!         x1: &[f32], 
//!         y1: &[f32], 
//!         x2: &[f32], 
//!         y2: &[f32]) -> Vec<f32> {
//! 
//!         let mut result: Vec<f32> = Vec::with_capacity(x1.len());
//!         result.set_len(x1.len()); // for efficiency
//! 
//!         // Operations have to be done in terms of the vector width
//!         // so that it will work with any size vector.
//!         // the width of a vector type is provided as a constant
//!         // so the compiler is free to optimize it more.
//!         let mut i = 0;
//!         //S::VF32_WIDTH is a constant, 4 when using SSE, 8 when using AVX2, etc
//!         while i < x1.len() {
//!             //load data from your vec into a SIMD value
//!             let xv1 = S::loadu_ps(&x1[i]);
//!             let yv1 = S::loadu_ps(&y1[i]);
//!             let xv2 = S::loadu_ps(&x2[i]);
//!             let yv2 = S::loadu_ps(&y2[i]);
//! 
//!             // Use the usual intrinsic syntax if you prefer
//!             let mut xdiff = S::sub_ps(xv1, xv2);
//!             // Or use operater overloading if you like
//!             let mut ydiff = yv1 - yv2;
//!             xdiff *= xdiff;
//!             ydiff *= ydiff;
//!             let distance = S::sqrt_ps(xdiff + ydiff);
//!             // Store the SIMD value into the result vec
//!             S::storeu_ps(&mut result[i], distance);
//!             // Increment i by the vector width
//!             i += S::VF32_WIDTH
//!         }
//!         result
//!     }
//! 
//!     //Call distance as an SSE2 function
//!     #[target_feature(enable = "sse2")]
//!     unsafe fn distance_sse2(
//!         x1: &[f32], 
//!         y1: &[f32], 
//!         x2: &[f32], 
//!         y2: &[f32]) -> Vec<f32> {
//!         distance::<Sse2>(x1, y1, x2, y2)
//!     }
//!     //Call distance as an SSE41 function
//!     #[target_feature(enable = "sse4.1")]
//!     unsafe fn distance_sse41(
//!         x1: &[f32], 
//!         y1: &[f32], 
//!         x2: &[f32], 
//!         y2: &[f32]) -> Vec<f32> {
//!         distance::<Sse41>(x1, y1, x2, y2)
//!     }
//!     //Call distance as an AVX2 function
//!     #[target_feature(enable = "avx2")]
//!     unsafe fn distance_avx2(
//!         x1: &[f32], 
//!         y1: &[f32], 
//!         x2: &[f32], 
//!         y2: &[f32]) -> Vec<f32> {
//!         distance::<Avx2>(x1, y1, x2, y2)
//!     }
//! ```
#![no_std]
#[macro_use]
#[cfg(test)]
extern crate std;
#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;
use core::fmt::Debug;
use core::ops::*;
#[macro_use]
mod macros;
pub mod avx2;
pub mod overloads;
pub mod sse2;
pub mod sse41;

pub trait Simd {
    /// Vi32 stands for Vector of i32s.  Corresponds to __m128i when used
    /// with the Sse impl, __m256i when used with Avx2, or a single i32
    /// when used with Scalar.
    type Vi32: Copy
        + Debug
        + Add<Self::Vi32, Output = Self::Vi32>
        + Sub<Self::Vi32, Output = Self::Vi32>
        + Mul<Self::Vi32, Output = Self::Vi32>
        + AddAssign<Self::Vi32>
        + SubAssign<Self::Vi32>
        + MulAssign<Self::Vi32>
        + BitAnd<Self::Vi32, Output = Self::Vi32>
        + BitOr<Self::Vi32, Output = Self::Vi32>
        + BitXor<Self::Vi32, Output = Self::Vi32>
        + BitAndAssign<Self::Vi32>
        + BitOrAssign<Self::Vi32>
        + BitXorAssign<Self::Vi32>
        + Index<usize, Output = i32>
        + IndexMut<usize>;
    /// Vf32 stands for Vector of f32s.  Corresponds to __m128 when used
    /// with the Sse impl, __m256 when used with Avx2, or a single f32
    /// when used with Scalar.
    type Vf32: Copy
        + Debug
        + Add<Self::Vf32, Output = Self::Vf32>
        + Sub<Self::Vf32, Output = Self::Vf32>
        + Mul<Self::Vf32, Output = Self::Vf32>
        + Div<Self::Vf32, Output = Self::Vf32>
        + AddAssign<Self::Vf32>
        + SubAssign<Self::Vf32>
        + MulAssign<Self::Vf32>
        + DivAssign<Self::Vf32>
        + BitAnd<Self::Vf32, Output = Self::Vf32>
        + BitOr<Self::Vf32, Output = Self::Vf32>
        + BitXor<Self::Vf32, Output = Self::Vf32>
        + BitAndAssign<Self::Vf32>
        + BitOrAssign<Self::Vf32>
        + BitXorAssign<Self::Vf32>
        + Index<usize, Output = f32>
        + IndexMut<usize>;

    /// Vi64 stands for Vector of f64s.  Corresponds to __m128 when used
    /// with the Sse impl, __m256 when used with Avx2, or a single f64
    /// when used with Scalar.
    type Vf64: Copy
        + Debug
        + Index<usize, Output = f64>
        + IndexMut<usize>
        + Add<Self::Vf64, Output = Self::Vf64>
        + Sub<Self::Vf64, Output = Self::Vf64>
        + Mul<Self::Vf64, Output = Self::Vf64>
        + Div<Self::Vf64, Output = Self::Vf64>
        + AddAssign<Self::Vf64>
        + SubAssign<Self::Vf64>
        + MulAssign<Self::Vf64>
        + DivAssign<Self::Vf64>
        + BitAnd<Self::Vf64, Output = Self::Vf64>
        + BitOr<Self::Vf64, Output = Self::Vf64>
        + BitXor<Self::Vf64, Output = Self::Vf64>
        + BitAndAssign<Self::Vf64>
        + BitOrAssign<Self::Vf64>
        + BitXorAssign<Self::Vf64>;

    type Vi64: Copy
        + Debug
        + Index<usize, Output = i64>
        + IndexMut<usize>
        + Add<Self::Vi64, Output = Self::Vi64>
        + Sub<Self::Vi64, Output = Self::Vi64>
        + AddAssign<Self::Vi64>
        + SubAssign<Self::Vi64>
        + BitAnd<Self::Vi64, Output = Self::Vi64>
        + BitOr<Self::Vi64, Output = Self::Vi64>
        + BitXor<Self::Vi64, Output = Self::Vi64>
        + BitAndAssign<Self::Vi64>
        + BitOrAssign<Self::Vi64>
        + BitXorAssign<Self::Vi64>;

    /// The width of the vector lane.  Necessary for creating
    /// lane width agnostic code.
    const VF32_WIDTH: usize;
    const VF64_WIDTH: usize;
    const VI32_WIDTH: usize;
    const VI64_WIDTH: usize;

    unsafe fn div_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn div_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64;
    /// Equivalent to transmuting the SIMD type to an array and accessing
    /// it at the index i.
    unsafe fn abs_ps(a: Self::Vf32) -> Self::Vf32;
    unsafe fn abs_pd(a: Self::Vf64) -> Self::Vf64;
    unsafe fn add_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn add_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn add_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64;
    unsafe fn and_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn and_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64;
    unsafe fn andnot_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn andnot_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64;
    unsafe fn andnot_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn andnot_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64;
    /// This is provided for convenience, it uses casts and the blendv_ps
    /// intrinsics to implement it.
    unsafe fn blendv_epi32(a: Self::Vi32, b: Self::Vi32, mask: Self::Vi32) -> Self::Vi32;
    unsafe fn blendv_ps(a: Self::Vf32, b: Self::Vf32, mask: Self::Vf32) -> Self::Vf32;
    unsafe fn blendv_pd(a: Self::Vf64, b: Self::Vf64, mask: Self::Vf64) -> Self::Vf64;
    unsafe fn castps_epi32(a: Self::Vf32) -> Self::Vi32;
    unsafe fn castpd_epi64(a: Self::Vf64) -> Self::Vi64;
    unsafe fn castepi32_ps(a: Self::Vi32) -> Self::Vf32;
    unsafe fn castepi64_pd(a: Self::Vi64) -> Self::Vf64;
    unsafe fn ceil_ps(a: Self::Vf32) -> Self::Vf32;
    unsafe fn ceil_pd(a: Self::Vf64) -> Self::Vf64;
    unsafe fn cmpeq_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn cmpneq_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn cmpge_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn cmpgt_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn cmple_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn cmplt_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn cmpeq_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn cmpneq_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn cmpge_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn cmpgt_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn cmple_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn cmplt_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn cvtepi32_ps(a: Self::Vi32) -> Self::Vf32;
    unsafe fn cvtps_epi32(a: Self::Vf32) -> Self::Vi32;
    unsafe fn floor_ps(a: Self::Vf32) -> Self::Vf32;
    unsafe fn floor_pd(a: Self::Vf64) -> Self::Vf64;
    /// When using Sse2, fastfloor uses a faster version of floor
    /// that only works on floating point values small enough to fit in
    /// an i32.  This is important for performance if you don't need
    /// a complete floor.
    unsafe fn fastfloor_ps(a: Self::Vf32) -> Self::Vf32;
    /// Actual FMA instructions will be used when Avx2 is used,
    /// otherwise a mul and add are used to replicate it, allowing you to
    /// just always use FMA in your code and get best perf in both cases.
    unsafe fn fmadd_ps(a: Self::Vf32, b: Self::Vf32, c: Self::Vf32) -> Self::Vf32;
    /// Actual FMA instructions will be used when Avx2 is used,
    /// otherwise a mul and add are used to replicate it, allowing you to
    /// just always use FMA in your code and get best perf in both cases.
    unsafe fn fnmadd_ps(a: Self::Vf32, b: Self::Vf32, c: Self::Vf32) -> Self::Vf32;
    /// Adds all lanes together. Distinct from h_add which adds pairs.
    unsafe fn horizontal_add_ps(a: Self::Vf32) -> f32;
    /// Adds all lanes together. Distinct from h_add which adds pairs.
    unsafe fn horizontal_add_pd(a: Self::Vf64) -> f64;
    /// Sse2 and Sse41 paths will simulate a gather by breaking out and
    /// doing scalar array accesses, because gather doesn't exist until Avx2.
    unsafe fn i32gather_epi32(arr: &[i32], index: Self::Vi32) -> Self::Vi32;
    /// Sse2 and Sse41 paths will simulate a gather by breaking out and
    /// doing scalar array accesses, because gather doesn't exist until Avx2.
    unsafe fn i32gather_ps(arr: &[f32], index: Self::Vi32) -> Self::Vf32;
    unsafe fn loadu_ps(a: &f32) -> Self::Vf32;
    unsafe fn loadu_pd(a: &f64) -> Self::Vf64;
    unsafe fn loadu_epi32(a: &i32) -> Self::Vi32;
    unsafe fn loadu_epi64(a: &i64) -> Self::Vi64;
    unsafe fn storeu_ps(a: &mut f32, b: Self::Vf32);
    unsafe fn storeu_pd(a: &mut f64, b: Self::Vf64);
    unsafe fn storeu_epi32(a: &mut i32, b: Self::Vi32);
    unsafe fn storeu_epi64(a: &mut i64, b: Self::Vi64);
    unsafe fn max_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn min_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn max_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn min_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn max_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64;
    unsafe fn min_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64;
    unsafe fn mul_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn mul_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64;
    /// Mullo is implemented for Sse2 by combining other Sse2 operations.
    unsafe fn mullo_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn not_epi32(a: Self::Vi32) -> Self::Vi32;
    unsafe fn or_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn or_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64;
    unsafe fn or_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn or_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64;
    unsafe fn rcp_ps(a: Self::Vf32) -> Self::Vf32;
    /// Round is implemented for Sse2 by combining other Sse2 operations.
    unsafe fn round_ps(a: Self::Vf32) -> Self::Vf32;
    unsafe fn round_pd(a: Self::Vf64) -> Self::Vf64;
    unsafe fn set1_epi32(a: i32) -> Self::Vi32;
    unsafe fn set1_ps(a: f32) -> Self::Vf32;
    unsafe fn set1_pd(a: f64) -> Self::Vf64;
    unsafe fn setzero_ps() -> Self::Vf32;
    unsafe fn setzero_pd() -> Self::Vf64;
    unsafe fn setzero_epi32() -> Self::Vi32;
    unsafe fn setzero_epi64() -> Self::Vi64;
    /// amt must be a constant
    unsafe fn srai_epi32(a: Self::Vi32, amt_const: i32) -> Self::Vi32;
    /// amt must be a constant
    unsafe fn srli_epi32(a: Self::Vi32, amt_const: i32) -> Self::Vi32;
    /// amt must be a constant
    unsafe fn slli_epi32(a: Self::Vi32, amt_const: i32) -> Self::Vi32;
    /// amt does not have to be a constant, but may be slower than the srai version
    unsafe fn sra_epi32(a: Self::Vi32, amt: i32) -> Self::Vi32;
    /// amt does not have to be a constant, but may be slower than the srli version
    unsafe fn srl_epi32(a: Self::Vi32, amt: i32) -> Self::Vi32;
    /// amt does not have to be a constant, but may be slower than the slli version
    unsafe fn sll_epi32(a: Self::Vi32, amt: i32) -> Self::Vi32;
    unsafe fn sub_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn sub_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn sqrt_ps(a: Self::Vf32) -> Self::Vf32;
    unsafe fn rsqrt_ps(a: Self::Vf32) -> Self::Vf32;
    unsafe fn sqrt_pd(a: Self::Vf64) -> Self::Vf64;
    unsafe fn rsqrt_pd(a: Self::Vf64) -> Self::Vf64;
    unsafe fn shuffle_epi32(a: Self::Vi32, imm8: i32) -> Self::Vi32;
    unsafe fn shuffle_ps(a: Self::Vf32, Self::Vf32, imm8: i32) -> Self::Vf32;
    unsafe fn xor_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn xor_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64;
    unsafe fn xor_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn xor_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64;
}

#[cfg(test)]
mod tests {

    use super::*;
    use avx2::*;
    use sse2::*;
    use sse41::*;
    use tests::std::vec::Vec;

    // If using runtime feature detection, you will want to be sure this inlines
    // so you can leverage target_feature attributes
    #[inline(always)]
    unsafe fn distance<S: Simd>(
        x1: &[f32], 
        y1: &[f32], 
        x2: &[f32], 
        y2: &[f32]) -> Vec<f32> {

        let mut result: Vec<f32> = Vec::with_capacity(x1.len());
        result.set_len(x1.len()); // for efficiency

        // Operations have to be done in terms of the vector width
        // so that it will work with any size vector.
        // the width of a vector type is provided as a constant
        // so the compiler is free to optimize it more.
        let mut i = 0;
        //S::VF32_WIDTH is a constant, 4 when using SSE, 8 when using AVX2, etc
        while i < x1.len() {
            //load data from your vec into a SIMD value
            let xv1 = S::loadu_ps(&x1[i]);
            let yv1 = S::loadu_ps(&y1[i]);
            let xv2 = S::loadu_ps(&x2[i]);
            let yv2 = S::loadu_ps(&y2[i]);

            // Use the usual intrinsic syntax if you prefer
            let mut xdiff = S::sub_ps(xv1, xv2);
            // Or use operater overloading if you like
            let mut ydiff = yv1 - yv2;
            xdiff *= xdiff;
            ydiff *= ydiff;
            let distance = S::sqrt_ps(xdiff + ydiff);
            // Store the SIMD value into the result vec
            S::storeu_ps(&mut result[i], distance);
            // Increment i by the vector width
            i += S::VF32_WIDTH
        }
        result
    }

    //Call distance as an SSE2 function
    #[target_feature(enable = "sse2")]
    unsafe fn distance_sse2(
        x1: &[f32], 
        y1: &[f32], 
        x2: &[f32], 
        y2: &[f32]) -> Vec<f32> {
        distance::<Sse2>(x1, y1, x2, y2)
    }
    //Call distance as an SSE41 function
    #[target_feature(enable = "sse4.1")]
    unsafe fn distance_sse41(
        x1: &[f32], 
        y1: &[f32], 
        x2: &[f32], 
        y2: &[f32]) -> Vec<f32> {
        distance::<Sse41>(x1, y1, x2, y2)
    }
    //Call distance as an AVX2 function
    #[target_feature(enable = "avx2")]
    unsafe fn distance_avx2(
        x1: &[f32], 
        y1: &[f32], 
        x2: &[f32], 
        y2: &[f32]) -> Vec<f32> {
        distance::<Avx2>(x1, y1, x2, y2)
    }

    #[test]
    fn distance_test() {
        unsafe {
            let x1 = [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
            let y1 = [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
            let x2 = [8.0f32, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];
            let y2 = [8.0f32, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];

            let distances_sse2 = distance_sse2(&x1, &y1, &x2, &y2);
            println!("sse2 dist:{:?}", distances_sse2);
            let distances_sse41 = distance_sse41(&x1, &y1, &x2, &y2);
            println!("sse41 dist:{:?}", distances_sse41);
            let distances_avx2 = distance_avx2(&x1, &y1, &x2, &y2);
            println!("avx2 dist:{:?}", distances_avx2);
            for i in 0..8 {
                assert_eq!(distances_sse2[i], distances_sse41[i]);
                assert_eq!(distances_sse41[i], distances_avx2[i]);
            }
        }
    }

    #[inline(always)]
    unsafe fn minmax_ints<S: Simd>() -> (i32, i32, i32, i32) {
        let mut t1 = S::setzero_epi32();
        let mut t2 = S::setzero_epi32();
        for i in 0..S::VI32_WIDTH {
            let ias32 = i as i32;
            t1[i] = ias32;
            if i % 2 == 0 {
                t2[i] = ias32 * 10
            } else {
                t2[i] = -ias32;
            }
        }
        let a = S::min_epi32(t1, t2)[0];
        let b = S::min_epi32(t2, t1)[1];
        let c = S::max_epi32(t1, t2)[2];
        let d = S::max_epi32(t2, t1)[3];
        (a, b, c, d)
    }

    #[target_feature(enable = "sse2")]
    unsafe fn minmax_ints_sse2() -> (i32, i32, i32, i32) {
        minmax_ints::<Sse2>()
    }
    #[target_feature(enable = "sse4.1")]
    unsafe fn minmax_ints_sse41() -> (i32, i32, i32, i32) {
        minmax_ints::<Sse41>()
    }
    #[target_feature(enable = "avx2")]
    unsafe fn minmax_ints_avx2() -> (i32, i32, i32, i32) {
        minmax_ints::<Avx2>()
    }
    #[test]
    fn testminmax_ints_consistency() {
        unsafe {
            assert_eq!(minmax_ints_sse2(), minmax_ints_sse41());
            assert_eq!(minmax_ints_sse41(), minmax_ints_avx2());
        }
    }

    // If using runtime feature detection, you will want to be sure this inlines
    #[inline(always)]
    unsafe fn sample<S: Simd>() -> i32 {
        let a = S::set1_epi32(3);
        let b = S::set1_epi32(-1);
        let c = a + b; //2
                       // let d = a + 2; // 4
        c[S::VF32_WIDTH - 1]
    }

    // Make an sse2 version of sample
    #[target_feature(enable = "sse2")]
    unsafe fn sample_sse2() -> i32 {
        sample::<Sse2>()
    }

    // Make an avx2 version of sample
    #[target_feature(enable = "avx2")]
    unsafe fn sample_avx2() -> i32 {
        sample::<Avx2>()
    }
    #[target_feature(enable = "sse4.1")]
    unsafe fn sample_sse41() -> i32 {
        sample::<Sse41>()
    }
    #[inline(always)]
    unsafe fn setlanetest<S: Simd>() -> f32 {
        let mut a = S::set1_ps(1.0);
        a[0] = 5.0;
        a[0]
    }
    unsafe fn setlanetest_avx2() -> f32 {
        setlanetest::<Avx2>()
    }

    #[inline(always)]
    unsafe fn gathertest_simd<S: Simd>() -> f32 {
        let a = [4.0, 3.0, 2.0, 1.0];
        let iarr = [0, 1, 2, 3];

        let index = S::loadu_epi32(&iarr[0]);
        let result = S::i32gather_ps(&a, index);
        result[0]
    }
    unsafe fn gathertest_sse2() -> f32 {
        gathertest_simd::<Sse2>()
    }

    #[inline(always)]
    unsafe fn overload_test<S: Simd>() -> i32 {
        let a = S::set1_epi32(3);
        let b = S::set1_epi32(2);
        let c = a + b; // 5
        let d = c * b; // 10
        let mut e = d - a; // 7
        e *= b; // 14
        let mut result = S::set1_epi32(9);
        result[0] = e[0];
        result[0]
    }
    unsafe fn overload_test_sse2() -> i32 {
        overload_test::<Sse2>()
    }

    #[test]
    fn overloads() {
        unsafe {
            assert_eq!(overload_test_sse2(), 14);
        }
    }
    #[inline(always)]
    unsafe fn overload_float_test<S: Simd>() -> f32 {
        let a = S::set1_ps(3.0);
        let b = S::set1_ps(2.0);
        let c = a + b; // 5
        let d = c * b; // 10
        let e = d - a; // 7
        let e = e / b; // 3.5
        let e = e * S::set1_ps(2.0); //7
        e[0]
    }
    unsafe fn overload_float_test_sse2() -> f32 {
        overload_float_test::<Sse2>()
    }

    #[test]
    fn overloads_float() {
        unsafe {
            assert_eq!(overload_float_test_sse2(), 7.0);
        }
    }
    #[test]
    fn consistency() {
        unsafe {
            assert_eq!(sample_sse2(), sample_sse41());
            assert_eq!(sample_sse41(), sample_avx2());
        }
    }
    #[test]
    fn setlane() {
        unsafe {
            assert_eq!(setlanetest_avx2(), 5.0);
        }
    }
    #[test]
    fn gathertest() {
        unsafe {
            assert_eq!(gathertest_sse2(), 4.0);
        }
    }
    #[test]
    fn overloadi32() {
        unsafe {
            assert_eq!(sample_sse2(), 2);
            assert_eq!(sample_sse41(), 2);
            assert_eq!(sample_avx2(), 2);
        }
    }
}
