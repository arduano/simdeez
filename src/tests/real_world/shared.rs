pub(crate) fn assert_f32_close(actual: f32, expected: f32) {
    let tolerance = 1.0e-5 * expected.abs().max(1.0);
    assert!(
        (actual - expected).abs() <= tolerance,
        "expected {expected}, got {actual} (tolerance {tolerance})"
    );
}

pub(crate) fn assert_f32_slices_close(actual: &[f32], expected: &[f32]) {
    assert_eq!(actual.len(), expected.len());

    for (index, (&actual, &expected)) in actual.iter().zip(expected.iter()).enumerate() {
        let tolerance = 1.0e-5 * expected.abs().max(1.0);
        assert!(
            (actual - expected).abs() <= tolerance,
            "mismatch at index {index}: expected {expected}, got {actual} (tolerance {tolerance})"
        );
    }
}
