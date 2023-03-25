use super::transmute::*;
use crate::{InternalSimdBaseIo, SimdBaseOps};
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
    #[inline(always)]
    fn shl_const<const BY: i32>(self) -> Self {
        SimdInt::shl(self, BY)
    }

    /// Shift each value right by a constant n bits. This operation is faster in some instruction sets.
    ///
    /// For 64 bits, this operations is missing in most implementations
    /// and is emulated here under SSE2, SSE4.1, and AVX2.
    #[inline(always)]
    fn shr_const<const BY: i32>(self) -> Self {
        SimdInt::shr(self, BY)
    }

    /// Add every number in the vector together in unsigned arithmetic. When expanding the size of each number,
    /// it treats the numbers as unsigned, meaning the sign bit doesn't get moved around.
    fn horizontal_unsigned_add(self) -> Self::HorizontalAddScalar;
}

/// Operations shared by 8 bit int types
pub trait SimdInt8:
    SimdInt<Scalar = i8, HorizontalAddScalar = i64> + InternalSimdBaseIo + SimdTransmuteI8
{
    type SimdI16: SimdInt16;

    /// Splits the vector into two halves, then extends them both to be i16. This is useful for horizontal adding.
    fn extend_to_i16(self) -> (Self::SimdI16, Self::SimdI16);

    /// Splits the vector into two halves, then extends them both to be i16. This is useful for horizontal adding.
    /// The numbers are treated as unsigned, so the sign bit isn't moved. This is more efficient on some instruction sets.
    fn unsigned_extend_to_i16(self) -> (Self::SimdI16, Self::SimdI16);

    /// Adds (arbitrary) pairs of values in the vector, returning a i16 version of the vector.
    /// The way the pairs are chosen is implementation-defined.
    #[inline(always)]
    fn partial_horizontal_add(self) -> Self::SimdI16 {
        let (a, b) = self.extend_to_i16();
        a + b
    }

    /// Adds (arbitrary) pairs of values in the vector, returning a i16 version of the vector.
    /// When extending the numbers, they're treated as unsigned wich performs more efficiently on some instruction sets.
    /// The way the pairs are chosen is implementation-defined.
    #[inline(always)]
    fn partial_horizontal_unsigned_add(self) -> Self::SimdI16 {
        let (a, b) = self.unsigned_extend_to_i16();
        a + b
    }

    /// Gets the "mask" of a vector, where each bit in the u32 represents whether the value at that location
    /// is truthy. A value is truthy either if the highest bit is one, or if any bit is one, depending
    /// on the instruction set being used. Please always make sure at least the highest bit is set to 1.
    fn get_mask(self) -> u32;

    /// Checks if any element in the vector is truthy. A value is truthy either if the highest bit is one, or if any bit is one,
    /// depending on the instruction set being used. Please always make sure at least the highest bit is set to 1.
    #[inline(always)]
    fn is_any_truthy(self) -> bool {
        self.get_mask() != 0
    }

    /// Grabs the index of the last value that matches the given value. If no value matches, returns None.
    /// Index will always be smaller than Self::WIDTH.
    #[inline(always)]
    fn index_of_last_truthy(self) -> Option<usize> {
        let leading = self.get_mask().leading_zeros();
        if leading >= Self::WIDTH as u32 {
            None
        } else {
            Some(leading as usize)
        }
    }

    /// Grabs the index of the last value that matches the given value. If no value matches, returns None.
    /// Index will always be smaller than Self::WIDTH.
    #[inline(always)]
    fn index_of_last_falsy(self) -> Option<usize> {
        let leading = self.get_mask().leading_ones();
        if leading >= Self::WIDTH as u32 {
            None
        } else {
            Some(leading as usize)
        }
    }

    /// Grabs the index of the first value that matches the given value. If no value matches, returns None.
    /// Index will always be smaller than Self::WIDTH.
    #[inline(always)]
    fn index_of_first_truthy(self) -> Option<usize> {
        let trailing = self.get_mask().trailing_zeros();
        if trailing >= Self::WIDTH as u32 {
            None
        } else {
            Some(trailing as usize)
        }
    }

    /// Grabs the index of the first value that matches the given value. If no value matches, returns None.
    /// Index will always be smaller than Self::WIDTH.
    #[inline(always)]
    fn index_of_first_falsy(self) -> Option<usize> {
        let trailing = self.get_mask().trailing_ones();
        if trailing >= Self::WIDTH as u32 {
            None
        } else {
            Some(trailing as usize)
        }
    }

    /// Grabs the index of the first value that matches the given value. If no value matches, returns None.
    /// Index will always be smaller than Self::WIDTH.
    #[inline(always)]
    fn index_of_first_eq(self, value: i8) -> Option<usize> {
        let value = unsafe { Self::set1(value) };
        let mask = self.cmp_eq(value);
        mask.index_of_first_truthy()
    }
}

