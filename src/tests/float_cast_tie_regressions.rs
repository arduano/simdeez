#![allow(dead_code, unused_parens)]

use crate::prelude::*;

simd_unsafe_generate_all!(
    fn cast_f32_to_i32(values: &[f32], out: &mut [i32]) {
        assert_eq!(values.len(), out.len());

        let mut values = values;
        let mut out = out;

        while values.len() >= S::Vf32::WIDTH {
            let a = S::Vf32::load_from_slice(values);
            a.cast_i32().copy_to_slice(out);

            values = &values[S::Vf32::WIDTH..];
            out = &mut out[S::Vf32::WIDTH..];
        }

        for (&value, slot) in values.iter().zip(out.iter_mut()) {
            *slot = value.round_ties_even() as i32;
        }
    }
);

simd_unsafe_generate_all!(
    fn cast_f64_to_i64(values: &[f64], out: &mut [i64]) {
        assert_eq!(values.len(), out.len());

        let mut values = values;
        let mut out = out;

        while values.len() >= S::Vf64::WIDTH {
            let a = S::Vf64::load_from_slice(values);
            a.cast_i64().copy_to_slice(out);

            values = &values[S::Vf64::WIDTH..];
            out = &mut out[S::Vf64::WIDTH..];
        }

        for (&value, slot) in values.iter().zip(out.iter_mut()) {
            *slot = value.round_ties_even() as i64;
        }
    }
);

fn run_f32(values: &[f32], f: impl Fn(&[f32], &mut [i32])) -> Vec<i32> {
    let mut out = vec![0; values.len()];
    f(values, &mut out);
    out
}

fn run_f64(values: &[f64], f: impl Fn(&[f64], &mut [i64])) -> Vec<i64> {
    let mut out = vec![0; values.len()];
    f(values, &mut out);
    out
}

#[test]
fn float_cast_i32_ties_to_even_across_backends() {
    let values: Vec<f32> = vec![
        -5.5, -4.5, -3.5, -2.5, -1.5, -0.5, -0.499, 0.499, 0.5, 1.5, 2.5, 3.5, 4.5, 5.5,
    ];
    let expected = values
        .iter()
        .map(|&value| value.round_ties_even() as i32)
        .collect::<Vec<_>>();

    assert_eq!(run_f32(&values, cast_f32_to_i32), expected);

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
        if std::arch::is_x86_feature_detected!("sse2") {
            assert_eq!(
                run_f32(&values, |values, out| unsafe {
                    cast_f32_to_i32_sse2(values, out)
                }),
                expected
            );
        }
        if std::arch::is_x86_feature_detected!("sse4.1") {
            assert_eq!(
                run_f32(&values, |values, out| unsafe {
                    cast_f32_to_i32_sse41(values, out)
                }),
                expected
            );
        }
        if std::arch::is_x86_feature_detected!("avx2") && std::arch::is_x86_feature_detected!("fma")
        {
            assert_eq!(
                run_f32(&values, |values, out| unsafe {
                    cast_f32_to_i32_avx2(values, out)
                }),
                expected
            );
        }
    }
}

#[test]
fn float_cast_i32_respects_defined_range_boundaries_across_backends() {
    let lower = i32::MIN as f32;
    let upper_exclusive = -(i32::MIN as f32);
    let values = vec![
        lower,
        f32::from_bits(lower.to_bits() + 1),
        -65_537.5,
        -0.5,
        0.5,
        65_537.5,
        f32::from_bits(upper_exclusive.to_bits() - 2),
        f32::from_bits(upper_exclusive.to_bits() - 1),
    ];
    let expected = values
        .iter()
        .map(|&value| value.round_ties_even() as i32)
        .collect::<Vec<_>>();

    assert_eq!(run_f32(&values, cast_f32_to_i32), expected);

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
        if std::arch::is_x86_feature_detected!("sse2") {
            assert_eq!(
                run_f32(&values, |values, out| unsafe {
                    cast_f32_to_i32_sse2(values, out)
                }),
                expected
            );
        }
        if std::arch::is_x86_feature_detected!("sse4.1") {
            assert_eq!(
                run_f32(&values, |values, out| unsafe {
                    cast_f32_to_i32_sse41(values, out)
                }),
                expected
            );
        }
        if std::arch::is_x86_feature_detected!("avx2") && std::arch::is_x86_feature_detected!("fma")
        {
            assert_eq!(
                run_f32(&values, |values, out| unsafe {
                    cast_f32_to_i32_avx2(values, out)
                }),
                expected
            );
        }
    }
}

#[test]
fn float_cast_i64_ties_to_even_across_backends() {
    let values: Vec<f64> = vec![
        -5.5, -4.5, -3.5, -2.5, -1.5, -0.5, -0.499, 0.499, 0.5, 1.5, 2.5, 3.5, 4.5, 5.5,
    ];
    let expected = values
        .iter()
        .map(|&value| value.round_ties_even() as i64)
        .collect::<Vec<_>>();

    assert_eq!(run_f64(&values, cast_f64_to_i64), expected);

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
        if std::arch::is_x86_feature_detected!("sse2") {
            assert_eq!(
                run_f64(&values, |values, out| unsafe {
                    cast_f64_to_i64_sse2(values, out)
                }),
                expected
            );
        }
        if std::arch::is_x86_feature_detected!("sse4.1") {
            assert_eq!(
                run_f64(&values, |values, out| unsafe {
                    cast_f64_to_i64_sse41(values, out)
                }),
                expected
            );
        }
        if std::arch::is_x86_feature_detected!("avx2") && std::arch::is_x86_feature_detected!("fma")
        {
            assert_eq!(
                run_f64(&values, |values, out| unsafe {
                    cast_f64_to_i64_avx2(values, out)
                }),
                expected
            );
        }
    }
}

#[test]
fn float_cast_i64_respects_defined_range_boundaries_across_backends() {
    let lower = i64::MIN as f64;
    let upper_exclusive = -(i64::MIN as f64);
    let values = vec![
        lower,
        f64::from_bits(lower.to_bits() + 1),
        -4_503_599_627_370_495.5,
        -0.5,
        0.5,
        4_503_599_627_370_495.5,
        f64::from_bits(upper_exclusive.to_bits() - 2),
        f64::from_bits(upper_exclusive.to_bits() - 1),
    ];
    let expected = values
        .iter()
        .map(|&value| value.round_ties_even() as i64)
        .collect::<Vec<_>>();

    assert_eq!(run_f64(&values, cast_f64_to_i64), expected);

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
        if std::arch::is_x86_feature_detected!("sse2") {
            assert_eq!(
                run_f64(&values, |values, out| unsafe {
                    cast_f64_to_i64_sse2(values, out)
                }),
                expected
            );
        }
        if std::arch::is_x86_feature_detected!("sse4.1") {
            assert_eq!(
                run_f64(&values, |values, out| unsafe {
                    cast_f64_to_i64_sse41(values, out)
                }),
                expected
            );
        }
        if std::arch::is_x86_feature_detected!("avx2") && std::arch::is_x86_feature_detected!("fma")
        {
            assert_eq!(
                run_f64(&values, |values, out| unsafe {
                    cast_f64_to_i64_avx2(values, out)
                }),
                expected
            );
        }
    }
}
