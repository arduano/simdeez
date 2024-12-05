use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use simdeez::{prelude::*, simd_runtime_generate};

simd_runtime_generate!(
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

struct NumberParser<'a> {
    string: &'a [u8],
}

impl<'a> NumberParser<'a> {
    fn new(string: &'a str) -> Self {
        Self {
            string: string.as_bytes(),
        }
    }

    fn get_next(&mut self) -> Option<u64> {
        #![allow(clippy::manual_is_ascii_check)]

        let is_num_byte = |b: u8| (b'0'..=b'9').contains(&b);

        if self.string.is_empty() {
            return None;
        }

        while !is_num_byte(self.string[0]) {
            self.string = &self.string[1..];
            if self.string.is_empty() {
                return None;
            }
        }

        let mut num = 0;
        while is_num_byte(self.string[0]) {
            num *= 10;
            num += (self.string[0] - b'0') as u64;
            self.string = &self.string[1..];
            if self.string.is_empty() {
                break;
            }
        }

        Some(num)
    }
}

struct NumberParserSimd<'a, S: Simd> {
    string: &'a [i8],
    _simd: std::marker::PhantomData<S>,
}

impl<'a, S: Simd> NumberParserSimd<'a, S> {
    fn new(string: &'a str) -> Self {
        Self {
            // Transmuting here is safe because we're converting from u8 to i8
            // This is necessary to make loading easier below
            string: unsafe { core::mem::transmute::<&[u8], &[i8]>(string.as_bytes()) },
            _simd: std::marker::PhantomData,
        }
    }

    fn get_next(&mut self) -> Option<u64> {
        let zero_char = S::Vi8::set1(b'0' as i8);
        let nine_char = S::Vi8::set1(b'9' as i8);

        let next_simd_match_vec =
            |chars: S::Vi8| chars.cmp_gte(zero_char) & chars.cmp_lte(nine_char);

        if self.string.is_empty() {
            return None;
        }

        // Locate the first number byte
        loop {
            if self.string.is_empty() {
                return None;
            }

            let chars = S::Vi8::load_from_slice(self.string);
            let mask = next_simd_match_vec(chars);

            if let Some(index) = mask.index_of_first_truthy() {
                self.string = &self.string[index..];
                break;
            } else {
                if self.string.len() < S::Vi8::WIDTH {
                    self.string = &self.string[0..0];
                    return None;
                }
                self.string = &self.string[S::Vi8::WIDTH..];
            }
        }

        let mut number_end_str = self.string;
        let mut num_length = 0;

        // Locate the end of the number
        loop {
            let chars = S::Vi8::load_from_slice(number_end_str);
            let mask = next_simd_match_vec(chars);

            if let Some(index) = mask.index_of_first_falsy() {
                number_end_str = &number_end_str[index..];
                num_length += index;
                break;
            } else if self.string.len() <= S::Vi8::WIDTH {
                number_end_str = &number_end_str[0..0];
                num_length += self.string.len();
                break;
            } else {
                number_end_str = &number_end_str[S::Vi8::WIDTH..];
                num_length += S::Vi8::WIDTH;
            }
        }

        // Parse the number
        let mut total = 0;
        let mut num_str = &self.string[..num_length];

        loop {
            let num_chars = S::Vi8::load_from_slice(num_str);
            let digits = num_chars - zero_char;

            let length = num_str.len().min(S::Vi8::WIDTH);
            for digit in digits.iter().take(length) {
                total *= 10;
                total += digit as u64;
            }

            if num_str.len() <= S::Vi8::WIDTH {
                break;
            } else {
                num_str = &num_str[S::Vi8::WIDTH..];
            }
        }

        self.string = number_end_str;

        Some(total)
    }
}

fn scalar_get_sum(prompt: &str) -> u64 {
    let mut parser = NumberParser::new(prompt);

    let mut sum = 0;
    while let Some(num) = parser.get_next() {
        sum += num;
    }

    sum
}

simd_runtime_generate!(
    fn simd_get_sum(prompt: &str) -> u64 {
        let mut parser = NumberParserSimd::<S>::new(prompt);

        let mut sum = 0;
        while let Some(num) = parser.get_next() {
            sum += num;
        }

        sum
    }
);

fn main() {
    let mut rng = ChaCha8Rng::seed_from_u64(0);
    let chars = "abcdefghijklmnopqrstuvwxyz";
    let nums = "1234567890";

    let parts_iter = std::iter::repeat_with(move || {
        let mut string = String::new();
        for _ in 0..rng.gen_range(1..100) {
            string.push(chars.chars().nth(rng.gen_range(0..chars.len())).unwrap());
        }
        for _ in 0..rng.gen_range(1..13) {
            string.push(nums.chars().nth(rng.gen_range(0..nums.len())).unwrap());
        }
        string
    });

    let prompt = parts_iter.take(10000).collect::<Vec<_>>().join("");

    dbg!(scalar_get_sum(&prompt));
    dbg!(simd_get_sum(&prompt));
    dbg!(simd_get_sum_scalar(&prompt));
}
