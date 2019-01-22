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
pub mod libm;
pub mod overloads;
pub mod scalar;
pub mod sse2;
pub mod sse41;

/// Grouping all the constraints shared by associated types in
/// the Simd trait into this marker trait drastically reduces
/// compile time.
pub trait SimdBase<T,U> : 
      Copy
    + Debug
    + IndexMut<usize>
    + Add<T, Output = T>
    + Sub<T, Output = T>
    + AddAssign<T>
    + SubAssign<T>
    + BitAnd<T, Output = T>
    + BitOr<T, Output = T>
    + BitXor<T, Output = T>
    + BitAndAssign<T>
    + BitOrAssign<T>
    + BitXorAssign<T>
    + Index<usize, Output = U> {}

/// 16 and 32 bit int types share all of these
/// constraints, grouping them here speeds up
/// compile times considerably
pub trait SimdSmallInt<T,U> : SimdBase<T,U>
    + Mul<T, Output = T>        
    + MulAssign<T>               
    + Not<Output = T>
    + Shl<i32, Output = T>
    + ShlAssign<i32>
    + Shr<i32, Output = T>
    + ShrAssign<i32> {}

/// f32 and f64 share these constraints, grouping
/// them here speeds up compile times considerably
pub trait SimdFloat<T,U> : SimdBase<T,U>
    + Mul<T, Output = T>
    + Div<T, Output = T>    
    + MulAssign<T>
    + DivAssign<T> {}

pub trait Simd {
    
    /// Vector of i16s.  Corresponds to __m128i when used
    /// with the Sse impl, __m256i when used with Avx2, or a single i16
    /// when used with Scalar.
    type Vi16: SimdSmallInt<Self::Vi16,i16>;
        
    /// Vector of i32s.  Corresponds to __m128i when used
    /// with the Sse impl, __m256i when used with Avx2, or a single i32
    /// when used with Scalar.
    type Vi32: SimdSmallInt<Self::Vi32,i32>;

    /// Vector of i64s.  Corresponds to __m128i when used
    /// with the Sse impl, __m256i when used with Avx2, or a single i64
    /// when used with Scalar.
    type Vi64: SimdBase<Self::Vi64,i64>                                     
        + Not<Output = Self::Vi64>;
        
    /// Vector of f32s.  Corresponds to __m128 when used
    /// with the Sse impl, __m256 when used with Avx2, or a single f32
    /// when used with Scalar.
    type Vf32: SimdFloat<Self::Vf32,f32>;

    /// Vector of f64s.  Corresponds to __m128d when used
    /// with the Sse impl, __m256d when used with Avx2, or a single f64
    /// when used with Scalar.
    type Vf64: SimdFloat<Self::Vf64,f64>;
        
    
    /// The width of the vector lane.  Necessary for creating
    /// lane width agnostic code.
    const VF32_WIDTH: usize;
    const VF64_WIDTH: usize;
    const VI16_WIDTH: usize;
    const VI32_WIDTH: usize;
    const VI64_WIDTH: usize;

