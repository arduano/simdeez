use criterion::{Criterion, Throughput};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use simdeez::prelude::*;
use simdeez::scalar::Scalar;
use std::hint::black_box;

pub const INPUT_LEN: usize = 1 << 20;

pub fn make_unary_inputs(len: usize, seed: u64, range: core::ops::Range<f32>) -> Vec<f32> {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    (0..len).map(|_| rng.gen_range(range.clone())).collect()
}

pub fn make_positive_inputs(len: usize, seed: u64, min: f32, max: f32) -> Vec<f32> {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    (0..len).map(|_| rng.gen_range(min..max)).collect()
}

pub fn make_unary_inputs_f64(len: usize, seed: u64, range: core::ops::Range<f64>) -> Vec<f64> {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    (0..len).map(|_| rng.gen_range(range.clone())).collect()
}

pub fn make_positive_inputs_f64(len: usize, seed: u64, min: f64, max: f64) -> Vec<f64> {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    (0..len).map(|_| rng.gen_range(min..max)).collect()
}

pub fn make_binary_inputs(
    len: usize,
    seed: u64,
    range_a: core::ops::Range<f32>,
    range_b: core::ops::Range<f32>,
) -> (Vec<f32>, Vec<f32>) {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let a = (0..len).map(|_| rng.gen_range(range_a.clone())).collect();
    let b = (0..len).map(|_| rng.gen_range(range_b.clone())).collect();
    (a, b)
}

pub fn make_binary_inputs_f64(
    len: usize,
    seed: u64,
    range_a: core::ops::Range<f64>,
    range_b: core::ops::Range<f64>,
) -> (Vec<f64>, Vec<f64>) {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let a = (0..len).map(|_| rng.gen_range(range_a.clone())).collect();
    let b = (0..len).map(|_| rng.gen_range(range_b.clone())).collect();
    (a, b)
}

#[inline(always)]
pub fn simdeez_unary_sum_impl<S: Simd>(input: &[f32], op: impl Fn(S::Vf32) -> S::Vf32) -> f32 {
    let mut sum = 0.0f32;
    let mut i = 0;
    while i + S::Vf32::WIDTH <= input.len() {
        let v = S::Vf32::load_from_slice(&input[i..]);
        sum += op(v).horizontal_add();
        i += S::Vf32::WIDTH;
    }
    sum
}

#[inline(always)]
pub fn simdeez_unary_sum_impl_f64<S: Simd>(input: &[f64], op: impl Fn(S::Vf64) -> S::Vf64) -> f64 {
    let mut sum = 0.0f64;
    let mut i = 0;
    while i + S::Vf64::WIDTH <= input.len() {
        let v = S::Vf64::load_from_slice(&input[i..]);
        sum += op(v).horizontal_add();
        i += S::Vf64::WIDTH;
    }
    sum
}

#[inline(always)]
pub fn simdeez_binary_sum_impl<S: Simd>(
    a: &[f32],
    b: &[f32],
    op: impl Fn(S::Vf32, S::Vf32) -> S::Vf32,
) -> f32 {
    let mut sum = 0.0f32;
    let mut i = 0;
    while i + S::Vf32::WIDTH <= a.len() {
        let va = S::Vf32::load_from_slice(&a[i..]);
        let vb = S::Vf32::load_from_slice(&b[i..]);
        sum += op(va, vb).horizontal_add();
        i += S::Vf32::WIDTH;
    }
    sum
}

#[inline(always)]
pub fn simdeez_binary_sum_impl_f64<S: Simd>(
    a: &[f64],
    b: &[f64],
    op: impl Fn(S::Vf64, S::Vf64) -> S::Vf64,
) -> f64 {
    let mut sum = 0.0f64;
    let mut i = 0;
    while i + S::Vf64::WIDTH <= a.len() {
        let va = S::Vf64::load_from_slice(&a[i..]);
        let vb = S::Vf64::load_from_slice(&b[i..]);
        sum += op(va, vb).horizontal_add();
        i += S::Vf64::WIDTH;
    }
    sum
}

type ScalarVf32 = <Scalar as Simd>::Vf32;
type ScalarVf64 = <Scalar as Simd>::Vf64;

pub struct BenchTargets {
    pub scalar_native: fn(&[f32]) -> f32,
    pub simdeez_runtime: fn(&[f32]) -> f32,
    pub simdeez_scalar: fn(&[f32]) -> f32,
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    pub simdeez_sse2: unsafe fn(&[f32]) -> f32,
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    pub simdeez_sse41: unsafe fn(&[f32]) -> f32,
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    pub simdeez_avx2: unsafe fn(&[f32]) -> f32,
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    pub simdeez_avx512: unsafe fn(&[f32]) -> f32,
}

pub struct BenchTargetsF64 {
    pub scalar_native: fn(&[f64]) -> f64,
    pub simdeez_runtime: fn(&[f64]) -> f64,
    pub simdeez_scalar: fn(&[f64]) -> f64,
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    pub simdeez_sse2: unsafe fn(&[f64]) -> f64,
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    pub simdeez_sse41: unsafe fn(&[f64]) -> f64,
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    pub simdeez_avx2: unsafe fn(&[f64]) -> f64,
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    pub simdeez_avx512: unsafe fn(&[f64]) -> f64,
}

