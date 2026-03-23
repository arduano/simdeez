#![cfg(target_arch = "wasm32")]

use crate::base::SimdBaseIo;
use crate::engines::wasm32::*;
use crate::SimdConsts;

macro_rules! wasm_unaligned_roundtrip_test {
    ($name:ident, $vec_ty:ty, $scalar_ty:ty, [$($value:expr),+ $(,)?]) => {
        #[test]
        fn $name() {
            let input: [$scalar_ty; <$vec_ty as SimdConsts>::WIDTH] = [$($value),+];
            let mut expected_bytes = [0u8; 16];

            unsafe {
                core::ptr::copy_nonoverlapping(
                    input.as_ptr() as *const u8,
                    expected_bytes.as_mut_ptr(),
                    16,
                );
                for offset in 1usize..16 {
                    let mut bytes = vec![0xA5u8; 16 + offset + 16];
                    core::ptr::copy_nonoverlapping(
                        input.as_ptr() as *const u8,
                        bytes.as_mut_ptr().add(offset),
                        16,
                    );

                    let loaded = <$vec_ty as SimdBaseIo>::load_from_ptr_unaligned(
                        bytes.as_ptr().add(offset) as *const $scalar_ty,
                    );
                    let loaded_array = loaded.as_array();
                    let mut loaded_bytes = [0u8; 16];
                    core::ptr::copy_nonoverlapping(
                        loaded_array.as_ptr() as *const u8,
                        loaded_bytes.as_mut_ptr(),
                        16,
                    );
                    assert_eq!(loaded_bytes, expected_bytes);
                    assert!(bytes[..offset].iter().all(|&byte| byte == 0xA5));
                    assert!(bytes[offset + 16..].iter().all(|&byte| byte == 0xA5));

                    let output = <$vec_ty as SimdBaseIo>::load_from_array(input);
                    let mut dest = vec![0x5Au8; 16 + offset + 16];
                    output.copy_to_ptr_unaligned(dest.as_mut_ptr().add(offset) as *mut $scalar_ty);

                    let mut roundtrip_bytes = [0u8; 16];
                    core::ptr::copy_nonoverlapping(
                        dest.as_ptr().add(offset),
                        roundtrip_bytes.as_mut_ptr(),
                        16,
                    );
                    assert_eq!(roundtrip_bytes, expected_bytes);
                    assert!(dest[..offset].iter().all(|&byte| byte == 0x5A));
                    assert!(dest[offset + 16..].iter().all(|&byte| byte == 0x5A));
                }
            }
        }
    };
}

// These deliberately use misaligned byte offsets so the wasm unaligned load/store paths are exercised
// through the public SimdBaseIo pointer APIs instead of relying on aligned typed pointer dereferences.
wasm_unaligned_roundtrip_test!(
    wasm_unaligned_i8_roundtrip,
    I8x16Wasm,
    i8,
    [-128, -64, -32, -16, -8, -4, -2, -1, 0, 1, 2, 4, 8, 16, 32, 64]
);
wasm_unaligned_roundtrip_test!(
    wasm_unaligned_i16_roundtrip,
    I16x8Wasm,
    i16,
    [-32768, -12345, -1024, -1, 0, 1, 1024, 32767]
);
wasm_unaligned_roundtrip_test!(
    wasm_unaligned_i32_roundtrip,
    I32x4Wasm,
    i32,
    [i32::MIN, -123456789, 123456789, i32::MAX]
);
wasm_unaligned_roundtrip_test!(
    wasm_unaligned_i64_roundtrip,
    I64x2Wasm,
    i64,
    [i64::MIN, i64::MAX]
);
wasm_unaligned_roundtrip_test!(
    wasm_unaligned_f32_roundtrip,
    F32x4Wasm,
    f32,
    [-123.5, -0.0, 0.0, f32::from_bits(0x7fc0_0001)]
);
wasm_unaligned_roundtrip_test!(
    wasm_unaligned_f64_roundtrip,
    F64x2Wasm,
    f64,
    [-123.5, f64::from_bits(0x7ff8_0000_0000_0001)]
);
