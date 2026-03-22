use super::*;
use crate::math::SimdMathF32InverseTrig;

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

fn run_f32_inverse_trig_special_lanes<S: Simd>() {
    let inputs = [
        f32::NAN,
        f32::INFINITY,
        f32::NEG_INFINITY,
        0.0,
        -0.0,
        1.0,
        -1.0,
        f32::from_bits(0x3F80_0001),
        -f32::from_bits(0x3F80_0001),
        0.5,
        -0.5,
        4.0,
        -4.0,
        0.414_213_57,
        0.414_213_63,
        2.414_213_4,
        2.414_214,
    ];

    for chunk in inputs.chunks(S::Vf32::WIDTH) {
        let v = S::Vf32::load_from_slice(chunk);
        let asin = v.asin_u35();
        let acos = v.acos_u35();
        let atan = v.atan_u35();

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
            assert_f32_contract(
                "atan_u35",
                x,
                atan[lane],
                x.atan(),
                contracts::ATAN_U35_F32_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("{e}"));
        }
    }
}

fn run_f32_inverse_trig_symmetry<S: Simd>() {
    let inputs = [-0.875, -0.75, -0.5, -0.125, 0.125, 0.5, 0.75, 0.875];

    for chunk in inputs.chunks(S::Vf32::WIDTH) {
        let v = S::Vf32::load_from_slice(chunk);
        let neg_v = -v;

        let asin = v.asin_u35();
        let asin_neg = neg_v.asin_u35();
        let atan = v.atan_u35();
        let atan_neg = neg_v.atan_u35();

        for lane in 0..chunk.len() {
            assert_f32_contract("asin symmetry", chunk[lane], asin_neg[lane], -asin[lane], 2)
                .unwrap_or_else(|e| panic!("{e}"));
            assert_f32_contract("atan symmetry", chunk[lane], atan_neg[lane], -atan[lane], 2)
                .unwrap_or_else(|e| panic!("{e}"));
        }
    }
}

fn run_f32_inverse_trig_identity<S: Simd>() {
    let inputs = [-1.0, -0.875, -0.5, -0.125, 0.0, 0.125, 0.5, 0.875, 1.0];

    for chunk in inputs.chunks(S::Vf32::WIDTH) {
        let v = S::Vf32::load_from_slice(chunk);
        let sum = v.asin_u35() + v.acos_u35();

        for (lane, &x) in chunk.iter().enumerate() {
            assert_f32_contract(
                "asin+acos identity",
                x,
                sum[lane],
                std::f32::consts::FRAC_PI_2,
                8,
            )
            .unwrap_or_else(|e| panic!("{e}"));
        }
    }
}

simd_math_targeted_all_backends!(f32_inverse_trig_near_one, run_f32_inverse_trig_near_one);
simd_math_targeted_all_backends!(
    f32_inverse_trig_special_lanes,
    run_f32_inverse_trig_special_lanes
);
simd_math_targeted_all_backends!(f32_inverse_trig_symmetry, run_f32_inverse_trig_symmetry);
simd_math_targeted_all_backends!(f32_inverse_trig_identity, run_f32_inverse_trig_identity);
