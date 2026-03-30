use super::*;
use crate::math::{SimdMathF32Hyperbolic, SimdMathF32InverseHyperbolic, SimdMathF64Hyperbolic};

fn run_f32_hyperbolic_edges<S: Simd>() {
    let inputs = [
        -100.0f32, -40.0, -20.0, -10.0, -1.0, -0.5, -0.0, 0.0, 0.5, 1.0, 10.0, 20.0, 40.0, 100.0,
    ];
    for chunk in inputs.chunks(S::Vf32::WIDTH) {
        let v = S::Vf32::load_from_slice(chunk);
        let sinh = v.sinh_u35();
        let cosh = v.cosh_u35();
        let tanh = v.tanh_u35();
        let acosh = v.acosh_u35();
        let atanh = v.atanh_u35();
        for (lane, &x) in chunk.iter().enumerate() {
            assert_f32_contract(
                "sinh_u35",
                x,
                sinh[lane],
                x.sinh(),
                contracts::SINH_U35_F32_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("{e}"));
            assert_f32_contract(
                "cosh_u35",
                x,
                cosh[lane],
                x.cosh(),
                contracts::COSH_U35_F32_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("{e}"));
            assert_f32_contract(
                "tanh_u35",
                x,
                tanh[lane],
                x.tanh(),
                contracts::TANH_U35_F32_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("{e}"));
            assert_f32_contract(
                "acosh_u35",
                x,
                acosh[lane],
                x.acosh(),
                contracts::ACOSH_U35_F32_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("{e}"));
            assert_f32_contract(
                "atanh_u35",
                x,
                atanh[lane],
                x.atanh(),
                contracts::ATANH_U35_F32_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("{e}"));
        }
    }

    let near_one = [
        0.999_999_9f32,
        -0.999_999_9,
        1.0 - f32::EPSILON,
        -1.0 + f32::EPSILON,
        1.0,
        -1.0,
        1.0 + f32::EPSILON,
    ];
    for chunk in near_one.chunks(S::Vf32::WIDTH) {
        let v = S::Vf32::load_from_slice(chunk);
        let atanh = v.atanh_u35();
        let acosh = v.acosh_u35();
        for (lane, &x) in chunk.iter().enumerate() {
            assert_f32_contract(
                "atanh_u35",
                x,
                atanh[lane],
                x.atanh(),
                contracts::ATANH_U35_F32_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("{e}"));
            assert_f32_contract(
                "acosh_u35",
                x,
                acosh[lane],
                x.acosh(),
                contracts::ACOSH_U35_F32_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("{e}"));
        }
    }
}

fn push_boundary_triplet(inputs: &mut Vec<f32>, center: f32) {
    inputs.push(f32::from_bits(center.to_bits().saturating_sub(1)));
    inputs.push(center);
    inputs.push(f32::from_bits(center.to_bits().saturating_add(1)));
}

fn run_f32_hyperbolic_fast_path_boundaries<S: Simd>() {
    let mut inputs = Vec::new();
    for center in [
        -80.0f32, -40.0, -0.625, -0.5, -0.0, 0.0, 0.5, 0.625, 40.0, 80.0,
    ] {
        push_boundary_triplet(&mut inputs, center);
    }

    check_targeted_unary_f32::<S>(
        "sinh_u35",
        &inputs,
        contracts::SINH_U35_F32_MAX_ULP,
        |v| v.sinh_u35(),
        f32::sinh,
    );
    check_targeted_unary_f32::<S>(
        "cosh_u35",
        &inputs,
        contracts::COSH_U35_F32_MAX_ULP,
        |v| v.cosh_u35(),
        f32::cosh,
    );
    check_targeted_unary_f32::<S>(
        "tanh_u35",
        &inputs,
        contracts::TANH_U35_F32_MAX_ULP,
        |v| v.tanh_u35(),
        f32::tanh,
    );
}

