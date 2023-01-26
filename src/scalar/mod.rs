use super::*;

mod simd;
pub use self::overloads::*;
pub use self::simd::*;

use crate::libm_ext::FloatExt;

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

    #[inline(always)]
    unsafe fn zeroes() -> Self {
        I16x1(0)
    }

    #[inline(always)]
    unsafe fn set1(x: Self::Scalar) -> Self {
        I16x1(x)
    }

    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        I16x1(self.0.wrapping_add(rhs.0))
    }

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        I16x1(self.0.wrapping_sub(rhs.0))
    }

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        I16x1(self.0.wrapping_mul(rhs.0))
    }

    #[inline(always)]
    fn bit_and(self, rhs: Self) -> Self {
        I16x1(self.0 & rhs.0)
    }

    #[inline(always)]
    fn bit_or(self, rhs: Self) -> Self {
        I16x1(self.0 | rhs.0)
    }

    #[inline(always)]
    fn bit_xor(self, rhs: Self) -> Self {
        I16x1(self.0 ^ rhs.0)
    }

    #[inline(always)]
    fn bit_not(self) -> Self {
        I16x1(!self.0)
    }

    #[inline(always)]
    fn abs(self) -> Self {
        I16x1(self.0.abs())
    }

    #[inline(always)]
    fn and_not(self, rhs: Self) -> Self {
        I16x1(!self.0 & rhs.0)
    }

    #[inline(always)]
    fn blendv(self, a: Self, b: Self) -> Self {
        I16x1(if self.0 != 0 { a.0 } else { b.0 })
    }

    #[inline(always)]
    fn cmp_eq(self, rhs: Self) -> Self {
        I16x1(if self.0 == rhs.0 { -1 } else { 0 })
    }

    #[inline(always)]
    fn cmp_neq(self, rhs: Self) -> Self {
        I16x1(if self.0 != rhs.0 { -1 } else { 0 })
    }

    #[inline(always)]
    fn cmp_lt(self, rhs: Self) -> Self {
        I16x1(if self.0 < rhs.0 { -1 } else { 0 })
    }

    #[inline(always)]
    fn cmp_lte(self, rhs: Self) -> Self {
        I16x1(if self.0 <= rhs.0 { -1 } else { 0 })
    }

    #[inline(always)]
    fn cmp_gt(self, rhs: Self) -> Self {
        I16x1(if self.0 > rhs.0 { -1 } else { 0 })
    }

    #[inline(always)]
    fn cmp_gte(self, rhs: Self) -> Self {
        I16x1(if self.0 >= rhs.0 { -1 } else { 0 })
    }

    #[inline(always)]
    fn max(self, rhs: Self) -> Self {
        I16x1(self.0.max(rhs.0))
    }

    #[inline(always)]
    fn min(self, rhs: Self) -> Self {
        I16x1(self.0.min(rhs.0))
    }

    #[inline(always)]
    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        I16x1(array[0])
    }

    #[inline(always)]
    unsafe fn load_from_ptr(ptr: *const Self::Scalar) -> Self {
        I16x1(*ptr)
    }

    #[inline(always)]
    unsafe fn copy_to_ptr(self, ptr: *mut Self::Scalar) {
        *ptr = self.0;
    }

    #[inline(always)]
    unsafe fn underlying_value(self) -> Self::UnderlyingType {
        self.0
    }

    #[inline(always)]
    unsafe fn underlying_value_mut(&mut self) -> &mut Self::UnderlyingType {
        &mut self.0
    }

    #[inline(always)]
    unsafe fn from_underlying_value(value: Self::UnderlyingType) -> Self {
        I16x1(value)
    }
}

impl SimdInt for I16x1 {
    #[inline(always)]
    fn shl(self, rhs: i32) -> Self {
        I16x1(self.0 << rhs)
    }

