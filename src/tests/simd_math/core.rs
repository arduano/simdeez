use super::*;
use crate::math::{SimdMathF32Core, SimdMathF64Core};

fn run_f32_log2_u35_contract<S: Simd>() {
    check_unary_f32::<S>(
        "log2_u35",
        contracts::LOG2_U35_F32_MAX_ULP,
        |v| v.log2_u35(),
        f32::log2,
    );
}

fn run_f32_exp2_u35_contract<S: Simd>() {
    check_unary_f32::<S>(
        "exp2_u35",
        contracts::EXP2_U35_F32_MAX_ULP,
        |v| v.exp2_u35(),
        f32::exp2,
    );
}

fn run_f32_ln_u35_contract<S: Simd>() {
    check_unary_f32::<S>(
        "ln_u35",
        contracts::LN_U35_F32_MAX_ULP,
        |v| v.ln_u35(),
        f32::ln,
    );
}

fn run_f32_exp_u35_contract<S: Simd>() {
    check_unary_f32::<S>(
        "exp_u35",
        contracts::EXP_U35_F32_MAX_ULP,
        |v| v.exp_u35(),
        f32::exp,
    );
}

fn run_f32_sin_u35_contract<S: Simd>() {
    check_unary_f32::<S>(
        "sin_u35",
        contracts::SIN_U35_F32_MAX_ULP,
        |v| v.sin_u35(),
        f32::sin,
    );
}

fn run_f32_cos_u35_contract<S: Simd>() {
    check_unary_f32::<S>(
        "cos_u35",
        contracts::COS_U35_F32_MAX_ULP,
        |v| v.cos_u35(),
        f32::cos,
    );
}

fn run_f32_tan_u35_contract<S: Simd>() {
    check_unary_f32::<S>(
        "tan_u35",
        contracts::TAN_U35_F32_MAX_ULP,
        |v| v.tan_u35(),
        f32::tan,
    );
}

fn run_f64_log2_u35_contract<S: Simd>() {
    check_unary_f64::<S>(
        "log2_u35",
        contracts::LOG2_U35_F64_MAX_ULP,
        |v| v.log2_u35(),
        f64::log2,
    );
}

fn run_f64_exp2_u35_contract<S: Simd>() {
    check_unary_f64::<S>(
        "exp2_u35",
        contracts::EXP2_U35_F64_MAX_ULP,
        |v| v.exp2_u35(),
        f64::exp2,
    );
}

fn run_f64_ln_u35_contract<S: Simd>() {
    check_unary_f64::<S>(
        "ln_u35",
        contracts::LN_U35_F64_MAX_ULP,
        |v| v.ln_u35(),
        f64::ln,
    );
}

fn run_f64_exp_u35_contract<S: Simd>() {
    check_unary_f64::<S>(
        "exp_u35",
        contracts::EXP_U35_F64_MAX_ULP,
        |v| v.exp_u35(),
        f64::exp,
    );
}

fn run_f64_sin_u35_contract<S: Simd>() {
    check_unary_f64::<S>(
        "sin_u35",
        contracts::SIN_U35_F64_MAX_ULP,
        |v| v.sin_u35(),
        f64::sin,
    );
}

fn run_f64_cos_u35_contract<S: Simd>() {
    check_unary_f64::<S>(
        "cos_u35",
        contracts::COS_U35_F64_MAX_ULP,
        |v| v.cos_u35(),
        f64::cos,
    );
}

fn run_f64_tan_u35_contract<S: Simd>() {
    check_unary_f64::<S>(
        "tan_u35",
        contracts::TAN_U35_F64_MAX_ULP,
        |v| v.tan_u35(),
        f64::tan,
    );
}

simd_math_all_backends!(f32_log2_u35_contract, run_f32_log2_u35_contract);
simd_math_all_backends!(f32_exp2_u35_contract, run_f32_exp2_u35_contract);
simd_math_all_backends!(f32_ln_u35_contract, run_f32_ln_u35_contract);
simd_math_all_backends!(f32_exp_u35_contract, run_f32_exp_u35_contract);
simd_math_all_backends!(f32_sin_u35_contract, run_f32_sin_u35_contract);
simd_math_all_backends!(f32_cos_u35_contract, run_f32_cos_u35_contract);
simd_math_all_backends!(f32_tan_u35_contract, run_f32_tan_u35_contract);
simd_math_all_backends!(f64_log2_u35_contract, run_f64_log2_u35_contract);
simd_math_all_backends!(f64_exp2_u35_contract, run_f64_exp2_u35_contract);
simd_math_all_backends!(f64_ln_u35_contract, run_f64_ln_u35_contract);
simd_math_all_backends!(f64_exp_u35_contract, run_f64_exp_u35_contract);
simd_math_all_backends!(f64_sin_u35_contract, run_f64_sin_u35_contract);
simd_math_all_backends!(f64_cos_u35_contract, run_f64_cos_u35_contract);
simd_math_all_backends!(f64_tan_u35_contract, run_f64_tan_u35_contract);
