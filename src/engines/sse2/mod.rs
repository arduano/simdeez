#[cfg(target_arch = "x86")]
use core::arch::x86::*;

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use paste::paste;

use core::ops::*;

use crate::{
    InternalSimdBaseIo, SimdBaseOps, SimdConsts, SimdFloat, SimdFloat32, SimdFloat64, SimdInt,
    SimdInt16, SimdInt32, SimdInt64, SimdInt8,
};

use crate::ops::*;

mod simd;
pub use self::simd::*;

define_simd_type!(i8, 16, __m128i);
impl_simd_int_overloads!(I8x16);

impl InternalSimdBaseIo for I8x16 {
    #[inline(always)]
    unsafe fn zeroes() -> Self {
        I8x16(_mm_setzero_si128())
    }

    #[inline(always)]
    unsafe fn set1(x: Self::Scalar) -> Self {
        I8x16(_mm_set1_epi8(x))
    }

    #[inline(always)]
    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        I8x16(_mm_loadu_si128(array.as_ptr() as *const __m128i))
    }

    #[inline(always)]
    unsafe fn load_from_ptr_unaligned(ptr: *const Self::Scalar) -> Self {
        I8x16(_mm_loadu_si128(ptr as *const __m128i))
    }

    #[inline(always)]
    unsafe fn copy_to_ptr_unaligned(self, ptr: *mut Self::Scalar) {
        _mm_storeu_si128(ptr as *mut __m128i, self.0);
    }

    #[inline(always)]
    unsafe fn load_from_ptr_aligned(ptr: *const Self::Scalar) -> Self {
        I8x16(_mm_load_si128(ptr as *const __m128i))
    }

    #[inline(always)]
    unsafe fn copy_to_ptr_aligned(self, ptr: *mut Self::Scalar) {
        _mm_store_si128(ptr as *mut __m128i, self.0);
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
        I8x16(value)
    }
}

