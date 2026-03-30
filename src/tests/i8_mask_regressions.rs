#![allow(dead_code, unused_parens)]

use crate::prelude::*;
use crate::SimdBitMask;

fn encode_index(value: Option<usize>) -> usize {
    value.unwrap_or(usize::MAX)
}

simd_unsafe_generate_all!(
    fn summarize_i8_mask_helpers(input: &[i8]) -> (u32, bool, usize, usize, usize, usize) {
        assert_eq!(input.len(), S::Vi8::WIDTH);
        let value = S::Vi8::load_from_slice(input);
        (
            value.get_mask().to_u32_lossy(),
            value.is_any_truthy(),
            encode_index(value.index_of_first_truthy()),
            encode_index(value.index_of_last_truthy()),
            encode_index(value.index_of_first_falsy()),
            encode_index(value.index_of_last_falsy()),
        )
    }
);

fn assert_mask_summary(
    actual: (u32, bool, usize, usize, usize, usize),
    expected_mask: u32,
    first_truthy: Option<usize>,
    last_truthy: Option<usize>,
    first_falsy: Option<usize>,
    last_falsy: Option<usize>,
) {
    assert_eq!(actual.0, expected_mask);
    assert_eq!(actual.1, first_truthy.is_some());
    assert_eq!(actual.2, encode_index(first_truthy));
    assert_eq!(actual.3, encode_index(last_truthy));
    assert_eq!(actual.4, encode_index(first_falsy));
    assert_eq!(actual.5, encode_index(last_falsy));
}

// This locks down the canonical compare-mask contract for i8 mask helpers:
// truthy lanes are `-1`, falsy lanes are `0`, and mask/index helpers should interpret them consistently.
#[test]
fn i8_mask_helpers_follow_canonical_compare_masks() {
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
        assert_mask_summary(
            summarize_i8_mask_helpers_scalar(&[-1]),
            0b1,
            Some(0),
            Some(0),
            None,
            None,
        );
        assert_mask_summary(
            summarize_i8_mask_helpers_scalar(&[0]),
            0b0,
            None,
            None,
            Some(0),
            Some(0),
        );

        let sse_input = [0, -1, 0, -1, -1, 0, 0, -1, 0, 0, -1, 0, -1, 0, 0, -1];
        let avx_input = [
            0, -1, 0, -1, -1, 0, 0, -1, 0, 0, -1, 0, -1, 0, 0, -1, -1, 0, 0, -1, 0, -1, 0, 0, -1,
            0, -1, 0, 0, -1, -1, 0,
        ];

        if std::arch::is_x86_feature_detected!("sse2") {
            assert_mask_summary(
                unsafe { summarize_i8_mask_helpers_sse2(&sse_input) },
                0x949a,
                Some(1),
                Some(15),
                Some(0),
                Some(14),
            );
        }

        if std::arch::is_x86_feature_detected!("sse4.1") {
            assert_mask_summary(
                unsafe { summarize_i8_mask_helpers_sse41(&sse_input) },
                0x949a,
                Some(1),
                Some(15),
                Some(0),
                Some(14),
            );
        }

        if std::arch::is_x86_feature_detected!("avx2") && std::arch::is_x86_feature_detected!("fma")
        {
            assert_mask_summary(
                unsafe { summarize_i8_mask_helpers_avx2(&avx_input) },
                0x6529_949a,
                Some(1),
                Some(30),
                Some(0),
                Some(31),
            );
        }
    }
}
