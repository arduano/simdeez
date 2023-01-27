use crate::SimdBaseOps;
use core::ops::*;

/// Operations shared by 16 and 32 bit int types
pub trait SimdInt:
    SimdBaseOps + Shl<i32, Output = Self> + ShlAssign<i32> + Shr<i32, Output = Self> + ShrAssign<i32>
{
    /// Shift each value left by n bits.
    ///
    /// For 64 bits, this operations is missing in most implementations
    /// and is emulated here under SSE2, SSE4.1, and AVX2.
    fn shl(self, rhs: i32) -> Self;

    /// Shift each value right by n bits.
    ///
    /// For 64 bits, this operations is missing in most implementations
    /// and is emulated here under SSE2, SSE4.1, and AVX2.
    fn shr(self, rhs: i32) -> Self;

    /// Shift each value left by a constant n bits. This operation is faster in some instruction sets.
    ///
    /// For 64 bits, this operations is missing in most implementations
    /// and is emulated here under SSE2, SSE4.1, and AVX2.
    fn shl_const<const BY: i32>(self) -> Self {
        SimdInt::shl(self, BY)
    }

    /// Shift each value right by a constant n bits. This operation is faster in some instruction sets.
    ///
    /// For 64 bits, this operations is missing in most implementations
    /// and is emulated here under SSE2, SSE4.1, and AVX2.
    fn shr_const<const BY: i32>(self) -> Self {
        SimdInt::shr(self, BY)
    }
}

/// Operations shared by 16 bit int types
pub trait SimdInt16: SimdInt<Scalar = i16> {}

/// Operations shared by 32 bit int types
pub trait SimdInt32: SimdInt<Scalar = i32> {
    type SimdF32: SimdFloat32;

    /// Bit cast to f32.
    /// This function is only used for compilation and does not generate any instructions, thus it has zero latency.
    fn bitcast_f32(self) -> Self::SimdF32;

    /// Element-wise cast to f32
    fn cast_f32(self) -> Self::SimdF32;
}

/// Operations shared by 64 bt int types
pub trait SimdInt64: SimdInt<Scalar = i64> {
    type SimdF64: SimdFloat64;

    /// Bit cast to f64.
    /// This function is only used for compilation and does not generate any instructions, thus it has zero latency.
    fn bitcast_f64(self) -> Self::SimdF64;

    /// Element-wise cast to f64
    fn cast_f64(self) -> Self::SimdF64;
}

/// Operations shared by f32 and f64 floating point types
pub trait SimdFloat:
    SimdBaseOps
    + Div<Self, Output = Self>
    + DivAssign<Self>
    + Div<Self::Scalar, Output = Self>
    + DivAssign<Self::Scalar> //
{
    /// Element-wise divide between two vectors
    fn div(self, rhs: Self) -> Self;

    /// Element-wise ceilings between two vectors
    fn ceil(self) -> Self;

    /// Element-wise floors between two vectors
    fn floor(self) -> Self;

    /// Element-wise rounds between two vectors
    fn round(self) -> Self;

    /// Alternative element-wise ceilings between two vectors.
    /// When using Sse2, this uses a faster version of ceiling
    /// that only works on floating point values small enough to fit in
    /// an i32.  This is a big performance boost if you don't need
    /// a complete ceiling.
    fn fast_ceil(self) -> Self;

    /// Alternative element-wise floors between two vectors.
    /// When using Sse2, this uses a faster version of floor
    /// that only works on floating point values small enough to fit in
    /// an i32.  This is a big performance boost if you don't need
    /// a complete floor.
    fn fast_floor(self) -> Self;

    /// Alternative element-wise rounds between two vectors.
    /// When using Sse2, this uses a faster version of round
    /// that only works on floating point values small enough to fit in
    /// an i32.  This is a big performance boost if you don't need
    /// a complete round.
    fn fast_round(self) -> Self;

    /// Element-wise multiply add. This performs `Self * A + B`
    fn mul_add(self, a: Self, b: Self) -> Self;

    /// Element-wise multiply subtract. This performs `Self * A - B`
    fn mul_sub(self, a: Self, b: Self) -> Self;

    /// Element-wise negative multiply add. This performs `-(Self * A) + B`
    fn neg_mul_add(self, a: Self, b: Self) -> Self;

    /// Element-wise negative multiply subtract. This performs `-(Self * A) - B`
    fn neg_mul_sub(self, a: Self, b: Self) -> Self;

    /// Adds all of the elements in the vector together and returns the scalar result
    fn horizontal_add(self) -> Self::Scalar;

    /// Element-wise square root
    fn sqrt(self) -> Self;

    /// Element-wise approximate inverse square root
    fn rsqrt(self) -> Self;
}

