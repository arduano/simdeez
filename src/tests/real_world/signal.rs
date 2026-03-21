use crate::prelude::*;

use super::shared::{assert_f32_close, assert_f32_slices_close};

fn reference_noise_gate_f32(input: &[f32]) -> Vec<f32> {
    input
        .iter()
        .map(|&value| {
            let magnitude = value.abs();
            if magnitude > 0.12 {
                value
            } else if magnitude > 0.02 {
                value * 0.25
            } else {
                0.0
            }
        })
        .collect()
}

fn reference_crossfade_f32(lhs: &[f32], rhs: &[f32], weights: &[f32]) -> Vec<f32> {
    lhs.iter()
        .zip(rhs.iter())
        .zip(weights.iter())
        .map(|((&left, &right), &weight)| {
            let weight = weight.clamp(0.0, 1.0);
            left + (right - left) * weight
        })
        .collect()
}

fn reference_window_energy_f32(input: &[f32]) -> f32 {
    input.iter().map(|&value| value * value).sum()
}

simd_unsafe_generate_all!(
    fn noise_gate_f32(input: &[f32], output: &mut [f32]) {
        assert_eq!(input.len(), output.len());

        let mut input = input;
        let mut output = output;
        let low = S::Vf32::set1(0.02);
        let high = S::Vf32::set1(0.12);
        let attenuation = S::Vf32::set1(0.25);
        let zero = S::Vf32::set1(0.0);

        while input.len() >= S::Vf32::WIDTH {
            let values = S::Vf32::load_from_slice(input);
            let magnitude = values.abs();
            let attenuated = values.mul(attenuation);
            let attenuated_or_silent = magnitude.cmp_gt(low).blendv(zero, attenuated);
            let gated = magnitude.cmp_gt(high).blendv(attenuated_or_silent, values);
            gated.copy_to_slice(output);

            input = &input[S::Vf32::WIDTH..];
            output = &mut output[S::Vf32::WIDTH..];
        }

        for (value, slot) in input.iter().zip(output.iter_mut()) {
            let magnitude = value.abs();
            *slot = if magnitude > 0.12 {
                *value
            } else if magnitude > 0.02 {
                *value * 0.25
            } else {
                0.0
            };
        }
    }
);

simd_unsafe_generate_all!(
    fn crossfade_f32(lhs: &[f32], rhs: &[f32], weights: &[f32], output: &mut [f32]) {
        assert_eq!(lhs.len(), rhs.len());
        assert_eq!(lhs.len(), weights.len());
        assert_eq!(lhs.len(), output.len());

        let mut lhs = lhs;
        let mut rhs = rhs;
        let mut weights = weights;
        let mut output = output;
        let zero = S::Vf32::set1(0.0);
        let one = S::Vf32::set1(1.0);

        while lhs.len() >= S::Vf32::WIDTH {
            let left = S::Vf32::load_from_slice(lhs);
            let right = S::Vf32::load_from_slice(rhs);
            let weight = S::Vf32::load_from_slice(weights).max(zero).min(one);
            let delta = right - left;
            let mixed = delta.mul_add(weight, left);
            mixed.copy_to_slice(output);

            lhs = &lhs[S::Vf32::WIDTH..];
            rhs = &rhs[S::Vf32::WIDTH..];
            weights = &weights[S::Vf32::WIDTH..];
            output = &mut output[S::Vf32::WIDTH..];
        }

        for (((left, right), weight), slot) in lhs
            .iter()
            .zip(rhs.iter())
            .zip(weights.iter())
            .zip(output.iter_mut())
        {
            let weight = weight.clamp(0.0, 1.0);
            *slot = *left + (*right - *left) * weight;
        }
    }
);

simd_unsafe_generate_all!(
    fn window_energy_f32(input: &[f32]) -> f32 {
        let mut input = input;
        let mut accum = S::Vf32::zeroes();

        while input.len() >= S::Vf32::WIDTH {
            let values = S::Vf32::load_from_slice(input);
            accum += values * values;
            input = &input[S::Vf32::WIDTH..];
        }

        let mut sum = accum.horizontal_add();
        for &value in input {
            sum += value * value;
        }

        sum
    }
);

fn assert_noise_gate_matches_all_backends(input: &[f32]) {
    let expected = reference_noise_gate_f32(input);

    let mut portable = vec![0.0; input.len()];
    noise_gate_f32(input, &mut portable);
    assert_f32_slices_close(&portable, &expected);

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
        if std::arch::is_x86_feature_detected!("sse2") {
            let mut output = vec![0.0; input.len()];
            unsafe { noise_gate_f32_sse2(input, &mut output) };
            assert_f32_slices_close(&output, &expected);
        }
        if std::arch::is_x86_feature_detected!("sse4.1") {
            let mut output = vec![0.0; input.len()];
            unsafe { noise_gate_f32_sse41(input, &mut output) };
            assert_f32_slices_close(&output, &expected);
        }
        if std::arch::is_x86_feature_detected!("avx2") && std::arch::is_x86_feature_detected!("fma")
        {
            let mut output = vec![0.0; input.len()];
            unsafe { noise_gate_f32_avx2(input, &mut output) };
            assert_f32_slices_close(&output, &expected);
        }
    }
}

