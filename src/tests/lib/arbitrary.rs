//! This module allows generating a (seeded) random list of inputs that includes special numbers as well as
//! seeded random ones. The IMPORTANT_[ty] constants contain the important edge cases to try out.

use core::iter;

use rand::{
    distributions::uniform::{SampleRange, SampleUniform},
    prelude::*,
};
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};

use crate::{SimdBase, SimdBaseIo};

use super::ScalarNumber;

const IMPORTANT_F32: [f32; 10] = [
    0.0,
    1.0,
    -1.0,
    0.5,
    -0.5,
    1.5,
    -1.5,
    f32::MAX,
    f32::MIN,
    f32::NAN,
];

const IMPORTANT_F64: [f64; 10] = [
    0.0,
    1.0,
    -1.0,
    0.5,
    -0.5,
    1.5,
    -1.5,
    f64::MAX,
    f64::MIN,
    f64::NAN,
];

const IMPORTANT_I8: [i8; 7] = [0, 1, -1, 2, -2, i8::MAX, i8::MIN];
const IMPORTANT_I16: [i16; 7] = [0, 1, -1, 2, -2, i16::MAX, i16::MIN];
const IMPORTANT_I32: [i32; 7] = [0, 1, -1, 2, -2, i32::MAX, i32::MIN];
const IMPORTANT_I64: [i64; 7] = [0, 1, -1, 2, -2, i64::MAX, i64::MIN];

fn iter_arbitrary_f32(interval: usize) -> impl Iterator<Item = f32> {
    assert!(interval > IMPORTANT_F32.len());

    let rng_count = interval - IMPORTANT_F32.len();

    let make_important_iter = || IMPORTANT_F32.iter().cloned();
    let mut i = 0;
    let mut make_random_iter = move || {
        i += 1;
        let mut rng = ChaCha8Rng::seed_from_u64((interval + i) as u64);
        iter::repeat_with(move || f32::from_bits(rng.gen_range(0..u32::MAX)))
    };

    iter::repeat_with(move || make_important_iter().chain(make_random_iter().take(rng_count)))
        .flatten()
}

fn iter_arbitrary_f64(interval: usize) -> impl Iterator<Item = f64> {
    assert!(interval > IMPORTANT_F64.len());

    let rng_count = interval - IMPORTANT_F64.len();

    let make_important_iter = || IMPORTANT_F64.iter().cloned();
    let mut i = 0;
    let mut make_random_iter = move || {
        i += 1;
        let mut rng = ChaCha8Rng::seed_from_u64((interval + i) as u64);
        iter::repeat_with(move || f64::from_bits(rng.gen_range(0..u64::MAX)))
    };

    iter::repeat_with(move || make_important_iter().chain(make_random_iter().take(rng_count)))
        .flatten()
}

fn iter_arbitrary_ints<T: SampleUniform + Clone>(
    interval: usize,
    important_ints: &'static [T],
    range: impl SampleRange<T> + Clone,
) -> impl Iterator<Item = T> {
    assert!(interval > important_ints.len());

    let rng_count = interval - important_ints.len();

    let make_important_iter = move || important_ints.iter().cloned();
    let mut i = 0;
    let mut make_random_iter = move || {
        i += 1;
        let mut rng = ChaCha8Rng::seed_from_u64((interval + i) as u64);
        let range = range.clone();
        iter::repeat_with(move || rng.gen_range(range.clone()))
    };

    iter::repeat_with(move || make_important_iter().chain(make_random_iter().take(rng_count)))
        .flatten()
}

fn iter_arbitrary_i8(interval: usize) -> impl Iterator<Item = i8> {
    iter_arbitrary_ints(interval, &IMPORTANT_I8, i8::MIN..=i8::MAX)
}

fn iter_arbitrary_i16(interval: usize) -> impl Iterator<Item = i16> {
    iter_arbitrary_ints(interval, &IMPORTANT_I16, i16::MIN..=i16::MAX)
}

fn iter_arbitrary_i32(interval: usize) -> impl Iterator<Item = i32> {
    iter_arbitrary_ints(interval, &IMPORTANT_I32, i32::MIN..=i32::MAX)
}

fn iter_arbitrary_i64(interval: usize) -> impl Iterator<Item = i64> {
    iter_arbitrary_ints(interval, &IMPORTANT_I64, i64::MIN..=i64::MAX)
}

