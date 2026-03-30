use crate::shared::{self, BenchTargets, BenchTargetsF64, INPUT_LEN};
use criterion::Criterion;
use simdeez::math::{SimdMathF32Hyperbolic, SimdMathF64Hyperbolic};
use simdeez::scalar::Scalar;
use simdeez::{prelude::*, simd_unsafe_generate_all};

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

#[inline(never)]
fn scalar_sinh_sum_f64(input: &[f64]) -> f64 {
    input.iter().copied().map(f64::sinh).sum()
}

#[inline(never)]
fn scalar_cosh_sum_f64(input: &[f64]) -> f64 {
    input.iter().copied().map(f64::cosh).sum()
}

#[inline(never)]
fn scalar_tanh_sum_f64(input: &[f64]) -> f64 {
    input.iter().copied().map(f64::tanh).sum()
}

simd_unsafe_generate_all!(
    fn simdeez_sinh_sum_f64(input: &[f64]) -> f64 {
        simdeez_sum_impl_f64::<S>(input, |v| v.sinh_u35())
    }
);

simd_unsafe_generate_all!(
    fn simdeez_cosh_sum_f64(input: &[f64]) -> f64 {
        simdeez_sum_impl_f64::<S>(input, |v| v.cosh_u35())
    }
);

simd_unsafe_generate_all!(
    fn simdeez_tanh_sum_f64(input: &[f64]) -> f64 {
        simdeez_sum_impl_f64::<S>(input, |v| v.tanh_u35())
    }
);

#[inline(never)]
fn forced_scalar_sinh_sum(input: &[f32]) -> f32 {
    shared::force_scalar_sum(input, |v: <Scalar as Simd>::Vf32| v.sinh_u35())
}

#[inline(never)]
fn forced_scalar_cosh_sum(input: &[f32]) -> f32 {
    shared::force_scalar_sum(input, |v: <Scalar as Simd>::Vf32| v.cosh_u35())
}

#[inline(never)]
fn forced_scalar_tanh_sum(input: &[f32]) -> f32 {
    shared::force_scalar_sum(input, |v: <Scalar as Simd>::Vf32| v.tanh_u35())
}

#[inline(never)]
fn forced_scalar_sinh_sum_f64(input: &[f64]) -> f64 {
    shared::force_scalar_sum_f64(input, |v: <Scalar as Simd>::Vf64| v.sinh_u35())
}

#[inline(never)]
fn forced_scalar_cosh_sum_f64(input: &[f64]) -> f64 {
    shared::force_scalar_sum_f64(input, |v: <Scalar as Simd>::Vf64| v.cosh_u35())
}

#[inline(never)]
fn forced_scalar_tanh_sum_f64(input: &[f64]) -> f64 {
    shared::force_scalar_sum_f64(input, |v: <Scalar as Simd>::Vf64| v.tanh_u35())
}

#[inline(always)]
fn simdeez_sum_impl_f64<S: Simd>(input: &[f64], op: impl Fn(S::Vf64) -> S::Vf64) -> f64 {
    let mut sum = 0.0f64;
    let mut i = 0;

    while i + S::Vf64::WIDTH <= input.len() {
        let v = S::Vf64::load_from_slice(&input[i..]);
        sum += op(v).horizontal_add();
        i += S::Vf64::WIDTH;
    }

    sum
}

fn make_unary_inputs_f64(len: usize, seed: u64, range: core::ops::Range<f64>) -> Vec<f64> {
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;

    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    (0..len).map(|_| rng.gen_range(range.clone())).collect()
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
            simdeez_scalar: forced_scalar_sinh_sum,
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
            simdeez_scalar: forced_scalar_cosh_sum,
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
            simdeez_scalar: forced_scalar_tanh_sum,
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

    let sinh_inputs_f64 = make_unary_inputs_f64(INPUT_LEN, 0xA11C_E106, -5.0..5.0);
    let cosh_inputs_f64 = make_unary_inputs_f64(INPUT_LEN, 0xA11C_E107, -5.0..5.0);
    let tanh_inputs_f64 = make_unary_inputs_f64(INPUT_LEN, 0xA11C_E108, -20.0..20.0);

    shared::bench_variants_f64(
        c,
        "simd_math/f64/sinh_u35",
        &sinh_inputs_f64,
        BenchTargetsF64 {
            scalar_native: scalar_sinh_sum_f64,
            simdeez_runtime: simdeez_sinh_sum_f64,
            simdeez_scalar: forced_scalar_sinh_sum_f64,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse2: simdeez_sinh_sum_f64_sse2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse41: simdeez_sinh_sum_f64_sse41,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx2: simdeez_sinh_sum_f64_avx2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx512: simdeez_sinh_sum_f64_avx512,
        },
    );
    shared::bench_variants_f64(
        c,
        "simd_math/f64/cosh_u35",
        &cosh_inputs_f64,
        BenchTargetsF64 {
            scalar_native: scalar_cosh_sum_f64,
            simdeez_runtime: simdeez_cosh_sum_f64,
            simdeez_scalar: forced_scalar_cosh_sum_f64,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse2: simdeez_cosh_sum_f64_sse2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse41: simdeez_cosh_sum_f64_sse41,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx2: simdeez_cosh_sum_f64_avx2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx512: simdeez_cosh_sum_f64_avx512,
        },
    );
    shared::bench_variants_f64(
        c,
        "simd_math/f64/tanh_u35",
        &tanh_inputs_f64,
        BenchTargetsF64 {
            scalar_native: scalar_tanh_sum_f64,
            simdeez_runtime: simdeez_tanh_sum_f64,
            simdeez_scalar: forced_scalar_tanh_sum_f64,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse2: simdeez_tanh_sum_f64_sse2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse41: simdeez_tanh_sum_f64_sse41,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx2: simdeez_tanh_sum_f64_avx2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx512: simdeez_tanh_sum_f64_avx512,
        },
    );
}
