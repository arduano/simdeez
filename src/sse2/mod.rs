use super::*;
#[cfg(target_arch = "x86")]
use core::arch::x86::*;
use core::mem;

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;
use core::slice::Iter;

mod simd;
pub use self::overloads::*;
pub use self::simd::*;

#[derive(Copy, Debug, Clone)]
pub struct I16x8(pub __m128i);
impl_simd_base_overloads!(I16x8);
impl_simd_small_int_overloads!(I16x8);

impl SimdBase for I16x8 {
    const WIDTH: usize = 8;
    type Scalar = i16;
    type ArrayRepresentation = [i16; 8];

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

impl SimdSmallInt for I16x8 {
    unsafe fn shl(self, rhs: i32) -> Self {
        todo!()
    }

    unsafe fn shr(self, rhs: i32) -> Self {
        todo!()
    }
}

#[derive(Copy, Debug, Clone)]
pub struct I32x4(pub __m128i);
impl_simd_base_overloads!(I32x4);
impl_simd_small_int_overloads!(I32x4);

impl SimdBase for I32x4 {
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

impl SimdSmallInt for I32x4 {
    unsafe fn shl(self, rhs: i32) -> Self {
        todo!()
    }

    unsafe fn shr(self, rhs: i32) -> Self {
        todo!()
    }
}

#[derive(Copy, Debug, Clone)]
pub struct I64x2(pub __m128i);
impl_simd_base_overloads!(I64x2);
impl_simd_large_int_overloads!(I64x2);

impl SimdBase for I64x2 {
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

impl SimdInt64 for I64x2 {
    unsafe fn shl(self, rhs: i32) -> Self {
        todo!()
    }

    unsafe fn shr(self, rhs: i32) -> Self {
        todo!()
    }
}

#[derive(Copy, Debug, Clone)]
pub struct F32x4(pub __m128);
impl_simd_base_overloads!(F32x4);
impl_simd_float_overloads!(F32x4);

impl SimdBase for F32x4 {
    const WIDTH: usize = 4;
    type Scalar = f32;
    type ArrayRepresentation = [f32; 4];

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

impl SimdFloat for F32x4 {
    unsafe fn div(self, rhs: Self) -> Self {
        todo!()
    }
}

#[derive(Copy, Debug, Clone)]
pub struct F64x2(pub __m128d);
impl_simd_base_overloads!(F64x2);
impl_simd_float_overloads!(F64x2);

impl SimdBase for F64x2 {
    const WIDTH: usize = 2;
    type Scalar = f64;
    type ArrayRepresentation = [f64; 2];

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

impl SimdFloat for F64x2 {
    unsafe fn div(self, rhs: Self) -> Self {
        todo!()
    }
}