fn run_f32_hyperbolic_special_values_and_mixed_lanes<S: Simd>() {
    let mut inputs = vec![
        f32::NAN,
        f32::from_bits(0x7FC0_1234),
        f32::NEG_INFINITY,
        f32::INFINITY,
        -0.0,
        0.0,
        -80.0,
        80.0,
        -0.5,
        0.5,
        -1.0e-6,
        1.0e-6,
        -20.0,
        20.0,
        -100.0,
        100.0,
    ];

    while !inputs.len().is_multiple_of(S::Vf32::WIDTH) {
        inputs.push(0.25);
    }

    for chunk in inputs.chunks(S::Vf32::WIDTH) {
        let v = S::Vf32::load_from_slice(chunk);
        let sinh = v.sinh_u35();
        let cosh = v.cosh_u35();
        let tanh = v.tanh_u35();

        for (lane, &x) in chunk.iter().enumerate() {
            assert_f32_contract(
                "sinh_u35",
                x,
                sinh[lane],
                x.sinh(),
                contracts::SINH_U35_F32_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("{e}"));
            assert_f32_contract(
                "cosh_u35",
                x,
                cosh[lane],
                x.cosh(),
                contracts::COSH_U35_F32_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("{e}"));
            assert_f32_contract(
                "tanh_u35",
                x,
                tanh[lane],
                x.tanh(),
                contracts::TANH_U35_F32_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("{e}"));
        }
    }
}

simd_math_targeted_all_backends!(f32_hyperbolic_edges, run_f32_hyperbolic_edges);
simd_math_targeted_all_backends!(
    f32_hyperbolic_fast_path_boundaries,
    run_f32_hyperbolic_fast_path_boundaries
);
simd_math_targeted_all_backends!(
    f32_hyperbolic_special_values_and_mixed_lanes,
    run_f32_hyperbolic_special_values_and_mixed_lanes
);

fn run_f32_hyperbolic_signed_zero_semantics<S: Simd>() {
    let mut lanes = vec![0.0f32; S::Vf32::WIDTH];
    lanes[0] = -0.0;

    let input = S::Vf32::load_from_slice(&lanes);
    let sinh = input.sinh_u35();
    let tanh = input.tanh_u35();

    assert_eq!(sinh[0].to_bits(), (-0.0f32).sinh().to_bits());
    assert_eq!(tanh[0].to_bits(), (-0.0f32).tanh().to_bits());

    if S::Vf32::WIDTH > 1 {
        assert_eq!(sinh[1].to_bits(), 0.0f32.sinh().to_bits());
        assert_eq!(tanh[1].to_bits(), 0.0f32.tanh().to_bits());
    }

    let cosh = input.cosh_u35();
    assert_eq!(cosh[0].to_bits(), (-0.0f32).cosh().to_bits());
}

simd_math_targeted_all_backends!(
    f32_hyperbolic_signed_zero_semantics,
    run_f32_hyperbolic_signed_zero_semantics
);

fn push_boundary_triplet_f64(inputs: &mut Vec<f64>, center: f64) {
    inputs.push(f64::from_bits(center.to_bits().saturating_sub(1)));
    inputs.push(center);
    inputs.push(f64::from_bits(center.to_bits().saturating_add(1)));
}

fn run_f64_hyperbolic_fast_path_boundaries<S: Simd>() {
    let mut inputs = Vec::new();
    for center in [
        -30.0f64, -20.0, -0.625, -0.5, -0.0, 0.0, 0.5, 0.625, 20.0, 30.0,
    ] {
        push_boundary_triplet_f64(&mut inputs, center);
    }

    check_targeted_unary_f64::<S>(
        "sinh_u35",
        &inputs,
        contracts::SINH_U35_F64_MAX_ULP,
        |v| v.sinh_u35(),
        f64::sinh,
    );
    check_targeted_unary_f64::<S>(
        "cosh_u35",
        &inputs,
        contracts::COSH_U35_F64_MAX_ULP,
        |v| v.cosh_u35(),
        f64::cosh,
    );
    check_targeted_unary_f64::<S>(
        "tanh_u35",
        &inputs,
        contracts::TANH_U35_F64_MAX_ULP,
        |v| v.tanh_u35(),
        f64::tanh,
    );
}

fn run_f64_hyperbolic_special_values_and_mixed_lanes<S: Simd>() {
    let mut inputs = vec![
        f64::NAN,
        f64::from_bits(0x7FF8_0000_0000_1234),
        f64::NEG_INFINITY,
        f64::INFINITY,
        -0.0,
        0.0,
        -20.0,
        20.0,
        -0.5,
        0.5,
        -1.0e-12,
        1.0e-12,
        -5.0,
        5.0,
        -100.0,
        100.0,
    ];

    while !inputs.len().is_multiple_of(S::Vf64::WIDTH) {
        inputs.push(0.25);
    }

    for chunk in inputs.chunks(S::Vf64::WIDTH) {
        let v = S::Vf64::load_from_slice(chunk);
        let sinh = v.sinh_u35();
        let cosh = v.cosh_u35();
        let tanh = v.tanh_u35();

        for (lane, &x) in chunk.iter().enumerate() {
            assert_f64_contract(
                "sinh_u35",
                x,
                sinh[lane],
                x.sinh(),
                contracts::SINH_U35_F64_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("{e}"));
            assert_f64_contract(
                "cosh_u35",
                x,
                cosh[lane],
                x.cosh(),
                contracts::COSH_U35_F64_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("{e}"));
            assert_f64_contract(
                "tanh_u35",
                x,
                tanh[lane],
                x.tanh(),
                contracts::TANH_U35_F64_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("{e}"));
        }
    }
}

