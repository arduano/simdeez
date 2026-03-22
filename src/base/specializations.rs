use super::transmute::*;
use crate::{Simd, SimdBaseOps};
use core::ops::*;

pub trait SimdBitMask: Copy {
    const WORDS: usize;

    fn word(self, index: usize) -> u32;

    #[inline(always)]
    fn to_u32_lossy(self) -> u32 {
        self.word(0)
    }

    #[inline(always)]
    fn to_u64_lossy(self) -> u64 {
        let mut out = 0u64;
        let words = if Self::WORDS < 2 { Self::WORDS } else { 2 };
        let mut index = 0;
        while index < words {
            out |= (self.word(index) as u64) << (index * 32);
            index += 1;
        }
        out
    }

    #[inline(always)]
    fn to_u128_lossy(self) -> u128 {
        let mut out = 0u128;
        let words = if Self::WORDS < 4 { Self::WORDS } else { 4 };
        let mut index = 0;
        while index < words {
            out |= (self.word(index) as u128) << (index * 32);
            index += 1;
        }
        out
    }

    #[inline(always)]
    fn write_to_u32_slice(self, dst: &mut [u32]) -> usize {
        let words = if Self::WORDS < dst.len() {
            Self::WORDS
        } else {
            dst.len()
        };
        let mut index = 0;
        while index < words {
            dst[index] = self.word(index);
            index += 1;
        }
        words
    }

    #[inline(always)]
    fn valid_bits(index: usize, bit_len: usize) -> u32 {
        let start = index * 32;
        if start >= bit_len {
            return 0;
        }

        let remaining = bit_len - start;
        if remaining >= 32 {
            u32::MAX
        } else {
            (1u32 << remaining) - 1
        }
    }

    #[inline(always)]
    fn any_in_mask(self, bit_len: usize) -> bool {
        for index in 0..Self::WORDS {
            if (self.word(index) & Self::valid_bits(index, bit_len)) != 0 {
                return true;
            }
        }
        false
    }

    #[inline(always)]
    fn all_in_mask(self, bit_len: usize) -> bool {
        for index in 0..Self::WORDS {
            let valid = Self::valid_bits(index, bit_len);
            if (self.word(index) & valid) != valid {
                return false;
            }
        }
        true
    }

    #[inline(always)]
    fn first_set_in_mask(self, bit_len: usize) -> Option<usize> {
        for index in 0..Self::WORDS {
            let word = self.word(index) & Self::valid_bits(index, bit_len);
            if word != 0 {
                return Some(index * 32 + word.trailing_zeros() as usize);
            }
        }
        None
    }

    #[inline(always)]
    fn first_unset_in_mask(self, bit_len: usize) -> Option<usize> {
        for index in 0..Self::WORDS {
            let valid = Self::valid_bits(index, bit_len);
            let word = (!self.word(index)) & valid;
            if word != 0 {
                return Some(index * 32 + word.trailing_zeros() as usize);
            }
        }
        None
    }

    #[inline(always)]
    fn last_set_in_mask(self, bit_len: usize) -> Option<usize> {
        for index in (0..Self::WORDS).rev() {
            let word = self.word(index) & Self::valid_bits(index, bit_len);
            if word != 0 {
                return Some(index * 32 + (31 - word.leading_zeros()) as usize);
            }
        }
        None
    }

    #[inline(always)]
    fn last_unset_in_mask(self, bit_len: usize) -> Option<usize> {
        for index in (0..Self::WORDS).rev() {
            let valid = Self::valid_bits(index, bit_len);
            let word = (!self.word(index)) & valid;
            if word != 0 {
                return Some(index * 32 + (31 - word.leading_zeros()) as usize);
            }
        }
        None
    }
}

impl SimdBitMask for u32 {
    const WORDS: usize = 1;

    #[inline(always)]
    fn word(self, index: usize) -> u32 {
        debug_assert_eq!(index, 0);
        if index == 0 {
            self
        } else {
            0
        }
    }
}

impl<const N: usize> SimdBitMask for [u32; N] {
    const WORDS: usize = N;

    #[inline(always)]
    fn word(self, index: usize) -> u32 {
        self[index]
    }
}

