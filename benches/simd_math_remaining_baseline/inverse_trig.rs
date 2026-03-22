use criterion::Criterion;
use simdeez::math::SimdMathF32InverseTrig;
use simdeez::{prelude::*, simd_unsafe_generate_all};

use crate::shared::{self, INPUT_LEN};

#[inline(never)]
fn scalar_asin_sum(input: &[f32]) -> f32 {
    input.iter().copied().map(f32::asin).sum()
}

simd_unsafe_generate_all!(
    fn simdeez_asin_sum(input: &[f32]) -> f32 {
        shared::simdeez_unary_sum_impl::<S>(input, |v| v.asin_u35())
    }
);

pub fn register(c: &mut Criterion) {
    let asin_inputs = shared::make_unary_inputs(INPUT_LEN, 0xDEADB001, -1.0..1.0);

    shared::bench_unary(
        c,
        "simd_math_baseline/f32/asin_u35",
        &asin_inputs,
        scalar_asin_sum,
        simdeez_asin_sum,
    );
}
