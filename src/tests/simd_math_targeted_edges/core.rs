use super::*;
use crate::math::{SimdMathF32Core, SimdMathF64Core};

fn run_f32_log2_u35_reduction_boundaries<S: Simd>() {
    let mut inputs = vec![
        f32::from_bits(0x3EFFFFFE),
        f32::from_bits(0x3EFFFFFF),
        f32::from_bits(0x3F000000),
        f32::from_bits(0x3F000001),
        f32::from_bits(0x3F7FFFFF),
        f32::from_bits(0x3F800000),
        f32::from_bits(0x3F800001),
        f32::from_bits(0x3FFFFFFF),
        f32::from_bits(0x40000000),
        f32::from_bits(0x40000001),
    ];

    for &scale in &[0.5f32, 1.0, 2.0, 8.0] {
        let pivot = ::core::f32::consts::FRAC_1_SQRT_2 * scale;
        inputs.push(f32::from_bits(pivot.to_bits() - 1));
        inputs.push(pivot);
        inputs.push(f32::from_bits(pivot.to_bits() + 1));
    }

    check_targeted_unary_f32::<S>(
        "log2_u35",
        &inputs,
        contracts::LOG2_U35_F32_MAX_ULP,
        |v| v.log2_u35(),
        f32::log2,
    );
}

fn run_f32_exp2_u35_fast_domain_boundaries<S: Simd>() {
    let mut inputs = vec![
        -126.0001,
        -126.0,
        -125.9999,
        -1.0001,
        -1.0,
        -0.9999,
        -0.0001,
        -0.0,
        0.0,
        0.0001,
        0.9999,
        1.0,
        1.0001,
        125.9999,
        126.0,
        126.0001,
        f32::NEG_INFINITY,
        f32::INFINITY,
        f32::NAN,
    ];

    for k in -4..=4 {
        let center = k as f32;
        inputs.push(center - 1.0 / 1024.0);
        inputs.push(center);
        inputs.push(center + 1.0 / 1024.0);
    }

    check_targeted_unary_f32::<S>(
        "exp2_u35",
        &inputs,
        contracts::EXP2_U35_F32_MAX_ULP,
        |v| v.exp2_u35(),
        f32::exp2,
    );
}

fn run_f32_trig_pi_boundaries<S: Simd>() {
    let mut inputs = vec![
        0.0,
        -0.0,
        ::core::f32::consts::PI,
        -::core::f32::consts::PI,
        ::core::f32::consts::FRAC_PI_2,
        -::core::f32::consts::FRAC_PI_2,
        ::core::f32::consts::FRAC_PI_4,
        -::core::f32::consts::FRAC_PI_4,
    ];

    for k in -8..=8 {
        let base = (k as f32) * ::core::f32::consts::FRAC_PI_2;
        inputs.push(f32::from_bits(base.to_bits().saturating_sub(1)));
        inputs.push(base);
        inputs.push(f32::from_bits(base.to_bits().saturating_add(1)));
    }

    check_targeted_unary_f32::<S>(
        "sin_u35",
        &inputs,
        contracts::SIN_U35_F32_MAX_ULP,
        |v| v.sin_u35(),
        f32::sin,
    );
    check_targeted_unary_f32::<S>(
        "cos_u35",
        &inputs,
        contracts::COS_U35_F32_MAX_ULP,
        |v| v.cos_u35(),
        f32::cos,
    );
}

fn run_f32_tan_pole_neighborhoods<S: Simd>() {
    let mut inputs = vec![
        -100.0,
        -10.0,
        -1.0,
        -0.0,
        0.0,
        1.0,
        10.0,
        100.0,
        f32::NAN,
        f32::INFINITY,
        f32::NEG_INFINITY,
    ];

    for k in -12..=12 {
        let pole = (k as f32 + 0.5) * ::core::f32::consts::PI;
        for delta in [1.0e-2, 1.0e-4, 1.0e-6] {
            inputs.push(pole - delta);
            inputs.push(pole + delta);
        }
    }

    check_targeted_unary_f32::<S>(
        "tan_u35",
        &inputs,
        contracts::TAN_U35_F32_MAX_ULP,
        |v| v.tan_u35(),
        f32::tan,
    );
}