/// Operations shared by 16 and 32 bit int types
pub trait SimdInt:
    SimdBaseOps + Shl<i32, Output = Self> + ShlAssign<i32> + Shr<i32, Output = Self> + ShrAssign<i32>
{
    /// Shift each value left by n bits.
    ///
    /// Shift counts use wrapping semantics (`rhs mod lane_bit_width`) so behavior is
    /// defined for negative and out-of-range counts across all backends.
    ///
    /// For 64 bits, this operations is missing in most implementations
    /// and is emulated here under SSE2, SSE4.1, and AVX2.
    fn shl(self, rhs: i32) -> Self;

    /// Shift each value right by n bits.
    ///
    /// Shift counts use wrapping semantics (`rhs mod lane_bit_width`) so behavior is
    /// defined for negative and out-of-range counts across all backends.
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

    fn from_i64(value: i64) -> Self;
}

/// Operations shared by 8 bit int types
pub trait SimdInt8: SimdInt<Scalar = i8, HorizontalAddScalar = i64> + SimdTransmuteI8 {
    type BitMask: SimdBitMask;

    /// Splits the vector into two halves, then extends them both to be i16. This is useful for horizontal adding.
    fn extend_to_i16(self) -> (<Self::Engine as Simd>::Vi16, <Self::Engine as Simd>::Vi16);

    /// Splits the vector into two halves, then extends them both to be i16. This is useful for horizontal adding.
    /// The numbers are treated as unsigned, so the sign bit isn't moved. This is more efficient on some instruction sets.
    fn unsigned_extend_to_i16(self)
        -> (<Self::Engine as Simd>::Vi16, <Self::Engine as Simd>::Vi16);

    /// Adds (arbitrary) pairs of values in the vector, returning a i16 version of the vector.
    /// The way the pairs are chosen is implementation-defined.
    #[inline(always)]
    fn partial_horizontal_add(self) -> <Self::Engine as Simd>::Vi16 {
        let (a, b) = self.extend_to_i16();
        a + b
    }

    /// Adds (arbitrary) pairs of values in the vector, returning a i16 version of the vector.
    /// When extending the numbers, they're treated as unsigned wich performs more efficiently on some instruction sets.
    /// The way the pairs are chosen is implementation-defined.
    #[inline(always)]
    fn partial_horizontal_unsigned_add(self) -> <Self::Engine as Simd>::Vi16 {
        let (a, b) = self.unsigned_extend_to_i16();
        a + b
    }

    /// Gets the lane truthiness mask for the vector.
    ///
    /// Use the helper methods on [`SimdBitMask`] (or the default methods below) instead of assuming
    /// that all lanes fit into a single integer. This keeps the API usable for wider future SIMD backends.
    ///
    /// Portably, these mask helpers are intended for **canonical compare masks**: each truthy lane should
    /// be all bits set (`-1`) and each falsy lane should be zero. Compare operations in simdeez already
    /// produce masks in that form.
    fn get_mask(self) -> Self::BitMask;

    /// Checks if any element in the vector is truthy. A value is truthy either if the highest bit is one, or if any bit is one,
    /// depending on the instruction set being used. For portable behavior, prefer canonical compare masks where truthy lanes
    /// are `-1` and falsy lanes are `0`.
    #[inline(always)]
    fn is_any_truthy(self) -> bool {
        self.get_mask().any_in_mask(Self::WIDTH)
    }

    /// Checks if all elements in the vector are truthy. A value is truthy either if the highest bit is one, or if any bit is one,
    /// depending on the instruction set being used. For portable behavior, prefer canonical compare masks where truthy lanes
    /// are `-1` and falsy lanes are `0`.
    #[inline(always)]
    fn is_truthy(self) -> bool {
        self.get_mask().all_in_mask(Self::WIDTH)
    }

    /// Grabs the index of the last value that matches the given value. If no value matches, returns None.
    /// Index will always be smaller than Self::WIDTH.
    #[inline(always)]
    fn index_of_last_truthy(self) -> Option<usize> {
        self.get_mask().last_set_in_mask(Self::WIDTH)
    }

    /// Grabs the index of the last value that matches the given value. If no value matches, returns None.
    /// Index will always be smaller than Self::WIDTH.
    #[inline(always)]
    fn index_of_last_falsy(self) -> Option<usize> {
        self.get_mask().last_unset_in_mask(Self::WIDTH)
    }

    /// Grabs the index of the first value that matches the given value. If no value matches, returns None.
    /// Index will always be smaller than Self::WIDTH.
    #[inline(always)]
    fn index_of_first_truthy(self) -> Option<usize> {
        self.get_mask().first_set_in_mask(Self::WIDTH)
    }

