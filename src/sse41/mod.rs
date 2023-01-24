pub use super::*;
#[cfg(target_arch = "x86")]
use core::arch::x86::*;
use core::mem;

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

mod simd;
pub use self::simd::*;
pub use self::sse2::*;

#[derive(Copy, Debug, Clone)]
pub struct I64x2_41(pub __m128i);
impl_simd_base_overloads!(I64x2_41);
impl_simd_large_int_overloads!(I64x2_41);

impl SimdBase for I64x2_41 {
    const WIDTH: usize = 2;
    type Scalar = i64;
    type ArrayRepresentation = [i64; 2];

    unsafe fn add(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn sub(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn mul(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn bit_and(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn bit_or(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn bit_xor(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn bit_not(self) -> Self {
        todo!()
    }
}

impl SimdInt64 for I64x2_41 {
    unsafe fn shl(self, rhs: i32) -> Self {
        todo!()
    }

    unsafe fn shr(self, rhs: i32) -> Self {
        todo!()
    }
}

#[derive(Copy, Debug, Clone)]
pub struct I32x4_41(pub __m128i);
impl_simd_base_overloads!(I32x4_41);
impl_simd_small_int_overloads!(I32x4_41);

impl SimdBase for I32x4_41 {
    const WIDTH: usize = 4;
    type Scalar = i32;
    type ArrayRepresentation = [i32; 4];

    unsafe fn add(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn sub(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn mul(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn bit_and(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn bit_or(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn bit_xor(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn bit_not(self) -> Self {
        todo!()
    }
}

impl SimdSmallInt for I32x4_41 {
    unsafe fn shl(self, rhs: i32) -> Self {
        todo!()
    }

    unsafe fn shr(self, rhs: i32) -> Self {
        todo!()
    }
}
