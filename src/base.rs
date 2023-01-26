use core::fmt::Debug;
use core::ops::*;

/// Operations shared by all SIMD types
pub trait SimdBase:
    Copy
    + Debug
    + IndexMut<usize>
    + Index<usize, Output = Self::Scalar>
    + Add<Self, Output = Self>
    + Add<Self::Scalar, Output = Self>
    + AddAssign<Self>
    + AddAssign<Self::Scalar>
    + Sub<Self, Output = Self>
    + Sub<Self::Scalar, Output = Self>
    + SubAssign<Self>
    + SubAssign<Self::Scalar>
    + Mul<Self, Output = Self>
    + Mul<Self::Scalar, Output = Self>
    + MulAssign<Self>
    + MulAssign<Self::Scalar>
    + BitAnd<Self, Output = Self>
    + BitAnd<Self::Scalar, Output = Self>
    + BitAndAssign<Self>
    + BitAndAssign<Self::Scalar>
    + BitOr<Self, Output = Self>
    + BitOr<Self::Scalar, Output = Self>
    + BitOrAssign<Self>
    + BitOrAssign<Self::Scalar>
    + BitXor<Self, Output = Self>
    + BitXor<Self::Scalar, Output = Self>
    + BitXorAssign<Self>
    + BitXorAssign<Self::Scalar>
    + core::marker::Sync
    + core::marker::Send
    + Not<Output = Self>
{
    type Scalar: Copy + Debug + core::marker::Sync + core::marker::Send;
    const WIDTH: usize;

    /// The type of the transmuted array representation. This is to make indexing operations easier.
    /// We are unable to use `&[Self::Scalar; Self::WIDTH]` because constants are not allowed.
    type ArrayRepresentation: Index<usize, Output = Self::Scalar> + IndexMut<usize> + Clone;

    /// The underlying intrinsic SIMD type.
    type UnderlyingType: Copy + Debug + core::marker::Sync + core::marker::Send;

    /// Initialize a vector with all elements set to zero
    unsafe fn zeroes() -> Self;

    /// Initialize a vector with all elements set to the same value
    unsafe fn set1(x: Self::Scalar) -> Self;

    /// Element-wise add between two vectors
    fn add(self, rhs: Self) -> Self;
    /// Element-wise subtract between two vectors
    fn sub(self, rhs: Self) -> Self;
    /// Element-wise multiply between two vectors
    fn mul(self, rhs: Self) -> Self;

    /// Binary and between two vectors
    fn bit_and(self, rhs: Self) -> Self;
    /// Binary or between two vectors
    fn bit_or(self, rhs: Self) -> Self;
    /// Binary xor between two vectors
    fn bit_xor(self, rhs: Self) -> Self;

    /// Binary not operation for a vector
    fn bit_not(self) -> Self;

    /// Element-wise absolute value
    fn abs(self) -> Self;

    /// Binary and not between two vectors `(!a) & b`
    fn and_not(self, rhs: Self) -> Self;

    /// Element-wise "blend" between two vectors. A is selected if the mask value
    /// is zero, and B is selected if the mask value is all 1's. undefined behavior if
    /// it's anything in  between. See note below.
    ///
    /// Note: SSE2 will select B only when all bits are 1, while SSE41 and AVX2 only
    /// check the high bit. To maintain portability ensure all bits are 1 when using
    /// blend. Results of comparison operations adhere to this.
    fn blendv(self, a: Self, b: Self) -> Self;

    /// Element-wise equality between two vectors. If two elements are equal, it returns all 1's
    /// in the corresponding element of the result, otherwise it returns all 0's.
    fn cmp_eq(self, rhs: Self) -> Self;

    /// Element-wise inequality between two vectors. If two elements are not equal, it returns all 1's
    /// in the corresponding element of the result, otherwise it returns all 0's.
    fn cmp_neq(self, rhs: Self) -> Self;

    /// Element-wise less than between two vectors. If the first element is less than the second element,
    /// it returns all 1's in the corresponding element of the result, otherwise it returns all 0's.
    fn cmp_lt(self, rhs: Self) -> Self;

    /// Element-wise less than or equal to between two vectors. If the first element is less than or equal to the second element,
    /// it returns all 1's in the corresponding element of the result, otherwise it returns all 0's.
    fn cmp_lte(self, rhs: Self) -> Self;

    /// Element-wise greater than between two vectors. If the first element is greater than the second element,
    /// it returns all 1's in the corresponding element of the result, otherwise it returns all 0's.
    fn cmp_gt(self, rhs: Self) -> Self;

    /// Element-wise greater than or equal to between two vectors. If the first element is greater than or equal to the second element,
    /// it returns all 1's in the corresponding element of the result, otherwise it returns all 0's.
    fn cmp_gte(self, rhs: Self) -> Self;

    /// Element-wise maximum between two vectors.
    fn max(self, rhs: Self) -> Self;

    /// Element-wise minimum between two vectors.
    fn min(self, rhs: Self) -> Self;

    /// Transmutes the vector into a array representation defined by `Self::ArrayRepresentation`.
    /// Please don't use this function directly unless necessary.
    #[inline(always)]
    unsafe fn transmute_into_array_ref(&self) -> &Self::ArrayRepresentation {
        core::mem::transmute(self)
    }

    /// Transmutes the vector into a mutable array representation defined by `Self::ArrayRepresentation`.
    /// Please don't use this function directly unless necessary.
    #[inline(always)]
    unsafe fn transmute_into_array_mut(&mut self) -> &mut Self::ArrayRepresentation {
        core::mem::transmute(self)
    }

    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self;
    unsafe fn as_array(&self) -> Self::ArrayRepresentation {
        self.transmute_into_array_ref().clone()
    }

    unsafe fn load_from_ptr(ptr: *const Self::Scalar) -> Self;
    unsafe fn copy_to_ptr(self, ptr: *mut Self::Scalar);

    unsafe fn underlying_value(self) -> Self::UnderlyingType;
    unsafe fn underlying_value_mut(&mut self) -> &mut Self::UnderlyingType;
    unsafe fn from_underlying_value(value: Self::UnderlyingType) -> Self;

    #[inline(always)]
    fn iter(&self) -> SimdArrayIterator<'_, Self> {
        SimdArrayIterator {
            simd: self,
            index: 0,
        }
    }

    #[inline(always)]
    fn iter_mut(&self) -> SimdArrayIterator<'_, Self> {
        SimdArrayIterator {
            simd: self,
            index: 0,
        }
    }
}

