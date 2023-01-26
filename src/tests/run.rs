use crate::{elementwise_eq_tester, test_constify_imm8};

use super::*;

use crate::{avx2::*, scalar::*, sse2::*, sse41::*, *};

elementwise_eq_tester_impl!(SimdBase, add, two_arg, EqPrecision::exact());
elementwise_eq_tester_impl!(SimdBase, sub, two_arg, EqPrecision::exact());
elementwise_eq_tester_impl!(SimdBase, mul, two_arg, EqPrecision::exact());

elementwise_eq_tester_impl!(SimdBase, bit_and, two_arg, EqPrecision::exact());
elementwise_eq_tester_impl!(SimdBase, bit_or, two_arg, EqPrecision::exact());
elementwise_eq_tester_impl!(SimdBase, bit_xor, two_arg, EqPrecision::exact());
elementwise_eq_tester_impl!(SimdBase, bit_not, one_arg, EqPrecision::exact());
elementwise_eq_tester_impl!(SimdBase, and_not, two_arg, EqPrecision::exact());

// Abs breaks on minimum integer values as they dont have a corrisponding maximum, causing undefined behavior.
elementwise_eq_tester_impl!(SimdBase, abs, one_arg_abs_filtered, EqPrecision::exact());

elementwise_eq_tester_impl!(SimdBase, cmp_eq, two_arg, EqPrecision::exact());
elementwise_eq_tester_impl!(SimdBase, cmp_lt, two_arg, EqPrecision::exact());
elementwise_eq_tester_impl!(SimdBase, cmp_lte, two_arg, EqPrecision::exact());
elementwise_eq_tester_impl!(SimdBase, cmp_gt, two_arg, EqPrecision::exact());
elementwise_eq_tester_impl!(SimdBase, cmp_gte, two_arg, EqPrecision::exact());

elementwise_eq_tester_impl!(SimdBase, blendv, iter_blendv_ags, EqPrecision::exact());

// We filter out NaN numbers for neq because under some hardware implementations (including Avx2)
// it appears that `NaN != [number]` is false. Technically that's invalid according to the floatin point
// spec, but it seems like a hardware thing that we cant avoid.
elementwise_eq_tester_impl!(
    SimdBase,
    cmp_neq,
    two_arg_nan_filtered,
    EqPrecision::exact()
);

elementwise_eq_tester_impl!(SimdBase, max, two_arg_nan_filtered, EqPrecision::exact());
elementwise_eq_tester_impl!(SimdBase, min, two_arg_nan_filtered, EqPrecision::exact());

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
elementwise_eq_tester_impl!(SimdFloat, rsqrt, one_arg, EqPrecision::almost(3)); // Has very low precision

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