fn iter_arbitrary_blendv_i8() -> impl Iterator<Item = i8> {
    [-1, 0].iter().cycle().copied()
}

fn iter_arbitrary_blendv_i16() -> impl Iterator<Item = i16> {
    [-1, 0].iter().cycle().copied()
}

fn iter_arbitrary_blendv_i32() -> impl Iterator<Item = i32> {
    [-1, 0].iter().cycle().copied()
}

fn iter_arbitrary_blendv_i64() -> impl Iterator<Item = i64> {
    [-1, 0].iter().cycle().copied()
}

fn iter_arbitrary_blendv_f32() -> impl Iterator<Item = f32> {
    iter::once(f32::from_bits(u32::MAX))
        .chain(iter::once(f32::from_bits(0)))
        .cycle()
}

fn iter_arbitrary_blendv_f64() -> impl Iterator<Item = f64> {
    iter::once(f64::from_bits(u64::MAX))
        .chain(iter::once(f64::from_bits(0)))
        .cycle()
}

fn iter_arbitrary_bitshift_ints(max: u32) -> impl Iterator<Item = i32> {
    let mut i = 0;
    iter::repeat_with(move || {
        i += 1;
        i %= max;
        i as i32
    })
}

/// Convert an iterator of scalars into an iterator of SIMD vectors.
fn iter_as_simd<S: SimdBase<Scalar = N>, N>(
    mut iter: impl Iterator<Item = N>,
) -> impl Iterator<Item = S> {
    iter::repeat_with(move || {
        let mut v = <S as SimdBaseIo>::zeroes();
        for i in 0..S::WIDTH {
            v[i] = iter.next().unwrap();
        }
        v
    })
}

/// Generate a random iterator of scalars of tuples for that type.
///
/// E.g. `RandSimd::f32().two_arg()` will generate an iterator of `(S,S)` values
/// where S's scalar type is f32 and S is inferred.
pub struct RandSimd;
pub struct IterRandSimdForScalar<N, I: Iterator<Item = N>, I2: Iterator<Item = N>> {
    any: Box<dyn Fn(usize) -> I>,
    blendv: Box<dyn Fn() -> I2>,
    scalar_size: usize,
}

impl RandSimd {
    pub fn f32() -> IterRandSimdForScalar<f32, impl Iterator<Item = f32>, impl Iterator<Item = f32>>
    {
        IterRandSimdForScalar {
            any: Box::new(iter_arbitrary_f32),
            blendv: Box::new(iter_arbitrary_blendv_f32),
            scalar_size: 4,
        }
    }
    pub fn f64() -> IterRandSimdForScalar<f64, impl Iterator<Item = f64>, impl Iterator<Item = f64>>
    {
        IterRandSimdForScalar {
            any: Box::new(iter_arbitrary_f64),
            blendv: Box::new(iter_arbitrary_blendv_f64),
            scalar_size: 8,
        }
    }
    pub fn i8() -> IterRandSimdForScalar<i8, impl Iterator<Item = i8>, impl Iterator<Item = i8>> {
        IterRandSimdForScalar {
            any: Box::new(iter_arbitrary_i8),
            blendv: Box::new(iter_arbitrary_blendv_i8),
            scalar_size: 1,
        }
    }
    pub fn i16() -> IterRandSimdForScalar<i16, impl Iterator<Item = i16>, impl Iterator<Item = i16>>
    {
        IterRandSimdForScalar {
            any: Box::new(iter_arbitrary_i16),
            blendv: Box::new(iter_arbitrary_blendv_i16),
            scalar_size: 2,
        }
    }
    pub fn i32() -> IterRandSimdForScalar<i32, impl Iterator<Item = i32>, impl Iterator<Item = i32>>
    {
        IterRandSimdForScalar {
            any: Box::new(iter_arbitrary_i32),
            blendv: Box::new(iter_arbitrary_blendv_i32),
            scalar_size: 4,
        }
    }
    pub fn i64() -> IterRandSimdForScalar<i64, impl Iterator<Item = i64>, impl Iterator<Item = i64>>
    {
        IterRandSimdForScalar {
            any: Box::new(iter_arbitrary_i64),
            blendv: Box::new(iter_arbitrary_blendv_i64),
            scalar_size: 8,
        }
    }
}

