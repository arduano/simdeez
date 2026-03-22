use criterion::Criterion;
use simdeez::math::{SimdMathF32InverseTrig, SimdMathF64InverseTrig};
use simdeez::scalar::Scalar;
use simdeez::{prelude::*, simd_unsafe_generate_all};

use crate::shared::{self, BenchTargets, BenchTargetsF64, INPUT_LEN};

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

#[inline(never)]
fn scalar_asin_sum_f64(input: &[f64]) -> f64 {
    input.iter().copied().map(f64::asin).sum()
}

#[inline(never)]
fn scalar_acos_sum_f64(input: &[f64]) -> f64 {
    input.iter().copied().map(f64::acos).sum()
}

#[inline(never)]
fn scalar_atan_sum_f64(input: &[f64]) -> f64 {
    input.iter().copied().map(f64::atan).sum()
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

simd_unsafe_generate_all!(
    fn simdeez_asin_sum_f64(input: &[f64]) -> f64 {
        shared::simdeez_sum_impl_f64::<S>(input, |v| v.asin_u35())
    }
);

simd_unsafe_generate_all!(
    fn simdeez_acos_sum_f64(input: &[f64]) -> f64 {
        shared::simdeez_sum_impl_f64::<S>(input, |v| v.acos_u35())
    }
);

simd_unsafe_generate_all!(
    fn simdeez_atan_sum_f64(input: &[f64]) -> f64 {
        shared::simdeez_sum_impl_f64::<S>(input, |v| v.atan_u35())
    }
);

#[inline(never)]
fn forced_scalar_asin_sum(input: &[f32]) -> f32 {
    shared::force_scalar_sum(input, |v: <Scalar as Simd>::Vf32| v.asin_u35())
}

#[inline(never)]
fn forced_scalar_acos_sum(input: &[f32]) -> f32 {
    shared::force_scalar_sum(input, |v: <Scalar as Simd>::Vf32| v.acos_u35())
}

#[inline(never)]
fn forced_scalar_atan_sum(input: &[f32]) -> f32 {
    shared::force_scalar_sum(input, |v: <Scalar as Simd>::Vf32| v.atan_u35())
}

#[inline(never)]
fn forced_scalar_asin_sum_f64(input: &[f64]) -> f64 {
    shared::force_scalar_sum_f64(input, |v: <Scalar as Simd>::Vf64| v.asin_u35())
}

#[inline(never)]
fn forced_scalar_acos_sum_f64(input: &[f64]) -> f64 {
    shared::force_scalar_sum_f64(input, |v: <Scalar as Simd>::Vf64| v.acos_u35())
}

#[inline(never)]
fn forced_scalar_atan_sum_f64(input: &[f64]) -> f64 {
    shared::force_scalar_sum_f64(input, |v: <Scalar as Simd>::Vf64| v.atan_u35())
}

pub fn register(c: &mut Criterion) {
    let inverse_inputs = shared::make_inverse_trig_inputs(INPUT_LEN, 0xA11C_E101);
    let atan_inputs = shared::make_atan_inputs(INPUT_LEN, 0xA11C_E102);
    let inverse_inputs_f64 = shared::make_inverse_trig_inputs_f64(INPUT_LEN, 0xA11C_E201);
    let atan_inputs_f64 = shared::make_atan_inputs_f64(INPUT_LEN, 0xA11C_E202);

    shared::bench_variants(
        c,
        "simd_math/f32/asin_u35",
        &inverse_inputs,
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

    shared::bench_variants(
        c,
        "simd_math/f32/acos_u35",
        &inverse_inputs,
        BenchTargets {
            scalar_native: scalar_acos_sum,
            simdeez_runtime: simdeez_acos_sum,
            simdeez_scalar: forced_scalar_acos_sum,
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
            simdeez_scalar: forced_scalar_atan_sum,
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

    shared::bench_variants_f64(
        c,
        "simd_math/f64/asin_u35",
        &inverse_inputs_f64,
        BenchTargetsF64 {
            scalar_native: scalar_asin_sum_f64,
            simdeez_runtime: simdeez_asin_sum_f64,
            simdeez_scalar: forced_scalar_asin_sum_f64,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse2: simdeez_asin_sum_f64_sse2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse41: simdeez_asin_sum_f64_sse41,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx2: simdeez_asin_sum_f64_avx2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx512: simdeez_asin_sum_f64_avx512,
        },
    );

    shared::bench_variants_f64(
        c,
        "simd_math/f64/acos_u35",
        &inverse_inputs_f64,
        BenchTargetsF64 {
            scalar_native: scalar_acos_sum_f64,
            simdeez_runtime: simdeez_acos_sum_f64,
            simdeez_scalar: forced_scalar_acos_sum_f64,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse2: simdeez_acos_sum_f64_sse2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse41: simdeez_acos_sum_f64_sse41,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx2: simdeez_acos_sum_f64_avx2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx512: simdeez_acos_sum_f64_avx512,
        },
    );

    shared::bench_variants_f64(
        c,
        "simd_math/f64/atan_u35",
        &atan_inputs_f64,
        BenchTargetsF64 {
            scalar_native: scalar_atan_sum_f64,
            simdeez_runtime: simdeez_atan_sum_f64,
            simdeez_scalar: forced_scalar_atan_sum_f64,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse2: simdeez_atan_sum_f64_sse2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse41: simdeez_atan_sum_f64_sse41,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx2: simdeez_atan_sum_f64_avx2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx512: simdeez_atan_sum_f64_avx512,
        },
    );
}
