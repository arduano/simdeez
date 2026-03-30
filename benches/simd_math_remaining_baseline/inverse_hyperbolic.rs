use criterion::Criterion;
use simdeez::math::{SimdMathF32InverseHyperbolic, SimdMathF64InverseHyperbolic};
use simdeez::scalar::Scalar;
use simdeez::{prelude::*, simd_unsafe_generate_all};

use crate::shared::{self, BenchTargets, BenchTargetsF64, INPUT_LEN};

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

#[inline(never)]
fn scalar_asinh_sum_f64(input: &[f64]) -> f64 {
    input.iter().copied().map(f64::asinh).sum()
}

#[inline(never)]
fn scalar_acosh_sum_f64(input: &[f64]) -> f64 {
    input.iter().copied().map(f64::acosh).sum()
}

#[inline(never)]
fn scalar_atanh_sum_f64(input: &[f64]) -> f64 {
    input.iter().copied().map(f64::atanh).sum()
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

simd_unsafe_generate_all!(
    fn simdeez_asinh_sum_f64(input: &[f64]) -> f64 {
        shared::simdeez_unary_sum_impl_f64::<S>(input, |v| v.asinh_u35())
    }
);

simd_unsafe_generate_all!(
    fn simdeez_acosh_sum_f64(input: &[f64]) -> f64 {
        shared::simdeez_unary_sum_impl_f64::<S>(input, |v| v.acosh_u35())
    }
);

simd_unsafe_generate_all!(
    fn simdeez_atanh_sum_f64(input: &[f64]) -> f64 {
        shared::simdeez_unary_sum_impl_f64::<S>(input, |v| v.atanh_u35())
    }
);

#[inline(never)]
fn forced_scalar_asinh_sum(input: &[f32]) -> f32 {
    shared::force_scalar_sum(input, |v: <Scalar as Simd>::Vf32| v.asinh_u35())
}

#[inline(never)]
fn forced_scalar_acosh_sum(input: &[f32]) -> f32 {
    shared::force_scalar_sum(input, |v: <Scalar as Simd>::Vf32| v.acosh_u35())
}

#[inline(never)]
fn forced_scalar_atanh_sum(input: &[f32]) -> f32 {
    shared::force_scalar_sum(input, |v: <Scalar as Simd>::Vf32| v.atanh_u35())
}

#[inline(never)]
fn forced_scalar_asinh_sum_f64(input: &[f64]) -> f64 {
    shared::force_scalar_sum_f64(input, |v: <Scalar as Simd>::Vf64| v.asinh_u35())
}

#[inline(never)]
fn forced_scalar_acosh_sum_f64(input: &[f64]) -> f64 {
    shared::force_scalar_sum_f64(input, |v: <Scalar as Simd>::Vf64| v.acosh_u35())
}

#[inline(never)]
fn forced_scalar_atanh_sum_f64(input: &[f64]) -> f64 {
    shared::force_scalar_sum_f64(input, |v: <Scalar as Simd>::Vf64| v.atanh_u35())
}

pub fn register(c: &mut Criterion) {
    let asinh_inputs = shared::make_unary_inputs(INPUT_LEN, 0xDEADB001, -16_384.0..16_384.0);
    let acosh_inputs = shared::make_positive_inputs(INPUT_LEN, 0xDEADB002, 1.0, 16_384.0);
    let atanh_inputs = shared::make_unary_inputs(INPUT_LEN, 0xDEADB003, -0.999_999..0.999_999);

    let asinh_inputs_f64 =
        shared::make_unary_inputs_f64(INPUT_LEN, 0xDEADB101, -16_384.0..16_384.0);
    let acosh_inputs_f64 = shared::make_positive_inputs_f64(INPUT_LEN, 0xDEADB102, 1.0, 16_384.0);
    let atanh_inputs_f64 =
        shared::make_unary_inputs_f64(INPUT_LEN, 0xDEADB103, -0.999_999_999_999..0.999_999_999_999);

    shared::bench_unary_variants(
        c,
        "simd_math_baseline/f32/asinh_u35",
        &asinh_inputs,
        BenchTargets {
            scalar_native: scalar_asinh_sum,
            simdeez_runtime: simdeez_asinh_sum,
            simdeez_scalar: forced_scalar_asinh_sum,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse2: simdeez_asinh_sum_sse2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse41: simdeez_asinh_sum_sse41,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx2: simdeez_asinh_sum_avx2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx512: simdeez_asinh_sum_avx512,
        },
    );

    shared::bench_unary_variants(
        c,
        "simd_math_baseline/f32/acosh_u35",
        &acosh_inputs,
        BenchTargets {
            scalar_native: scalar_acosh_sum,
            simdeez_runtime: simdeez_acosh_sum,
            simdeez_scalar: forced_scalar_acosh_sum,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse2: simdeez_acosh_sum_sse2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse41: simdeez_acosh_sum_sse41,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx2: simdeez_acosh_sum_avx2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx512: simdeez_acosh_sum_avx512,
        },
    );

    shared::bench_unary_variants(
        c,
        "simd_math_baseline/f32/atanh_u35",
        &atanh_inputs,
        BenchTargets {
            scalar_native: scalar_atanh_sum,
            simdeez_runtime: simdeez_atanh_sum,
            simdeez_scalar: forced_scalar_atanh_sum,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse2: simdeez_atanh_sum_sse2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse41: simdeez_atanh_sum_sse41,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx2: simdeez_atanh_sum_avx2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx512: simdeez_atanh_sum_avx512,
        },
    );

    shared::bench_unary_variants_f64(
        c,
        "simd_math_baseline/f64/asinh_u35",
        &asinh_inputs_f64,
        BenchTargetsF64 {
            scalar_native: scalar_asinh_sum_f64,
            simdeez_runtime: simdeez_asinh_sum_f64,
            simdeez_scalar: forced_scalar_asinh_sum_f64,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse2: simdeez_asinh_sum_f64_sse2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse41: simdeez_asinh_sum_f64_sse41,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx2: simdeez_asinh_sum_f64_avx2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx512: simdeez_asinh_sum_f64_avx512,
        },
    );

    shared::bench_unary_variants_f64(
        c,
        "simd_math_baseline/f64/acosh_u35",
        &acosh_inputs_f64,
        BenchTargetsF64 {
            scalar_native: scalar_acosh_sum_f64,
            simdeez_runtime: simdeez_acosh_sum_f64,
            simdeez_scalar: forced_scalar_acosh_sum_f64,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse2: simdeez_acosh_sum_f64_sse2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse41: simdeez_acosh_sum_f64_sse41,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx2: simdeez_acosh_sum_f64_avx2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx512: simdeez_acosh_sum_f64_avx512,
        },
    );

    shared::bench_unary_variants_f64(
        c,
        "simd_math_baseline/f64/atanh_u35",
        &atanh_inputs_f64,
        BenchTargetsF64 {
            scalar_native: scalar_atanh_sum_f64,
            simdeez_runtime: simdeez_atanh_sum_f64,
            simdeez_scalar: forced_scalar_atanh_sum_f64,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse2: simdeez_atanh_sum_f64_sse2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse41: simdeez_atanh_sum_f64_sse41,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx2: simdeez_atanh_sum_f64_avx2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx512: simdeez_atanh_sum_f64_avx512,
        },
    );
}