impl<N: ScalarNumber, I: Iterator<Item = N>, I2: Iterator<Item = N>>
    IterRandSimdForScalar<N, I, I2>
{
    /// Iterate 1000 random inputs, starting from the important numbers
    pub fn one_arg<S: SimdBase<Scalar = N>>(self) -> impl Iterator<Item = (S,)> {
        let iter = iter_as_simd((self.any)(1000));
        iter.map(|v| (v,)).take(1000 * S::WIDTH)
    }

    /// Iterate `14 * 15 * 20 * S::WIDTH` random inputs, with the first argument looping at 14 and
    /// the second arguming looping at 15 inputs, effectively putting each special number against each
    /// other one in the end.
    pub fn two_arg<S: SimdBase<Scalar = N>>(self) -> impl Iterator<Item = (S, S)> {
        let iter1 = iter_as_simd((self.any)(14));
        let iter2 = iter_as_simd((self.any)(15));
        iter1.zip(iter2).take(14 * 15 * 20 * S::WIDTH)
    }

    /// Same as two_arg except with periods of 14, 15 and 16.
    pub fn three_arg<S: SimdBase<Scalar = N>>(self) -> impl Iterator<Item = (S, S, S)> {
        let mut iter1 = iter_as_simd((self.any)(14));
        let mut iter2 = iter_as_simd((self.any)(15));
        let mut iter3 = iter_as_simd((self.any)(16));

        iter::repeat_with(move || {
            (
                iter1.next().unwrap(),
                iter2.next().unwrap(),
                iter3.next().unwrap(),
            )
        })
        .take(1680 * S::WIDTH)
    }

    /// Same as one_arg, except filtering out invalid inputs for abs functions
    pub fn one_arg_abs_filtered<S: SimdBase<Scalar = N>>(self) -> impl Iterator<Item = (S,)> {
        let iter = iter_as_simd((self.any)(1000).filter(|v| !v.is_minimum_int()));
        iter.map(|v| (v,)).take(1000 * S::WIDTH)
    }

    /// Same as one_arg, except without values that can cause undefined behavior when rounding
    pub fn one_arg_rounding_safe<S: SimdBase<Scalar = N>>(self) -> impl Iterator<Item = (S,)> {
        let iter =
            iter_as_simd((self.any)(1000).filter(|v| !v.is_undefined_behavior_when_rounding()));
        iter.map(|v| (v,)).take(1000 * S::WIDTH)
    }

    /// Same as two_arg, except filtering out NaN floats
    pub fn two_arg_nan_filtered<S: SimdBase<Scalar = N>>(self) -> impl Iterator<Item = (S, S)> {
        let iter1 = iter_as_simd((self.any)(15).filter(|v| !v.is_float_nan()));
        let iter2 = iter_as_simd((self.any)(16).filter(|v| !v.is_float_nan()));
        iter1.zip(iter2).take(14 * 15 * 20 * S::WIDTH)
    }

    /// A blendv mask that's not all 0's or all 1's is undefined behavior between different architectures.
    pub fn iter_blendv_ags<S: SimdBase<Scalar = N>>(self) -> impl Iterator<Item = (S, S, S)> {
        let mut mask_iter = iter_as_simd((self.blendv)());
        let mut iter2 = iter_as_simd((self.any)(15));
        let mut iter3 = iter_as_simd((self.any)(16));

        iter::repeat_with(move || {
            (
                mask_iter.next().unwrap(),
                iter2.next().unwrap(),
                iter3.next().unwrap(),
            )
        })
        .take(100 * S::WIDTH)
    }

    /// Same as one_arg, except with a second arg as a bitshift i32 number
    pub fn one_arg_and_bitshift_arg<S: SimdBase<Scalar = N>>(
        self,
    ) -> impl Iterator<Item = (S, i32)> {
        let iter = iter_as_simd((self.any)(1000));
        let bitshift_iter = iter_arbitrary_bitshift_ints(self.scalar_size as u32 * 8);
        iter.zip(bitshift_iter).take(1000 * S::WIDTH)
    }

    /// Same as one_arg, except it adjusts the values to avoid undefined behavior
    pub fn float_to_int_cast_values<S: SimdBase<Scalar = N>>(self) -> impl Iterator<Item = (S,)> {
        let iter = iter_as_simd((self.any)(1000).filter(|v| {
            !v.is_undefined_behavior_when_casting() && !v.is_undefined_behavior_when_rounding()
        }));
        iter.map(|v| (v,)).take(1000 * S::WIDTH)
    }
}
