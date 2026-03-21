use crate::prelude::*;

use super::shared::assert_f32_slices_close;

fn reference_quantize_f32(input: &[f32]) -> Vec<i32> {
    input
        .iter()
        .map(|&value| (value.abs() * 1.75 + 0.125).floor().clamp(0.0, 255.0) as i32)
        .collect()
}

fn reference_normalize_luma_f32(input: &[f32]) -> Vec<f32> {
    input
        .iter()
        .map(|&value| {
            let normalized = ((value - 16.0) * (1.0 / 219.0)).clamp(0.0, 1.0);
            let lifted = (normalized * 1.12).clamp(0.0, 1.0);
            let softened = normalized * 0.9;
            if normalized > 0.62 {
                lifted
            } else {
                softened
            }
        })
        .collect()
}

simd_unsafe_generate_all!(
    fn quantize_f32_to_i32(input: &[f32], output: &mut [i32]) {
        assert_eq!(input.len(), output.len());

        let mut input = input;
        let mut output = output;
        let scale = S::Vf32::set1(1.75);
        let bias = S::Vf32::set1(0.125);
        let lower = S::Vf32::set1(0.0);
        let upper = S::Vf32::set1(255.0);

        while input.len() >= S::Vf32::WIDTH {
            let values = S::Vf32::load_from_slice(input);
            let quantized = values
                .abs()
                .mul_add(scale, bias)
                .floor()
                .max(lower)
                .min(upper)
                .cast_i32();
            quantized.copy_to_slice(output);

            input = &input[S::Vf32::WIDTH..];
            output = &mut output[S::Vf32::WIDTH..];
        }

        for (value, slot) in input.iter().zip(output.iter_mut()) {
            *slot = (value.abs() * 1.75 + 0.125).floor().clamp(0.0, 255.0) as i32;
        }
    }
);

simd_unsafe_generate_all!(
    fn normalize_luma_f32(input: &[f32], output: &mut [f32]) {
        assert_eq!(input.len(), output.len());

        let mut input = input;
        let mut output = output;
        let black_level = S::Vf32::set1(16.0);
        let inv_range = S::Vf32::set1(1.0 / 219.0);
        let zero = S::Vf32::set1(0.0);
        let one = S::Vf32::set1(1.0);
        let threshold = S::Vf32::set1(0.62);
        let lift = S::Vf32::set1(1.12);
        let soften = S::Vf32::set1(0.9);

        while input.len() >= S::Vf32::WIDTH {
            let values = S::Vf32::load_from_slice(input);
            let normalized = (values - black_level).mul(inv_range).max(zero).min(one);
            let lifted = normalized.mul(lift).min(one);
            let softened = normalized.mul(soften);
            let output_chunk = normalized.cmp_gt(threshold).blendv(softened, lifted);
            output_chunk.copy_to_slice(output);

            input = &input[S::Vf32::WIDTH..];
            output = &mut output[S::Vf32::WIDTH..];
        }

        for (value, slot) in input.iter().zip(output.iter_mut()) {
            let normalized = ((value - 16.0) * (1.0 / 219.0)).clamp(0.0, 1.0);
            let lifted = (normalized * 1.12).clamp(0.0, 1.0);
            let softened = normalized * 0.9;
            *slot = if normalized > 0.62 { lifted } else { softened };
        }
    }
);

fn assert_quantize_matches_all_backends(input: &[f32]) {
    let expected = reference_quantize_f32(input);

    let mut portable = vec![0; input.len()];
    quantize_f32_to_i32(input, &mut portable);
    assert_eq!(portable, expected);

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
        if std::arch::is_x86_feature_detected!("sse2") {
            let mut output = vec![0; input.len()];
            unsafe { quantize_f32_to_i32_sse2(input, &mut output) };
            assert_eq!(output, expected);
        }
        if std::arch::is_x86_feature_detected!("sse4.1") {
            let mut output = vec![0; input.len()];
            unsafe { quantize_f32_to_i32_sse41(input, &mut output) };
            assert_eq!(output, expected);
        }
        if std::arch::is_x86_feature_detected!("avx2") && std::arch::is_x86_feature_detected!("fma")
        {
            let mut output = vec![0; input.len()];
            unsafe { quantize_f32_to_i32_avx2(input, &mut output) };
            assert_eq!(output, expected);
        }
    }
}

fn assert_normalize_luma_matches_all_backends(input: &[f32]) {
    let expected = reference_normalize_luma_f32(input);

    let mut portable = vec![0.0; input.len()];
    normalize_luma_f32(input, &mut portable);
    assert_f32_slices_close(&portable, &expected);

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
        if std::arch::is_x86_feature_detected!("sse2") {
            let mut output = vec![0.0; input.len()];
            unsafe { normalize_luma_f32_sse2(input, &mut output) };
            assert_f32_slices_close(&output, &expected);
        }
        if std::arch::is_x86_feature_detected!("sse4.1") {
            let mut output = vec![0.0; input.len()];
            unsafe { normalize_luma_f32_sse41(input, &mut output) };
            assert_f32_slices_close(&output, &expected);
        }
        if std::arch::is_x86_feature_detected!("avx2") && std::arch::is_x86_feature_detected!("fma")
        {
            let mut output = vec![0.0; input.len()];
            unsafe { normalize_luma_f32_avx2(input, &mut output) };
            assert_f32_slices_close(&output, &expected);
        }
    }
}

// This is the kind of magnitude-to-bucket stage used before writing image intensities or feature
// maps back to integer buffers, where clamp and floor must agree across portable and native paths.
#[test]
fn real_world_quantize_f32_to_i32_matches_reference() {
    let input = [
        -300.0, -255.9, -144.3, -33.2, -8.75, -1.1, -0.4, 0.0, 0.2, 0.7, 1.1, 3.9, 7.3, 12.8, 31.6,
        63.2, 91.7, 127.4, 144.6, 199.1, 255.3, 300.7,
    ]
    .into_iter()
    .cycle()
    .take(53)
    .enumerate()
    .map(|(i, value)| value + (i % 5) as f32 * 0.03125)
    .collect::<Vec<_>>();

    assert_quantize_matches_all_backends(&[]);
    assert_quantize_matches_all_backends(&input[..3]);
    assert_quantize_matches_all_backends(&input[..17]);
    assert_quantize_matches_all_backends(&input);
}

// This mirrors a luma-normalization pass from imaging/video pipelines, where incoming studio-range
// values are normalized, thresholded, and gently lifted before later stages consume them.
#[test]
fn real_world_normalize_luma_f32_matches_reference() {
    let input = [
        -20.0, 0.0, 8.0, 15.5, 16.0, 23.0, 48.0, 64.0, 96.0, 128.0, 160.0, 186.0, 210.0, 235.0,
        260.0,
    ]
    .into_iter()
    .cycle()
    .take(67)
    .enumerate()
    .map(|(i, value)| value + ((i % 9) as f32 - 4.0) * 0.375)
    .collect::<Vec<_>>();

    assert_normalize_luma_matches_all_backends(&[]);
    assert_normalize_luma_matches_all_backends(&input[..5]);
    assert_normalize_luma_matches_all_backends(&input[..19]);
    assert_normalize_luma_matches_all_backends(&input);
}
