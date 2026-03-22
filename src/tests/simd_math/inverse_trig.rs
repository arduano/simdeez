use super::*;
use crate::math::{SimdMathF32InverseTrig, SimdMathF64InverseTrig};

fn run_f32_asin_u35_contract<S: Simd>() {
    check_unary_f32::<S>(
        "asin_u35",
        contracts::ASIN_U35_F32_MAX_ULP,
        |v| v.asin_u35(),
        f32::asin,
    );
}

fn run_f32_acos_u35_contract<S: Simd>() {
    check_unary_f32::<S>(
        "acos_u35",
        contracts::ACOS_U35_F32_MAX_ULP,
        |v| v.acos_u35(),
        f32::acos,
    );
}

fn run_f32_atan_u35_contract<S: Simd>() {
    check_unary_f32::<S>(
        "atan_u35",
        contracts::ATAN_U35_F32_MAX_ULP,
        |v| v.atan_u35(),
        f32::atan,
    );
}

fn run_f64_asin_u35_contract<S: Simd>() {
    check_unary_f64::<S>(
        "asin_u35",
        contracts::ASIN_U35_F64_MAX_ULP,
        |v| v.asin_u35(),
        f64::asin,
    );
}

fn run_f64_acos_u35_contract<S: Simd>() {
    check_unary_f64::<S>(
        "acos_u35",
        contracts::ACOS_U35_F64_MAX_ULP,
        |v| v.acos_u35(),
        f64::acos,
    );
}

fn run_f64_atan_u35_contract<S: Simd>() {
    check_unary_f64::<S>(
        "atan_u35",
        contracts::ATAN_U35_F64_MAX_ULP,
        |v| v.atan_u35(),
        f64::atan,
    );
}

simd_math_all_backends!(f32_asin_u35_contract, run_f32_asin_u35_contract);
simd_math_all_backends!(f32_acos_u35_contract, run_f32_acos_u35_contract);
simd_math_all_backends!(f32_atan_u35_contract, run_f32_atan_u35_contract);
simd_math_all_backends!(f64_asin_u35_contract, run_f64_asin_u35_contract);
simd_math_all_backends!(f64_acos_u35_contract, run_f64_acos_u35_contract);
simd_math_all_backends!(f64_atan_u35_contract, run_f64_atan_u35_contract);
