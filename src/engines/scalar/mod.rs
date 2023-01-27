use crate::{
    libm_ext::FloatExt, InternalSimdBaseIo, SimdBaseOps, SimdConsts, SimdFloat, SimdFloat32,
    SimdFloat64, SimdInt, SimdInt16, SimdInt32, SimdInt64,
};

use core::ops::*;

mod simd;
pub use self::simd::*;

define_simd_type!(i16, 1, i16);
impl_simd_int_overloads!(I16x1);

impl InternalSimdBaseIo for I16x1 {
    #[inline(always)]
    unsafe fn zeroes() -> Self {
        I16x1(0)
    }

    #[inline(always)]
    unsafe fn set1(x: Self::Scalar) -> Self {
        I16x1(x)
    }

    #[inline(always)]
    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        I16x1(array[0])
    }

    #[inline(always)]
    unsafe fn load_from_ptr_unaligned(ptr: *const Self::Scalar) -> Self {
        I16x1(*ptr)
    }

    #[inline(always)]
    unsafe fn copy_to_ptr_unaligned(self, ptr: *mut Self::Scalar) {
        *ptr = self.0;
    }

    #[inline(always)]
    unsafe fn load_from_ptr_aligned(ptr: *const Self::Scalar) -> Self {
        I16x1(*ptr)
    }

    #[inline(always)]
    unsafe fn copy_to_ptr_aligned(self, ptr: *mut Self::Scalar) {
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

impl SimdBaseOps for I16x1 {
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
        I16x1(if self.0 != 0 { b.0 } else { a.0 })
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
}

impl SimdInt for I16x1 {
    #[inline(always)]
    fn shl(self, rhs: i32) -> Self {
        I16x1(((self.0 as u16) << rhs) as i16)
    }

    #[inline(always)]
    fn shr(self, rhs: i32) -> Self {
        I16x1(((self.0 as u16) >> rhs) as i16)
    }
}

impl SimdInt16 for I16x1 {}

define_simd_type!(i32, 1, i32);
impl_simd_int_overloads!(I32x1);

impl InternalSimdBaseIo for I32x1 {
    #[inline(always)]
    unsafe fn zeroes() -> Self {
        I32x1(0)
    }

    #[inline(always)]
    unsafe fn set1(x: Self::Scalar) -> Self {
        I32x1(x)
    }

    #[inline(always)]
    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        I32x1(array[0])
    }

    #[inline(always)]
    unsafe fn load_from_ptr_unaligned(ptr: *const Self::Scalar) -> Self {
        I32x1(*ptr)
    }

    #[inline(always)]
    unsafe fn copy_to_ptr_unaligned(self, ptr: *mut Self::Scalar) {
        *ptr = self.0;
    }

    #[inline(always)]
    unsafe fn load_from_ptr_aligned(ptr: *const Self::Scalar) -> Self {
        I32x1(*ptr)
    }

    #[inline(always)]
    unsafe fn copy_to_ptr_aligned(self, ptr: *mut Self::Scalar) {
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

impl SimdBaseOps for I32x1 {
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
        I32x1(if self.0 != 0 { b.0 } else { a.0 })
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
}

impl SimdInt for I32x1 {
    #[inline(always)]
    fn shl(self, rhs: i32) -> Self {
        I32x1(((self.0 as u32) << rhs) as i32)
    }

    #[inline(always)]
    fn shr(self, rhs: i32) -> Self {
        I32x1(((self.0 as u32) >> rhs) as i32)
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

define_simd_type!(i64, 1, i64);
impl_simd_int_overloads!(I64x1);

impl InternalSimdBaseIo for I64x1 {
    #[inline(always)]
    unsafe fn zeroes() -> Self {
        I64x1(0)
    }

    #[inline(always)]
    unsafe fn set1(x: Self::Scalar) -> Self {
        I64x1(x)
    }

    #[inline(always)]
    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        I64x1(array[0])
    }

    #[inline(always)]
    unsafe fn load_from_ptr_unaligned(ptr: *const Self::Scalar) -> Self {
        I64x1(*ptr)
    }

    #[inline(always)]
    unsafe fn copy_to_ptr_unaligned(self, ptr: *mut Self::Scalar) {
        *ptr = self.0;
    }

    #[inline(always)]
    unsafe fn load_from_ptr_aligned(ptr: *const Self::Scalar) -> Self {
        I64x1(*ptr)
    }

    #[inline(always)]
    unsafe fn copy_to_ptr_aligned(self, ptr: *mut Self::Scalar) {
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

impl SimdBaseOps for I64x1 {
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
        I64x1(if self.0 != 0 { b.0 } else { a.0 })
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
}

impl SimdInt for I64x1 {
    #[inline(always)]
    fn shl(self, rhs: i32) -> Self {
        I64x1(((self.0 as u64) << rhs) as i64)
    }

    #[inline(always)]
    fn shr(self, rhs: i32) -> Self {
        I64x1(((self.0 as u64) >> rhs) as i64)
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
        unsafe { Self::SimdF64::load_from_array([self[0] as f64]) }
    }
}

define_simd_type!(f32, 1, f32);
impl_simd_float_overloads!(F32x1);

impl InternalSimdBaseIo for F32x1 {
    #[inline(always)]
    unsafe fn zeroes() -> Self {
        F32x1(0.0)
    }

    #[inline(always)]
    unsafe fn set1(x: Self::Scalar) -> Self {
        F32x1(x)
    }

    #[inline(always)]
    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        F32x1(array[0])
    }

    #[inline(always)]
    unsafe fn load_from_ptr_unaligned(ptr: *const Self::Scalar) -> Self {
        F32x1(*ptr)
    }

    #[inline(always)]
    unsafe fn copy_to_ptr_unaligned(self, ptr: *mut Self::Scalar) {
        *ptr = self.0;
    }

    #[inline(always)]
    unsafe fn load_from_ptr_aligned(ptr: *const Self::Scalar) -> Self {
        F32x1(*ptr)
    }

    #[inline(always)]
    unsafe fn copy_to_ptr_aligned(self, ptr: *mut Self::Scalar) {
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

impl SimdBaseOps for F32x1 {
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
        I32x1(self.0.m_round() as i32)
    }

    #[inline(always)]
    fn fast_inverse(self) -> Self {
        F32x1(self.0.recip())
    }
}

define_simd_type!(f64, 1, f64);
impl_simd_float_overloads!(F64x1);

impl InternalSimdBaseIo for F64x1 {
    #[inline(always)]
    unsafe fn zeroes() -> Self {
        F64x1(0.0)
    }

    #[inline(always)]
    unsafe fn set1(x: Self::Scalar) -> Self {
        F64x1(x)
    }

    #[inline(always)]
    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        F64x1(array[0])
    }

    #[inline(always)]
    unsafe fn load_from_ptr_unaligned(ptr: *const Self::Scalar) -> Self {
        F64x1(*ptr)
    }

    #[inline(always)]
    unsafe fn copy_to_ptr_unaligned(self, ptr: *mut Self::Scalar) {
        *ptr = self.0;
    }

    #[inline(always)]
    unsafe fn load_from_ptr_aligned(ptr: *const Self::Scalar) -> Self {
        F64x1(*ptr)
    }

    #[inline(always)]
    unsafe fn copy_to_ptr_aligned(self, ptr: *mut Self::Scalar) {
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

impl SimdBaseOps for F64x1 {
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
        F64x1(if self.0 != 0.0 { b.0 } else { a.0 })
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
        unsafe { Self::SimdI64::load_from_array([self[0].m_round() as i64]) }
    }
}
