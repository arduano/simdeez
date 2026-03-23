use criterion::Criterion;
use simdeez::math::SimdMathF32InverseTrig;
use simdeez::scalar::Scalar;
use simdeez::{prelude::*, simd_unsafe_generate_all};

use crate::shared::{self, BenchTargets, INPUT_LEN};

#[inline(never)]
fn scalar_asin_sum(input: &[f32]) -> f32 {
    input.iter().copied().map(f32::asin).sum()
}

simd_unsafe_generate_all!(
    fn simdeez_asin_sum(input: &[f32]) -> f32 {
        shared::simdeez_unary_sum_impl::<S>(input, |v| v.asin_u35())
    }
);

#[inline(never)]
fn forced_scalar_asin_sum(input: &[f32]) -> f32 {
    shared::force_scalar_sum(input, |v: <Scalar as Simd>::Vf32| v.asin_u35())
}

pub fn register(c: &mut Criterion) {
    let asin_inputs = shared::make_unary_inputs(INPUT_LEN, 0xDEADB001, -1.0..1.0);

    shared::bench_unary_variants(
        c,
        "simd_math_baseline/f32/asin_u35",
        &asin_inputs,
        BenchTargets {
            scalar_native: scalar_asin_sum,
            simdeez_runtime: simdeez_asin_sum,
            simdeez_scalar: forced_scalar_asin_sum,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse2: simdeez_asin_sum_sse2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse41: simdeez_asin_sum_sse41,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx2: simdeez_asin_sum_avx2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx512: simdeez_asin_sum_avx512,
        },
    );
}
