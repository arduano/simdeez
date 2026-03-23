#![allow(dead_code, unused_parens)]

use crate::prelude::*;

fn assert_f32_bits_eq(actual: &[f32], expected: &[f32]) {
    assert_eq!(actual.len(), expected.len());
    for (actual, expected) in actual.iter().zip(expected.iter()) {
        assert_eq!(actual.to_bits(), expected.to_bits());
    }
}

fn assert_f64_bits_eq(actual: &[f64], expected: &[f64]) {
    assert_eq!(actual.len(), expected.len());
    for (actual, expected) in actual.iter().zip(expected.iter()) {
        assert_eq!(actual.to_bits(), expected.to_bits());
    }
}

fn assert_f32_scalarish_eq(actual: &[f32], expected: &[f32]) {
    assert_eq!(actual.len(), expected.len());
    for (actual, expected) in actual.iter().zip(expected.iter()) {
        if expected.is_nan() {
            assert!(actual.is_nan());
        } else {
            assert_eq!(*actual, *expected);
        }
    }
}

fn assert_f64_scalarish_eq(actual: &[f64], expected: &[f64]) {
    assert_eq!(actual.len(), expected.len());
    for (actual, expected) in actual.iter().zip(expected.iter()) {
        if expected.is_nan() {
            assert!(actual.is_nan());
        } else {
            assert_eq!(*actual, *expected);
        }
    }
}

