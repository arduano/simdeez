use super::*;

mod simd;
pub use self::overloads::*;
pub use self::simd::*;

// Newtypes for i16 vectors
// We have to do this to allow for overloading of
// __m128i etc
#[derive(Copy, Clone)]
pub struct I16x1(pub i16);
impl_simd_base_overloads!(I16x1);
impl_simd_int_overloads!(I16x1);

impl SimdBase for I16x1 {
    const WIDTH: usize = 1;
    type Scalar = i16;
    type ArrayRepresentation = [i16; 1];
    type UnderlyingType = i16;

    unsafe fn zeroes() -> Self {
        todo!()
    }

    unsafe fn set1(x: Self::Scalar) -> Self {
        todo!()
    }

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

    unsafe fn abs(self) -> Self {
        todo!()
    }

    unsafe fn and_not(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn blendv(self, a: Self, b: Self) -> Self {
        todo!()
    }

    unsafe fn cmp_eq(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn cmp_neq(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn cmp_lt(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn cmp_lte(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn cmp_gt(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn cmp_gte(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn max(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn min(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        todo!()
    }
}

impl SimdInt for I16x1 {
    unsafe fn shl(self, rhs: i32) -> Self {
        todo!()
    }

    unsafe fn shr(self, rhs: i32) -> Self {
        todo!()
    }
}

impl SimdInt16 for I16x1 {}

// Newtypes for i32 vectors
// We have to do this to allow for overloading of
// __m128i etc
#[derive(Copy, Clone)]
pub struct I32x1(pub i32);
impl_simd_base_overloads!(I32x1);
impl_simd_int_overloads!(I32x1);

impl SimdBase for I32x1 {
    const WIDTH: usize = 1;
    type Scalar = i32;
    type ArrayRepresentation = [i32; 1];
    type UnderlyingType = i32;

    unsafe fn zeroes() -> Self {
        todo!()
    }

    unsafe fn set1(x: Self::Scalar) -> Self {
        todo!()
    }

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

    unsafe fn abs(self) -> Self {
        todo!()
    }

    unsafe fn and_not(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn blendv(self, a: Self, b: Self) -> Self {
        todo!()
    }

    unsafe fn cmp_eq(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn cmp_neq(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn cmp_lt(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn cmp_lte(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn cmp_gt(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn cmp_gte(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn max(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn min(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        todo!()
    }
}

impl SimdInt for I32x1 {
    unsafe fn shl(self, rhs: i32) -> Self {
        todo!()
    }

    unsafe fn shr(self, rhs: i32) -> Self {
        todo!()
    }
}

impl SimdInt32 for I32x1 {
    type SimdF32 = F32x1;

    unsafe fn bitcast_f32(self) -> Self::SimdF32 {
        todo!()
    }

    unsafe fn cast_f32(self) -> Self::SimdF32 {
        todo!()
    }
}

// Newtypes for i64 vectors
// We have to do this to allow for overloading of
// __m128i etc
#[derive(Copy, Clone)]
pub struct I64x1(pub i64);
impl_simd_base_overloads!(I64x1);
impl_simd_int_overloads!(I64x1);

impl SimdBase for I64x1 {
    const WIDTH: usize = 1;
    type Scalar = i64;
    type ArrayRepresentation = [i64; 1];
    type UnderlyingType = i64;

    unsafe fn zeroes() -> Self {
        todo!()
    }

    unsafe fn set1(x: Self::Scalar) -> Self {
        todo!()
    }

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

    unsafe fn abs(self) -> Self {
        todo!()
    }

    unsafe fn and_not(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn blendv(self, a: Self, b: Self) -> Self {
        todo!()
    }

    unsafe fn cmp_eq(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn cmp_neq(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn cmp_lt(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn cmp_lte(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn cmp_gt(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn cmp_gte(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn max(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn min(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        todo!()
    }
}

impl SimdInt for I64x1 {
    unsafe fn shl(self, rhs: i32) -> Self {
        todo!()
    }

    unsafe fn shr(self, rhs: i32) -> Self {
        todo!()
    }
}

impl SimdInt64 for I64x1 {
    type SimdF64 = F64x1;

    unsafe fn bitcast_f64(self) -> Self::SimdF64 {
        todo!()
    }

    unsafe fn cast_f64(self) -> Self::SimdF64 {
        todo!()
    }
}

// Newtypes for f32 vectors
// We have to do this to allow for overloading of
// __m128 etc
#[derive(Copy, Clone)]
pub struct F32x1(pub f32);
impl_simd_base_overloads!(F32x1);
impl_simd_float_overloads!(F32x1);

impl SimdBase for F32x1 {
    const WIDTH: usize = 1;
    type Scalar = f32;
    type ArrayRepresentation = [f32; 1];
    type UnderlyingType = f32;

    unsafe fn zeroes() -> Self {
        todo!()
    }

    unsafe fn set1(x: Self::Scalar) -> Self {
        todo!()
    }

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

    unsafe fn abs(self) -> Self {
        todo!()
    }

    unsafe fn and_not(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn blendv(self, a: Self, b: Self) -> Self {
        todo!()
    }

    unsafe fn cmp_eq(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn cmp_neq(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn cmp_lt(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn cmp_lte(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn cmp_gt(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn cmp_gte(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn max(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn min(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        todo!()
    }
}

impl SimdFloat for F32x1 {
    unsafe fn div(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn ceil(self) -> Self {
        todo!()
    }

    unsafe fn floor(self) -> Self {
        todo!()
    }

    unsafe fn round(self) -> Self {
        todo!()
    }

    unsafe fn fast_ceil(self) -> Self {
        todo!()
    }

    unsafe fn fast_floor(self) -> Self {
        todo!()
    }

    unsafe fn fast_round(self) -> Self {
        todo!()
    }

    unsafe fn mul_add(self, a: Self, b: Self) -> Self {
        todo!()
    }

    unsafe fn mul_sub(self, a: Self, b: Self) -> Self {
        todo!()
    }

    unsafe fn neg_mul_add(self, a: Self, b: Self) -> Self {
        todo!()
    }

    unsafe fn neg_mul_sub(self, a: Self, b: Self) -> Self {
        todo!()
    }

    unsafe fn horizontal_add(self) -> Self::Scalar {
        todo!()
    }

    unsafe fn sqrt(self) -> Self {
        todo!()
    }

    unsafe fn rsqrt(self) -> Self {
        todo!()
    }
}

impl SimdFloat32 for F32x1 {
    type SimdI32 = I32x1;

    unsafe fn bitcast_i32(self) -> Self::SimdI32 {
        todo!()
    }

    unsafe fn cast_i32(self) -> Self::SimdI32 {
        todo!()
    }

    unsafe fn fast_inverse(self) -> Self {
        todo!()
    }
}

// Newtypes for f64 vectors
#[derive(Copy, Clone)]
pub struct F64x1(pub f64);
impl_simd_base_overloads!(F64x1);
impl_simd_float_overloads!(F64x1);

impl SimdBase for F64x1 {
    const WIDTH: usize = 1;
    type Scalar = f64;
    type ArrayRepresentation = [f64; 1];
    type UnderlyingType = f64;

    unsafe fn zeroes() -> Self {
        todo!()
    }

    unsafe fn set1(x: Self::Scalar) -> Self {
        todo!()
    }

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

    unsafe fn abs(self) -> Self {
        todo!()
    }

    unsafe fn and_not(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn blendv(self, a: Self, b: Self) -> Self {
        todo!()
    }

    unsafe fn cmp_eq(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn cmp_neq(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn cmp_lt(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn cmp_lte(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn cmp_gt(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn cmp_gte(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn max(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn min(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        todo!()
    }
}

impl SimdFloat for F64x1 {
    unsafe fn div(self, rhs: Self) -> Self {
        todo!()
    }

    unsafe fn ceil(self) -> Self {
        todo!()
    }

    unsafe fn floor(self) -> Self {
        todo!()
    }

    unsafe fn round(self) -> Self {
        todo!()
    }

    unsafe fn fast_ceil(self) -> Self {
        todo!()
    }

    unsafe fn fast_floor(self) -> Self {
        todo!()
    }

    unsafe fn fast_round(self) -> Self {
        todo!()
    }

    unsafe fn mul_add(self, a: Self, b: Self) -> Self {
        todo!()
    }

    unsafe fn mul_sub(self, a: Self, b: Self) -> Self {
        todo!()
    }

    unsafe fn neg_mul_add(self, a: Self, b: Self) -> Self {
        todo!()
    }

    unsafe fn neg_mul_sub(self, a: Self, b: Self) -> Self {
        todo!()
    }

    unsafe fn horizontal_add(self) -> Self::Scalar {
        todo!()
    }

    unsafe fn sqrt(self) -> Self {
        todo!()
    }

    unsafe fn rsqrt(self) -> Self {
        todo!()
    }
}

impl SimdFloat64 for F64x1 {
    type SimdI64 = I64x1;

    unsafe fn bitcast_i64(self) -> Self::SimdI64 {
        todo!()
    }

    unsafe fn cast_i64(self) -> Self::SimdI64 {
        todo!()
    }
}
