#![allow(dead_code, unused_parens)]

use crate::prelude::*;

fn mask(value: bool) -> i64 {
    if value {
        u64::MAX as i64
    } else {
        0
    }
}

fn bits(value: u64) -> i64 {
    value as i64
}

struct CompareOutputs {
    gt: Vec<i64>,
    eq: Vec<i64>,
    min: Vec<i64>,
    max: Vec<i64>,
}

fn reference_compare_i64(lhs: &[i64], rhs: &[i64]) -> CompareOutputs {
    CompareOutputs {
        gt: lhs
            .iter()
            .zip(rhs.iter())
            .map(|(&a, &b)| mask(a > b))
            .collect(),
        eq: lhs
            .iter()
            .zip(rhs.iter())
            .map(|(&a, &b)| mask(a == b))
            .collect(),
        min: lhs
            .iter()
            .zip(rhs.iter())
            .map(|(&a, &b)| a.min(b))
            .collect(),
        max: lhs
            .iter()
            .zip(rhs.iter())
            .map(|(&a, &b)| a.max(b))
            .collect(),
    }
}

simd_unsafe_generate_all!(
    fn compare_i64_edges(
        lhs: &[i64],
        rhs: &[i64],
        gt_out: &mut [i64],
        eq_out: &mut [i64],
        min_out: &mut [i64],
        max_out: &mut [i64],
    ) {
        assert_eq!(lhs.len(), rhs.len());
        assert_eq!(lhs.len(), gt_out.len());
        assert_eq!(lhs.len(), eq_out.len());
        assert_eq!(lhs.len(), min_out.len());
        assert_eq!(lhs.len(), max_out.len());

        let mut lhs = lhs;
        let mut rhs = rhs;
        let mut gt_out = gt_out;
        let mut eq_out = eq_out;
        let mut min_out = min_out;
        let mut max_out = max_out;

        while lhs.len() >= S::Vi64::WIDTH {
            let a = S::Vi64::load_from_slice(lhs);
            let b = S::Vi64::load_from_slice(rhs);
            a.cmp_gt(b).copy_to_slice(gt_out);
            a.cmp_eq(b).copy_to_slice(eq_out);
            a.min(b).copy_to_slice(min_out);
            a.max(b).copy_to_slice(max_out);

            lhs = &lhs[S::Vi64::WIDTH..];
            rhs = &rhs[S::Vi64::WIDTH..];
            gt_out = &mut gt_out[S::Vi64::WIDTH..];
            eq_out = &mut eq_out[S::Vi64::WIDTH..];
            min_out = &mut min_out[S::Vi64::WIDTH..];
            max_out = &mut max_out[S::Vi64::WIDTH..];
        }

        for (((((&a, &b), gt_slot), eq_slot), min_slot), max_slot) in lhs
            .iter()
            .zip(rhs.iter())
            .zip(gt_out.iter_mut())
            .zip(eq_out.iter_mut())
            .zip(min_out.iter_mut())
            .zip(max_out.iter_mut())
        {
            *gt_slot = mask(a > b);
            *eq_slot = mask(a == b);
            *min_slot = a.min(b);
            *max_slot = a.max(b);
        }
    }
);

fn assert_compare_outputs(actual: CompareOutputs, expected: &CompareOutputs) {
    assert_eq!(actual.gt, expected.gt);
    assert_eq!(actual.eq, expected.eq);
    assert_eq!(actual.min, expected.min);
    assert_eq!(actual.max, expected.max);
}

fn run_compare_i64(
    lhs: &[i64],
    rhs: &[i64],
    f: impl Fn(&[i64], &[i64], &mut [i64], &mut [i64], &mut [i64], &mut [i64]),
) -> CompareOutputs {
    let mut gt = vec![0; lhs.len()];
    let mut eq = vec![0; lhs.len()];
    let mut min = vec![0; lhs.len()];
    let mut max = vec![0; lhs.len()];
    f(lhs, rhs, &mut gt, &mut eq, &mut min, &mut max);
    CompareOutputs { gt, eq, min, max }
}

// This directly targets the compare/min/max paths used by the SSE2/SSE4.1 i64 backends,
// especially cases where the top 32 bits are equal and the low 32 bits decide signed ordering.
#[test]
fn i64_backend_compare_regression_cases() {
    let lhs = vec![
        i64::MIN,
        i64::MIN + 1,
        bits(0xffff_ffff_0000_0000),
        bits(0xffff_ffff_7fff_ffff),
        bits(0xffff_ffff_8000_0000),
        -17,
        -1,
        0,
        1,
        17,
        bits(0x0000_0001_0000_0000),
        bits(0x0000_0001_0000_0001),
        bits(0x0000_0001_ffff_ffff),
        i64::MAX - 1,
        i64::MAX,
    ];
    let rhs = vec![
        i64::MIN + 1,
        i64::MIN,
        bits(0xffff_ffff_0000_0001),
        bits(0xffff_ffff_7fff_fffe),
        bits(0xffff_ffff_ffff_ffff),
        -18,
        -1,
        1,
        0,
        17,
        bits(0x0000_0001_ffff_ffff),
        bits(0x0000_0001_0000_0000),
        bits(0x0000_0001_0000_0001),
        i64::MAX,
        i64::MAX - 1,
    ];
    let expected = reference_compare_i64(&lhs, &rhs);

    assert_compare_outputs(run_compare_i64(&lhs, &rhs, compare_i64_edges), &expected);

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
        if std::arch::is_x86_feature_detected!("sse2") {
            assert_compare_outputs(
                run_compare_i64(&lhs, &rhs, |lhs, rhs, gt, eq, min, max| unsafe {
                    compare_i64_edges_sse2(lhs, rhs, gt, eq, min, max)
                }),
                &expected,
            );
        }

        if std::arch::is_x86_feature_detected!("sse4.1") {
            assert_compare_outputs(
                run_compare_i64(&lhs, &rhs, |lhs, rhs, gt, eq, min, max| unsafe {
                    compare_i64_edges_sse41(lhs, rhs, gt, eq, min, max)
                }),
                &expected,
            );
        }
    }
}
