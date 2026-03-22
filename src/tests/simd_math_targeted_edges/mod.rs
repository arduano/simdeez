#![allow(unused_imports)]

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

use crate::math::SimdMathF32Core;
use crate::math::{contracts, SimdMathF32};
use crate::{Simd, SimdBaseIo, SimdConsts};

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

fn check_targeted_unary_f32<S: Simd>(
    fn_name: &str,
    inputs: &[f32],
    max_ulp: u32,
    simd_fn: impl Fn(S::Vf32) -> S::Vf32,
    scalar_ref: impl Fn(f32) -> f32,
) {
    for chunk in inputs.chunks(S::Vf32::WIDTH) {
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

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
fn run_log2_u35_vector_apply_avx2(input: &[f32], output: &mut [f32]) {
    assert_eq!(input.len(), output.len());

    Avx2::invoke(|| {
        let mut in_rest = input;
        let mut out_rest = output;

        while in_rest.len() >= <Avx2 as Simd>::Vf32::WIDTH {
            let v = <Avx2 as Simd>::Vf32::load_from_slice(in_rest);
            v.log2_u35().copy_to_slice(out_rest);

            in_rest = &in_rest[<Avx2 as Simd>::Vf32::WIDTH..];
            out_rest = &mut out_rest[<Avx2 as Simd>::Vf32::WIDTH..];
        }

        for (&x, out) in in_rest.iter().zip(out_rest.iter_mut()) {
            *out = x.log2();
        }
    });
}

macro_rules! simd_math_backend_targeted_test {
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

macro_rules! simd_math_targeted_all_backends {
    ($name:ident, $runner:ident) => {
        simd_math_backend_targeted_test!($name, Scalar, $runner);
        simd_math_backend_targeted_test!($name, Avx512, $runner);
        simd_math_backend_targeted_test!($name, Avx2, $runner);
        simd_math_backend_targeted_test!($name, Sse2, $runner);
        simd_math_backend_targeted_test!($name, Sse41, $runner);
        simd_math_backend_targeted_test!($name, Neon, $runner);
        simd_math_backend_targeted_test!($name, Wasm, $runner);
    };
}

mod binary_misc;
mod core;
mod hyperbolic;
mod inverse_hyperbolic;
mod inverse_trig;