pub struct BinaryBenchTargets {
    pub scalar_native: fn(&[f32], &[f32]) -> f32,
    pub simdeez_runtime: fn(&[f32], &[f32]) -> f32,
    pub simdeez_scalar: fn(&[f32], &[f32]) -> f32,
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    pub simdeez_sse2: unsafe fn(&[f32], &[f32]) -> f32,
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    pub simdeez_sse41: unsafe fn(&[f32], &[f32]) -> f32,
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    pub simdeez_avx2: unsafe fn(&[f32], &[f32]) -> f32,
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    pub simdeez_avx512: unsafe fn(&[f32], &[f32]) -> f32,
}

pub struct BinaryBenchTargetsF64 {
    pub scalar_native: fn(&[f64], &[f64]) -> f64,
    pub simdeez_runtime: fn(&[f64], &[f64]) -> f64,
    pub simdeez_scalar: fn(&[f64], &[f64]) -> f64,
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    pub simdeez_sse2: unsafe fn(&[f64], &[f64]) -> f64,
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    pub simdeez_sse41: unsafe fn(&[f64], &[f64]) -> f64,
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    pub simdeez_avx2: unsafe fn(&[f64], &[f64]) -> f64,
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    pub simdeez_avx512: unsafe fn(&[f64], &[f64]) -> f64,
}

#[inline(never)]
pub fn force_scalar_sum(input: &[f32], op: impl Fn(ScalarVf32) -> ScalarVf32) -> f32 {
    simdeez_unary_sum_impl::<Scalar>(input, op)
}

#[inline(never)]
pub fn force_scalar_sum_f64(input: &[f64], op: impl Fn(ScalarVf64) -> ScalarVf64) -> f64 {
    simdeez_unary_sum_impl_f64::<Scalar>(input, op)
}

#[inline(never)]
pub fn force_scalar_binary_sum(
    a: &[f32],
    b: &[f32],
    op: impl Fn(ScalarVf32, ScalarVf32) -> ScalarVf32,
) -> f32 {
    simdeez_binary_sum_impl::<Scalar>(a, b, op)
}

#[inline(never)]
pub fn force_scalar_binary_sum_f64(
    a: &[f64],
    b: &[f64],
    op: impl Fn(ScalarVf64, ScalarVf64) -> ScalarVf64,
) -> f64 {
    simdeez_binary_sum_impl_f64::<Scalar>(a, b, op)
}

pub fn bench_unary_variants(c: &mut Criterion, name: &str, input: &[f32], targets: BenchTargets) {
    let mut group = c.benchmark_group(name);
    group.throughput(Throughput::Elements(input.len() as u64));
    group.bench_function("scalar-native", |b| {
        b.iter(|| black_box((targets.scalar_native)(black_box(input))))
    });
    group.bench_function("simdeez-runtime", |b| {
        b.iter(|| black_box((targets.simdeez_runtime)(black_box(input))))
    });
    group.bench_function("simdeez-forced-scalar", |b| {
        b.iter(|| black_box((targets.simdeez_scalar)(black_box(input))))
    });

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
        if std::is_x86_feature_detected!("sse2") {
            group.bench_function("simdeez-forced-sse2", |b| {
                b.iter(|| unsafe { black_box((targets.simdeez_sse2)(black_box(input))) })
            });
        }
        if std::is_x86_feature_detected!("sse4.1") {
            group.bench_function("simdeez-forced-sse41", |b| {
                b.iter(|| unsafe { black_box((targets.simdeez_sse41)(black_box(input))) })
            });
        }
        if std::is_x86_feature_detected!("avx2") && std::is_x86_feature_detected!("fma") {
            group.bench_function("simdeez-forced-avx2", |b| {
                b.iter(|| unsafe { black_box((targets.simdeez_avx2)(black_box(input))) })
            });
        }
        if std::is_x86_feature_detected!("avx512f")
            && std::is_x86_feature_detected!("avx512bw")
            && std::is_x86_feature_detected!("avx512dq")
        {
            group.bench_function("simdeez-forced-avx512", |b| {
                b.iter(|| unsafe { black_box((targets.simdeez_avx512)(black_box(input))) })
            });
        }
    }

    group.finish();
}

