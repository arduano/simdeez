use crate::base::SimdBaseIo;
use crate::engines::scalar::I8x1;
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
use crate::engines::{avx2::I8x32, sse2::I8x16, sse41::I8x16_41};
use crate::prelude::*;

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
fn assert_truthy_contract<V>(all_truthy_mask: V, mixed_mask: V, all_zero: V)
where
    V: SimdInt8 + SimdBaseIo<Scalar = i8> + core::fmt::Debug,
{
    assert!(all_truthy_mask.is_truthy());
    assert!(all_truthy_mask.is_any_truthy());

    assert!(!mixed_mask.is_truthy());
    assert!(mixed_mask.is_any_truthy());

    assert!(!all_zero.is_truthy());
    assert!(!all_zero.is_any_truthy());
}

// This locks down the intended contract for SimdInt8 truthiness helpers on canonical compare masks:
// `-1` means truthy and `0` means falsy. The x86 bug fixed in this PR made some backends behave like
// "any truthy" (or the inverse) instead of "all truthy" for `is_truthy`.
#[test]
fn i8_is_truthy_requires_all_lanes_to_be_truthy_mask() {
    assert!(I8x1::from(-1).is_truthy());
    assert!(I8x1::from(-1).is_any_truthy());
    assert!(!I8x1::from(0).is_truthy());
    assert!(!I8x1::from(0).is_any_truthy());

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
        if std::arch::is_x86_feature_detected!("sse2") {
            let all_nonzero = unsafe { I8x16::load_from_array([-1; 16]) };
            let mixed_zero = unsafe {
                I8x16::load_from_array([
                    -1, -1, -1, -1, -1, -1, -1, -1, 0, -1, -1, -1, -1, -1, -1, -1,
                ])
            };
            let all_zero = unsafe { I8x16::load_from_array([0; 16]) };
            assert_truthy_contract(all_nonzero, mixed_zero, all_zero);
        }

        if std::arch::is_x86_feature_detected!("sse4.1") {
            let all_nonzero = unsafe { I8x16_41::load_from_array([-1; 16]) };
            let mixed_zero = unsafe {
                I8x16_41::load_from_array([
                    -1, -1, -1, -1, -1, -1, -1, -1, 0, -1, -1, -1, -1, -1, -1, -1,
                ])
            };
            let all_zero = unsafe { I8x16_41::load_from_array([0; 16]) };
            assert_truthy_contract(all_nonzero, mixed_zero, all_zero);
        }

        if std::arch::is_x86_feature_detected!("avx2") && std::arch::is_x86_feature_detected!("fma")
        {
            let all_nonzero = unsafe { I8x32::load_from_array([-1; 32]) };
            let mixed_zero = unsafe {
                I8x32::load_from_array([
                    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, -1, -1, -1, -1,
                    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
                ])
            };
            let all_zero = unsafe { I8x32::load_from_array([0; 32]) };
            assert_truthy_contract(all_nonzero, mixed_zero, all_zero);
        }
    }
}
