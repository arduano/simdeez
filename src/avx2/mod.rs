use super::*;
use core::arch::x86_64::*;

mod simd;
pub use self::overloads::*;
pub use self::simd::*;

#[derive(Copy, Clone)]
pub struct I16x16(pub __m256i);
impl_simd_base_overloads!(I16x16);
impl_simd_int_overloads!(I16x16);

impl SimdBase for I16x16 {
    const WIDTH: usize = 16;
    type Scalar = i16;
    type ArrayRepresentation = [i16; 16];
    type UnderlyingType = __m256i;

    unsafe fn zeroes() -> Self {
        I16x16(_mm256_setzero_si256())
    }

    unsafe fn set1(x: Self::Scalar) -> Self {
        I16x16(_mm256_set1_epi16(x))
    }

    unsafe fn add(self, rhs: Self) -> Self {
        I16x16(_mm256_add_epi16(self.0, rhs.0))
    }

    unsafe fn sub(self, rhs: Self) -> Self {
        I16x16(_mm256_sub_epi16(self.0, rhs.0))
    }

    unsafe fn mul(self, rhs: Self) -> Self {
        I16x16(_mm256_mullo_epi16(self.0, rhs.0))
    }

    unsafe fn bit_and(self, rhs: Self) -> Self {
        I16x16(_mm256_and_si256(self.0, rhs.0))
    }

    unsafe fn bit_or(self, rhs: Self) -> Self {
        I16x16(_mm256_or_si256(self.0, rhs.0))
    }

    unsafe fn bit_xor(self, rhs: Self) -> Self {
        I16x16(_mm256_xor_si256(self.0, rhs.0))
    }

    unsafe fn bit_not(self) -> Self {
        self.bit_xor(Self::set1(-1))
    }

    unsafe fn abs(self) -> Self {
        I16x16(_mm256_abs_epi16(self.0))
    }

    unsafe fn and_not(self, rhs: Self) -> Self {
        I16x16(_mm256_andnot_si256(self.0, rhs.0))
    }

    unsafe fn blendv(self, a: Self, b: Self) -> Self {
        I16x16(_mm256_blendv_epi8(a.0, b.0, self.0))
    }

    unsafe fn cmp_eq(self, rhs: Self) -> Self {
        I16x16(_mm256_cmpeq_epi16(self.0, rhs.0))
    }

    unsafe fn cmp_neq(self, rhs: Self) -> Self {
        self.cmp_eq(rhs).bit_not()
    }

    unsafe fn cmp_lt(self, rhs: Self) -> Self {
        I16x16(_mm256_cmpgt_epi16(rhs.0, self.0))
    }

    unsafe fn cmp_lte(self, rhs: Self) -> Self {
        self.cmp_gt(rhs).bit_not()
    }

    unsafe fn cmp_gt(self, rhs: Self) -> Self {
        I16x16(_mm256_cmpgt_epi16(self.0, rhs.0))
    }

    unsafe fn cmp_gte(self, rhs: Self) -> Self {
        self.cmp_lt(rhs).bit_not()
    }

    unsafe fn max(self, rhs: Self) -> Self {
        I16x16(_mm256_max_epi16(self.0, rhs.0))
    }

    unsafe fn min(self, rhs: Self) -> Self {
        I16x16(_mm256_min_epi16(self.0, rhs.0))
    }

    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        I16x16(_mm256_loadu_si256(array.as_ptr() as *const _))
    }
}

impl SimdInt for I16x16 {
    unsafe fn shl(self, rhs: i32) -> Self {
        let rhs = _mm_cvtsi32_si128(rhs);
        I16x16(_mm256_srl_epi16(self.0, rhs))
    }

    unsafe fn shr(self, rhs: i32) -> Self {
        let rhs = _mm_cvtsi32_si128(rhs);
        I16x16(_mm256_sll_epi16(self.0, rhs))
    }

    unsafe fn shl_const<const BY: i32>(self) -> Self {
        I16x16(_mm256_slli_epi16(self.0, BY))
    }

    unsafe fn shr_const<const BY: i32>(self) -> Self {
        I16x16(_mm256_srli_epi16(self.0, BY))
    }
}

