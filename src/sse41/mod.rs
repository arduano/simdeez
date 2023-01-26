use super::*;
#[cfg(target_arch = "x86")]
use core::arch::x86::*;

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

mod simd;
pub use self::overloads::*;
pub use self::simd::*;

#[derive(Copy, Clone)]
pub struct I16x8_41(pub __m128i);
impl_simd_base_overloads!(I16x8_41);
impl_simd_int_overloads!(I16x8_41);

impl SimdBase for I16x8_41 {
    const WIDTH: usize = 8;
    type Scalar = i16;
    type ArrayRepresentation = [i16; 8];
    type UnderlyingType = __m128i;

    unsafe fn zeroes() -> Self {
        I16x8_41(_mm_setzero_si128())
    }

    unsafe fn set1(x: Self::Scalar) -> Self {
        I16x8_41(_mm_set1_epi16(x))
    }

    fn add(self, rhs: Self) -> Self {
        unsafe { I16x8_41(_mm_add_epi16(self.0, rhs.0)) }
    }

    fn sub(self, rhs: Self) -> Self {
        unsafe { I16x8_41(_mm_sub_epi16(self.0, rhs.0)) }
    }

    fn mul(self, rhs: Self) -> Self {
        unsafe { I16x8_41(_mm_mullo_epi16(self.0, rhs.0)) }
    }

    fn bit_and(self, rhs: Self) -> Self {
        unsafe { I16x8_41(_mm_and_si128(self.0, rhs.0)) }
    }

    fn bit_or(self, rhs: Self) -> Self {
        unsafe { I16x8_41(_mm_or_si128(self.0, rhs.0)) }
    }

    fn bit_xor(self, rhs: Self) -> Self {
        unsafe { I16x8_41(_mm_xor_si128(self.0, rhs.0)) }
    }

    fn bit_not(self) -> Self {
        unsafe { Self::set1(-1).bit_xor(self) }
    }

    fn abs(self) -> Self {
        unsafe {
            let mask = self.cmp_lt(Self::zeroes());
            self.bit_xor(mask) - mask
        }
    }

    fn and_not(self, rhs: Self) -> Self {
        unsafe { I16x8_41(_mm_andnot_si128(self.0, rhs.0)) }
    }

    fn blendv(self, a: Self, b: Self) -> Self {
        unsafe {
            let mask = self.cmp_eq(Self::set1(-1));
            let a = mask.and_not(a);
            let b = mask.bit_and(b);
            a.bit_or(b)
        }
    }

    fn cmp_eq(self, rhs: Self) -> Self {
        unsafe { I16x8_41(_mm_cmpeq_epi16(self.0, rhs.0)) }
    }

    fn cmp_neq(self, rhs: Self) -> Self {
        self.cmp_eq(rhs).bit_not()
    }

    fn cmp_lt(self, rhs: Self) -> Self {
        unsafe { I16x8_41(_mm_cmplt_epi16(self.0, rhs.0)) }
    }

    fn cmp_lte(self, rhs: Self) -> Self {
        self.cmp_gt(rhs).bit_not()
    }

    fn cmp_gt(self, rhs: Self) -> Self {
        unsafe { I16x8_41(_mm_cmpgt_epi16(self.0, rhs.0)) }
    }

    fn cmp_gte(self, rhs: Self) -> Self {
        self.cmp_lt(rhs).bit_not()
    }

    fn max(self, rhs: Self) -> Self {
        unsafe { I16x8_41(_mm_max_epi16(self.0, rhs.0)) }
    }

    fn min(self, rhs: Self) -> Self {
        unsafe { I16x8_41(_mm_min_epi16(self.0, rhs.0)) }
    }

    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        I16x8_41(_mm_loadu_si128(array.as_ptr() as *const __m128i))
    }

    unsafe fn load_from_ptr(ptr: *const Self::Scalar) -> Self {
        I16x8_41(_mm_loadu_si128(ptr as *const __m128i))
    }

    unsafe fn copy_to_ptr(self, ptr: *mut Self::Scalar) {
        _mm_storeu_si128(ptr as *mut __m128i, self.0);
    }

    unsafe fn underlying_value(self) -> Self::UnderlyingType {
        self.0
    }

    unsafe fn underlying_value_mut(&mut self) -> &mut Self::UnderlyingType {
        &mut self.0
    }

    unsafe fn from_underlying_value(value: Self::UnderlyingType) -> Self {
        I16x8_41(value)
    }
}

