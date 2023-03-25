use core::ops::*;

use crate::ops::*;
use crate::*;

mod simd;
pub use self::simd::*;

define_simd_type!(Scalar, i8, 1, i8);
impl_simd_int_overloads!(I8x1);
impl_i8_simd_type!(Scalar, I8x1, I16x1);

define_simd_type!(Scalar, i16, 1, i16);
impl_simd_int_overloads!(I16x1);
impl_i16_simd_type!(Scalar, I16x1, I32x1);

define_simd_type!(Scalar, i32, 1, i32);
impl_simd_int_overloads!(I32x1);
impl_i32_simd_type!(Scalar, I32x1, F32x1, I64x1);

define_simd_type!(Scalar, i64, 1, i64);
impl_simd_int_overloads!(I64x1);
impl_i64_simd_type!(Scalar, I64x1, F64x1);

define_simd_type!(Scalar, f32, 1, f32);
impl_simd_float_overloads!(F32x1);
impl_f32_simd_type!(Scalar, F32x1, I32x1);

define_simd_type!(Scalar, f64, 1, f64);
impl_simd_float_overloads!(F64x1);
impl_f64_simd_type!(Scalar, F64x1, I64x1);

impl From<i8> for I8x1 {
    fn from(val: i8) -> Self {
        I8x1(val)
    }
}

impl From<i16> for I16x1 {
    fn from(val: i16) -> Self {
        I16x1(val)
    }
}

impl From<i32> for I32x1 {
    fn from(val: i32) -> Self {
        I32x1(val)
    }
}

impl From<i64> for I64x1 {
    fn from(val: i64) -> Self {
        I64x1(val)
    }
}

impl From<f32> for F32x1 {
    fn from(val: f32) -> Self {
        F32x1(val)
    }
}

impl From<f64> for F64x1 {
    fn from(val: f64) -> Self {
        F64x1(val)
    }
}

impl From<I8x1> for i8 {
    fn from(v: I8x1) -> i8 {
        v.0
    }
}

impl From<I16x1> for i16 {
    fn from(v: I16x1) -> i16 {
        v.0
    }
}

impl From<I32x1> for i32 {
    fn from(v: I32x1) -> i32 {
        v.0
    }
}

impl From<I64x1> for i64 {
    fn from(v: I64x1) -> i64 {
        v.0
    }
}

impl From<F32x1> for f32 {
    fn from(v: F32x1) -> f32 {
        v.0
    }
}

impl From<F64x1> for f64 {
    fn from(v: F64x1) -> f64 {
        v.0
    }
}
