use quickcheck::Arbitrary;

use crate::SimdBase;

#[derive(Clone, Copy, Debug)]
pub enum EqPrecision {
    Exact,
    Almost { figs: usize },
}

impl EqPrecision {
    pub fn exact() -> EqPrecision {
        Self::exact()
    }

    pub fn almost(figs: usize) -> EqPrecision {
        Self::Almost { figs }
    }
}

pub trait ScalarNumber: Arbitrary + PartialEq + Copy + core::fmt::Display {
    fn almost_eq(self, other: Self, precision: EqPrecision) -> bool;
}

impl ScalarNumber for i16 {
    fn almost_eq(self, other: Self, precision: EqPrecision) -> bool {
        self == other
    }
}
impl ScalarNumber for i32 {
    fn almost_eq(self, other: Self, precision: EqPrecision) -> bool {
        self == other
    }
}
impl ScalarNumber for i64 {
    fn almost_eq(self, other: Self, precision: EqPrecision) -> bool {
        self == other
    }
}
impl ScalarNumber for f32 {
    fn almost_eq(self, other: Self, precision: EqPrecision) -> bool {
        if self.is_nan() && other.is_nan() {
            return true;
        }

        if (self.is_infinite() && self > 0.0) || (other.is_infinite() && other > 0.0) {
            return true;
        }

        if (self.is_infinite() && self < 0.0) || (other.is_infinite() && other < 0.0) {
            return true;
        }

        match precision {
            EqPrecision::Exact => self == other,
            EqPrecision::Almost { figs } => {
                1.0 / (self.log10() - other.log10()).abs() < figs as f32
            }
        }
    }
}
impl ScalarNumber for f64 {
    fn almost_eq(self, other: Self, precision: EqPrecision) -> bool {
        if self.is_nan() && other.is_nan() {
            return true;
        }

        if (self.is_infinite() && self > 0.0) || (other.is_infinite() && other > 0.0) {
            return true;
        }

        if (self.is_infinite() && self < 0.0) || (other.is_infinite() && other < 0.0) {
            return true;
        }

        match precision {
            EqPrecision::Exact => self == other,
            EqPrecision::Almost { figs } => {
                1.0 / (self.log10() - other.log10()).abs() < figs as f64
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ArbitrarySimd<Scalar, S: SimdBase<Scalar = Scalar>>(pub S);

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

pub trait SimdTupleIterable<S: ScalarNumber> {
    type AsScalar;
    type AsTuple<V: SimdBase<Scalar = S>>;

    fn wrap_scalars<V: SimdBase<Scalar = S>>(scalars: Self::AsScalar) -> Self::AsTuple<V>;

    fn iter_scalars<'a>(&'a self) -> Box<dyn 'a + Iterator<Item = Self::AsScalar>>;
}

impl<S: ScalarNumber, T: SimdBase<Scalar = S>> SimdTupleIterable<S> for (T,) {
    type AsScalar = (S,);
    type AsTuple<V: SimdBase<Scalar = S>> = (V,);

    fn iter_scalars<'a>(&'a self) -> Box<dyn 'a + Iterator<Item = Self::AsScalar>> {
        let range = 0..T::WIDTH;
        let iter = range.map(move |i| (self.0[i],));
        Box::new(iter)
    }

    fn wrap_scalars<V: SimdBase<Scalar = S>>(scalars: Self::AsScalar) -> Self::AsTuple<V> {
        unsafe { (V::set1(scalars.0),) }
    }
}

impl<S: ScalarNumber, T: SimdBase<Scalar = S>> SimdTupleIterable<S> for (T, T) {
    type AsScalar = (S, S);
    type AsTuple<V: SimdBase<Scalar = S>> = (V, V);

    fn iter_scalars<'a>(&'a self) -> Box<dyn 'a + Iterator<Item = Self::AsScalar>> {
        let range = 0..T::WIDTH;
        let iter = range.map(move |i| (self.0[i], self.1[i]));
        Box::new(iter)
    }

    fn wrap_scalars<V: SimdBase<Scalar = S>>(scalars: Self::AsScalar) -> Self::AsTuple<V> {
        unsafe { (V::set1(scalars.0), V::set1(scalars.1)) }
    }
}

impl<S: ScalarNumber, T: SimdBase<Scalar = S>> SimdTupleIterable<S> for (T, T, T) {
    type AsScalar = (S, S, S);
    type AsTuple<V: SimdBase<Scalar = S>> = (V, V, V);

    fn iter_scalars<'a>(&'a self) -> Box<dyn 'a + Iterator<Item = Self::AsScalar>> {
        let range = 0..T::WIDTH;
        let iter = range.map(move |i| (self.0[i], self.1[i], self.2[i]));
        Box::new(iter)
    }

    fn wrap_scalars<V: SimdBase<Scalar = S>>(scalars: Self::AsScalar) -> Self::AsTuple<V> {
        unsafe { (V::set1(scalars.0), V::set1(scalars.1), V::set1(scalars.2)) }
    }
}