fn run_f64_hyperbolic_edges<S: Simd>() {
    let inputs = [
        -100.0f64, -40.0, -20.0, -10.0, -1.0, -0.5, -0.0, 0.0, 0.5, 1.0, 10.0, 20.0, 40.0, 100.0,
    ];
    for chunk in inputs.chunks(S::Vf64::WIDTH) {
        let v = S::Vf64::load_from_slice(chunk);
        let sinh = v.sinh_u35();
        let cosh = v.cosh_u35();
        let tanh = v.tanh_u35();

        for (lane, &x) in chunk.iter().enumerate() {
            assert_f64_contract(
                "sinh_u35",
                x,
                sinh[lane],
                x.sinh(),
                contracts::SINH_U35_F64_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("{e}"));
            assert_f64_contract(
                "cosh_u35",
                x,
                cosh[lane],
                x.cosh(),
                contracts::COSH_U35_F64_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("{e}"));
            assert_f64_contract(
                "tanh_u35",
                x,
                tanh[lane],
                x.tanh(),
                contracts::TANH_U35_F64_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("{e}"));
        }
    }
}

simd_math_targeted_all_backends!(f64_hyperbolic_edges, run_f64_hyperbolic_edges);
simd_math_targeted_all_backends!(
    f64_hyperbolic_fast_path_boundaries,
    run_f64_hyperbolic_fast_path_boundaries
);
simd_math_targeted_all_backends!(
    f64_hyperbolic_special_values_and_mixed_lanes,
    run_f64_hyperbolic_special_values_and_mixed_lanes
);

fn run_f64_hyperbolic_scalar_patch_cutover<S: Simd>() {
    let mut inputs = Vec::new();
    for center in [-1.0f64, 1.0] {
        push_boundary_triplet_f64(&mut inputs, center);
    }
    inputs.extend_from_slice(&[-20.0, -1.0e-12, -0.0, 0.0, 1.0e-12, 20.0]);

    while !inputs.len().is_multiple_of(S::Vf64::WIDTH) {
        inputs.push(0.25);
    }

    for chunk in inputs.chunks(S::Vf64::WIDTH) {
        let v = S::Vf64::load_from_slice(chunk);
        let sinh = v.sinh_u35();
        let tanh = v.tanh_u35();

        for (lane, &x) in chunk.iter().enumerate() {
            assert_f64_contract(
                "sinh_u35",
                x,
                sinh[lane],
                x.sinh(),
                contracts::SINH_U35_F64_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("scalar-patch sinh_u35({x:?}) {e}"));
            assert_f64_contract(
                "tanh_u35",
                x,
                tanh[lane],
                x.tanh(),
                contracts::TANH_U35_F64_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("scalar-patch tanh_u35({x:?}) {e}"));
        }
    }
}

simd_math_targeted_all_backends!(
    f64_hyperbolic_scalar_patch_cutover,
    run_f64_hyperbolic_scalar_patch_cutover
);

fn run_f64_hyperbolic_signed_zero_semantics<S: Simd>() {
    let mut lanes = vec![0.0f64; S::Vf64::WIDTH];
    lanes[0] = -0.0;

    let input = S::Vf64::load_from_slice(&lanes);
    let sinh = input.sinh_u35();
    let tanh = input.tanh_u35();

    assert_eq!(sinh[0].to_bits(), (-0.0f64).sinh().to_bits());
    assert_eq!(tanh[0].to_bits(), (-0.0f64).tanh().to_bits());

    if S::Vf64::WIDTH > 1 {
        assert_eq!(sinh[1].to_bits(), 0.0f64.sinh().to_bits());
        assert_eq!(tanh[1].to_bits(), 0.0f64.tanh().to_bits());
    }

    let cosh = input.cosh_u35();
    assert_eq!(cosh[0].to_bits(), (-0.0f64).cosh().to_bits());
}

simd_math_targeted_all_backends!(
    f64_hyperbolic_signed_zero_semantics,
    run_f64_hyperbolic_signed_zero_semantics
);
