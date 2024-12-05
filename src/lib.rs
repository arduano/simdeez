//! A library that abstracts over SIMD instruction sets, including ones with differing widths.
//! SIMDeez is designed to allow you to write a function one time and produce scalar, SSE2, SSE41, AVX2 and Neon versions of the function.
//! You can either have the version you want selected automatically at runtime, at compiletime, or
//! select yourself by hand.
//!
//! SIMDeez is currently in Beta, if there are intrinsics you need that are not currently implemented, create an issue
//! and I'll add them. PRs to add more intrinsics are welcome. Currently things are well fleshed out for i32, i64, f32, and f64 types.
//!
//! As Rust stabilizes support for AVX-512 I plan to add those as well.
//!
//! Refer to the excellent [Intel Intrinsics Guide](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#) for documentation on these functions.
//!
//! # Features
//!
//! * SSE2, SSE41, AVX2, Neon and scalar fallback
//! * Can be used with compile time or run time selection
//! * No runtime overhead
//! * Uses familiar intel intrinsic naming conventions, easy to port.
//!   * `_mm_add_ps(a,b)` becomes `add_ps(a,b)`
//! * Fills in missing intrinsics in older APIs with fast SIMD workarounds.
//!   * ceil, floor, round,blend, etc
//! * Can be used by `#[no_std]` projects
//! * Operator overloading: `let sum = va + vb` or `s *= s`
//! * Extract or set a single lane with the index operator: `let v1 = v[1];`
//!
//! # Trig Functions via Sleef-sys
//! A number of trigonometric and other common math functions are provided
//! in vectorized form via the Sleef-sys crate. This is an optional feature `sleef` that you can enable.
//! Doing so currently requires nightly, as well as having CMake and Clang installed.
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
//!use simdeez::{prelude::*, simd_runtime_generate};
//!
//! // If you want your SIMD function to use use runtime feature detection to call
//!// the fastest available version, use the simd_runtime_generate macro:
//!simd_runtime_generate!(
//!    fn distance(x1: &[f32], y1: &[f32], x2: &[f32], y2: &[f32]) -> Vec<f32> {
//!        let mut result: Vec<f32> = Vec::with_capacity(x1.len());
//!        result.set_len(x1.len()); // for efficiency
//!
//!        // Set each slice to the same length for iteration efficiency
//!        let mut x1 = &x1[..x1.len()];
//!        let mut y1 = &y1[..x1.len()];
//!        let mut x2 = &x2[..x1.len()];
//!        let mut y2 = &y2[..x1.len()];
//!        let mut res = &mut result[..x1.len()];
//!
//!        // Operations have to be done in terms of the vector width
//!        // so that it will work with any size vector.
//!        // the width of a vector type is provided as a constant
//!        // so the compiler is free to optimize it more.
//!        // Vf32::WIDTH is a constant, 4 when using SSE, 8 when using AVX2, etc
//!        while x1.len() >= S::Vf32::WIDTH {
//!            //load data from your vec into an SIMD value
//!            let xv1 = S::Vf32::load_from_slice(&x1);
//!            let yv1 = S::Vf32::load_from_slice(&y1);
//!            let xv2 = S::Vf32::load_from_slice(&x2);
//!            let yv2 = S::Vf32::load_from_slice(&y2);
//!
//!            // Use the usual intrinsic syntax if you prefer
//!            let mut xdiff = xv1 - xv2;
//!            // Or use operater overloading if you like
//!            let mut ydiff = yv1 - yv2;
//!            xdiff *= xdiff;
//!            ydiff *= ydiff;
//!            let distance = (xdiff + ydiff).sqrt();
//!            // Store the SIMD value into the result vec
//!            distance.copy_to_slice(res);
//!
//!            // Move each slice to the next position
//!            x1 = &x1[S::Vf32::WIDTH..];
//!            y1 = &y1[S::Vf32::WIDTH..];
//!            x2 = &x2[S::Vf32::WIDTH..];
//!            y2 = &y2[S::Vf32::WIDTH..];
//!            res = &mut res[S::Vf32::WIDTH..];
//!        }
//!
//!        // (Optional) Compute the remaining elements. Not necessary if you are sure the length
//!        // of your data is always a multiple of the maximum S::Vf32_WIDTH you compile for (4 for SSE, 8 for AVX2, etc).
//!        // This can be asserted by putting `assert_eq!(x1.len(), 0);` here
//!        for i in 0..x1.len() {
//!            let mut xdiff = x1[i] - x2[i];
//!            let mut ydiff = y1[i] - y2[i];
//!            xdiff *= xdiff;
//!            ydiff *= ydiff;
//!            let distance = (xdiff + ydiff).sqrt();
//!            res[i] = distance;
//!        }
//!
//!        result
//!    }
//!);
//!
//!const SIZE: usize = 200;
//!
//!fn main() {
//!    let raw = (0..4)
//!        .map(|i| (0..SIZE).map(|j| (i*j) as f32).collect::<Vec<f32>>())
//!        .collect::<Vec<Vec<f32>>>();
//!
//!    let distances = distance(
//!        raw[0].as_slice(),
//!        raw[1].as_slice(),
//!        raw[2].as_slice(),
//!        raw[3].as_slice(),
//!    );
//!    assert_eq!(distances.len(), SIZE);
//!    dbg!(distances);
//!}
//! ```
//!
//! This will generate 5 functions for you:
//! * `distance<S:Simd>` the generic version of your function
//! * `distance_scalar`  a scalar fallback
//! * `distance_sse2`    SSE2 version
//! * `distance_sse41`   SSE41 version
//! * `distance_avx2`    AVX2 version
//! * `distance_neon`    Neon version
//! * `distance_runtime_select`  picks the fastest of the above at runtime
//!
//! You can use any of these you wish, though typically you would use the runtime_select version
//! unless you want to force an older instruction set to avoid throttling or for other arcane
//! reasons.
//!
//! Optionally you can use the `simd_compiletime_generate!` macro in the same way.  This will
//! produce 2 active functions via the `cfg` attribute feature:
//!
//! * `distance<S:Simd>`      the generic version of your function
//! * `distance_compiletime`  the fastest instruction set availble for the given compile time
//!   feature set
//!
//! You may also forgo the macros if you know what you are doing, just keep in mind there are lots
//! of arcane subtleties with inlining and target_features that must be managed. See how the macros
//! expand for more detail.
#![allow(clippy::missing_safety_doc)] // TODO: Work on the safety of functions
#![cfg_attr(all(feature = "no_std", not(test)), no_std)]
#[macro_use]
#[cfg(test)]
extern crate std;
pub extern crate paste;

