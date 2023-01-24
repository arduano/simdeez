use simdeez::{sse2::Sse2, Simd};

pub fn main() {
    unsafe {
        for i in 0..100 {
            let val = i as f32 / 100.0 * 0.0002 + 0.5 - 0.0001 + 1.0;
            let floats = Sse2::set1_ps(val);
            let ints = Sse2::cvtps_epi32(floats);

            println!("{} -> {}", floats[0], ints[0]);
        }
    }
}
