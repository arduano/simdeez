use super::*;
use crate::math::{SimdMathF32Hyperbolic, SimdMathF64Hyperbolic};

fn run_f32_sinh_u35_contract<S: Simd>() {
    check_unary_f32::<S>(
        "sinh_u35",
        contracts::SINH_U35_F32_MAX_ULP,
        |v| v.sinh_u35(),
        f32::sinh,
    );
}

fn run_f32_cosh_u35_contract<S: Simd>() {
    check_unary_f32::<S>(
        "cosh_u35",
        contracts::COSH_U35_F32_MAX_ULP,
        |v| v.cosh_u35(),
        f32::cosh,
    );
}

fn run_f32_tanh_u35_contract<S: Simd>() {
    check_unary_f32::<S>(
        "tanh_u35",
        contracts::TANH_U35_F32_MAX_ULP,
        |v| v.tanh_u35(),
        f32::tanh,
    );
}

fn run_f64_sinh_u35_contract<S: Simd>() {
    check_unary_f64::<S>(
        "sinh_u35",
        contracts::SINH_U35_F64_MAX_ULP,
        |v| v.sinh_u35(),
        f64::sinh,
    );
}

fn run_f64_cosh_u35_contract<S: Simd>() {
    check_unary_f64::<S>(
        "cosh_u35",
        contracts::COSH_U35_F64_MAX_ULP,
        |v| v.cosh_u35(),
        f64::cosh,
    );
}

fn run_f64_tanh_u35_contract<S: Simd>() {
    check_unary_f64::<S>(
        "tanh_u35",
        contracts::TANH_U35_F64_MAX_ULP,
        |v| v.tanh_u35(),
        f64::tanh,
    );
}

simd_math_all_backends!(f32_sinh_u35_contract, run_f32_sinh_u35_contract);
simd_math_all_backends!(f32_cosh_u35_contract, run_f32_cosh_u35_contract);
simd_math_all_backends!(f32_tanh_u35_contract, run_f32_tanh_u35_contract);
simd_math_all_backends!(f64_sinh_u35_contract, run_f64_sinh_u35_contract);
simd_math_all_backends!(f64_cosh_u35_contract, run_f64_cosh_u35_contract);
simd_math_all_backends!(f64_tanh_u35_contract, run_f64_tanh_u35_contract);
