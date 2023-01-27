#[cfg(target_arch = "x86")]
use core::arch::x86::*;

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use core::ops::*;

use crate::{
    libm_ext::FloatExt, InternalSimdBaseIo, SimdBaseOps, SimdConsts, SimdFloat, SimdFloat32,
    SimdFloat64, SimdInt, SimdInt16, SimdInt32, SimdInt64,
};

mod simd;
pub use self::simd::*;

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
        unsafe { I16x16(_mm256_andnot_si256(self.0, rhs.0)) }
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
}

impl SimdInt16 for I16x16 {}

define_simd_type!(i32, 8, __m256i);
impl_simd_int_overloads!(I32x8);

impl InternalSimdBaseIo for I32x8 {
    #[inline(always)]
    unsafe fn zeroes() -> Self {
        I32x8(_mm256_setzero_si256())
    }

    #[inline(always)]
    unsafe fn set1(x: Self::Scalar) -> Self {
        I32x8(_mm256_set1_epi32(x))
    }

    #[inline(always)]
    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        I32x8(_mm256_loadu_si256(array.as_ptr() as *const _))
    }

    #[inline(always)]
    unsafe fn load_from_ptr_unaligned(ptr: *const Self::Scalar) -> Self {
        I32x8(_mm256_loadu_si256(ptr as *const _))
    }

    #[inline(always)]
    unsafe fn copy_to_ptr_unaligned(self, ptr: *mut Self::Scalar) {
        _mm256_storeu_si256(ptr as *mut _, self.0);
    }

    #[inline(always)]
    unsafe fn load_from_ptr_aligned(ptr: *const Self::Scalar) -> Self {
        I32x8(_mm256_load_si256(ptr as *const _))
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
        I32x8(value)
    }
}

