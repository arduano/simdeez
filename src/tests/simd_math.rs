#![allow(unused_imports)]

use rand::Rng;
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};

use super::*;

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
use crate::engines::avx512::Avx512;
#[cfg(target_arch = "aarch64")]
use crate::engines::neon::Neon;
use crate::engines::scalar::Scalar;
#[cfg(target_arch = "wasm32")]
use crate::engines::wasm32::Wasm;
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
use crate::engines::{avx2::Avx2, sse2::Sse2, sse41::Sse41};

use crate::math::{contracts, SimdMathF32, SimdMathF64};
use crate::{Simd, SimdBaseIo, SimdConsts};

fn f32_input_space() -> Vec<f32> {
    let mut values = vec![
        f32::NAN,
        f32::from_bits(0x7FC0_1234),
        f32::INFINITY,
        f32::NEG_INFINITY,
        0.0,
        -0.0,
        1.0,
        -1.0,
        2.0,
        0.5,
        f32::MIN_POSITIVE,
        f32::from_bits(1),
        f32::MAX,
        f32::MIN,
        core::f32::consts::PI,
        -core::f32::consts::PI,
        core::f32::consts::E,
        -100.0,
        100.0,
        128.0,
        -150.0,
    ];

    let mut rng = ChaCha8Rng::seed_from_u64(0x51DE_EF32);
    for _ in 0..10_000 {
        values.push(f32::from_bits(rng.gen::<u32>()));
    }

    values
}

fn f64_input_space() -> Vec<f64> {
    let mut values = vec![
        f64::NAN,
        f64::from_bits(0x7FF8_0000_0000_1234),
        f64::INFINITY,
        f64::NEG_INFINITY,
        0.0,
        -0.0,
        1.0,
        -1.0,
        2.0,
        0.5,
        f64::MIN_POSITIVE,
        f64::from_bits(1),
        f64::MAX,
        f64::MIN,
        core::f64::consts::PI,
        -core::f64::consts::PI,
        core::f64::consts::E,
        -1000.0,
        1000.0,
        1024.0,
        -2000.0,
    ];

    let mut rng = ChaCha8Rng::seed_from_u64(0x51DE_EF64);
    for _ in 0..8_000 {
        values.push(f64::from_bits(rng.gen::<u64>()));
    }

    values
}

fn assert_f32_contract(
    fn_name: &str,
    input: f32,
    actual: f32,
    expected: f32,
    max_ulp: u32,
) -> Result<(), String> {
    if expected.is_nan() {
        if actual.is_nan() {
            return Ok(());
        }
        return Err(format!("{fn_name}({input:?}) expected NaN, got {actual:?}"));
    }

    if expected.is_infinite() {
        if actual.to_bits() == expected.to_bits() {
            return Ok(());
        }
        return Err(format!(
            "{fn_name}({input:?}) expected {:?}, got {:?}",
            expected, actual
        ));
    }

    if expected == 0.0 {
        if actual.to_bits() == expected.to_bits() {
            return Ok(());
        }
        return Err(format!(
            "{fn_name}({input:?}) expected signed zero bits {:08x}, got {:08x}",
            expected.to_bits(),
            actual.to_bits()
        ));
    }

    if actual.is_nan() || actual.is_infinite() {
        return Err(format!(
            "{fn_name}({input:?}) expected finite {expected:?}, got {actual:?}"
        ));
    }

    let ulp = ulp_distance_f32(actual, expected)
        .ok_or_else(|| format!("{fn_name}({input:?}) failed to compute f32 ULP distance"))?;
    if ulp > max_ulp {
        return Err(format!(
            "{fn_name}({input:?}) ULP distance {ulp} exceeds max {max_ulp} (actual={actual:?}, expected={expected:?})"
        ));
    }

    Ok(())
}

