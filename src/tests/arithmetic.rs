use quickcheck::{Arbitrary, Gen, TestResult, Testable};

use crate::{avx2::*, scalar::*, sse41::*, sse2::*};
use crate::{tests::ArbitrarySimd, Simd, SimdBase, SimdFloat};

use super::ScalarNumber;

fn check_function_1_arg<N: ScalarNumber, S: 'static + SimdBase<Scalar = N>>(
    f: unsafe fn(S) -> S,
    check: fn(S::Scalar) -> S::Scalar,
) {
    use quickcheck::quickcheck;

    struct Check<N: ScalarNumber, S: 'static + SimdBase<Scalar = N>> {
        f: unsafe fn(S) -> S,
        check: fn(S::Scalar) -> S::Scalar,
    }

    impl<N: ScalarNumber, S: 'static + SimdBase<Scalar = N>> Testable for Check<N, S> {
        fn result<G: Gen>(&self, gen: &mut G) -> TestResult {
            let arbitrary: ArbitrarySimd<N, S> = Arbitrary::arbitrary(gen);

            unsafe {
                let result = (self.f)(arbitrary.0);

                let mut expected = S::zeroes();
                let mut success = true;
                for i in 0..S::WIDTH {
                    expected[i] = (self.check)(arbitrary.0[i]);

                    if !expected[i].almost_eq(result[i]) {
                        success = false;
                    }
                }

                if success {
                    TestResult::passed()
                } else {
                    TestResult::error(format!(
                        "\nExpected {:?}, got {:?}\nInputs: {:?}\n",
                        expected, result, arbitrary.0
                    ))
                }
            }
        }
    }

    let checker = Check { f, check };

    quickcheck(checker)
}

fn check_function_2_arg<N: ScalarNumber, S: 'static + SimdBase<Scalar = N>>(
    f: unsafe fn(S, S) -> S,
    check: fn(S::Scalar, S::Scalar) -> S::Scalar,
) {
    use quickcheck::quickcheck;

    struct Check<N: ScalarNumber, S: 'static + SimdBase<Scalar = N>> {
        f: unsafe fn(S, S) -> S,
        check: fn(S::Scalar, S::Scalar) -> S::Scalar,
    }

    impl<N: ScalarNumber, S: 'static + SimdBase<Scalar = N>> Testable for Check<N, S> {
        fn result<G: Gen>(&self, gen: &mut G) -> TestResult {
            let arbitrary1: ArbitrarySimd<N, S> = Arbitrary::arbitrary(gen);
            let arbitrary2: ArbitrarySimd<N, S> = Arbitrary::arbitrary(gen);

            unsafe {
                let result = (self.f)(arbitrary1.0, arbitrary2.0);

                let mut expected = S::zeroes();
                let mut success = true;
                for i in 0..S::WIDTH {
                    expected[i] = (self.check)(arbitrary1.0[i], arbitrary2.0[i]);

                    if !expected[i].almost_eq(result[i]) {
                        success = false;
                    }
                }

                if success {
                    TestResult::passed()
                } else {
                    TestResult::error(format!(
                        "\nExpected {:?}, got {:?}\nInputs: {:?}, {:?}\n",
                        expected, result, arbitrary1.0, arbitrary2.0
                    ))
                }
            }
        }
    }

    let checker = Check { f, check };

    quickcheck(checker)
}

fn check_function_3_arg<N: ScalarNumber, S: 'static + SimdBase<Scalar = N>>(
    f: unsafe fn(S, S, S) -> S,
    check: fn(S::Scalar, S::Scalar, S::Scalar) -> S::Scalar,
) {
    use quickcheck::quickcheck;

    struct Check<N: ScalarNumber, S: 'static + SimdBase<Scalar = N>> {
        f: unsafe fn(S, S, S) -> S,
        check: fn(S::Scalar, S::Scalar, S::Scalar) -> S::Scalar,
    }

    impl<N: ScalarNumber, S: 'static + SimdBase<Scalar = N>> Testable for Check<N, S> {
        fn result<G: Gen>(&self, gen: &mut G) -> TestResult {
            let arbitrary1: ArbitrarySimd<N, S> = Arbitrary::arbitrary(gen);
            let arbitrary2: ArbitrarySimd<N, S> = Arbitrary::arbitrary(gen);
            let arbitrary3: ArbitrarySimd<N, S> = Arbitrary::arbitrary(gen);

            unsafe {
                let result = (self.f)(arbitrary1.0, arbitrary2.0, arbitrary3.0);

                let mut expected = S::zeroes();
                let mut success = true;
                for i in 0..S::WIDTH {
                    expected[i] = (self.check)(arbitrary1.0[i], arbitrary2.0[i], arbitrary3.0[i]);

                    if !expected[i].almost_eq(result[i]) {
                        success = false;
                    }
                }

                if success {
                    TestResult::passed()
                } else {
                    TestResult::error(format!(
                        "\nExpected {:?}, got {:?}\nInputs: {:?}, {:?}, {:?}\n",
                        expected, result, arbitrary1.0, arbitrary2.0, arbitrary3.0
                    ))
                }
            }
        }
    }

    let checker = Check { f, check };

    quickcheck(checker)
}

