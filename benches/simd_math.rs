use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use simdeez::{prelude::*, simd_unsafe_generate_all};
use std::{hint::black_box, time::Duration};

const INPUT_LEN: usize = 1 << 20;

fn make_positive_log_inputs(len: usize, seed: u64) -> Vec<f32> {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    (0..len)
        .map(|_| {
            let log2x = rng.gen_range(-20.0f32..20.0f32);
            let mantissa = rng.gen_range(1.0f32..2.0f32);
            mantissa * log2x.exp2()
        })
        .collect()
}

fn make_exp2_inputs(len: usize, seed: u64) -> Vec<f32> {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    (0..len)
        .map(|_| rng.gen_range(-100.0f32..100.0f32))
        .collect()
}

fn make_exp_inputs(len: usize, seed: u64) -> Vec<f32> {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    (0..len).map(|_| rng.gen_range(-80.0f32..80.0f32)).collect()
}

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

#[inline(always)]
fn simdeez_log2_sum_impl<S: Simd>(input: &[f32]) -> f32 {
    let mut sum = 0.0f32;
    let mut i = 0;

    while i + S::Vf32::WIDTH <= input.len() {
        let v = S::Vf32::load_from_slice(&input[i..]);
        sum += v.log2_u35().horizontal_add();
        i += S::Vf32::WIDTH;
    }

    for &x in &input[i..] {
        sum += x.log2();
    }

    sum
}

#[inline(always)]
fn simdeez_exp2_sum_impl<S: Simd>(input: &[f32]) -> f32 {
    let mut sum = 0.0f32;
    let mut i = 0;

    while i + S::Vf32::WIDTH <= input.len() {
        let v = S::Vf32::load_from_slice(&input[i..]);
        sum += v.exp2_u35().horizontal_add();
        i += S::Vf32::WIDTH;
    }

    for &x in &input[i..] {
        sum += x.exp2();
    }

    sum
}

#[inline(always)]
fn simdeez_ln_sum_impl<S: Simd>(input: &[f32]) -> f32 {
    let mut sum = 0.0f32;
    let mut i = 0;

    while i + S::Vf32::WIDTH <= input.len() {
        let v = S::Vf32::load_from_slice(&input[i..]);
        sum += v.ln_u35().horizontal_add();
        i += S::Vf32::WIDTH;
    }

    for &x in &input[i..] {
        sum += x.ln();
    }

    sum
}

#[inline(always)]
fn simdeez_exp_sum_impl<S: Simd>(input: &[f32]) -> f32 {
    let mut sum = 0.0f32;
    let mut i = 0;

    while i + S::Vf32::WIDTH <= input.len() {
        let v = S::Vf32::load_from_slice(&input[i..]);
        sum += v.exp_u35().horizontal_add();
        i += S::Vf32::WIDTH;
    }

    for &x in &input[i..] {
        sum += x.exp();
    }

    sum
}

simd_unsafe_generate_all!(
    fn simdeez_log2_sum(input: &[f32]) -> f32 {
        simdeez_log2_sum_impl::<S>(input)
    }
);

simd_unsafe_generate_all!(
    fn simdeez_exp2_sum(input: &[f32]) -> f32 {
        simdeez_exp2_sum_impl::<S>(input)
    }
);

simd_unsafe_generate_all!(
    fn simdeez_ln_sum(input: &[f32]) -> f32 {
        simdeez_ln_sum_impl::<S>(input)
    }
);

simd_unsafe_generate_all!(
    fn simdeez_exp_sum(input: &[f32]) -> f32 {
        simdeez_exp_sum_impl::<S>(input)
    }
);

struct BenchTargets {
    scalar_native: fn(&[f32]) -> f32,
    simdeez_runtime: fn(&[f32]) -> f32,
    simdeez_scalar: fn(&[f32]) -> f32,
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    simdeez_sse2: unsafe fn(&[f32]) -> f32,
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    simdeez_sse41: unsafe fn(&[f32]) -> f32,
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    simdeez_avx2: unsafe fn(&[f32]) -> f32,
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    simdeez_avx512: unsafe fn(&[f32]) -> f32,
}

fn bench_variants(c: &mut Criterion, group_name: &str, input: &[f32], targets: BenchTargets) {
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

fn criterion_benchmark(c: &mut Criterion) {
    let log_inputs = make_positive_log_inputs(INPUT_LEN, 0xA11C_E001);
    let exp2_inputs = make_exp2_inputs(INPUT_LEN, 0xA11C_E002);
    let exp_inputs = make_exp_inputs(INPUT_LEN, 0xA11C_E003);

    bench_variants(
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

    bench_variants(
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

    bench_variants(
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

    bench_variants(
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

criterion_group! {
    name = benches;
    config = Criterion::default()
        .sample_size(20)
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(2));
    targets = criterion_benchmark
}
criterion_main!(benches);
