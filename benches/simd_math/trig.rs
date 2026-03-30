use criterion::Criterion;
use simdeez::math::SimdMathF32Core;
#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
use simdeez::scalar::Scalar;
use simdeez::{prelude::*, simd_unsafe_generate_all};

use crate::shared::{self, BenchTargets, INPUT_LEN};

#[inline(never)]
fn scalar_sin_sum(input: &[f32]) -> f32 {
    input.iter().copied().map(f32::sin).sum()
}

#[inline(never)]
fn scalar_cos_sum(input: &[f32]) -> f32 {
    input.iter().copied().map(f32::cos).sum()
}

#[inline(never)]
fn scalar_tan_sum(input: &[f32]) -> f32 {
    input.iter().copied().map(f32::tan).sum()
}

simd_unsafe_generate_all!(
    fn simdeez_sin_sum(input: &[f32]) -> f32 {
        shared::simdeez_sum_impl::<S>(input, |v| v.sin_u35())
    }
);

simd_unsafe_generate_all!(
    fn simdeez_cos_sum(input: &[f32]) -> f32 {
        shared::simdeez_sum_impl::<S>(input, |v| v.cos_u35())
    }
);

simd_unsafe_generate_all!(
    fn simdeez_tan_sum(input: &[f32]) -> f32 {
        shared::simdeez_sum_impl::<S>(input, |v| v.tan_u35())
    }
);

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
#[inline(never)]
fn simdeez_sin_sum_scalar(input: &[f32]) -> f32 {
    shared::force_scalar_sum(input, |v: <Scalar as Simd>::Vf32| v.sin_u35())
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
#[inline(never)]
fn simdeez_cos_sum_scalar(input: &[f32]) -> f32 {
    shared::force_scalar_sum(input, |v: <Scalar as Simd>::Vf32| v.cos_u35())
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
#[inline(never)]
fn simdeez_tan_sum_scalar(input: &[f32]) -> f32 {
    shared::force_scalar_sum(input, |v: <Scalar as Simd>::Vf32| v.tan_u35())
}

pub fn register(c: &mut Criterion) {
    let trig_inputs = shared::make_trig_inputs(INPUT_LEN, 0xA11C_E004);
    let tan_inputs = shared::make_tan_inputs(INPUT_LEN, 0xA11C_E005);

    shared::bench_variants(
        c,
        "simd_math/f32/sin_u35",
        &trig_inputs,
        BenchTargets {
            scalar_native: scalar_sin_sum,
            simdeez_runtime: simdeez_sin_sum,
            simdeez_scalar: simdeez_sin_sum_scalar,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse2: simdeez_sin_sum_sse2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse41: simdeez_sin_sum_sse41,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx2: simdeez_sin_sum_avx2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx512: simdeez_sin_sum_avx512,
        },
    );

    shared::bench_variants(
        c,
        "simd_math/f32/cos_u35",
        &trig_inputs,
        BenchTargets {
            scalar_native: scalar_cos_sum,
            simdeez_runtime: simdeez_cos_sum,
            simdeez_scalar: simdeez_cos_sum_scalar,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse2: simdeez_cos_sum_sse2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse41: simdeez_cos_sum_sse41,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx2: simdeez_cos_sum_avx2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx512: simdeez_cos_sum_avx512,
        },
    );

    shared::bench_variants(
        c,
        "simd_math/f32/tan_u35",
        &tan_inputs,
        BenchTargets {
            scalar_native: scalar_tan_sum,
            simdeez_runtime: simdeez_tan_sum,
            simdeez_scalar: simdeez_tan_sum_scalar,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse2: simdeez_tan_sum_sse2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse41: simdeez_tan_sum_sse41,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx2: simdeez_tan_sum_avx2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx512: simdeez_tan_sum_avx512,
        },
    );
}
