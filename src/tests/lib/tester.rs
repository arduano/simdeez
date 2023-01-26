use core::fmt::Debug;

use crate::SimdBase;

use super::{
    fn_tuple::{Func, Tuple},
    EqPrecision, ScalarNumber, SimdTupleIterable,
};

/// For each input, get the result, then, run the checker function on the result
/// and panic if the checker errors
pub fn check_function<Args: Tuple + Debug + Clone, R>(
    inputs: impl Iterator<Item = Args>,
    f: impl Func<Args, Output = R>,
    check: impl Fn(R, Args) -> Result<(), String>,
) {
    for args in inputs {
        let result = f.call(args.clone());
        if let Err(str) = check(result, args.clone()) {
            panic!("\nFailed for {:?}: {}", args, str);
        }
    }
}

/// Run `check_function` except with a scalar checker. This is to test element-wise functions.
pub fn check_elementwise_function<
    N: ScalarNumber,
    Args: Tuple + Debug + Clone + SimdTupleIterable<N>,
    R: SimdBase,
>(
    inputs: impl Iterator<Item = Args>,
    f: impl Func<Args, Output = R>,
    check: impl Fn(R::Scalar, Args::AsScalar) -> Result<(), String>,
) {
    check_function(inputs, f, |result, args| {
        let scalar_iter = args.iter_scalars();
        for (i, (scalar, expected)) in result.iter().zip(scalar_iter).enumerate() {
            if let Err(str) = check(scalar, expected) {
                return Err(format!("Failed for element {}: {}", i, str));
            }
        }
        Ok(())
    });
}

/// Compares a SIMD function against a corresponding scalar function
pub fn elementwise_eq_tester<
    N: ScalarNumber,
    Args: Tuple + Debug + Clone + SimdTupleIterable<N>,
    ScalarArg: SimdBase<Scalar = N>,
    R: SimdBase<Scalar = N>,
>(
    inputs: impl Iterator<Item = Args>,
    precision: EqPrecision,
    simd_fn: impl Func<Args, Output = R>,
    scalar_fn: impl Func<Args::AsTuple<ScalarArg>, Output = ScalarArg>,
) {
    check_elementwise_function(inputs, simd_fn, |result, args| {
        let scalar_result = scalar_fn.call(Args::wrap_scalars(args))[0];
        let equal = scalar_result.almost_eq(result, precision);
        match equal {
            true => Ok(()),
            false => Err(format!("Expected {}, got {}", scalar_result, result)),
        }
    });
}

/// Compares a SIMD function against a corresponding scalar function
pub fn bitshift_eq_tester<
    N: ScalarNumber + Eq,
    SimdArg: SimdBase<Scalar = N>,
    ScalarArg: SimdBase<Scalar = N>,
>(
    inputs: impl Iterator<Item = (SimdArg, i32)>,
    simd_fn: impl Func<(SimdArg, i32), Output = SimdArg>,
    scalar_fn: impl Func<(ScalarArg, i32), Output = ScalarArg>,
) {
    check_function(inputs, simd_fn, |result, args| unsafe {
        for i in 0..SimdArg::WIDTH {
            let scalar_result = scalar_fn.call((ScalarArg::set1(args.0[i]), args.1))[0];
            let equal = scalar_result == result[i];
            if !equal {
                return Err(format!(
                    "Failed for element {}: Expected {}, got {}",
                    i, scalar_result, result[i]
                ));
            }
        }
        Ok(())
    });
}

#[macro_export]
macro_rules! elementwise_eq_tester {
    (< $simd_kind:ident :: $simd_ty:ident as $base_kind:ident >  :: $fn_name:ident, $inputs:expr, $precision:expr) => {{
        let f = <<$simd_kind as Simd>::$simd_ty as $base_kind>::$fn_name;
        let sf = <<Scalar as Simd>::$simd_ty as $base_kind>::$fn_name;
        elementwise_eq_tester($inputs, $precision, f, sf);
    }};
}

#[macro_export]
macro_rules! bitshift_eq_tester {
    ($simd_kind:ident :: $simd_ty:ident :: $fn_name:ident, $inputs:expr) => {{
        let f = <<$simd_kind as Simd>::$simd_ty as SimdInt>::$fn_name;
        let sf = <<Scalar as Simd>::$simd_ty as SimdInt>::$fn_name;
        bitshift_eq_tester($inputs, f, sf);
    }};
}

#[macro_export]
macro_rules! const_bitshift_eq_tester {
    ($simd_kind:ident :: $simd_ty:ident :: $fn_name:ident, $inputs:expr) => {{
        let f = |val, s| {
            macro_rules! expand {
                ($imm8:literal) => {
                    <<$simd_kind as Simd>::$simd_ty as SimdInt>::$fn_name::<$imm8>(val)
                };
            }
            test_constify_imm8!(s, expand)
        };
        let sf = |val, s| {
            macro_rules! expand {
                ($imm8:literal) => {
                    <<Scalar as Simd>::$simd_ty as SimdInt>::$fn_name::<$imm8>(val)
                };
            }
            test_constify_imm8!(s, expand)
        };
        bitshift_eq_tester($inputs, f, sf);
    }};
}