simd_unsafe_generate_all!(
    fn div_f32_edges(lhs: &[f32], rhs: &[f32], out: &mut [f32]) {
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
    fn div_f64_edges(lhs: &[f64], rhs: &[f64], out: &mut [f64]) {
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
    fn ceil_f32_edges(values: &[f32], out: &mut [f32]) {
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
    fn floor_f32_edges(values: &[f32], out: &mut [f32]) {
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
    fn round_f32_edges(values: &[f32], out: &mut [f32]) {
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
    fn sqrt_f32_edges(values: &[f32], out: &mut [f32]) {
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
    fn ceil_f64_edges(values: &[f64], out: &mut [f64]) {
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
    fn floor_f64_edges(values: &[f64], out: &mut [f64]) {
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
    fn round_f64_edges(values: &[f64], out: &mut [f64]) {
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

simd_unsafe_generate_all!(
    fn sqrt_f64_edges(values: &[f64], out: &mut [f64]) {
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

fn run_f32_unary(values: &[f32], f: impl Fn(&[f32], &mut [f32])) -> Vec<f32> {
    let mut out = vec![0.0; values.len()];
    f(values, &mut out);
    out
}

fn run_f64_unary(values: &[f64], f: impl Fn(&[f64], &mut [f64])) -> Vec<f64> {
    let mut out = vec![0.0; values.len()];
    f(values, &mut out);
    out
}

fn run_f32_binary(lhs: &[f32], rhs: &[f32], f: impl Fn(&[f32], &[f32], &mut [f32])) -> Vec<f32> {
    let mut out = vec![0.0; lhs.len()];
    f(lhs, rhs, &mut out);
    out
}

fn run_f64_binary(lhs: &[f64], rhs: &[f64], f: impl Fn(&[f64], &[f64], &mut [f64])) -> Vec<f64> {
    let mut out = vec![0.0; lhs.len()];
    f(lhs, rhs, &mut out);
    out
}

#[test]
fn float_div_special_values_match_scalar_for_f32() {
    let lhs = vec![
        -0.0,
        0.0,
        1.0,
        -1.0,
        f32::MIN_POSITIVE,
        f32::from_bits(1),
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::NAN,
        9.0,
    ];
    let rhs = vec![
        1.0,
        -1.0,
        0.0,
        -0.0,
        f32::MIN_POSITIVE,
        -f32::from_bits(1),
        f32::INFINITY,
        f32::NEG_INFINITY,
        1.0,
        f32::NAN,
    ];
    let expected = lhs
        .iter()
        .zip(rhs.iter())
        .map(|(&a, &b)| a / b)
        .collect::<Vec<_>>();

    assert_f32_bits_eq(&run_f32_binary(&lhs, &rhs, div_f32_edges), &expected);
}

#[test]
fn float_div_special_values_match_scalar_for_f64() {
    let lhs = vec![
        -0.0,
        0.0,
        1.0,
        -1.0,
        f64::MIN_POSITIVE,
        f64::from_bits(1),
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NAN,
        9.0,
    ];
    let rhs = vec![
        1.0,
        -1.0,
        0.0,
        -0.0,
        f64::MIN_POSITIVE,
        -f64::from_bits(1),
        f64::INFINITY,
        f64::NEG_INFINITY,
        1.0,
        f64::NAN,
    ];
    let expected = lhs
        .iter()
        .zip(rhs.iter())
        .map(|(&a, &b)| a / b)
        .collect::<Vec<_>>();

    assert_f64_bits_eq(&run_f64_binary(&lhs, &rhs, div_f64_edges), &expected);
}

#[test]
fn float_rounding_special_values_match_scalar_for_f32() {
    let values = vec![
        -f32::from_bits(1),
        -f32::MIN_POSITIVE,
        -1.5,
        -1.0,
        -0.75,
        -0.25,
        -0.0,
        0.0,
        0.25,
        0.75,
        1.0,
        1.5,
        8_388_608.0,
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::NAN,
    ];

    assert_f32_bits_eq(
        &run_f32_unary(&values, ceil_f32_edges),
        &values.iter().map(|&v| v.ceil()).collect::<Vec<_>>(),
    );
    assert_f32_scalarish_eq(
        &run_f32_unary(&values, floor_f32_edges),
        &values.iter().map(|&v| v.floor()).collect::<Vec<_>>(),
    );
    assert_f32_scalarish_eq(
        &run_f32_unary(&values, round_f32_edges),
        &values.iter().map(|&v| v.round()).collect::<Vec<_>>(),
    );
}

#[test]
fn float_rounding_special_values_match_scalar_for_f64() {
    let values = vec![
        -f64::from_bits(1),
        -f64::MIN_POSITIVE,
        -1.5,
        -1.0,
        -0.75,
        -0.25,
        -0.0,
        0.0,
        0.25,
        0.75,
        1.0,
        1.5,
        4_503_599_627_370_496.0,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NAN,
    ];

    assert_f64_scalarish_eq(
        &run_f64_unary(&values, ceil_f64_edges),
        &values.iter().map(|&v| v.ceil()).collect::<Vec<_>>(),
    );
    assert_f64_scalarish_eq(
        &run_f64_unary(&values, floor_f64_edges),
        &values.iter().map(|&v| v.floor()).collect::<Vec<_>>(),
    );
    assert_f64_scalarish_eq(
        &run_f64_unary(&values, round_f64_edges),
        &values.iter().map(|&v| v.round()).collect::<Vec<_>>(),
    );
}

#[test]
fn float_sqrt_special_values_match_scalar_for_f32() {
    let values = vec![
        -f32::INFINITY,
        -4.0,
        -f32::MIN_POSITIVE,
        -0.0,
        0.0,
        f32::from_bits(1),
        f32::MIN_POSITIVE,
        4.0,
        f32::INFINITY,
        f32::NAN,
    ];

    assert_f32_bits_eq(
        &run_f32_unary(&values, sqrt_f32_edges),
        &values.iter().map(|&v| v.sqrt()).collect::<Vec<_>>(),
    );
}

#[test]
fn float_sqrt_special_values_match_scalar_for_f64() {
    let values = vec![
        -f64::INFINITY,
        -4.0,
        -f64::MIN_POSITIVE,
        -0.0,
        0.0,
        f64::from_bits(1),
        f64::MIN_POSITIVE,
        4.0,
        f64::INFINITY,
        f64::NAN,
    ];

    assert_f64_bits_eq(
        &run_f64_unary(&values, sqrt_f64_edges),
        &values.iter().map(|&v| v.sqrt()).collect::<Vec<_>>(),
    );
}