fn assert_f64_contract(
    fn_name: &str,
    input: f64,
    actual: f64,
    expected: f64,
    max_ulp: u64,
) -> Result<(), String> {
    if expected.is_nan() {
        if actual.is_nan() {
            return Ok(());
        }
        return Err(format!("{fn_name}({input:?}) expected NaN, got {actual:?}"));
    }

    if expected.is_infinite() {
        if actual.to_bits() == expected.to_bits() {
            return Ok(());
        }
        return Err(format!(
            "{fn_name}({input:?}) expected {:?}, got {:?}",
            expected, actual
        ));
    }

    if expected == 0.0 {
        if actual.to_bits() == expected.to_bits() {
            return Ok(());
        }
        return Err(format!(
            "{fn_name}({input:?}) expected signed zero bits {:016x}, got {:016x}",
            expected.to_bits(),
            actual.to_bits()
        ));
    }

    if actual.is_nan() || actual.is_infinite() {
        return Err(format!(
            "{fn_name}({input:?}) expected finite {expected:?}, got {actual:?}"
        ));
    }

    let ulp = ulp_distance_f64(actual, expected)
        .ok_or_else(|| format!("{fn_name}({input:?}) failed to compute f64 ULP distance"))?;
    if ulp > max_ulp {
        return Err(format!(
            "{fn_name}({input:?}) ULP distance {ulp} exceeds max {max_ulp} (actual={actual:?}, expected={expected:?})"
        ));
    }

    Ok(())
}

fn check_unary_f32<S: Simd>(
    fn_name: &str,
    max_ulp: u32,
    simd_fn: impl Fn(S::Vf32) -> S::Vf32,
    scalar_ref: impl Fn(f32) -> f32,
) {
    for chunk in f32_input_space().chunks(S::Vf32::WIDTH) {
        let input = S::Vf32::load_from_slice(chunk);
        let output = simd_fn(input);

        for (lane, &x) in chunk.iter().enumerate() {
            let actual = output[lane];
            let expected = scalar_ref(x);
            if let Err(err) = assert_f32_contract(fn_name, x, actual, expected, max_ulp) {
                panic!("{err}");
            }
        }
    }
}

fn check_unary_f64<S: Simd>(
    fn_name: &str,
    max_ulp: u64,
    simd_fn: impl Fn(S::Vf64) -> S::Vf64,
    scalar_ref: impl Fn(f64) -> f64,
) {
    for chunk in f64_input_space().chunks(S::Vf64::WIDTH) {
        let input = S::Vf64::load_from_slice(chunk);
        let output = simd_fn(input);

        for (lane, &x) in chunk.iter().enumerate() {
            let actual = output[lane];
            let expected = scalar_ref(x);
            if let Err(err) = assert_f64_contract(fn_name, x, actual, expected, max_ulp) {
                panic!("{err}");
            }
        }
    }
}

fn check_binary_f32<S: Simd>(
    fn_name: &str,
    max_ulp: u32,
    simd_fn: impl Fn(S::Vf32, S::Vf32) -> S::Vf32,
    scalar_ref: impl Fn(f32, f32) -> f32,
) {
    let lhs_values = f32_input_space();
    let rhs_values = f32_input_space();
    for (lhs_chunk, rhs_chunk) in lhs_values
        .chunks(S::Vf32::WIDTH)
        .zip(rhs_values.chunks(S::Vf32::WIDTH))
    {
        let lhs = S::Vf32::load_from_slice(lhs_chunk);
        let rhs = S::Vf32::load_from_slice(rhs_chunk);
        let output = simd_fn(lhs, rhs);

        for lane in 0..lhs_chunk.len() {
            let x = lhs_chunk[lane];
            let y = rhs_chunk[lane];
            let actual = output[lane];
            let expected = scalar_ref(x, y);
            if let Err(err) = assert_f32_contract(fn_name, x, actual, expected, max_ulp) {
                panic!("{fn_name}({x:?}, {y:?}) lane {lane}: {err}");
            }
        }
    }
}