fn run_f32_trig_large_and_mixed_lanes<S: Simd>() {
    let inputs = vec![
        0.25,
        -0.5,
        123.456,
        -2048.0,
        8192.0,
        -8192.0,
        16384.0,
        -16384.0,
        f32::from_bits(1),
        -f32::from_bits(1),
        f32::NAN,
        f32::INFINITY,
        f32::NEG_INFINITY,
        ::core::f32::consts::PI * 0.5 - 1.0e-4,
        ::core::f32::consts::PI * 0.5 + 1.0e-4,
        -::core::f32::consts::PI * 0.5 + 1.0e-4,
    ];

    check_targeted_unary_f32::<S>(
        "sin_u35",
        &inputs,
        contracts::SIN_U35_F32_MAX_ULP,
        |v| v.sin_u35(),
        f32::sin,
    );
    check_targeted_unary_f32::<S>(
        "cos_u35",
        &inputs,
        contracts::COS_U35_F32_MAX_ULP,
        |v| v.cos_u35(),
        f32::cos,
    );
    check_targeted_unary_f32::<S>(
        "tan_u35",
        &inputs,
        contracts::TAN_U35_F32_MAX_ULP,
        |v| v.tan_u35(),
        f32::tan,
    );
}

fn run_f32_trig_fast_range_boundaries<S: Simd>() {
    let edge = 8192.0f32;
    let inputs = vec![
        f32::from_bits(edge.to_bits().saturating_sub(2)),
        f32::from_bits(edge.to_bits().saturating_sub(1)),
        edge,
        f32::from_bits(edge.to_bits().saturating_add(1)),
        f32::from_bits(edge.to_bits().saturating_add(2)),
        -f32::from_bits(edge.to_bits().saturating_sub(2)),
        -f32::from_bits(edge.to_bits().saturating_sub(1)),
        -edge,
        -f32::from_bits(edge.to_bits().saturating_add(1)),
        -f32::from_bits(edge.to_bits().saturating_add(2)),
    ];

    check_targeted_unary_f32::<S>(
        "sin_u35",
        &inputs,
        contracts::SIN_U35_F32_MAX_ULP,
        |v| v.sin_u35(),
        f32::sin,
    );
    check_targeted_unary_f32::<S>(
        "cos_u35",
        &inputs,
        contracts::COS_U35_F32_MAX_ULP,
        |v| v.cos_u35(),
        f32::cos,
    );
    check_targeted_unary_f32::<S>(
        "tan_u35",
        &inputs,
        contracts::TAN_U35_F32_MAX_ULP,
        |v| v.tan_u35(),
        f32::tan,
    );
}

fn run_f32_trig_symmetry_identities<S: Simd>() {
    let inputs = [
        -3.0f32,
        -1.0,
        -0.5,
        -0.0,
        0.0,
        0.5,
        1.0,
        3.0,
        ::core::f32::consts::FRAC_PI_3,
        -::core::f32::consts::FRAC_PI_3,
    ];

    for chunk in inputs.chunks(S::Vf32::WIDTH) {
        let x = S::Vf32::load_from_slice(chunk);
        let sx = x.sin_u35();
        let cx = x.cos_u35();
        let tx = x.tan_u35();

        let neg_x = -x;
        let sneg = neg_x.sin_u35();
        let cneg = neg_x.cos_u35();
        let tneg = neg_x.tan_u35();

        for lane in 0..chunk.len() {
            if chunk[lane] == 0.0 {
                continue;
            }

            assert_f32_contract(
                "sin parity",
                chunk[lane],
                sneg[lane],
                -sx[lane],
                contracts::SIN_U35_F32_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("{e}"));
            assert_f32_contract(
                "cos parity",
                chunk[lane],
                cneg[lane],
                cx[lane],
                contracts::COS_U35_F32_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("{e}"));
            assert_f32_contract(
                "tan parity",
                chunk[lane],
                tneg[lane],
                -tx[lane],
                contracts::TAN_U35_F32_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("{e}"));
        }
    }
}

simd_math_targeted_all_backends!(
    f32_log2_u35_reduction_boundaries,
    run_f32_log2_u35_reduction_boundaries
);
simd_math_targeted_all_backends!(
    f32_exp2_u35_fast_domain_boundaries,
    run_f32_exp2_u35_fast_domain_boundaries
);
simd_math_targeted_all_backends!(f32_trig_pi_boundaries, run_f32_trig_pi_boundaries);
simd_math_targeted_all_backends!(f32_tan_pole_neighborhoods, run_f32_tan_pole_neighborhoods);
simd_math_targeted_all_backends!(
    f32_trig_large_and_mixed_lanes,
    run_f32_trig_large_and_mixed_lanes
);
simd_math_targeted_all_backends!(
    f32_trig_fast_range_boundaries,
    run_f32_trig_fast_range_boundaries
);
simd_math_targeted_all_backends!(
    f32_trig_symmetry_identities,
    run_f32_trig_symmetry_identities
);

