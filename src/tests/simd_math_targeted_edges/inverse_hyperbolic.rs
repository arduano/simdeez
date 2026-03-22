use super::*;
use crate::math::{SimdMathF32InverseHyperbolic, SimdMathF64InverseHyperbolic};

fn run_f32_inverse_hyperbolic_domain_edges<S: Simd>() {
    let acosh_inputs = [
        0.0f32,
        0.5,
        1.0 - f32::EPSILON,
        1.0,
        1.0 + f32::EPSILON,
        1.000_001,
        2.0,
        10.0,
        f32::INFINITY,
        f32::NAN,
    ];
    check_targeted_unary_f32::<S>(
        "acosh_u35",
        &acosh_inputs,
        contracts::ACOSH_U35_F32_MAX_ULP,
        |v| v.acosh_u35(),
        f32::acosh,
    );

    let atanh_inputs = [
        -1.0f32,
        -1.0 + f32::EPSILON,
        -0.999_999,
        -0.5,
        -0.0,
        0.0,
        0.5,
        0.999_999,
        1.0 - f32::EPSILON,
        1.0,
        1.0 + f32::EPSILON,
        f32::NAN,
    ];
    check_targeted_unary_f32::<S>(
        "atanh_u35",
        &atanh_inputs,
        contracts::ATANH_U35_F32_MAX_ULP,
        |v| v.atanh_u35(),
        f32::atanh,
    );

    let asinh_inputs = [
        f32::NEG_INFINITY,
        -1.0e20,
        -1000.0,
        -1.0,
        -f32::MIN_POSITIVE,
        -0.0,
        0.0,
        f32::MIN_POSITIVE,
        1.0,
        1000.0,
        1.0e20,
        f32::INFINITY,
        f32::NAN,
    ];
    check_targeted_unary_f32::<S>(
        "asinh_u35",
        &asinh_inputs,
        contracts::ASINH_U35_F32_MAX_ULP,
        |v| v.asinh_u35(),
        f32::asinh,
    );
}

fn run_f32_inverse_hyperbolic_mixed_lanes<S: Simd>() {
    let mut lanes = vec![0.0f32; S::Vf32::WIDTH];
    let pattern = [
        -0.75f32,
        -0.0,
        0.0,
        0.75,
        0.999_999,
        -0.999_999,
        1.0,
        -1.0,
        1.0 + f32::EPSILON,
        f32::NAN,
        f32::INFINITY,
        f32::NEG_INFINITY,
        1.0,
        2.0,
        10.0,
        0.5,
    ];
    for (i, lane) in lanes.iter_mut().enumerate() {
        *lane = pattern[i % pattern.len()];
    }

    let input = S::Vf32::load_from_slice(&lanes);
    let asinh = input.asinh_u35();
    let acosh = input.acosh_u35();
    let atanh = input.atanh_u35();

    for (lane, &x) in lanes.iter().enumerate() {
        assert_f32_contract(
            "asinh_u35",
            x,
            asinh[lane],
            x.asinh(),
            contracts::ASINH_U35_F32_MAX_ULP,
        )
        .unwrap_or_else(|e| panic!("lane {lane}: {e}"));

        assert_f32_contract(
            "acosh_u35",
            x,
            acosh[lane],
            x.acosh(),
            contracts::ACOSH_U35_F32_MAX_ULP,
        )
        .unwrap_or_else(|e| panic!("lane {lane}: {e}"));

        assert_f32_contract(
            "atanh_u35",
            x,
            atanh[lane],
            x.atanh(),
            contracts::ATANH_U35_F32_MAX_ULP,
        )
        .unwrap_or_else(|e| panic!("lane {lane}: {e}"));
    }
}

simd_math_targeted_all_backends!(
    f32_inverse_hyperbolic_domain_edges,
    run_f32_inverse_hyperbolic_domain_edges
);
simd_math_targeted_all_backends!(
    f32_inverse_hyperbolic_mixed_lanes,
    run_f32_inverse_hyperbolic_mixed_lanes
);

