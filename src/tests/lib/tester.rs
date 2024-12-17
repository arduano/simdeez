use core::{fmt::Debug, ops::Add};

use crate::{SimdBase, SimdBaseIo};

use super::{
    fn_tuple::{Func, Tuple},
    EqPrecision, IntScalarNumber, ScalarNumber, SimdTupleIterable,
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
                return Err(format!("Failed for element {i}: {str}"));
            }
        }
        Ok(())
    });
}

pub fn elementwise_eq_tester<
    N: ScalarNumber,
    RN: ScalarNumber,
    Args: Tuple + Debug + Clone + SimdTupleIterable<N>,
    ScalarArg: SimdBase<Scalar = N>,
    SimdRet: SimdBase<Scalar = RN>,
    ScalarRet: SimdBase<Scalar = RN>,
>(
    inputs: impl Iterator<Item = Args>,
    precision: EqPrecision,
    simd_fn: impl Func<Args, Output = SimdRet>,
    scalar_fn: impl Func<Args::AsTuple<ScalarArg>, Output = ScalarRet>,
) {
    check_elementwise_function(inputs, simd_fn, |result, args| {
        let scalar_result = scalar_fn.call(Args::wrap_scalars(args))[0];
        let equal = scalar_result.almost_eq(result, precision);
        match equal {
            true => Ok(()),
            false => Err(format!("Expected {scalar_result}, got {result}")),
        }
    });
}

pub fn bitshift_eq_tester<
    N: ScalarNumber + Eq,
    SimdArg: SimdBase<Scalar = N>,
    ScalarArg: SimdBase<Scalar = N>,