impl SimdInt16 for I16x16 {}

#[derive(Copy, Clone)]
pub struct I32x8(pub __m256i);
impl_simd_base_overloads!(I32x8);
impl_simd_int_overloads!(I32x8);

impl SimdBase for I32x8 {
    const WIDTH: usize = 8;
    type Scalar = i32;
    type ArrayRepresentation = [i32; 8];
    type UnderlyingType = __m256i;

    unsafe fn zeroes() -> Self {
        I32x8(_mm256_setzero_si256())
    }

    unsafe fn set1(x: Self::Scalar) -> Self {
        I32x8(_mm256_set1_epi32(x))
    }

    unsafe fn add(self, rhs: Self) -> Self {
        I32x8(_mm256_add_epi32(self.0, rhs.0))
    }

    unsafe fn sub(self, rhs: Self) -> Self {
        I32x8(_mm256_sub_epi32(self.0, rhs.0))
    }

    unsafe fn mul(self, rhs: Self) -> Self {
        I32x8(_mm256_mullo_epi32(self.0, rhs.0))
    }

    unsafe fn bit_and(self, rhs: Self) -> Self {
        I32x8(_mm256_and_si256(self.0, rhs.0))
    }

    unsafe fn bit_or(self, rhs: Self) -> Self {
        I32x8(_mm256_or_si256(self.0, rhs.0))
    }

    unsafe fn bit_xor(self, rhs: Self) -> Self {
        I32x8(_mm256_xor_si256(self.0, rhs.0))
    }

    unsafe fn bit_not(self) -> Self {
        self.bit_xor(Self::set1(-1))
    }

    unsafe fn abs(self) -> Self {
        I32x8(_mm256_abs_epi32(self.0))
    }

    unsafe fn and_not(self, rhs: Self) -> Self {
        I32x8(_mm256_andnot_si256(self.0, rhs.0))
    }

    unsafe fn blendv(self, a: Self, b: Self) -> Self {
        I32x8(_mm256_blendv_epi8(a.0, b.0, self.0))
    }

    unsafe fn cmp_eq(self, rhs: Self) -> Self {
        I32x8(_mm256_cmpeq_epi32(self.0, rhs.0))
    }

    unsafe fn cmp_neq(self, rhs: Self) -> Self {
        self.cmp_eq(rhs).bit_not()
    }

    unsafe fn cmp_lt(self, rhs: Self) -> Self {
        I32x8(_mm256_cmpgt_epi32(rhs.0, self.0))
    }

    unsafe fn cmp_lte(self, rhs: Self) -> Self {
        self.cmp_gt(rhs).bit_not()
    }

    unsafe fn cmp_gt(self, rhs: Self) -> Self {
        I32x8(_mm256_cmpgt_epi32(self.0, rhs.0))
    }

    unsafe fn cmp_gte(self, rhs: Self) -> Self {
        self.cmp_lt(rhs).bit_not()
    }

    unsafe fn max(self, rhs: Self) -> Self {
        I32x8(_mm256_max_epi32(self.0, rhs.0))
    }

    unsafe fn min(self, rhs: Self) -> Self {
        I32x8(_mm256_min_epi32(self.0, rhs.0))
    }

    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        I32x8(_mm256_loadu_si256(array.as_ptr() as *const _))
    }
}

impl SimdInt for I32x8 {
    unsafe fn shl(self, rhs: i32) -> Self {
        let rhs = _mm_cvtsi32_si128(rhs);
        I32x8(_mm256_sll_epi32(self.0, rhs))
    }

    unsafe fn shr(self, rhs: i32) -> Self {
        let rhs = _mm_cvtsi32_si128(rhs);
        I32x8(_mm256_srl_epi32(self.0, rhs))
    }

    unsafe fn shl_const<const BY: i32>(self) -> Self {
        I32x8(_mm256_slli_epi32(self.0, BY))
    }

    unsafe fn shr_const<const BY: i32>(self) -> Self {
        I32x8(_mm256_srli_epi32(self.0, BY))
    }
}

impl SimdInt32 for I32x8 {
    type SimdF32 = F32x8;

