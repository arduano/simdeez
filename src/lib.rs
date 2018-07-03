//! SIMDeez abstracts over the various sets of SIMD instructions such that
//! you can write a single function, and use it to produce SSE2,
//! SSE41, AVX2, or scalar versions of that function.  This can be combined
//! with `cfg` attributes to produce the optimum function at compile time,
//! or with `target_feature` attributes for use with runtime selection,
//! either automatically or letting users decide
//! ---
//! Support for more instructions sets such as AVX-512, and NEON can be
//! added as Rust adds support for those intrinsics.
//! ---
//! SIMDeez functions follow the naming conventions of the intel intrinsics
//! unless otherwise noted.
//! See the [Intel Intrinsics Guide](https://software.intel.com/sites/landingpage/IntrinsicsGuide/)
//! for documentation.
//! ---
//! SIMDeez is currently in an Alpha state, not all intrinsics are covered.
//! I will be slowly adding more as time and need permits. PRs are welcome, and
//! I would consider putting more time into the project with corporate sponsorship.
//!
//! # Examples
//! ```rust
//! use simdeez::*;
//! use simdeez::avx2::*;
//! use simdeez::sse2::*;
//! use simdeez::sse41::*;
//! use simdeez::scalar::*;
//! // If using runtime feature detection, you will want to be sure this inlines
//! #[inline(always)]
//! unsafe fn sample<S: Simd>() -> f32 {
//!     // function names mirror the intel intrinsics, minus the _mm_ part, call them as usual
//!     let a = S::set1_ps(1.5);
//!     let b = S::set1_ps(2.5);
//!     let mut c = S::add_ps(a,b);
//!     // Or you can use overloaded operators when applicable:
//!     let overloads = a*b+b-c/a;
//!     // If your SIMD instruction set doesn't have floor, round, gather etc,  SIMDeez handles it for you
//!     c = S::floor_ps(c);
//!     // You can get the width (as a const!)  of the instruction set you are working with
//!     let width = S::WIDTH_BYTES;    
//!     // And set or get individual lanes with ease using the index operator.
//!     let first = c[0];
//!     let last = c[S::VF32_WIDTH-1];
//!     first+last
//! }
//!
//! // Make an sse2 version of sample
//! #[target_feature(enable = "sse2")]
//! unsafe fn sample_sse2() -> f32 {
//!     sample::<Sse2>()
//! }
//! // Make an avx2 version of sample
//! #[target_feature(enable = "avx2")]
//! unsafe fn sample_avx2() -> f32 {
//!     sample::<Avx2>()
//! }
//! // An SSE4.1 version
//! #[target_feature(enable = "sse4.1")]
//! unsafe fn sample_sse41() -> f32 {
//!     sample::<Sse41>()
//! }
//! // Or even scalar (perf may suffer here)
//! unsafe fn sample_scalar() -> f32 {
//!     sample::<Scalar>()
//! }
//!
//!
//! ```
use std::fmt::Debug;
use std::ops::*;
#[macro_use]
mod macros;
pub mod avx2;
pub mod overloads;
pub mod scalar;
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
    type Vf64: Copy + Debug + Index<usize, Output = f64> + IndexMut<usize>
        + Add<Self::Vf64, Output = Self::Vf64>
        + Mul<Self::Vf64, Output = Self::Vf64>
        + AddAssign<Self::Vf64>;

    /// The width of the vector lane.  Necessary for creating
    /// lane width agnostic code.
    const VF32_WIDTH: usize;
    const VF64_WIDTH: usize;
    const VI32_WIDTH: usize;

    unsafe fn div_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    /// Equivalent to transmuting the SIMD type to an array and accessing
    /// it at the index i.
    unsafe fn abs_ps(a: Self::Vf32) -> Self::Vf32;
    unsafe fn add_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn add_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn add_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64;
    unsafe fn and_si(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn andnot_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn andnot_si(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    /// This is provided for convenience, it uses casts and the blendv_ps
    /// intrinsics to implement it.
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
    /// Sse2 and Sse41 paths will simulate a gather by breaking out and
    /// doing scalar array accesses, because gather doesn't exist until Avx2.
    unsafe fn i32gather_epi32(arr: &[i32], index: Self::Vi32) -> Self::Vi32;
    /// Sse2 and Sse41 paths will simulate a gather by breaking out and
    /// doing scalar array accesses, because gather doesn't exist until Avx2.
    unsafe fn i32gather_ps(arr: &[f32], index: Self::Vi32) -> Self::Vf32;
    unsafe fn loadu_ps(a: &f32) -> Self::Vf32;
    unsafe fn loadu_pd(a: &f64) -> Self::Vf64;
    unsafe fn loadu_si(a: &i32) -> Self::Vi32;
    unsafe fn storeu_ps(a: &mut f32, b: Self::Vf32);
    unsafe fn max_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn min_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn mul_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn mul_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64;
    /// Mullo is implemented for Sse2 by combining other Sse2 operations.
    unsafe fn mullo_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn or_si(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    /// Round is implemented for Sse2 by combining other Sse2 operations.
    unsafe fn round_ps(a: Self::Vf32) -> Self::Vf32;
    unsafe fn set1_epi32(a: i32) -> Self::Vi32;
    unsafe fn set1_ps(a: f32) -> Self::Vf32;
    unsafe fn set1_pd(a: f64) -> Self::Vf64;
    unsafe fn setzero_ps() -> Self::Vf32;
    unsafe fn setzero_pd() -> Self::Vf64;
    unsafe fn setzero_si() -> Self::Vi32;
    /// Shift operations for Sse require that imm8 be a constant,
    /// we handle this with a macro, so if you pass a non constant
    /// you will get a compile time error. This is necessary, the hardware
    /// demands it. With Avx2 you are free to pass anything.
    unsafe fn srai_epi32(a: Self::Vi32, imm8: i32) -> Self::Vi32;
    unsafe fn sub_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn sub_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn sqrt_ps(a: Self::Vf32) -> Self::Vf32;
    unsafe fn rsqrt_ps(a: Self::Vf32) -> Self::Vf32;
    unsafe fn xor_si(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
}

#[cfg(test)]
mod tests {
    use super::*;
    use avx2::*;
    use scalar::*;
    use sse2::*;
    use sse41::*;
    // If using runtime feature detection, you will want to be sure this inlines
    #[inline(always)]
    unsafe fn sample<S: Simd>() -> i32 {
        let a = S::set1_epi32(3);
        let b = S::set1_epi32(-1);
        let c = S::cmpgt_epi32(a, b);
        let c = c + b;
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
    unsafe fn sample_scalar() -> i32 {
        sample::<Scalar>()
    }

    #[inline(always)]
    unsafe fn setlanetest<S: Simd>() -> f32 {
        let mut a = S::set1_ps(1.0);
        a[0] = 5.0;
        a[0]
    }
    unsafe fn setlanetest_scalar() -> f32 {
        setlanetest::<Scalar>()
    }
    unsafe fn setlanetest_avx2() -> f32 {
        setlanetest::<Avx2>()
    }

    #[inline(always)]
    unsafe fn gathertest_simd<S: Simd>() -> f32 {
        let a = [4.0, 3.0, 2.0, 1.0];
        let iarr = [0, 1, 2, 3];

        let index = S::loadu_si(&iarr[0]);
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
        let e = e + 2.0; //7
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
            assert_eq!(sample_avx2(), sample_scalar());
        }
    }
    #[test]
    fn setlane() {
        unsafe {
            assert_eq!(setlanetest_avx2(), 5.0);
            assert_eq!(setlanetest_scalar(), 5.0);
        }
    }
    #[test]
    fn gathertest() {
        unsafe {
            assert_eq!(gathertest_sse2(), 4.0);
        }
    }
}