impl SimdInt for I16x8_41 {
    fn shl(self, rhs: i32) -> Self {
        unsafe {
            let rhs = _mm_cvtsi32_si128(rhs);
            I16x8_41(_mm_sll_epi16(self.0, rhs))
        }
    }

    fn shr(self, rhs: i32) -> Self {
        unsafe {
            let rhs = _mm_cvtsi32_si128(rhs);
            I16x8_41(_mm_srl_epi16(self.0, rhs))
        }
    }

    fn shl_const<const BY: i32>(self) -> Self {
        unsafe { I16x8_41(_mm_slli_epi16(self.0, BY)) }
    }

    fn shr_const<const BY: i32>(self) -> Self {
        unsafe { I16x8_41(_mm_srli_epi16(self.0, BY)) }
    }
}

impl SimdInt16 for I16x8_41 {}

#[derive(Copy, Clone)]
pub struct I32x4_41(pub __m128i);
impl_simd_base_overloads!(I32x4_41);
impl_simd_int_overloads!(I32x4_41);

impl SimdBase for I32x4_41 {
    const WIDTH: usize = 4;
    type Scalar = i32;
    type ArrayRepresentation = [i32; 4];
    type UnderlyingType = __m128i;

    unsafe fn zeroes() -> Self {
        I32x4_41(_mm_setzero_si128())
    }

    unsafe fn set1(x: Self::Scalar) -> Self {
        I32x4_41(_mm_set1_epi32(x))
    }

    fn add(self, rhs: Self) -> Self {
        unsafe { I32x4_41(_mm_add_epi32(self.0, rhs.0)) }
    }

    fn sub(self, rhs: Self) -> Self {
        unsafe { I32x4_41(_mm_sub_epi32(self.0, rhs.0)) }
    }

    fn mul(self, rhs: Self) -> Self {
        unsafe { I32x4_41(_mm_mullo_epi32(self.0, rhs.0)) }
    }

    fn bit_and(self, rhs: Self) -> Self {
        unsafe { I32x4_41(_mm_and_si128(self.0, rhs.0)) }
    }

    fn bit_or(self, rhs: Self) -> Self {
        unsafe { I32x4_41(_mm_or_si128(self.0, rhs.0)) }
    }

    fn bit_xor(self, rhs: Self) -> Self {
        unsafe { I32x4_41(_mm_xor_si128(self.0, rhs.0)) }
    }

    fn bit_not(self) -> Self {
        unsafe { Self::set1(-1).bit_xor(self) }
    }

    fn abs(self) -> Self {
        unsafe {
            let mask = self.cmp_lt(Self::zeroes());
            self.bit_xor(mask) - mask
        }
    }

    fn and_not(self, rhs: Self) -> Self {
        unsafe { I32x4_41(_mm_andnot_si128(self.0, rhs.0)) }
    }

    fn blendv(self, a: Self, b: Self) -> Self {
        self.bitcast_f32()
            .blendv(a.bitcast_f32(), b.bitcast_f32())
            .bitcast_i32()
    }

    fn cmp_eq(self, rhs: Self) -> Self {
        unsafe { I32x4_41(_mm_cmpeq_epi32(self.0, rhs.0)) }
    }

    fn cmp_neq(self, rhs: Self) -> Self {
        self.cmp_eq(rhs).bit_not()
    }

    fn cmp_lt(self, rhs: Self) -> Self {
        unsafe { I32x4_41(_mm_cmplt_epi32(self.0, rhs.0)) }
    }

    fn cmp_lte(self, rhs: Self) -> Self {
        self.cmp_gt(rhs).bit_not()
    }

    fn cmp_gt(self, rhs: Self) -> Self {
        unsafe { I32x4_41(_mm_cmpgt_epi32(self.0, rhs.0)) }
    }

    fn cmp_gte(self, rhs: Self) -> Self {
        self.cmp_lt(rhs).bit_not()
    }