fn run_f32_log_exp_boundary_lanes<S: Simd>() {
    let mut inputs_log = vec![
        f32::from_bits(1),
        f32::MIN_POSITIVE,
        0.5,
        std::f32::consts::FRAC_1_SQRT_2,
        1.0,
        2.0,
        128.0,
        f32::INFINITY,
        f32::NAN,
        -1.0,
        0.0,
        -0.0,
    ];

    for &scale in &[0.5f32, 1.0, 2.0, 8.0, 128.0] {
        let pivot = std::f32::consts::FRAC_1_SQRT_2 * scale;
        inputs_log.push(f32::from_bits(pivot.to_bits().saturating_sub(1)));
        inputs_log.push(pivot);
        inputs_log.push(f32::from_bits(pivot.to_bits().saturating_add(1)));
    }

    check_targeted_unary_f32::<S>(
        "log2_u35",
        &inputs_log,
        contracts::LOG2_U35_F32_MAX_ULP,
        |v| v.log2_u35(),
        f32::log2,
    );
    check_targeted_unary_f32::<S>(
        "ln_u35",
        &inputs_log,
        contracts::LN_U35_F32_MAX_ULP,
        |v| v.ln_u35(),
        f32::ln,
    );

    let mut inputs_exp = vec![
        -126.0f32,
        -125.5,
        -10.0,
        -1.0,
        -0.0,
        0.0,
        1.0,
        10.0,
        126.0,
        127.0,
        127.25,
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::NAN,
    ];

    for k in -4..=4 {
        let center = k as f32;
        inputs_exp.push(center - 1.0 / 1024.0);
        inputs_exp.push(center);
        inputs_exp.push(center + 1.0 / 1024.0);
    }

    for &center in &[-126.0f32, -125.5, -1.0, -0.5, 0.0, 0.5, 1.0, 126.0, 127.0] {
        inputs_exp.push(f32::from_bits(center.to_bits().saturating_sub(1)));
        inputs_exp.push(center);
        inputs_exp.push(f32::from_bits(center.to_bits().saturating_add(1)));
    }

    check_targeted_unary_f32::<S>(
        "exp2_u35",
        &inputs_exp,
        contracts::EXP2_U35_F32_MAX_ULP,
        |v| v.exp2_u35(),
        f32::exp2,
    );

    let mut inputs_exp_e = vec![
        -104.0f32,
        -103.98,
        -1.0,
        -0.0,
        0.0,
        1.0,
        88.0,
        88.7,
        89.0,
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::NAN,
    ];

    for &center in &[-104.0f32, -103.97, -1.0, 0.0, 1.0, 88.5, 88.7] {
        inputs_exp_e.push(f32::from_bits(center.to_bits().saturating_sub(1)));
        inputs_exp_e.push(center);
        inputs_exp_e.push(f32::from_bits(center.to_bits().saturating_add(1)));
    }

    check_targeted_unary_f32::<S>(
        "exp_u35",
        &inputs_exp_e,
        contracts::EXP_U35_F32_MAX_ULP,
        |v| v.exp_u35(),
        f32::exp,
    );
}

simd_math_targeted_all_backends!(f32_log_exp_boundary_lanes, run_f32_log_exp_boundary_lanes);

fn run_f64_log_exp_boundary_lanes<S: Simd>() {
    let mut inputs_log = vec![
        f64::from_bits(1),
        f64::MIN_POSITIVE,
        0.5,
        std::f64::consts::FRAC_1_SQRT_2,
        1.0,
        2.0,
        1024.0,
        f64::INFINITY,
        f64::NAN,
        -1.0,
        0.0,
        -0.0,
    ];

    for &scale in &[0.5f64, 1.0, 2.0, 8.0, 1024.0] {
        let pivot = std::f64::consts::FRAC_1_SQRT_2 * scale;
        inputs_log.push(f64::from_bits(pivot.to_bits() - 1));
        inputs_log.push(pivot);
        inputs_log.push(f64::from_bits(pivot.to_bits() + 1));
    }

    check_targeted_unary_f64::<S>(
        "log2_u35",
        &inputs_log,
        contracts::LOG2_U35_F64_MAX_ULP,
        |v| v.log2_u35(),
        f64::log2,
    );
    check_targeted_unary_f64::<S>(
        "ln_u35",
        &inputs_log,
        contracts::LN_U35_F64_MAX_ULP,
        |v| v.ln_u35(),
        f64::ln,
    );

    let mut inputs_exp = vec![
        -1022.0,
        -1021.75,
        -10.0,
        -1.0,
        -0.0,
        0.0,
        1.0,
        10.0,
        1022.0,
        1023.0,
        1023.25,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NAN,
    ];

    for k in -4..=4 {
        let center = k as f64;
        inputs_exp.push(center - 1.0 / 4096.0);
        inputs_exp.push(center);
        inputs_exp.push(center + 1.0 / 4096.0);
    }

    for &center in &[
        -1022.0f64, -1021.5, -1.0, -0.5, 0.0, 0.5, 1.0, 1022.0, 1023.0,
    ] {
        inputs_exp.push(f64::from_bits(center.to_bits().saturating_sub(1)));
        inputs_exp.push(center);
        inputs_exp.push(f64::from_bits(center.to_bits().saturating_add(1)));
    }

    check_targeted_unary_f64::<S>(
        "exp2_u35",
        &inputs_exp,
        contracts::EXP2_U35_F64_MAX_ULP,
        |v| v.exp2_u35(),
        f64::exp2,
    );
    check_targeted_unary_f64::<S>(
        "exp_u35",
        &inputs_exp,
        contracts::EXP_U35_F64_MAX_ULP,
        |v| v.exp_u35(),
        f64::exp,
    );
}

