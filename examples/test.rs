use rand::prelude::*;
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};
use simdeez::{avx2::Avx2, sse2::I32x4, SimdBase};

pub fn main() {
    unsafe {
        let mask = I32x4::set1(-1);
        let a = I32x4::set1(1);
        let b = I32x4::set1(2);

        dbg!(mask, a, b);

        let mask = mask.cmp_eq(I32x4::set1(-1));
        let a = mask.and_not(a);
        let b = mask.bit_and(b);
        let result = a.bit_or(b);

        dbg!(mask, a, b, result);
    }
}
