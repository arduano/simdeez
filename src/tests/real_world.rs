use crate::prelude::*;

fn reference_find_first_eq_i8(data: &[i8], needle: i8) -> Option<usize> {
    data.iter().position(|&value| value == needle)
}

fn reference_byte_checksum_i8(data: &[i8]) -> i64 {
    data.iter()
        .fold(0i64, |sum, &value| sum.wrapping_add((value as u8) as i64))
}

fn reference_quantize_f32(input: &[f32]) -> Vec<i32> {
    input
        .iter()
        .map(|&value| (value.abs() * 1.75 + 0.125).floor().clamp(0.0, 255.0) as i32)
        .collect()
}

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
    fn find_first_eq_i8(data: &[i8], needle: i8) -> Option<usize> {
        let mut offset = 0usize;
        let needle = S::Vi8::set1(needle);

        while data.len() >= S::Vi8::WIDTH {
            let chunk = S::Vi8::load_from_slice(data);
            if let Some(index) = chunk.cmp_eq(needle).index_of_first_truthy() {
                return Some(offset + index);
            }

            offset += S::Vi8::WIDTH;
            data = &data[S::Vi8::WIDTH..];
        }

        for (index, &value) in data.iter().enumerate() {
            if value == needle[0] {
                return Some(offset + index);
            }
        }

        None
    }
);

simd_unsafe_generate_all!(
    fn byte_checksum_i8(data: &[i8]) -> i64 {
        let mut sum = 0i64;

        while data.len() >= S::Vi8::WIDTH {
            let chunk = S::Vi8::load_from_slice(data);
            sum = sum.wrapping_add(chunk.horizontal_unsigned_add());
            data = &data[S::Vi8::WIDTH..];
        }

        for &value in data {
            sum = sum.wrapping_add((value as u8) as i64);
        }

        sum
    }
);

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

fn assert_find_first_eq_matches_all_backends(data: &[i8], needle: i8) {
    let expected = reference_find_first_eq_i8(data, needle);
    assert_eq!(find_first_eq_i8_scalar(data, needle), expected);

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
        if std::arch::is_x86_feature_detected!("sse2") {
            assert_eq!(unsafe { find_first_eq_i8_sse2(data, needle) }, expected);
        }
        if std::arch::is_x86_feature_detected!("sse4.1") {
            assert_eq!(unsafe { find_first_eq_i8_sse41(data, needle) }, expected);
        }
        if std::arch::is_x86_feature_detected!("avx2") && std::arch::is_x86_feature_detected!("fma")
        {
            assert_eq!(unsafe { find_first_eq_i8_avx2(data, needle) }, expected);
        }
    }
}

fn assert_byte_checksum_matches_all_backends(data: &[i8]) {
    let expected = reference_byte_checksum_i8(data);
    assert_eq!(byte_checksum_i8_scalar(data), expected);

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
        if std::arch::is_x86_feature_detected!("sse2") {
            assert_eq!(unsafe { byte_checksum_i8_sse2(data) }, expected);
        }
        if std::arch::is_x86_feature_detected!("sse4.1") {
            assert_eq!(unsafe { byte_checksum_i8_sse41(data) }, expected);
        }
        if std::arch::is_x86_feature_detected!("avx2") && std::arch::is_x86_feature_detected!("fma")
        {
            assert_eq!(unsafe { byte_checksum_i8_avx2(data) }, expected);
        }
    }
}

fn assert_quantize_matches_all_backends(input: &[f32]) {
    let expected = reference_quantize_f32(input);

    let mut scalar = vec![0; input.len()];
    quantize_f32_to_i32_scalar(input, &mut scalar);
    assert_eq!(scalar, expected);

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

fn assert_adaptive_select_matches_all_backends(lhs: &[i32], rhs: &[i32]) {
    let expected = reference_adaptive_select_i32(lhs, rhs);

    let mut scalar = vec![0; lhs.len()];
    adaptive_select_i32_scalar(lhs, rhs, &mut scalar);
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

#[test]
fn real_world_find_first_eq_i8_matches_reference() {
    let cases = [
        (Vec::<i8>::new(), 7i8),
        (vec![3i8], 3i8),
        (vec![3i8], -2i8),
        (
            (0..33)
                .map(|i| ((i * 5) as i8).wrapping_sub(60))
                .collect::<Vec<i8>>(),
            -15i8,
        ),
        (
            (0..64)
                .map(|i| (i as i8).wrapping_mul(7))
                .collect::<Vec<i8>>(),
            42i8,
        ),
    ];

    for (mut data, needle) in cases {
        assert_find_first_eq_matches_all_backends(&data, needle);

        if !data.is_empty() {
            let middle = data.len() / 2;
            data[middle] = needle;
            assert_find_first_eq_matches_all_backends(&data, needle);

            let tail = data.len() - 1;
            data.fill(needle.wrapping_add(1));
            data[tail] = needle;
            assert_find_first_eq_matches_all_backends(&data, needle);
        }
    }
}

#[test]
fn real_world_byte_checksum_i8_matches_reference() {
    let data = (0..257)
        .map(|i| ((i * 37) as i8).wrapping_sub(113))
        .collect::<Vec<_>>();

    assert_byte_checksum_matches_all_backends(&[]);
    assert_byte_checksum_matches_all_backends(&data[..1]);
    assert_byte_checksum_matches_all_backends(&data[..15]);
    assert_byte_checksum_matches_all_backends(&data[..63]);
    assert_byte_checksum_matches_all_backends(&data);
}

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
