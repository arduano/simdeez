use criterion::Criterion;
use simdeez::math::SimdMathF32Core;
#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
use simdeez::scalar::Scalar;
use simdeez::{prelude::*, simd_unsafe_generate_all};

use crate::shared::{self, BenchTargets, INPUT_LEN};

#[inline(never)]
fn scalar_log2_sum(input: &[f32]) -> f32 {
    input.iter().copied().map(f32::log2).sum()
}

#[inline(never)]
fn scalar_exp2_sum(input: &[f32]) -> f32 {
    input.iter().copied().map(f32::exp2).sum()
}

#[inline(never)]
fn scalar_ln_sum(input: &[f32]) -> f32 {
    input.iter().copied().map(f32::ln).sum()
}

#[inline(never)]
fn scalar_exp_sum(input: &[f32]) -> f32 {
    input.iter().copied().map(f32::exp).sum()
}

simd_unsafe_generate_all!(
    fn simdeez_log2_sum(input: &[f32]) -> f32 {
        shared::simdeez_sum_impl::<S>(input, |v| v.log2_u35())
    }
);

simd_unsafe_generate_all!(
    fn simdeez_exp2_sum(input: &[f32]) -> f32 {
        shared::simdeez_sum_impl::<S>(input, |v| v.exp2_u35())
    }
);

simd_unsafe_generate_all!(
    fn simdeez_ln_sum(input: &[f32]) -> f32 {
        shared::simdeez_sum_impl::<S>(input, |v| v.ln_u35())
    }
);

simd_unsafe_generate_all!(
    fn simdeez_exp_sum(input: &[f32]) -> f32 {
        shared::simdeez_sum_impl::<S>(input, |v| v.exp_u35())
    }
);

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
#[inline(never)]
fn simdeez_log2_sum_scalar(input: &[f32]) -> f32 {
    shared::force_scalar_sum(input, |v: <Scalar as Simd>::Vf32| v.log2_u35())
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
#[inline(never)]
fn simdeez_exp2_sum_scalar(input: &[f32]) -> f32 {
    shared::force_scalar_sum(input, |v: <Scalar as Simd>::Vf32| v.exp2_u35())
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
#[inline(never)]
fn simdeez_ln_sum_scalar(input: &[f32]) -> f32 {
    shared::force_scalar_sum(input, |v: <Scalar as Simd>::Vf32| v.ln_u35())
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
#[inline(never)]
fn simdeez_exp_sum_scalar(input: &[f32]) -> f32 {
    shared::force_scalar_sum(input, |v: <Scalar as Simd>::Vf32| v.exp_u35())
}

pub fn register(c: &mut Criterion) {
    let log_inputs = shared::make_positive_log_inputs(INPUT_LEN, 0xA11C_E001);
    let exp2_inputs = shared::make_exp2_inputs(INPUT_LEN, 0xA11C_E002);
    let exp_inputs = shared::make_exp_inputs(INPUT_LEN, 0xA11C_E003);

    shared::bench_variants(
        c,
        "simd_math/f32/log2_u35",
        &log_inputs,
        BenchTargets {
            scalar_native: scalar_log2_sum,
            simdeez_runtime: simdeez_log2_sum,
            simdeez_scalar: simdeez_log2_sum_scalar,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse2: simdeez_log2_sum_sse2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse41: simdeez_log2_sum_sse41,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx2: simdeez_log2_sum_avx2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx512: simdeez_log2_sum_avx512,
        },
    );

    shared::bench_variants(
        c,
        "simd_math/f32/exp2_u35",
        &exp2_inputs,
        BenchTargets {
            scalar_native: scalar_exp2_sum,
            simdeez_runtime: simdeez_exp2_sum,
            simdeez_scalar: simdeez_exp2_sum_scalar,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse2: simdeez_exp2_sum_sse2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse41: simdeez_exp2_sum_sse41,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx2: simdeez_exp2_sum_avx2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx512: simdeez_exp2_sum_avx512,
        },
    );

    shared::bench_variants(
        c,
        "simd_math/f32/ln_u35",
        &log_inputs,
        BenchTargets {
            scalar_native: scalar_ln_sum,
            simdeez_runtime: simdeez_ln_sum,
            simdeez_scalar: simdeez_ln_sum_scalar,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse2: simdeez_ln_sum_sse2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse41: simdeez_ln_sum_sse41,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx2: simdeez_ln_sum_avx2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx512: simdeez_ln_sum_avx512,
        },
    );

    shared::bench_variants(
        c,
        "simd_math/f32/exp_u35",
        &exp_inputs,
        BenchTargets {
            scalar_native: scalar_exp_sum,
            simdeez_runtime: simdeez_exp_sum,
            simdeez_scalar: simdeez_exp_sum_scalar,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse2: simdeez_exp_sum_sse2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse41: simdeez_exp_sum_sse41,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx2: simdeez_exp_sum_avx2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx512: simdeez_exp_sum_avx512,
        },
    );
}