    /// Grabs the index of the first value that matches the given value. If no value matches, returns None.
    /// Index will always be smaller than Self::WIDTH.
    #[inline(always)]
    fn index_of_first_falsy(self) -> Option<usize> {
        self.get_mask().first_unset_in_mask(Self::WIDTH)
    }

    /// Grabs the index of the first value that matches the given value. If no value matches, returns None.
    /// Index will always be smaller than Self::WIDTH.
    #[inline(always)]
    fn index_of_first_eq(self, value: i8) -> Option<usize> {
        let value = Self::set1(value);
        let mask = self.cmp_eq(value);
        mask.index_of_first_truthy()
    }
}

/// Operations shared by 16 bit int types
pub trait SimdInt16: SimdInt<Scalar = i16, HorizontalAddScalar = i64> + SimdTransmuteI16 {
    /// Splits the vector into two halves, then extends them both to be i32. This is useful for horizontal adding.
    fn extend_to_i32(self) -> (<Self::Engine as Simd>::Vi32, <Self::Engine as Simd>::Vi32);

    /// Splits the vector into two halves, then extends them both to be i32. This is useful for horizontal adding.
    /// The numbers are treated as unsigned, so the sign bit isn't moved. This is more efficient on some instruction sets.
    fn unsigned_extend_to_i32(self)
        -> (<Self::Engine as Simd>::Vi32, <Self::Engine as Simd>::Vi32);

    /// Adds (arbitrary) pairs of values in the vector, returning a i32 version of the vector.
    /// The way the pairs are chosen is implementation-defined.
    #[inline(always)]
    fn partial_horizontal_add(self) -> <Self::Engine as Simd>::Vi32 {
        let (a, b) = self.extend_to_i32();
        a + b
    }

    /// Adds (arbitrary) pairs of values in the vector, returning a i32 version of the vector.
    /// When extending the numbers, they're treated as unsigned wich performs more efficiently on some instruction sets.
    /// The way the pairs are chosen is implementation-defined.
    #[inline(always)]
    fn partial_horizontal_unsigned_add(self) -> <Self::Engine as Simd>::Vi32 {
        let (a, b) = self.unsigned_extend_to_i32();
        a + b
    }
}

/// Operations shared by 32 bit int types
pub trait SimdInt32: SimdInt<Scalar = i32, HorizontalAddScalar = i64> + SimdTransmuteI32 {
    /// Bit cast to f32.
    /// This function is only used for compilation and does not generate any instructions, thus it has zero latency.
    fn bitcast_f32(self) -> <Self::Engine as Simd>::Vf32;

    /// Element-wise cast to f32
    fn cast_f32(self) -> <Self::Engine as Simd>::Vf32;

    /// Splits the vector into two halves, then extends them both to be i64. This is useful for horizontal adding.
    fn extend_to_i64(self) -> (<Self::Engine as Simd>::Vi64, <Self::Engine as Simd>::Vi64);

    /// Splits the vector into two halves, then extends them both to be i32. This is useful for horizontal adding.
    /// The numbers are treated as unsigned, so the sign bit isn't moved. This is more efficient on some instruction sets.
    fn unsigned_extend_to_i64(self)
        -> (<Self::Engine as Simd>::Vi64, <Self::Engine as Simd>::Vi64);

    /// Adds (arbitrary) pairs of values in the vector, returning a i64 version of the vector.
    /// The way the pairs are chosen is implementation-defined.
    #[inline(always)]
    fn partial_horizontal_add(self) -> <Self::Engine as Simd>::Vi64 {
        let (a, b) = self.extend_to_i64();
        a + b
    }

    /// Adds (arbitrary) pairs of values in the vector, returning a i64 version of the vector.
    /// When extending the numbers, they're treated as unsigned wich performs more efficiently on some instruction sets.
    /// The way the pairs are chosen is implementation-defined.
    #[inline(always)]
    fn partial_horizontal_unsigned_add(self) -> <Self::Engine as Simd>::Vi64 {
        let (a, b) = self.unsigned_extend_to_i64();
        a + b
    }
}

/// Operations shared by 64 bt int types
pub trait SimdInt64: SimdInt<Scalar = i64, HorizontalAddScalar = i64> + SimdTransmuteI64 {
    /// Bit cast to f64.
    /// This function is only used for compilation and does not generate any instructions, thus it has zero latency.
    fn bitcast_f64(self) -> <Self::Engine as Simd>::Vf64;

