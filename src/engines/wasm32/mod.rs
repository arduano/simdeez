#[cfg(target_arch = "wasm32")]
use core::arch::wasm32::*;

use core::ops::*;

use crate::ops::*;
use crate::*;

mod simd;
pub use self::simd::*;

define_simd_type!(Wasm, i8, 16, v128, Wasm);
impl_simd_int_overloads!(I8x16Wasm);
impl_i8_simd_type!(Wasm, I8x16Wasm, I16x8Wasm);

define_simd_type!(Wasm, i16, 8, v128, Wasm);
impl_simd_int_overloads!(I16x8Wasm);
impl_i16_simd_type!(Wasm, I16x8Wasm, I32x4Wasm);

define_simd_type!(Wasm, i32, 4, v128, Wasm);
impl_simd_int_overloads!(I32x4Wasm);
impl_i32_simd_type!(Wasm, I32x4Wasm, F32x4Wasm, I64x2Wasm);

define_simd_type!(Wasm, i64, 2, v128, Wasm);
impl_simd_int_overloads!(I64x2Wasm);
impl_i64_simd_type!(Wasm, I64x2Wasm, F64x2Wasm);

define_simd_type!(Wasm, f32, 4, v128, Wasm);
impl_simd_float_overloads!(F32x4Wasm);
impl_f32_simd_type!(Wasm, F32x4Wasm, I32x4Wasm);

define_simd_type!(Wasm, f64, 2, v128, Wasm);
impl_simd_float_overloads!(F64x2Wasm);
impl_f64_simd_type!(Wasm, F64x2Wasm, I64x2Wasm);
