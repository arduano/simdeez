use super::*;
use core::arch::wasm32::*;
use core::mem;
mod overloads;
mod wasm32;
pub use self::overloads::*;
pub use self::wasm32::*;

#[derive(Copy, Clone)]
pub struct I16x8_W32(pub v128);
impl SimdBase<I16x8_W32, i16> for I16x8_W32 {}
impl SimdSmallInt<I16x8_W32, i16> for I16x8_W32 {}

#[derive(Copy, Clone)]
pub struct I32x4_W32(pub v128);
impl SimdBase<I32x4_W32, i32> for I32x4_W32 {}
impl SimdSmallInt<I32x4_W32, i32> for I32x4_W32 {}

#[derive(Copy, Clone)]
pub struct I64x2_W32(pub v128);
impl SimdBase<I64x2_W32, i64> for I64x2_W32 {}

#[derive(Copy, Clone)]
pub struct F32x4_W32(pub v128);
impl SimdBase<F32x4_W32, f32> for F32x4_W32 {}
impl SimdFloat<F32x4_W32, f32> for F32x4_W32 {}

#[derive(Copy, Clone)]
pub struct F64x2_W32(pub v128);
impl SimdBase<F64x2_W32, f64> for F64x2_W32 {}
impl SimdFloat<F64x2_W32, f64> for F64x2_W32 {}
