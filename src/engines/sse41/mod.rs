#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use core::ops::*;

use crate::{
    InternalSimdBaseIo, SimdBaseOps, SimdConsts, SimdFloat, SimdFloat32, SimdFloat64, SimdInt,
    SimdInt16, SimdInt32, SimdInt64, SimdInt8,
};

use crate::ops::*;

mod simd;
pub use self::simd::*;

define_simd_type!(i8, 16, __m128i, _41);
impl_simd_int_overloads!(I8x16_41);
impl_i8_simd_type!(Sse41, I8x16_41, I16x8_41);

define_simd_type!(i16, 8, __m128i, _41);
impl_simd_int_overloads!(I16x8_41);
impl_i16_simd_type!(Sse41, I16x8_41, I32x4_41);

define_simd_type!(i32, 4, __m128i, _41);
impl_simd_int_overloads!(I32x4_41);
impl_i32_simd_type!(Sse41, I32x4_41, F32x4_41, I64x2_41);

define_simd_type!(i64, 2, __m128i, _41);
impl_simd_int_overloads!(I64x2_41);
impl_i64_simd_type!(Sse41, I64x2_41, F64x2_41);

define_simd_type!(f32, 4, __m128, _41);
impl_simd_float_overloads!(F32x4_41);
impl_f32_simd_type!(Sse41, F32x4_41, I32x4_41);

define_simd_type!(f64, 2, __m128d, _41);
impl_simd_float_overloads!(F64x2_41);
impl_f64_simd_type!(Sse41, F64x2_41, I64x2_41);
