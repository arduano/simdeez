#[cfg(target_arch = "x86")]
use core::arch::x86::*;

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use core::ops::*;

use paste::paste;

use crate::{
    InternalSimdBaseIo, SimdBaseOps, SimdConsts, SimdFloat, SimdFloat32, SimdFloat64, SimdInt,
    SimdInt16, SimdInt32, SimdInt64, SimdInt8,
};

use crate::ops::*;
mod simd;
pub use self::simd::*;

define_simd_type!(i8, 32, __m256i);
impl_simd_int_overloads!(I8x32);

impl InternalSimdBaseIo for I8x32 {
    #[inline(always)]
    unsafe fn zeroes() -> Self {
        I8x32(_mm256_setzero_si256())
    }

    #[inline(always)]
    unsafe fn set1(x: Self::Scalar) -> Self {
        I8x32(_mm256_set1_epi8(x))
    }

    #[inline(always)]
    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        I8x32(_mm256_loadu_si256(array.as_ptr() as *const _))
    }

    #[inline(always)]
    unsafe fn load_from_ptr_unaligned(ptr: *const Self::Scalar) -> Self {
        I8x32(_mm256_loadu_si256(ptr as *const _))
    }

    #[inline(always)]
    unsafe fn copy_to_ptr_unaligned(self, ptr: *mut Self::Scalar) {
        _mm256_storeu_si256(ptr as *mut _, self.0);
    }

    #[inline(always)]
    unsafe fn load_from_ptr_aligned(ptr: *const Self::Scalar) -> Self {
        I8x32(_mm256_load_si256(ptr as *const _))
    }

    #[inline(always)]
    unsafe fn copy_to_ptr_aligned(self, ptr: *mut Self::Scalar) {
        _mm256_store_si256(ptr as *mut _, self.0);
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
        I8x32(value)
    }
}

