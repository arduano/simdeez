use criterion::Criterion;
use simdeez::math::SimdMathF32InverseHyperbolic;
use simdeez::{prelude::*, simd_unsafe_generate_all};

use crate::shared::{self, INPUT_LEN};

#[inline(never)]
fn scalar_asinh_sum(input: &[f32]) -> f32 {
    input.iter().copied().map(f32::asinh).sum()
}

#[inline(never)]
fn scalar_acosh_sum(input: &[f32]) -> f32 {
    input.iter().copied().map(f32::acosh).sum()
}

#[inline(never)]
fn scalar_atanh_sum(input: &[f32]) -> f32 {
    input.iter().copied().map(f32::atanh).sum()
}

simd_unsafe_generate_all!(
    fn simdeez_asinh_sum(input: &[f32]) -> f32 {
        shared::simdeez_unary_sum_impl::<S>(input, |v| v.asinh_u35())
    }
);

simd_unsafe_generate_all!(
    fn simdeez_acosh_sum(input: &[f32]) -> f32 {
        shared::simdeez_unary_sum_impl::<S>(input, |v| v.acosh_u35())
    }
);

simd_unsafe_generate_all!(
    fn simdeez_atanh_sum(input: &[f32]) -> f32 {
        shared::simdeez_unary_sum_impl::<S>(input, |v| v.atanh_u35())
    }
);

pub fn register(c: &mut Criterion) {
    let asinh_inputs = shared::make_unary_inputs(INPUT_LEN, 0xDEADB001, -16_384.0..16_384.0);
    let acosh_inputs = shared::make_positive_inputs(INPUT_LEN, 0xDEADB002, 1.0, 16_384.0);
    let atanh_inputs = shared::make_unary_inputs(INPUT_LEN, 0xDEADB003, -0.999_999..0.999_999);

    shared::bench_unary(
        c,
        "simd_math_baseline/f32/asinh_u35",
        &asinh_inputs,
        scalar_asinh_sum,
        simdeez_asinh_sum,
    );

    shared::bench_unary(
        c,
        "simd_math_baseline/f32/acosh_u35",
        &acosh_inputs,
        scalar_acosh_sum,
        simdeez_acosh_sum,
    );

    shared::bench_unary(
        c,
        "simd_math_baseline/f32/atanh_u35",
        &atanh_inputs,
        scalar_atanh_sum,
        simdeez_atanh_sum,
    );
}