    fn max(self, rhs: Self) -> Self {
        unsafe { I32x4_41(_mm_max_epi32(self.0, rhs.0)) }
    }

    fn min(self, rhs: Self) -> Self {
        unsafe { I32x4_41(_mm_min_epi32(self.0, rhs.0)) }
    }

    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        I32x4_41(_mm_loadu_si128(array.as_ptr() as *const __m128i))
    }

    unsafe fn load_from_ptr(ptr: *const Self::Scalar) -> Self {
        I32x4_41(_mm_loadu_si128(ptr as *const __m128i))
    }

    unsafe fn copy_to_ptr(self, ptr: *mut Self::Scalar) {
        _mm_storeu_si128(ptr as *mut __m128i, self.0);
    }

    unsafe fn underlying_value(self) -> Self::UnderlyingType {
        self.0
    }

    unsafe fn underlying_value_mut(&mut self) -> &mut Self::UnderlyingType {
        &mut self.0
    }

    unsafe fn from_underlying_value(value: Self::UnderlyingType) -> Self {
        I32x4_41(value)
    }
}

impl SimdInt for I32x4_41 {
    fn shl(self, rhs: i32) -> Self {
        unsafe {
            let rhs = _mm_cvtsi32_si128(rhs);
            I32x4_41(_mm_sll_epi32(self.0, rhs))
        }
    }

    fn shr(self, rhs: i32) -> Self {
        unsafe {
            let rhs = _mm_cvtsi32_si128(rhs);
            I32x4_41(_mm_srl_epi32(self.0, rhs))
        }
    }

    fn shl_const<const BY: i32>(self) -> Self {
        unsafe { I32x4_41(_mm_slli_epi32(self.0, BY)) }
    }

    fn shr_const<const BY: i32>(self) -> Self {
        unsafe { I32x4_41(_mm_srli_epi32(self.0, BY)) }
    }
}

impl SimdInt32 for I32x4_41 {
    type SimdF32 = F32x4_41;

    fn bitcast_f32(self) -> Self::SimdF32 {
        unsafe { F32x4_41(_mm_castsi128_ps(self.0)) }
    }

    fn cast_f32(self) -> Self::SimdF32 {
        unsafe { F32x4_41(_mm_cvtepi32_ps(self.0)) }
    }
}

#[derive(Copy, Clone)]
pub struct I64x2_41(pub __m128i);
impl_simd_base_overloads!(I64x2_41);
impl_simd_int_overloads!(I64x2_41);

impl SimdBase for I64x2_41 {
    const WIDTH: usize = 2;
    type Scalar = i64;
    type ArrayRepresentation = [i64; 2];
    type UnderlyingType = __m128i;

    unsafe fn zeroes() -> Self {
        I64x2_41(_mm_setzero_si128())
    }

    unsafe fn set1(x: Self::Scalar) -> Self {
        I64x2_41(_mm_set1_epi64x(x))
    }

    fn add(self, rhs: Self) -> Self {
        unsafe { I64x2_41(_mm_add_epi64(self.0, rhs.0)) }
    }

    fn sub(self, rhs: Self) -> Self {
        unsafe { I64x2_41(_mm_sub_epi64(self.0, rhs.0)) }
    }

    fn mul(self, rhs: Self) -> Self {
        unsafe {
            let mut new = Self::zeroes();
            new[0] = self[0].wrapping_mul(rhs[0]);
            new[1] = self[1].wrapping_mul(rhs[1]);
            new
        }
    }

    fn bit_and(self, rhs: Self) -> Self {
        unsafe { I64x2_41(_mm_and_si128(self.0, rhs.0)) }
    }

    fn bit_or(self, rhs: Self) -> Self {
        unsafe { I64x2_41(_mm_or_si128(self.0, rhs.0)) }
    }

    fn bit_xor(self, rhs: Self) -> Self {
        unsafe { I64x2_41(_mm_xor_si128(self.0, rhs.0)) }
    }

    fn bit_not(self) -> Self {
        unsafe { Self::set1(-1).bit_xor(self) }
    }

    fn abs(self) -> Self {
        unsafe {
            let mask = self.cmp_lt(Self::zeroes());
            self.bit_xor(mask) - mask
        }
    }

