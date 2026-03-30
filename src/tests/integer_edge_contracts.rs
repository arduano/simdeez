#![allow(dead_code, unused_parens)]

use crate::prelude::*;

macro_rules! test_integer_edges {
    ($name:ident, $simd_ty:ident, $scalar_ty:ty, $unsigned_ty:ty, $bits:expr) => {
        mod $name {
            use super::*;

            simd_unsafe_generate_all!(
                fn shifts(
                    values: &[$scalar_ty],
                    count: i32,
                    left_out: &mut [$scalar_ty],
                    right_out: &mut [$scalar_ty],
                ) {
                    assert_eq!(values.len(), left_out.len());
                    assert_eq!(values.len(), right_out.len());

                    let mut values = values;
                    let mut left_out = left_out;
                    let mut right_out = right_out;

                    while values.len() >= S::$simd_ty::WIDTH {
                        let v = S::$simd_ty::load_from_slice(values);
                        v.shl(count).copy_to_slice(left_out);
                        v.shr(count).copy_to_slice(right_out);

                        values = &values[S::$simd_ty::WIDTH..];
                        left_out = &mut left_out[S::$simd_ty::WIDTH..];
                        right_out = &mut right_out[S::$simd_ty::WIDTH..];
                    }

                    for ((&value, left), right) in values
                        .iter()
                        .zip(left_out.iter_mut())
                        .zip(right_out.iter_mut())
                    {
                        let count = count as u32;
                        *left = value.wrapping_shl(count);
                        *right = ((value as $unsigned_ty).wrapping_shr(count)) as $scalar_ty;
                    }
                }
            );

            simd_unsafe_generate_all!(
                fn abs_values(values: &[$scalar_ty], out: &mut [$scalar_ty]) {
                    assert_eq!(values.len(), out.len());

                    let mut values = values;
                    let mut out = out;

                    while values.len() >= S::$simd_ty::WIDTH {
                        let v = S::$simd_ty::load_from_slice(values);
                        v.abs().copy_to_slice(out);

                        values = &values[S::$simd_ty::WIDTH..];
                        out = &mut out[S::$simd_ty::WIDTH..];
                    }

                    for (&value, slot) in values.iter().zip(out.iter_mut()) {
                        *slot = value.wrapping_abs();
                    }
                }
            );

            #[test]
            fn shift_count_wraps_and_abs_min_wraps() {
                let values: Vec<$scalar_ty> = vec![
                    <$scalar_ty>::MIN,
                    <$scalar_ty>::MIN + 1,
                    -123,
                    -17,
                    -1,
                    0,
                    1,
                    17,
                    123,
                    <$scalar_ty>::MAX - 1,
                    <$scalar_ty>::MAX,
                ]
                .into_iter()
                .cycle()
                .take(257)
                .collect();

                let counts = vec![
                    -257,
                    -129,
                    -65,
                    -33,
                    -1,
                    0,
                    1,
                    7,
                    ($bits - 1) as i32,
                    $bits as i32,
                    ($bits + 1) as i32,
                    3 * ($bits as i32) + 5,
                ];

                for &count in &counts {
                    let mut left = vec![0 as $scalar_ty; values.len()];
                    let mut right = vec![0 as $scalar_ty; values.len()];
                    shifts(&values, count, &mut left, &mut right);

                    let expected_left: Vec<$scalar_ty> = values
                        .iter()
                        .map(|&v| v.wrapping_shl(count as u32))
                        .collect();
                    let expected_right: Vec<$scalar_ty> = values
                        .iter()
                        .map(|&v| ((v as $unsigned_ty).wrapping_shr(count as u32)) as $scalar_ty)
                        .collect();

                    assert_eq!(left, expected_left, "left shift mismatch for count={count}");
                    assert_eq!(
                        right, expected_right,
                        "right shift mismatch for count={count}"
                    );
                }

                let mut abs_out = vec![0 as $scalar_ty; values.len()];
                abs_values(&values, &mut abs_out);
                let expected_abs: Vec<$scalar_ty> =
                    values.iter().map(|&v| v.wrapping_abs()).collect();
                assert_eq!(abs_out, expected_abs);
                assert!(abs_out.contains(&<$scalar_ty>::MIN));
            }
        }
    };
}

test_integer_edges!(i8_edges, Vi8, i8, u8, 8);
test_integer_edges!(i16_edges, Vi16, i16, u16, 16);
test_integer_edges!(i32_edges, Vi32, i32, u32, 32);
test_integer_edges!(i64_edges, Vi64, i64, u64, 64);
