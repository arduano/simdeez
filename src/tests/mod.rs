use quickcheck::Arbitrary;

use crate::SimdBase;

mod arithmetic;

trait ScalarNumber: Arbitrary + PartialEq + Copy {}

impl ScalarNumber for i16 {}
impl ScalarNumber for i32 {}
impl ScalarNumber for i64 {}
impl ScalarNumber for f32 {}
impl ScalarNumber for f64 {}

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
