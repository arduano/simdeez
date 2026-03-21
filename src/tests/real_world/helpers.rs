#![allow(dead_code)]

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub fn assert_x86_scalar_i8(
    portable: impl Fn(&[i8], i8) -> Option<usize>,
    sse2: impl Fn(&[i8], i8) -> Option<usize>,
    sse41: impl Fn(&[i8], i8) -> Option<usize>,
    avx2: impl Fn(&[i8], i8) -> Option<usize>,
    data: &[i8],
    needle: i8,
    expected: Option<usize>,
) {
    assert_eq!(portable(data, needle), expected);

    if std::arch::is_x86_feature_detected!("sse2") {
        assert_eq!(sse2(data, needle), expected);
    }
    if std::arch::is_x86_feature_detected!("sse4.1") {
        assert_eq!(sse41(data, needle), expected);
    }
    if std::arch::is_x86_feature_detected!("avx2") && std::arch::is_x86_feature_detected!("fma") {
        assert_eq!(avx2(data, needle), expected);
    }
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub fn assert_x86_scalar_i64(
    portable: impl Fn(&[i8]) -> i64,
    sse2: impl Fn(&[i8]) -> i64,
    sse41: impl Fn(&[i8]) -> i64,
    avx2: impl Fn(&[i8]) -> i64,
    data: &[i8],
    expected: i64,
) {
    assert_eq!(portable(data), expected);

    if std::arch::is_x86_feature_detected!("sse2") {
        assert_eq!(sse2(data), expected);
    }
    if std::arch::is_x86_feature_detected!("sse4.1") {
        assert_eq!(sse41(data), expected);
    }
    if std::arch::is_x86_feature_detected!("avx2") && std::arch::is_x86_feature_detected!("fma") {
        assert_eq!(avx2(data), expected);
    }
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub fn assert_x86_scalar_i32_slice(
    portable: impl Fn(&[f32], &mut [i32]),
    sse2: impl Fn(&[f32], &mut [i32]),
    sse41: impl Fn(&[f32], &mut [i32]),
    avx2: impl Fn(&[f32], &mut [i32]),
    input: &[f32],
    expected: &[i32],
) {
    let mut out = vec![0; input.len()];
    portable(input, &mut out);
    assert_eq!(out, expected);

    if std::arch::is_x86_feature_detected!("sse2") {
        let mut out = vec![0; input.len()];
        sse2(input, &mut out);
        assert_eq!(out, expected);
    }
    if std::arch::is_x86_feature_detected!("sse4.1") {
        let mut out = vec![0; input.len()];
        sse41(input, &mut out);
        assert_eq!(out, expected);
    }
    if std::arch::is_x86_feature_detected!("avx2") && std::arch::is_x86_feature_detected!("fma") {
        let mut out = vec![0; input.len()];
        avx2(input, &mut out);
        assert_eq!(out, expected);
    }
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub fn assert_x86_scalar_i32x2_slice(
    portable: impl Fn(&[i32], &[i32], &mut [i32]),
    sse2: impl Fn(&[i32], &[i32], &mut [i32]),
    sse41: impl Fn(&[i32], &[i32], &mut [i32]),
    avx2: impl Fn(&[i32], &[i32], &mut [i32]),
    lhs: &[i32],
    rhs: &[i32],
    expected: &[i32],
) {
    let mut out = vec![0; lhs.len()];
    portable(lhs, rhs, &mut out);
    assert_eq!(out, expected);

    if std::arch::is_x86_feature_detected!("sse2") {
        let mut out = vec![0; lhs.len()];
        sse2(lhs, rhs, &mut out);
        assert_eq!(out, expected);
    }
    if std::arch::is_x86_feature_detected!("sse4.1") {
        let mut out = vec![0; lhs.len()];
        sse41(lhs, rhs, &mut out);
        assert_eq!(out, expected);
    }
    if std::arch::is_x86_feature_detected!("avx2") && std::arch::is_x86_feature_detected!("fma") {
        let mut out = vec![0; lhs.len()];
        avx2(lhs, rhs, &mut out);
        assert_eq!(out, expected);
    }
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub fn assert_x86_scalar_f32_slice(
    portable: impl Fn(&[f32], &mut [f32]),
    sse2: impl Fn(&[f32], &mut [f32]),
    sse41: impl Fn(&[f32], &mut [f32]),
    avx2: impl Fn(&[f32], &mut [f32]),
    input: &[f32],
    expected: &[f32],
) {
    let mut out = vec![0.0; input.len()];
    portable(input, &mut out);
    assert_eq!(out, expected);

    if std::arch::is_x86_feature_detected!("sse2") {
        let mut out = vec![0.0; input.len()];
        sse2(input, &mut out);
        assert_eq!(out, expected);
    }
    if std::arch::is_x86_feature_detected!("sse4.1") {
        let mut out = vec![0.0; input.len()];
        sse41(input, &mut out);
        assert_eq!(out, expected);
    }
    if std::arch::is_x86_feature_detected!("avx2") && std::arch::is_x86_feature_detected!("fma") {
        let mut out = vec![0.0; input.len()];
        avx2(input, &mut out);
        assert_eq!(out, expected);
    }
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub fn assert_x86_scalar_f64_slice(
    portable: impl Fn(&[f64], &mut [f64]),
    sse2: impl Fn(&[f64], &mut [f64]),
    sse41: impl Fn(&[f64], &mut [f64]),
    avx2: impl Fn(&[f64], &mut [f64]),
    input: &[f64],
    expected: &[f64],
) {
    let mut out = vec![0.0; input.len()];
    portable(input, &mut out);
    assert_eq!(out, expected);

    if std::arch::is_x86_feature_detected!("sse2") {
        let mut out = vec![0.0; input.len()];
        sse2(input, &mut out);
        assert_eq!(out, expected);
    }
    if std::arch::is_x86_feature_detected!("sse4.1") {
        let mut out = vec![0.0; input.len()];
        sse41(input, &mut out);
        assert_eq!(out, expected);
    }
    if std::arch::is_x86_feature_detected!("avx2") && std::arch::is_x86_feature_detected!("fma") {
        let mut out = vec![0.0; input.len()];
        avx2(input, &mut out);
        assert_eq!(out, expected);
    }
}
