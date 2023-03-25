#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;

use core::ops::*;

use crate::ops::*;
use crate::*;

mod simd;
pub use self::simd::*;

define_simd_type!(Neon, i8, 16, int8x16_t, Neon);
impl_simd_int_overloads!(I8x16Neon);
impl_i8_simd_type!(Neon, I8x16Neon, I16x8Neon);

define_simd_type!(Neon, i16, 8, int16x8_t, Neon);
impl_simd_int_overloads!(I16x8Neon);
impl_i16_simd_type!(Neon, I16x8Neon, I32x4Neon);

define_simd_type!(Neon, i32, 4, int32x4_t, Neon);
impl_simd_int_overloads!(I32x4Neon);
impl_i32_simd_type!(Neon, I32x4Neon, F32x4Neon, I64x2Neon);

define_simd_type!(Neon, i64, 2, int64x2_t, Neon);
impl_simd_int_overloads!(I64x2Neon);
impl_i64_simd_type!(Neon, I64x2Neon, F64x2Neon);

define_simd_type!(Neon, f32, 4, float32x4_t, Neon);
impl_simd_float_overloads!(F32x4Neon);
impl_f32_simd_type!(Neon, F32x4Neon, I32x4Neon);

define_simd_type!(Neon, f64, 2, float64x2_t, Neon);
impl_simd_float_overloads!(F64x2Neon);
impl_f64_simd_type!(Neon, F64x2Neon, I64x2Neon);