/// Operations shared by 16 and 32 bit int types
pub trait SimdInt:
    SimdBase + Shl<i32, Output = Self> + ShlAssign<i32> + Shr<i32, Output = Self> + ShrAssign<i32>
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
    SimdBase
    + Div<Self, Output = Self>
    + DivAssign<Self>
    + Div<Self::Scalar, Output = Self>
    + DivAssign<Self::Scalar>
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

pub struct SimdArrayIterator<'a, S: SimdBase> {
    simd: &'a S,
    index: usize,
}

impl<'a, S: SimdBase> Iterator for SimdArrayIterator<'a, S> {
    type Item = S::Scalar;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= S::WIDTH {
            return None;
        }

        let value = unsafe {
            let underlying_ptr = &self.simd.underlying_value() as *const S::UnderlyingType;
            let ptr_scalar = underlying_ptr as *const S::Scalar;
            let ptr = ptr_scalar.add(self.index);
            *ptr
        };

        self.index += 1;

        Some(value)
    }
}

pub struct SimdArrayMutIterator<'a, S: SimdBase> {
    simd: &'a S,
    index: usize,
}

impl<'a, S: SimdBase> Iterator for SimdArrayMutIterator<'a, S> {
    type Item = &'a mut S::Scalar;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= S::WIDTH {
            return None;
        }

        let value = unsafe {
            let underlying_ptr = &self.simd.underlying_value() as *const S::UnderlyingType;
            let ptr_scalar = underlying_ptr as *mut S::Scalar;
            let ptr = ptr_scalar.add(self.index);
            &mut *ptr
        };

        self.index += 1;

        Some(value)
    }
}
