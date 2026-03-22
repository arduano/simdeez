use criterion::Criterion;
use simdeez::math::SimdMathF32InverseTrig;
#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
use simdeez::scalar::Scalar;
use simdeez::{prelude::*, simd_unsafe_generate_all};

use crate::shared::{self, BenchTargets, INPUT_LEN};

#[inline(never)]
fn scalar_asin_sum(input: &[f32]) -> f32 {
    input.iter().copied().map(f32::asin).sum()
}

#[inline(never)]
fn scalar_acos_sum(input: &[f32]) -> f32 {
    input.iter().copied().map(f32::acos).sum()
}

#[inline(never)]
fn scalar_atan_sum(input: &[f32]) -> f32 {
    input.iter().copied().map(f32::atan).sum()
}

simd_unsafe_generate_all!(
    fn simdeez_asin_sum(input: &[f32]) -> f32 {
        shared::simdeez_sum_impl::<S>(input, |v| v.asin_u35())
    }
);

simd_unsafe_generate_all!(
    fn simdeez_acos_sum(input: &[f32]) -> f32 {
        shared::simdeez_sum_impl::<S>(input, |v| v.acos_u35())
    }
);

simd_unsafe_generate_all!(
    fn simdeez_atan_sum(input: &[f32]) -> f32 {
        shared::simdeez_sum_impl::<S>(input, |v| v.atan_u35())
    }
);

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
#[inline(never)]
fn simdeez_asin_sum_scalar(input: &[f32]) -> f32 {
    shared::force_scalar_sum(input, |v: <Scalar as Simd>::Vf32| v.asin_u35())
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
#[inline(never)]
fn simdeez_acos_sum_scalar(input: &[f32]) -> f32 {
    shared::force_scalar_sum(input, |v: <Scalar as Simd>::Vf32| v.acos_u35())
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
#[inline(never)]
fn simdeez_atan_sum_scalar(input: &[f32]) -> f32 {
    shared::force_scalar_sum(input, |v: <Scalar as Simd>::Vf32| v.atan_u35())
}

pub fn register(c: &mut Criterion) {
    let inverse_inputs = shared::make_inverse_trig_inputs(INPUT_LEN, 0xA11C_E101);
    let atan_inputs = shared::make_atan_inputs(INPUT_LEN, 0xA11C_E102);

    shared::bench_variants(
        c,
        "simd_math/f32/asin_u35",
        &inverse_inputs,
        BenchTargets {
            scalar_native: scalar_asin_sum,
            simdeez_runtime: simdeez_asin_sum,
            simdeez_scalar: simdeez_asin_sum_scalar,
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

    shared::bench_variants(
        c,
        "simd_math/f32/acos_u35",
        &inverse_inputs,
        BenchTargets {
            scalar_native: scalar_acos_sum,
            simdeez_runtime: simdeez_acos_sum,
            simdeez_scalar: simdeez_acos_sum_scalar,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse2: simdeez_acos_sum_sse2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse41: simdeez_acos_sum_sse41,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx2: simdeez_acos_sum_avx2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx512: simdeez_acos_sum_avx512,
        },
    );

    shared::bench_variants(
        c,
        "simd_math/f32/atan_u35",
        &atan_inputs,
        BenchTargets {
            scalar_native: scalar_atan_sum,
            simdeez_runtime: simdeez_atan_sum,
            simdeez_scalar: simdeez_atan_sum_scalar,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse2: simdeez_atan_sum_sse2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse41: simdeez_atan_sum_sse41,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx2: simdeez_atan_sum_avx2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx512: simdeez_atan_sum_avx512,
        },
    );
}
