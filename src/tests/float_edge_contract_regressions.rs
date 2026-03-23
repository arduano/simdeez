#![allow(dead_code, unused_parens)]

use crate::prelude::*;

fn assert_f32_min_max_contract(a: f32, b: f32, min: f32, max: f32) {
    if a.is_nan() || b.is_nan() {
        assert!(min.is_nan() || min == a || min == b);
        assert!(max.is_nan() || max == a || max == b);
        return;
    }

    if a == 0.0 && b == 0.0 {
        assert_eq!(min, 0.0);
        assert_eq!(max, 0.0);
        return;
    }

    assert_eq!(min.to_bits(), scalar_simd_min_f32(a, b).to_bits());
    assert_eq!(max.to_bits(), scalar_simd_max_f32(a, b).to_bits());
}

fn assert_f64_min_max_contract(a: f64, b: f64, min: f64, max: f64) {
    if a.is_nan() || b.is_nan() {
        assert!(min.is_nan() || min == a || min == b);
        assert!(max.is_nan() || max == a || max == b);
        return;
    }

    if a == 0.0 && b == 0.0 {
        assert_eq!(min, 0.0);
        assert_eq!(max, 0.0);
        return;
    }

    assert_eq!(min.to_bits(), scalar_simd_min_f64(a, b).to_bits());
    assert_eq!(max.to_bits(), scalar_simd_max_f64(a, b).to_bits());
}