    unsafe fn div_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn div_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64;
    unsafe fn abs_ps(a: Self::Vf32) -> Self::Vf32;
    unsafe fn abs_pd(a: Self::Vf64) -> Self::Vf64;
    unsafe fn add_epi16(a: Self::Vi16, b: Self::Vi16) -> Self::Vi16;
    unsafe fn sub_epi16(a: Self::Vi16, b: Self::Vi16) -> Self::Vi16;
    unsafe fn mullo_epi16(a: Self::Vi16, b: Self::Vi16) -> Self::Vi16;
    unsafe fn mulhi_epi16(a: Self::Vi16, b: Self::Vi16) -> Self::Vi16;
    unsafe fn add_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn add_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn add_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64;
    unsafe fn and_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn and_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64;
    unsafe fn andnot_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn andnot_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64;
    unsafe fn andnot_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn andnot_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64;
    /// Note SSE2 will select B only when all bits are 1, while SSE41 and AVX2 only
    /// check the high bit. To maintain portability ensure all bits are 1 when using
    /// blend. Results of comparison operations adhere to this.
    unsafe fn blendv_epi32(a: Self::Vi32, b: Self::Vi32, mask: Self::Vi32) -> Self::Vi32;
    /// Note SSE2 will select B only when all bits are 1, while SSE41 and AVX2 only
    /// check the high bit. To maintain portability ensure all bits are 1 when using
    /// blend. Results of comparison operations adhere to this.
    unsafe fn blendv_epi64(a: Self::Vi64, b: Self::Vi64, mask: Self::Vi64) -> Self::Vi64;
    /// Note SSE2 will select B only when all bits are 1, while SSE41 and AVX2 only
    /// check the high bit. To maintain portability ensure all bits are 1 when using
    /// blend. Results of comparison operations adhere to this.
    unsafe fn blendv_ps(a: Self::Vf32, b: Self::Vf32, mask: Self::Vf32) -> Self::Vf32;
    /// Note SSE2 will select B only when all bits are 1, while SSE41 and AVX2 only
    /// check the high bit. To maintain portability ensure all bits are 1 when using
    /// blend. Results of comparison operations adhere to this.
    unsafe fn blendv_pd(a: Self::Vf64, b: Self::Vf64, mask: Self::Vf64) -> Self::Vf64;
    unsafe fn castps_epi32(a: Self::Vf32) -> Self::Vi32;
    unsafe fn castpd_epi64(a: Self::Vf64) -> Self::Vi64;
    unsafe fn castepi32_ps(a: Self::Vi32) -> Self::Vf32;
    unsafe fn castepi64_pd(a: Self::Vi64) -> Self::Vf64;
    unsafe fn castepi32_epi64(a: Self::Vi32) -> Self::Vi64;
    unsafe fn castepi64_epi32(a: Self::Vi64) -> Self::Vi32;
    unsafe fn castps_pd(a: Self::Vf32) -> Self::Vf64;
    unsafe fn castpd_ps(a: Self::Vf64) -> Self::Vf32;
    unsafe fn ceil_ps(a: Self::Vf32) -> Self::Vf32;
    unsafe fn ceil_pd(a: Self::Vf64) -> Self::Vf64;
    unsafe fn cmpeq_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64;
    unsafe fn cmpneq_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64;
    unsafe fn cmpge_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64;
    unsafe fn cmpgt_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64;
    unsafe fn cmple_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64;
    unsafe fn cmplt_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64;
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
    unsafe fn cmpeq_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64;
    unsafe fn cmpneq_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64;
    unsafe fn cmpge_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64;
    unsafe fn cmpgt_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64;
    unsafe fn cmple_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64;
    unsafe fn cmplt_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64;
    unsafe fn cvtepi32_ps(a: Self::Vi32) -> Self::Vf32;
    unsafe fn cvtps_epi32(a: Self::Vf32) -> Self::Vi32;
    unsafe fn floor_ps(a: Self::Vf32) -> Self::Vf32;
    unsafe fn floor_pd(a: Self::Vf64) -> Self::Vf64;
    /// When using Sse2, fastround uses a faster version of floor
    /// that only works on floating point values small enough to fit in
    /// an i32.  This is a big performance boost if you don't need
    /// a complete floor.
    unsafe fn fastround_ps(a: Self::Vf32) -> Self::Vf32;
    /// When using Sse2, fastceil uses a faster version of floor
    /// that only works on floating point values small enough to fit in
    /// an i32.  This is a big performance boost if you don't need
    /// a complete floor.
    unsafe fn fastceil_ps(a: Self::Vf32) -> Self::Vf32;
    /// When using Sse2, fastfloor uses a faster version of floor
    /// that only works on floating point values small enough to fit in
    /// an i32.  This is a big performance boost if you don't need
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
    /// Actual FMA instructions will be used when Avx2 is used,
    /// otherwise a mul and add are used to replicate it, allowing you to
    /// just always use FMA in your code and get best perf in both cases.
    unsafe fn fmadd_pd(a: Self::Vf64, b: Self::Vf64, c: Self::Vf64) -> Self::Vf64;
    /// Actual FMA instructions will be used when Avx2 is used,
    /// otherwise a mul and add are used to replicate it, allowing you to
    /// just always use FMA in your code and get best perf in both cases.
    unsafe fn fnmadd_pd(a: Self::Vf64, b: Self::Vf64, c: Self::Vf64) -> Self::Vf64;
    /// Actual FMA instructions will be used when Avx2 is used,
    /// otherwise a mul and sub are used to replicate it, allowing you to
    /// just always use FMA in your code and get best perf in both cases.
    unsafe fn fmsub_ps(a: Self::Vf32, b: Self::Vf32, c: Self::Vf32) -> Self::Vf32;
    /// Actual FMA instructions will be used when Avx2 is used,
    /// otherwise a mul and sub are used to replicate it, allowing you to
    /// just always use FMA in your code and get best perf in both cases.
    unsafe fn fnmsub_ps(a: Self::Vf32, b: Self::Vf32, c: Self::Vf32) -> Self::Vf32;
    /// Actual FMA instructions will be used when Avx2 is used,
    /// otherwise a mul and sub are used to replicate it, allowing you to
    /// just always use FMA in your code and get best perf in both cases.
    unsafe fn fmsub_pd(a: Self::Vf64, b: Self::Vf64, c: Self::Vf64) -> Self::Vf64;
    /// Actual FMA instructions will be used when Avx2 is used,
    /// otherwise a mul and sub are used to replicate it, allowing you to
    /// just always use FMA in your code and get best perf in both cases.
    unsafe fn fnmsub_pd(a: Self::Vf64, b: Self::Vf64, c: Self::Vf64) -> Self::Vf64;
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
    unsafe fn load_ps(a: &f32) -> Self::Vf32;
    unsafe fn load_pd(a: &f64) -> Self::Vf64;
    unsafe fn load_epi32(a: &i32) -> Self::Vi32;
    unsafe fn load_epi64(a: &i64) -> Self::Vi64;
    unsafe fn loadu_ps(a: &f32) -> Self::Vf32;
    unsafe fn loadu_pd(a: &f64) -> Self::Vf64;
    unsafe fn loadu_epi32(a: &i32) -> Self::Vi32;
    unsafe fn loadu_epi64(a: &i64) -> Self::Vi64;
    /// Note, SSE2 and SSE4 will load when mask[i] is nonzero, where AVX2
    /// will store only when the high bit is set. To ensure portability
    /// ensure that the high bit is set.
    unsafe fn maskload_epi32(mem_addr: &i32, mask: Self::Vi32) -> Self::Vi32;
    /// Note, SSE2 and SSE4 will load when mask[i] is nonzero, where AVX2
    /// will store only when the high bit is set. To ensure portability
    /// ensure that the high bit is set.
    unsafe fn maskload_epi64(mem_addr: &i64, mask: Self::Vi64) -> Self::Vi64;
    /// Note, SSE2 and SSE4 will load when mask[i] is nonzero, where AVX2
    /// will store only when the high bit is set. To ensure portability
    /// ensure that the high bit is set.
    unsafe fn maskload_ps(mem_addr: &f32, mask: Self::Vi32) -> Self::Vf32;
    /// Note, SSE2 and SSE4 will load when mask[i] is nonzero, where AVX2
    /// will store only when the high bit is set. To ensure portability
    /// ensure that the high bit is set.
    unsafe fn maskload_pd(mem_addr: &f64, mask: Self::Vi64) -> Self::Vf64;
    unsafe fn store_ps(mem_addr: &mut f32, a: Self::Vf32);
    unsafe fn store_pd(mem_addr: &mut f64, a: Self::Vf64);
    unsafe fn store_epi32(mem_addr: &mut i32, a: Self::Vi32);
    unsafe fn store_epi64(mem_addr: &mut i64, a: Self::Vi64);
    unsafe fn storeu_ps(mem_addr: &mut f32, a: Self::Vf32);
    unsafe fn storeu_pd(mem_addr: &mut f64, a: Self::Vf64);
    unsafe fn storeu_epi32(mem_addr: &mut i32, a: Self::Vi32);
    unsafe fn storeu_epi64(mem_addr: &mut i64, a: Self::Vi64);
    /// Note, SSE2 and SSE4 will store when mask[i] is nonzero, where AVX2
    /// will store only when the high bit is set. To ensure portability ensure the
    /// high bit is set.
    unsafe fn maskstore_epi32(mem_addr: &mut i32, mask: Self::Vi32, a: Self::Vi32);
    /// Note, SSE2 and SSE4 will store when mask[i] is nonzero, where AVX2
    /// will store only when the high bit is set. To ensure portability ensure the
    /// high bit is set.
    unsafe fn maskstore_epi64(mem_addr: &mut i64, mask: Self::Vi64, a: Self::Vi64);
    /// Note, SSE2 and SSE4 will store when mask[i] is nonzero, where AVX2
    /// will store only when the high bit is set. To ensure portability ensure the
    /// high bit is set.
    unsafe fn maskstore_ps(mem_addr: &mut f32, mask: Self::Vi32, a: Self::Vf32);
    /// Note, SSE2 and SSE4 will store when mask[i] is nonzero, where AVX2
    /// will store only when the high bit is set. To ensure portability ensure the
    /// high bit is set.
    unsafe fn maskstore_pd(mem_addr: &mut f64, mask: Self::Vi64, a: Self::Vf64);
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
    unsafe fn not_epi64(a: Self::Vi64) -> Self::Vi64;
    unsafe fn or_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn or_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64;
    unsafe fn or_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn or_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64;
    unsafe fn rcp_ps(a: Self::Vf32) -> Self::Vf32;
    /// Round is implemented for Sse2 by combining other Sse2 operations.
    unsafe fn round_ps(a: Self::Vf32) -> Self::Vf32;
    unsafe fn round_pd(a: Self::Vf64) -> Self::Vf64;
    unsafe fn set1_epi32(a: i32) -> Self::Vi32;
    unsafe fn set1_epi64(a: i64) -> Self::Vi64;
    unsafe fn set1_ps(a: f32) -> Self::Vf32;
    unsafe fn set1_pd(a: f64) -> Self::Vf64;
    unsafe fn setzero_ps() -> Self::Vf32;
    unsafe fn setzero_pd() -> Self::Vf64;
    unsafe fn setzero_epi32() -> Self::Vi32;
    unsafe fn setzero_epi64() -> Self::Vi64;
    /// amt must be a constant
    unsafe fn srai_epi32(a: Self::Vi32, amt_const: i32) -> Self::Vi32;
    /// amt must be a constant
    unsafe fn srai_epi64(a: Self::Vi64, amt_const: i32) -> Self::Vi64;
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
    unsafe fn shuffle_ps(a: Self::Vf32, b: Self::Vf32, imm8: i32) -> Self::Vf32;
    unsafe fn unpackhi_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn unpacklo_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn unpackhi_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64;
    unsafe fn unpacklo_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64;
    unsafe fn unpackhi_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64;
    unsafe fn unpacklo_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64;
    unsafe fn xor_epi32(a: Self::Vi32, b: Self::Vi32) -> Self::Vi32;
    unsafe fn xor_epi64(a: Self::Vi64, b: Self::Vi64) -> Self::Vi64;
    unsafe fn xor_ps(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
    unsafe fn xor_pd(a: Self::Vf64, b: Self::Vf64) -> Self::Vf64;
}