    unsafe fn bitcast_f32(self) -> Self::SimdF32 {
        F32x8(_mm256_castsi256_ps(self.0))
    }

    unsafe fn cast_f32(self) -> Self::SimdF32 {
        F32x8(_mm256_cvtepi32_ps(self.0))
    }
}

#[derive(Copy, Clone)]
pub struct I64x4(pub __m256i);
impl_simd_base_overloads!(I64x4);
impl_simd_int_overloads!(I64x4);

impl SimdBase for I64x4 {
    const WIDTH: usize = 4;
    type Scalar = i64;
    type ArrayRepresentation = [i64; 4];
    type UnderlyingType = __m256i;

    unsafe fn zeroes() -> Self {
        I64x4(_mm256_setzero_si256())
    }

    unsafe fn set1(x: Self::Scalar) -> Self {
        I64x4(_mm256_set1_epi64x(x))
    }

    unsafe fn add(self, rhs: Self) -> Self {
        I64x4(_mm256_add_epi64(self.0, rhs.0))
    }

    unsafe fn sub(self, rhs: Self) -> Self {
        I64x4(_mm256_sub_epi64(self.0, rhs.0))
    }

    unsafe fn mul(self, rhs: Self) -> Self {
        I64x4(_mm256_mul_epi32(self.0, rhs.0))
    }

    unsafe fn bit_and(self, rhs: Self) -> Self {
        I64x4(_mm256_and_si256(self.0, rhs.0))
    }

    unsafe fn bit_or(self, rhs: Self) -> Self {
        I64x4(_mm256_or_si256(self.0, rhs.0))
    }

    unsafe fn bit_xor(self, rhs: Self) -> Self {
        I64x4(_mm256_xor_si256(self.0, rhs.0))
    }

    unsafe fn bit_not(self) -> Self {
        self.bit_xor(Self::set1(-1))
    }

    unsafe fn abs(self) -> Self {
        // Manually implemented `v < 0 ? -v : v`
        let zero = I64x4::set1(0); // zero
        let mask = self.cmp_lt(zero); // mask = v < 0
        let neg = zero - self; // neg = -v
        mask.blendv(self, neg) // mask ? neg : v
    }

    unsafe fn and_not(self, rhs: Self) -> Self {
        I64x4(_mm256_andnot_si256(self.0, rhs.0))
    }

    unsafe fn blendv(self, a: Self, b: Self) -> Self {
        I64x4(_mm256_blendv_epi8(a.0, b.0, self.0))
    }

    unsafe fn cmp_eq(self, rhs: Self) -> Self {
        I64x4(_mm256_cmpeq_epi64(self.0, rhs.0))
    }

    unsafe fn cmp_neq(self, rhs: Self) -> Self {
        self.cmp_eq(rhs).bit_not()
    }

    unsafe fn cmp_lt(self, rhs: Self) -> Self {
        I64x4(_mm256_cmpgt_epi64(rhs.0, self.0))
    }

    unsafe fn cmp_lte(self, rhs: Self) -> Self {
        self.cmp_gt(rhs).bit_not()
    }

    unsafe fn cmp_gt(self, rhs: Self) -> Self {
        I64x4(_mm256_cmpgt_epi64(self.0, rhs.0))
    }

    unsafe fn cmp_gte(self, rhs: Self) -> Self {
        self.cmp_lt(rhs).bit_not()
    }

    unsafe fn max(self, rhs: Self) -> Self {
        // Manually implemented `v > w ? v : w`
        let mask = self.cmp_gt(rhs); // mask = v > w
        mask.blendv(rhs, self) // mask ? w : v
    }

    unsafe fn min(self, rhs: Self) -> Self {
        // Manually implemented `v < w ? v : w`
        let mask = self.cmp_lt(rhs); // mask = v < w
        mask.blendv(rhs, self) // mask ? w : v
    }

    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        I64x4(_mm256_loadu_si256(array.as_ptr() as *const _))
    }
}

impl SimdInt for I64x4 {
    unsafe fn shl(self, rhs: i32) -> Self {
        let rhs = _mm_cvtsi32_si128(rhs);
        I64x4(_mm256_sll_epi64(self.0, rhs))
    }