impl SimdBaseOps for I8x32 {
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        unsafe { I8x32(_mm256_add_epi8(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        unsafe { I8x32(_mm256_sub_epi8(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        // There is no multiply operation for i8, although the compiler may automatically
        // find optimizations for this.
        let mut array = [0i8; 32];
        for i in 0..32 {
            array[i] = self[i].wrapping_mul(rhs[i]);
        }
        unsafe { I8x32::load_from_array(array) }
    }

    #[inline(always)]
    fn bit_and(self, rhs: Self) -> Self {
        unsafe { I8x32(_mm256_and_si256(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_or(self, rhs: Self) -> Self {
        unsafe { I8x32(_mm256_or_si256(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_xor(self, rhs: Self) -> Self {
        unsafe { I8x32(_mm256_xor_si256(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_not(self) -> Self {
        unsafe { self.bit_xor(Self::set1(-1)) }
    }

    #[inline(always)]
    fn abs(self) -> Self {
        unsafe { I8x32(_mm256_abs_epi8(self.0)) }
    }

    #[inline(always)]
    fn and_not(self, rhs: Self) -> Self {
        unsafe { I8x32(_mm256_andnot_si256(rhs.0, self.0)) }
    }

    #[inline(always)]
    fn blendv(self, a: Self, b: Self) -> Self {
        unsafe { I8x32(_mm256_blendv_epi8(a.0, b.0, self.0)) }
    }

    #[inline(always)]
    fn cmp_eq(self, rhs: Self) -> Self {
        unsafe { I8x32(_mm256_cmpeq_epi8(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn cmp_neq(self, rhs: Self) -> Self {
        self.cmp_eq(rhs).bit_not()
    }

    #[inline(always)]
    fn cmp_lt(self, rhs: Self) -> Self {
        unsafe { I8x32(_mm256_cmpgt_epi8(rhs.0, self.0)) }
    }

    #[inline(always)]
    fn cmp_lte(self, rhs: Self) -> Self {
        self.cmp_gt(rhs).bit_not()
    }

    #[inline(always)]
    fn cmp_gt(self, rhs: Self) -> Self {
        unsafe { I8x32(_mm256_cmpgt_epi8(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn cmp_gte(self, rhs: Self) -> Self {
        self.cmp_lt(rhs).bit_not()
    }

    #[inline(always)]
    fn max(self, rhs: Self) -> Self {
        unsafe { I8x32(_mm256_max_epi8(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn min(self, rhs: Self) -> Self {
        unsafe { I8x32(_mm256_min_epi8(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn horizontal_add(self) -> Self::HorizontalAddScalar {
        self.partial_horizontal_add()
            .partial_horizontal_add()
            .partial_horizontal_add()
            .partial_horizontal_add()
    }
}

impl SimdInt for I8x32 {
    #[inline(always)]
    fn shl(self, rhs: i32) -> Self {
        unsafe {
            // Do 16 bit shifts, then mask out the bits that are shifted in.
            let rhs2 = _mm_cvtsi32_si128(rhs);
            let shifted_i16 = _mm256_sll_epi16(self.0, rhs2);

            let mask = 0x00FFu16 >> (8 - rhs) << 8;
            let mask = _mm256_set1_epi16(mask as i16);
            I8x32(_mm256_andnot_si256(mask, shifted_i16))
        }
    }

    #[inline(always)]
    fn shr(self, rhs: i32) -> Self {
        unsafe {
            // Do 16 bit shifts, then mask out the bits that are shifted in.
            let rhs2 = _mm_cvtsi32_si128(rhs);
            let shifted_i16 = _mm256_srl_epi16(self.0, rhs2);

            let mask = 0xFF00u16 << (8 - rhs) >> 8;
            let mask = _mm256_set1_epi16(mask as i16);
            I8x32(_mm256_andnot_si256(mask, shifted_i16))
        }
    }

    #[inline(always)]
    fn horizontal_unsigned_add(self) -> Self::HorizontalAddScalar {
        self.partial_horizontal_unsigned_add()
            .partial_horizontal_unsigned_add()
            .partial_horizontal_unsigned_add()
            .partial_horizontal_add()
    }
}

impl SimdInt8 for I8x32 {
    type SimdI16 = I16x16;

    #[inline(always)]
    fn extend_to_i16(self) -> (Self::SimdI16, Self::SimdI16) {
        unsafe {
            let a = _mm256_cvtepi8_epi16(_mm256_extracti128_si256(self.0, 0));
            let b = _mm256_cvtepi8_epi16(_mm256_extracti128_si256(self.0, 1));
            (I16x16(a), I16x16(b))
        }
    }

    #[inline(always)]
    fn unsigned_extend_to_i16(self) -> (Self::SimdI16, Self::SimdI16) {
        unsafe {
            let a = _mm256_cvtepu8_epi16(_mm256_extracti128_si256(self.0, 0));
            let b = _mm256_cvtepu8_epi16(_mm256_extracti128_si256(self.0, 1));
            (I16x16(a), I16x16(b))
        }
    }

    #[inline(always)]
    fn get_mask(self) -> u32 {
        unsafe { _mm256_movemask_epi8(self.0) as u32 }
    }
}

define_simd_type!(i16, 16, __m256i);
impl_simd_int_overloads!(I16x16);

impl InternalSimdBaseIo for I16x16 {
    #[inline(always)]
    unsafe fn zeroes() -> Self {
        I16x16(_mm256_setzero_si256())
    }

    #[inline(always)]
    unsafe fn set1(x: Self::Scalar) -> Self {
        I16x16(_mm256_set1_epi16(x))
    }

    #[inline(always)]
    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        I16x16(_mm256_loadu_si256(array.as_ptr() as *const _))
    }

    #[inline(always)]
    unsafe fn load_from_ptr_unaligned(ptr: *const Self::Scalar) -> Self {
        I16x16(_mm256_loadu_si256(ptr as *const _))
    }

    #[inline(always)]
    unsafe fn copy_to_ptr_unaligned(self, ptr: *mut Self::Scalar) {
        _mm256_storeu_si256(ptr as *mut _, self.0);
    }

    #[inline(always)]
    unsafe fn load_from_ptr_aligned(ptr: *const Self::Scalar) -> Self {
        I16x16(_mm256_load_si256(ptr as *const _))
    }

    #[inline(always)]
    unsafe fn copy_to_ptr_aligned(self, ptr: *mut Self::Scalar) {
        _mm256_store_si256(ptr as *mut _, self.0);
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
        I16x16(value)
    }
}

impl SimdBaseOps for I16x16 {
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        unsafe { I16x16(_mm256_add_epi16(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        unsafe { I16x16(_mm256_sub_epi16(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        unsafe { I16x16(_mm256_mullo_epi16(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_and(self, rhs: Self) -> Self {
        unsafe { I16x16(_mm256_and_si256(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_or(self, rhs: Self) -> Self {
        unsafe { I16x16(_mm256_or_si256(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_xor(self, rhs: Self) -> Self {
        unsafe { I16x16(_mm256_xor_si256(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_not(self) -> Self {
        unsafe { self.bit_xor(Self::set1(-1)) }
    }

    #[inline(always)]
    fn abs(self) -> Self {
        unsafe { I16x16(_mm256_abs_epi16(self.0)) }
    }

    #[inline(always)]
    fn and_not(self, rhs: Self) -> Self {
        unsafe { I16x16(_mm256_andnot_si256(rhs.0, self.0)) }
    }

    #[inline(always)]
    fn blendv(self, a: Self, b: Self) -> Self {
        unsafe { I16x16(_mm256_blendv_epi8(a.0, b.0, self.0)) }
    }

    #[inline(always)]
    fn cmp_eq(self, rhs: Self) -> Self {
        unsafe { I16x16(_mm256_cmpeq_epi16(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn cmp_neq(self, rhs: Self) -> Self {
        self.cmp_eq(rhs).bit_not()
    }

    #[inline(always)]
    fn cmp_lt(self, rhs: Self) -> Self {
        unsafe { I16x16(_mm256_cmpgt_epi16(rhs.0, self.0)) }
    }

    #[inline(always)]
    fn cmp_lte(self, rhs: Self) -> Self {
        self.cmp_gt(rhs).bit_not()
    }

    #[inline(always)]
    fn cmp_gt(self, rhs: Self) -> Self {
        unsafe { I16x16(_mm256_cmpgt_epi16(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn cmp_gte(self, rhs: Self) -> Self {
        self.cmp_lt(rhs).bit_not()
    }

    #[inline(always)]
    fn max(self, rhs: Self) -> Self {
        unsafe { I16x16(_mm256_max_epi16(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn min(self, rhs: Self) -> Self {
        unsafe { I16x16(_mm256_min_epi16(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn horizontal_add(self) -> Self::HorizontalAddScalar {
        self.partial_horizontal_add()
            .partial_horizontal_add()
            .partial_horizontal_add()
    }
}

impl SimdInt for I16x16 {
    #[inline(always)]
    fn shl(self, rhs: i32) -> Self {
        unsafe {
            let rhs = _mm_cvtsi32_si128(rhs);
            I16x16(_mm256_sll_epi16(self.0, rhs))
        }
    }

    #[inline(always)]
    fn shr(self, rhs: i32) -> Self {
        unsafe {
            let rhs = _mm_cvtsi32_si128(rhs);
            I16x16(_mm256_srl_epi16(self.0, rhs))
        }
    }

    #[inline(always)]
    fn shl_const<const BY: i32>(self) -> Self {
        unsafe { I16x16(_mm256_slli_epi16(self.0, BY)) }
    }

    #[inline(always)]
    fn shr_const<const BY: i32>(self) -> Self {
        unsafe { I16x16(_mm256_srli_epi16(self.0, BY)) }
    }

    #[inline(always)]
    fn horizontal_unsigned_add(self) -> Self::HorizontalAddScalar {
        self.partial_horizontal_unsigned_add()
            .partial_horizontal_unsigned_add()
            .horizontal_unsigned_add()
    }
}

impl SimdInt16 for I16x16 {
    type SimdI32 = I32x8;

    #[inline(always)]
    fn extend_to_i32(self) -> (Self::SimdI32, Self::SimdI32) {
        unsafe {
            let a = _mm256_cvtepi16_epi32(_mm256_extracti128_si256(self.0, 0));
            let b = _mm256_cvtepi16_epi32(_mm256_extracti128_si256(self.0, 1));
            (I32x8(a), I32x8(b))
        }
    }

    #[inline(always)]
    fn unsigned_extend_to_i32(self) -> (Self::SimdI32, Self::SimdI32) {
        unsafe {
            let a = _mm256_cvtepu16_epi32(_mm256_extracti128_si256(self.0, 0));
            let b = _mm256_cvtepu16_epi32(_mm256_extracti128_si256(self.0, 1));
            (I32x8(a), I32x8(b))
        }
    }
}

define_simd_type!(i32, 8, __m256i);
impl_simd_int_overloads!(I32x8);
impl_i32_simd_type!(Avx2, I32x8, F32x8, I64x4);

define_simd_type!(i64, 4, __m256i);
impl_simd_int_overloads!(I64x4);
impl_i64_simd_type!(Avx2, I64x4, F64x4);

define_simd_type!(f32, 8, __m256);
impl_simd_float_overloads!(F32x8);
impl_f32_simd_type!(Avx2, F32x8, I32x8);

define_simd_type!(f64, 4, __m256d);
impl_simd_float_overloads!(F64x4);
impl_f64_simd_type!(Avx2, F64x4, I64x4);
