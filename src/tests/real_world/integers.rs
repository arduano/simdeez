use crate::prelude::*;

fn reference_adaptive_select_i32(lhs: &[i32], rhs: &[i32]) -> Vec<i32> {
    lhs.iter()
        .zip(rhs.iter())
        .map(|(&left, &right)| {
            let hi = left.max(right);
            let lo = left.min(right);
            let spread = hi - lo;
            let expanded = hi.min(4089) + 7;
            let compressed = lo.max(-4089) - 7;
            if spread > 1024 {
                expanded
            } else {
                compressed
            }
        })
        .collect()
}

simd_unsafe_generate_all!(
    fn adaptive_select_i32(lhs: &[i32], rhs: &[i32], output: &mut [i32]) {
        assert_eq!(lhs.len(), rhs.len());
        assert_eq!(lhs.len(), output.len());

        let mut lhs = lhs;
        let mut rhs = rhs;
        let mut output = output;
        let threshold = S::Vi32::set1(1024);
        let upper_pre_add = S::Vi32::set1(4089);
        let lower_pre_sub = S::Vi32::set1(-4089);

        while lhs.len() >= S::Vi32::WIDTH {
            let left = S::Vi32::load_from_slice(lhs);
            let right = S::Vi32::load_from_slice(rhs);
            let hi = left.max(right);
            let lo = left.min(right);
            let spread = hi - lo;
            let expanded = hi.min(upper_pre_add) + 7;
            let compressed = lo.max(lower_pre_sub) - 7;
            let selected = spread.cmp_gt(threshold).blendv(compressed, expanded);
            selected.copy_to_slice(output);

            lhs = &lhs[S::Vi32::WIDTH..];
            rhs = &rhs[S::Vi32::WIDTH..];
            output = &mut output[S::Vi32::WIDTH..];
        }

        for ((&left, &right), slot) in lhs.iter().zip(rhs.iter()).zip(output.iter_mut()) {
            let hi = left.max(right);
            let lo = left.min(right);
            let spread = hi - lo;
            let expanded = hi.min(4089) + 7;
            let compressed = lo.max(-4089) - 7;
            *slot = if spread > 1024 { expanded } else { compressed };
        }
    }
);

fn assert_adaptive_select_matches_all_backends(lhs: &[i32], rhs: &[i32]) {
    let expected = reference_adaptive_select_i32(lhs, rhs);

    let mut scalar = vec![0; lhs.len()];
    adaptive_select_i32(lhs, rhs, &mut scalar);
    assert_eq!(scalar, expected);

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
        if std::arch::is_x86_feature_detected!("sse2") {
            let mut output = vec![0; lhs.len()];
            unsafe { adaptive_select_i32_sse2(lhs, rhs, &mut output) };
            assert_eq!(output, expected);
        }
        if std::arch::is_x86_feature_detected!("sse4.1") {
            let mut output = vec![0; lhs.len()];
            unsafe { adaptive_select_i32_sse41(lhs, rhs, &mut output) };
            assert_eq!(output, expected);
        }
        if std::arch::is_x86_feature_detected!("avx2") && std::arch::is_x86_feature_detected!("fma")
        {
            let mut output = vec![0; lhs.len()];
            unsafe { adaptive_select_i32_avx2(lhs, rhs, &mut output) };
            assert_eq!(output, expected);
        }
    }
}

// This resembles integer decision stages in codecs and DSP control paths where a fast per-lane
// spread check chooses between compressing noise and preserving strong transients.
#[test]
fn real_world_adaptive_select_i32_matches_reference() {
    let lhs = [
        -5000, -4096, -4090, -2048, -1025, -1024, -17, -1, 0, 1, 9, 17, 1023, 1024, 2047, 4090,
        4096, 5000,
    ]
    .into_iter()
    .cycle()
    .take(61)
    .enumerate()
    .map(|(i, value)| value + ((i % 7) as i32 * 3 - 9))
    .collect::<Vec<_>>();
    let rhs = lhs
        .iter()
        .enumerate()
        .map(|(i, &value)| {
            let delta = match i % 6 {
                0 => -1500,
                1 => -1024,
                2 => -17,
                3 => 17,
                4 => 1024,
                _ => 1500,
            };
            value.wrapping_neg().clamp(-5000, 5000) + delta
        })
        .collect::<Vec<_>>();

    assert_adaptive_select_matches_all_backends(&[], &[]);
    assert_adaptive_select_matches_all_backends(&lhs[..5], &rhs[..5]);
    assert_adaptive_select_matches_all_backends(&lhs[..19], &rhs[..19]);
    assert_adaptive_select_matches_all_backends(&lhs, &rhs);
}
