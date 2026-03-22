use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use simdeez::{prelude::*, simd_unsafe_generate_all};
use std::{hint::black_box, time::Duration};

const INPUT_LEN: usize = 1 << 20;

fn make_unary_inputs(len: usize, seed: u64, range: core::ops::Range<f32>) -> Vec<f32> {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    (0..len).map(|_| rng.gen_range(range.clone())).collect()
}

fn make_positive_inputs(len: usize, seed: u64, min: f32, max: f32) -> Vec<f32> {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    (0..len).map(|_| rng.gen_range(min..max)).collect()
}

fn make_binary_inputs(
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

#[inline(never)]
fn scalar_asin_sum(input: &[f32]) -> f32 {
    input.iter().copied().map(f32::asin).sum()
}
#[inline(never)]
fn scalar_tanh_sum(input: &[f32]) -> f32 {
    input.iter().copied().map(f32::tanh).sum()
}
#[inline(never)]
fn scalar_atanh_sum(input: &[f32]) -> f32 {
    input.iter().copied().map(f32::atanh).sum()
}
#[inline(never)]
fn scalar_log10_sum(input: &[f32]) -> f32 {
    input.iter().copied().map(f32::log10).sum()
}
#[inline(never)]
fn scalar_atan2_sum(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(&x, &y)| x.atan2(y)).sum()
}
#[inline(never)]
fn scalar_fmod_sum(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(&x, &y)| x % y).sum()
}
#[inline(never)]
fn scalar_hypot_sum(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(&x, &y)| x.hypot(y)).sum()
}

#[inline(always)]
fn simdeez_unary_sum_impl<S: Simd>(input: &[f32], op: impl Fn(S::Vf32) -> S::Vf32) -> f32 {
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
fn simdeez_binary_sum_impl<S: Simd>(
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

simd_unsafe_generate_all!(
    fn simdeez_asin_sum(input: &[f32]) -> f32 {
        simdeez_unary_sum_impl::<S>(input, |v| v.asin_u35())
    }
);
simd_unsafe_generate_all!(
    fn simdeez_tanh_sum(input: &[f32]) -> f32 {
        simdeez_unary_sum_impl::<S>(input, |v| v.tanh_u35())
    }
);
simd_unsafe_generate_all!(
    fn simdeez_atanh_sum(input: &[f32]) -> f32 {
        simdeez_unary_sum_impl::<S>(input, |v| v.atanh_u35())
    }
);
simd_unsafe_generate_all!(
    fn simdeez_log10_sum(input: &[f32]) -> f32 {
        simdeez_unary_sum_impl::<S>(input, |v| v.log10_u35())
    }
);
simd_unsafe_generate_all!(
    fn simdeez_atan2_sum(a: &[f32], b: &[f32]) -> f32 {
        simdeez_binary_sum_impl::<S>(a, b, |x, y| x.atan2_u35(y))
    }
);
simd_unsafe_generate_all!(
    fn simdeez_hypot_sum(a: &[f32], b: &[f32]) -> f32 {
        simdeez_binary_sum_impl::<S>(a, b, |x, y| x.hypot_u35(y))
    }
);
simd_unsafe_generate_all!(
    fn simdeez_fmod_sum(a: &[f32], b: &[f32]) -> f32 {
        simdeez_binary_sum_impl::<S>(a, b, |x, y| x.fmod(y))
    }
);

fn bench_unary(
    c: &mut Criterion,
    name: &str,
    input: &[f32],
    scalar: fn(&[f32]) -> f32,
    simd: fn(&[f32]) -> f32,
) {
    let mut group = c.benchmark_group(name);
    group.throughput(Throughput::Elements(input.len() as u64));
    group.bench_function("scalar-native", |b| {
        b.iter(|| black_box(scalar(black_box(input))))
    });
    group.bench_function("simdeez-runtime", |b| {
        b.iter(|| black_box(simd(black_box(input))))
    });
    group.finish();
}

fn bench_binary(
    c: &mut Criterion,
    name: &str,
    a: &[f32],
    b: &[f32],
    scalar: fn(&[f32], &[f32]) -> f32,
    simd: fn(&[f32], &[f32]) -> f32,
) {
    let mut group = c.benchmark_group(name);
    group.throughput(Throughput::Elements(a.len() as u64));
    group.bench_function("scalar-native", |ben| {
        ben.iter(|| black_box(scalar(black_box(a), black_box(b))))
    });
    group.bench_function("simdeez-runtime", |ben| {
        ben.iter(|| black_box(simd(black_box(a), black_box(b))))
    });
    group.finish();
}

fn criterion_benchmark(c: &mut Criterion) {
    let asin_inputs = make_unary_inputs(INPUT_LEN, 0xDEADB001, -1.0..1.0);
    let tanh_inputs = make_unary_inputs(INPUT_LEN, 0xDEADB002, -40.0..40.0);
    let atanh_inputs = make_unary_inputs(INPUT_LEN, 0xDEADB003, -0.999_999..0.999_999);
    let log10_inputs = make_positive_inputs(INPUT_LEN, 0xDEADB004, 1.0e-20, 1.0e20);
    let (atan2_y, atan2_x) =
        make_binary_inputs(INPUT_LEN, 0xDEADB005, -100.0..100.0, -100.0..100.0);
    let (hypot_x, hypot_y) =
        make_binary_inputs(INPUT_LEN, 0xDEADB006, -1.0e20..1.0e20, -1.0e20..1.0e20);
    let (fmod_x, mut fmod_y) =
        make_binary_inputs(INPUT_LEN, 0xDEADB007, -1000.0..1000.0, -1000.0..1000.0);
    for y in &mut fmod_y {
        if *y == 0.0 {
            *y = 1.0;
        }
    }

    bench_unary(
        c,
        "simd_math_baseline/f32/asin_u35",
        &asin_inputs,
        scalar_asin_sum,
        simdeez_asin_sum,
    );
    bench_unary(
        c,
        "simd_math_baseline/f32/tanh_u35",
        &tanh_inputs,
        scalar_tanh_sum,
        simdeez_tanh_sum,
    );
    bench_unary(
        c,
        "simd_math_baseline/f32/atanh_u35",
        &atanh_inputs,
        scalar_atanh_sum,
        simdeez_atanh_sum,
    );
    bench_unary(
        c,
        "simd_math_baseline/f32/log10_u35",
        &log10_inputs,
        scalar_log10_sum,
        simdeez_log10_sum,
    );

    bench_binary(
        c,
        "simd_math_baseline/f32/atan2_u35",
        &atan2_y,
        &atan2_x,
        scalar_atan2_sum,
        simdeez_atan2_sum,
    );
    bench_binary(
        c,
        "simd_math_baseline/f32/hypot_u35",
        &hypot_x,
        &hypot_y,
        scalar_hypot_sum,
        simdeez_hypot_sum,
    );
    bench_binary(
        c,
        "simd_math_baseline/f32/fmod",
        &fmod_x,
        &fmod_y,
        scalar_fmod_sum,
        simdeez_fmod_sum,
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