fn check_binary_f64<S: Simd>(
    fn_name: &str,
    max_ulp: u64,
    simd_fn: impl Fn(S::Vf64, S::Vf64) -> S::Vf64,
    scalar_ref: impl Fn(f64, f64) -> f64,
) {
    let lhs_values = f64_input_space();
    let rhs_values = f64_input_space();
    for (lhs_chunk, rhs_chunk) in lhs_values
        .chunks(S::Vf64::WIDTH)
        .zip(rhs_values.chunks(S::Vf64::WIDTH))
    {
        let lhs = S::Vf64::load_from_slice(lhs_chunk);
        let rhs = S::Vf64::load_from_slice(rhs_chunk);
        let output = simd_fn(lhs, rhs);

        for lane in 0..lhs_chunk.len() {
            let x = lhs_chunk[lane];
            let y = rhs_chunk[lane];
            let actual = output[lane];
            let expected = scalar_ref(x, y);
            if let Err(err) = assert_f64_contract(fn_name, x, actual, expected, max_ulp) {
                panic!("{fn_name}({x:?}, {y:?}) lane {lane}: {err}");
            }
        }
    }
}

fn run_f32_log2_u35_contract<S: Simd>() {
    check_unary_f32::<S>(
        "log2_u35",
        contracts::LOG2_U35_F32_MAX_ULP,
        <S::Vf32 as SimdMathF32>::log2_u35,
        f32::log2,
    );
}

fn run_f32_exp2_u35_contract<S: Simd>() {
    check_unary_f32::<S>(
        "exp2_u35",
        contracts::EXP2_U35_F32_MAX_ULP,
        <S::Vf32 as SimdMathF32>::exp2_u35,
        f32::exp2,
    );
}

fn run_f32_ln_u35_contract<S: Simd>() {
    check_unary_f32::<S>(
        "ln_u35",
        contracts::LN_U35_F32_MAX_ULP,
        <S::Vf32 as SimdMathF32>::ln_u35,
        f32::ln,
    );
}

fn run_f32_exp_u35_contract<S: Simd>() {
    check_unary_f32::<S>(
        "exp_u35",
        contracts::EXP_U35_F32_MAX_ULP,
        <S::Vf32 as SimdMathF32>::exp_u35,
        f32::exp,
    );
}

fn run_f32_sin_u35_contract<S: Simd>() {
    check_unary_f32::<S>(
        "sin_u35",
        contracts::SIN_U35_F32_MAX_ULP,
        <S::Vf32 as SimdMathF32>::sin_u35,
        f32::sin,
    );
}

fn run_f32_cos_u35_contract<S: Simd>() {
    check_unary_f32::<S>(
        "cos_u35",
        contracts::COS_U35_F32_MAX_ULP,
        <S::Vf32 as SimdMathF32>::cos_u35,
        f32::cos,
    );
}

fn run_f32_tan_u35_contract<S: Simd>() {
    check_unary_f32::<S>(
        "tan_u35",
        contracts::TAN_U35_F32_MAX_ULP,
        <S::Vf32 as SimdMathF32>::tan_u35,
        f32::tan,
    );
}

fn run_f64_log2_u35_contract<S: Simd>() {
    check_unary_f64::<S>(
        "log2_u35",
        contracts::LOG2_U35_F64_MAX_ULP,
        <S::Vf64 as SimdMathF64>::log2_u35,
        f64::log2,
    );
}

fn run_f64_exp2_u35_contract<S: Simd>() {
    check_unary_f64::<S>(
        "exp2_u35",
        contracts::EXP2_U35_F64_MAX_ULP,
        <S::Vf64 as SimdMathF64>::exp2_u35,
        f64::exp2,
    );
}

fn run_f64_ln_u35_contract<S: Simd>() {
    check_unary_f64::<S>(
        "ln_u35",
        contracts::LN_U35_F64_MAX_ULP,
        <S::Vf64 as SimdMathF64>::ln_u35,
        f64::ln,
    );
}