    #[inline(always)]
    fn shr(self, rhs: i32) -> Self {
        I16x1(self.0 >> rhs)
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

    #[inline(always)]
    unsafe fn zeroes() -> Self {
        I32x1(0)
    }

    #[inline(always)]
    unsafe fn set1(x: Self::Scalar) -> Self {
        I32x1(x)
    }

    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        I32x1(self.0.wrapping_add(rhs.0))
    }

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        I32x1(self.0.wrapping_sub(rhs.0))
    }

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        I32x1(self.0.wrapping_mul(rhs.0))
    }

    #[inline(always)]
    fn bit_and(self, rhs: Self) -> Self {
        I32x1(self.0 & rhs.0)
    }

    #[inline(always)]
    fn bit_or(self, rhs: Self) -> Self {
        I32x1(self.0 | rhs.0)
    }

    #[inline(always)]
    fn bit_xor(self, rhs: Self) -> Self {
        I32x1(self.0 ^ rhs.0)
    }

    #[inline(always)]
    fn bit_not(self) -> Self {
        I32x1(!self.0)
    }

    #[inline(always)]
    fn abs(self) -> Self {
        I32x1(self.0.abs())
    }

    #[inline(always)]
    fn and_not(self, rhs: Self) -> Self {
        I32x1(!self.0 & rhs.0)
    }

    #[inline(always)]
    fn blendv(self, a: Self, b: Self) -> Self {
        I32x1(if self.0 != 0 { a.0 } else { b.0 })
    }

    #[inline(always)]
    fn cmp_eq(self, rhs: Self) -> Self {
        I32x1(if self.0 == rhs.0 { -1 } else { 0 })
    }

    #[inline(always)]
    fn cmp_neq(self, rhs: Self) -> Self {
        I32x1(if self.0 != rhs.0 { -1 } else { 0 })
    }

    #[inline(always)]
    fn cmp_lt(self, rhs: Self) -> Self {
        I32x1(if self.0 < rhs.0 { -1 } else { 0 })
    }

    #[inline(always)]
    fn cmp_lte(self, rhs: Self) -> Self {
        I32x1(if self.0 <= rhs.0 { -1 } else { 0 })
    }

    #[inline(always)]
    fn cmp_gt(self, rhs: Self) -> Self {
        I32x1(if self.0 > rhs.0 { -1 } else { 0 })
    }

    #[inline(always)]
    fn cmp_gte(self, rhs: Self) -> Self {
        I32x1(if self.0 >= rhs.0 { -1 } else { 0 })
    }

    #[inline(always)]
    fn max(self, rhs: Self) -> Self {
        I32x1(self.0.max(rhs.0))
    }

    #[inline(always)]
    fn min(self, rhs: Self) -> Self {
        I32x1(self.0.min(rhs.0))
    }

    #[inline(always)]
    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        I32x1(array[0])
    }

    #[inline(always)]
    unsafe fn load_from_ptr(ptr: *const Self::Scalar) -> Self {
        I32x1(*ptr)
    }

    #[inline(always)]
    unsafe fn copy_to_ptr(self, ptr: *mut Self::Scalar) {
        *ptr = self.0;
    }

    #[inline(always)]
    unsafe fn underlying_value(self) -> Self::UnderlyingType {
        self.0
    }

    #[inline(always)]
    unsafe fn underlying_value_mut(&mut self) -> &mut Self::UnderlyingType {
        &mut self.0
    }

    #[inline(always)]
    unsafe fn from_underlying_value(value: Self::UnderlyingType) -> Self {
        I32x1(value)
    }
}

impl SimdInt for I32x1 {
    #[inline(always)]
    fn shl(self, rhs: i32) -> Self {
        I32x1(self.0 << rhs)
    }

    #[inline(always)]
    fn shr(self, rhs: i32) -> Self {
        I32x1(self.0 >> rhs)
    }
}

impl SimdInt32 for I32x1 {
    type SimdF32 = F32x1;

    #[inline(always)]
    fn bitcast_f32(self) -> Self::SimdF32 {
        F32x1(f32::from_bits(self.0 as u32))
    }

