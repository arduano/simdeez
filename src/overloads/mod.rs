use super::*;
use core::mem;

// Newtypes for i16 vectors
// We have to do this to allow for overloading of
// __m128i etc
#[derive(Copy, Debug, Clone)]
pub struct I16x1(pub i16);
impl SimdBase<I16x1, i16> for I16x1 {}
impl SimdSmallInt<I16x1, i16> for I16x1 {}

#[derive(Copy, Debug, Clone)]
pub struct I16x16(pub __m256i);
impl SimdBase<I16x16, i16> for I16x16 {}
impl SimdSmallInt<I16x16, i16> for I16x16 {}

// Newtypes for i32 vectors
// We have to do this to allow for overloading of
// __m128i etc
#[derive(Copy, Debug, Clone)]
pub struct I32x1(pub i32);
impl SimdBase<I32x1, i32> for I32x1 {}
impl SimdSmallInt<I32x1, i32> for I32x1 {}

#[derive(Copy, Debug, Clone)]
pub struct I32x8(pub __m256i);
impl SimdBase<I32x8, i32> for I32x8 {}
impl SimdSmallInt<I32x8, i32> for I32x8 {}

// Newtypes for i64 vectors
// We have to do this to allow for overloading of
// __m128i etc
#[derive(Copy, Debug, Clone)]
pub struct I64x1(pub i64);
impl SimdBase<I64x1, i64> for I64x1 {}
#[derive(Copy, Debug, Clone)]
pub struct I64x4(pub __m256i);
impl SimdBase<I64x4, i64> for I64x4 {}

// Newtypes for f32 vectors
// We have to do this to allow for overloading of
// __m128 etc
#[derive(Copy, Debug, Clone)]
pub struct F32x1(pub f32);
impl SimdBase<F32x1, f32> for F32x1 {}
impl SimdFloat<F32x1, f32> for F32x1 {}
#[derive(Copy, Debug, Clone)]
pub struct F32x8(pub __m256);
impl SimdBase<F32x8, f32> for F32x8 {}
impl SimdFloat<F32x8, f32> for F32x8 {}

// Newtypes for f64 vectors
#[derive(Copy, Debug, Clone)]
pub struct F64x1(pub f64);
impl SimdBase<F64x1, f64> for F64x1 {}
impl SimdFloat<F64x1, f64> for F64x1 {}
#[derive(Copy, Debug, Clone)]
pub struct F64x4(pub __m256d);
impl SimdBase<F64x4, f64> for F64x4 {}
impl SimdFloat<F64x4, f64> for F64x4 {}

mod index;
pub use self::index::*;
mod index_mut;
pub use self::index_mut::*;
mod add;
pub use self::add::*;
mod add_assign;
pub use self::add_assign::*;
mod sub;
pub use self::sub::*;
mod sub_assign;
pub use self::sub_assign::*;
mod mul;
pub use self::mul::*;
mod mul_assign;
pub use self::mul_assign::*;
mod div;
pub use self::div::*;
mod div_assign;
pub use self::div_assign::*;
mod and;
pub use self::and::*;
mod and_assign;
pub use self::and_assign::*;
mod or;
pub use self::or::*;
mod or_assign;
pub use self::or_assign::*;
mod xor;
pub use self::xor::*;
mod xor_assign;
pub use self::xor_assign::*;
mod not;
pub use self::not::*;
mod shl;
pub use self::shl::*;
mod shl_assign;
pub use self::shl_assign::*;
mod shr;
pub use self::shr::*;
mod shr_assign;
pub use self::shr_assign::*;
