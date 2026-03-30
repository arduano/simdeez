use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

#[path = "simd_math_remaining_baseline/binary_misc.rs"]
mod binary_misc;
#[path = "simd_math_remaining_baseline/inverse_hyperbolic.rs"]
mod inverse_hyperbolic;
#[path = "simd_math_remaining_baseline/inverse_trig.rs"]
mod inverse_trig;
#[path = "simd_math_remaining_baseline/shared.rs"]
mod shared;

fn criterion_benchmark(c: &mut Criterion) {
    inverse_trig::register(c);
    inverse_hyperbolic::register(c);
    binary_misc::register(c);
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
