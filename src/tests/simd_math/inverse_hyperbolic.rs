use super::*;
use crate::math::{SimdMathF32InverseHyperbolic, SimdMathF64InverseHyperbolic};

fn run_f32_asinh_u35_contract<S: Simd>() {
    check_unary_f32::<S>(
        "asinh_u35",
        contracts::ASINH_U35_F32_MAX_ULP,
        |v| v.asinh_u35(),
        f32::asinh,
    );
}

fn run_f32_acosh_u35_contract<S: Simd>() {
    check_unary_f32::<S>(
        "acosh_u35",
        contracts::ACOSH_U35_F32_MAX_ULP,
        |v| v.acosh_u35(),
        f32::acosh,
    );
}

fn run_f32_atanh_u35_contract<S: Simd>() {
    check_unary_f32::<S>(
        "atanh_u35",
        contracts::ATANH_U35_F32_MAX_ULP,
        |v| v.atanh_u35(),
        f32::atanh,
    );
}

fn run_f64_asinh_u35_contract<S: Simd>() {
    check_unary_f64::<S>(
        "asinh_u35",
        contracts::ASINH_U35_F64_MAX_ULP,
        |v| v.asinh_u35(),
        f64::asinh,
    );
}

fn run_f64_acosh_u35_contract<S: Simd>() {
    check_unary_f64::<S>(
        "acosh_u35",
        contracts::ACOSH_U35_F64_MAX_ULP,
        |v| v.acosh_u35(),
        f64::acosh,
    );
}

fn run_f64_atanh_u35_contract<S: Simd>() {
    check_unary_f64::<S>(
        "atanh_u35",
        contracts::ATANH_U35_F64_MAX_ULP,
        |v| v.atanh_u35(),
        f64::atanh,
    );
}

simd_math_all_backends!(f32_asinh_u35_contract, run_f32_asinh_u35_contract);
simd_math_all_backends!(f32_acosh_u35_contract, run_f32_acosh_u35_contract);
simd_math_all_backends!(f32_atanh_u35_contract, run_f32_atanh_u35_contract);
simd_math_all_backends!(f64_asinh_u35_contract, run_f64_asinh_u35_contract);
simd_math_all_backends!(f64_acosh_u35_contract, run_f64_acosh_u35_contract);
simd_math_all_backends!(f64_atanh_u35_contract, run_f64_atanh_u35_contract);
