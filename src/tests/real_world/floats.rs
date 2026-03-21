#![allow(unused_parens, dead_code)]

use crate::prelude::*;

use super::helpers::*;

fn reference_quantize_f32(input: &[f32]) -> Vec<i32> {
    input
        .iter()
        .map(|&value| (value.abs() * 1.75 + 0.125).floor().clamp(0.0, 255.0) as i32)
        .collect()
}

fn reference_noise_gate_f32(input: &[f32]) -> Vec<f32> {
    input
        .iter()
        .map(|&value| {
            if value.abs() > 0.0625 {
                (value * 1.5).clamp(-1.0, 1.0)
            } else {
                0.0
            }
        })
        .collect()
}

fn reference_floor_bucket_f32(input: &[f32]) -> Vec<f32> {
    input
        .iter()
        .map(|&value| ((value * 8.0) + 0.25).floor().clamp(0.0, 31.0))
        .collect()
}

fn reference_floor_bucket_f64(input: &[f64]) -> Vec<f64> {
    input
        .iter()
        .map(|&value| ((value.abs() * 4.0) + 0.5).floor().clamp(0.0, 63.0))
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
    fn noise_gate_f32(input: &[f32], output: &mut [f32]) {
        assert_eq!(input.len(), output.len());

        let mut input = input;
        let mut output = output;
        let threshold = S::Vf32::set1(0.0625);
        let zero = S::Vf32::set1(0.0);
        let gain = S::Vf32::set1(1.5);
        let min_level = S::Vf32::set1(-1.0);
        let max_level = S::Vf32::set1(1.0);

        while input.len() >= S::Vf32::WIDTH {
            let values = S::Vf32::load_from_slice(input);
            let boosted = (values * gain).max(min_level).min(max_level);
            let gated = values.abs().cmp_gt(threshold).blendv(zero, boosted);
            gated.copy_to_slice(output);

            input = &input[S::Vf32::WIDTH..];
            output = &mut output[S::Vf32::WIDTH..];
        }

        for (value, slot) in input.iter().zip(output.iter_mut()) {
            *slot = if value.abs() > 0.0625 {
                (value * 1.5).clamp(-1.0, 1.0)
            } else {
                0.0
            };
        }
    }
);

simd_unsafe_generate_all!(
    fn floor_bucket_f32(input: &[f32], output: &mut [f32]) {
        assert_eq!(input.len(), output.len());

        let mut input = input;
        let mut output = output;
        let scale = S::Vf32::set1(8.0);
        let bias = S::Vf32::set1(0.25);
        let lower = S::Vf32::set1(0.0);
        let upper = S::Vf32::set1(31.0);

        while input.len() >= S::Vf32::WIDTH {
            let values = S::Vf32::load_from_slice(input);
            let bucketed = values.mul_add(scale, bias).floor().max(lower).min(upper);
            bucketed.copy_to_slice(output);

            input = &input[S::Vf32::WIDTH..];
            output = &mut output[S::Vf32::WIDTH..];
        }

        for (value, slot) in input.iter().zip(output.iter_mut()) {
            *slot = ((value * 8.0) + 0.25).floor().clamp(0.0, 31.0);
        }
    }
);

simd_unsafe_generate_all!(
    fn floor_bucket_f64(input: &[f64], output: &mut [f64]) {
        assert_eq!(input.len(), output.len());

        let mut input = input;
        let mut output = output;
        let scale = S::Vf64::set1(4.0);
        let bias = S::Vf64::set1(0.5);
        let lower = S::Vf64::set1(0.0);
        let upper = S::Vf64::set1(63.0);

        while input.len() >= S::Vf64::WIDTH {
            let values = S::Vf64::load_from_slice(input);
            let bucketed = values.abs().mul_add(scale, bias).floor().max(lower).min(upper);
            bucketed.copy_to_slice(output);

            input = &input[S::Vf64::WIDTH..];
            output = &mut output[S::Vf64::WIDTH..];
        }

        for (value, slot) in input.iter().zip(output.iter_mut()) {
            *slot = ((value.abs() * 4.0) + 0.5).floor().clamp(0.0, 63.0);
        }
    }
);

fn assert_quantize_matches_all_backends(input: &[f32]) {
    let expected = reference_quantize_f32(input);

    let mut scalar = vec![0; input.len()];
    quantize_f32_to_i32(input, &mut scalar);
    assert_eq!(scalar, expected);

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    assert_x86_scalar_i32_slice(
        quantize_f32_to_i32,
        |input, out| unsafe { quantize_f32_to_i32_sse2(input, out) },
        |input, out| unsafe { quantize_f32_to_i32_sse41(input, out) },
        |input, out| unsafe { quantize_f32_to_i32_avx2(input, out) },
        input,
        &expected,
    );
}

fn assert_noise_gate_matches_all_backends(input: &[f32]) {
    let expected = reference_noise_gate_f32(input);

    let mut scalar = vec![0.0; input.len()];
    noise_gate_f32(input, &mut scalar);
    assert_eq!(scalar, expected);

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    assert_x86_scalar_f32_slice(
        noise_gate_f32,
        |input, out| unsafe { noise_gate_f32_sse2(input, out) },
        |input, out| unsafe { noise_gate_f32_sse41(input, out) },
        |input, out| unsafe { noise_gate_f32_avx2(input, out) },
        input,
        &expected,
    );
}