    #[inline(always)]
    fn cast_f32(self) -> Self::SimdF32 {
        F32x1(self.0 as f32)
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

    #[inline(always)]
    unsafe fn zeroes() -> Self {
        I64x1(0)
    }

    #[inline(always)]
    unsafe fn set1(x: Self::Scalar) -> Self {
        I64x1(x)
    }

    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        I64x1(self.0.wrapping_add(rhs.0))
    }

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        I64x1(self.0.wrapping_sub(rhs.0))
    }

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        I64x1(self.0.wrapping_mul(rhs.0))
    }

    #[inline(always)]
    fn bit_and(self, rhs: Self) -> Self {
        I64x1(self.0 & rhs.0)
    }

    #[inline(always)]
    fn bit_or(self, rhs: Self) -> Self {
        I64x1(self.0 | rhs.0)
    }

    #[inline(always)]
    fn bit_xor(self, rhs: Self) -> Self {
        I64x1(self.0 ^ rhs.0)
    }

    #[inline(always)]
    fn bit_not(self) -> Self {
        I64x1(!self.0)
    }

    #[inline(always)]
    fn abs(self) -> Self {
        I64x1(self.0.abs())
    }

    #[inline(always)]
    fn and_not(self, rhs: Self) -> Self {
        I64x1(!self.0 & rhs.0)
    }

    #[inline(always)]
    fn blendv(self, a: Self, b: Self) -> Self {
        I64x1(if self.0 != 0 { a.0 } else { b.0 })
    }

    #[inline(always)]
    fn cmp_eq(self, rhs: Self) -> Self {
        I64x1(if self.0 == rhs.0 { -1 } else { 0 })
    }

    #[inline(always)]
    fn cmp_neq(self, rhs: Self) -> Self {
        I64x1(if self.0 != rhs.0 { -1 } else { 0 })
    }

    #[inline(always)]
    fn cmp_lt(self, rhs: Self) -> Self {
        I64x1(if self.0 < rhs.0 { -1 } else { 0 })
    }

    #[inline(always)]
    fn cmp_lte(self, rhs: Self) -> Self {
        I64x1(if self.0 <= rhs.0 { -1 } else { 0 })
    }

    #[inline(always)]
    fn cmp_gt(self, rhs: Self) -> Self {
        I64x1(if self.0 > rhs.0 { -1 } else { 0 })
    }

    #[inline(always)]
    fn cmp_gte(self, rhs: Self) -> Self {
        I64x1(if self.0 >= rhs.0 { -1 } else { 0 })
    }

    #[inline(always)]
    fn max(self, rhs: Self) -> Self {
        I64x1(self.0.max(rhs.0))
    }

    #[inline(always)]
    fn min(self, rhs: Self) -> Self {
        I64x1(self.0.min(rhs.0))
    }

    #[inline(always)]
    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        I64x1(array[0])
    }

    #[inline(always)]
    unsafe fn load_from_ptr(ptr: *const Self::Scalar) -> Self {
        I64x1(*ptr)
    }

    #[inline(always)]
    unsafe fn copy_to_ptr(self, ptr: *mut Self::Scalar) {
        *ptr = self.0;
    }

    #[inline(always)]
    unsafe fn underlying_value(self) -> Self::UnderlyingType {
        self.0
    }

    #[inline(always)]
    unsafe fn underlying_value_mut(&mut self) -> &mut Self::UnderlyingType {
        &mut self.0
    }

    #[inline(always)]
    unsafe fn from_underlying_value(value: Self::UnderlyingType) -> Self {
        I64x1(value)
    }
}

impl SimdInt for I64x1 {
    #[inline(always)]
    fn shl(self, rhs: i32) -> Self {
        I64x1(self.0 << rhs)
    }

    #[inline(always)]
    fn shr(self, rhs: i32) -> Self {
        I64x1(self.0 >> rhs)
    }
}

impl SimdInt64 for I64x1 {
    type SimdF64 = F64x1;

    #[inline(always)]
    fn bitcast_f64(self) -> Self::SimdF64 {
        F64x1(f64::from_bits(self.0 as u64))
    }