/// Operations shared by 16 bit int types
pub trait SimdInt16: SimdInt<Scalar = i16, HorizontalAddScalar = i64> + SimdTransmuteI16 {
    type SimdI32: SimdInt32;

    /// Splits the vector into two halves, then extends them both to be i32. This is useful for horizontal adding.
    fn extend_to_i32(self) -> (Self::SimdI32, Self::SimdI32);

    /// Splits the vector into two halves, then extends them both to be i32. This is useful for horizontal adding.
    /// The numbers are treated as unsigned, so the sign bit isn't moved. This is more efficient on some instruction sets.
    fn unsigned_extend_to_i32(self) -> (Self::SimdI32, Self::SimdI32);

    /// Adds (arbitrary) pairs of values in the vector, returning a i32 version of the vector.
    /// The way the pairs are chosen is implementation-defined.
    #[inline(always)]
    fn partial_horizontal_add(self) -> Self::SimdI32 {
        let (a, b) = self.extend_to_i32();
        a + b
    }

    /// Adds (arbitrary) pairs of values in the vector, returning a i32 version of the vector.
    /// When extending the numbers, they're treated as unsigned wich performs more efficiently on some instruction sets.
    /// The way the pairs are chosen is implementation-defined.
    #[inline(always)]
    fn partial_horizontal_unsigned_add(self) -> Self::SimdI32 {
        let (a, b) = self.unsigned_extend_to_i32();
        a + b
    }
}

/// Operations shared by 32 bit int types
pub trait SimdInt32: SimdInt<Scalar = i32, HorizontalAddScalar = i64> + SimdTransmuteI32 {
    type SimdF32: SimdFloat32;
    type SimdI64: SimdInt64;

    /// Bit cast to f32.
    /// This function is only used for compilation and does not generate any instructions, thus it has zero latency.
    fn bitcast_f32(self) -> Self::SimdF32;

    /// Element-wise cast to f32
    fn cast_f32(self) -> Self::SimdF32;

    /// Splits the vector into two halves, then extends them both to be i64. This is useful for horizontal adding.
    fn extend_to_i64(self) -> (Self::SimdI64, Self::SimdI64);

    /// Splits the vector into two halves, then extends them both to be i32. This is useful for horizontal adding.
    /// The numbers are treated as unsigned, so the sign bit isn't moved. This is more efficient on some instruction sets.
    fn unsigned_extend_to_i64(self) -> (Self::SimdI64, Self::SimdI64);

    /// Adds (arbitrary) pairs of values in the vector, returning a i64 version of the vector.
    /// The way the pairs are chosen is implementation-defined.
    #[inline(always)]
    fn partial_horizontal_add(self) -> Self::SimdI64 {
        let (a, b) = self.extend_to_i64();
        a + b
    }

    /// Adds (arbitrary) pairs of values in the vector, returning a i64 version of the vector.
    /// When extending the numbers, they're treated as unsigned wich performs more efficiently on some instruction sets.
    /// The way the pairs are chosen is implementation-defined.
    #[inline(always)]
    fn partial_horizontal_unsigned_add(self) -> Self::SimdI64 {
        let (a, b) = self.unsigned_extend_to_i64();
        a + b
    }
}

/// Operations shared by 64 bt int types
pub trait SimdInt64: SimdInt<Scalar = i64, HorizontalAddScalar = i64> + SimdTransmuteI64 {
    type SimdF64: SimdFloat64;

    /// Bit cast to f64.
    /// This function is only used for compilation and does not generate any instructions, thus it has zero latency.
    fn bitcast_f64(self) -> Self::SimdF64;

    /// Element-wise cast to f64
    fn cast_f64(self) -> Self::SimdF64;

    fn partial_horizontal_add(self) -> i64;
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

    /// Element-wise square root
    fn sqrt(self) -> Self;

    /// Element-wise approximate inverse square root
    fn rsqrt(self) -> Self;
}

/// Operations shared by 32 bit float types
pub trait SimdFloat32:
    SimdFloat<Scalar = f32, HorizontalAddScalar = f32> + SimdTransmuteF32
{
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
pub trait SimdFloat64:
    SimdFloat<Scalar = f64, HorizontalAddScalar = f64> + SimdTransmuteF64
{
    type SimdI64: SimdInt64;

    /// Bit cast to i64.
    /// This function is only used for compilation and does not generate any instructions, thus it has zero latency.
    fn bitcast_i64(self) -> Self::SimdI64;

    /// Element-wise cast to i64 (rounded, not floored).
    fn cast_i64(self) -> Self::SimdI64;
}
