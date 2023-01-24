use quickcheck::Arbitrary;

use crate::SimdBase;

mod arithmetic;

trait ScalarNumber: Arbitrary + PartialEq + Copy {
    fn almost_eq(self, other: Self) -> bool;
}

impl ScalarNumber for i16 {
    fn almost_eq(self, other: Self) -> bool {
        self == other
    }
}
impl ScalarNumber for i32 {
    fn almost_eq(self, other: Self) -> bool {
        self == other
    }
}
impl ScalarNumber for i64 {
    fn almost_eq(self, other: Self) -> bool {
        self == other
    }
}
impl ScalarNumber for f32 {
    fn almost_eq(self, other: Self) -> bool {
        if self.is_nan() != other.is_nan() {
            return false;
        }

        if self.is_nan() || other.is_nan() {
            return true;
        }

        return (self - other).abs() < 0.00001;
    }
}
impl ScalarNumber for f64 {
    fn almost_eq(self, other: Self) -> bool {
        if self.is_nan() != other.is_nan() {
            return false;
        }

        if self.is_nan() || other.is_nan() {
            return true;
        }

        return (self - other).abs() < 0.00000001;
    }
}

#[derive(Clone, Copy, Debug)]
struct ArbitrarySimd<Scalar, S: SimdBase<Scalar = Scalar>>(S);

impl<S: 'static + SimdBase<Scalar = N>, N: ScalarNumber> Arbitrary for ArbitrarySimd<N, S> {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
        unsafe {
            let mut data = S::zeroes();
            for i in 0..S::WIDTH {
                data[i] = Arbitrary::arbitrary(g);
            }
            ArbitrarySimd(data)
        }
    }
}