    fn and_not(self, rhs: Self) -> Self {
        unsafe { I64x2_41(_mm_andnot_si128(self.0, rhs.0)) }
    }

    fn blendv(self, a: Self, b: Self) -> Self {
        self.bitcast_f64()
            .blendv(a.bitcast_f64(), b.bitcast_f64())
            .bitcast_i64()
    }

    fn cmp_eq(self, rhs: Self) -> Self {
        unsafe { I64x2_41(_mm_cmpeq_epi64(self.0, rhs.0)) }
    }

    fn cmp_neq(self, rhs: Self) -> Self {
        self.cmp_eq(rhs).bit_not()
    }

    fn cmp_lt(self, rhs: Self) -> Self {
        unsafe {
            let mut new = Self::zeroes();
            new[0] = if self[0] < rhs[0] { -1 } else { 0 };
            new[1] = if self[1] < rhs[1] { -1 } else { 0 };
            new
        }
    }

    fn cmp_lte(self, rhs: Self) -> Self {
        unsafe {
            let mut new = Self::zeroes();
            new[0] = if self[0] <= rhs[0] { -1 } else { 0 };
            new[1] = if self[1] <= rhs[1] { -1 } else { 0 };
            new
        }
    }

    fn cmp_gt(self, rhs: Self) -> Self {
        unsafe {
            let mut new = Self::zeroes();
            new[0] = if self[0] > rhs[0] { -1 } else { 0 };
            new[1] = if self[1] > rhs[1] { -1 } else { 0 };
            new
        }
    }

    fn cmp_gte(self, rhs: Self) -> Self {
        unsafe {
            let mut new = Self::zeroes();
            new[0] = if self[0] >= rhs[0] { -1 } else { 0 };
            new[1] = if self[1] >= rhs[1] { -1 } else { 0 };
            new
        }
    }

    fn max(self, rhs: Self) -> Self {
        let cmp = self.cmp_gt(rhs);
        cmp.blendv(rhs, self)
    }

    fn min(self, rhs: Self) -> Self {
        let cmp = self.cmp_lt(rhs);
        cmp.blendv(rhs, self)
    }

    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        I64x2_41(_mm_loadu_si128(array.as_ptr() as *const _))
    }

    unsafe fn load_from_ptr(ptr: *const Self::Scalar) -> Self {
        I64x2_41(_mm_loadu_si128(ptr as *const _))
    }

    unsafe fn copy_to_ptr(self, ptr: *mut Self::Scalar) {
        _mm_storeu_si128(ptr as *mut _, self.0);
    }

    unsafe fn underlying_value(self) -> Self::UnderlyingType {
        self.0
    }

    unsafe fn underlying_value_mut(&mut self) -> &mut Self::UnderlyingType {
        &mut self.0
    }

    unsafe fn from_underlying_value(value: Self::UnderlyingType) -> Self {
        I64x2_41(value)
    }
}

impl SimdInt for I64x2_41 {
    fn shl(self, rhs: i32) -> Self {
        unsafe {
            let rhs = _mm_cvtsi32_si128(rhs);
            I64x2_41(_mm_sll_epi64(self.0, rhs))
        }
    }

    fn shr(self, rhs: i32) -> Self {
        unsafe {
            let rhs = _mm_cvtsi32_si128(rhs);
            I64x2_41(_mm_srl_epi64(self.0, rhs))
        }
    }

    fn shl_const<const BY: i32>(self) -> Self {
        unsafe { I64x2_41(_mm_slli_epi64(self.0, BY)) }
    }

    fn shr_const<const BY: i32>(self) -> Self {
        unsafe { I64x2_41(_mm_srli_epi64(self.0, BY)) }
    }
}

impl SimdInt64 for I64x2_41 {
    type SimdF64 = F64x2_41;

    fn bitcast_f64(self) -> Self::SimdF64 {
        unsafe { F64x2_41(_mm_castsi128_pd(self.0)) }
    }

    fn cast_f64(self) -> Self::SimdF64 {
        unsafe {
            let x = self + Self::set1(0x0018000000000000);
            x.bitcast_f64() - Self::set1(0x0018000000000000).bitcast_f64()
        }
    }
}

