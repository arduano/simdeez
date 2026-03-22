#![allow(dead_code, unused_parens)]

use crate::prelude::*;

fn mask_f32(value: bool) -> f32 {
    if value {
        f32::from_bits(u32::MAX)
    } else {
        0.0
    }
}

fn mask_f64(value: bool) -> f64 {
    if value {
        f64::from_bits(u64::MAX)
    } else {
        0.0
    }
}

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

simd_unsafe_generate_all!(
    fn compare_neq_mask_f32(lhs: &[f32], rhs: &[f32], out: &mut [f32]) {
        assert_eq!(lhs.len(), rhs.len());
        assert_eq!(lhs.len(), out.len());

        let mut lhs = lhs;
        let mut rhs = rhs;
        let mut out = out;

        while lhs.len() >= S::Vf32::WIDTH {
            let a = S::Vf32::load_from_slice(lhs);
            let b = S::Vf32::load_from_slice(rhs);
            a.cmp_neq(b).copy_to_slice(out);

            lhs = &lhs[S::Vf32::WIDTH..];
            rhs = &rhs[S::Vf32::WIDTH..];
            out = &mut out[S::Vf32::WIDTH..];
        }

        for ((&a, &b), slot) in lhs.iter().zip(rhs.iter()).zip(out.iter_mut()) {
            *slot = mask_f32(a != b);
        }
    }
);

simd_unsafe_generate_all!(
    fn compare_neq_mask_f64(lhs: &[f64], rhs: &[f64], out: &mut [f64]) {
        assert_eq!(lhs.len(), rhs.len());
        assert_eq!(lhs.len(), out.len());

        let mut lhs = lhs;
        let mut rhs = rhs;
        let mut out = out;

        while lhs.len() >= S::Vf64::WIDTH {
            let a = S::Vf64::load_from_slice(lhs);
            let b = S::Vf64::load_from_slice(rhs);
            a.cmp_neq(b).copy_to_slice(out);

            lhs = &lhs[S::Vf64::WIDTH..];
            rhs = &rhs[S::Vf64::WIDTH..];
            out = &mut out[S::Vf64::WIDTH..];
        }

        for ((&a, &b), slot) in lhs.iter().zip(rhs.iter()).zip(out.iter_mut()) {
            *slot = mask_f64(a != b);
        }
    }
);

fn run_f32(lhs: &[f32], rhs: &[f32], f: impl Fn(&[f32], &[f32], &mut [f32])) -> Vec<f32> {
    let mut out = vec![0.0; lhs.len()];
    f(lhs, rhs, &mut out);
    out
}

fn run_f64(lhs: &[f64], rhs: &[f64], f: impl Fn(&[f64], &[f64], &mut [f64])) -> Vec<f64> {
    let mut out = vec![0.0; lhs.len()];
    f(lhs, rhs, &mut out);
    out
}

// This directly targets the NaN semantics of cmp_neq so every backend agrees with scalar `!=`,
// including cases like `NaN != finite` and `NaN != NaN`.
#[test]
fn float_cmp_neq_nan_semantics_match_scalar_for_f32() {
    let lhs = vec![
        f32::NAN,
        f32::NAN,
        -f32::NAN,
        f32::INFINITY,
        1.0,
        -0.0,
        7.0,
        3.5,
    ];
    let rhs = vec![f32::NAN, 1.0, -5.0, f32::INFINITY, 1.0, 0.0, f32::NAN, -2.0];
    let expected = lhs
        .iter()
        .zip(rhs.iter())
        .map(|(&a, &b)| mask_f32(a != b))
        .collect::<Vec<_>>();

    assert_f32_bits_eq(&run_f32(&lhs, &rhs, compare_neq_mask_f32), &expected);

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
        if std::arch::is_x86_feature_detected!("sse2") {
            assert_f32_bits_eq(
                &run_f32(&lhs, &rhs, |lhs, rhs, out| unsafe {
                    compare_neq_mask_f32_sse2(lhs, rhs, out)
                }),
                &expected,
            );
        }
        if std::arch::is_x86_feature_detected!("sse4.1") {
            assert_f32_bits_eq(
                &run_f32(&lhs, &rhs, |lhs, rhs, out| unsafe {
                    compare_neq_mask_f32_sse41(lhs, rhs, out)
                }),
                &expected,
            );
        }
        if std::arch::is_x86_feature_detected!("avx2") && std::arch::is_x86_feature_detected!("fma")
        {
            assert_f32_bits_eq(
                &run_f32(&lhs, &rhs, |lhs, rhs, out| unsafe {
                    compare_neq_mask_f32_avx2(lhs, rhs, out)
                }),
                &expected,
            );
        }
    }
}

// Same regression for f64 so AVX compare predicates match the scalar contract there too.
#[test]
fn float_cmp_neq_nan_semantics_match_scalar_for_f64() {
    let lhs = vec![
        f64::NAN,
        f64::NAN,
        -f64::NAN,
        f64::INFINITY,
        1.0,
        -0.0,
        7.0,
        3.5,
    ];
    let rhs = vec![f64::NAN, 1.0, -5.0, f64::INFINITY, 1.0, 0.0, f64::NAN, -2.0];
    let expected = lhs
        .iter()
        .zip(rhs.iter())
        .map(|(&a, &b)| mask_f64(a != b))
        .collect::<Vec<_>>();

    assert_f64_bits_eq(&run_f64(&lhs, &rhs, compare_neq_mask_f64), &expected);

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
        if std::arch::is_x86_feature_detected!("sse2") {
            assert_f64_bits_eq(
                &run_f64(&lhs, &rhs, |lhs, rhs, out| unsafe {
                    compare_neq_mask_f64_sse2(lhs, rhs, out)
                }),
                &expected,
            );
        }
        if std::arch::is_x86_feature_detected!("sse4.1") {
            assert_f64_bits_eq(
                &run_f64(&lhs, &rhs, |lhs, rhs, out| unsafe {
                    compare_neq_mask_f64_sse41(lhs, rhs, out)
                }),
                &expected,
            );
        }
        if std::arch::is_x86_feature_detected!("avx2") && std::arch::is_x86_feature_detected!("fma")
        {
            assert_f64_bits_eq(
                &run_f64(&lhs, &rhs, |lhs, rhs, out| unsafe {
                    compare_neq_mask_f64_avx2(lhs, rhs, out)
                }),
                &expected,
            );
        }
    }
}
