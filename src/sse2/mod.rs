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

#[derive(Copy, Clone)]
pub struct I16x8(pub __m128i);
impl_simd_base_overloads!(I16x8);
impl_simd_int_overloads!(I16x8);

impl SimdBase for I16x8 {
    const WIDTH: usize = 8;
    type Scalar = i16;
    type ArrayRepresentation = [i16; 8];
    type UnderlyingType = __m128i;

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

    unsafe fn load_from_ptr(ptr: *const Self::Scalar) -> Self {
        todo!()
    }

    unsafe fn copy_to_ptr(self, ptr: *mut Self::Scalar) {
        todo!()
    }

    unsafe fn underlying_value(self) -> Self::UnderlyingType {
        todo!()
    }

    unsafe fn underlying_value_mut(&mut self) -> &mut Self::UnderlyingType {
        todo!()
    }

    unsafe fn from_underlying_value(value: Self::UnderlyingType) -> Self {
        todo!()
    }
}

impl SimdInt for I16x8 {
    unsafe fn shl(self, rhs: i32) -> Self {
        todo!()
    }

    unsafe fn shr(self, rhs: i32) -> Self {
        todo!()
    }
}

impl SimdInt16 for I16x8 {}

#[derive(Copy, Clone)]
pub struct I32x4(pub __m128i);
impl_simd_base_overloads!(I32x4);
impl_simd_int_overloads!(I32x4);

impl SimdBase for I32x4 {
    const WIDTH: usize = 4;
    type Scalar = i32;
    type ArrayRepresentation = [i32; 4];
    type UnderlyingType = __m128i;

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

    unsafe fn load_from_ptr(ptr: *const Self::Scalar) -> Self {
        todo!()
    }

    unsafe fn copy_to_ptr(self, ptr: *mut Self::Scalar) {
        todo!()
    }

    unsafe fn underlying_value(self) -> Self::UnderlyingType {
        todo!()
    }

    unsafe fn underlying_value_mut(&mut self) -> &mut Self::UnderlyingType {
        todo!()
    }

    unsafe fn from_underlying_value(value: Self::UnderlyingType) -> Self {
        todo!()
    }
}

impl SimdInt for I32x4 {
    unsafe fn shl(self, rhs: i32) -> Self {
        todo!()
    }

    unsafe fn shr(self, rhs: i32) -> Self {
        todo!()
    }
}

impl SimdInt32 for I32x4 {
    type SimdF32 = F32x4;

    unsafe fn bitcast_f32(self) -> Self::SimdF32 {
        todo!()
    }

    unsafe fn cast_f32(self) -> Self::SimdF32 {
        todo!()
    }
}

#[derive(Copy, Clone)]
pub struct I64x2(pub __m128i);
impl_simd_base_overloads!(I64x2);
impl_simd_int_overloads!(I64x2);

impl SimdBase for I64x2 {
    const WIDTH: usize = 2;
    type Scalar = i64;
    type ArrayRepresentation = [i64; 2];
    type UnderlyingType = __m128i;

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

    unsafe fn load_from_ptr(ptr: *const Self::Scalar) -> Self {
        todo!()
    }

    unsafe fn copy_to_ptr(self, ptr: *mut Self::Scalar) {
        todo!()
    }

    unsafe fn underlying_value(self) -> Self::UnderlyingType {
        todo!()
    }

    unsafe fn underlying_value_mut(&mut self) -> &mut Self::UnderlyingType {
        todo!()
    }

    unsafe fn from_underlying_value(value: Self::UnderlyingType) -> Self {
        todo!()
    }
}

impl SimdInt for I64x2 {
    unsafe fn shl(self, rhs: i32) -> Self {
        todo!()
    }

    unsafe fn shr(self, rhs: i32) -> Self {
        todo!()
    }
}

impl SimdInt64 for I64x2 {
    type SimdF64 = F64x2;

    unsafe fn bitcast_f64(self) -> Self::SimdF64 {
        todo!()
    }

    unsafe fn cast_f64(self) -> Self::SimdF64 {
        todo!()
    }
}

#[derive(Copy, Clone)]
pub struct F32x4(pub __m128);
impl_simd_base_overloads!(F32x4);
impl_simd_float_overloads!(F32x4);

impl SimdBase for F32x4 {
    const WIDTH: usize = 4;
    type Scalar = f32;
    type ArrayRepresentation = [f32; 4];
    type UnderlyingType = __m128;

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

    unsafe fn load_from_ptr(ptr: *const Self::Scalar) -> Self {
        todo!()
    }

    unsafe fn copy_to_ptr(self, ptr: *mut Self::Scalar) {
        todo!()
    }

    unsafe fn underlying_value(self) -> Self::UnderlyingType {
        todo!()
    }

    unsafe fn underlying_value_mut(&mut self) -> &mut Self::UnderlyingType {
        todo!()
    }

    unsafe fn from_underlying_value(value: Self::UnderlyingType) -> Self {
        todo!()
    }
}

impl SimdFloat for F32x4 {
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

impl SimdFloat32 for F32x4 {
    type SimdI32 = I32x4;

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

#[derive(Copy, Clone)]
pub struct F64x2(pub __m128d);
impl_simd_base_overloads!(F64x2);
impl_simd_float_overloads!(F64x2);

impl SimdBase for F64x2 {
    const WIDTH: usize = 2;
    type Scalar = f64;
    type ArrayRepresentation = [f64; 2];
    type UnderlyingType = __m128d;

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

    unsafe fn load_from_ptr(ptr: *const Self::Scalar) -> Self {
        todo!()
    }

    unsafe fn copy_to_ptr(self, ptr: *mut Self::Scalar) {
        todo!()
    }

    unsafe fn underlying_value(self) -> Self::UnderlyingType {
        todo!()
    }

    unsafe fn underlying_value_mut(&mut self) -> &mut Self::UnderlyingType {
        todo!()
    }

    unsafe fn from_underlying_value(value: Self::UnderlyingType) -> Self {
        todo!()
    }
}

impl SimdFloat for F64x2 {
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

impl SimdFloat64 for F64x2 {
    type SimdI64 = I64x2;

    unsafe fn bitcast_i64(self) -> Self::SimdI64 {
        todo!()
    }

    unsafe fn cast_i64(self) -> Self::SimdI64 {
        todo!()
    }
}
