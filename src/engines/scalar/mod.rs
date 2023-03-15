use core::ops::*;

use crate::{
    InternalSimdBaseIo, SimdBaseOps, SimdConsts, SimdFloat, SimdFloat32, SimdFloat64, SimdInt,
    SimdInt16, SimdInt32, SimdInt64, SimdInt8,
};

use crate::ops::*;

mod simd;
pub use self::simd::*;

define_simd_type!(i8, 1, i8);
impl_simd_int_overloads!(I8x1);
impl_i8_simd_type!(Scalar, I8x1, I16x1);

define_simd_type!(i16, 1, i16);
impl_simd_int_overloads!(I16x1);
impl_i16_simd_type!(Scalar, I16x1, I32x1);

define_simd_type!(i32, 1, i32);
impl_simd_int_overloads!(I32x1);
impl_i32_simd_type!(Scalar, I32x1, F32x1, I64x1);

define_simd_type!(i64, 1, i64);
impl_simd_int_overloads!(I64x1);
impl_i64_simd_type!(Scalar, I64x1, F64x1);

define_simd_type!(f32, 1, f32);
impl_simd_float_overloads!(F32x1);
impl_f32_simd_type!(Scalar, F32x1, I32x1);

define_simd_type!(f64, 1, f64);
impl_simd_float_overloads!(F64x1);
impl_f64_simd_type!(Scalar, F64x1, I64x1);
