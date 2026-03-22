use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

#[path = "simd_math/f64_core.rs"]
mod f64_core;
#[path = "simd_math/hyperbolic.rs"]
mod hyperbolic;
#[path = "simd_math/inverse_trig.rs"]
mod inverse_trig;
#[path = "simd_math/log_exp.rs"]
mod log_exp;
#[path = "simd_math/shared.rs"]
mod shared;
#[path = "simd_math/trig.rs"]
mod trig;

fn criterion_benchmark(c: &mut Criterion) {
    log_exp::register(c);
    f64_core::register(c);
    hyperbolic::register(c);
    inverse_trig::register(c);
    trig::register(c);
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