#[derive(Copy, Clone)]
pub struct F32x4_41(pub __m128);
impl_simd_base_overloads!(F32x4_41);
impl_simd_float_overloads!(F32x4_41);

impl SimdBase for F32x4_41 {
    const WIDTH: usize = 4;
    type Scalar = f32;
    type ArrayRepresentation = [f32; 4];
    type UnderlyingType = __m128;

    unsafe fn zeroes() -> Self {
        F32x4_41(_mm_setzero_ps())
    }

    unsafe fn set1(x: Self::Scalar) -> Self {
        F32x4_41(_mm_set1_ps(x))
    }

    fn add(self, rhs: Self) -> Self {
        unsafe { F32x4_41(_mm_add_ps(self.0, rhs.0)) }
    }

    fn sub(self, rhs: Self) -> Self {
        unsafe { F32x4_41(_mm_sub_ps(self.0, rhs.0)) }
    }

    fn mul(self, rhs: Self) -> Self {
        unsafe { F32x4_41(_mm_mul_ps(self.0, rhs.0)) }
    }

    fn bit_and(self, rhs: Self) -> Self {
        unsafe { F32x4_41(_mm_and_ps(self.0, rhs.0)) }
    }

    fn bit_or(self, rhs: Self) -> Self {
        unsafe { F32x4_41(_mm_or_ps(self.0, rhs.0)) }
    }

    fn bit_xor(self, rhs: Self) -> Self {
        unsafe { F32x4_41(_mm_xor_ps(self.0, rhs.0)) }
    }

    fn bit_not(self) -> Self {
        unsafe { I32x4_41::set1(-1).bitcast_f32().bit_xor(self) }
    }

    fn abs(self) -> Self {
        unsafe { Self::set1(-0.0).and_not(self) }
    }

    fn and_not(self, rhs: Self) -> Self {
        unsafe { F32x4_41(_mm_andnot_ps(self.0, rhs.0)) }
    }

    fn blendv(self, a: Self, b: Self) -> Self {
        unsafe { F32x4_41(_mm_blendv_ps(a.0, b.0, self.0)) }
    }

    fn cmp_eq(self, rhs: Self) -> Self {
        unsafe { F32x4_41(_mm_cmpeq_ps(self.0, rhs.0)) }
    }

    fn cmp_neq(self, rhs: Self) -> Self {
        unsafe { F32x4_41(_mm_cmpneq_ps(self.0, rhs.0)) }
    }

    fn cmp_lt(self, rhs: Self) -> Self {
        unsafe { F32x4_41(_mm_cmplt_ps(self.0, rhs.0)) }
    }

    fn cmp_lte(self, rhs: Self) -> Self {
        unsafe { F32x4_41(_mm_cmple_ps(self.0, rhs.0)) }
    }

    fn cmp_gt(self, rhs: Self) -> Self {
        unsafe { F32x4_41(_mm_cmpgt_ps(self.0, rhs.0)) }
    }

    fn cmp_gte(self, rhs: Self) -> Self {
        unsafe { F32x4_41(_mm_cmpge_ps(self.0, rhs.0)) }
    }

    fn max(self, rhs: Self) -> Self {
        unsafe { F32x4_41(_mm_max_ps(self.0, rhs.0)) }
    }

    fn min(self, rhs: Self) -> Self {
        unsafe { F32x4_41(_mm_min_ps(self.0, rhs.0)) }
    }

    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        F32x4_41(_mm_loadu_ps(array.as_ptr()))
    }

    unsafe fn load_from_ptr(ptr: *const Self::Scalar) -> Self {
        F32x4_41(_mm_loadu_ps(ptr as *const f32))
    }

    unsafe fn copy_to_ptr(self, ptr: *mut Self::Scalar) {
        _mm_storeu_ps(ptr as *mut f32, self.0);
    }

    unsafe fn underlying_value(self) -> Self::UnderlyingType {
        self.0
    }

    unsafe fn underlying_value_mut(&mut self) -> &mut Self::UnderlyingType {
        &mut self.0
    }

    unsafe fn from_underlying_value(value: Self::UnderlyingType) -> Self {
        F32x4_41(value)
    }
}

