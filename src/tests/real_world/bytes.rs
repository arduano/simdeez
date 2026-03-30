use crate::prelude::*;

fn reference_find_first_eq_i8(data: &[i8], needle: i8) -> Option<usize> {
    data.iter().position(|&value| value == needle)
}

fn reference_byte_checksum_i8(data: &[i8]) -> i64 {
    data.iter()
        .fold(0i64, |sum, &value| sum.wrapping_add((value as u8) as i64))
}

simd_unsafe_generate_all!(
    fn find_first_eq_i8(data: &[i8], needle: i8) -> Option<usize> {
        let mut data = data;
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
        let mut data = data;
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

fn assert_find_first_eq_matches_all_backends(data: &[i8], needle: i8) {
    let expected = reference_find_first_eq_i8(data, needle);
    assert_eq!(find_first_eq_i8(data, needle), expected);

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
    assert_eq!(byte_checksum_i8(data), expected);

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

// This models parser and codec hot loops that scan byte streams for a sentinel marker without
// branching on every element, which matters when delimiters land near SIMD-lane boundaries.
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
fn real_world_find_first_eq_i8_respects_chunk_and_tail_boundaries() {
    for len in [15usize, 16, 17, 31, 32, 33] {
        let mut data = (0..len)
            .map(|i| (i as i8).wrapping_mul(11).wrapping_sub(73))
            .collect::<Vec<_>>();
        let needle = 101i8;

        assert_find_first_eq_matches_all_backends(&data, needle);

        for &index in &[0usize, len / 2, len - 1] {
            data.fill(needle.wrapping_sub(1));
            data[index] = needle;
            assert_find_first_eq_matches_all_backends(&data, needle);
        }
    }
}

// This mirrors byte-oriented checksums used in packet capture, texture uploads, and log shipping
// where unsigned accumulation has to stay correct across chunked SIMD reductions and scalar tails.
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
fn real_world_byte_checksum_i8_respects_chunk_and_tail_boundaries() {
    let data = (0..65)
        .map(|i| match i % 4 {
            0 => i8::MIN,
            1 => i8::MAX,
            2 => -1,
            _ => 1,
        })
        .collect::<Vec<_>>();

    for &len in &[15usize, 16, 17, 31, 32, 33, data.len()] {
        assert_byte_checksum_matches_all_backends(&data[..len]);
    }
}