    #[inline(always)]
    fn cast_f64(self) -> Self::SimdF64 {
        F64x1(self.0 as f64)
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

    #[inline(always)]
    unsafe fn zeroes() -> Self {
        F32x1(0.0)
    }

    #[inline(always)]
    unsafe fn set1(x: Self::Scalar) -> Self {
        F32x1(x)
    }

    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        F32x1(self.0 + rhs.0)
    }

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        F32x1(self.0 - rhs.0)
    }

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        F32x1(self.0 * rhs.0)
    }

    #[inline(always)]
    fn bit_and(self, rhs: Self) -> Self {
        F32x1(f32::from_bits(self.0.to_bits() & rhs.0.to_bits()))
    }

    #[inline(always)]
    fn bit_or(self, rhs: Self) -> Self {
        F32x1(f32::from_bits(self.0.to_bits() | rhs.0.to_bits()))
    }

    #[inline(always)]
    fn bit_xor(self, rhs: Self) -> Self {
        F32x1(f32::from_bits(self.0.to_bits() ^ rhs.0.to_bits()))
    }

    #[inline(always)]
    fn bit_not(self) -> Self {
        F32x1(f32::from_bits(!self.0.to_bits()))
    }

    #[inline(always)]
    fn abs(self) -> Self {
        F32x1(self.0.m_abs())
    }

    #[inline(always)]
    fn and_not(self, rhs: Self) -> Self {
        F32x1(f32::from_bits((!self.0.to_bits()) & rhs.0.to_bits()))
    }

    #[inline(always)]
    fn blendv(self, a: Self, b: Self) -> Self {
        F32x1(if self.0.to_bits() != 0 { b.0 } else { a.0 })
    }

    #[inline(always)]
    fn cmp_eq(self, rhs: Self) -> Self {
        F32x1(if self.0 == rhs.0 {
            f32::from_bits(u32::MAX)
        } else {
            0.0
        })
    }

    #[inline(always)]
    fn cmp_neq(self, rhs: Self) -> Self {
        F32x1(if self.0 != rhs.0 {
            f32::from_bits(u32::MAX)
        } else {
            0.0
        })
    }

    #[inline(always)]
    fn cmp_lt(self, rhs: Self) -> Self {
        F32x1(if self.0 < rhs.0 {
            f32::from_bits(u32::MAX)
        } else {
            0.0
        })
    }

    #[inline(always)]
    fn cmp_lte(self, rhs: Self) -> Self {
        F32x1(if self.0 <= rhs.0 {
            f32::from_bits(u32::MAX)
        } else {
            0.0
        })
    }

    #[inline(always)]
    fn cmp_gt(self, rhs: Self) -> Self {
        F32x1(if self.0 > rhs.0 {
            f32::from_bits(u32::MAX)
        } else {
            0.0
        })
    }

    #[inline(always)]
    fn cmp_gte(self, rhs: Self) -> Self {
        F32x1(if self.0 >= rhs.0 {
            f32::from_bits(u32::MAX)
        } else {
            0.0
        })
    }

    #[inline(always)]
    fn max(self, rhs: Self) -> Self {
        F32x1(self.0.max(rhs.0))
    }

    #[inline(always)]
    fn min(self, rhs: Self) -> Self {
        F32x1(self.0.min(rhs.0))
    }

    #[inline(always)]
    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        F32x1(array[0])
    }

    #[inline(always)]
    unsafe fn load_from_ptr(ptr: *const Self::Scalar) -> Self {
        F32x1(*ptr)
    }

    #[inline(always)]
    unsafe fn copy_to_ptr(self, ptr: *mut Self::Scalar) {
        *ptr = self.0;
    }

    #[inline(always)]
    unsafe fn underlying_value(self) -> Self::UnderlyingType {
        self.0
    }

    #[inline(always)]
    unsafe fn underlying_value_mut(&mut self) -> &mut Self::UnderlyingType {
        &mut self.0
    }

    #[inline(always)]
    unsafe fn from_underlying_value(value: Self::UnderlyingType) -> Self {
        F32x1(value)
    }
}

