use super::*;

mod overloads;
mod scalar;
pub use self::overloads::*;
pub use self::scalar::*;

// Newtypes for i16 vectors
// We have to do this to allow for overloading of
// __m128i etc
#[derive(Copy, Debug, Clone)]
pub struct I16x1(pub i16);
impl SimdBase<I16x1, i16> for I16x1 {}
impl SimdSmallInt<I16x1, i16> for I16x1 {}

// Newtypes for i32 vectors
// We have to do this to allow for overloading of
// __m128i etc
#[derive(Copy, Debug, Clone)]
pub struct I32x1(pub i32);
impl SimdBase<I32x1, i32> for I32x1 {}
impl SimdSmallInt<I32x1, i32> for I32x1 {}

// Newtypes for i64 vectors
// We have to do this to allow for overloading of
// __m128i etc
#[derive(Copy, Debug, Clone)]
pub struct I64x1(pub i64);
impl SimdBase<I64x1, i64> for I64x1 {}

// Newtypes for f32 vectors
// We have to do this to allow for overloading of
// __m128 etc
#[derive(Copy, Debug, Clone)]
pub struct F32x1(pub f32);
impl SimdBase<F32x1, f32> for F32x1 {}
impl SimdFloat<F32x1, f32> for F32x1 {}

// Newtypes for f64 vectors
#[derive(Copy, Debug, Clone)]
pub struct F64x1(pub f64);
impl SimdBase<F64x1, f64> for F64x1 {}
impl SimdFloat<F64x1, f64> for F64x1 {}