    unsafe fn shr(self, rhs: i32) -> Self {
        let rhs = _mm_cvtsi32_si128(rhs);
        I64x4(_mm256_srl_epi64(self.0, rhs))
    }

    unsafe fn shl_const<const BY: i32>(self) -> Self {
        I64x4(_mm256_slli_epi64(self.0, BY))
    }

    unsafe fn shr_const<const BY: i32>(self) -> Self {
        I64x4(_mm256_srli_epi64(self.0, BY))
    }
}

impl SimdInt64 for I64x4 {
    type SimdF64 = F64x4;

    unsafe fn bitcast_f64(self) -> Self::SimdF64 {
        todo!()
    }

    unsafe fn cast_f64(self) -> Self::SimdF64 {
        todo!()
    }
}

#[derive(Copy, Clone)]
pub struct F32x8(pub __m256);
impl_simd_base_overloads!(F32x8);
impl_simd_float_overloads!(F32x8);

impl SimdBase for F32x8 {
    const WIDTH: usize = 8;
    type Scalar = f32;
    type ArrayRepresentation = [f32; 8];
    type UnderlyingType = __m256;

    unsafe fn zeroes() -> Self {
        F32x8(_mm256_setzero_ps())
    }

    unsafe fn set1(x: Self::Scalar) -> Self {
        F32x8(_mm256_set1_ps(x))
    }

    unsafe fn add(self, rhs: Self) -> Self {
        F32x8(_mm256_add_ps(self.0, rhs.0))
    }

    unsafe fn sub(self, rhs: Self) -> Self {
        F32x8(_mm256_sub_ps(self.0, rhs.0))
    }

    unsafe fn mul(self, rhs: Self) -> Self {
        F32x8(_mm256_mul_ps(self.0, rhs.0))
    }

    unsafe fn bit_and(self, rhs: Self) -> Self {
        F32x8(_mm256_and_ps(self.0, rhs.0))
    }

    unsafe fn bit_or(self, rhs: Self) -> Self {
        F32x8(_mm256_or_ps(self.0, rhs.0))
    }

    unsafe fn bit_xor(self, rhs: Self) -> Self {
        F32x8(_mm256_xor_ps(self.0, rhs.0))
    }

    unsafe fn bit_not(self) -> Self {
        self.bit_xor(I32x8::set1(-1).cast_f32())
    }

    unsafe fn abs(self) -> Self {
        let b = _mm256_set1_ps(-0.0f32);
        F32x8(_mm256_andnot_ps(b, self.0))
    }

    unsafe fn and_not(self, rhs: Self) -> Self {
        F32x8(_mm256_andnot_ps(self.0, rhs.0))
    }

    unsafe fn blendv(self, a: Self, b: Self) -> Self {
        F32x8(_mm256_blendv_ps(a.0, b.0, self.0))
    }

    unsafe fn cmp_eq(self, rhs: Self) -> Self {
        F32x8(_mm256_cmp_ps(self.0, rhs.0, _CMP_EQ_OQ))
    }

    unsafe fn cmp_neq(self, rhs: Self) -> Self {
        F32x8(_mm256_cmp_ps(self.0, rhs.0, _CMP_NEQ_OQ))
    }

    unsafe fn cmp_lt(self, rhs: Self) -> Self {
        F32x8(_mm256_cmp_ps(self.0, rhs.0, _CMP_LT_OQ))
    }

    unsafe fn cmp_lte(self, rhs: Self) -> Self {
        F32x8(_mm256_cmp_ps(self.0, rhs.0, _CMP_LE_OQ))
    }

    unsafe fn cmp_gt(self, rhs: Self) -> Self {
        F32x8(_mm256_cmp_ps(self.0, rhs.0, _CMP_GT_OQ))
    }

    unsafe fn cmp_gte(self, rhs: Self) -> Self {
        F32x8(_mm256_cmp_ps(self.0, rhs.0, _CMP_GE_OQ))
    }

    unsafe fn max(self, rhs: Self) -> Self {
        F32x8(_mm256_max_ps(self.0, rhs.0))
    }