impl SimdFloat for F32x4_41 {
    fn div(self, rhs: Self) -> Self {
        unsafe { F32x4_41(_mm_div_ps(self.0, rhs.0)) }
    }

    fn ceil(self) -> Self {
        unsafe { F32x4_41(_mm_ceil_ps(self.0)) }
    }

    fn floor(self) -> Self {
        unsafe { F32x4_41(_mm_floor_ps(self.0)) }
    }

    fn round(self) -> Self {
        unsafe {
            F32x4_41(_mm_round_ps(
                self.0,
                _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC,
            ))
        }
    }

    fn fast_ceil(self) -> Self {
        self.ceil()
    }

    fn fast_floor(self) -> Self {
        self.floor()
    }

    fn fast_round(self) -> Self {
        self.round()
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        self * a + b
    }

    fn mul_sub(self, a: Self, b: Self) -> Self {
        self * a - b
    }

    fn neg_mul_add(self, a: Self, b: Self) -> Self {
        -self * a + b
    }

    fn neg_mul_sub(self, a: Self, b: Self) -> Self {
        -self * a - b
    }

    fn horizontal_add(self) -> Self::Scalar {
        unsafe {
            let mut tmp = _mm_hadd_ps(self.0, self.0);
            tmp = _mm_hadd_ps(tmp, tmp);
            _mm_cvtss_f32(tmp)
        }
    }

    fn sqrt(self) -> Self {
        unsafe { F32x4_41(_mm_sqrt_ps(self.0)) }
    }

    fn rsqrt(self) -> Self {
        unsafe { F32x4_41(_mm_rsqrt_ps(self.0)) }
    }
}

impl SimdFloat32 for F32x4_41 {
    type SimdI32 = I32x4_41;

    fn bitcast_i32(self) -> Self::SimdI32 {
        unsafe { I32x4_41(_mm_castps_si128(self.0)) }
    }

    fn cast_i32(self) -> Self::SimdI32 {
        unsafe { I32x4_41(_mm_cvtps_epi32(self.0)) }
    }

    fn fast_inverse(self) -> Self {
        unsafe { F32x4_41(_mm_rcp_ps(self.0)) }
    }
}

#[derive(Copy, Clone)]
pub struct F64x2_41(pub __m128d);
impl_simd_base_overloads!(F64x2_41);
impl_simd_float_overloads!(F64x2_41);

impl SimdBase for F64x2_41 {
    const WIDTH: usize = 2;
    type Scalar = f64;
    type ArrayRepresentation = [f64; 2];
    type UnderlyingType = __m128d;

    unsafe fn zeroes() -> Self {
        F64x2_41(_mm_setzero_pd())
    }

    unsafe fn set1(x: Self::Scalar) -> Self {
        F64x2_41(_mm_set1_pd(x))
    }

    fn add(self, rhs: Self) -> Self {
        unsafe { F64x2_41(_mm_add_pd(self.0, rhs.0)) }
    }

    fn sub(self, rhs: Self) -> Self {
        unsafe { F64x2_41(_mm_sub_pd(self.0, rhs.0)) }
    }

    fn mul(self, rhs: Self) -> Self {
        unsafe { F64x2_41(_mm_mul_pd(self.0, rhs.0)) }
    }

    fn bit_and(self, rhs: Self) -> Self {
        unsafe { F64x2_41(_mm_and_pd(self.0, rhs.0)) }
    }

    fn bit_or(self, rhs: Self) -> Self {
        unsafe { F64x2_41(_mm_or_pd(self.0, rhs.0)) }
    }

    fn bit_xor(self, rhs: Self) -> Self {
        unsafe { F64x2_41(_mm_xor_pd(self.0, rhs.0)) }
    }

    fn bit_not(self) -> Self {
        unsafe { I64x2_41::set1(-1).bitcast_f64().bit_xor(self) }
    }

    fn abs(self) -> Self {
        unsafe { Self::set1(-0.0).and_not(self) }
    }

    fn and_not(self, rhs: Self) -> Self {
        unsafe { F64x2_41(_mm_andnot_pd(self.0, rhs.0)) }
    }

