#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;

use core::ops::*;

use crate::{
    InternalSimdBaseIo, SimdBaseOps, SimdConsts, SimdFloat, SimdFloat32, SimdFloat64, SimdInt,
    SimdInt16, SimdInt32, SimdInt64, SimdInt8,
};

use crate::ops::*;

mod simd;
pub use self::simd::*;

define_simd_type!(i8, 16, int8x16_t, _N);
impl_simd_int_overloads!(I8x16_N);
impl_i8_simd_type!(Neon, I8x16_N, I16x8_N);

define_simd_type!(i16, 8, int16x8_t, _N);
impl_simd_int_overloads!(I16x8_N);
impl_i16_simd_type!(Neon, I16x8_N, I32x4_N);

define_simd_type!(i32, 4, int32x4_t, _N);
impl_simd_int_overloads!(I32x4_N);
impl_i32_simd_type!(Neon, I32x4_N, F32x4_N, I64x2_N);

define_simd_type!(i64, 2, int64x2_t, _N);
impl_simd_int_overloads!(I64x2_N);
impl_i64_simd_type!(Neon, I64x2_N, F64x2_N);

define_simd_type!(f32, 4, float32x4_t, _N);
impl_simd_float_overloads!(F32x4_N);
impl_f32_simd_type!(Neon, F32x4_N, I32x4_N);

define_simd_type!(f64, 2, float64x2_t, _N);
impl_simd_float_overloads!(F64x2_N);
impl_f64_simd_type!(Neon, F64x2_N, I64x2_N);