fn run_f64_exp_u35_contract<S: Simd>() {
    check_unary_f64::<S>(
        "exp_u35",
        contracts::EXP_U35_F64_MAX_ULP,
        <S::Vf64 as SimdMathF64>::exp_u35,
        f64::exp,
    );
}

fn run_f64_sin_u35_contract<S: Simd>() {
    check_unary_f64::<S>(
        "sin_u35",
        contracts::SIN_U35_F64_MAX_ULP,
        <S::Vf64 as SimdMathF64>::sin_u35,
        f64::sin,
    );
}

fn run_f64_cos_u35_contract<S: Simd>() {
    check_unary_f64::<S>(
        "cos_u35",
        contracts::COS_U35_F64_MAX_ULP,
        <S::Vf64 as SimdMathF64>::cos_u35,
        f64::cos,
    );
}

fn run_f64_tan_u35_contract<S: Simd>() {
    check_unary_f64::<S>(
        "tan_u35",
        contracts::TAN_U35_F64_MAX_ULP,
        <S::Vf64 as SimdMathF64>::tan_u35,
        f64::tan,
    );
}

fn run_f32_asin_u35_contract<S: Simd>() {
    check_unary_f32::<S>(
        "asin_u35",
        contracts::ASIN_U35_F32_MAX_ULP,
        <S::Vf32 as SimdMathF32>::asin_u35,
        f32::asin,
    );
}

fn run_f32_acos_u35_contract<S: Simd>() {
    check_unary_f32::<S>(
        "acos_u35",
        contracts::ACOS_U35_F32_MAX_ULP,
        <S::Vf32 as SimdMathF32>::acos_u35,
        f32::acos,
    );
}

fn run_f32_atan_u35_contract<S: Simd>() {
    check_unary_f32::<S>(
        "atan_u35",
        contracts::ATAN_U35_F32_MAX_ULP,
        <S::Vf32 as SimdMathF32>::atan_u35,
        f32::atan,
    );
}

fn run_f32_sinh_u35_contract<S: Simd>() {
    check_unary_f32::<S>(
        "sinh_u35",
        contracts::SINH_U35_F32_MAX_ULP,
        <S::Vf32 as SimdMathF32>::sinh_u35,
        f32::sinh,
    );
}

fn run_f32_cosh_u35_contract<S: Simd>() {
    check_unary_f32::<S>(
        "cosh_u35",
        contracts::COSH_U35_F32_MAX_ULP,
        <S::Vf32 as SimdMathF32>::cosh_u35,
        f32::cosh,
    );
}

fn run_f32_tanh_u35_contract<S: Simd>() {
    check_unary_f32::<S>(
        "tanh_u35",
        contracts::TANH_U35_F32_MAX_ULP,
        <S::Vf32 as SimdMathF32>::tanh_u35,
        f32::tanh,
    );
}

fn run_f32_asinh_u35_contract<S: Simd>() {
    check_unary_f32::<S>(
        "asinh_u35",
        contracts::ASINH_U35_F32_MAX_ULP,
        <S::Vf32 as SimdMathF32>::asinh_u35,
        f32::asinh,
    );
}

fn run_f32_acosh_u35_contract<S: Simd>() {
    check_unary_f32::<S>(
        "acosh_u35",
        contracts::ACOSH_U35_F32_MAX_ULP,
        <S::Vf32 as SimdMathF32>::acosh_u35,
        f32::acosh,
    );
}

fn run_f32_atanh_u35_contract<S: Simd>() {
    check_unary_f32::<S>(
        "atanh_u35",
        contracts::ATANH_U35_F32_MAX_ULP,
        <S::Vf32 as SimdMathF32>::atanh_u35,
        f32::atanh,
    );
}

fn run_f32_log10_u35_contract<S: Simd>() {
    check_unary_f32::<S>(
        "log10_u35",
        contracts::LOG10_U35_F32_MAX_ULP,
        <S::Vf32 as SimdMathF32>::log10_u35,
        f32::log10,
    );
}

