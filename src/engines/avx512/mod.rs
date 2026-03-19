#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use core::ops::*;

use crate::ops::*;
use crate::*;

mod simd;
pub use self::simd::*;

define_simd_type!(Avx512, i8, 64, __m512i);
impl_simd_int_overloads!(I8x64);
impl_i8_simd_type!(Avx512, I8x64, I16x32);

define_simd_type!(Avx512, i16, 32, __m512i);
impl_simd_int_overloads!(I16x32);
impl_i16_simd_type!(Avx512, I16x32, I32x16);

define_simd_type!(Avx512, i32, 16, __m512i);
impl_simd_int_overloads!(I32x16);
impl_i32_simd_type!(Avx512, I32x16, F32x16, I64x8);

define_simd_type!(Avx512, i64, 8, __m512i);
impl_simd_int_overloads!(I64x8);
impl_i64_simd_type!(Avx512, I64x8, F64x8);

define_simd_type!(Avx512, f32, 16, __m512);
impl_simd_float_overloads!(F32x16);
impl_f32_simd_type!(Avx512, F32x16, I32x16);

define_simd_type!(Avx512, f64, 8, __m512d);
impl_simd_float_overloads!(F64x8);
impl_f64_simd_type!(Avx512, F64x8, I64x8);
