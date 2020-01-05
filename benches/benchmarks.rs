#[macro_use]
extern crate criterion;
extern crate rand;
extern crate simdeez;
use criterion::Criterion;
use criterion::Fun;
use simdeez::avx2::*;
use simdeez::sse2::*;
use simdeez::sse41::*;
use simdeez::*;
use std::f32;

#[inline(always)]
unsafe fn floor<S: Simd>(inval: &[f32], out: &mut [f32]) {
    let n = S::loadu_ps(&inval[0]);
    S::storeu_ps(&mut out[0], S::fast_floor_ps(n));
}

#[target_feature(enable = "sse2")]
unsafe fn floorsse2(inval: &[f32], out: &mut [f32]) {
    floor::<Sse2>(inval, out)
}
#[target_feature(enable = "sse4.1")]
unsafe fn floorsse41(inval: &[f32], out: &mut [f32]) {
    floor::<Sse41>(inval, out)
}

fn floor_bench(c: &mut Criterion) {
    let mut nums = Vec::new();
    for _i in 0..8129 {
        nums.push(rand::random::<f32>() * f32::MAX);
    }
    let mut result = Vec::with_capacity(nums.len());
    unsafe { result.set_len(nums.len()) }
    c.bench_function("first", move |b| {
        b.iter(|| {
            let mut i = 0;
            while i < nums.len() - 4 {
                unsafe {
                    floorsse2(&nums[i..i + 4], &mut result[i..i + 4]);
                }
                i += 4;
            }
        })
    });
}
criterion_group!(benches, floor_bench);
criterion_main!(benches);
