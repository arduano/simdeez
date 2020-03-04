use super::*;
use core::mem;

mod avx;
mod overloads;
pub use self::avx::*;
pub use self::overloads::*;

#[derive(Copy, Debug, Clone)]
pub struct I16x16(pub __m256i);
impl SimdBase<I16x16, i16> for I16x16 {}
impl SimdSmallInt<I16x16, i16> for I16x16 {}

#[derive(Copy, Debug, Clone)]
pub struct I32x8(pub __m256i);
impl SimdBase<I32x8, i32> for I32x8 {}
impl SimdSmallInt<I32x8, i32> for I32x8 {}

#[derive(Copy, Debug, Clone)]
pub struct I64x4(pub __m256i);
impl SimdBase<I64x4, i64> for I64x4 {}

#[derive(Copy, Debug, Clone)]
pub struct F32x8(pub __m256);
impl SimdBase<F32x8, f32> for F32x8 {}
impl SimdFloat<F32x8, f32> for F32x8 {}

#[derive(Copy, Debug, Clone)]
pub struct F64x4(pub __m256d);
impl SimdBase<F64x4, f64> for F64x4 {}
impl SimdFloat<F64x4, f64> for F64x4 {}
