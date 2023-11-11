#![allow(unused_imports)]

use crate::elementwise_eq_tester;

use super::*;

#[cfg(target_arch = "aarch64")]
use crate::engines::neon::Neon;
use crate::engines::scalar::*;
#[cfg(target_arch = "wasm32")]
use crate::engines::wasm32::Wasm;
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
use crate::engines::{avx2::*, sse2::*, sse41::*};

use crate::*;

elementwise_eq_tester_impl!(SimdBaseOps, add, two_arg, EqPrecision::exact());
elementwise_eq_tester_impl!(SimdBaseOps, sub, two_arg, EqPrecision::exact());
elementwise_eq_tester_impl!(SimdBaseOps, mul, two_arg, EqPrecision::exact());

elementwise_eq_tester_impl!(SimdBaseOps, bit_and, two_arg, EqPrecision::exact());
elementwise_eq_tester_impl!(SimdBaseOps, bit_or, two_arg, EqPrecision::exact());
elementwise_eq_tester_impl!(SimdBaseOps, bit_xor, two_arg, EqPrecision::exact());
elementwise_eq_tester_impl!(SimdBaseOps, bit_not, one_arg, EqPrecision::exact());
elementwise_eq_tester_impl!(SimdBaseOps, and_not, two_arg, EqPrecision::exact());

// Abs breaks on minimum integer values as they dont have a corrisponding maximum, causing undefined behavior.
elementwise_eq_tester_impl!(SimdBaseOps, abs, one_arg_abs_filtered, EqPrecision::exact());

elementwise_eq_tester_impl!(SimdBaseOps, cmp_eq, two_arg, EqPrecision::exact());
elementwise_eq_tester_impl!(SimdBaseOps, cmp_lt, two_arg, EqPrecision::exact());
elementwise_eq_tester_impl!(SimdBaseOps, cmp_lte, two_arg, EqPrecision::exact());
elementwise_eq_tester_impl!(SimdBaseOps, cmp_gt, two_arg, EqPrecision::exact());
elementwise_eq_tester_impl!(SimdBaseOps, cmp_gte, two_arg, EqPrecision::exact());

elementwise_eq_tester_impl!(SimdBaseOps, blendv, iter_blendv_ags, EqPrecision::exact());

// We filter out NaN numbers for neq because under some hardware implementations (including Avx2)
// it appears that `NaN != [number]` is false. Technically that's invalid according to the floatin point
// spec, but it seems like a hardware thing that we cant avoid.
elementwise_eq_tester_impl!(
    SimdBaseOps,
    cmp_neq,
    two_arg_nan_filtered,
    EqPrecision::exact()
);

elementwise_eq_tester_impl!(SimdBaseOps, max, two_arg_nan_filtered, EqPrecision::exact());
elementwise_eq_tester_impl!(SimdBaseOps, min, two_arg_nan_filtered, EqPrecision::exact());

// We use "almost" precision for these functions because at higher numbers there's always small errors.
// However if there's an error in smaller numbers, it would be caught pretty easily by this precision.
elementwise_eq_tester_impl!(SimdFloat, div, two_arg, EqPrecision::almost(6));
elementwise_eq_tester_impl!(SimdFloat, ceil, one_arg, EqPrecision::almost(6));
elementwise_eq_tester_impl!(SimdFloat, floor, one_arg, EqPrecision::almost(6));
elementwise_eq_tester_impl!(
    SimdFloat,
    round,
    one_arg_rounding_safe,
    EqPrecision::almost(6)
);

elementwise_eq_tester_impl!(SimdFloat, mul_add, three_arg, EqPrecision::almost(5));
elementwise_eq_tester_impl!(SimdFloat, mul_sub, three_arg, EqPrecision::almost(5));
elementwise_eq_tester_impl!(SimdFloat, neg_mul_add, three_arg, EqPrecision::almost(5));
elementwise_eq_tester_impl!(SimdFloat, neg_mul_sub, three_arg, EqPrecision::almost(5));

elementwise_eq_tester_impl!(SimdFloat, sqrt, one_arg, EqPrecision::almost(7));
elementwise_eq_tester_impl!(SimdFloat, rsqrt, one_arg, EqPrecision::almost(2)); // Has very low precision

bitshift_eq_tester_impl!(dyn shl);
bitshift_eq_tester_impl!(dyn shr);
bitshift_eq_tester_impl!(const shl_const);
bitshift_eq_tester_impl!(const shr_const);

elementwise_eq_tester_impl!(
    SimdFloat32,
    cast_i32,
    float_to_int_cast_values,
    EqPrecision::exact()
);
elementwise_eq_tester_impl!(SimdInt32, cast_f32, one_arg, EqPrecision::exact());
elementwise_eq_tester_impl!(
    SimdFloat64,
    cast_i64,
    one_arg_rounding_safe,
    EqPrecision::exact()
);
elementwise_eq_tester_impl!(SimdInt64, cast_f64, one_arg, EqPrecision::exact());

horizontal_add_tester_impl!(signed);
horizontal_add_tester_impl!(unsigned);
