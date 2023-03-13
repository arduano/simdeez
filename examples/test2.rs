use simdeez::{prelude::*, simd_runtime_generate_v2};

struct NumberParser<'s> {
    string: &'s str,
}

simd_runtime_generate_v2!(
    fn sum() {
        let mut zero = S::Vi8::zeroes();

        zero[7] = -1;

        let mask = zero.get_mask();

        println!("{:#b}", mask);
        println!("{}", mask.trailing_zeros());
    }
);

fn main() {
    sum();
}