fn run_f64_inverse_hyperbolic_domain_edges<S: Simd>() {
    let acosh_inputs = [
        0.0f64,
        0.5,
        1.0 - f64::EPSILON,
        1.0,
        1.0 + f64::EPSILON,
        1.000_000_000_001,
        2.0,
        10.0,
        f64::INFINITY,
        f64::NAN,
    ];
    check_targeted_unary_f64::<S>(
        "acosh_u35",
        &acosh_inputs,
        contracts::ACOSH_U35_F64_MAX_ULP,
        |v| v.acosh_u35(),
        f64::acosh,
    );

    let atanh_inputs = [
        -1.0f64,
        -1.0 + f64::EPSILON,
        -0.999_999_999_999,
        -0.5,
        -0.0,
        0.0,
        0.5,
        0.999_999_999_999,
        1.0 - f64::EPSILON,
        1.0,
        1.0 + f64::EPSILON,
        f64::NAN,
    ];
    check_targeted_unary_f64::<S>(
        "atanh_u35",
        &atanh_inputs,
        contracts::ATANH_U35_F64_MAX_ULP,
        |v| v.atanh_u35(),
        f64::atanh,
    );

    let asinh_inputs = [
        f64::NEG_INFINITY,
        -1.0e300,
        -1000.0,
        -1.0,
        -f64::MIN_POSITIVE,
        -0.0,
        0.0,
        f64::MIN_POSITIVE,
        1.0,
        1000.0,
        1.0e300,
        f64::INFINITY,
        f64::NAN,
    ];
    check_targeted_unary_f64::<S>(
        "asinh_u35",
        &asinh_inputs,
        contracts::ASINH_U35_F64_MAX_ULP,
        |v| v.asinh_u35(),
        f64::asinh,
    );
}

fn run_f64_inverse_hyperbolic_mixed_lanes<S: Simd>() {
    let mut lanes = vec![0.0f64; S::Vf64::WIDTH];
    let pattern = [
        -0.75f64,
        -0.0,
        0.0,
        0.75,
        0.999_999_999_999,
        -0.999_999_999_999,
        1.0,
        -1.0,
        1.0 + f64::EPSILON,
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        1.0,
        2.0,
        10.0,
        0.5,
    ];
    for (i, lane) in lanes.iter_mut().enumerate() {
        *lane = pattern[i % pattern.len()];
    }

    let input = S::Vf64::load_from_slice(&lanes);
    let asinh = input.asinh_u35();
    let acosh = input.acosh_u35();
    let atanh = input.atanh_u35();

    for (lane, &x) in lanes.iter().enumerate() {
        assert_f64_contract(
            "asinh_u35",
            x,
            asinh[lane],
            x.asinh(),
            contracts::ASINH_U35_F64_MAX_ULP,
        )
        .unwrap_or_else(|e| panic!("lane {lane}: {e}"));

        assert_f64_contract(
            "acosh_u35",
            x,
            acosh[lane],
            x.acosh(),
            contracts::ACOSH_U35_F64_MAX_ULP,
        )
        .unwrap_or_else(|e| panic!("lane {lane}: {e}"));

        assert_f64_contract(
            "atanh_u35",
            x,
            atanh[lane],
            x.atanh(),
            contracts::ATANH_U35_F64_MAX_ULP,
        )
        .unwrap_or_else(|e| panic!("lane {lane}: {e}"));
    }
}

simd_math_targeted_all_backends!(
    f64_inverse_hyperbolic_domain_edges,
    run_f64_inverse_hyperbolic_domain_edges
);
simd_math_targeted_all_backends!(
    f64_inverse_hyperbolic_mixed_lanes,
    run_f64_inverse_hyperbolic_mixed_lanes
);

fn run_f64_inverse_hyperbolic_signed_zero_semantics<S: Simd>() {
    let mut lanes = vec![0.0f64; S::Vf64::WIDTH];
    lanes[0] = -0.0;

    let input = S::Vf64::load_from_slice(&lanes);
    let asinh = input.asinh_u35();
    let atanh = input.atanh_u35();

    assert_eq!(asinh[0].to_bits(), (-0.0f64).asinh().to_bits());
    assert_eq!(atanh[0].to_bits(), (-0.0f64).atanh().to_bits());

    if S::Vf64::WIDTH > 1 {
        assert_eq!(asinh[1].to_bits(), 0.0f64.asinh().to_bits());
        assert_eq!(atanh[1].to_bits(), 0.0f64.atanh().to_bits());
    }

    let ones = S::Vf64::set1(1.0);
    let acosh = ones.acosh_u35();
    assert_eq!(acosh[0].to_bits(), 1.0f64.acosh().to_bits());
}

simd_math_targeted_all_backends!(
    f64_inverse_hyperbolic_signed_zero_semantics,
    run_f64_inverse_hyperbolic_signed_zero_semantics
);