    /// Element-wise cast to f64
    fn cast_f64(self) -> <Self::Engine as Simd>::Vf64;

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

    /// Element-wise approximate inverse square root.
    ///
    /// Accuracy and edge-case handling are backend dependent, especially for non-positive,
    /// subnormal, infinite, and NaN inputs. Only finite positive lanes are guaranteed to
    /// produce a meaningful approximation of `1.0 / sqrt(x)`.
    fn rsqrt(self) -> Self;

    fn from_f64(value: f64) -> Self;
}

/// Operations shared by 32 bit float types
pub trait SimdFloat32:
    SimdFloat<Scalar = f32, HorizontalAddScalar = f32> + SimdTransmuteF32
{
    /// Bit cast to i32.
    /// This function is only used for compilation and does not generate any instructions, thus it has zero latency.
    fn bitcast_i32(self) -> <Self::Engine as Simd>::Vi32;

    /// Element-wise cast to i32 (rounded to nearest, ties to even; not floored).
    /// Note, this may cause undefined behavior when casting from numbers outside the range of i32.
    /// E.g. a very large positive float may become i32::MIN.
    fn cast_i32(self) -> <Self::Engine as Simd>::Vi32;

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
    /// Bit cast to i64.
    /// This function is only used for compilation and does not generate any instructions, thus it has zero latency.
    fn bitcast_i64(self) -> <Self::Engine as Simd>::Vi64;

    /// Element-wise cast to i64 (rounded to nearest, ties to even; not floored).
    fn cast_i64(self) -> <Self::Engine as Simd>::Vi64;
}

#[cfg(test)]
mod tests {
    use super::SimdBitMask;

    #[test]
    fn bitmask_helpers_work_for_single_word_masks() {
        let mask = 0b0010_1000u32;
        assert!(mask.any_in_mask(6));
        assert!(!mask.all_in_mask(6));
        assert_eq!(mask.first_set_in_mask(6), Some(3));
        assert_eq!(mask.last_set_in_mask(6), Some(5));
        assert_eq!(mask.first_unset_in_mask(6), Some(0));
        assert_eq!(mask.last_unset_in_mask(6), Some(4));
        assert_eq!(mask.to_u32_lossy(), 0b0010_1000);
        assert_eq!(mask.to_u64_lossy(), 0b0010_1000);
        assert_eq!(mask.to_u128_lossy(), 0b0010_1000);
    }

    #[test]
    fn bitmask_can_be_written_to_word_slices() {
        let mask = [0x89AB_CDEFu32, 0x0123_4567u32];
        let mut out = [0u32; 4];
        let written = mask.write_to_u32_slice(&mut out);
        assert_eq!(written, 2);
        assert_eq!(out, [0x89AB_CDEF, 0x0123_4567, 0, 0]);
    }

    #[test]
    fn bitmask_lossy_integer_exports_pack_words_in_order() {
        let mask = [
            0x89AB_CDEFu32,
            0x0123_4567u32,
            0xDEAD_BEEFu32,
            0xCAFE_BABEu32,
            0xFFFF_FFFFu32,
        ];
        assert_eq!(mask.to_u32_lossy(), 0x89AB_CDEF);
        assert_eq!(mask.to_u64_lossy(), 0x0123_4567_89AB_CDEFu64);
        assert_eq!(
            mask.to_u128_lossy(),
            0xCAFE_BABE_DEAD_BEEF_0123_4567_89AB_CDEFu128
        );
    }

    #[test]
    fn bitmask_helpers_work_across_multiple_words() {
        let mask = [0u32, 1u32 << 5];
        assert!(mask.any_in_mask(64));
        assert_eq!(mask.first_set_in_mask(64), Some(37));
        assert_eq!(mask.last_set_in_mask(64), Some(37));
        assert_eq!(mask.first_unset_in_mask(64), Some(0));
        assert_eq!(mask.last_unset_in_mask(64), Some(63));
    }

    #[test]
    fn bitmask_helpers_ignore_padding_bits_outside_mask() {
        let mask = [u32::MAX, u32::MAX];
        assert!(mask.all_in_mask(40));
        assert_eq!(mask.first_unset_in_mask(40), None);
        assert_eq!(mask.last_unset_in_mask(40), None);
    }
}
