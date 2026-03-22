#![allow(dead_code)]

use crate::prelude::*;

simd_unsafe_generate_all!(
    fn mul_i8_lanes(input_a: &[i8], input_b: &[i8], out: &mut [i8]) {
        assert_eq!(input_a.len(), S::Vi8::WIDTH);
        assert_eq!(input_b.len(), S::Vi8::WIDTH);
        assert_eq!(out.len(), S::Vi8::WIDTH);

        let a = S::Vi8::load_from_slice(input_a);
        let b = S::Vi8::load_from_slice(input_b);
        a.mul(b).copy_to_slice(out);
    }
);

fn expected_mul(a: &[i8], b: &[i8]) -> Vec<i8> {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| x.wrapping_mul(*y))
        .collect()
}

#[test]
fn i8_mul_matches_wrapping_semantics_on_extreme_patterns() {
    // Alternating signs and high-magnitude values stress odd/even lane handling in
    // epi16-based implementations.
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
        let sse_a = [
            -128, 127, -1, 1, -64, 64, -17, 17, -33, 33, -2, 2, -99, 99, -45, 45,
        ];
        let sse_b = [
            2, -2, -1, -1, 3, -3, 11, -11, 7, -7, 127, -128, 5, -5, 9, -9,
        ];
        let sse_expected = expected_mul(&sse_a, &sse_b);

        let mut scalar_out = [0; 1];
        mul_i8_lanes_scalar(&[-128], &[2], &mut scalar_out);
        assert_eq!(scalar_out, [0]);

        if std::arch::is_x86_feature_detected!("sse2") {
            let mut out = [0; 16];
            unsafe { mul_i8_lanes_sse2(&sse_a, &sse_b, &mut out) };
            assert_eq!(out.as_slice(), sse_expected.as_slice());
        }

        if std::arch::is_x86_feature_detected!("sse4.1") {
            let mut out = [0; 16];
            unsafe { mul_i8_lanes_sse41(&sse_a, &sse_b, &mut out) };
            assert_eq!(out.as_slice(), sse_expected.as_slice());
        }

        let avx_a = [
            -128, 127, -1, 1, -64, 64, -17, 17, -33, 33, -2, 2, -99, 99, -45, 45, -120, 120, -7, 7,
            -55, 55, -13, 13, -101, 101, -37, 37, -5, 5, -88, 88,
        ];
        let avx_b = [
            2, -2, -1, -1, 3, -3, 11, -11, 7, -7, 127, -128, 5, -5, 9, -9, 6, -6, 15, -15, 19, -19,
            4, -4, 12, -12, 31, -31, 63, -63, 10, -10,
        ];
        let avx_expected = expected_mul(&avx_a, &avx_b);

        if std::arch::is_x86_feature_detected!("avx2") && std::arch::is_x86_feature_detected!("fma")
        {
            let mut out = [0; 32];
            unsafe { mul_i8_lanes_avx2(&avx_a, &avx_b, &mut out) };
            assert_eq!(out.as_slice(), avx_expected.as_slice());
        }
    }
}
