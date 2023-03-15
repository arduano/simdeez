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

define_simd_type!(i8, 32, __m256i);
impl_simd_int_overloads!(I8x32);
impl_i8_simd_type!(Avx2, I8x32, I16x16);

define_simd_type!(i16, 16, __m256i);
impl_simd_int_overloads!(I16x16);
impl_i16_simd_type!(Avx2, I16x16, I32x8);

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
