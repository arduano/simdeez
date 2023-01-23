use criterion::{criterion_group, criterion_main, Criterion};
use std::arch::x86_64::*;

const A: [f32; 8] = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
const B: [f32; 8] = [7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0, 0.0];

fn double_load() -> __m256 {
    unsafe {
        let suma = _mm_add_ps(
            _mm_loadu_ps(A.get_unchecked(0)),
            _mm_loadu_ps(B.get_unchecked(0)),
        );
        let sumb = _mm_add_ps(
            _mm_loadu_ps(A.get_unchecked(4)),
            _mm_loadu_ps(B.get_unchecked(4)),
        );
        let suma = std::mem::transmute::<__m128, [f32; 4]>(suma);
        let sumb = std::mem::transmute::<__m128, [f32; 4]>(sumb);
        _mm256_loadu2_m128(&sumb[0], &suma[0])
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("double load", |f| f.iter(double_load));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
