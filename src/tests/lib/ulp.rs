#[inline]
fn ordered_u32(bits: u32) -> u32 {
    if bits & 0x8000_0000 != 0 {
        !bits
    } else {
        bits | 0x8000_0000
    }
}

#[inline]
fn ordered_u64(bits: u64) -> u64 {
    if bits & 0x8000_0000_0000_0000 != 0 {
        !bits
    } else {
        bits | 0x8000_0000_0000_0000
    }
}

/// Returns the ULP distance between two f32 values, or `None` if either is NaN.
pub fn ulp_distance_f32(a: f32, b: f32) -> Option<u32> {
    if a.is_nan() || b.is_nan() {
        return None;
    }

    let oa = ordered_u32(a.to_bits());
    let ob = ordered_u32(b.to_bits());
    Some(oa.abs_diff(ob))
}

/// Returns the ULP distance between two f64 values, or `None` if either is NaN.
pub fn ulp_distance_f64(a: f64, b: f64) -> Option<u64> {
    if a.is_nan() || b.is_nan() {
        return None;
    }

    let oa = ordered_u64(a.to_bits());
    let ob = ordered_u64(b.to_bits());
    Some(oa.abs_diff(ob))
}

#[cfg(test)]
mod tests {
    use super::{ulp_distance_f32, ulp_distance_f64};

    #[test]
    fn ulp_distance_handles_zero_signs() {
        assert_eq!(ulp_distance_f32(0.0, -0.0), Some(1));
        assert_eq!(ulp_distance_f64(0.0, -0.0), Some(1));
    }

    #[test]
    fn ulp_distance_matches_adjacent_bit_pattern_step() {
        let a = 1.0f32;
        let b = f32::from_bits(a.to_bits() + 1);
        assert_eq!(ulp_distance_f32(a, b), Some(1));

        let c = 1.0f64;
        let d = f64::from_bits(c.to_bits() + 1);
        assert_eq!(ulp_distance_f64(c, d), Some(1));
    }

    #[test]
    fn ulp_distance_rejects_nan() {
        assert_eq!(ulp_distance_f32(f32::NAN, 1.0), None);
        assert_eq!(ulp_distance_f64(f64::NAN, 1.0), None);
    }
}