impl SimdFloat for F32x1 {
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        F32x1(self.0 / rhs.0)
    }

    #[inline(always)]
    fn ceil(self) -> Self {
        F32x1(self.0.m_ceil())
    }

    #[inline(always)]
    fn floor(self) -> Self {
        F32x1(self.0.m_floor())
    }

    #[inline(always)]
    fn round(self) -> Self {
        F32x1(self.0.m_round())
    }

    #[inline(always)]
    fn fast_ceil(self) -> Self {
        self.ceil()
    }

    #[inline(always)]
    fn fast_floor(self) -> Self {
        self.floor()
    }

    #[inline(always)]
    fn fast_round(self) -> Self {
        self.round()
    }

    #[inline(always)]
    fn mul_add(self, a: Self, b: Self) -> Self {
        F32x1(self.0 * a.0 + b.0)
    }

    #[inline(always)]
    fn mul_sub(self, a: Self, b: Self) -> Self {
        F32x1(self.0 * a.0 - b.0)
    }

    #[inline(always)]
    fn neg_mul_add(self, a: Self, b: Self) -> Self {
        -self * a + b
    }

    #[inline(always)]
    fn neg_mul_sub(self, a: Self, b: Self) -> Self {
        -self * a - b
    }

    #[inline(always)]
    fn horizontal_add(self) -> Self::Scalar {
        self.0
    }

    #[inline(always)]
    fn sqrt(self) -> Self {
        F32x1(self.0.m_sqrt())
    }

    #[inline(always)]
    fn rsqrt(self) -> Self {
        F32x1(1.0 / self.0.m_sqrt())
    }
}

impl SimdFloat32 for F32x1 {
    type SimdI32 = I32x1;

    #[inline(always)]
    fn bitcast_i32(self) -> Self::SimdI32 {
        I32x1(self.0.to_bits() as i32)
    }

    #[inline(always)]
    fn cast_i32(self) -> Self::SimdI32 {
        I32x1(self.0 as i32)
    }