macro_rules! build_checker {
    ($sub:ident, $f:ident, check_function_1_arg) => {
        |x| {
            type S = <Scalar as Simd>::$sub;
            unsafe {
                let x = S::from_underlying_value(x);
                let result = S::$f(x);
                result.underlying_value()
            }
        }
    };
    ($sub:ident, $f:ident, check_function_2_arg) => {
        |x, y| {
            type S = <Scalar as Simd>::$sub;
            unsafe {
                let x = S::from_underlying_value(x);
                let y = S::from_underlying_value(y);
                let result = S::$f(x, y);
                result.underlying_value()
            }
        }
    };
    ($sub:ident, $f:ident, check_function_3_arg) => {
        |x, y, z| {
            type S = <Scalar as Simd>::$sub;
            unsafe {
                let x = S::from_underlying_value(x);
                let y = S::from_underlying_value(y);
                let z = S::from_underlying_value(z);
                let result = S::$f(x, y, z);
                result.underlying_value()
            }
        }
    };
}

macro_rules! check_fn_against_scalar {
    ($checker:ident, $simd:ident, $sub:ident, $f:ident) => {
        $checker(
            <$simd as Simd>::$sub::$f,
            build_checker!($sub, $f, $checker),
        );
    };
}

macro_rules! check_function {
    (base, $checker:ident, $simd:ident, $f:ident) => {
        check_fn_against_scalar!($checker, $simd, Vi16, $f);
        check_fn_against_scalar!($checker, $simd, Vi32, $f);
        check_fn_against_scalar!($checker, $simd, Vi64, $f);
        check_fn_against_scalar!($checker, $simd, Vf32, $f);
        check_fn_against_scalar!($checker, $simd, Vf64, $f);
    };
    (int, $checker:ident, $simd:ident, $f:ident) => {
        check_fn_against_scalar!($checker, $simd, Vi16, $f);
        check_fn_against_scalar!($checker, $simd, Vi32, $f);
        check_fn_against_scalar!($checker, $simd, Vi64, $f);
    };
    (float, $checker:ident, $simd:ident, $f:ident) => {
        check_fn_against_scalar!($checker, $simd, Vf32, $f);
        check_fn_against_scalar!($checker, $simd, Vf64, $f);
    };
    (i16, $checker:ident, $simd:ident, $f:ident) => {
        check_fn_against_scalar!($checker, $simd, Vi16, $f);
    };
    (i32, $checker:ident, $simd:ident, $f:ident) => {
        check_fn_against_scalar!($checker, $simd, Vi32, $f);
    };
    (i64, $checker:ident, $simd:ident, $f:ident) => {
        check_fn_against_scalar!($checker, $simd, Vi64, $f);
    };
    (f32, $checker:ident, $simd:ident, $f:ident) => {
        check_fn_against_scalar!($checker, $simd, Vf32, $f);
    };
    (f64, $checker:ident, $simd:ident, $f:ident) => {
        check_fn_against_scalar!($checker, $simd, Vf64, $f);
    };

    ($kind:ident, $simd:ident, $f:ident, 1_arg) => {
        check_function!($kind, check_function_1_arg, $simd, $f);
    };
    ($kind:ident, $simd:ident, $f:ident, 2_arg) => {
        check_function!($kind, check_function_2_arg, $simd, $f);
    };
    ($kind:ident, $simd:ident, $f:ident, 3_arg) => {
        check_function!($kind, check_function_3_arg, $simd, $f);
    };
}

macro_rules! make_tests {
    ($kind:ident, $f:ident, $($rest:tt)+) => {
        paste::item! {
            #[test]
            fn [<$f _avx2>]() {
                check_function!($kind, Avx2, $f, $($rest)+);
            }

            #[test]
            fn [<$f _scalar>]() {
                check_function!($kind, Scalar, $f, $($rest)+);
            }

            #[test]
            fn [<$f _sse2>]() {
                check_function!($kind, Sse2, $f, $($rest)+);
            }

            #[test]
            fn [<$f _sse41>]() {
                check_function!($kind, Sse41, $f, $($rest)+);
            }
        }
    };
}

mod base {
    use super::*;

    make_tests!(base, add, 2_arg);
    make_tests!(base, sub, 2_arg);
    make_tests!(base, mul, 2_arg);

    make_tests!(base, bit_and, 2_arg);
    make_tests!(base, bit_or, 2_arg);
    make_tests!(base, bit_xor, 2_arg);

    make_tests!(base, bit_not, 1_arg);

    make_tests!(base, abs, 1_arg);

    make_tests!(base, and_not, 2_arg);

    make_tests!(base, cmp_eq, 2_arg);
    make_tests!(base, cmp_neq, 2_arg);
    make_tests!(base, cmp_lt, 2_arg);
    make_tests!(base, cmp_lte, 2_arg);
    make_tests!(base, cmp_gt, 2_arg);
    make_tests!(base, cmp_gte, 2_arg);

    make_tests!(base, min, 2_arg);
    make_tests!(base, max, 2_arg);
}

mod float {
    use super::*;

    make_tests!(float, div, 2_arg);

    make_tests!(float, ceil, 1_arg);
    make_tests!(float, floor, 1_arg);
    make_tests!(float, round, 1_arg);
    make_tests!(float, fast_ceil, 1_arg);
    make_tests!(float, fast_floor, 1_arg);
    make_tests!(float, fast_round, 1_arg);

    make_tests!(float, mul_add, 3_arg);
    make_tests!(float, mul_sub, 3_arg);
    make_tests!(float, neg_mul_add, 3_arg);
    make_tests!(float, neg_mul_sub, 3_arg);

    make_tests!(float, sqrt, 1_arg);
    make_tests!(float, rsqrt, 1_arg);
}

mod float32 {
    // FIXME: Create a better test for this as it's too imprecise
    // make_tests!(f32, fast_inverse, 1_arg);
}
