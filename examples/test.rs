use rand::Rng;
use simdeez::{prelude::*, simd_runtime_generate_v2};

simd_runtime_generate_v2!(
    fn sum(source: &[f32], target: &mut [f32]) {
        let mut source = &source[..source.len()];
        let mut target = &mut target[..source.len()];

        loop {
            let src = S::Vf32::load_from_slice(source);
            let src2 = S::Vf32::load_from_slice(target);
            let sum = src + src2;

            sum.copy_to_slice(target);

            source = &source[S::Vf32::WIDTH..];
            target = &mut target[S::Vf32::WIDTH..];

            if source.len() <= S::Vf32::WIDTH {
                break;
            }
        }
    }
);

fn main() {
    let mut rng = rand::thread_rng();

    let manynum: Vec<f32> = (0..100000000).map(|_| rng.gen()).collect();
    let mut manynum2 = manynum.clone();

    sum(&manynum, &mut manynum2);

    dbg!(manynum2);
}
