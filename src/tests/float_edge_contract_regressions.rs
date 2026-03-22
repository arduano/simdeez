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
