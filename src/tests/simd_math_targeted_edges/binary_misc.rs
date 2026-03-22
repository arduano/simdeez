use super::*;
use crate::math::SimdMathF32BinaryMisc;

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

simd_math_targeted_all_backends!(
    f32_atan2_signed_zero_and_quadrants,
    run_f32_atan2_signed_zero_and_quadrants
);
simd_math_targeted_all_backends!(f32_hypot_and_fmod_edges, run_f32_hypot_and_fmod_edges);
