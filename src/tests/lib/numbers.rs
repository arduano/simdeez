use quickcheck::Arbitrary;

use crate::SimdBase;

#[derive(Clone, Copy, Debug)]
pub enum EqPrecision {
    Exact,
    Almost { figs: usize },
}

impl EqPrecision {
    pub fn exact() -> EqPrecision {
        Self::Exact
    }

    pub fn almost(figs: usize) -> EqPrecision {
        Self::Almost { figs }
    }
}

pub trait ScalarNumber: Arbitrary + PartialEq + Copy + core::fmt::Display {
    fn almost_eq(self, other: Self, _precision: EqPrecision) -> bool {
        self == other
    }

    fn is_minimum_int(&self) -> bool {
        false
    }

    fn is_float_nan(&self) -> bool {
        false
    }

    /// For floating point numbers, rounding from .5 numbers can cause undefined behavior.
    fn is_undefined_behavior_when_rounding(self) -> bool {
        false
    }

    /// Some floating point numbers appear to cause undefined behavior when casting, e.g. a very large positive float
    /// would end up as a negative integer.
    fn is_undefined_behavior_when_casting(self) -> bool {
        false
    }
}

impl ScalarNumber for i16 {
    fn is_minimum_int(&self) -> bool {
        *self == i16::MIN
    }
}
impl ScalarNumber for i32 {
    fn is_minimum_int(&self) -> bool {
        *self == i32::MIN
    }
}
impl ScalarNumber for i64 {
    fn is_minimum_int(&self) -> bool {
        *self == i64::MIN
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

        if self == 0.0 && other == 0.0 {
            return true;
        }

        if self.is_sign_negative() != other.is_sign_negative() {
            return false;
        }

        match precision {
            EqPrecision::Exact => self == other,
            EqPrecision::Almost { figs } => {
                let bigger = self.max(other);
                let norm_diff = (self / bigger) - (other / bigger);
                let epsilon = 10.0f32.powi(-(figs as i32));
                norm_diff < epsilon
            }
        }
    }

    fn is_float_nan(&self) -> bool {
        self.is_nan()
    }

    fn is_undefined_behavior_when_rounding(self) -> bool {
        // Anything that's close to ending in .5 may cause undefined behavior
        ((self.abs() % 1.0) - 0.5).abs() < f32::EPSILON
    }

    fn is_undefined_behavior_when_casting(self) -> bool {
        // Anything that's outside the maximum range of an i32 may cause undefined behavior
        // e.g. resulting in i32::MIN from a large positive float
        let range = (i32::MIN as f32)..=(i32::MAX as f32);
        !range.contains(&self)
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

        if self == 0.0 && other == 0.0 {
            return true;
        }

        if self.is_sign_negative() != other.is_sign_negative() {
            return false;
        }

        match precision {
            EqPrecision::Exact => self == other,
            EqPrecision::Almost { figs } => {
                let bigger = self.max(other);
                let norm_diff = (self / bigger) - (other / bigger);
                let epsilon = 10.0f64.powi(-(figs as i32));
                norm_diff < epsilon
            }
        }
    }

    fn is_float_nan(&self) -> bool {
        self.is_nan()
    }

    fn is_undefined_behavior_when_rounding(self) -> bool {
        // Anything that's close to ending in .5 may cause undefined behavior
        ((self.abs() % 1.0) - 0.5).abs() < f64::EPSILON
    }

    fn is_undefined_behavior_when_casting(self) -> bool {
        // Anything that's outside the maximum range of an i32 may cause undefined behavior
        // e.g. resulting in i64::MIN from a large positive float
        let range = (i64::MIN as f64)..=(i64::MAX as f64);
        !range.contains(&self)
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