    fn blendv(self, a: Self, b: Self) -> Self {
        unsafe { F64x2_41(_mm_blendv_pd(a.0, b.0, self.0)) }
    }

    fn cmp_eq(self, rhs: Self) -> Self {
        unsafe { F64x2_41(_mm_cmpeq_pd(self.0, rhs.0)) }
    }

    fn cmp_neq(self, rhs: Self) -> Self {
        unsafe { F64x2_41(_mm_cmpneq_pd(self.0, rhs.0)) }
    }

    fn cmp_lt(self, rhs: Self) -> Self {
        unsafe { F64x2_41(_mm_cmplt_pd(self.0, rhs.0)) }
    }

    fn cmp_lte(self, rhs: Self) -> Self {
        unsafe { F64x2_41(_mm_cmple_pd(self.0, rhs.0)) }
    }

    fn cmp_gt(self, rhs: Self) -> Self {
        unsafe { F64x2_41(_mm_cmpgt_pd(self.0, rhs.0)) }
    }

    fn cmp_gte(self, rhs: Self) -> Self {
        unsafe { F64x2_41(_mm_cmpge_pd(self.0, rhs.0)) }
    }

    fn max(self, rhs: Self) -> Self {
        unsafe { F64x2_41(_mm_max_pd(self.0, rhs.0)) }
    }

    fn min(self, rhs: Self) -> Self {
        unsafe { F64x2_41(_mm_min_pd(self.0, rhs.0)) }
    }

    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        F64x2_41(_mm_loadu_pd(array.as_ptr()))
    }

    unsafe fn load_from_ptr(ptr: *const Self::Scalar) -> Self {
        F64x2_41(_mm_loadu_pd(ptr))
    }

    unsafe fn copy_to_ptr(self, ptr: *mut Self::Scalar) {
        _mm_storeu_pd(ptr, self.0);
    }

    unsafe fn underlying_value(self) -> Self::UnderlyingType {
        self.0
    }

    unsafe fn underlying_value_mut(&mut self) -> &mut Self::UnderlyingType {
        &mut self.0
    }

    unsafe fn from_underlying_value(value: Self::UnderlyingType) -> Self {
        F64x2_41(value)
    }
}

impl SimdFloat for F64x2_41 {
    fn div(self, rhs: Self) -> Self {
        unsafe { F64x2_41(_mm_div_pd(self.0, rhs.0)) }
    }

    fn ceil(self) -> Self {
        unsafe { F64x2_41(_mm_ceil_pd(self.0)) }
    }

    fn floor(self) -> Self {
        unsafe { F64x2_41(_mm_floor_pd(self.0)) }
    }

    fn round(self) -> Self {
        unsafe {
            F64x2_41(_mm_round_pd(
                self.0,
                _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC,
            ))
        }
    }

    fn fast_ceil(self) -> Self {
        self.ceil()
    }

    fn fast_floor(self) -> Self {
        self.floor()
    }

    fn fast_round(self) -> Self {
        self.round()
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        self * a + b
    }

    fn mul_sub(self, a: Self, b: Self) -> Self {
        self * a - b
    }

    fn neg_mul_add(self, a: Self, b: Self) -> Self {
        -self * a + b
    }

    fn neg_mul_sub(self, a: Self, b: Self) -> Self {
        -self * a - b
    }

    fn horizontal_add(self) -> Self::Scalar {
        unsafe {
            let arr = self.transmute_into_array_ref();
            arr[0] + arr[1]
        }
    }

    fn sqrt(self) -> Self {
        unsafe { F64x2_41(_mm_sqrt_pd(self.0)) }
    }

    fn rsqrt(self) -> Self {
        unsafe { Self::set1(1.0) / self.sqrt() }
    }
}

impl SimdFloat64 for F64x2_41 {
    type SimdI64 = I64x2_41;

    fn bitcast_i64(self) -> Self::SimdI64 {
        unsafe { I64x2_41(_mm_castpd_si128(self.0)) }
    }

    fn cast_i64(self) -> Self::SimdI64 {
        unsafe {
            let x = self + Self::set1(f64::from_bits(0x0018000000000000));
            x.bitcast_i64() - I64x2_41::set1(0x0018000000000000)
        }
    }
}
