use core::fmt::Debug;
use core::ops::*;

mod io;

mod iters;
pub use iters::*;

mod transmute;
pub use transmute::*;

mod specializations;
pub use specializations::*;

pub use io::InternalSimdBaseIo;
pub use io::SimdBaseIo;

pub trait SimdConsts: 'static + Copy + core::marker::Sync + core::marker::Send + Debug {
    type Scalar: Copy + Debug + core::marker::Sync + core::marker::Send;
    type HorizontalAddScalar: Copy + Debug + core::marker::Sync + core::marker::Send;
    const WIDTH: usize;

    /// The type of the transmuted array representation. This is to make indexing operations easier.
    /// We are unable to use `&[Self::Scalar; Self::WIDTH]` because constants are not allowed.
    type ArrayRepresentation: Index<usize, Output = Self::Scalar> + IndexMut<usize> + Clone;

    /// The underlying intrinsic SIMD type.
    type UnderlyingType: Copy + Debug + core::marker::Sync + core::marker::Send;
}

/// Operations shared by all SIMD types
pub trait SimdBaseOps:
    SimdConsts
    + IndexMut<usize>
    + Index<usize, Output = <Self as SimdConsts>::Scalar>
    + Add<Self, Output = Self>
    + Add<<Self as SimdConsts>::Scalar, Output = Self>
    + AddAssign<Self>
    + AddAssign<<Self as SimdConsts>::Scalar>
    + Sub<Self, Output = Self>
    + Sub<<Self as SimdConsts>::Scalar, Output = Self>
    + SubAssign<Self>
    + SubAssign<<Self as SimdConsts>::Scalar>
    + Mul<Self, Output = Self>
    + Mul<<Self as SimdConsts>::Scalar, Output = Self>
    + MulAssign<Self>
    + MulAssign<<Self as SimdConsts>::Scalar>
    + BitAnd<Self, Output = Self>
    + BitAnd<<Self as SimdConsts>::Scalar, Output = Self>
    + BitAndAssign<Self>
    + BitAndAssign<<Self as SimdConsts>::Scalar>
    + BitOr<Self, Output = Self>
    + BitOr<<Self as SimdConsts>::Scalar, Output = Self>
    + BitOrAssign<Self>
    + BitOrAssign<<Self as SimdConsts>::Scalar>
    + BitXor<Self, Output = Self>
    + BitXor<<Self as SimdConsts>::Scalar, Output = Self>
    + BitXorAssign<Self>
    + BitXorAssign<<Self as SimdConsts>::Scalar>
    + Not<Output = Self>
{
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

    /// Binary and not between two vectors `self & (!rhs)`
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

    /// Add every number in the vector together
    fn horizontal_add(self) -> Self::HorizontalAddScalar;
}

pub trait SimdBase: SimdBaseOps + SimdBaseIo + SimdIter {}
impl<T: SimdBaseOps + SimdBaseIo + SimdIter> SimdBase for T {}
