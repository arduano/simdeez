use criterion::{Criterion, Throughput};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use simdeez::prelude::*;
use simdeez::scalar::Scalar;
use std::hint::black_box;

pub const INPUT_LEN: usize = 1 << 20;

pub fn make_positive_log_inputs(len: usize, seed: u64) -> Vec<f32> {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    (0..len)
        .map(|_| {
            let log2x = rng.gen_range(-20.0f32..20.0f32);
            let mantissa = rng.gen_range(1.0f32..2.0f32);
            mantissa * log2x.exp2()
        })
        .collect()
}

pub fn make_exp2_inputs(len: usize, seed: u64) -> Vec<f32> {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    (0..len)
        .map(|_| rng.gen_range(-100.0f32..100.0f32))
        .collect()
}

pub fn make_exp_inputs(len: usize, seed: u64) -> Vec<f32> {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    (0..len).map(|_| rng.gen_range(-80.0f32..80.0f32)).collect()
}

pub fn make_unary_inputs(len: usize, seed: u64, range: core::ops::Range<f32>) -> Vec<f32> {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    (0..len).map(|_| rng.gen_range(range.clone())).collect()
}

pub fn make_trig_inputs(len: usize, seed: u64) -> Vec<f32> {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    (0..len)
        .map(|_| rng.gen_range(-20.0f32 * core::f32::consts::PI..20.0f32 * core::f32::consts::PI))
        .collect()
}

pub fn make_inverse_trig_inputs(len: usize, seed: u64) -> Vec<f32> {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    (0..len).map(|_| rng.gen_range(-1.0f32..1.0f32)).collect()
}

pub fn make_inverse_trig_inputs_f64(len: usize, seed: u64) -> Vec<f64> {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    (0..len).map(|_| rng.gen_range(-1.0f64..1.0f64)).collect()
}

pub fn make_atan_inputs(len: usize, seed: u64) -> Vec<f32> {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    (0..len).map(|_| rng.gen_range(-64.0f32..64.0f32)).collect()
}

pub fn make_atan_inputs_f64(len: usize, seed: u64) -> Vec<f64> {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    (0..len).map(|_| rng.gen_range(-64.0f64..64.0f64)).collect()
}

pub fn make_tan_inputs(len: usize, seed: u64) -> Vec<f32> {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let half_pi = core::f32::consts::FRAC_PI_2;
    (0..len)
        .map(|_| {
            let mut x =
                rng.gen_range(-20.0f32 * core::f32::consts::PI..20.0f32 * core::f32::consts::PI);
            let k = (x / core::f32::consts::PI).round();
            let nearest_pole = (k + 0.5) * core::f32::consts::PI;
            if (x - nearest_pole).abs() < 1.0e-3 {
                x += if x >= 0.0 { 2.5e-3 } else { -2.5e-3 };
            }
            if x == half_pi || x == -half_pi {
                x += 2.5e-3;
            }
            x
        })
        .collect()
}

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

pub fn bench_variants(c: &mut Criterion, group_name: &str, input: &[f32], targets: BenchTargets) {
    let mut group = c.benchmark_group(group_name);
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
        } else {
            eprintln!("[bench] skipped simdeez-forced-sse2 for {group_name}: CPU lacks sse2");
        }

        if std::is_x86_feature_detected!("sse4.1") {
            group.bench_function("simdeez-forced-sse41", |b| {
                b.iter(|| unsafe { black_box((targets.simdeez_sse41)(black_box(input))) })
            });
        } else {
            eprintln!("[bench] skipped simdeez-forced-sse41 for {group_name}: CPU lacks sse4.1");
        }

        if std::is_x86_feature_detected!("avx2") && std::is_x86_feature_detected!("fma") {
            group.bench_function("simdeez-forced-avx2", |b| {
                b.iter(|| unsafe { black_box((targets.simdeez_avx2)(black_box(input))) })
            });
        } else {
            eprintln!("[bench] skipped simdeez-forced-avx2 for {group_name}: CPU lacks avx2/fma");
        }

        if std::is_x86_feature_detected!("avx512f")
            && std::is_x86_feature_detected!("avx512bw")
            && std::is_x86_feature_detected!("avx512dq")
        {
            group.bench_function("simdeez-forced-avx512", |b| {
                b.iter(|| unsafe { black_box((targets.simdeez_avx512)(black_box(input))) })
            });
        } else {
            eprintln!(
                "[bench] skipped simdeez-forced-avx512 for {group_name}: CPU lacks avx512f+bw+dq"
            );
        }
    }

    group.finish();
}

pub fn bench_variants_f64(
    c: &mut Criterion,
    group_name: &str,
    input: &[f64],
    targets: BenchTargetsF64,
) {
    let mut group = c.benchmark_group(group_name);
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
        } else {
            eprintln!("[bench] skipped simdeez-forced-sse2 for {group_name}: CPU lacks sse2");
        }

        if std::is_x86_feature_detected!("sse4.1") {
            group.bench_function("simdeez-forced-sse41", |b| {
                b.iter(|| unsafe { black_box((targets.simdeez_sse41)(black_box(input))) })
            });
        } else {
            eprintln!("[bench] skipped simdeez-forced-sse41 for {group_name}: CPU lacks sse4.1");
        }

        if std::is_x86_feature_detected!("avx2") && std::is_x86_feature_detected!("fma") {
            group.bench_function("simdeez-forced-avx2", |b| {
                b.iter(|| unsafe { black_box((targets.simdeez_avx2)(black_box(input))) })
            });
        } else {
            eprintln!("[bench] skipped simdeez-forced-avx2 for {group_name}: CPU lacks avx2/fma");
        }

        if std::is_x86_feature_detected!("avx512f")
            && std::is_x86_feature_detected!("avx512bw")
            && std::is_x86_feature_detected!("avx512dq")
        {
            group.bench_function("simdeez-forced-avx512", |b| {
                b.iter(|| unsafe { black_box((targets.simdeez_avx512)(black_box(input))) })
            });
        } else {
            eprintln!(
                "[bench] skipped simdeez-forced-avx512 for {group_name}: CPU lacks avx512f+bw+dq"
            );
        }
    }

    group.finish();
}

type ScalarVf32 = <Scalar as Simd>::Vf32;
type ScalarVf64 = <Scalar as Simd>::Vf64;

#[inline(never)]
pub fn force_scalar_sum(input: &[f32], op: impl Fn(ScalarVf32) -> ScalarVf32) -> f32 {
    simdeez_sum_impl::<Scalar>(input, op)
}

#[inline(never)]
pub fn force_scalar_sum_f64(input: &[f64], op: impl Fn(ScalarVf64) -> ScalarVf64) -> f64 {
    simdeez_sum_impl_f64::<Scalar>(input, op)
}

#[inline(always)]
pub fn simdeez_sum_impl<S: Simd>(input: &[f32], op: impl Fn(S::Vf32) -> S::Vf32) -> f32 {
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
pub fn simdeez_sum_impl_f64<S: Simd>(input: &[f64], op: impl Fn(S::Vf64) -> S::Vf64) -> f64 {
    let mut sum = 0.0f64;
    let mut i = 0;

    while i + S::Vf64::WIDTH <= input.len() {
        let v = S::Vf64::load_from_slice(&input[i..]);
        sum += op(v).horizontal_add();
        i += S::Vf64::WIDTH;
    }

    sum
}
