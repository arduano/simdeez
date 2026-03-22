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

simd_math_targeted_all_backends!(f32_inverse_trig_near_one, run_f32_inverse_trig_near_one);
