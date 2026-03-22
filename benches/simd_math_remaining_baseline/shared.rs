use criterion::{Criterion, Throughput};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use simdeez::prelude::*;
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

pub fn bench_unary(
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

pub fn bench_unary_f64(
    c: &mut Criterion,
    name: &str,
    input: &[f64],
    scalar: fn(&[f64]) -> f64,
    simd: fn(&[f64]) -> f64,
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

pub fn bench_binary(
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
