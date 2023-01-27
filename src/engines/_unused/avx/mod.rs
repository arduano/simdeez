use super::*;
use core::mem;

mod avx;
mod overloads;
pub use self::avx::*;
pub use self::overloads::*;

#[derive(Copy, Clone)]
pub struct I16x16(pub __m256i);
impl SimdBaseOps<I16x16, i16> for I16x16 {}
impl SimdSmallInt<I16x16, i16> for I16x16 {}

#[derive(Copy, Clone)]
pub struct I32x8(pub __m256i);
impl SimdBaseOps<I32x8, i32> for I32x8 {}
impl SimdSmallInt<I32x8, i32> for I32x8 {}

#[derive(Copy, Clone)]
pub struct I64x4(pub __m256i);
impl SimdBaseOps<I64x4, i64> for I64x4 {}

#[derive(Copy, Clone)]
pub struct F32x8(pub __m256);
impl SimdBaseOps<F32x8, f32> for F32x8 {}
impl SimdFloat<F32x8, f32> for F32x8 {}

#[derive(Copy, Clone)]
pub struct F64x4(pub __m256d);
impl SimdBaseOps<F64x4, f64> for F64x4 {}
impl SimdFloat<F64x4, f64> for F64x4 {}