#[macro_export]
macro_rules! with_feature_flag {
    (Avx2, $($r:tt)+) => {
        #[cfg(target_feature = "avx2")]
        $($r)+
    };
    (Sse2, $($r:tt)+) => {
        #[cfg(target_feature = "sse2")]
        $($r)+
    };
    (Sse41, $($r:tt)+) => {
        #[cfg(target_feature = "sse4.1")]
        $($r)+
    };
    (Scalar, $($r:tt)+) => {
        $($r)+
    };
}

#[macro_export]
macro_rules! elementwise_eq_tester_impl {
    (@full $simd:ident, $simd_ty:ident, $simd_base:ident, $simd_fn:ident, $arg_cnt:ident, $precision:expr) => {
        with_feature_flag!($simd,
            paste::item! {
                #[test]
                fn [<$simd_fn _ $simd:lower _ $simd_ty>]() {
                    elementwise_eq_tester!(
                        <$simd:: [<V$simd_ty>] as $simd_base>::$simd_fn,
                        RandSimd::$simd_ty().$arg_cnt(),
                        $precision
                    );
                }
            }
        );
    };

    (@simdkind $simd_ty:ident, $simd_base:ident, $simd_fn:ident, $arg_cnt:ident, $precision:expr) => {
        elementwise_eq_tester_impl!(@full Scalar, $simd_ty, $simd_base, $simd_fn, $arg_cnt, $precision);
        elementwise_eq_tester_impl!(@full Avx2, $simd_ty, $simd_base, $simd_fn, $arg_cnt, $precision);
        elementwise_eq_tester_impl!(@full Sse2, $simd_ty, $simd_base, $simd_fn, $arg_cnt, $precision);
        elementwise_eq_tester_impl!(@full Sse41, $simd_ty, $simd_base, $simd_fn, $arg_cnt, $precision);
    };

    (SimdBase, $simd_fn:ident, $arg_cnt:ident, $precision:expr) => {
        elementwise_eq_tester_impl!(@simdkind i16, SimdBase, $simd_fn, $arg_cnt, $precision);
        elementwise_eq_tester_impl!(@simdkind i32, SimdBase, $simd_fn, $arg_cnt, $precision);
        elementwise_eq_tester_impl!(@simdkind i64, SimdBase, $simd_fn, $arg_cnt, $precision);
        elementwise_eq_tester_impl!(@simdkind f32, SimdBase, $simd_fn, $arg_cnt, $precision);
        elementwise_eq_tester_impl!(@simdkind f64, SimdBase, $simd_fn, $arg_cnt, $precision);
    };

    (SimdInt, $simd_fn:ident, $arg_cnt:ident, $precision:expr) => {
        elementwise_eq_tester_impl!(@simdkind i16, SimdInt, $simd_fn, $arg_cnt, $precision);
        elementwise_eq_tester_impl!(@simdkind i32, SimdInt, $simd_fn, $arg_cnt, $precision);
        elementwise_eq_tester_impl!(@simdkind i64, SimdInt, $simd_fn, $arg_cnt, $precision);
    };

    (SimdFloat, $simd_fn:ident, $arg_cnt:ident, $precision:expr) => {
        elementwise_eq_tester_impl!(@simdkind f32, SimdFloat, $simd_fn, $arg_cnt, $precision);
        elementwise_eq_tester_impl!(@simdkind f64, SimdFloat, $simd_fn, $arg_cnt, $precision);
    };

    (SimdFloat32, $simd_fn:ident, $arg_cnt:ident, $precision:expr) => {
        elementwise_eq_tester_impl!(@simdkind f32, SimdFloat32, $simd_fn, $arg_cnt, $precision);
    };
}

#[macro_export]
macro_rules! bitshift_eq_tester_impl {
    (@full dyn, $simd:ident, $simd_ty:ident, $simd_fn:ident) => {
        with_feature_flag!($simd,
            paste::item! {
                #[test]
                fn [<$simd_fn _ $simd:lower _ $simd_ty>]() {
                    bitshift_eq_tester!(
                        $simd:: [<V$simd_ty>]::$simd_fn,
                        RandSimd::$simd_ty().one_arg_and_bitshift_arg()
                    );
                }
            }
        );
    };

    (@full const, $simd:ident, $simd_ty:ident, $simd_fn:ident) => {
        with_feature_flag!($simd,
            paste::item! {
                #[test]
                fn [<$simd_fn _ $simd:lower _ $simd_ty>]() {
                    const_bitshift_eq_tester!(
                        $simd:: [<V$simd_ty>]::$simd_fn,
                        RandSimd::$simd_ty().one_arg_and_bitshift_arg()
                    );
                }
            }
        );
    };

    (@simdkind $is_const:ident, $simd_ty:ident, $simd_fn:ident) => {
        bitshift_eq_tester_impl!(@full $is_const, Scalar, $simd_ty, $simd_fn);
        bitshift_eq_tester_impl!(@full $is_const, Avx2, $simd_ty, $simd_fn);
        bitshift_eq_tester_impl!(@full $is_const, Sse2, $simd_ty, $simd_fn);
        bitshift_eq_tester_impl!(@full $is_const, Sse41, $simd_ty, $simd_fn);
    };

    ($is_const:ident $simd_fn:ident) => {
        bitshift_eq_tester_impl!(@simdkind $is_const, i16, $simd_fn);
        bitshift_eq_tester_impl!(@simdkind $is_const, i32, $simd_fn);
        bitshift_eq_tester_impl!(@simdkind $is_const, i64, $simd_fn);
    };
}