fn run_f64_exp_fast_mask_boundaries<S: Simd>() {
    let mut inputs_exp2 = vec![f64::NEG_INFINITY, f64::INFINITY, f64::NAN];
    for &center in &[-1022.0f64, 1023.0] {
        inputs_exp2.push(f64::from_bits(center.to_bits() - 2));
        inputs_exp2.push(f64::from_bits(center.to_bits() - 1));
        inputs_exp2.push(center);
        inputs_exp2.push(f64::from_bits(center.to_bits() + 1));
        inputs_exp2.push(f64::from_bits(center.to_bits() + 2));
    }

    check_targeted_unary_f64::<S>(
        "exp2_u35 fast-mask boundary",
        &inputs_exp2,
        contracts::EXP2_U35_F64_MAX_ULP,
        |v| v.exp2_u35(),
        f64::exp2,
    );

    let mut inputs_exp = vec![f64::NEG_INFINITY, f64::INFINITY, f64::NAN];
    for &center in &[-708.0f64, 709.0] {
        inputs_exp.push(f64::from_bits(center.to_bits() - 2));
        inputs_exp.push(f64::from_bits(center.to_bits() - 1));
        inputs_exp.push(center);
        inputs_exp.push(f64::from_bits(center.to_bits() + 1));
        inputs_exp.push(f64::from_bits(center.to_bits() + 2));
    }

    check_targeted_unary_f64::<S>(
        "exp_u35 fast-mask boundary",
        &inputs_exp,
        contracts::EXP_U35_F64_MAX_ULP,
        |v| v.exp_u35(),
        f64::exp,
    );
}

fn run_f64_trig_pi_boundaries<S: Simd>() {
    let mut inputs = vec![
        -0.0,
        0.0,
        std::f64::consts::PI,
        -std::f64::consts::PI,
        std::f64::consts::FRAC_PI_2,
        -std::f64::consts::FRAC_PI_2,
        std::f64::consts::FRAC_PI_4,
        -std::f64::consts::FRAC_PI_4,
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
    ];

    for k in -12..=12 {
        let base = (k as f64) * std::f64::consts::FRAC_PI_2;
        inputs.push(f64::from_bits(base.to_bits().saturating_sub(1)));
        inputs.push(base);
        inputs.push(f64::from_bits(base.to_bits().saturating_add(1)));
    }

    check_targeted_unary_f64::<S>(
        "sin_u35",
        &inputs,
        contracts::SIN_U35_F64_MAX_ULP,
        |v| v.sin_u35(),
        f64::sin,
    );
    check_targeted_unary_f64::<S>(
        "cos_u35",
        &inputs,
        contracts::COS_U35_F64_MAX_ULP,
        |v| v.cos_u35(),
        f64::cos,
    );
}

fn run_f64_tan_pole_neighborhoods<S: Simd>() {
    let mut inputs = vec![-1.0, -0.0, 0.0, 1.0, 10.0, -10.0, f64::NAN, f64::INFINITY];

    for k in -16..=16 {
        let pole = (k as f64 + 0.5) * std::f64::consts::PI;
        for delta in [1.0e-4, 1.0e-6, 1.0e-8] {
            inputs.push(pole - delta);
            inputs.push(pole + delta);
        }
    }

    check_targeted_unary_f64::<S>(
        "tan_u35",
        &inputs,
        contracts::TAN_U35_F64_MAX_ULP,
        |v| v.tan_u35(),
        f64::tan,
    );
}