#[cfg(test)]
mod tests;

mod ops;

pub mod prelude;

use core::ops::*;

mod invoking;

#[macro_use]
mod overloads;

mod base;
pub use base::*;

mod libm_ext;

mod engines;

pub use engines::scalar;

/// The abstract SIMD trait which is implemented by Avx2, Sse41, etc
pub trait Simd: 'static + Sync + Send {
    /// Vector of i8s.  Corresponds to __m128i when used
    /// with the Sse impl, __m256i when used with Avx2, or a single i8
    /// when used with Scalar.
    type Vi8: SimdInt8<Scalar = i8> + SimdBaseIo;

    /// Vector of i16s.  Corresponds to __m128i when used
    /// with the Sse impl, __m256i when used with Avx2, or a single i16
    /// when used with Scalar.
    type Vi16: SimdInt16<Scalar = i16> + SimdBaseIo;

    /// Vector of i32s.  Corresponds to __m128i when used
    /// with the Sse impl, __m256i when used with Avx2, or a single i32
    /// when used with Scalar.
    type Vi32: SimdInt32<Engine = Self, Scalar = i32> + SimdBaseIo;

    /// Vector of i64s.  Corresponds to __m128i when used
    /// with the Sse impl, __m256i when used with Avx2, or a single i64
    /// when used with Scalar.
    type Vi64: SimdInt64<Engine = Self, Scalar = i64> + SimdBaseIo;

    /// Vector of f32s.  Corresponds to __m128 when used
    /// with the Sse impl, __m256 when used with Avx2, or a single f32
    /// when used with Scalar.
    type Vf32: SimdFloat32<Engine = Self, Scalar = f32> + SimdBaseIo;

    /// Vector of f64s.  Corresponds to __m128d when used
    /// with the Sse impl, __m256d when used with Avx2, or a single f64
    /// when used with Scalar.
    type Vf64: SimdFloat64<Engine = Self, Scalar = f64> + SimdBaseIo;

    // The width of the vector lane.  Necessary for creating
    // lane width agnostic code.
    #[deprecated(note = "The VF32_WIDTH is deprecated, please use the Vf32::WIDTH instead.")]
    const VF32_WIDTH: usize = Self::Vf32::WIDTH;
    #[deprecated(note = "The VF64_WIDTH is deprecated, please use the Vf64::WIDTH instead.")]
    const VF64_WIDTH: usize = Self::Vf64::WIDTH;
    #[deprecated(note = "The VI16_WIDTH is deprecated, please use the Vi16::WIDTH instead.")]
    const VI16_WIDTH: usize = Self::Vi16::WIDTH;
    #[deprecated(note = "The VI32_WIDTH is deprecated, please use the Vi32::WIDTH instead.")]
    const VI32_WIDTH: usize = Self::Vi32::WIDTH;
    #[deprecated(note = "The VI64_WIDTH is deprecated, please use the Vi64::WIDTH instead.")]
    const VI64_WIDTH: usize = Self::Vi64::WIDTH;

    fn invoke<R>(f: impl FnOnce() -> R) -> R;

    #[inline(always)]
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn mul_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        a * b
    }
    #[inline(always)]
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn mul_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        a * b
    }
    #[inline(always)]
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn not_epi32(a: Self::Vi32) -> Self::Vi32 {
        !a
    }
    #[inline(always)]
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn not_epi64(a: Self::Vi64) -> Self::Vi64 {
        !a
    }
    #[inline(always)]
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn or_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        a | b
    }
    #[inline(always)]
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn or_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        a | b
    }
    #[inline(always)]
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn or_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        a | b
    }
    #[inline(always)]
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn or_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        a | b
    }
    #[inline(always)]
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn xor_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        a ^ b
    }
    #[inline(always)]
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn xor_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        a ^ b
    }
    #[inline(always)]
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn xor_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        a ^ b
    }
    #[inline(always)]
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn xor_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        a ^ b
    }
    /// amt must be a constant
    #[inline(always)]
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn slli_epi32(a: Self::Vi32, amt_const: i32) -> Self::Vi32 {
        a << amt_const
    }
    /// amt must be a constant
    #[inline(always)]
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn srai_epi32(a: Self::Vi32, amt_const: i32) -> Self::Vi32 {
        a >> amt_const
    }
    #[inline(always)]
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn div_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        a / b
    }
    #[inline(always)]
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn div_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        a / b
    }
    #[inline(always)]
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn add_epi16(a: Self::Vi16, b: Self::Vi16) -> Self::Vi16 {
        a + b
    }
    #[inline(always)]
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn sub_epi16(a: Self::Vi16, b: Self::Vi16) -> Self::Vi16 {
        a - b
    }
    #[inline(always)]
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn add_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        a + b
    }
    #[inline(always)]
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn add_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        a + b
    }
    #[inline(always)]
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn add_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        a + b
    }
    #[inline(always)]
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn add_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        a + b
    }
    #[inline(always)]
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn and_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        a & b
    }
    #[inline(always)]
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn and_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        a & b
    }
    #[inline(always)]
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn and_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        a & b
    }
    #[inline(always)]
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn and_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        a & b
    }

    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn abs_ps(a: Self::Vf32) -> Self::Vf32 {
        a.abs()
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn abs_pd(a: Self::Vf64) -> Self::Vf64 {
        a.abs()
    }

    // Mullo is implemented for Sse2 by combining other Sse2 operations.

    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn mullo_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        a * b
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn mullo_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        a * b
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn mullo_epi16(a: Self::Vi16, b: Self::Vi16) -> Self::Vi16 {
        a * b
    }

    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn andnot_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        b.and_not(a)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn andnot_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        b.and_not(a)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn andnot_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        b.and_not(a)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn andnot_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        b.and_not(a)
    }

    /// Note SSE2 will select B only when all bits are 1, while SSE41 and AVX2 only
    /// check the high bit. To maintain portability ensure all bits are 1 when using
    /// blend. Results of comparison operations adhere to this.
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn blendv_epi32(a: Self::Vi32, b: Self::Vi32, mask: Self::Vi32) -> Self::Vi32 {
        a.blendv(b, mask)
    }
    /// Note SSE2 will select B only when all bits are 1, while SSE41 and AVX2 only
    /// check the high bit. To maintain portability ensure all bits are 1 when using
    /// blend. Results of comparison operations adhere to this.
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn blendv_epi64(a: Self::Vi64, b: Self::Vi64, mask: Self::Vi64) -> Self::Vi64 {
        a.blendv(b, mask)
    }
    /// Note SSE2 will select B only when all bits are 1, while SSE41 and AVX2 only
    /// check the high bit. To maintain portability ensure all bits are 1 when using
    /// blend. Results of comparison operations adhere to this.
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn blendv_ps(a: Self::Vf32, b: Self::Vf32, mask: Self::Vf32) -> Self::Vf32 {
        a.blendv(b, mask)
    }
    /// Note SSE2 will select B only when all bits are 1, while SSE41 and AVX2 only
    /// check the high bit. To maintain portability ensure all bits are 1 when using
    /// blend. Results of comparison operations adhere to this.
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn blendv_pd(a: Self::Vf64, b: Self::Vf64, mask: Self::Vf64) -> Self::Vf64 {
        a.blendv(b, mask)
    }

    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn castps_epi32(a: Self::Vf32) -> Self::Vi32 {
        a.bitcast_i32()
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn castpd_epi64(a: Self::Vf64) -> Self::Vi64 {
        a.bitcast_i64()
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn castepi32_ps(a: Self::Vi32) -> Self::Vf32 {
        a.bitcast_f32()
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn castepi64_pd(a: Self::Vi64) -> Self::Vf64 {
        a.bitcast_f64()
    }

    /// Converts the type of a f32 vector to a f64 vector without changing the underlying bits.
    #[deprecated(
        note = "These functions have unpredictable behavior and will be deleted in the future. Please use a manual implementation instead."
    )]
    unsafe fn castps_pd(_a: Self::Vf32) -> Self::Vf64 {
        panic!("Deprecated")
    }
    /// Converts the type of a f64 vector to a f32 vector without changing the underlying bits.
    #[deprecated(
        note = "These functions have unpredictable behavior and will be deleted in the future. Please use a manual implementation instead."
    )]
    unsafe fn castpd_ps(_a: Self::Vf64) -> Self::Vf32 {
        panic!("Deprecated")
    }

    /// Currently scalar will have different results in some cases depending on the
    /// current SSE rounding mode.
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn cvtps_epi32(a: Self::Vf32) -> Self::Vi32 {
        a.cast_i32()
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn cvtpd_epi64(a: Self::Vf64) -> Self::Vi64 {
        a.cast_i64()
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn cvtepi32_ps(a: Self::Vi32) -> Self::Vf32 {
        a.cast_f32()
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn cvtepi64_pd(a: Self::Vi64) -> Self::Vf64 {
        a.cast_f64()
    }

    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn cmpeq_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        a.cmp_eq(b)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn cmpneq_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        a.cmp_neq(b)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn cmpge_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        a.cmp_gte(b)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn cmpgt_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        a.cmp_gt(b)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn cmple_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        a.cmp_lte(b)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn cmplt_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        a.cmp_lt(b)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn cmpeq_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        a.cmp_eq(b)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn cmpneq_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        a.cmp_neq(b)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn cmpge_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        a.cmp_gte(b)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn cmpgt_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        a.cmp_gt(b)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn cmple_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        a.cmp_lte(b)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn cmplt_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        a.cmp_lt(b)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn cmpeq_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        a.cmp_eq(b)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn cmpneq_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        a.cmp_neq(b)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn cmpge_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        a.cmp_gte(b)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn cmpgt_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        a.cmp_gt(b)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn cmple_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        a.cmp_lte(b)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn cmplt_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        a.cmp_lt(b)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn cmpeq_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        a.cmp_eq(b)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn cmpneq_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        a.cmp_neq(b)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn cmpge_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        a.cmp_gte(b)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn cmpgt_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        a.cmp_gt(b)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn cmple_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        a.cmp_lte(b)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn cmplt_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        a.cmp_lt(b)
    }

    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn ceil_ps(a: Self::Vf32) -> Self::Vf32 {
        a.ceil()
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn ceil_pd(a: Self::Vf64) -> Self::Vf64 {
        a.ceil()
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn floor_ps(a: Self::Vf32) -> Self::Vf32 {
        a.floor()
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn floor_pd(a: Self::Vf64) -> Self::Vf64 {
        a.floor()
    }
    /// When using Sse2, fastround uses a faster version of floor
    /// that only works on floating point values small enough to fit in
    /// an i32.  This is a big performance boost if you don't need
    /// a complete floor.
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn fast_round_ps(a: Self::Vf32) -> Self::Vf32 {
        a.fast_round()
    }
    /// When using Sse2, fastceil uses a faster version of floor
    /// that only works on floating point values small enough to fit in
    /// an i32.  This is a big performance boost if you don't need
    /// a complete floor.
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn fast_ceil_ps(a: Self::Vf32) -> Self::Vf32 {
        a.fast_ceil()
    }
    /// When using Sse2, fastfloor uses a faster version of floor
    /// that only works on floating point values small enough to fit in
    /// an i32.  This is a big performance boost if you don't need
    /// a complete floor.
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn fast_floor_ps(a: Self::Vf32) -> Self::Vf32 {
        a.fast_floor()
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn fast_floor_pd(a: Self::Vf64) -> Self::Vf64 {
        a.fast_floor()
    }
    /// Actual FMA instructions will be used when Avx2 is used,
    /// otherwise a mul and add are used to replicate it, allowing you to
    /// just always use FMA in your code and get best perf in both cases.
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn fmadd_ps(a: Self::Vf32, b: Self::Vf32, c: Self::Vf32) -> Self::Vf32 {
        a.mul_add(b, c)
    }
    /// Actual FMA instructions will be used when Avx2 is used,
    /// otherwise a mul and add are used to replicate it, allowing you to
    /// just always use FMA in your code and get best perf in both cases.
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn fnmadd_ps(a: Self::Vf32, b: Self::Vf32, c: Self::Vf32) -> Self::Vf32 {
        a.neg_mul_add(b, c)
    }
    /// Actual FMA instructions will be used when Avx2 is used,
    /// otherwise a mul and add are used to replicate it, allowing you to
    /// just always use FMA in your code and get best perf in both cases.
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn fmadd_pd(a: Self::Vf64, b: Self::Vf64, c: Self::Vf64) -> Self::Vf64 {
        a.mul_add(b, c)
    }
    /// Actual FMA instructions will be used when Avx2 is used,
    /// otherwise a mul and add are used to replicate it, allowing you to
    /// just always use FMA in your code and get best perf in both cases.
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn fnmadd_pd(a: Self::Vf64, b: Self::Vf64, c: Self::Vf64) -> Self::Vf64 {
        a.neg_mul_add(b, c)
    }
    /// Actual FMA instructions will be used when Avx2 is used,
    /// otherwise a mul and sub are used to replicate it, allowing you to
    /// just always use FMA in your code and get best perf in both cases.
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn fmsub_ps(a: Self::Vf32, b: Self::Vf32, c: Self::Vf32) -> Self::Vf32 {
        a.neg_mul_sub(b, c)
    }
    /// Actual FMA instructions will be used when Avx2 is used,
    /// otherwise a mul and sub are used to replicate it, allowing you to
    /// just always use FMA in your code and get best perf in both cases.
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn fnmsub_ps(a: Self::Vf32, b: Self::Vf32, c: Self::Vf32) -> Self::Vf32 {
        a.mul_sub(b, c)
    }
    /// Actual FMA instructions will be used when Avx2 is used,
    /// otherwise a mul and sub are used to replicate it, allowing you to
    /// just always use FMA in your code and get best perf in both cases.
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn fmsub_pd(a: Self::Vf64, b: Self::Vf64, c: Self::Vf64) -> Self::Vf64 {
        a.neg_mul_sub(b, c)
    }
    /// Actual FMA instructions will be used when Avx2 is used,
    /// otherwise a mul and sub are used to replicate it, allowing you to
    /// just always use FMA in your code and get best perf in both cases.
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn fnmsub_pd(a: Self::Vf64, b: Self::Vf64, c: Self::Vf64) -> Self::Vf64 {
        a.mul_sub(b, c)
    }
    /// Adds all lanes together. Distinct from h_add which adds pairs.
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn horizontal_add_ps(a: Self::Vf32) -> f32 {
        a.horizontal_add()
    }
    /// Adds all lanes together. Distinct from h_add which adds pairs.
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn horizontal_add_pd(a: Self::Vf64) -> f64 {
        a.horizontal_add()
    }
    /// Sse2 and Sse41 paths will simulate a gather by breaking out and
    /// doing scalar array accesses, because gather doesn't exist until Avx2.
    #[deprecated(
        note = "These functions have unpredictable behavior and will be deleted in the future. Please use a manual implementation instead."
    )]
    unsafe fn i32gather_epi32(_arr: &[i32], _index: Self::Vi32) -> Self::Vi32 {
        panic!("Deprecated")
    }
    #[deprecated(
        note = "These functions have unpredictable behavior and will be deleted in the future. Please use a manual implementation instead."
    )]
    unsafe fn i64gather_epi64(_arr: &[i64], _index: Self::Vi64) -> Self::Vi64 {
        panic!("Deprecated")
    }
    /// Sse2 and Sse41 paths will simulate a gather by breaking out and
    /// doing scalar array accesses, because gather doesn't exist until Avx2.
    #[deprecated(
        note = "These functions have unpredictable behavior and will be deleted in the future. Please use a manual implementation instead."
    )]
    unsafe fn i32gather_ps(_arr: &[f32], _index: Self::Vi32) -> Self::Vf32 {
        panic!("Deprecated")
    }

    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn load_ps(a: &f32) -> Self::Vf32 {
        SimdBaseIo::load_from_ptr_aligned(a)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn load_pd(a: &f64) -> Self::Vf64 {
        SimdBaseIo::load_from_ptr_aligned(a)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn load_epi16(a: &i16) -> Self::Vi16 {
        SimdBaseIo::load_from_ptr_aligned(a)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn load_epi32(a: &i32) -> Self::Vi32 {
        SimdBaseIo::load_from_ptr_aligned(a)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn load_epi64(a: &i64) -> Self::Vi64 {
        SimdBaseIo::load_from_ptr_aligned(a)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn loadu_ps(a: &f32) -> Self::Vf32 {
        SimdBaseIo::load_from_ptr_unaligned(a)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn loadu_pd(a: &f64) -> Self::Vf64 {
        SimdBaseIo::load_from_ptr_unaligned(a)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn loadu_epi32(a: &i32) -> Self::Vi32 {
        SimdBaseIo::load_from_ptr_unaligned(a)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn loadu_epi64(a: &i64) -> Self::Vi64 {
        SimdBaseIo::load_from_ptr_unaligned(a)
    }

    /// Note, SSE2 and SSE4 will load when mask\[i\] is nonzero, where AVX2
    /// will store only when the high bit is set. To ensure portability
    /// ensure that the high bit is set.
    #[deprecated(
        note = "These functions have unpredictable behavior and will be deleted in the future. Please use a manual implementation instead."
    )]
    unsafe fn maskload_epi32(_mem_addr: &i32, _mask: Self::Vi32) -> Self::Vi32 {
        panic!("Deprecated")
    }
    /// Note, SSE2 and SSE4 will load when mask\[i\] is nonzero, where AVX2
    /// will store only when the high bit is set. To ensure portability
    /// ensure that the high bit is set.
    #[deprecated(
        note = "These functions have unpredictable behavior and will be deleted in the future. Please use a manual implementation instead."
    )]
    unsafe fn maskload_epi64(_mem_addr: &i64, _mask: Self::Vi64) -> Self::Vi64 {
        panic!("Deprecated")
    }
    /// Note, SSE2 and SSE4 will load when mask\[i\] is nonzero, where AVX2
    /// will store only when the high bit is set. To ensure portability
    /// ensure that the high bit is set.
    #[deprecated(
        note = "These functions have unpredictable behavior and will be deleted in the future. Please use a manual implementation instead."
    )]
    unsafe fn maskload_ps(_mem_addr: &f32, _mask: Self::Vi32) -> Self::Vf32 {
        panic!("Deprecated")
    }
    /// Note, SSE2 and SSE4 will load when mask\[i\] is nonzero, where AVX2
    /// will store only when the high bit is set. To ensure portability
    /// ensure that the high bit is set.
    #[deprecated(
        note = "These functions have unpredictable behavior and will be deleted in the future. Please use a manual implementation instead."
    )]
    unsafe fn maskload_pd(_mem_addr: &f64, _mask: Self::Vi64) -> Self::Vf64 {
        panic!("Deprecated")
    }

    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn store_ps(mem_addr: &mut f32, a: Self::Vf32) {
        SimdBaseIo::copy_to_ptr_aligned(a, mem_addr)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn store_pd(mem_addr: &mut f64, a: Self::Vf64) {
        SimdBaseIo::copy_to_ptr_aligned(a, mem_addr)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn store_epi32(mem_addr: &mut i32, a: Self::Vi32) {
        SimdBaseIo::copy_to_ptr_aligned(a, mem_addr)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn store_epi64(mem_addr: &mut i64, a: Self::Vi64) {
        SimdBaseIo::copy_to_ptr_aligned(a, mem_addr)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn storeu_ps(mem_addr: &mut f32, a: Self::Vf32) {
        SimdBaseIo::copy_to_ptr_unaligned(a, mem_addr)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn storeu_pd(mem_addr: &mut f64, a: Self::Vf64) {
        SimdBaseIo::copy_to_ptr_unaligned(a, mem_addr)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn storeu_epi32(mem_addr: &mut i32, a: Self::Vi32) {
        SimdBaseIo::copy_to_ptr_unaligned(a, mem_addr)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn storeu_epi64(mem_addr: &mut i64, a: Self::Vi64) {
        SimdBaseIo::copy_to_ptr_unaligned(a, mem_addr)
    }

    /// Note, SSE2 and SSE4 will store when mask\[i\] is nonzero, where AVX2
    /// will store only when the high bit is set. To ensure portability ensure the
    /// high bit is set.
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn maskstore_epi32(mem_addr: &mut i32, mask: Self::Vi32, a: Self::Vi32) {
        if mask[0] != 0 {
            *mem_addr = a[0];
        }
    }
    /// Note, SSE2 and SSE4 will store when mask\[i\] is nonzero, where AVX2
    /// will store only when the high bit is set. To ensure portability ensure the
    /// high bit is set.
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn maskstore_epi64(mem_addr: &mut i64, mask: Self::Vi64, a: Self::Vi64) {
        if mask[0] != 0 {
            *mem_addr = a[0];
        }
    }
    /// Note, SSE2 and SSE4 will store when mask\[i\] is nonzero, where AVX2
    /// will store only when the high bit is set. To ensure portability ensure the
    /// high bit is set.
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn maskstore_ps(mem_addr: &mut f32, mask: Self::Vi32, a: Self::Vf32) {
        if mask[0] != 0 {
            *mem_addr = a[0];
        }
    }
    /// Note, SSE2 and SSE4 will store when mask\[i\] is nonzero, where AVX2
    /// will store only when the high bit is set. To ensure portability ensure the
    /// high bit is set.
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn maskstore_pd(mem_addr: &mut f64, mask: Self::Vi64, a: Self::Vf64) {
        if mask[0] != 0 {
            *mem_addr = a[0];
        }
    }

    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn max_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        a.max(b)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn min_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        a.min(b)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn max_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        a.max(b)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn min_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        a.min(b)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn max_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        a.max(b)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn min_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        a.min(b)
    }

    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn rcp_ps(a: Self::Vf32) -> Self::Vf32 {
        a.fast_inverse()
    }
    /// Round is implemented for Sse2 by combining other Sse2 operations.
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn round_ps(a: Self::Vf32) -> Self::Vf32 {
        a.round()
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn round_pd(a: Self::Vf64) -> Self::Vf64 {
        a.round()
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn set1_epi32(a: i32) -> Self::Vi32 {
        SimdBaseIo::set1(a)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn set1_epi64(a: i64) -> Self::Vi64 {
        SimdBaseIo::set1(a)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn set1_ps(a: f32) -> Self::Vf32 {
        SimdBaseIo::set1(a)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn set1_pd(a: f64) -> Self::Vf64 {
        SimdBaseIo::set1(a)
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn setzero_ps() -> Self::Vf32 {
        SimdBaseIo::zeroes()
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn setzero_pd() -> Self::Vf64 {
        SimdBaseIo::zeroes()
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn setzero_epi32() -> Self::Vi32 {
        SimdBaseIo::zeroes()
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn setzero_epi64() -> Self::Vi64 {
        SimdBaseIo::zeroes()
    }

    /// amt must be a constant
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn srai_epi64(a: Self::Vi64, amt_const: i32) -> Self::Vi64 {
        let shifted = a >> amt_const;
        let ones: Self::Vi64 = SimdBaseIo::set1(i64::MAX);
        let mask = ones << (64 - amt_const);
        shifted ^ mask
    }
    /// amt must be a constant
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn srli_epi32(a: Self::Vi32, amt_const: i32) -> Self::Vi32 {
        a >> amt_const
    }

    /// amt does not have to be a constant, but may be slower than the srai version
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn sra_epi32(a: Self::Vi32, amt: i32) -> Self::Vi32 {
        let shifted = a >> amt;
        let ones: Self::Vi32 = SimdBaseIo::set1(i32::MAX);
        let mask = ones << (32 - amt);
        shifted ^ mask
    }

    /// amt does not have to be a constant, but may be slower than the srli version
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn srl_epi32(a: Self::Vi32, amt: i32) -> Self::Vi32 {
        a >> amt
    }
    /// amt does not have to be a constant, but may be slower than the slli version
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn sll_epi32(a: Self::Vi32, amt: i32) -> Self::Vi32 {
        a << amt
    }

    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn sub_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32 {
        a - b
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn sub_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64 {
        a - b
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn sub_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32 {
        a - b
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn sub_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64 {
        a - b
    }

    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn sqrt_ps(a: Self::Vf32) -> Self::Vf32 {
        a.sqrt()
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn rsqrt_ps(a: Self::Vf32) -> Self::Vf32 {
        a.rsqrt()
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn sqrt_pd(a: Self::Vf64) -> Self::Vf64 {
        a.sqrt()
    }
    #[deprecated(
        note = "Functions on the Simd trait are deprecated, please use the functions on the Vf32, Vf64, Vi16, Vi32, and Vi64 types instead."
    )]
    unsafe fn rsqrt_pd(a: Self::Vf64) -> Self::Vf64 {
        a.rsqrt()
    }

    /// Using the shuffle function is undefined behavior because imm8 behaves differently on different
    /// architectures.
    #[deprecated(
        note = "These functions have unpredictable behavior and will be deleted in the future. Please use a manual implementation instead."
    )]
    unsafe fn shuffle_epi32<const IMM8: i32>(_a: Self::Vi32) -> Self::Vi32 {
        panic!("Deprecated")
    }

    cfg_if::cfg_if! {
        if #[cfg(feature = "sleef")] {
            unsafe fn sin_ps(a: Self::Vf32) -> Self::Vf32;
            unsafe fn fast_sin_ps(a: Self::Vf32) -> Self::Vf32;
            unsafe fn cos_ps(a: Self::Vf32) -> Self::Vf32;
            unsafe fn fast_cos_ps(a: Self::Vf32) -> Self::Vf32;
            unsafe fn asin_ps(a: Self::Vf32) -> Self::Vf32;
            unsafe fn fast_asin_ps(a: Self::Vf32) -> Self::Vf32;
            unsafe fn acos_ps(a: Self::Vf32) -> Self::Vf32;
            unsafe fn fast_acos_ps(a: Self::Vf32) -> Self::Vf32;
            unsafe fn tan_ps(a: Self::Vf32) -> Self::Vf32;
            unsafe fn fast_tan_ps(a: Self::Vf32) -> Self::Vf32;
            unsafe fn atan_ps(a: Self::Vf32) -> Self::Vf32;
            unsafe fn fast_atan_ps(a: Self::Vf32) -> Self::Vf32;

            //hyperbolic
            unsafe fn sinh_ps(a: Self::Vf32) -> Self::Vf32;
            unsafe fn fast_sinh_ps(a: Self::Vf32) -> Self::Vf32;
            unsafe fn cosh_ps(a: Self::Vf32) -> Self::Vf32;
            unsafe fn fast_cosh_ps(a: Self::Vf32) -> Self::Vf32;
            unsafe fn asinh_ps(a: Self::Vf32) -> Self::Vf32;
            unsafe fn acosh_ps(a: Self::Vf32) -> Self::Vf32;
            unsafe fn tanh_ps(a: Self::Vf32) -> Self::Vf32;
            unsafe fn fast_tanh_ps(a: Self::Vf32) -> Self::Vf32;
            unsafe fn atanh_ps(a: Self::Vf32) -> Self::Vf32;

            unsafe fn atan2_ps(a: Self::Vf32,b: Self::Vf32) -> Self::Vf32;
            unsafe fn fast_atan2_ps(a: Self::Vf32,b: Self::Vf32) -> Self::Vf32;
            unsafe fn ln_ps(a:Self::Vf32) -> Self::Vf32;
            unsafe fn fast_ln_ps(a:Self::Vf32) -> Self::Vf32;
            unsafe fn log2_ps(a:Self::Vf32) -> Self::Vf32;
            unsafe fn log10_ps(a:Self::Vf32) -> Self::Vf32;
            unsafe fn hypot_ps(a:Self::Vf32,b:Self::Vf32) -> Self::Vf32;
            unsafe fn fast_hypot_ps(a:Self::Vf32,b:Self::Vf32) -> Self::Vf32;

            unsafe fn fmod_ps(a:Self::Vf32,b:Self::Vf32) -> Self::Vf32;
        }
    }
}
