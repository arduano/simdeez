use crate::engines::scalar::I8x1;
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
use crate::engines::{avx2::I8x32, sse2::I8x16, sse41::I8x16_41};
use crate::prelude::*;
use crate::SimdBitMask;

fn assert_mask_helper_contract<V>(
    value: V,
    expected_mask: u32,
    first_truthy: Option<usize>,
    last_truthy: Option<usize>,
    first_falsy: Option<usize>,
    last_falsy: Option<usize>,
) where
    V: SimdInt8,
{
    assert_eq!(value.get_mask().to_u32_lossy(), expected_mask);
    assert_eq!(value.is_any_truthy(), first_truthy.is_some());
    assert_eq!(value.index_of_first_truthy(), first_truthy);
    assert_eq!(value.index_of_last_truthy(), last_truthy);
    assert_eq!(value.index_of_first_falsy(), first_falsy);
    assert_eq!(value.index_of_last_falsy(), last_falsy);
}

// This locks down the canonical compare-mask contract for i8 mask helpers:
// truthy lanes are `-1`, falsy lanes are `0`, and mask/index helpers should interpret them consistently.
#[test]
fn i8_mask_helpers_follow_canonical_compare_masks() {
    assert_mask_helper_contract(I8x1::from(-1), 0b1, Some(0), Some(0), None, None);
    assert_mask_helper_contract(I8x1::from(0), 0b0, None, None, Some(0), Some(0));

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
        if std::arch::is_x86_feature_detected!("sse2") {
            let value = unsafe {
                I8x16::load_from_array([0, -1, 0, -1, -1, 0, 0, -1, 0, 0, -1, 0, -1, 0, 0, -1])
            };
            assert_mask_helper_contract(
                value,
                0b1001_0100_1001_1010,
                Some(1),
                Some(15),
                Some(0),
                Some(14),
            );
        }

        if std::arch::is_x86_feature_detected!("sse4.1") {
            let value = unsafe {
                I8x16_41::load_from_array([0, -1, 0, -1, -1, 0, 0, -1, 0, 0, -1, 0, -1, 0, 0, -1])
            };
            assert_mask_helper_contract(
                value,
                0b1001_0100_1001_1010,
                Some(1),
                Some(15),
                Some(0),
                Some(14),
            );
        }

        if std::arch::is_x86_feature_detected!("avx2") && std::arch::is_x86_feature_detected!("fma")
        {
            let value = unsafe {
                I8x32::load_from_array([
                    0, -1, 0, -1, -1, 0, 0, -1, 0, 0, -1, 0, -1, 0, 0, -1, -1, 0, 0, -1, 0, -1, 0,
                    0, -1, 0, -1, 0, 0, -1, -1, 0,
                ])
            };
            assert_mask_helper_contract(value, 0x6529_949a, Some(1), Some(30), Some(0), Some(31));
        }
    }
}