    unsafe fn min(self, rhs: Self) -> Self {
        F32x8(_mm256_min_ps(self.0, rhs.0))
    }

    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        F32x8(_mm256_loadu_ps(array.as_ptr()))
    }
}

impl SimdFloat for F32x8 {
    unsafe fn div(self, rhs: Self) -> Self {
        F32x8(_mm256_div_ps(self.0, rhs.0))
    }

    unsafe fn ceil(self) -> Self {
        F32x8(_mm256_ceil_ps(self.0))
    }

    unsafe fn floor(self) -> Self {
        F32x8(_mm256_floor_ps(self.0))
    }

    unsafe fn round(self) -> Self {
        F32x8(_mm256_round_ps(
            self.0,
            _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC,
        ))
    }

    unsafe fn fast_ceil(self) -> Self {
        self.ceil()
    }

    unsafe fn fast_floor(self) -> Self {
        self.floor()
    }

    unsafe fn fast_round(self) -> Self {
        self.round()
    }

    unsafe fn mul_add(self, a: Self, b: Self) -> Self {
        F32x8(_mm256_fmadd_ps(self.0, a.0, b.0))
    }

    unsafe fn mul_sub(self, a: Self, b: Self) -> Self {
        F32x8(_mm256_fmsub_ps(self.0, a.0, b.0))
    }

    unsafe fn neg_mul_add(self, a: Self, b: Self) -> Self {
        F32x8(_mm256_fnmadd_ps(self.0, a.0, b.0))
    }

    unsafe fn neg_mul_sub(self, a: Self, b: Self) -> Self {
        F32x8(_mm256_fnmsub_ps(self.0, a.0, b.0))
    }

    unsafe fn horizontal_add(self) -> Self::Scalar {
        let a = _mm256_hadd_ps(self.0, self.0);
        let b = _mm256_hadd_ps(a, a);
        let c = _mm256_hadd_ps(b, b);
        _mm_cvtss_f32(_mm256_castps256_ps128(c))
    }

    unsafe fn sqrt(self) -> Self {
        F32x8(_mm256_sqrt_ps(self.0))
    }

    unsafe fn rsqrt(self) -> Self {
        F32x8(_mm256_rsqrt_ps(self.0))
    }
}

impl SimdFloat32 for F32x8 {
    type SimdI32 = I32x8;

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
pub struct F64x4(pub __m256d);
impl_simd_base_overloads!(F64x4);
impl_simd_float_overloads!(F64x4);

impl SimdBase for F64x4 {
    const WIDTH: usize = 4;
    type Scalar = f64;
    type ArrayRepresentation = [f64; 4];
    type UnderlyingType = __m256d;

    unsafe fn zeroes() -> Self {
        F64x4(_mm256_setzero_pd())
    }

    unsafe fn set1(x: Self::Scalar) -> Self {
        F64x4(_mm256_set1_pd(x))
    }

    unsafe fn add(self, rhs: Self) -> Self {
        F64x4(_mm256_add_pd(self.0, rhs.0))
    }

    unsafe fn sub(self, rhs: Self) -> Self {
        F64x4(_mm256_sub_pd(self.0, rhs.0))
    }

    unsafe fn mul(self, rhs: Self) -> Self {
        F64x4(_mm256_mul_pd(self.0, rhs.0))
    }

    unsafe fn bit_and(self, rhs: Self) -> Self {
        F64x4(_mm256_and_pd(self.0, rhs.0))
    }

    unsafe fn bit_or(self, rhs: Self) -> Self {
        F64x4(_mm256_or_pd(self.0, rhs.0))
    }

    unsafe fn bit_xor(self, rhs: Self) -> Self {
        F64x4(_mm256_xor_pd(self.0, rhs.0))
    }

    unsafe fn bit_not(self) -> Self {
        self.bit_xor(I64x4::set1(-1).cast_f64())
    }

    unsafe fn abs(self) -> Self {
        let b = _mm256_set1_pd(-0.0f64);
        F64x4(_mm256_andnot_pd(b, self.0))
    }

