#[macro_use]
extern crate criterion;
extern crate simdeez;
use criterion::Criterion;
use criterion::Fun;
use simdeez::avx2::*;
use simdeez::sse2::*;
use simdeez::sse41::*;
use simdeez::*;

fn first_bench(c: &mut Criterion) {
    c.bench_function("first", |b| b.iter(|| 1 + 1));
}

criterion_group!(benches, first_bench);
criterion_main!(benches);
