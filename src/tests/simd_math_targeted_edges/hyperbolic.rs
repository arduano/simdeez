use super::*;
use crate::math::{SimdMathF32Hyperbolic, SimdMathF32InverseHyperbolic};

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

simd_math_targeted_all_backends!(f32_hyperbolic_edges, run_f32_hyperbolic_edges);
