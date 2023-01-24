use quickcheck::{Arbitrary, Gen, TestResult, Testable};

use crate::{avx2::*, scalar::*};
use crate::{tests::ArbitrarySimd, Simd, SimdBase};

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

                    if expected[i] != result[i] {
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

                    if expected[i] != result[i] {
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

                    if expected[i] != result[i] {
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

macro_rules! check_function {
    (base, $checker:ident, $simd:ident, $f:ident, $check:expr) => {
        $checker(<$simd as Simd>::Vi16::$f, $check);
        $checker(<$simd as Simd>::Vi32::$f, $check);
        $checker(<$simd as Simd>::Vi64::$f, $check);
        $checker(<$simd as Simd>::Vf32::$f, $check);
        $checker(<$simd as Simd>::Vf64::$f, $check);
    };
    (int, $checker:ident, $simd:ident, $f:ident, $check:expr) => {
        $checker(<$simd as Simd>::Vi16::$f, $check);
        $checker(<$simd as Simd>::Vi32::$f, $check);
        $checker(<$simd as Simd>::Vi64::$f, $check);
    };
    (float, $checker:ident, $simd:ident, $f:ident, $check:expr) => {
        $checker(<$simd as Simd>::Vf32::$f, $check);
        $checker(<$simd as Simd>::Vf64::$f, $check);
    };
    (i16, $checker:ident, $simd:ident, $f:ident, $check:expr) => {
        $checker(<$simd as Simd>::Vi16::$f, $check);
    };
    (i32, $checker:ident, $simd:ident, $f:ident, $check:expr) => {
        $checker(<$simd as Simd>::Vi32::$f, $check);
    };
    (i64, $checker:ident, $simd:ident, $f:ident, $check:expr) => {
        $checker(<$simd as Simd>::Vi64::$f, $check);
    };
    (f32, $checker:ident, $simd:ident, $f:ident, $check:expr) => {
        $checker(<$simd as Simd>::Vf32::$f, $check);
    };
    (f64, $checker:ident, $simd:ident, $f:ident, $check:expr) => {
        $checker(<$simd as Simd>::Vf64::$f, $check);
    };

    ($kind:ident, $simd:ident, $f:ident, |$a1:ident| $($check:tt)+) => {
        check_function!($kind, check_function_1_arg, $simd, $f, |$a1| $($check)+);
    };
    ($kind:ident, $simd:ident, $f:ident, |$a1:ident, $a2:ident| $($check:tt)+) => {
        check_function!($kind, check_function_2_arg, $simd, $f, |$a1, $a2| $($check)+);
    };
    ($kind:ident, $simd:ident, $f:ident, |$a1:ident, $a2:ident, $a3:ident| $($check:tt)+) => {
        check_function!($kind, check_function_3_arg, $simd, $f, |$a1, $a2, $a3| $($check)+);
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
        }
    };
}

#[test]
fn test_add() {
    assert_eq!(1, 1);
}

make_tests!(base, add, |x, y| x + y);
