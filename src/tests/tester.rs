use core::fmt::Debug;

use crate::{avx2::Avx2, scalar::Scalar, Simd, SimdBase, SimdFloat};

use super::{
    arbitrary::RandSimd,
    fn_tuple::{Func, Tuple},
    EqPrecision, ScalarNumber, SimdTupleIterable,
};

/// For each input, get the result, then, run the checker function on the result
/// and panic if the checker errors
fn check_function<Args: Tuple + Debug + Clone, R>(
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
fn check_elementwise_function<
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

fn elementwise_eq_tester<
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

macro_rules! elementwise_eq_tester {
    ($inputs:expr, $precision:expr, $simd_kind:ident :: $simd_ty:ident :: $fn_name:ident) => {{
        type S = <$simd_kind as Simd>::$simd_ty;
        type Sc = <Scalar as Simd>::$simd_ty;
        elementwise_eq_tester($inputs, $precision, S::$fn_name, Sc::$fn_name);
    }};
}

#[test]
fn test_ASDFASDFASDFSDAFSafD() {
    // elementwise_eq_tester(
    //     RandSimd::f32().three_arg(),
    //     EqPrecision::Exact,
    //     <<Avx2 as Simd>::Vf32 as SimdFloat>::mul_add,
    //     <<Scalar as Simd>::Vf32 as SimdFloat>::mul_add,
    // );
    elementwise_eq_tester!(
        RandSimd::f32().three_arg(),
        EqPrecision::Exact,
        Avx2::Vf32::mul_add
    );
}