>(
    inputs: impl Iterator<Item = (SimdArg, i32)>,
    simd_fn: impl Func<(SimdArg, i32), Output = SimdArg>,
    scalar_fn: impl Func<(ScalarArg, i32), Output = ScalarArg>,
) {
    check_function(inputs, simd_fn, |result, args| {
        for i in 0..SimdArg::WIDTH {
            let scalar_result =
                scalar_fn.call((<ScalarArg as SimdBaseIo>::set1(args.0[i]), args.1))[0];
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

pub fn horizontal_add_tester<
    RN: ScalarNumber + Add<Output = RN> + Default,
    N: ScalarNumber + Add<Output = N> + Default + Into<RN>,
    SimdArg: SimdBase<Scalar = N>,
>(
    inputs: impl Iterator<Item = (SimdArg,)>,
    simd_fn: impl Func<(SimdArg,), Output = RN>,
) {
    check_function(inputs, simd_fn, |result, args| {
        let mut sum: RN = Default::default();
        for scalar in args.0.iter() {
            sum = sum.unchecked_add(scalar.into());
        }

        let equal = sum.almost_eq(result, EqPrecision::almost(5));
        if !equal {
            return Err(format!("Failed: Expected sum to be {sum}, got {result}",));
        }
        Ok(())
    });
}

pub fn unsigned_horizontal_add_tester<
    N: IntScalarNumber + Add<Output = N> + Default,
    SimdArg: SimdBase<Scalar = N>,
>(
    inputs: impl Iterator<Item = (SimdArg,)>,
    simd_fn: impl Func<(SimdArg,), Output = i64>,
) {
    check_function(inputs, simd_fn, |result, args| {
        let mut sum: i64 = 0;
        for scalar in args.0.iter() {
            sum = sum.wrapping_add(scalar.unsigned_cast_to_i64());
        }

        let equal = sum.almost_eq(result, EqPrecision::almost(5));
        if !equal {
            return Err(format!("Failed: Expected sum to be {sum}, got {result}",));
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
    ($scalar_ty:ident, $simd_kind:ident :: $simd_ty:ident :: $fn_name:ident, $inputs:expr) => {{
        let f = |val, s| {
            macro_rules! expand {
                ($imm8:literal) => {
                    <<$simd_kind as Simd>::$simd_ty as SimdInt>::$fn_name::<$imm8>(val)
                };
            }
            test_constify_imm8_for_bitshift!($scalar_ty, s, expand)
        };
        let sf = |val, s| {
            macro_rules! expand {
                ($imm8:literal) => {
                    <<Scalar as Simd>::$simd_ty as SimdInt>::$fn_name::<$imm8>(val)
                };
            }
            test_constify_imm8_for_bitshift!($scalar_ty, s, expand)
        };
        bitshift_eq_tester($inputs, f, sf);
    }};
}

#[macro_export]
macro_rules! horizontal_add_tester {
    (signed $simd_kind:ident :: $simd_ty:ident, $inputs:expr) => {{
        let f = <<$simd_kind as Simd>::$simd_ty as SimdBaseOps>::horizontal_add;
        horizontal_add_tester($inputs, f);
    }};

    (unsigned $simd_kind:ident :: $simd_ty:ident, $inputs:expr) => {{
        let f = <<$simd_kind as Simd>::$simd_ty as SimdInt>::horizontal_unsigned_add;
        unsigned_horizontal_add_tester($inputs, f);
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
    (Neon, $($r:tt)+) => {
        #[cfg(target_feature = "neon")]
        $($r)+
    };
    (Wasm, $($r:tt)+) => {
        #[cfg(target_feature = "simd128")]
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
        elementwise_eq_tester_impl!(@full Neon, $simd_ty, $simd_base, $simd_fn, $arg_cnt, $precision);
        elementwise_eq_tester_impl!(@full Wasm, $simd_ty, $simd_base, $simd_fn, $arg_cnt, $precision);
    };

    (SimdBaseOps, $simd_fn:ident, $arg_cnt:ident, $precision:expr) => {
        elementwise_eq_tester_impl!(@simdkind i8, SimdBaseOps, $simd_fn, $arg_cnt, $precision);
        elementwise_eq_tester_impl!(@simdkind i16, SimdBaseOps, $simd_fn, $arg_cnt, $precision);
        elementwise_eq_tester_impl!(@simdkind i32, SimdBaseOps, $simd_fn, $arg_cnt, $precision);
        elementwise_eq_tester_impl!(@simdkind i64, SimdBaseOps, $simd_fn, $arg_cnt, $precision);
        elementwise_eq_tester_impl!(@simdkind f32, SimdBaseOps, $simd_fn, $arg_cnt, $precision);
        elementwise_eq_tester_impl!(@simdkind f64, SimdBaseOps, $simd_fn, $arg_cnt, $precision);
    };

    (SimdInt, $simd_fn:ident, $arg_cnt:ident, $precision:expr) => {
        elementwise_eq_tester_impl!(@simdkind i8, SimdInt, $simd_fn, $arg_cnt, $precision);
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

    (SimdInt32, $simd_fn:ident, $arg_cnt:ident, $precision:expr) => {
        elementwise_eq_tester_impl!(@simdkind i32, SimdInt32, $simd_fn, $arg_cnt, $precision);
    };

    (SimdFloat64, $simd_fn:ident, $arg_cnt:ident, $precision:expr) => {
        elementwise_eq_tester_impl!(@simdkind f64, SimdFloat64, $simd_fn, $arg_cnt, $precision);
    };

    (SimdInt64, $simd_fn:ident, $arg_cnt:ident, $precision:expr) => {
        elementwise_eq_tester_impl!(@simdkind i64, SimdInt64, $simd_fn, $arg_cnt, $precision);
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
                        $simd_ty,
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
        bitshift_eq_tester_impl!(@full $is_const, Neon, $simd_ty, $simd_fn);
        bitshift_eq_tester_impl!(@full $is_const, Wasm, $simd_ty, $simd_fn);
    };

    ($is_const:ident $simd_fn:ident) => {
        bitshift_eq_tester_impl!(@simdkind $is_const, i8, $simd_fn);
        bitshift_eq_tester_impl!(@simdkind $is_const, i16, $simd_fn);
        bitshift_eq_tester_impl!(@simdkind $is_const, i32, $simd_fn);
        bitshift_eq_tester_impl!(@simdkind $is_const, i64, $simd_fn);
    };
}

#[macro_export]
macro_rules! horizontal_add_tester_impl {
    (@full $kind:ident, $simd:ident, $simd_ty:ident) => {
        with_feature_flag!($simd,
            paste::item! {
                #[test]
                fn [<$kind _horizontal_add_ $simd:lower _ $simd_ty>]() {
                    horizontal_add_tester!(
                        $kind
                        $simd:: [<V$simd_ty>],
                        RandSimd::$simd_ty().one_arg()
                    );
                }
            }
        );
    };

    (@simdkind $kind:ident, $simd_ty:ident) => {
        horizontal_add_tester_impl!(@full $kind, Scalar, $simd_ty);
        horizontal_add_tester_impl!(@full $kind, Avx2, $simd_ty);
        horizontal_add_tester_impl!(@full $kind, Sse2, $simd_ty);
        horizontal_add_tester_impl!(@full $kind, Sse41, $simd_ty);
        horizontal_add_tester_impl!(@full $kind, Neon, $simd_ty);
        horizontal_add_tester_impl!(@full $kind, Wasm, $simd_ty);
    };

    (signed) => {
        horizontal_add_tester_impl!(@simdkind signed, i8);
        horizontal_add_tester_impl!(@simdkind signed, i16);
        horizontal_add_tester_impl!(@simdkind signed, i32);
        horizontal_add_tester_impl!(@simdkind signed, i64);
        horizontal_add_tester_impl!(@simdkind signed, f32);
        horizontal_add_tester_impl!(@simdkind signed, f64);
    };

    (unsigned) => {
        horizontal_add_tester_impl!(@simdkind unsigned, i8);
        horizontal_add_tester_impl!(@simdkind unsigned, i16);
        horizontal_add_tester_impl!(@simdkind unsigned, i32);
        horizontal_add_tester_impl!(@simdkind unsigned, i64);
    };
}