fn assert_floor_bucket_f32_matches_all_backends(input: &[f32]) {
    let expected = reference_floor_bucket_f32(input);

    let mut scalar = vec![0.0; input.len()];
    floor_bucket_f32(input, &mut scalar);
    assert_eq!(scalar, expected);

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    assert_x86_scalar_f32_slice(
        floor_bucket_f32,
        |input, out| unsafe { floor_bucket_f32_sse2(input, out) },
        |input, out| unsafe { floor_bucket_f32_sse41(input, out) },
        |input, out| unsafe { floor_bucket_f32_avx2(input, out) },
        input,
        &expected,
    );
}

fn assert_floor_bucket_f64_matches_all_backends(input: &[f64]) {
    let expected = reference_floor_bucket_f64(input);

    let mut scalar = vec![0.0; input.len()];
    floor_bucket_f64(input, &mut scalar);
    assert_eq!(scalar, expected);

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    assert_x86_scalar_f64_slice(
        floor_bucket_f64,
        |input, out| unsafe { floor_bucket_f64_sse2(input, out) },
        |input, out| unsafe { floor_bucket_f64_sse41(input, out) },
        |input, out| unsafe { floor_bucket_f64_avx2(input, out) },
        input,
        &expected,
    );
}

/// Real-world context: this looks like feature quantization in image, ML, or DSP pipelines,
/// where floating-point magnitudes are scaled, biased, bucketed, saturated, and converted to
/// integer bins for later table lookups or compact storage.
#[test]
fn real_world_quantize_f32_to_i32_matches_reference() {
    let input = [
        -300.0, -255.9, -144.3, -33.2, -8.75, -1.1, -0.4, 0.0, 0.2, 0.7, 1.1, 3.9, 7.3, 12.8,
        31.6, 63.2, 91.7, 127.4, 144.6, 199.1, 255.3, 300.7,
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

/// Real-world context: this models an audio or sensor noise gate where tiny values are treated
/// as noise, louder samples are amplified, and the result is clipped back into a safe range.
/// It stresses compares, mask-controlled blending, and tail handling on signed floats.
#[test]
fn real_world_noise_gate_f32_matches_reference() {
    let input = [
        -1.2, -0.75, -0.1, -0.0625, -0.04, -0.001, 0.0, 0.001, 0.04, 0.0625, 0.09, 0.35, 0.8,
        1.2,
    ]
    .into_iter()
    .cycle()
    .take(57)
    .enumerate()
    .map(|(i, value)| value + ((i % 4) as f32 - 1.5) * 0.0078125)
    .collect::<Vec<_>>();

    assert_noise_gate_matches_all_backends(&[]);
    assert_noise_gate_matches_all_backends(&input[..5]);
    assert_noise_gate_matches_all_backends(&input[..21]);
    assert_noise_gate_matches_all_backends(&input);
}

/// Real-world context: this is like bucketing normalized scores, texture coordinates, or UI
/// progress values into coarse levels before histogramming, palette lookup, or fixed-step logic.
/// It emphasizes fused scaling, flooring, and saturation on non-integer floats.
#[test]
fn real_world_floor_bucket_f32_matches_reference() {
    let input = [
        -3.0, -1.0, -0.125, 0.0, 0.01, 0.125, 0.24, 0.49, 0.5, 0.75, 1.0, 1.25, 1.99, 2.1, 3.8,
        4.2,
    ]
    .into_iter()
    .cycle()
    .take(45)
    .enumerate()
    .map(|(i, value)| value + (i % 3) as f32 * 0.015625)
    .collect::<Vec<_>>();

    assert_floor_bucket_f32_matches_all_backends(&[]);
    assert_floor_bucket_f32_matches_all_backends(&input[..7]);
    assert_floor_bucket_f32_matches_all_backends(&input[..19]);
    assert_floor_bucket_f32_matches_all_backends(&input);
}

/// Real-world context: this mirrors wider-precision telemetry or finance-style preprocessing,
/// where double-precision values are normalized into capped buckets before downstream scoring,
/// aggregation, or anomaly detection. It checks the same pipeline shape on `f64` lanes.
#[test]
fn real_world_floor_bucket_f64_matches_reference() {
    let input = [
        -100.0, -17.5, -3.25, -0.5, -0.01, 0.0, 0.01, 0.5, 1.0, 1.25, 2.5, 4.0, 8.75, 15.0, 32.0,
        100.0,
    ]
    .into_iter()
    .cycle()
    .take(37)
    .enumerate()
    .map(|(i, value)| value + (i % 5) as f64 * 0.03125)
    .collect::<Vec<_>>();

    assert_floor_bucket_f64_matches_all_backends(&[]);
    assert_floor_bucket_f64_matches_all_backends(&input[..3]);
    assert_floor_bucket_f64_matches_all_backends(&input[..13]);
    assert_floor_bucket_f64_matches_all_backends(&input);
}