fn run_f32_atan2_u35_contract<S: Simd>() {
    check_binary_f32::<S>(
        "atan2_u35",
        contracts::ATAN2_U35_F32_MAX_ULP,
        <S::Vf32 as SimdMathF32>::atan2_u35,
        f32::atan2,
    );
}

fn run_f32_hypot_u35_contract<S: Simd>() {
    check_binary_f32::<S>(
        "hypot_u35",
        contracts::HYPOT_U35_F32_MAX_ULP,
        <S::Vf32 as SimdMathF32>::hypot_u35,
        f32::hypot,
    );
}

fn run_f32_fmod_contract<S: Simd>() {
    check_binary_f32::<S>("fmod", 0, <S::Vf32 as SimdMathF32>::fmod, |x, y| x % y);
}

fn run_f64_asin_u35_contract<S: Simd>() {
    check_unary_f64::<S>(
        "asin_u35",
        contracts::ASIN_U35_F64_MAX_ULP,
        <S::Vf64 as SimdMathF64>::asin_u35,
        f64::asin,
    );
}

fn run_f64_acos_u35_contract<S: Simd>() {
    check_unary_f64::<S>(
        "acos_u35",
        contracts::ACOS_U35_F64_MAX_ULP,
        <S::Vf64 as SimdMathF64>::acos_u35,
        f64::acos,
    );
}

fn run_f64_atan_u35_contract<S: Simd>() {
    check_unary_f64::<S>(
        "atan_u35",
        contracts::ATAN_U35_F64_MAX_ULP,
        <S::Vf64 as SimdMathF64>::atan_u35,
        f64::atan,
    );
}

fn run_f64_sinh_u35_contract<S: Simd>() {
    check_unary_f64::<S>(
        "sinh_u35",
        contracts::SINH_U35_F64_MAX_ULP,
        <S::Vf64 as SimdMathF64>::sinh_u35,
        f64::sinh,
    );
}

fn run_f64_cosh_u35_contract<S: Simd>() {
    check_unary_f64::<S>(
        "cosh_u35",
        contracts::COSH_U35_F64_MAX_ULP,
        <S::Vf64 as SimdMathF64>::cosh_u35,
        f64::cosh,
    );
}

fn run_f64_tanh_u35_contract<S: Simd>() {
    check_unary_f64::<S>(
        "tanh_u35",
        contracts::TANH_U35_F64_MAX_ULP,
        <S::Vf64 as SimdMathF64>::tanh_u35,
        f64::tanh,
    );
}

fn run_f64_asinh_u35_contract<S: Simd>() {
    check_unary_f64::<S>(
        "asinh_u35",
        contracts::ASINH_U35_F64_MAX_ULP,
        <S::Vf64 as SimdMathF64>::asinh_u35,
        f64::asinh,
    );
}

fn run_f64_acosh_u35_contract<S: Simd>() {
    check_unary_f64::<S>(
        "acosh_u35",
        contracts::ACOSH_U35_F64_MAX_ULP,
        <S::Vf64 as SimdMathF64>::acosh_u35,
        f64::acosh,
    );
}

fn run_f64_atanh_u35_contract<S: Simd>() {
    check_unary_f64::<S>(
        "atanh_u35",
        contracts::ATANH_U35_F64_MAX_ULP,
        <S::Vf64 as SimdMathF64>::atanh_u35,
        f64::atanh,
    );
}

fn run_f64_log10_u35_contract<S: Simd>() {
    check_unary_f64::<S>(
        "log10_u35",
        contracts::LOG10_U35_F64_MAX_ULP,
        <S::Vf64 as SimdMathF64>::log10_u35,
        f64::log10,
    );
}

fn run_f64_atan2_u35_contract<S: Simd>() {
    check_binary_f64::<S>(
        "atan2_u35",
        contracts::ATAN2_U35_F64_MAX_ULP,
        <S::Vf64 as SimdMathF64>::atan2_u35,
        f64::atan2,
    );
}

