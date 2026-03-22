use criterion::Criterion;
use simdeez::math::SimdMathF32Hyperbolic;
#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
use simdeez::scalar::Scalar;
use simdeez::{prelude::*, simd_unsafe_generate_all};

use crate::shared::{self, BenchTargets, INPUT_LEN};

#[inline(never)]
fn scalar_sinh_sum(input: &[f32]) -> f32 {
    input.iter().copied().map(f32::sinh).sum()
}

#[inline(never)]
fn scalar_cosh_sum(input: &[f32]) -> f32 {
    input.iter().copied().map(f32::cosh).sum()
}

#[inline(never)]
fn scalar_tanh_sum(input: &[f32]) -> f32 {
    input.iter().copied().map(f32::tanh).sum()
}

simd_unsafe_generate_all!(
    fn simdeez_sinh_sum(input: &[f32]) -> f32 {
        shared::simdeez_sum_impl::<S>(input, |v| v.sinh_u35())
    }
);

simd_unsafe_generate_all!(
    fn simdeez_cosh_sum(input: &[f32]) -> f32 {
        shared::simdeez_sum_impl::<S>(input, |v| v.cosh_u35())
    }
);

simd_unsafe_generate_all!(
    fn simdeez_tanh_sum(input: &[f32]) -> f32 {
        shared::simdeez_sum_impl::<S>(input, |v| v.tanh_u35())
    }
);

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
#[inline(never)]
fn simdeez_sinh_sum_scalar(input: &[f32]) -> f32 {
    shared::force_scalar_sum(input, |v: <Scalar as Simd>::Vf32| v.sinh_u35())
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
#[inline(never)]
fn simdeez_cosh_sum_scalar(input: &[f32]) -> f32 {
    shared::force_scalar_sum(input, |v: <Scalar as Simd>::Vf32| v.cosh_u35())
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
#[inline(never)]
fn simdeez_tanh_sum_scalar(input: &[f32]) -> f32 {
    shared::force_scalar_sum(input, |v: <Scalar as Simd>::Vf32| v.tanh_u35())
}

pub fn register(c: &mut Criterion) {
    let sinh_inputs = shared::make_unary_inputs(INPUT_LEN, 0xA11C_E006, -5.0..5.0);
    let cosh_inputs = shared::make_unary_inputs(INPUT_LEN, 0xA11C_E007, -5.0..5.0);
    let tanh_inputs = shared::make_unary_inputs(INPUT_LEN, 0xA11C_E008, -40.0..40.0);

    shared::bench_variants(
        c,
        "simd_math/f32/sinh_u35",
        &sinh_inputs,
        BenchTargets {
            scalar_native: scalar_sinh_sum,
            simdeez_runtime: simdeez_sinh_sum,
            simdeez_scalar: simdeez_sinh_sum_scalar,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse2: simdeez_sinh_sum_sse2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse41: simdeez_sinh_sum_sse41,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx2: simdeez_sinh_sum_avx2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx512: simdeez_sinh_sum_avx512,
        },
    );

    shared::bench_variants(
        c,
        "simd_math/f32/cosh_u35",
        &cosh_inputs,
        BenchTargets {
            scalar_native: scalar_cosh_sum,
            simdeez_runtime: simdeez_cosh_sum,
            simdeez_scalar: simdeez_cosh_sum_scalar,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse2: simdeez_cosh_sum_sse2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse41: simdeez_cosh_sum_sse41,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx2: simdeez_cosh_sum_avx2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx512: simdeez_cosh_sum_avx512,
        },
    );

    shared::bench_variants(
        c,
        "simd_math/f32/tanh_u35",
        &tanh_inputs,
        BenchTargets {
            scalar_native: scalar_tanh_sum,
            simdeez_runtime: simdeez_tanh_sum,
            simdeez_scalar: simdeez_tanh_sum_scalar,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse2: simdeez_tanh_sum_sse2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse41: simdeez_tanh_sum_sse41,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx2: simdeez_tanh_sum_avx2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx512: simdeez_tanh_sum_avx512,
        },
    );
}