fn assert_crossfade_matches_all_backends(lhs: &[f32], rhs: &[f32], weights: &[f32]) {
    let expected = reference_crossfade_f32(lhs, rhs, weights);

    let mut portable = vec![0.0; lhs.len()];
    crossfade_f32(lhs, rhs, weights, &mut portable);
    assert_f32_slices_close(&portable, &expected);

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
        if std::arch::is_x86_feature_detected!("sse2") {
            let mut output = vec![0.0; lhs.len()];
            unsafe { crossfade_f32_sse2(lhs, rhs, weights, &mut output) };
            assert_f32_slices_close(&output, &expected);
        }
        if std::arch::is_x86_feature_detected!("sse4.1") {
            let mut output = vec![0.0; lhs.len()];
            unsafe { crossfade_f32_sse41(lhs, rhs, weights, &mut output) };
            assert_f32_slices_close(&output, &expected);
        }
        if std::arch::is_x86_feature_detected!("avx2") && std::arch::is_x86_feature_detected!("fma")
        {
            let mut output = vec![0.0; lhs.len()];
            unsafe { crossfade_f32_avx2(lhs, rhs, weights, &mut output) };
            assert_f32_slices_close(&output, &expected);
        }
    }
}

fn assert_window_energy_matches_all_backends(input: &[f32]) {
    let expected = reference_window_energy_f32(input);
    assert_f32_close(window_energy_f32(input), expected);

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
        if std::arch::is_x86_feature_detected!("sse2") {
            assert_f32_close(unsafe { window_energy_f32_sse2(input) }, expected);
        }
        if std::arch::is_x86_feature_detected!("sse4.1") {
            assert_f32_close(unsafe { window_energy_f32_sse41(input) }, expected);
        }
        if std::arch::is_x86_feature_detected!("avx2") && std::arch::is_x86_feature_detected!("fma")
        {
            assert_f32_close(unsafe { window_energy_f32_avx2(input) }, expected);
        }
    }
}

// This models a simple denoise gate from audio/sensor pipelines, where low-amplitude content is
// muted or attenuated before later analysis so that tails and SIMD remainders behave consistently.
#[test]
fn real_world_noise_gate_f32_matches_reference() {
    let input = [
        -0.3, -0.15, -0.12, -0.08, -0.03, -0.02, -0.005, 0.0, 0.004, 0.019, 0.02, 0.045, 0.11,
        0.12, 0.25,
    ]
    .into_iter()
    .cycle()
    .take(71)
    .enumerate()
    .map(|(i, value)| value + ((i % 5) as f32 - 2.0) * 0.0015)
    .collect::<Vec<_>>();

    assert_noise_gate_matches_all_backends(&[]);
    assert_noise_gate_matches_all_backends(&input[..7]);
    assert_noise_gate_matches_all_backends(&input[..23]);
    assert_noise_gate_matches_all_backends(&input);
}

// This is the core of crossfades and linear resampling, where lane-wise interpolation weights are
// clamped before mixing streams so portable dispatch and native backends produce the same blend.
#[test]
fn real_world_crossfade_f32_matches_reference() {
    let lhs = (0..59)
        .map(|i| ((i as f32 * 0.13).sin() * 0.8) - 0.4)
        .collect::<Vec<_>>();
    let rhs = (0..59)
        .map(|i| ((i as f32 * 0.17).cos() * 0.6) + 0.2)
        .collect::<Vec<_>>();
    let weights = (0..59)
        .map(|i| ((i % 11) as f32 - 3.0) * 0.17)
        .collect::<Vec<_>>();

    assert_crossfade_matches_all_backends(&[], &[], &[]);
    assert_crossfade_matches_all_backends(&lhs[..4], &rhs[..4], &weights[..4]);
    assert_crossfade_matches_all_backends(&lhs[..18], &rhs[..18], &weights[..18]);
    assert_crossfade_matches_all_backends(&lhs, &rhs, &weights);
}

// This reflects energy accumulation in VAD, metering, and feature extraction passes where SIMD
// accumulators reduce many samples before a final scalar fold and backend ordering can shift rounding.
#[test]
fn real_world_window_energy_f32_matches_reference() {
    let input = (0..173)
        .map(|i| {
            let centered = (i as f32 - 86.0) * 0.03125;
            centered.sin() * 0.75 + centered.cos() * 0.125
        })
        .collect::<Vec<_>>();

    assert_window_energy_matches_all_backends(&[]);
    assert_window_energy_matches_all_backends(&input[..5]);
    assert_window_energy_matches_all_backends(&input[..29]);
    assert_window_energy_matches_all_backends(&input);
}
