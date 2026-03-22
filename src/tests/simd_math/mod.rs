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
        ::core::f32::consts::PI,
        -::core::f32::consts::PI,
        ::core::f32::consts::E,
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
        ::core::f64::consts::PI,
        -::core::f64::consts::PI,
        ::core::f64::consts::E,
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

mod binary_misc;
mod core;
mod hyperbolic;
mod inverse_hyperbolic;
mod inverse_trig;
