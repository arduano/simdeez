use core::fmt::Debug;
use core::ops::*;

/// Operations shared by all SIMD types
pub trait SimdBase:
    Copy
    + Debug
    + IndexMut<usize>
    + Index<usize, Output = Self::Scalar>
    + Add<Self, Output = Self>
    + AddAssign<Self>
    + Sub<Self, Output = Self>
    + SubAssign<Self>
    + Mul<Self, Output = Self>
    + MulAssign<Self>
    + BitAnd<Self, Output = Self>
    + BitAndAssign<Self>
    + BitOr<Self, Output = Self>
    + BitOrAssign<Self>
    + BitXor<Self, Output = Self>
    + BitXorAssign<Self>
    + core::marker::Sync
    + core::marker::Send
    + Not<Output = Self>
{
    type Scalar: Copy + Debug + core::marker::Sync + core::marker::Send;
    const WIDTH: usize;

    /// The type of the transmuted array representation. This is to make indexing operations easier.
    /// We are unable to use `&[Self::Scalar; Self::WIDTH]` because constants are not allowed.
    type ArrayRepresentation: Index<usize, Output = Self::Scalar> + IndexMut<usize>;

    /// Element-wise add between two vectors
    unsafe fn add(self, rhs: Self) -> Self;
    /// Element-wise subtract between two vectors
    unsafe fn sub(self, rhs: Self) -> Self;
    /// Element-wise multiply between two vectors
    unsafe fn mul(self, rhs: Self) -> Self;

    /// Binary and between two vectors
    unsafe fn bit_and(self, rhs: Self) -> Self;
    /// Binary or between two vectors
    unsafe fn bit_or(self, rhs: Self) -> Self;
    /// Binary xor between two vectors
    unsafe fn bit_xor(self, rhs: Self) -> Self;

    /// Binary not operation for a vector
    unsafe fn bit_not(self) -> Self;

    /// Element-wise absolute value
    unsafe fn abs(self) -> Self;

    /// Element-wise and not between two vectors
    unsafe fn and_not(self, rhs: Self) -> Self;

    /// Element-wise "blend" between two vectors. A is selected if the mask value
    /// is zero, and B is selected if the mask value is all 1's. undefined behavior if
    /// it's anything in  between. See note below.
    ///
    /// Note: SSE2 will select B only when all bits are 1, while SSE41 and AVX2 only
    /// check the high bit. To maintain portability ensure all bits are 1 when using
    /// blend. Results of comparison operations adhere to this.
    unsafe fn blendv(self, a: Self, b: Self) -> Self;

    /// Element-wise equality between two vectors. If two elements are equal, it returns all 1's
    /// in the corresponding element of the result, otherwise it returns all 0's.
    unsafe fn cmp_eq(self, rhs: Self) -> Self;

    /// Element-wise inequality between two vectors. If two elements are not equal, it returns all 1's
    /// in the corresponding element of the result, otherwise it returns all 0's.
    unsafe fn cmp_neq(self, rhs: Self) -> Self;

    /// Element-wise less than between two vectors. If the first element is less than the second element,
    /// it returns all 1's in the corresponding element of the result, otherwise it returns all 0's.
    unsafe fn cmp_lt(self, rhs: Self) -> Self;

    /// Element-wise less than or equal to between two vectors. If the first element is less than or equal to the second element,
    /// it returns all 1's in the corresponding element of the result, otherwise it returns all 0's.
    unsafe fn cmp_lte(self, rhs: Self) -> Self;

    /// Element-wise greater than between two vectors. If the first element is greater than the second element,
    /// it returns all 1's in the corresponding element of the result, otherwise it returns all 0's.
    unsafe fn cmp_gt(self, rhs: Self) -> Self;

    /// Element-wise greater than or equal to between two vectors. If the first element is greater than or equal to the second element,
    /// it returns all 1's in the corresponding element of the result, otherwise it returns all 0's.
    unsafe fn cmp_gte(self, rhs: Self) -> Self;
    
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
}

/// Operations shared by 16 and 32 bit int types
pub trait SimdSmallInt:
    SimdBase + Shl<i32, Output = Self> + ShlAssign<i32> + Shr<i32, Output = Self> + ShrAssign<i32>
{
    /// Shift each value left by n bits
    unsafe fn shl(self, rhs: i32) -> Self;

    /// Shift each value right by n bits
    unsafe fn shr(self, rhs: i32) -> Self;
}

/// Operations shared by 16 and 32 bit int types
pub trait SimdInt32: SimdSmallInt {
    /// Bit cast to f32.
    /// This function is only used for compilation and does not generate any instructions, thus it has zero latency.
    unsafe fn bitcast_f32(self, rhs: i32) -> Self;
}

/// Operations shared by 64 bt int types
pub trait SimdInt64: SimdBase {
    /// Shift each value left by n bits. This operations is missing in most implementations
    /// and is emulated here under SSE2, SSE4.1, and AVX2.
    unsafe fn shl(self, rhs: i32) -> Self;

    /// Shift each value right by n bits. This operations is missing in most implementations
    /// and is emulated here under SSE2, SSE4.1, and AVX2.
    unsafe fn shr(self, rhs: i32) -> Self;

    /// Bit cast to f64
    /// This function is only used for compilation and does not generate any instructions, thus it has zero latency.
    unsafe fn bitcast_f64(self, rhs: i32) -> Self;
}

/// Operations shared by f32 and f64 floating point types
pub trait SimdFloat: SimdBase + Div<Self, Output = Self> {
    /// Element-wise divide between two vectors
    unsafe fn div(self, rhs: Self) -> Self;
}

pub trait SimdFloat32: SimdFloat {
    /// Bit cast to i32
    /// This function is only used for compilation and does not generate any instructions, thus it has zero latency.
    unsafe fn bitcast_i32(self, rhs: i32) -> Self;

    /// Bit cast to f64
    /// This function is only used for compilation and does not generate any instructions, thus it has zero latency.
    unsafe fn bitcast_f64(self, rhs: i32) -> Self;
}

pub trait SimdFloat64: SimdFloat {
    /// Bit cast to i64
    /// This function is only used for compilation and does not generate any instructions, thus it has zero latency.
    unsafe fn bitcast_i64(self, rhs: i32) -> Self;

    /// Bit cast to f32
    /// This function is only used for compilation and does not generate any instructions, thus it has zero latency.
    unsafe fn bitcast_f32(self, rhs: i32) -> Self;
}
