use criterion::Criterion;
use simdeez::math::SimdMathF32Hyperbolic;
use simdeez::{prelude::*, simd_unsafe_generate_all};

use crate::shared::{self, INPUT_LEN};

#[inline(never)]
fn scalar_tanh_sum(input: &[f32]) -> f32 {
    input.iter().copied().map(f32::tanh).sum()
}

simd_unsafe_generate_all!(
    fn simdeez_tanh_sum(input: &[f32]) -> f32 {
        shared::simdeez_unary_sum_impl::<S>(input, |v| v.tanh_u35())
    }
);

pub fn register(c: &mut Criterion) {
    let tanh_inputs = shared::make_unary_inputs(INPUT_LEN, 0xDEADB002, -40.0..40.0);

    shared::bench_unary(
        c,
        "simd_math_baseline/f32/tanh_u35",
        &tanh_inputs,
        scalar_tanh_sum,
        simdeez_tanh_sum,
    );
}
