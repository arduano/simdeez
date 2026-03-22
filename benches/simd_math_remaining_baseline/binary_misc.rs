use criterion::Criterion;
use simdeez::math::{SimdMathF32BinaryMisc, SimdMathF64BinaryMisc};
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

#[inline(never)]
fn scalar_log10_sum_f64(input: &[f64]) -> f64 {
    input.iter().copied().map(f64::log10).sum()
}

#[inline(never)]
fn scalar_atan2_sum_f64(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(&x, &y)| x.atan2(y)).sum()
}

#[inline(never)]
fn scalar_fmod_sum_f64(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(&x, &y)| x % y).sum()
}

#[inline(never)]
fn scalar_hypot_sum_f64(a: &[f64], b: &[f64]) -> f64 {
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

simd_unsafe_generate_all!(
    fn simdeez_log10_sum_f64(input: &[f64]) -> f64 {
        shared::simdeez_unary_sum_impl_f64::<S>(input, |v| v.log10_u35())
    }
);

simd_unsafe_generate_all!(
    fn simdeez_atan2_sum_f64(a: &[f64], b: &[f64]) -> f64 {
        shared::simdeez_binary_sum_impl_f64::<S>(a, b, |x, y| x.atan2_u35(y))
    }
);

simd_unsafe_generate_all!(
    fn simdeez_hypot_sum_f64(a: &[f64], b: &[f64]) -> f64 {
        shared::simdeez_binary_sum_impl_f64::<S>(a, b, |x, y| x.hypot_u35(y))
    }
);

simd_unsafe_generate_all!(
    fn simdeez_fmod_sum_f64(a: &[f64], b: &[f64]) -> f64 {
        shared::simdeez_binary_sum_impl_f64::<S>(a, b, |x, y| x.fmod(y))
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
    let log10_inputs_f64 =
        shared::make_positive_inputs_f64(INPUT_LEN, 0xDEADB104, 1.0e-200, 1.0e200);
    let (atan2_y_f64, atan2_x_f64) =
        shared::make_binary_inputs_f64(INPUT_LEN, 0xDEADB105, -100.0..100.0, -100.0..100.0);
    let (hypot_x_f64, hypot_y_f64) =
        shared::make_binary_inputs_f64(INPUT_LEN, 0xDEADB106, -1.0e200..1.0e200, -1.0e200..1.0e200);
    let (fmod_x_f64, mut fmod_y_f64) =
        shared::make_binary_inputs_f64(INPUT_LEN, 0xDEADB107, -1000.0..1000.0, -1000.0..1000.0);
    for y in &mut fmod_y_f64 {
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

    shared::bench_unary_f64(
        c,
        "simd_math_baseline/f64/log10_u35",
        &log10_inputs_f64,
        scalar_log10_sum_f64,
        simdeez_log10_sum_f64,
    );
    shared::bench_binary_f64(
        c,
        "simd_math_baseline/f64/atan2_u35",
        &atan2_y_f64,
        &atan2_x_f64,
        scalar_atan2_sum_f64,
        simdeez_atan2_sum_f64,
    );
    shared::bench_binary_f64(
        c,
        "simd_math_baseline/f64/hypot_u35",
        &hypot_x_f64,
        &hypot_y_f64,
        scalar_hypot_sum_f64,
        simdeez_hypot_sum_f64,
    );
    shared::bench_binary_f64(
        c,
        "simd_math_baseline/f64/fmod",
        &fmod_x_f64,
        &fmod_y_f64,
        scalar_fmod_sum_f64,
        simdeez_fmod_sum_f64,
    );
}
