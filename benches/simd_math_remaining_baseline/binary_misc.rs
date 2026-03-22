use criterion::Criterion;
use simdeez::math::SimdMathF32BinaryMisc;
use simdeez::{prelude::*, simd_unsafe_generate_all};

use crate::shared::{self, INPUT_LEN};

#[inline(never)]
fn scalar_log10_sum(input: &[f32]) -> f32 {
    input.iter().copied().map(f32::log10).sum()
}

#[inline(never)]
fn scalar_atan2_sum(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(&x, &y)| x.atan2(y)).sum()
}

#[inline(never)]
fn scalar_fmod_sum(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(&x, &y)| x % y).sum()
}

#[inline(never)]
fn scalar_hypot_sum(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(&x, &y)| x.hypot(y)).sum()
}

simd_unsafe_generate_all!(
    fn simdeez_log10_sum(input: &[f32]) -> f32 {
        shared::simdeez_unary_sum_impl::<S>(input, |v| v.log10_u35())
    }
);

simd_unsafe_generate_all!(
    fn simdeez_atan2_sum(a: &[f32], b: &[f32]) -> f32 {
        shared::simdeez_binary_sum_impl::<S>(a, b, |x, y| x.atan2_u35(y))
    }
);

simd_unsafe_generate_all!(
    fn simdeez_hypot_sum(a: &[f32], b: &[f32]) -> f32 {
        shared::simdeez_binary_sum_impl::<S>(a, b, |x, y| x.hypot_u35(y))
    }
);

simd_unsafe_generate_all!(
    fn simdeez_fmod_sum(a: &[f32], b: &[f32]) -> f32 {
        shared::simdeez_binary_sum_impl::<S>(a, b, |x, y| x.fmod(y))
    }
);

pub fn register(c: &mut Criterion) {
    let log10_inputs = shared::make_positive_inputs(INPUT_LEN, 0xDEADB004, 1.0e-20, 1.0e20);
    let (atan2_y, atan2_x) =
        shared::make_binary_inputs(INPUT_LEN, 0xDEADB005, -100.0..100.0, -100.0..100.0);
    let (hypot_x, hypot_y) =
        shared::make_binary_inputs(INPUT_LEN, 0xDEADB006, -1.0e20..1.0e20, -1.0e20..1.0e20);
    let (fmod_x, mut fmod_y) =
        shared::make_binary_inputs(INPUT_LEN, 0xDEADB007, -1000.0..1000.0, -1000.0..1000.0);
    for y in &mut fmod_y {
        if *y == 0.0 {
            *y = 1.0;
        }
    }

    shared::bench_unary(
        c,
        "simd_math_baseline/f32/log10_u35",
        &log10_inputs,
        scalar_log10_sum,
        simdeez_log10_sum,
    );

    shared::bench_binary(
        c,
        "simd_math_baseline/f32/atan2_u35",
        &atan2_y,
        &atan2_x,
        scalar_atan2_sum,
        simdeez_atan2_sum,
    );
    shared::bench_binary(
        c,
        "simd_math_baseline/f32/hypot_u35",
        &hypot_x,
        &hypot_y,
        scalar_hypot_sum,
        simdeez_hypot_sum,
    );
    shared::bench_binary(
        c,
        "simd_math_baseline/f32/fmod",
        &fmod_x,
        &fmod_y,
        scalar_fmod_sum,
        simdeez_fmod_sum,
    );
}