#[inline]
fn scalar_simd_min_f32(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

#[inline]
fn scalar_simd_max_f32(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

#[inline]
fn scalar_simd_min_f64(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}

#[inline]
fn scalar_simd_max_f64(a: f64, b: f64) -> f64 {
    if a > b {
        a
    } else {
        b
    }
}

simd_unsafe_generate_all!(
    fn min_max_f32(lhs: &[f32], rhs: &[f32], min_out: &mut [f32], max_out: &mut [f32]) {
        assert_eq!(lhs.len(), rhs.len());
        assert_eq!(lhs.len(), min_out.len());
        assert_eq!(lhs.len(), max_out.len());

        let mut lhs = lhs;
        let mut rhs = rhs;
        let mut min_out = min_out;
        let mut max_out = max_out;

        while lhs.len() >= S::Vf32::WIDTH {
            let a = S::Vf32::load_from_slice(lhs);
            let b = S::Vf32::load_from_slice(rhs);
            a.min(b).copy_to_slice(min_out);
            a.max(b).copy_to_slice(max_out);

            lhs = &lhs[S::Vf32::WIDTH..];
            rhs = &rhs[S::Vf32::WIDTH..];
            min_out = &mut min_out[S::Vf32::WIDTH..];
            max_out = &mut max_out[S::Vf32::WIDTH..];
        }

        for ((&a, &b), (min_slot, max_slot)) in lhs
            .iter()
            .zip(rhs.iter())
            .zip(min_out.iter_mut().zip(max_out.iter_mut()))
        {
            *min_slot = scalar_simd_min_f32(a, b);
            *max_slot = scalar_simd_max_f32(a, b);
        }
    }
);

simd_unsafe_generate_all!(
    fn min_max_f64(lhs: &[f64], rhs: &[f64], min_out: &mut [f64], max_out: &mut [f64]) {
        assert_eq!(lhs.len(), rhs.len());
        assert_eq!(lhs.len(), min_out.len());
        assert_eq!(lhs.len(), max_out.len());

        let mut lhs = lhs;
        let mut rhs = rhs;
        let mut min_out = min_out;
        let mut max_out = max_out;

        while lhs.len() >= S::Vf64::WIDTH {
            let a = S::Vf64::load_from_slice(lhs);
            let b = S::Vf64::load_from_slice(rhs);
            a.min(b).copy_to_slice(min_out);
            a.max(b).copy_to_slice(max_out);

            lhs = &lhs[S::Vf64::WIDTH..];
            rhs = &rhs[S::Vf64::WIDTH..];
            min_out = &mut min_out[S::Vf64::WIDTH..];
            max_out = &mut max_out[S::Vf64::WIDTH..];
        }

        for ((&a, &b), (min_slot, max_slot)) in lhs
            .iter()
            .zip(rhs.iter())
            .zip(min_out.iter_mut().zip(max_out.iter_mut()))
        {
            *min_slot = scalar_simd_min_f64(a, b);
            *max_slot = scalar_simd_max_f64(a, b);
        }
    }
);

simd_unsafe_generate_all!(
    fn rsqrt_f32(values: &[f32], out: &mut [f32]) {
        assert_eq!(values.len(), out.len());

        let mut values = values;
        let mut out = out;

        while values.len() >= S::Vf32::WIDTH {
            let a = S::Vf32::load_from_slice(values);
            a.rsqrt().copy_to_slice(out);

            values = &values[S::Vf32::WIDTH..];
            out = &mut out[S::Vf32::WIDTH..];
        }

        for (&value, slot) in values.iter().zip(out.iter_mut()) {
            *slot = 1.0 / value.sqrt();
        }
    }
);

simd_unsafe_generate_all!(
    fn rsqrt_f64(values: &[f64], out: &mut [f64]) {
        assert_eq!(values.len(), out.len());

        let mut values = values;
        let mut out = out;

        while values.len() >= S::Vf64::WIDTH {
            let a = S::Vf64::load_from_slice(values);
            a.rsqrt().copy_to_slice(out);

            values = &values[S::Vf64::WIDTH..];
            out = &mut out[S::Vf64::WIDTH..];
        }

        for (&value, slot) in values.iter().zip(out.iter_mut()) {
            *slot = 1.0 / value.sqrt();
        }
    }
);

simd_unsafe_generate_all!(
    fn div_f32(lhs: &[f32], rhs: &[f32], out: &mut [f32]) {
        assert_eq!(lhs.len(), rhs.len());
        assert_eq!(lhs.len(), out.len());

        let mut lhs = lhs;
        let mut rhs = rhs;
        let mut out = out;

        while lhs.len() >= S::Vf32::WIDTH {
            let a = S::Vf32::load_from_slice(lhs);
            let b = S::Vf32::load_from_slice(rhs);
            (a / b).copy_to_slice(out);

            lhs = &lhs[S::Vf32::WIDTH..];
            rhs = &rhs[S::Vf32::WIDTH..];
            out = &mut out[S::Vf32::WIDTH..];
        }

        for ((&a, &b), slot) in lhs.iter().zip(rhs.iter()).zip(out.iter_mut()) {
            *slot = a / b;
        }
    }
);

simd_unsafe_generate_all!(
    fn div_f64(lhs: &[f64], rhs: &[f64], out: &mut [f64]) {
        assert_eq!(lhs.len(), rhs.len());
        assert_eq!(lhs.len(), out.len());

        let mut lhs = lhs;
        let mut rhs = rhs;
        let mut out = out;

        while lhs.len() >= S::Vf64::WIDTH {
            let a = S::Vf64::load_from_slice(lhs);
            let b = S::Vf64::load_from_slice(rhs);
            (a / b).copy_to_slice(out);

            lhs = &lhs[S::Vf64::WIDTH..];
            rhs = &rhs[S::Vf64::WIDTH..];
            out = &mut out[S::Vf64::WIDTH..];
        }

        for ((&a, &b), slot) in lhs.iter().zip(rhs.iter()).zip(out.iter_mut()) {
            *slot = a / b;
        }
    }
);

simd_unsafe_generate_all!(
    fn sqrt_f32(values: &[f32], out: &mut [f32]) {
        assert_eq!(values.len(), out.len());

        let mut values = values;
        let mut out = out;

        while values.len() >= S::Vf32::WIDTH {
            let a = S::Vf32::load_from_slice(values);
            a.sqrt().copy_to_slice(out);

            values = &values[S::Vf32::WIDTH..];
            out = &mut out[S::Vf32::WIDTH..];
        }

        for (&value, slot) in values.iter().zip(out.iter_mut()) {
            *slot = value.sqrt();
        }
    }
);

simd_unsafe_generate_all!(
    fn sqrt_f64(values: &[f64], out: &mut [f64]) {
        assert_eq!(values.len(), out.len());

        let mut values = values;
        let mut out = out;

        while values.len() >= S::Vf64::WIDTH {
            let a = S::Vf64::load_from_slice(values);
            a.sqrt().copy_to_slice(out);

            values = &values[S::Vf64::WIDTH..];
            out = &mut out[S::Vf64::WIDTH..];
        }

        for (&value, slot) in values.iter().zip(out.iter_mut()) {
            *slot = value.sqrt();
        }
    }
);

simd_unsafe_generate_all!(
    fn ceil_f32(values: &[f32], out: &mut [f32]) {
        assert_eq!(values.len(), out.len());

        let mut values = values;
        let mut out = out;

        while values.len() >= S::Vf32::WIDTH {
            let a = S::Vf32::load_from_slice(values);
            a.ceil().copy_to_slice(out);

            values = &values[S::Vf32::WIDTH..];
            out = &mut out[S::Vf32::WIDTH..];
        }

        for (&value, slot) in values.iter().zip(out.iter_mut()) {
            *slot = value.ceil();
        }
    }
);

simd_unsafe_generate_all!(
    fn floor_f32(values: &[f32], out: &mut [f32]) {
        assert_eq!(values.len(), out.len());

        let mut values = values;
        let mut out = out;

        while values.len() >= S::Vf32::WIDTH {
            let a = S::Vf32::load_from_slice(values);
            a.floor().copy_to_slice(out);

            values = &values[S::Vf32::WIDTH..];
            out = &mut out[S::Vf32::WIDTH..];
        }

        for (&value, slot) in values.iter().zip(out.iter_mut()) {
            *slot = value.floor();
        }
    }
);

simd_unsafe_generate_all!(
    fn round_f32(values: &[f32], out: &mut [f32]) {
        assert_eq!(values.len(), out.len());

        let mut values = values;
        let mut out = out;

        while values.len() >= S::Vf32::WIDTH {
            let a = S::Vf32::load_from_slice(values);
            a.round().copy_to_slice(out);

            values = &values[S::Vf32::WIDTH..];
            out = &mut out[S::Vf32::WIDTH..];
        }

        for (&value, slot) in values.iter().zip(out.iter_mut()) {
            *slot = value.round();
        }
    }
);

simd_unsafe_generate_all!(
    fn ceil_f64(values: &[f64], out: &mut [f64]) {
        assert_eq!(values.len(), out.len());

        let mut values = values;
        let mut out = out;

        while values.len() >= S::Vf64::WIDTH {
            let a = S::Vf64::load_from_slice(values);
            a.ceil().copy_to_slice(out);

            values = &values[S::Vf64::WIDTH..];
            out = &mut out[S::Vf64::WIDTH..];
        }

        for (&value, slot) in values.iter().zip(out.iter_mut()) {
            *slot = value.ceil();
        }
    }
);

simd_unsafe_generate_all!(
    fn floor_f64(values: &[f64], out: &mut [f64]) {
        assert_eq!(values.len(), out.len());

        let mut values = values;
        let mut out = out;

        while values.len() >= S::Vf64::WIDTH {
            let a = S::Vf64::load_from_slice(values);
            a.floor().copy_to_slice(out);

            values = &values[S::Vf64::WIDTH..];
            out = &mut out[S::Vf64::WIDTH..];
        }

        for (&value, slot) in values.iter().zip(out.iter_mut()) {
            *slot = value.floor();
        }
    }
);

simd_unsafe_generate_all!(
    fn round_f64(values: &[f64], out: &mut [f64]) {
        assert_eq!(values.len(), out.len());

        let mut values = values;
        let mut out = out;

        while values.len() >= S::Vf64::WIDTH {
            let a = S::Vf64::load_from_slice(values);
            a.round().copy_to_slice(out);

            values = &values[S::Vf64::WIDTH..];
            out = &mut out[S::Vf64::WIDTH..];
        }

        for (&value, slot) in values.iter().zip(out.iter_mut()) {
            *slot = value.round();
        }
    }
);

fn assert_f32_bitwise_results(actual: &[f32], expected: &[f32]) {
    assert_eq!(actual.len(), expected.len());
    for (&actual, &expected) in actual.iter().zip(expected.iter()) {
        assert_eq!(actual.to_bits(), expected.to_bits());
    }
}

fn assert_f64_bitwise_results(actual: &[f64], expected: &[f64]) {
    assert_eq!(actual.len(), expected.len());
    for (&actual, &expected) in actual.iter().zip(expected.iter()) {
        assert_eq!(actual.to_bits(), expected.to_bits());
    }
}

fn assert_f32_scalarish_results(actual: &[f32], expected: &[f32]) {
    assert_eq!(actual.len(), expected.len());
    for (&actual, &expected) in actual.iter().zip(expected.iter()) {
        if expected.is_nan() {
            assert!(actual.is_nan());
        } else {
            assert_eq!(actual, expected);
        }
    }
}

fn assert_f64_scalarish_results(actual: &[f64], expected: &[f64]) {
    assert_eq!(actual.len(), expected.len());
    for (&actual, &expected) in actual.iter().zip(expected.iter()) {
        if expected.is_nan() {
            assert!(actual.is_nan());
        } else {
            assert_eq!(actual, expected);
        }
    }
}

#[test]
fn float_min_max_nan_and_signed_zero_contract_for_f32() {
    let lhs = vec![f32::NAN, 3.0, -0.0, 0.0, -5.0, 8.0, f32::NAN, -2.0];
    let rhs = vec![4.0, f32::NAN, 0.0, -0.0, -5.0, 3.0, f32::NAN, -3.0];

    let mut min = vec![0.0; lhs.len()];
    let mut max = vec![0.0; lhs.len()];

    min_max_f32(&lhs, &rhs, &mut min, &mut max);

    for (((&a, &b), &min), &max) in lhs.iter().zip(rhs.iter()).zip(min.iter()).zip(max.iter()) {
        assert_f32_min_max_contract(a, b, min, max);
    }
}

#[test]
fn float_min_max_nan_and_signed_zero_contract_for_f64() {
    let lhs = vec![f64::NAN, 3.0, -0.0, 0.0, -5.0, 8.0, f64::NAN, -2.0];
    let rhs = vec![4.0, f64::NAN, 0.0, -0.0, -5.0, 3.0, f64::NAN, -3.0];

    let mut min = vec![0.0; lhs.len()];
    let mut max = vec![0.0; lhs.len()];

    min_max_f64(&lhs, &rhs, &mut min, &mut max);

    for (((&a, &b), &min), &max) in lhs.iter().zip(rhs.iter()).zip(min.iter()).zip(max.iter()) {
        assert_f64_min_max_contract(a, b, min, max);
    }
}

#[test]
fn float_rsqrt_contract_is_for_finite_positive_inputs_f32() {
    let input = vec![0.03125_f32, 0.125, 0.5, 1.0, 2.0, 9.0, 64.0, 255.0];
    let expected = input.iter().map(|&x| 1.0f32 / x.sqrt()).collect::<Vec<_>>();

    let mut output = vec![0.0; input.len()];
    rsqrt_f32(&input, &mut output);

    for (&actual, &expected) in output.iter().zip(expected.iter()) {
        let rel = (actual - expected).abs() / expected.max(1.0);
        assert!(
            rel <= 0.02,
            "f32 rsqrt relative error too large: actual={actual}, expected={expected}, rel={rel}"
        );
    }
}

#[test]
fn float_rsqrt_contract_is_for_finite_positive_inputs_f64() {
    let input = vec![0.03125_f64, 0.125, 0.5, 1.0, 2.0, 9.0, 64.0, 255.0];
    let expected = input.iter().map(|&x| 1.0f64 / x.sqrt()).collect::<Vec<_>>();

    let mut output = vec![0.0; input.len()];
    rsqrt_f64(&input, &mut output);

    for (&actual, &expected) in output.iter().zip(expected.iter()) {
        let rel = (actual - expected).abs() / expected.max(1.0);
        assert!(
            rel <= 0.02,
            "f64 rsqrt relative error too large: actual={actual}, expected={expected}, rel={rel}"
        );
    }
}

#[test]
fn float_div_special_semantics_match_scalar_bits_for_f32() {
    let lhs = vec![
        1.0f32,
        1.0,
        -1.0,
        0.0,
        -0.0,
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::NAN,
        5.0,
    ];
    let rhs = vec![
        0.0f32,
        -0.0,
        0.0,
        0.0,
        2.0,
        f32::INFINITY,
        f32::INFINITY,
        1.0,
        f32::NAN,
    ];
    let expected = lhs
        .iter()
        .zip(rhs.iter())
        .map(|(&a, &b)| a / b)
        .collect::<Vec<_>>();

    let mut actual = vec![0.0; lhs.len()];
    div_f32(&lhs, &rhs, &mut actual);
    assert_f32_bitwise_results(&actual, &expected);
}

#[test]
fn float_div_special_semantics_match_scalar_bits_for_f64() {
    let lhs = vec![
        1.0f64,
        1.0,
        -1.0,
        0.0,
        -0.0,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NAN,
        5.0,
    ];
    let rhs = vec![
        0.0f64,
        -0.0,
        0.0,
        0.0,
        2.0,
        f64::INFINITY,
        f64::INFINITY,
        1.0,
        f64::NAN,
    ];
    let expected = lhs
        .iter()
        .zip(rhs.iter())
        .map(|(&a, &b)| a / b)
        .collect::<Vec<_>>();

    let mut actual = vec![0.0; lhs.len()];
    div_f64(&lhs, &rhs, &mut actual);
    assert_f64_bitwise_results(&actual, &expected);
}

#[test]
fn float_sqrt_special_semantics_match_scalar_bits_for_f32() {
    let input = vec![
        -0.0f32,
        0.0,
        4.0,
        f32::MIN_POSITIVE,
        f32::INFINITY,
        -1.0,
        f32::NAN,
    ];
    let expected = input.iter().map(|&value| value.sqrt()).collect::<Vec<_>>();

    let mut actual = vec![0.0; input.len()];
    sqrt_f32(&input, &mut actual);
    assert_f32_bitwise_results(&actual, &expected);
}

#[test]
fn float_sqrt_special_semantics_match_scalar_bits_for_f64() {
    let input = vec![
        -0.0f64,
        0.0,
        4.0,
        f64::MIN_POSITIVE,
        f64::INFINITY,
        -1.0,
        f64::NAN,
    ];
    let expected = input.iter().map(|&value| value.sqrt()).collect::<Vec<_>>();

    let mut actual = vec![0.0; input.len()];
    sqrt_f64(&input, &mut actual);
    assert_f64_bitwise_results(&actual, &expected);
}

#[test]
fn float_rounding_special_semantics_match_scalar_bits_for_f32() {
    let input = vec![
        -0.7f32,
        -0.75,
        -0.3,
        -0.0,
        0.0,
        0.3,
        0.75,
        0.7,
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::NAN,
    ];

    let expected_ceil = input.iter().map(|&value| value.ceil()).collect::<Vec<_>>();
    let expected_floor = input.iter().map(|&value| value.floor()).collect::<Vec<_>>();
    let expected_round = input.iter().map(|&value| value.round()).collect::<Vec<_>>();

    let mut actual_ceil = vec![0.0; input.len()];
    let mut actual_floor = vec![0.0; input.len()];
    let mut actual_round = vec![0.0; input.len()];

    ceil_f32(&input, &mut actual_ceil);
    floor_f32(&input, &mut actual_floor);
    round_f32(&input, &mut actual_round);

    assert_f32_scalarish_results(&actual_ceil, &expected_ceil);
    assert_f32_scalarish_results(&actual_floor, &expected_floor);
    assert_f32_scalarish_results(&actual_round, &expected_round);
}

#[test]
fn float_rounding_special_semantics_match_scalar_bits_for_f64() {
    let input = vec![
        -0.7f64,
        -0.75,
        -0.3,
        -0.0,
        0.0,
        0.3,
        0.75,
        0.7,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NAN,
    ];

    let expected_ceil = input.iter().map(|&value| value.ceil()).collect::<Vec<_>>();
    let expected_floor = input.iter().map(|&value| value.floor()).collect::<Vec<_>>();
    let expected_round = input.iter().map(|&value| value.round()).collect::<Vec<_>>();

    let mut actual_ceil = vec![0.0; input.len()];
    let mut actual_floor = vec![0.0; input.len()];
    let mut actual_round = vec![0.0; input.len()];

    ceil_f64(&input, &mut actual_ceil);
    floor_f64(&input, &mut actual_floor);
    round_f64(&input, &mut actual_round);

    assert_f64_scalarish_results(&actual_ceil, &expected_ceil);
    assert_f64_scalarish_results(&actual_floor, &expected_floor);
    assert_f64_scalarish_results(&actual_round, &expected_round);
}
