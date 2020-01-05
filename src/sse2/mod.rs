use super::*;
use core::mem;

mod overloads;
pub use self::overloads::*;

#[derive(Copy, Debug, Clone)]
pub struct I16x8(pub __m128i);
impl SimdBase<I16x8, i16> for I16x8 {}
impl SimdSmallInt<I16x8, i16> for I16x8 {}

#[derive(Copy, Debug, Clone)]
pub struct I32x4(pub __m128i);
impl SimdBase<I32x4, i32> for I32x4 {}
impl SimdSmallInt<I32x4, i32> for I32x4 {}

#[derive(Copy, Debug, Clone)]
pub struct I64x2(pub __m128i);
impl SimdBase<I64x2, i64> for I64x2 {}

#[derive(Copy, Debug, Clone)]
pub struct F32x4(pub __m128);
impl SimdBase<F32x4, f32> for F32x4 {}
impl SimdFloat<F32x4, f32> for F32x4 {}

#[derive(Copy, Debug, Clone)]
pub struct F64x2(pub __m128d);
impl SimdBase<F64x2, f64> for F64x2 {}
impl SimdFloat<F64x2, f64> for F64x2 {}