    unsafe fn and_not(self, rhs: Self) -> Self {
        F64x4(_mm256_andnot_pd(self.0, rhs.0))
    }

    unsafe fn blendv(self, a: Self, b: Self) -> Self {
        F64x4(_mm256_blendv_pd(a.0, b.0, self.0))
    }

    unsafe fn cmp_eq(self, rhs: Self) -> Self {
        F64x4(_mm256_cmp_pd(self.0, rhs.0, _CMP_EQ_OQ))
    }

    unsafe fn cmp_neq(self, rhs: Self) -> Self {
        F64x4(_mm256_cmp_pd(self.0, rhs.0, _CMP_NEQ_OQ))
    }

    unsafe fn cmp_lt(self, rhs: Self) -> Self {
        F64x4(_mm256_cmp_pd(self.0, rhs.0, _CMP_LT_OQ))
    }

    unsafe fn cmp_lte(self, rhs: Self) -> Self {
        F64x4(_mm256_cmp_pd(self.0, rhs.0, _CMP_LE_OQ))
    }

    unsafe fn cmp_gt(self, rhs: Self) -> Self {
        F64x4(_mm256_cmp_pd(self.0, rhs.0, _CMP_GT_OQ))
    }

    unsafe fn cmp_gte(self, rhs: Self) -> Self {
        F64x4(_mm256_cmp_pd(self.0, rhs.0, _CMP_GE_OQ))
    }

    unsafe fn max(self, rhs: Self) -> Self {
        F64x4(_mm256_max_pd(self.0, rhs.0))
    }

    unsafe fn min(self, rhs: Self) -> Self {
        F64x4(_mm256_min_pd(self.0, rhs.0))
    }

    unsafe fn load_from_array(array: Self::ArrayRepresentation) -> Self {
        F64x4(_mm256_loadu_pd(array.as_ptr() as *const f64))
    }
}

impl SimdFloat for F64x4 {
    unsafe fn div(self, rhs: Self) -> Self {
        F64x4(_mm256_div_pd(self.0, rhs.0))
    }

    unsafe fn ceil(self) -> Self {
        F64x4(_mm256_ceil_pd(self.0))
    }

    unsafe fn floor(self) -> Self {
        F64x4(_mm256_floor_pd(self.0))
    }

    unsafe fn round(self) -> Self {
        F64x4(_mm256_round_pd(self.0, _MM_FROUND_TO_NEAREST_INT))
    }

    unsafe fn fast_ceil(self) -> Self {
        self.ceil()
    }

    unsafe fn fast_floor(self) -> Self {
        self.floor()
    }

    unsafe fn fast_round(self) -> Self {
        self.round()
    }

    unsafe fn mul_add(self, a: Self, b: Self) -> Self {
        F64x4(_mm256_fmadd_pd(self.0, a.0, b.0))
    }

    unsafe fn mul_sub(self, a: Self, b: Self) -> Self {
        F64x4(_mm256_fmsub_pd(self.0, a.0, b.0))
    }

    unsafe fn neg_mul_add(self, a: Self, b: Self) -> Self {
        F64x4(_mm256_fnmadd_pd(self.0, a.0, b.0))
    }

    unsafe fn neg_mul_sub(self, a: Self, b: Self) -> Self {
        F64x4(_mm256_fnmsub_pd(self.0, a.0, b.0))
    }

    unsafe fn horizontal_add(self) -> Self::Scalar {
        let a = _mm256_hadd_pd(self.0, self.0);
        let b = _mm256_hadd_pd(a, a);
        _mm_cvtsd_f64(_mm256_castpd256_pd128(b))
    }

    unsafe fn sqrt(self) -> Self {
        F64x4(_mm256_sqrt_pd(self.0))
    }

    unsafe fn rsqrt(self) -> Self {
        Self::set1(1.0) / self.sqrt()
    }
}

impl SimdFloat64 for F64x4 {
    type SimdI64 = I64x4;

    unsafe fn bitcast_i64(self) -> Self::SimdI64 {
        todo!()
    }

    unsafe fn cast_i64(self) -> Self::SimdI64 {
        todo!()
    }
}
