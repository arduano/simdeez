use super::*;
use crate::math::{SimdMathF32BinaryMisc, SimdMathF64BinaryMisc};

fn run_f32_log10_u35_contract<S: Simd>() {
    check_unary_f32::<S>(
        "log10_u35",
        contracts::LOG10_U35_F32_MAX_ULP,
        |v| v.log10_u35(),
        f32::log10,
    );
}

fn run_f32_atan2_u35_contract<S: Simd>() {
    check_binary_f32::<S>(
        "atan2_u35",
        contracts::ATAN2_U35_F32_MAX_ULP,
        |x, y| x.atan2_u35(y),
        f32::atan2,
    );
}

fn run_f32_hypot_u35_contract<S: Simd>() {
    check_binary_f32::<S>(
        "hypot_u35",
        contracts::HYPOT_U35_F32_MAX_ULP,
        |x, y| x.hypot_u35(y),
        f32::hypot,
    );
}

fn run_f32_fmod_contract<S: Simd>() {
    check_binary_f32::<S>("fmod", 0, |x, y| x.fmod(y), |x, y| x % y);
}

fn run_f64_log10_u35_contract<S: Simd>() {
    check_unary_f64::<S>(
        "log10_u35",
        contracts::LOG10_U35_F64_MAX_ULP,
        |v| v.log10_u35(),
        f64::log10,
    );
}

fn run_f64_atan2_u35_contract<S: Simd>() {
    check_binary_f64::<S>(
        "atan2_u35",
        contracts::ATAN2_U35_F64_MAX_ULP,
        |x, y| x.atan2_u35(y),
        f64::atan2,
    );
}

fn run_f64_hypot_u35_contract<S: Simd>() {
    check_binary_f64::<S>(
        "hypot_u35",
        contracts::HYPOT_U35_F64_MAX_ULP,
        |x, y| x.hypot_u35(y),
        f64::hypot,
    );
}

fn run_f64_fmod_contract<S: Simd>() {
    check_binary_f64::<S>("fmod", 0, |x, y| x.fmod(y), |x, y| x % y);
}

simd_math_all_backends!(f32_log10_u35_contract, run_f32_log10_u35_contract);
simd_math_all_backends!(f32_atan2_u35_contract, run_f32_atan2_u35_contract);
simd_math_all_backends!(f32_hypot_u35_contract, run_f32_hypot_u35_contract);
simd_math_all_backends!(f32_fmod_contract, run_f32_fmod_contract);
simd_math_all_backends!(f64_log10_u35_contract, run_f64_log10_u35_contract);
simd_math_all_backends!(f64_atan2_u35_contract, run_f64_atan2_u35_contract);
simd_math_all_backends!(f64_hypot_u35_contract, run_f64_hypot_u35_contract);
simd_math_all_backends!(f64_fmod_contract, run_f64_fmod_contract);