pub fn bench_unary_variants_f64(
    c: &mut Criterion,
    name: &str,
    input: &[f64],
    targets: BenchTargetsF64,
) {
    let mut group = c.benchmark_group(name);
    group.throughput(Throughput::Elements(input.len() as u64));
    group.bench_function("scalar-native", |b| {
        b.iter(|| black_box((targets.scalar_native)(black_box(input))))
    });
    group.bench_function("simdeez-runtime", |b| {
        b.iter(|| black_box((targets.simdeez_runtime)(black_box(input))))
    });
    group.bench_function("simdeez-forced-scalar", |b| {
        b.iter(|| black_box((targets.simdeez_scalar)(black_box(input))))
    });

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
        if std::is_x86_feature_detected!("sse2") {
            group.bench_function("simdeez-forced-sse2", |b| {
                b.iter(|| unsafe { black_box((targets.simdeez_sse2)(black_box(input))) })
            });
        }
        if std::is_x86_feature_detected!("sse4.1") {
            group.bench_function("simdeez-forced-sse41", |b| {
                b.iter(|| unsafe { black_box((targets.simdeez_sse41)(black_box(input))) })
            });
        }
        if std::is_x86_feature_detected!("avx2") && std::is_x86_feature_detected!("fma") {
            group.bench_function("simdeez-forced-avx2", |b| {
                b.iter(|| unsafe { black_box((targets.simdeez_avx2)(black_box(input))) })
            });
        }
        if std::is_x86_feature_detected!("avx512f")
            && std::is_x86_feature_detected!("avx512bw")
            && std::is_x86_feature_detected!("avx512dq")
        {
            group.bench_function("simdeez-forced-avx512", |b| {
                b.iter(|| unsafe { black_box((targets.simdeez_avx512)(black_box(input))) })
            });
        }
    }

    group.finish();
}

pub fn bench_binary_variants(
    c: &mut Criterion,
    name: &str,
    a: &[f32],
    b: &[f32],
    targets: BinaryBenchTargets,
) {
    let mut group = c.benchmark_group(name);
    group.throughput(Throughput::Elements(a.len() as u64));
    group.bench_function("scalar-native", |ben| {
        ben.iter(|| black_box((targets.scalar_native)(black_box(a), black_box(b))))
    });
    group.bench_function("simdeez-runtime", |ben| {
        ben.iter(|| black_box((targets.simdeez_runtime)(black_box(a), black_box(b))))
    });
    group.bench_function("simdeez-forced-scalar", |ben| {
        ben.iter(|| black_box((targets.simdeez_scalar)(black_box(a), black_box(b))))
    });

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
        if std::is_x86_feature_detected!("sse2") {
            group.bench_function("simdeez-forced-sse2", |ben| {
                ben.iter(|| unsafe {
                    black_box((targets.simdeez_sse2)(black_box(a), black_box(b)))
                })
            });
        }
        if std::is_x86_feature_detected!("sse4.1") {
            group.bench_function("simdeez-forced-sse41", |ben| {
                ben.iter(|| unsafe {
                    black_box((targets.simdeez_sse41)(black_box(a), black_box(b)))
                })
            });
        }
        if std::is_x86_feature_detected!("avx2") && std::is_x86_feature_detected!("fma") {
            group.bench_function("simdeez-forced-avx2", |ben| {
                ben.iter(|| unsafe {
                    black_box((targets.simdeez_avx2)(black_box(a), black_box(b)))
                })
            });
        }
        if std::is_x86_feature_detected!("avx512f")
            && std::is_x86_feature_detected!("avx512bw")
            && std::is_x86_feature_detected!("avx512dq")
        {
            group.bench_function("simdeez-forced-avx512", |ben| {
                ben.iter(|| unsafe {
                    black_box((targets.simdeez_avx512)(black_box(a), black_box(b)))
                })
            });
        }
    }

    group.finish();
}

pub fn bench_binary_variants_f64(
    c: &mut Criterion,
    name: &str,
    a: &[f64],
    b: &[f64],
    targets: BinaryBenchTargetsF64,
) {
    let mut group = c.benchmark_group(name);
    group.throughput(Throughput::Elements(a.len() as u64));
    group.bench_function("scalar-native", |ben| {
        ben.iter(|| black_box((targets.scalar_native)(black_box(a), black_box(b))))
    });
    group.bench_function("simdeez-runtime", |ben| {
        ben.iter(|| black_box((targets.simdeez_runtime)(black_box(a), black_box(b))))
    });
    group.bench_function("simdeez-forced-scalar", |ben| {
        ben.iter(|| black_box((targets.simdeez_scalar)(black_box(a), black_box(b))))
    });

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
        if std::is_x86_feature_detected!("sse2") {
            group.bench_function("simdeez-forced-sse2", |ben| {
                ben.iter(|| unsafe {
                    black_box((targets.simdeez_sse2)(black_box(a), black_box(b)))
                })
            });
        }
        if std::is_x86_feature_detected!("sse4.1") {
            group.bench_function("simdeez-forced-sse41", |ben| {
                ben.iter(|| unsafe {
                    black_box((targets.simdeez_sse41)(black_box(a), black_box(b)))
                })
            });
        }
        if std::is_x86_feature_detected!("avx2") && std::is_x86_feature_detected!("fma") {
            group.bench_function("simdeez-forced-avx2", |ben| {
                ben.iter(|| unsafe {
                    black_box((targets.simdeez_avx2)(black_box(a), black_box(b)))
                })
            });
        }
        if std::is_x86_feature_detected!("avx512f")
            && std::is_x86_feature_detected!("avx512bw")
            && std::is_x86_feature_detected!("avx512dq")
        {
            group.bench_function("simdeez-forced-avx512", |ben| {
                ben.iter(|| unsafe {
                    black_box((targets.simdeez_avx512)(black_box(a), black_box(b)))
                })
            });
        }
    }

    group.finish();
}