impl SimdBaseOps for I8x16 {
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        unsafe { I8x16(_mm_add_epi8(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        unsafe { I8x16(_mm_sub_epi8(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        // There is no multiply operation for i8, although the compiler may automatically
        // find optimizations for this.
        let mut array = [0i8; 16];
        for i in 0..16 {
            array[i] = self[i].wrapping_mul(rhs[i]);
        }
        unsafe { I8x16::load_from_array(array) }
    }

    #[inline(always)]
    fn bit_and(self, rhs: Self) -> Self {
        unsafe { I8x16(_mm_and_si128(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_or(self, rhs: Self) -> Self {
        unsafe { I8x16(_mm_or_si128(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_xor(self, rhs: Self) -> Self {
        unsafe { I8x16(_mm_xor_si128(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_not(self) -> Self {
        unsafe { Self::set1(-1).bit_xor(self) }
    }

    #[inline(always)]
    fn abs(self) -> Self {
        unsafe {
            let mask = self.cmp_lt(Self::zeroes());
            self.bit_xor(mask) - mask
        }
    }

    #[inline(always)]
    fn and_not(self, rhs: Self) -> Self {
        unsafe { I8x16(_mm_andnot_si128(rhs.0, self.0)) }
    }

    #[inline(always)]
    fn blendv(self, a: Self, b: Self) -> Self {
        unsafe {
            let mask = self.cmp_eq(Self::set1(-1));
            let a = a.and_not(mask);
            let b = mask.bit_and(b);
            a.bit_or(b)
        }
    }

    #[inline(always)]
    fn cmp_eq(self, rhs: Self) -> Self {
        unsafe { I8x16(_mm_cmpeq_epi8(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn cmp_neq(self, rhs: Self) -> Self {
        self.cmp_eq(rhs).bit_not()
    }

    #[inline(always)]
    fn cmp_lt(self, rhs: Self) -> Self {
        unsafe { I8x16(_mm_cmplt_epi8(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn cmp_lte(self, rhs: Self) -> Self {
        self.cmp_gt(rhs).bit_not()
    }

    #[inline(always)]
    fn cmp_gt(self, rhs: Self) -> Self {
        unsafe { I8x16(_mm_cmpgt_epi8(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn cmp_gte(self, rhs: Self) -> Self {
        self.cmp_lt(rhs).bit_not()
    }

    #[inline(always)]
    fn max(self, rhs: Self) -> Self {
        unsafe { I8x16(_mm_max_epi8(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn min(self, rhs: Self) -> Self {
        unsafe { I8x16(_mm_min_epi8(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn horizontal_add(self) -> Self::HorizontalAddScalar {
        self.partial_horizontal_add()
            .partial_horizontal_add()
            .partial_horizontal_add()
            .partial_horizontal_add()
    }
}

impl SimdInt for I8x16 {
    #[inline(always)]
    fn shl(self, rhs: i32) -> Self {
        unsafe {
            // Do 16 bit shifts, then mask out the bits that are shifted in.
            let rhs2 = _mm_cvtsi32_si128(rhs);
            let shifted_i16 = _mm_sll_epi16(self.0, rhs2);

            let mask = 0x00FFu16 >> (8 - rhs) << 8;
            let mask = _mm_set1_epi16(mask as i16);
            I8x16(_mm_andnot_si128(mask, shifted_i16))
        }
    }

    #[inline(always)]
    fn shr(self, rhs: i32) -> Self {
        unsafe {
            // Do 16 bit shifts, then mask out the bits that are shifted in.
            let rhs2 = _mm_cvtsi32_si128(rhs);
            let shifted_i16 = _mm_srl_epi16(self.0, rhs2);

            let mask = 0xFF00u16 << (8 - rhs) >> 8;
            let mask = _mm_set1_epi16(mask as i16);
            I8x16(_mm_andnot_si128(mask, shifted_i16))
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

impl SimdInt8 for I8x16 {
    type SimdI16 = I16x8;

    #[inline(always)]
    fn extend_to_i16(self) -> (Self::SimdI16, Self::SimdI16) {
        let (part1, part2) = self.unsigned_extend_to_i16();

        let sign_extend = |val: I16x8| unsafe {
            // Extract the sign bits
            let sign_mask = I16x8::set1(0x80);
            let sign_bits = val.bit_and(sign_mask);

            // Convert any 0x80 to 0xFF00
            let mask = sign_bits << 1;
            let mask = mask | (mask << 1);
            let mask = mask | (mask << 2);
            let mask = mask | (mask << 4);

            // Or that back into the original value
            val | mask
        };

        (sign_extend(part1), sign_extend(part2))
    }

    #[inline(always)]
    fn unsigned_extend_to_i16(self) -> (Self::SimdI16, Self::SimdI16) {
        unsafe {
            let lo = _mm_unpacklo_epi8(self.0, _mm_setzero_si128());
            let hi = _mm_unpackhi_epi8(self.0, _mm_setzero_si128());
            (I16x8(lo), I16x8(hi))
        }
    }

    #[inline(always)]
    fn get_mask(self) -> u32 {
        unsafe { _mm_movemask_epi8(self.0) as u32 }
    }
}

define_simd_type!(i16, 8, __m128i);
impl_simd_int_overloads!(I16x8);

impl InternalSimdBaseIo for I16x8 {
    #[inline(always)]
    unsafe fn zeroes() -> Self {
        I16x8(_mm_setzero_si128())
    }

    #[inline(always)]
    unsafe fn set1(x: Self::Scalar) -> Self {
        I16x8(_mm_set1_epi16(x))
    }

    #[inline(always)]
    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        I16x8(_mm_loadu_si128(array.as_ptr() as *const __m128i))
    }

    #[inline(always)]
    unsafe fn load_from_ptr_unaligned(ptr: *const Self::Scalar) -> Self {
        I16x8(_mm_loadu_si128(ptr as *const __m128i))
    }

    #[inline(always)]
    unsafe fn copy_to_ptr_unaligned(self, ptr: *mut Self::Scalar) {
        _mm_storeu_si128(ptr as *mut __m128i, self.0);
    }

    #[inline(always)]
    unsafe fn load_from_ptr_aligned(ptr: *const Self::Scalar) -> Self {
        I16x8(_mm_load_si128(ptr as *const __m128i))
    }

    #[inline(always)]
    unsafe fn copy_to_ptr_aligned(self, ptr: *mut Self::Scalar) {
        _mm_store_si128(ptr as *mut __m128i, self.0);
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
        I16x8(value)
    }
}

impl SimdBaseOps for I16x8 {
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        unsafe { I16x8(_mm_add_epi16(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        unsafe { I16x8(_mm_sub_epi16(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        unsafe { I16x8(_mm_mullo_epi16(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_and(self, rhs: Self) -> Self {
        unsafe { I16x8(_mm_and_si128(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_or(self, rhs: Self) -> Self {
        unsafe { I16x8(_mm_or_si128(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_xor(self, rhs: Self) -> Self {
        unsafe { I16x8(_mm_xor_si128(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_not(self) -> Self {
        unsafe { Self::set1(-1).bit_xor(self) }
    }

    #[inline(always)]
    fn abs(self) -> Self {
        unsafe {
            let mask = self.cmp_lt(Self::zeroes());
            self.bit_xor(mask) - mask
        }
    }

    #[inline(always)]
    fn and_not(self, rhs: Self) -> Self {
        unsafe { I16x8(_mm_andnot_si128(rhs.0, self.0)) }
    }

    #[inline(always)]
    fn blendv(self, a: Self, b: Self) -> Self {
        unsafe {
            let mask = self.cmp_eq(Self::set1(-1));
            let a = a.and_not(mask);
            let b = mask.bit_and(b);
            a.bit_or(b)
        }
    }

    #[inline(always)]
    fn cmp_eq(self, rhs: Self) -> Self {
        unsafe { I16x8(_mm_cmpeq_epi16(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn cmp_neq(self, rhs: Self) -> Self {
        self.cmp_eq(rhs).bit_not()
    }

    #[inline(always)]
    fn cmp_lt(self, rhs: Self) -> Self {
        unsafe { I16x8(_mm_cmplt_epi16(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn cmp_lte(self, rhs: Self) -> Self {
        self.cmp_gt(rhs).bit_not()
    }

    #[inline(always)]
    fn cmp_gt(self, rhs: Self) -> Self {
        unsafe { I16x8(_mm_cmpgt_epi16(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn cmp_gte(self, rhs: Self) -> Self {
        self.cmp_lt(rhs).bit_not()
    }

    #[inline(always)]
    fn max(self, rhs: Self) -> Self {
        unsafe { I16x8(_mm_max_epi16(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn min(self, rhs: Self) -> Self {
        unsafe { I16x8(_mm_min_epi16(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn horizontal_add(self) -> Self::HorizontalAddScalar {
        self.partial_horizontal_add()
            .partial_horizontal_add()
            .partial_horizontal_add()
    }
}

impl SimdInt for I16x8 {
    #[inline(always)]
    fn shl(self, rhs: i32) -> Self {
        unsafe {
            let rhs = _mm_cvtsi32_si128(rhs);
            I16x8(_mm_sll_epi16(self.0, rhs))
        }
    }

    #[inline(always)]
    fn shr(self, rhs: i32) -> Self {
        unsafe {
            let rhs = _mm_cvtsi32_si128(rhs);
            I16x8(_mm_srl_epi16(self.0, rhs))
        }
    }

    #[inline(always)]
    fn shl_const<const BY: i32>(self) -> Self {
        unsafe { I16x8(_mm_slli_epi16(self.0, BY)) }
    }

    #[inline(always)]
    fn shr_const<const BY: i32>(self) -> Self {
        unsafe { I16x8(_mm_srli_epi16(self.0, BY)) }
    }

    #[inline(always)]
    fn horizontal_unsigned_add(self) -> Self::HorizontalAddScalar {
        self.partial_horizontal_unsigned_add()
            .partial_horizontal_unsigned_add()
            .horizontal_unsigned_add()
    }
}

impl SimdInt16 for I16x8 {
    type SimdI32 = I32x4;

    #[inline(always)]
    fn extend_to_i32(self) -> (Self::SimdI32, Self::SimdI32) {
        let (part1, part2) = self.unsigned_extend_to_i32();

        let sign_extend = |val: I32x4| unsafe {
            // Extract the sign bits
            let sign_mask = I32x4::set1(0x8000);
            let sign_bits = val.bit_and(sign_mask);

            // Convert any 0x8000 to 0xFFFF0000
            let mask = sign_bits << 1;
            let mask = mask | (mask << 1);
            let mask = mask | (mask << 2);
            let mask = mask | (mask << 4);
            let mask = mask | (mask << 8);

            // Or that back into the original value
            val | mask
        };

        (sign_extend(part1), sign_extend(part2))
    }

    #[inline(always)]
    fn unsigned_extend_to_i32(self) -> (Self::SimdI32, Self::SimdI32) {
        unsafe {
            let lo = _mm_unpacklo_epi16(self.0, _mm_setzero_si128());
            let hi = _mm_unpackhi_epi16(self.0, _mm_setzero_si128());
            (I32x4(lo), I32x4(hi))
        }
    }
}

define_simd_type!(i32, 4, __m128i);
impl_simd_int_overloads!(I32x4);
impl_i32_simd_type!(Sse2, I32x4, F32x4, I64x2);

define_simd_type!(i64, 2, __m128i);
impl_simd_int_overloads!(I64x2);
impl_i64_simd_type!(Sse2, I64x2, F64x2);

define_simd_type!(f32, 4, __m128);
impl_simd_float_overloads!(F32x4);
impl_f32_simd_type!(Sse2, F32x4, I32x4);

define_simd_type!(f64, 2, __m128d);
impl_simd_float_overloads!(F64x2);
impl_f64_simd_type!(Sse2, F64x2, I64x2);
