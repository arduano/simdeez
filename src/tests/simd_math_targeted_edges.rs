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

fn run_f32_log2_u35_reduction_boundaries<S: Simd>() {
    let mut inputs = vec![
        f32::from_bits(0x3EFFFFFE),
        f32::from_bits(0x3EFFFFFF),
        f32::from_bits(0x3F000000),
        f32::from_bits(0x3F000001),
        f32::from_bits(0x3F7FFFFF),
        f32::from_bits(0x3F800000),
        f32::from_bits(0x3F800001),
        f32::from_bits(0x3FFFFFFF),
        f32::from_bits(0x40000000),
        f32::from_bits(0x40000001),
    ];

    for &scale in &[0.5f32, 1.0, 2.0, 8.0] {
        let pivot = core::f32::consts::FRAC_1_SQRT_2 * scale;
        inputs.push(f32::from_bits(pivot.to_bits() - 1));
        inputs.push(pivot);
        inputs.push(f32::from_bits(pivot.to_bits() + 1));
    }

    check_targeted_unary_f32::<S>(
        "log2_u35",
        &inputs,
        contracts::LOG2_U35_F32_MAX_ULP,
        <S::Vf32 as SimdMathF32>::log2_u35,
        f32::log2,
    );
}

fn run_f32_exp2_u35_fast_domain_boundaries<S: Simd>() {
    let mut inputs = vec![
        -126.0001,
        -126.0,
        -125.9999,
        -1.0001,
        -1.0,
        -0.9999,
        -0.0001,
        -0.0,
        0.0,
        0.0001,
        0.9999,
        1.0,
        1.0001,
        125.9999,
        126.0,
        126.0001,
        f32::NEG_INFINITY,
        f32::INFINITY,
        f32::NAN,
    ];

    for k in -4..=4 {
        let center = k as f32;
        inputs.push(center - 1.0 / 1024.0);
        inputs.push(center);
        inputs.push(center + 1.0 / 1024.0);
    }

    check_targeted_unary_f32::<S>(
        "exp2_u35",
        &inputs,
        contracts::EXP2_U35_F32_MAX_ULP,
        <S::Vf32 as SimdMathF32>::exp2_u35,
        f32::exp2,
    );
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

simd_math_targeted_all_backends!(
    f32_log2_u35_reduction_boundaries,
    run_f32_log2_u35_reduction_boundaries
);
simd_math_targeted_all_backends!(
    f32_exp2_u35_fast_domain_boundaries,
    run_f32_exp2_u35_fast_domain_boundaries
);

fn run_f32_trig_pi_boundaries<S: Simd>() {
    let mut inputs = vec![
        0.0,
        -0.0,
        core::f32::consts::PI,
        -core::f32::consts::PI,
        core::f32::consts::FRAC_PI_2,
        -core::f32::consts::FRAC_PI_2,
        core::f32::consts::FRAC_PI_4,
        -core::f32::consts::FRAC_PI_4,
    ];

    for k in -8..=8 {
        let base = (k as f32) * core::f32::consts::FRAC_PI_2;
        inputs.push(f32::from_bits(base.to_bits().saturating_sub(1)));
        inputs.push(base);
        inputs.push(f32::from_bits(base.to_bits().saturating_add(1)));
    }

    check_targeted_unary_f32::<S>(
        "sin_u35",
        &inputs,
        contracts::SIN_U35_F32_MAX_ULP,
        <S::Vf32 as SimdMathF32>::sin_u35,
        f32::sin,
    );
    check_targeted_unary_f32::<S>(
        "cos_u35",
        &inputs,
        contracts::COS_U35_F32_MAX_ULP,
        <S::Vf32 as SimdMathF32>::cos_u35,
        f32::cos,
    );
}

fn run_f32_tan_pole_neighborhoods<S: Simd>() {
    let mut inputs = vec![
        -100.0,
        -10.0,
        -1.0,
        -0.0,
        0.0,
        1.0,
        10.0,
        100.0,
        f32::NAN,
        f32::INFINITY,
        f32::NEG_INFINITY,
    ];

    for k in -12..=12 {
        let pole = (k as f32 + 0.5) * core::f32::consts::PI;
        for delta in [1.0e-2, 1.0e-4, 1.0e-6] {
            inputs.push(pole - delta);
            inputs.push(pole + delta);
        }
    }

    check_targeted_unary_f32::<S>(
        "tan_u35",
        &inputs,
        contracts::TAN_U35_F32_MAX_ULP,
        <S::Vf32 as SimdMathF32>::tan_u35,
        f32::tan,
    );
}

fn run_f32_trig_large_and_mixed_lanes<S: Simd>() {
    let inputs = vec![
        0.25,
        -0.5,
        123.456,
        -2048.0,
        8192.0,
        -8192.0,
        16384.0,
        -16384.0,
        f32::from_bits(1),
        -f32::from_bits(1),
        f32::NAN,
        f32::INFINITY,
        f32::NEG_INFINITY,
        core::f32::consts::PI * 0.5 - 1.0e-4,
        core::f32::consts::PI * 0.5 + 1.0e-4,
        -core::f32::consts::PI * 0.5 + 1.0e-4,
    ];

    check_targeted_unary_f32::<S>(
        "sin_u35",
        &inputs,
        contracts::SIN_U35_F32_MAX_ULP,
        <S::Vf32 as SimdMathF32>::sin_u35,
        f32::sin,
    );
    check_targeted_unary_f32::<S>(
        "cos_u35",
        &inputs,
        contracts::COS_U35_F32_MAX_ULP,
        <S::Vf32 as SimdMathF32>::cos_u35,
        f32::cos,
    );
    check_targeted_unary_f32::<S>(
        "tan_u35",
        &inputs,
        contracts::TAN_U35_F32_MAX_ULP,
        <S::Vf32 as SimdMathF32>::tan_u35,
        f32::tan,
    );
}

simd_math_targeted_all_backends!(f32_trig_pi_boundaries, run_f32_trig_pi_boundaries);
simd_math_targeted_all_backends!(f32_tan_pole_neighborhoods, run_f32_tan_pole_neighborhoods);
simd_math_targeted_all_backends!(
    f32_trig_large_and_mixed_lanes,
    run_f32_trig_large_and_mixed_lanes
);

fn run_f32_trig_symmetry_identities<S: Simd>() {
    let inputs = [
        -3.0f32,
        -1.0,
        -0.5,
        -0.0,
        0.0,
        0.5,
        1.0,
        3.0,
        core::f32::consts::FRAC_PI_3,
        -core::f32::consts::FRAC_PI_3,
    ];

    for chunk in inputs.chunks(S::Vf32::WIDTH) {
        let x = S::Vf32::load_from_slice(chunk);
        let sx = x.sin_u35();
        let cx = x.cos_u35();
        let tx = x.tan_u35();

        let neg_x = -x;
        let sneg = neg_x.sin_u35();
        let cneg = neg_x.cos_u35();
        let tneg = neg_x.tan_u35();

        for lane in 0..chunk.len() {
            if chunk[lane] == 0.0 {
                continue;
            }

            let sin_expected = -sx[lane];
            let cos_expected = cx[lane];
            let tan_expected = -tx[lane];

            assert_f32_contract(
                "sin parity",
                chunk[lane],
                sneg[lane],
                sin_expected,
                contracts::SIN_U35_F32_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("{e}"));
            assert_f32_contract(
                "cos parity",
                chunk[lane],
                cneg[lane],
                cos_expected,
                contracts::COS_U35_F32_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("{e}"));
            assert_f32_contract(
                "tan parity",
                chunk[lane],
                tneg[lane],
                tan_expected,
                contracts::TAN_U35_F32_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("{e}"));
        }
    }
}

simd_math_targeted_all_backends!(
    f32_trig_symmetry_identities,
    run_f32_trig_symmetry_identities
);

fn run_f32_inverse_trig_near_one<S: Simd>() {
    let inputs = [
        f32::from_bits(0x3F7F_FFFE),
        f32::from_bits(0x3F7F_FFFF),
        1.0,
        f32::from_bits(0x3F80_0001),
        -f32::from_bits(0x3F7F_FFFE),
        -f32::from_bits(0x3F7F_FFFF),
        -1.0,
        -f32::from_bits(0x3F80_0001),
    ];
    for chunk in inputs.chunks(S::Vf32::WIDTH) {
        let v = S::Vf32::load_from_slice(chunk);
        let asin = v.asin_u35();
        let acos = v.acos_u35();
        for (lane, &x) in chunk.iter().enumerate() {
            assert_f32_contract(
                "asin_u35",
                x,
                asin[lane],
                x.asin(),
                contracts::ASIN_U35_F32_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("{e}"));
            assert_f32_contract(
                "acos_u35",
                x,
                acos[lane],
                x.acos(),
                contracts::ACOS_U35_F32_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("{e}"));
        }
    }
}

fn run_f32_atan2_signed_zero_and_quadrants<S: Simd>() {
    let ys = [0.0f32, -0.0, 1.0, -1.0, 2.0, -2.0, 1.0e-30, -1.0e-30];
    let xs = [0.0f32, -0.0, 1.0, -1.0, 2.0, -2.0, 1.0e-30, -1.0e-30];
    let mut y_inputs = Vec::new();
    let mut x_inputs = Vec::new();
    for &y in &ys {
        for &x in &xs {
            y_inputs.push(y);
            x_inputs.push(x);
        }
    }

    for (ys_chunk, xs_chunk) in y_inputs
        .chunks(S::Vf32::WIDTH)
        .zip(x_inputs.chunks(S::Vf32::WIDTH))
    {
        let yv = S::Vf32::load_from_slice(ys_chunk);
        let xv = S::Vf32::load_from_slice(xs_chunk);
        let out = yv.atan2_u35(xv);
        for lane in 0..ys_chunk.len() {
            let y = ys_chunk[lane];
            let x = xs_chunk[lane];
            assert_f32_contract(
                "atan2_u35",
                y,
                out[lane],
                y.atan2(x),
                contracts::ATAN2_U35_F32_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("atan2_u35({y:?}, {x:?}): {e}"));
        }
    }
}

fn run_f32_hyperbolic_edges<S: Simd>() {
    let inputs = [
        -100.0f32, -20.0, -10.0, -1.0, -0.0, 0.0, 1.0, 10.0, 20.0, 100.0,
    ];
    for chunk in inputs.chunks(S::Vf32::WIDTH) {
        let v = S::Vf32::load_from_slice(chunk);
        let tanh = v.tanh_u35();
        let acosh = v.acosh_u35();
        let atanh = v.atanh_u35();
        for (lane, &x) in chunk.iter().enumerate() {
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

fn run_f32_hypot_and_fmod_edges<S: Simd>() {
    let xs = [
        1.0e38f32,
        1.0e-38,
        3.0,
        -3.0,
        0.0,
        -0.0,
        f32::INFINITY,
        f32::NAN,
    ];
    let ys = [1.0e38f32, 1.0e-38, 2.0, -2.0, -0.0, 0.0, 2.0, 1.5];
    for (x_chunk, y_chunk) in xs.chunks(S::Vf32::WIDTH).zip(ys.chunks(S::Vf32::WIDTH)) {
        let xv = S::Vf32::load_from_slice(x_chunk);
        let yv = S::Vf32::load_from_slice(y_chunk);
        let h = xv.hypot_u35(yv);
        let r = xv.fmod(yv);
        for lane in 0..x_chunk.len() {
            let x = x_chunk[lane];
            let y = y_chunk[lane];
            assert_f32_contract(
                "hypot_u35",
                x,
                h[lane],
                x.hypot(y),
                contracts::HYPOT_U35_F32_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("{e}"));
            assert_f32_contract("fmod", x, r[lane], x % y, 0)
                .unwrap_or_else(|e| panic!("fmod({x:?},{y:?}) {e}"));
        }
    }
}

simd_math_targeted_all_backends!(f32_inverse_trig_near_one, run_f32_inverse_trig_near_one);
simd_math_targeted_all_backends!(
    f32_atan2_signed_zero_and_quadrants,
    run_f32_atan2_signed_zero_and_quadrants
);
simd_math_targeted_all_backends!(f32_hyperbolic_edges, run_f32_hyperbolic_edges);
simd_math_targeted_all_backends!(f32_hypot_and_fmod_edges, run_f32_hypot_and_fmod_edges);

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[test]
fn f32_log2_u35_mixed_exception_lanes_avx2() {
    let has_avx2 = std::is_x86_feature_detected!("avx2");
    let has_fma = std::is_x86_feature_detected!("fma");
    if !(has_avx2 && has_fma) {
        eprintln!("[test] skipped avx2/fma mixed-lane log2_u35 test: CPU lacks avx2/fma");
        return;
    }

    let input = vec![
        1.0,
        2.0,
        -1.0,
        0.0,
        -0.0,
        f32::from_bits(1),
        f32::INFINITY,
        f32::NAN,
        0.75,
        1.5,
        3.0,
        64.0,
        1024.0,
        0.25,
        f32::from_bits(0x7FC0_1234),
        f32::from_bits(0x0000_0100),
    ];

    let mut output = vec![0.0f32; input.len()];
    run_log2_u35_vector_apply_avx2(&input, &mut output);

    for (&x, &actual) in input.iter().zip(output.iter()) {
        let expected = x.log2();
        if let Err(err) = assert_f32_contract(
            "log2_u35",
            x,
            actual,
            expected,
            contracts::LOG2_U35_F32_MAX_ULP,
        ) {
            panic!("{err}");
        }
    }
}