    #[inline(always)]
    fn fast_inverse(self) -> Self {
        F32x1(self.0.recip())
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

    #[inline(always)]
    unsafe fn zeroes() -> Self {
        F64x1(0.0)
    }

    #[inline(always)]
    unsafe fn set1(x: Self::Scalar) -> Self {
        F64x1(x)
    }

    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        F64x1(self.0 + rhs.0)
    }

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        F64x1(self.0 - rhs.0)
    }

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        F64x1(self.0 * rhs.0)
    }

    #[inline(always)]
    fn bit_and(self, rhs: Self) -> Self {
        F64x1(f64::from_bits(self.0.to_bits() & rhs.0.to_bits()))
    }

    #[inline(always)]
    fn bit_or(self, rhs: Self) -> Self {
        F64x1(f64::from_bits(self.0.to_bits() | rhs.0.to_bits()))
    }

    #[inline(always)]
    fn bit_xor(self, rhs: Self) -> Self {
        F64x1(f64::from_bits(self.0.to_bits() ^ rhs.0.to_bits()))
    }

    #[inline(always)]
    fn bit_not(self) -> Self {
        F64x1(f64::from_bits(!self.0.to_bits()))
    }

    #[inline(always)]
    fn abs(self) -> Self {
        F64x1(self.0.m_abs())
    }

    #[inline(always)]
    fn and_not(self, rhs: Self) -> Self {
        F64x1(f64::from_bits((!self.0.to_bits()) & rhs.0.to_bits()))
    }

    #[inline(always)]
    fn blendv(self, a: Self, b: Self) -> Self {
        F64x1(if a.0 != 0.0 { self.0 } else { b.0 })
    }

    #[inline(always)]
    fn cmp_eq(self, rhs: Self) -> Self {
        F64x1(if self.0 == rhs.0 {
            f64::from_bits(u64::MAX)
        } else {
            0.0
        })
    }

    #[inline(always)]
    fn cmp_neq(self, rhs: Self) -> Self {
        F64x1(if self.0 != rhs.0 {
            f64::from_bits(u64::MAX)
        } else {
            0.0
        })
    }

    #[inline(always)]
    fn cmp_lt(self, rhs: Self) -> Self {
        F64x1(if self.0 < rhs.0 {
            f64::from_bits(u64::MAX)
        } else {
            0.0
        })
    }

    #[inline(always)]
    fn cmp_lte(self, rhs: Self) -> Self {
        F64x1(if self.0 <= rhs.0 {
            f64::from_bits(u64::MAX)
        } else {
            0.0
        })
    }

    #[inline(always)]
    fn cmp_gt(self, rhs: Self) -> Self {
        F64x1(if self.0 > rhs.0 {
            f64::from_bits(u64::MAX)
        } else {
            0.0
        })
    }

    #[inline(always)]
    fn cmp_gte(self, rhs: Self) -> Self {
        F64x1(if self.0 >= rhs.0 {
            f64::from_bits(u64::MAX)
        } else {
            0.0
        })
    }

    #[inline(always)]
    fn max(self, rhs: Self) -> Self {
        F64x1(self.0.max(rhs.0))
    }

    #[inline(always)]
    fn min(self, rhs: Self) -> Self {
        F64x1(self.0.min(rhs.0))
    }

    #[inline(always)]
    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        F64x1(array[0])
    }

    #[inline(always)]
    unsafe fn load_from_ptr(ptr: *const Self::Scalar) -> Self {
        F64x1(*ptr)
    }

    #[inline(always)]
    unsafe fn copy_to_ptr(self, ptr: *mut Self::Scalar) {
        *ptr = self.0;
    }

    #[inline(always)]
    unsafe fn underlying_value(self) -> Self::UnderlyingType {
        self.0
    }

    #[inline(always)]
    unsafe fn underlying_value_mut(&mut self) -> &mut Self::UnderlyingType {
        &mut self.0
    }

    #[inline(always)]
    unsafe fn from_underlying_value(value: Self::UnderlyingType) -> Self {
        F64x1(value)
    }
}

impl SimdFloat for F64x1 {
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        F64x1(self.0 / rhs.0)
    }

    #[inline(always)]
    fn ceil(self) -> Self {
        F64x1(self.0.m_ceil())
    }

    #[inline(always)]
    fn floor(self) -> Self {
        F64x1(self.0.m_floor())
    }

    #[inline(always)]
    fn round(self) -> Self {
        F64x1(self.0.m_round())
    }

    #[inline(always)]
    fn fast_ceil(self) -> Self {
        self.ceil()
    }

    #[inline(always)]
    fn fast_floor(self) -> Self {
        self.floor()
    }

    #[inline(always)]
    fn fast_round(self) -> Self {
        self.round()
    }

    #[inline(always)]
    fn mul_add(self, a: Self, b: Self) -> Self {
        self * a + b
    }

    #[inline(always)]
    fn mul_sub(self, a: Self, b: Self) -> Self {
        self * a - b
    }

    #[inline(always)]
    fn neg_mul_add(self, a: Self, b: Self) -> Self {
        -self * a + b
    }

    #[inline(always)]
    fn neg_mul_sub(self, a: Self, b: Self) -> Self {
        -self * a - b
    }

    #[inline(always)]
    fn horizontal_add(self) -> Self::Scalar {
        self.0
    }

    #[inline(always)]
    fn sqrt(self) -> Self {
        F64x1(self.0.m_sqrt())
    }

    #[inline(always)]
    fn rsqrt(self) -> Self {
        F64x1(1.0 / self.0.m_sqrt())
    }
}

impl SimdFloat64 for F64x1 {
    type SimdI64 = I64x1;

    #[inline(always)]
    fn bitcast_i64(self) -> Self::SimdI64 {
        I64x1(self.0.to_bits() as i64)
    }

    #[inline(always)]
    fn cast_i64(self) -> Self::SimdI64 {
        I64x1(self.0 as i64)
    }
}