fn run_f64_hypot_u35_contract<S: Simd>() {
    check_binary_f64::<S>(
        "hypot_u35",
        contracts::HYPOT_U35_F64_MAX_ULP,
        <S::Vf64 as SimdMathF64>::hypot_u35,
        f64::hypot,
    );
}

fn run_f64_fmod_contract<S: Simd>() {
    check_binary_f64::<S>("fmod", 0, <S::Vf64 as SimdMathF64>::fmod, |x, y| x % y);
}

macro_rules! simd_math_backend_test {
    ($name:ident, $simd:ident, $runner:ident) => {
        crate::with_feature_flag!(
            $simd,
            paste::item! {
                #[test]
                fn [<$name _ $simd:lower>]() {
                    $runner::<$simd>();
                }
            }
        );
    };
}

macro_rules! simd_math_all_backends {
    ($name:ident, $runner:ident) => {
        simd_math_backend_test!($name, Scalar, $runner);
        simd_math_backend_test!($name, Avx512, $runner);
        simd_math_backend_test!($name, Avx2, $runner);
        simd_math_backend_test!($name, Sse2, $runner);
        simd_math_backend_test!($name, Sse41, $runner);
        simd_math_backend_test!($name, Neon, $runner);
        simd_math_backend_test!($name, Wasm, $runner);
    };
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

simd_math_all_backends!(f32_asin_u35_contract, run_f32_asin_u35_contract);
simd_math_all_backends!(f32_acos_u35_contract, run_f32_acos_u35_contract);
simd_math_all_backends!(f32_atan_u35_contract, run_f32_atan_u35_contract);
simd_math_all_backends!(f32_sinh_u35_contract, run_f32_sinh_u35_contract);
simd_math_all_backends!(f32_cosh_u35_contract, run_f32_cosh_u35_contract);
simd_math_all_backends!(f32_tanh_u35_contract, run_f32_tanh_u35_contract);
simd_math_all_backends!(f32_asinh_u35_contract, run_f32_asinh_u35_contract);
simd_math_all_backends!(f32_acosh_u35_contract, run_f32_acosh_u35_contract);
simd_math_all_backends!(f32_atanh_u35_contract, run_f32_atanh_u35_contract);
simd_math_all_backends!(f32_log10_u35_contract, run_f32_log10_u35_contract);
simd_math_all_backends!(f32_atan2_u35_contract, run_f32_atan2_u35_contract);
simd_math_all_backends!(f32_hypot_u35_contract, run_f32_hypot_u35_contract);
simd_math_all_backends!(f32_fmod_contract, run_f32_fmod_contract);

simd_math_all_backends!(f64_asin_u35_contract, run_f64_asin_u35_contract);
simd_math_all_backends!(f64_acos_u35_contract, run_f64_acos_u35_contract);
simd_math_all_backends!(f64_atan_u35_contract, run_f64_atan_u35_contract);
simd_math_all_backends!(f64_sinh_u35_contract, run_f64_sinh_u35_contract);
simd_math_all_backends!(f64_cosh_u35_contract, run_f64_cosh_u35_contract);
simd_math_all_backends!(f64_tanh_u35_contract, run_f64_tanh_u35_contract);
simd_math_all_backends!(f64_asinh_u35_contract, run_f64_asinh_u35_contract);
simd_math_all_backends!(f64_acosh_u35_contract, run_f64_acosh_u35_contract);
simd_math_all_backends!(f64_atanh_u35_contract, run_f64_atanh_u35_contract);
simd_math_all_backends!(f64_log10_u35_contract, run_f64_log10_u35_contract);
simd_math_all_backends!(f64_atan2_u35_contract, run_f64_atan2_u35_contract);
simd_math_all_backends!(f64_hypot_u35_contract, run_f64_hypot_u35_contract);
simd_math_all_backends!(f64_fmod_contract, run_f64_fmod_contract);