/// Operations shared by 32 bit float types
pub trait SimdFloat32: SimdFloat<Scalar = f32> {
    type SimdI32: SimdInt32;

    /// Bit cast to i32.
    /// This function is only used for compilation and does not generate any instructions, thus it has zero latency.
    fn bitcast_i32(self) -> Self::SimdI32;

    /// Element-wise cast to i32 (rounded, not floored). Note, this may cause undefined behavior when casting from
    /// numbers outside the range of i32. E.g. a very large positive float may become i32::MIN.
    fn cast_i32(self) -> Self::SimdI32;

    /// Element-wise fast reciprocal (1.0 / x)
    fn fast_inverse(self) -> Self;

    cfg_if::cfg_if! {
        if #[cfg(feature = "sleef")] {
            fn sin(a: Self::Vf32) -> Self::Vf32;
            fn fast_sin(a: Self::Vf32) -> Self::Vf32;
            fn cos(a: Self::Vf32) -> Self::Vf32;
            fn fast_cos(a: Self::Vf32) -> Self::Vf32;
            fn asin(a: Self::Vf32) -> Self::Vf32;
            fn fast_asin(a: Self::Vf32) -> Self::Vf32;
            fn acos(a: Self::Vf32) -> Self::Vf32;
            fn fast_acos(a: Self::Vf32) -> Self::Vf32;
            fn tan(a: Self::Vf32) -> Self::Vf32;
            fn fast_tan(a: Self::Vf32) -> Self::Vf32;
            fn atan(a: Self::Vf32) -> Self::Vf32;
            fn fast_atan(a: Self::Vf32) -> Self::Vf32;

            //hyperbolic
            fn sinh(a: Self::Vf32) -> Self::Vf32;
            fn fast_sinh(a: Self::Vf32) -> Self::Vf32;
            fn cosh(a: Self::Vf32) -> Self::Vf32;
            fn fast_cosh(a: Self::Vf32) -> Self::Vf32;
            fn asinh(a: Self::Vf32) -> Self::Vf32;
            fn acosh(a: Self::Vf32) -> Self::Vf32;
            fn tanh(a: Self::Vf32) -> Self::Vf32;
            fn fast_tanh(a: Self::Vf32) -> Self::Vf32;
            fn atanh(a: Self::Vf32) -> Self::Vf32;

            fn atan2(a: Self::Vf32,b: Self::Vf32) -> Self::Vf32;
            fn fast_atan2(a: Self::Vf32,b: Self::Vf32) -> Self::Vf32;
            fn ln(a: Self::Vf32) -> Self::Vf32;
            fn fast_ln(a: Self::Vf32) -> Self::Vf32;
            fn log2(a: Self::Vf32) -> Self::Vf32;
            fn log10(a: Self::Vf32) -> Self::Vf32;
            fn hypot(a: Self::Vf32,b: Self::Vf32) -> Self::Vf32;
            fn fast_hypot(a: Self::Vf32,b: Self::Vf32) -> Self::Vf32;

            fn fmod(a: Self::Vf32, b: Self::Vf32) -> Self::Vf32;
        }
    }
}

/// Operations shared by 64 bit float types
pub trait SimdFloat64: SimdFloat<Scalar = f64> {
    type SimdI64: SimdInt64;

    /// Bit cast to i64.
    /// This function is only used for compilation and does not generate any instructions, thus it has zero latency.
    fn bitcast_i64(self) -> Self::SimdI64;

    /// Element-wise cast to i64 (rounded, not floored).
    fn cast_i64(self) -> Self::SimdI64;
}