simd_math_targeted_all_backends!(f64_log_exp_boundary_lanes, run_f64_log_exp_boundary_lanes);
simd_math_targeted_all_backends!(
    f64_exp_fast_mask_boundaries,
    run_f64_exp_fast_mask_boundaries
);
simd_math_targeted_all_backends!(f64_trig_pi_boundaries, run_f64_trig_pi_boundaries);
simd_math_targeted_all_backends!(f64_tan_pole_neighborhoods, run_f64_tan_pole_neighborhoods);

fn run_f64_trig_large_and_mixed_lanes<S: Simd>() {
    let inputs = vec![
        0.25,
        -0.5,
        123.456_789,
        -2048.0,
        8192.0,
        -8192.0,
        1.0e6,
        -1.0e6,
        f64::from_bits(1),
        -f64::from_bits(1),
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        std::f64::consts::PI * 0.5 - 1.0e-12,
        std::f64::consts::PI * 0.5 + 1.0e-12,
        -std::f64::consts::PI * 0.5 + 1.0e-12,
    ];

    check_targeted_unary_f64::<S>(
        "sin_u35",
        &inputs,
        contracts::SIN_U35_F64_MAX_ULP,
        |v| v.sin_u35(),
        f64::sin,
    );
    check_targeted_unary_f64::<S>(
        "cos_u35",
        &inputs,
        contracts::COS_U35_F64_MAX_ULP,
        |v| v.cos_u35(),
        f64::cos,
    );
    check_targeted_unary_f64::<S>(
        "tan_u35",
        &inputs,
        contracts::TAN_U35_F64_MAX_ULP,
        |v| v.tan_u35(),
        f64::tan,
    );
}

fn run_f64_trig_symmetry_identities<S: Simd>() {
    let inputs = [
        -3.0f64,
        -1.0,
        -0.5,
        -0.0,
        0.0,
        0.5,
        1.0,
        3.0,
        std::f64::consts::FRAC_PI_3,
        -std::f64::consts::FRAC_PI_3,
    ];

    for chunk in inputs.chunks(S::Vf64::WIDTH) {
        let x = S::Vf64::load_from_slice(chunk);
        let sx = x.sin_u35();
        let cx = x.cos_u35();
        let tx = x.tan_u35();

        let neg_x = -x;
        let sneg = neg_x.sin_u35();
        let cneg = neg_x.cos_u35();
        let tneg = neg_x.tan_u35();

        for lane in 0..chunk.len() {
            if chunk[lane] == 0.0 {
                continue;
            }

            assert_f64_contract(
                "sin parity",
                chunk[lane],
                sneg[lane],
                -sx[lane],
                contracts::SIN_U35_F64_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("{e}"));
            assert_f64_contract(
                "cos parity",
                chunk[lane],
                cneg[lane],
                cx[lane],
                contracts::COS_U35_F64_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("{e}"));
            assert_f64_contract(
                "tan parity",
                chunk[lane],
                tneg[lane],
                -tx[lane],
                contracts::TAN_U35_F64_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("{e}"));
        }
    }
}

simd_math_targeted_all_backends!(
    f64_trig_large_and_mixed_lanes,
    run_f64_trig_large_and_mixed_lanes
);
simd_math_targeted_all_backends!(
    f64_trig_symmetry_identities,
    run_f64_trig_symmetry_identities
);

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[test]
fn f32_log2_u35_mixed_exception_lanes_avx2() {
    let has_avx2 = std::is_x86_feature_detected!("avx2");
    let has_fma = std::is_x86_feature_detected!("fma");
    if !(has_avx2 && has_fma) {
        eprintln!("[test] skipped avx2/fma mixed-lane log2_u35 test: CPU lacks avx2/fma");
        return;
    }

    let input = vec![
        1.0,
        2.0,
        -1.0,
        0.0,
        -0.0,
        f32::from_bits(1),
        f32::INFINITY,
        f32::NAN,
        0.75,
        1.5,
        3.0,
        64.0,
        1024.0,
        0.25,
        f32::from_bits(0x7FC0_1234),
        f32::from_bits(0x0000_0100),
    ];

    let mut output = vec![0.0f32; input.len()];
    run_log2_u35_vector_apply_avx2(&input, &mut output);

    for (&x, &actual) in input.iter().zip(output.iter()) {
        let expected = x.log2();
        if let Err(err) = assert_f32_contract(
            "log2_u35",
            x,
            actual,
            expected,
            contracts::LOG2_U35_F32_MAX_ULP,
        ) {
            panic!("{err}");
        }
    }
}
