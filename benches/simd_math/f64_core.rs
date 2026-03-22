use criterion::{Criterion, Throughput};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use simdeez::math::SimdMathF64Core;
use simdeez::{prelude::*, simd_unsafe_generate_all};
use std::hint::black_box;

const INPUT_LEN: usize = 1 << 20;

fn make_positive_log_inputs(seed: u64) -> Vec<f64> {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    (0..INPUT_LEN)
        .map(|_| {
            let log2x = rng.gen_range(-40.0f64..40.0f64);
            let mantissa = rng.gen_range(1.0f64..2.0f64);
            mantissa * log2x.exp2()
        })
        .collect()
}

fn make_exp2_inputs(seed: u64) -> Vec<f64> {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    (0..INPUT_LEN)
        .map(|_| rng.gen_range(-1000.0f64..1000.0f64))
        .collect()
}

fn make_exp_inputs(seed: u64) -> Vec<f64> {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    (0..INPUT_LEN)
        .map(|_| rng.gen_range(-700.0f64..700.0f64))
        .collect()
}

fn make_trig_inputs(seed: u64) -> Vec<f64> {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    (0..INPUT_LEN)
        .map(|_| rng.gen_range(-100.0f64 * core::f64::consts::PI..100.0f64 * core::f64::consts::PI))
        .collect()
}

fn make_tan_inputs(seed: u64) -> Vec<f64> {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    (0..INPUT_LEN)
        .map(|_| {
            let mut x =
                rng.gen_range(-100.0f64 * core::f64::consts::PI..100.0f64 * core::f64::consts::PI);
            let k = (x / core::f64::consts::PI).round();
            let nearest_pole = (k + 0.5) * core::f64::consts::PI;
            if (x - nearest_pole).abs() < 1.0e-8 {
                x += if x >= 0.0 { 2.5e-8 } else { -2.5e-8 };
            }
            x
        })
        .collect()
}

#[inline(always)]
fn simdeez_sum_impl<S: Simd>(input: &[f64], op: impl Fn(S::Vf64) -> S::Vf64) -> f64 {
    let mut sum = 0.0f64;
    let mut i = 0;

    while i + S::Vf64::WIDTH <= input.len() {
        let v = S::Vf64::load_from_slice(&input[i..]);
        sum += op(v).horizontal_add();
        i += S::Vf64::WIDTH;
    }

    sum
}

#[inline(never)]
fn scalar_log2_sum(input: &[f64]) -> f64 {
    input.iter().copied().map(f64::log2).sum()
}
#[inline(never)]
fn scalar_exp2_sum(input: &[f64]) -> f64 {
    input.iter().copied().map(f64::exp2).sum()
}
#[inline(never)]
fn scalar_ln_sum(input: &[f64]) -> f64 {
    input.iter().copied().map(f64::ln).sum()
}
#[inline(never)]
fn scalar_exp_sum(input: &[f64]) -> f64 {
    input.iter().copied().map(f64::exp).sum()
}
#[inline(never)]
fn scalar_sin_sum(input: &[f64]) -> f64 {
    input.iter().copied().map(f64::sin).sum()
}
#[inline(never)]
fn scalar_cos_sum(input: &[f64]) -> f64 {
    input.iter().copied().map(f64::cos).sum()
}
#[inline(never)]
fn scalar_tan_sum(input: &[f64]) -> f64 {
    input.iter().copied().map(f64::tan).sum()
}

simd_unsafe_generate_all!(
    fn simdeez_log2_sum(input: &[f64]) -> f64 {
        simdeez_sum_impl::<S>(input, |v| v.log2_u35())
    }
);
simd_unsafe_generate_all!(
    fn simdeez_exp2_sum(input: &[f64]) -> f64 {
        simdeez_sum_impl::<S>(input, |v| v.exp2_u35())
    }
);
simd_unsafe_generate_all!(
    fn simdeez_ln_sum(input: &[f64]) -> f64 {
        simdeez_sum_impl::<S>(input, |v| v.ln_u35())
    }
);
simd_unsafe_generate_all!(
    fn simdeez_exp_sum(input: &[f64]) -> f64 {
        simdeez_sum_impl::<S>(input, |v| v.exp_u35())
    }
);
simd_unsafe_generate_all!(
    fn simdeez_sin_sum(input: &[f64]) -> f64 {
        simdeez_sum_impl::<S>(input, |v| v.sin_u35())
    }
);
simd_unsafe_generate_all!(
    fn simdeez_cos_sum(input: &[f64]) -> f64 {
        simdeez_sum_impl::<S>(input, |v| v.cos_u35())
    }
);
simd_unsafe_generate_all!(
    fn simdeez_tan_sum(input: &[f64]) -> f64 {
        simdeez_sum_impl::<S>(input, |v| v.tan_u35())
    }
);

fn bench_pair(
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

pub fn register(c: &mut Criterion) {
    let log_inputs = make_positive_log_inputs(0xF640_1001);
    let exp2_inputs = make_exp2_inputs(0xF640_1002);
    let exp_inputs = make_exp_inputs(0xF640_1003);
    let trig_inputs = make_trig_inputs(0xF640_1004);
    let tan_inputs = make_tan_inputs(0xF640_1005);

    bench_pair(
        c,
        "simd_math/f64/log2_u35",
        &log_inputs,
        scalar_log2_sum,
        simdeez_log2_sum,
    );
    bench_pair(
        c,
        "simd_math/f64/exp2_u35",
        &exp2_inputs,
        scalar_exp2_sum,
        simdeez_exp2_sum,
    );
    bench_pair(
        c,
        "simd_math/f64/ln_u35",
        &log_inputs,
        scalar_ln_sum,
        simdeez_ln_sum,
    );
    bench_pair(
        c,
        "simd_math/f64/exp_u35",
        &exp_inputs,
        scalar_exp_sum,
        simdeez_exp_sum,
    );
    bench_pair(
        c,
        "simd_math/f64/sin_u35",
        &trig_inputs,
        scalar_sin_sum,
        simdeez_sin_sum,
    );
    bench_pair(
        c,
        "simd_math/f64/cos_u35",
        &trig_inputs,
        scalar_cos_sum,
        simdeez_cos_sum,
    );
    bench_pair(
        c,
        "simd_math/f64/tan_u35",
        &tan_inputs,
        scalar_tan_sum,
        simdeez_tan_sum,
    );
}
