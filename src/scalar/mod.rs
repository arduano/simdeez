use super::*;

mod simd;
pub use self::overloads::*;
pub use self::simd::*;

// Newtypes for i16 vectors
// We have to do this to allow for overloading of
// __m128i etc
#[derive(Copy, Debug, Clone)]
pub struct I16x1(pub i16);
impl_simd_base_overloads!(I16x1);
impl_simd_small_int_overloads!(I16x1);

impl SimdBase for I16x1 {
    const WIDTH: usize = 1;
    type Scalar = i16;
    type ArrayRepresentation = [i16; 1];

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

impl SimdSmallInt for I16x1 {
    unsafe fn shl(self, rhs: i32) -> Self {
        todo!()
    }

    unsafe fn shr(self, rhs: i32) -> Self {
        todo!()
    }
}

// Newtypes for i32 vectors
// We have to do this to allow for overloading of
// __m128i etc
#[derive(Copy, Debug, Clone)]
pub struct I32x1(pub i32);
impl_simd_base_overloads!(I32x1);
impl_simd_small_int_overloads!(I32x1);

impl SimdBase for I32x1 {
    const WIDTH: usize = 1;
    type Scalar = i32;
    type ArrayRepresentation = [i32; 1];

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

impl SimdSmallInt for I32x1 {
    unsafe fn shl(self, rhs: i32) -> Self {
        todo!()
    }

    unsafe fn shr(self, rhs: i32) -> Self {
        todo!()
    }
}

// Newtypes for i64 vectors
// We have to do this to allow for overloading of
// __m128i etc
#[derive(Copy, Debug, Clone)]
pub struct I64x1(pub i64);
impl_simd_base_overloads!(I64x1);
impl_simd_large_int_overloads!(I64x1);

impl SimdBase for I64x1 {
    const WIDTH: usize = 1;
    type Scalar = i64;
    type ArrayRepresentation = [i64; 1];

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

impl SimdInt64 for I64x1 {
    unsafe fn shl(self, rhs: i32) -> Self {
        todo!()
    }

    unsafe fn shr(self, rhs: i32) -> Self {
        todo!()
    }
}

// Newtypes for f32 vectors
// We have to do this to allow for overloading of
// __m128 etc
#[derive(Copy, Debug, Clone)]
pub struct F32x1(pub f32);
impl_simd_base_overloads!(F32x1);
impl_simd_float_overloads!(F32x1);

impl SimdBase for F32x1 {
    const WIDTH: usize = 1;
    type Scalar = f32;
    type ArrayRepresentation = [f32; 1];

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

impl SimdFloat for F32x1 {
    unsafe fn div(self, rhs: Self) -> Self {
        todo!()
    }
}

// Newtypes for f64 vectors
#[derive(Copy, Debug, Clone)]
pub struct F64x1(pub f64);
impl_simd_base_overloads!(F64x1);
impl_simd_float_overloads!(F64x1);

impl SimdBase for F64x1 {
    const WIDTH: usize = 1;
    type Scalar = f64;
    type ArrayRepresentation = [f64; 1];

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

impl SimdFloat for F64x1 {
    unsafe fn div(self, rhs: Self) -> Self {
        todo!()
    }
}