impl SimdBaseOps for I32x8 {
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        unsafe { I32x8(_mm256_add_epi32(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        unsafe { I32x8(_mm256_sub_epi32(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        unsafe { I32x8(_mm256_mullo_epi32(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_and(self, rhs: Self) -> Self {
        unsafe { I32x8(_mm256_and_si256(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_or(self, rhs: Self) -> Self {
        unsafe { I32x8(_mm256_or_si256(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_xor(self, rhs: Self) -> Self {
        unsafe { I32x8(_mm256_xor_si256(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_not(self) -> Self {
        unsafe { self.bit_xor(Self::set1(-1)) }
    }

    #[inline(always)]
    fn abs(self) -> Self {
        unsafe { I32x8(_mm256_abs_epi32(self.0)) }
    }

    #[inline(always)]
    fn and_not(self, rhs: Self) -> Self {
        unsafe { I32x8(_mm256_andnot_si256(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn blendv(self, a: Self, b: Self) -> Self {
        self.bitcast_f32()
            .blendv(a.bitcast_f32(), b.bitcast_f32())
            .bitcast_i32()
    }

    #[inline(always)]
    fn cmp_eq(self, rhs: Self) -> Self {
        unsafe { I32x8(_mm256_cmpeq_epi32(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn cmp_neq(self, rhs: Self) -> Self {
        self.cmp_eq(rhs).bit_not()
    }

    #[inline(always)]
    fn cmp_lt(self, rhs: Self) -> Self {
        unsafe { I32x8(_mm256_cmpgt_epi32(rhs.0, self.0)) }
    }

    #[inline(always)]
    fn cmp_lte(self, rhs: Self) -> Self {
        self.cmp_gt(rhs).bit_not()
    }

    #[inline(always)]
    fn cmp_gt(self, rhs: Self) -> Self {
        unsafe { I32x8(_mm256_cmpgt_epi32(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn cmp_gte(self, rhs: Self) -> Self {
        self.cmp_lt(rhs).bit_not()
    }

    #[inline(always)]
    fn max(self, rhs: Self) -> Self {
        unsafe { I32x8(_mm256_max_epi32(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn min(self, rhs: Self) -> Self {
        unsafe { I32x8(_mm256_min_epi32(self.0, rhs.0)) }
    }
}

impl SimdInt for I32x8 {
    #[inline(always)]
    fn shl(self, rhs: i32) -> Self {
        unsafe {
            let rhs = _mm_cvtsi32_si128(rhs);
            I32x8(_mm256_sll_epi32(self.0, rhs))
        }
    }

    #[inline(always)]
    fn shr(self, rhs: i32) -> Self {
        unsafe {
            let rhs = _mm_cvtsi32_si128(rhs);
            I32x8(_mm256_srl_epi32(self.0, rhs))
        }
    }

    #[inline(always)]
    fn shl_const<const BY: i32>(self) -> Self {
        unsafe { I32x8(_mm256_slli_epi32(self.0, BY)) }
    }

    #[inline(always)]
    fn shr_const<const BY: i32>(self) -> Self {
        unsafe { I32x8(_mm256_srli_epi32(self.0, BY)) }
    }
}

impl SimdInt32 for I32x8 {
    type SimdF32 = F32x8;

    #[inline(always)]
    fn bitcast_f32(self) -> Self::SimdF32 {
        unsafe { F32x8(_mm256_castsi256_ps(self.0)) }
    }

    #[inline(always)]
    fn cast_f32(self) -> Self::SimdF32 {
        unsafe { F32x8(_mm256_cvtepi32_ps(self.0)) }
    }
}

define_simd_type!(i64, 4, __m256i);
impl_simd_int_overloads!(I64x4);

impl InternalSimdBaseIo for I64x4 {
    #[inline(always)]
    unsafe fn zeroes() -> Self {
        I64x4(_mm256_setzero_si256())
    }

    #[inline(always)]
    unsafe fn set1(x: Self::Scalar) -> Self {
        I64x4(_mm256_set1_epi64x(x))
    }

    #[inline(always)]
    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        I64x4(_mm256_loadu_si256(array.as_ptr() as *const _))
    }

    #[inline(always)]
    unsafe fn load_from_ptr_unaligned(ptr: *const Self::Scalar) -> Self {
        I64x4(_mm256_loadu_si256(ptr as *const _))
    }

    #[inline(always)]
    unsafe fn copy_to_ptr_unaligned(self, ptr: *mut Self::Scalar) {
        _mm256_storeu_si256(ptr as *mut _, self.0);
    }

    #[inline(always)]
    unsafe fn load_from_ptr_aligned(ptr: *const Self::Scalar) -> Self {
        I64x4(_mm256_load_si256(ptr as *const _))
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
        I64x4(value)
    }
}

impl SimdBaseOps for I64x4 {
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        unsafe { I64x4(_mm256_add_epi64(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        unsafe { I64x4(_mm256_sub_epi64(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        unsafe {
            Self::load_from_array([
                self[0].wrapping_mul(rhs[0]),
                self[1].wrapping_mul(rhs[1]),
                self[2].wrapping_mul(rhs[2]),
                self[3].wrapping_mul(rhs[3]),
            ])
        }
    }

    #[inline(always)]
    fn bit_and(self, rhs: Self) -> Self {
        unsafe { I64x4(_mm256_and_si256(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_or(self, rhs: Self) -> Self {
        unsafe { I64x4(_mm256_or_si256(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_xor(self, rhs: Self) -> Self {
        unsafe { I64x4(_mm256_xor_si256(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_not(self) -> Self {
        unsafe { self.bit_xor(Self::set1(-1)) }
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
        unsafe { I64x4(_mm256_andnot_si256(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn blendv(self, a: Self, b: Self) -> Self {
        self.bitcast_f64()
            .blendv(a.bitcast_f64(), b.bitcast_f64())
            .bitcast_i64()
    }

    #[inline(always)]
    fn cmp_eq(self, rhs: Self) -> Self {
        unsafe { I64x4(_mm256_cmpeq_epi64(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn cmp_neq(self, rhs: Self) -> Self {
        self.cmp_eq(rhs).bit_not()
    }

    #[inline(always)]
    fn cmp_lt(self, rhs: Self) -> Self {
        unsafe { I64x4(_mm256_cmpgt_epi64(rhs.0, self.0)) }
    }

    #[inline(always)]
    fn cmp_lte(self, rhs: Self) -> Self {
        self.cmp_gt(rhs).bit_not()
    }

    #[inline(always)]
    fn cmp_gt(self, rhs: Self) -> Self {
        unsafe { I64x4(_mm256_cmpgt_epi64(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn cmp_gte(self, rhs: Self) -> Self {
        self.cmp_lt(rhs).bit_not()
    }

    #[inline(always)]
    fn max(self, rhs: Self) -> Self {
        // Manually implemented `v > w ? v : w`
        let mask = self.cmp_gt(rhs); // mask = v > w
        mask.blendv(rhs, self) // mask ? w : v
    }

    #[inline(always)]
    fn min(self, rhs: Self) -> Self {
        // Manually implemented `v < w ? v : w`
        let mask = self.cmp_lt(rhs); // mask = v < w
        mask.blendv(rhs, self) // mask ? w : v
    }
}

impl SimdInt for I64x4 {
    #[inline(always)]
    fn shl(self, rhs: i32) -> Self {
        unsafe {
            let rhs = _mm_cvtsi32_si128(rhs);
            I64x4(_mm256_sll_epi64(self.0, rhs))
        }
    }

    #[inline(always)]
    fn shr(self, rhs: i32) -> Self {
        unsafe {
            let rhs = _mm_cvtsi32_si128(rhs);
            I64x4(_mm256_srl_epi64(self.0, rhs))
        }
    }

    #[inline(always)]
    fn shl_const<const BY: i32>(self) -> Self {
        unsafe { I64x4(_mm256_slli_epi64(self.0, BY)) }
    }

    #[inline(always)]
    fn shr_const<const BY: i32>(self) -> Self {
        unsafe { I64x4(_mm256_srli_epi64(self.0, BY)) }
    }
}

impl SimdInt64 for I64x4 {
    type SimdF64 = F64x4;

    #[inline(always)]
    fn bitcast_f64(self) -> Self::SimdF64 {
        unsafe { F64x4(_mm256_castsi256_pd(self.0)) }
    }

    #[inline(always)]
    fn cast_f64(self) -> Self::SimdF64 {
        unsafe {
            Self::SimdF64::load_from_array([
                self[0] as f64,
                self[1] as f64,
                self[2] as f64,
                self[3] as f64,
            ])
        }
    }
}

define_simd_type!(f32, 8, __m256);
impl_simd_float_overloads!(F32x8);

impl InternalSimdBaseIo for F32x8 {
    #[inline(always)]
    unsafe fn zeroes() -> Self {
        F32x8(_mm256_setzero_ps())
    }

    #[inline(always)]
    unsafe fn set1(x: Self::Scalar) -> Self {
        F32x8(_mm256_set1_ps(x))
    }

    #[inline(always)]
    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        F32x8(_mm256_loadu_ps(array.as_ptr()))
    }

    #[inline(always)]
    unsafe fn load_from_ptr_unaligned(ptr: *const Self::Scalar) -> Self {
        F32x8(_mm256_loadu_ps(ptr))
    }

    #[inline(always)]
    unsafe fn copy_to_ptr_unaligned(self, ptr: *mut Self::Scalar) {
        _mm256_storeu_ps(ptr, self.0);
    }

    #[inline(always)]
    unsafe fn load_from_ptr_aligned(ptr: *const Self::Scalar) -> Self {
        F32x8(_mm256_load_ps(ptr))
    }

    #[inline(always)]
    unsafe fn copy_to_ptr_aligned(self, ptr: *mut Self::Scalar) {
        _mm256_store_ps(ptr, self.0);
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
        F32x8(value)
    }
}

impl SimdBaseOps for F32x8 {
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        unsafe { F32x8(_mm256_add_ps(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        unsafe { F32x8(_mm256_sub_ps(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        unsafe { F32x8(_mm256_mul_ps(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_and(self, rhs: Self) -> Self {
        unsafe { F32x8(_mm256_and_ps(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_or(self, rhs: Self) -> Self {
        unsafe { F32x8(_mm256_or_ps(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_xor(self, rhs: Self) -> Self {
        unsafe { F32x8(_mm256_xor_ps(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_not(self) -> Self {
        unsafe { self.bit_xor(I32x8::set1(-1).bitcast_f32()) }
    }

    #[inline(always)]
    fn abs(self) -> Self {
        unsafe {
            let b = _mm256_set1_ps(-0.0f32);
            F32x8(_mm256_andnot_ps(b, self.0))
        }
    }

    #[inline(always)]
    fn and_not(self, rhs: Self) -> Self {
        unsafe { F32x8(_mm256_andnot_ps(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn blendv(self, a: Self, b: Self) -> Self {
        unsafe { F32x8(_mm256_blendv_ps(a.0, b.0, self.0)) }
    }

    #[inline(always)]
    fn cmp_eq(self, rhs: Self) -> Self {
        unsafe { F32x8(_mm256_cmp_ps(self.0, rhs.0, _CMP_EQ_OQ)) }
    }

    #[inline(always)]
    fn cmp_neq(self, rhs: Self) -> Self {
        unsafe { F32x8(_mm256_cmp_ps(self.0, rhs.0, _CMP_NEQ_OQ)) }
    }

    #[inline(always)]
    fn cmp_lt(self, rhs: Self) -> Self {
        unsafe { F32x8(_mm256_cmp_ps(self.0, rhs.0, _CMP_LT_OQ)) }
    }

    #[inline(always)]
    fn cmp_lte(self, rhs: Self) -> Self {
        unsafe { F32x8(_mm256_cmp_ps(self.0, rhs.0, _CMP_LE_OQ)) }
    }

    #[inline(always)]
    fn cmp_gt(self, rhs: Self) -> Self {
        unsafe { F32x8(_mm256_cmp_ps(self.0, rhs.0, _CMP_GT_OQ)) }
    }

    #[inline(always)]
    fn cmp_gte(self, rhs: Self) -> Self {
        unsafe { F32x8(_mm256_cmp_ps(self.0, rhs.0, _CMP_GE_OQ)) }
    }

    #[inline(always)]
    fn max(self, rhs: Self) -> Self {
        unsafe { F32x8(_mm256_max_ps(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn min(self, rhs: Self) -> Self {
        unsafe { F32x8(_mm256_min_ps(self.0, rhs.0)) }
    }
}

impl SimdFloat for F32x8 {
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        unsafe { F32x8(_mm256_div_ps(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn ceil(self) -> Self {
        unsafe { F32x8(_mm256_ceil_ps(self.0)) }
    }

    #[inline(always)]
    fn floor(self) -> Self {
        unsafe { F32x8(_mm256_floor_ps(self.0)) }
    }

    #[inline(always)]
    fn round(self) -> Self {
        unsafe {
            F32x8(_mm256_round_ps(
                self.0,
                _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC,
            ))
        }
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
        unsafe { F32x8(_mm256_fmadd_ps(self.0, a.0, b.0)) }
    }

    #[inline(always)]
    fn mul_sub(self, a: Self, b: Self) -> Self {
        unsafe { F32x8(_mm256_fmsub_ps(self.0, a.0, b.0)) }
    }

    #[inline(always)]
    fn neg_mul_add(self, a: Self, b: Self) -> Self {
        unsafe { F32x8(_mm256_fnmadd_ps(self.0, a.0, b.0)) }
    }

    #[inline(always)]
    fn neg_mul_sub(self, a: Self, b: Self) -> Self {
        unsafe { F32x8(_mm256_fnmsub_ps(self.0, a.0, b.0)) }
    }

    #[inline(always)]
    fn horizontal_add(self) -> Self::Scalar {
        unsafe {
            let a = _mm256_hadd_ps(self.0, self.0);
            let b = _mm256_hadd_ps(a, a);
            let transmuted: Self::ArrayRepresentation = core::mem::transmute(b);
            transmuted[0] + transmuted[4]
        }
    }

    #[inline(always)]
    fn sqrt(self) -> Self {
        unsafe { F32x8(_mm256_sqrt_ps(self.0)) }
    }

    #[inline(always)]
    fn rsqrt(self) -> Self {
        unsafe { F32x8(_mm256_rsqrt_ps(self.0)) }
    }
}

impl SimdFloat32 for F32x8 {
    type SimdI32 = I32x8;

    #[inline(always)]
    fn bitcast_i32(self) -> Self::SimdI32 {
        unsafe { I32x8(_mm256_castps_si256(self.0)) }
    }

    #[inline(always)]
    fn cast_i32(self) -> Self::SimdI32 {
        unsafe { I32x8(_mm256_cvtps_epi32(self.0)) }
    }

    #[inline(always)]
    fn fast_inverse(self) -> Self {
        unsafe { F32x8(_mm256_rcp_ps(self.0)) }
    }
}

define_simd_type!(f64, 4, __m256d);
impl_simd_float_overloads!(F64x4);

impl InternalSimdBaseIo for F64x4 {
    #[inline(always)]
    unsafe fn zeroes() -> Self {
        F64x4(_mm256_setzero_pd())
    }

    #[inline(always)]
    unsafe fn set1(x: Self::Scalar) -> Self {
        F64x4(_mm256_set1_pd(x))
    }

    #[inline(always)]
    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        F64x4(_mm256_loadu_pd(array.as_ptr() as *const f64))
    }

    #[inline(always)]
    unsafe fn load_from_ptr_unaligned(ptr: *const Self::Scalar) -> Self {
        F64x4(_mm256_loadu_pd(ptr as *const f64))
    }

    #[inline(always)]
    unsafe fn copy_to_ptr_unaligned(self, ptr: *mut Self::Scalar) {
        _mm256_storeu_pd(ptr as *mut f64, self.0);
    }

    #[inline(always)]
    unsafe fn load_from_ptr_aligned(ptr: *const Self::Scalar) -> Self {
        F64x4(_mm256_load_pd(ptr as *const f64))
    }

    #[inline(always)]
    unsafe fn copy_to_ptr_aligned(self, ptr: *mut Self::Scalar) {
        _mm256_store_pd(ptr as *mut f64, self.0);
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
        F64x4(value)
    }
}

impl SimdBaseOps for F64x4 {
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        unsafe { F64x4(_mm256_add_pd(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        unsafe { F64x4(_mm256_sub_pd(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        unsafe { F64x4(_mm256_mul_pd(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_and(self, rhs: Self) -> Self {
        unsafe { F64x4(_mm256_and_pd(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_or(self, rhs: Self) -> Self {
        unsafe { F64x4(_mm256_or_pd(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_xor(self, rhs: Self) -> Self {
        unsafe { F64x4(_mm256_xor_pd(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn bit_not(self) -> Self {
        unsafe { self.bit_xor(I64x4::set1(-1).bitcast_f64()) }
    }

    #[inline(always)]
    fn abs(self) -> Self {
        unsafe {
            let b = _mm256_set1_pd(-0.0f64);
            F64x4(_mm256_andnot_pd(b, self.0))
        }
    }

    #[inline(always)]
    fn and_not(self, rhs: Self) -> Self {
        unsafe { F64x4(_mm256_andnot_pd(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn blendv(self, a: Self, b: Self) -> Self {
        unsafe { F64x4(_mm256_blendv_pd(a.0, b.0, self.0)) }
    }

    #[inline(always)]
    fn cmp_eq(self, rhs: Self) -> Self {
        unsafe { F64x4(_mm256_cmp_pd(self.0, rhs.0, _CMP_EQ_OQ)) }
    }

    #[inline(always)]
    fn cmp_neq(self, rhs: Self) -> Self {
        unsafe { F64x4(_mm256_cmp_pd(self.0, rhs.0, _CMP_NEQ_OQ)) }
    }

    #[inline(always)]
    fn cmp_lt(self, rhs: Self) -> Self {
        unsafe { F64x4(_mm256_cmp_pd(self.0, rhs.0, _CMP_LT_OQ)) }
    }

    #[inline(always)]
    fn cmp_lte(self, rhs: Self) -> Self {
        unsafe { F64x4(_mm256_cmp_pd(self.0, rhs.0, _CMP_LE_OQ)) }
    }

    #[inline(always)]
    fn cmp_gt(self, rhs: Self) -> Self {
        unsafe { F64x4(_mm256_cmp_pd(self.0, rhs.0, _CMP_GT_OQ)) }
    }

    #[inline(always)]
    fn cmp_gte(self, rhs: Self) -> Self {
        unsafe { F64x4(_mm256_cmp_pd(self.0, rhs.0, _CMP_GE_OQ)) }
    }

    #[inline(always)]
    fn max(self, rhs: Self) -> Self {
        unsafe { F64x4(_mm256_max_pd(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn min(self, rhs: Self) -> Self {
        unsafe { F64x4(_mm256_min_pd(self.0, rhs.0)) }
    }
}

impl SimdFloat for F64x4 {
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        unsafe { F64x4(_mm256_div_pd(self.0, rhs.0)) }
    }

    #[inline(always)]
    fn ceil(self) -> Self {
        unsafe { F64x4(_mm256_ceil_pd(self.0)) }
    }

    #[inline(always)]
    fn floor(self) -> Self {
        unsafe { F64x4(_mm256_floor_pd(self.0)) }
    }

    #[inline(always)]
    fn round(self) -> Self {
        unsafe { F64x4(_mm256_round_pd(self.0, _MM_FROUND_TO_NEAREST_INT)) }
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
        unsafe { F64x4(_mm256_fmadd_pd(self.0, a.0, b.0)) }
    }

    #[inline(always)]
    fn mul_sub(self, a: Self, b: Self) -> Self {
        unsafe { F64x4(_mm256_fmsub_pd(self.0, a.0, b.0)) }
    }

    #[inline(always)]
    fn neg_mul_add(self, a: Self, b: Self) -> Self {
        unsafe { F64x4(_mm256_fnmadd_pd(self.0, a.0, b.0)) }
    }

    #[inline(always)]
    fn neg_mul_sub(self, a: Self, b: Self) -> Self {
        unsafe { F64x4(_mm256_fnmsub_pd(self.0, a.0, b.0)) }
    }

    #[inline(always)]
    fn horizontal_add(self) -> Self::Scalar {
        unsafe {
            let a = _mm256_hadd_pd(self.0, self.0);
            let transmuted: Self::ArrayRepresentation = core::mem::transmute(a);
            transmuted[0] + transmuted[2]
        }
    }

    #[inline(always)]
    fn sqrt(self) -> Self {
        unsafe { F64x4(_mm256_sqrt_pd(self.0)) }
    }

    #[inline(always)]
    fn rsqrt(self) -> Self {
        unsafe { Self::set1(1.0) / self.sqrt() }
    }
}

impl SimdFloat64 for F64x4 {
    type SimdI64 = I64x4;

    #[inline(always)]
    fn bitcast_i64(self) -> Self::SimdI64 {
        unsafe { I64x4(_mm256_castpd_si256(self.0)) }
    }

    #[inline(always)]
    fn cast_i64(self) -> Self::SimdI64 {
        unsafe {
            Self::SimdI64::load_from_array([
                self[0].m_round() as i64,
                self[1].m_round() as i64,
                self[2].m_round() as i64,
                self[3].m_round() as i64,
            ])
        }
    }
}
