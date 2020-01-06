pub use super::*;
use core::mem;
#[cfg(target_arch = "x86")]
use core::arch::x86::*;

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

mod sse41;
mod overloads;
pub use self::overloads::*;
pub use self::sse41::*;
pub use self::sse2::*;

#[derive(Copy, Debug, Clone)]
pub struct I64x2_41(pub __m128i);
impl SimdBase<I64x2_41, i64> for I64x2_41 {}
#[derive(Copy, Debug, Clone)]
pub struct I32x4_41(pub __m128i);
impl SimdBase<I32x4_41, i32> for I32x4_41 {}
impl SimdSmallInt<I32x4_41, i32> for I32x4_41 {}