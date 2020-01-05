use super::*;
use core::mem;

mod overloads;
pub use self::overloads::*;

#[derive(Copy, Debug, Clone)]
pub struct I64x2_41(pub __m128i);
impl SimdBase<I64x2_41, i64> for I64x2_41 {}
#[derive(Copy, Debug, Clone)]
pub struct I32x4_41(pub __m128i);
impl SimdBase<I32x4_41, i32> for I32x4_41 {}
impl SimdSmallInt<I32x4_41, i32> for I32x4_41 {}