use rand::prelude::*;
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};

const IMPORTANT_F32: [f32; 12] = [
    0.0,
    1.0,
    -1.0,
    0.5,
    -0.5,
    0.25,
    -0.25,
    1.5,
    -1.5,
    std::f32::MAX,
    std::f32::MIN,
    f32::NAN,
];

const IMPORTANT_F64: [f32; 12] = [
    0.0,
    1.0,
    -1.0,
    0.5,
    -0.5,
    0.25,
    -0.25,
    1.5,
    -1.5,
    std::f32::MAX,
    std::f32::MIN,
    f32::NAN,
];

fn iter_arbitrary_f32(interval: usize) -> impl Iterator<Item = f32> {
    assert!(interval > IMPORTANT_F32.len());

    let rng_count = interval - IMPORTANT_F32.len();

    let make_important_iter = || IMPORTANT_F32.iter().cloned();
    let mut i = 0;
    let mut make_random_iter = move || {
        i += 1;
        let mut rng = ChaCha8Rng::seed_from_u64((interval + i) as u64);
        std::iter::repeat_with(move || f32::from_bits(rng.gen_range(0..u32::MAX)))
    };

    std::iter::repeat_with(move || make_important_iter().chain(make_random_iter().take(rng_count)))
        .flatten()
}

fn iter_arbitrary_f64(interval: usize) -> impl Iterator<Item = f32> {
    assert!(interval > IMPORTANT_F32.len());

    let rng_count = interval - IMPORTANT_F32.len();

    let make_important_iter = || IMPORTANT_F32.iter().cloned();
    let mut i = 0;
    let mut make_random_iter = move || {
        i += 1;
        let mut rng = ChaCha8Rng::seed_from_u64((interval + i) as u64);
        std::iter::repeat_with(move || f32::from_bits(rng.gen_range(0..u32::MAX)))
    };

    std::iter::repeat_with(move || make_important_iter().chain(make_random_iter().take(rng_count)))
        .flatten()
}

pub fn main() {
    for f in iter_arbitrary_f32(20).take(1000) {
        println!("{}", f);
    }
}
