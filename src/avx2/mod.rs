use super::*;
use core::arch::x86_64::*;
use core::mem;

mod simd;
pub use self::overloads::*;
pub use self::simd::*;

#[derive(Copy, Debug, Clone)]
pub struct I16x16(pub __m256i);
impl_simd_base_overloads!(I16x16);
impl_simd_small_int_overloads!(I16x16);

impl SimdBase for I16x16 {
    const WIDTH: usize = 16;
    type Scalar = i16;
    type ArrayRepresentation = [i16; 16];

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

impl SimdSmallInt for I16x16 {
    unsafe fn shl(self, rhs: i32) -> Self {
        todo!()
    }

    unsafe fn shr(self, rhs: i32) -> Self {
        todo!()
    }
}

#[derive(Copy, Debug, Clone)]
pub struct I32x8(pub __m256i);
impl_simd_base_overloads!(I32x8);
impl_simd_small_int_overloads!(I32x8);

impl SimdBase for I32x8 {
    const WIDTH: usize = 8;
    type Scalar = i32;
    type ArrayRepresentation = [i32; 8];

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

impl SimdSmallInt for I32x8 {
    unsafe fn shl(self, rhs: i32) -> Self {
        todo!()
    }

    unsafe fn shr(self, rhs: i32) -> Self {
        todo!()
    }
}

#[derive(Copy, Debug, Clone)]
pub struct I64x4(pub __m256i);
impl_simd_base_overloads!(I64x4);
impl_simd_large_int_overloads!(I64x4);

impl SimdBase for I64x4 {
    const WIDTH: usize = 4;
    type Scalar = i64;
    type ArrayRepresentation = [i64; 4];

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

impl SimdInt64 for I64x4 {
    unsafe fn shl(self, rhs: i32) -> Self {
        todo!()
    }

    unsafe fn shr(self, rhs: i32) -> Self {
        todo!()
    }
}

#[derive(Copy, Debug, Clone)]
pub struct F32x8(pub __m256);
impl_simd_base_overloads!(F32x8);
impl_simd_float_overloads!(F32x8);

impl SimdBase for F32x8 {
    const WIDTH: usize = 8;
    type Scalar = f32;
    type ArrayRepresentation = [f32; 8];

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

impl SimdFloat for F32x8 {
    unsafe fn div(self, rhs: Self) -> Self {
        todo!()
    }
}

#[derive(Copy, Debug, Clone)]
pub struct F64x4(pub __m256d);
impl_simd_base_overloads!(F64x4);
impl_simd_float_overloads!(F64x4);

impl SimdBase for F64x4 {
    const WIDTH: usize = 4;
    type Scalar = f64;
    type ArrayRepresentation = [f64; 4];

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

impl SimdFloat for F64x4 {
    unsafe fn div(self, rhs: Self) -> Self {
        todo!()
    }
}
