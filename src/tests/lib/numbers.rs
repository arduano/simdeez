use crate::{SimdBase, SimdBaseIo};

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

pub trait ScalarNumber: PartialEq + Copy + core::fmt::Display {
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

    fn unchecked_add(self, other: Self) -> Self;
}

pub trait IntScalarNumber: ScalarNumber {
    fn unsigned_cast_to_i64(self) -> i64;
}

impl ScalarNumber for i8 {
    fn is_minimum_int(&self) -> bool {
        *self == i8::MIN
    }

    fn unchecked_add(self, other: Self) -> Self {
        self.wrapping_add(other)
    }
}

impl IntScalarNumber for i8 {
    fn unsigned_cast_to_i64(self) -> i64 {
        self as u8 as u64 as i64
    }
}

impl ScalarNumber for i16 {
    fn is_minimum_int(&self) -> bool {
        *self == i16::MIN
    }

    fn unchecked_add(self, other: Self) -> Self {
        self.wrapping_add(other)
    }
}

impl IntScalarNumber for i16 {
    fn unsigned_cast_to_i64(self) -> i64 {
        self as u16 as u64 as i64
    }
}

impl ScalarNumber for i32 {
    fn is_minimum_int(&self) -> bool {
        *self == i32::MIN
    }

    fn unchecked_add(self, other: Self) -> Self {
        self.wrapping_add(other)
    }
}

impl IntScalarNumber for i32 {
    fn unsigned_cast_to_i64(self) -> i64 {
        self as u32 as u64 as i64
    }
}

impl ScalarNumber for i64 {
    fn is_minimum_int(&self) -> bool {
        *self == i64::MIN
    }

    fn unchecked_add(self, other: Self) -> Self {
        self.wrapping_add(other)
    }
}

impl IntScalarNumber for i64 {
    fn unsigned_cast_to_i64(self) -> i64 {
        self
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
                let epsilon = 10.0f32.powi(-(figs as i32));
                if (self - other).abs() < epsilon {
                    return true;
                }
                let bigger = self.max(other);
                let norm_diff = (self / bigger) - (other / bigger);
                norm_diff.abs() < epsilon
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

    fn unchecked_add(self, other: Self) -> Self {
        self + other
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
                let epsilon = 10.0f64.powi(-(figs as i32));
                if (self - other).abs() < epsilon {
                    return true;
                }
                let bigger = self.max(other);
                let norm_diff = (self / bigger) - (other / bigger);
                norm_diff.abs() < epsilon
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

    fn unchecked_add(self, other: Self) -> Self {
        self + other
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
        (<V as SimdBaseIo>::set1(scalars.0),)
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
        (
            <V as SimdBaseIo>::set1(scalars.0),
            <V as SimdBaseIo>::set1(scalars.1),
        )
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
        (
            <V as SimdBaseIo>::set1(scalars.0),
            <V as SimdBaseIo>::set1(scalars.1),
            <V as SimdBaseIo>::set1(scalars.2),
        )
    }
}
