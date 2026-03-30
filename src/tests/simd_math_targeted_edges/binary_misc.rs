use super::*;
use crate::math::{SimdMathF32BinaryMisc, SimdMathF64BinaryMisc};

fn run_f32_atan2_signed_zero_and_quadrants<S: Simd>() {
    let ys = [
        0.0f32,
        -0.0,
        1.0,
        -1.0,
        2.0,
        -2.0,
        1.0e-30,
        -1.0e-30,
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::NAN,
    ];
    let xs = [
        0.0f32,
        -0.0,
        1.0,
        -1.0,
        2.0,
        -2.0,
        1.0e-30,
        -1.0e-30,
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::NAN,
    ];
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
        3.4e38,
        -3.4e38,
    ];
    let ys = [
        1.0e38f32, 1.0e-38, 2.0, -2.0, -0.0, 0.0, 2.0, 1.5, 1.0, -1.0,
    ];
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
            .unwrap_or_else(|e| panic!("hypot_u35({x:?},{y:?}) {e}"));
            assert_f32_contract("fmod", x, r[lane], x % y, 0)
                .unwrap_or_else(|e| panic!("fmod({x:?},{y:?}) {e}"));
        }
    }
}

fn run_f32_log10_domain_and_mixed_lanes<S: Simd>() {
    let xs = [
        10.0f32,
        1.0,
        0.1,
        f32::MIN_POSITIVE,
        0.0,
        -0.0,
        -1.0,
        -10.0,
        f32::INFINITY,
        f32::NAN,
    ];

    for chunk in xs.chunks(S::Vf32::WIDTH) {
        let xv = S::Vf32::load_from_slice(chunk);
        let out = xv.log10_u35();
        for lane in 0..chunk.len() {
            let x = chunk[lane];
            assert_f32_contract(
                "log10_u35",
                x,
                out[lane],
                x.log10(),
                contracts::LOG10_U35_F32_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("log10_u35({x:?}) {e}"));
        }
    }
}

fn run_f32_binary_misc_adversarial_lanes<S: Simd>() {
    let log10_inputs = [
        f32::from_bits(1),
        f32::from_bits(2),
        f32::MIN_POSITIVE,
        9.999_999e-11,
        1.0e-10,
        10.0,
        1000.0,
        0.0,
        -0.0,
        -1.0,
        f32::INFINITY,
        f32::NAN,
    ];
    for chunk in log10_inputs.chunks(S::Vf32::WIDTH) {
        let xv = S::Vf32::load_from_slice(chunk);
        let out = xv.log10_u35();
        for lane in 0..chunk.len() {
            let x = chunk[lane];
            assert_f32_contract(
                "log10_u35",
                x,
                out[lane],
                x.log10(),
                contracts::LOG10_U35_F32_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("adversarial log10_u35({x:?}) {e}"));
        }
    }

    let atan2_cases = [
        (0.0f32, -0.0f32),
        (-0.0, -0.0),
        (0.0, 0.0),
        (-0.0, 0.0),
        (f32::from_bits(1), -f32::from_bits(1)),
        (-f32::from_bits(1), -f32::from_bits(1)),
        (1.0e-30, -1.0),
        (-1.0e-30, -1.0),
        (1.0, -1.0e-30),
        (-1.0, -1.0e-30),
        (f32::INFINITY, -f32::INFINITY),
        (f32::NEG_INFINITY, -f32::INFINITY),
        (f32::NAN, 1.0),
        (1.0, f32::NAN),
    ];
    for chunk in atan2_cases.chunks(S::Vf32::WIDTH) {
        let mut ys = vec![0.0f32; chunk.len()];
        let mut xs = vec![0.0f32; chunk.len()];
        for (idx, (y, x)) in chunk.iter().copied().enumerate() {
            ys[idx] = y;
            xs[idx] = x;
        }

        let yv = S::Vf32::load_from_slice(&ys);
        let xv = S::Vf32::load_from_slice(&xs);
        let out = yv.atan2_u35(xv);
        for lane in 0..chunk.len() {
            let y = ys[lane];
            let x = xs[lane];
            assert_f32_contract(
                "atan2_u35",
                y,
                out[lane],
                y.atan2(x),
                contracts::ATAN2_U35_F32_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("adversarial atan2_u35({y:?}, {x:?}) {e}"));
        }
    }

    let binary_cases = [
        (f32::MAX, f32::MIN_POSITIVE),
        (f32::MIN_POSITIVE, f32::MAX),
        (1.0e38, 1.0e-38),
        (1.0e-38, 1.0e38),
        (-1.0e38, 1.0e-38),
        (6.0, 3.0),
        (6.0, -3.0),
        (-6.0, 3.0),
        (-6.0, -3.0),
        (0.0, 3.0),
        (-0.0, 3.0),
        (f32::INFINITY, 2.0),
        (2.0, 0.0),
        (f32::NAN, 2.0),
        (2.0, f32::NAN),
    ];

    for chunk in binary_cases.chunks(S::Vf32::WIDTH) {
        let mut xs = vec![0.0f32; chunk.len()];
        let mut ys = vec![0.0f32; chunk.len()];
        for (idx, (x, y)) in chunk.iter().copied().enumerate() {
            xs[idx] = x;
            ys[idx] = y;
        }

        let xv = S::Vf32::load_from_slice(&xs);
        let yv = S::Vf32::load_from_slice(&ys);
        let h = xv.hypot_u35(yv);
        let r = xv.fmod(yv);
        for lane in 0..chunk.len() {
            let x = xs[lane];
            let y = ys[lane];
            assert_f32_contract(
                "hypot_u35",
                x,
                h[lane],
                x.hypot(y),
                contracts::HYPOT_U35_F32_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("adversarial hypot_u35({x:?}, {y:?}) {e}"));
            assert_f32_contract("fmod", x, r[lane], x % y, 0)
                .unwrap_or_else(|e| panic!("adversarial fmod({x:?}, {y:?}) {e}"));
        }
    }
}

simd_math_targeted_all_backends!(
    f32_atan2_signed_zero_and_quadrants,
    run_f32_atan2_signed_zero_and_quadrants
);
simd_math_targeted_all_backends!(f32_hypot_and_fmod_edges, run_f32_hypot_and_fmod_edges);
simd_math_targeted_all_backends!(
    f32_log10_domain_and_mixed_lanes,
    run_f32_log10_domain_and_mixed_lanes
);
simd_math_targeted_all_backends!(
    f32_binary_misc_adversarial_lanes,
    run_f32_binary_misc_adversarial_lanes
);

fn run_f64_atan2_signed_zero_and_quadrants<S: Simd>() {
    let ys = [
        0.0f64,
        -0.0,
        1.0,
        -1.0,
        2.0,
        -2.0,
        1.0e-300,
        -1.0e-300,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NAN,
    ];
    let xs = [
        0.0f64,
        -0.0,
        1.0,
        -1.0,
        2.0,
        -2.0,
        1.0e-300,
        -1.0e-300,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NAN,
    ];
    let mut y_inputs = Vec::new();
    let mut x_inputs = Vec::new();
    for &y in &ys {
        for &x in &xs {
            y_inputs.push(y);
            x_inputs.push(x);
        }
    }

    for (ys_chunk, xs_chunk) in y_inputs
        .chunks(S::Vf64::WIDTH)
        .zip(x_inputs.chunks(S::Vf64::WIDTH))
    {
        let yv = S::Vf64::load_from_slice(ys_chunk);
        let xv = S::Vf64::load_from_slice(xs_chunk);
        let out = yv.atan2_u35(xv);
        for lane in 0..ys_chunk.len() {
            let y = ys_chunk[lane];
            let x = xs_chunk[lane];
            assert_f64_contract(
                "atan2_u35",
                y,
                out[lane],
                y.atan2(x),
                contracts::ATAN2_U35_F64_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("atan2_u35({y:?}, {x:?}): {e}"));
        }
    }
}

fn run_f64_hypot_and_fmod_edges<S: Simd>() {
    let xs = [
        1.0e308f64,
        1.0e-308,
        3.0,
        -3.0,
        0.0,
        -0.0,
        f64::INFINITY,
        f64::NAN,
        f64::MAX,
        -f64::MAX,
        9.0e15,
        -9.0e15,
    ];
    let ys = [
        1.0e308f64, 1.0e-308, 2.0, -2.0, -0.0, 0.0, 2.0, 1.5, 1.0, -1.0, 3.0, -3.0,
    ];
    for (x_chunk, y_chunk) in xs.chunks(S::Vf64::WIDTH).zip(ys.chunks(S::Vf64::WIDTH)) {
        let xv = S::Vf64::load_from_slice(x_chunk);
        let yv = S::Vf64::load_from_slice(y_chunk);
        let h = xv.hypot_u35(yv);
        let r = xv.fmod(yv);
        for lane in 0..x_chunk.len() {
            let x = x_chunk[lane];
            let y = y_chunk[lane];
            assert_f64_contract(
                "hypot_u35",
                x,
                h[lane],
                x.hypot(y),
                contracts::HYPOT_U35_F64_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("hypot_u35({x:?},{y:?}) {e}"));
            assert_f64_contract("fmod", x, r[lane], x % y, 0)
                .unwrap_or_else(|e| panic!("fmod({x:?},{y:?}) {e}"));
        }
    }
}

fn run_f64_log10_domain_and_mixed_lanes<S: Simd>() {
    let xs = [
        10.0f64,
        1.0,
        0.1,
        f64::MIN_POSITIVE,
        0.0,
        -0.0,
        -1.0,
        -10.0,
        f64::INFINITY,
        f64::NAN,
    ];

    for chunk in xs.chunks(S::Vf64::WIDTH) {
        let xv = S::Vf64::load_from_slice(chunk);
        let out = xv.log10_u35();
        for lane in 0..chunk.len() {
            let x = chunk[lane];
            assert_f64_contract(
                "log10_u35",
                x,
                out[lane],
                x.log10(),
                contracts::LOG10_U35_F64_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("log10_u35({x:?}) {e}"));
        }
    }
}

simd_math_targeted_all_backends!(
    f64_atan2_signed_zero_and_quadrants,
    run_f64_atan2_signed_zero_and_quadrants
);
simd_math_targeted_all_backends!(f64_hypot_and_fmod_edges, run_f64_hypot_and_fmod_edges);
simd_math_targeted_all_backends!(
    f64_log10_domain_and_mixed_lanes,
    run_f64_log10_domain_and_mixed_lanes
);

fn run_f64_binary_misc_adversarial_lanes<S: Simd>() {
    let log10_inputs = [
        f64::MIN_POSITIVE,
        f64::from_bits(1),
        f64::from_bits(2),
        1.0e-320,
        9.999_999_999_999_999e-101,
        1.0e-100,
        10.0,
        1.0e100,
        0.0,
        -0.0,
        -1.0,
        f64::INFINITY,
        f64::NAN,
    ];
    for chunk in log10_inputs.chunks(S::Vf64::WIDTH) {
        let xv = S::Vf64::load_from_slice(chunk);
        let out = xv.log10_u35();
        for lane in 0..chunk.len() {
            let x = chunk[lane];
            assert_f64_contract(
                "log10_u35",
                x,
                out[lane],
                x.log10(),
                contracts::LOG10_U35_F64_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("adversarial log10_u35({x:?}) {e}"));
        }
    }

    let atan2_cases = [
        (0.0, -0.0),
        (-0.0, -0.0),
        (0.0, 0.0),
        (-0.0, 0.0),
        (f64::MIN_POSITIVE, -f64::MIN_POSITIVE),
        (-f64::MIN_POSITIVE, -f64::MIN_POSITIVE),
        (1.0e-300, -1.0),
        (-1.0e-300, -1.0),
        (1.0, -1.0e-300),
        (-1.0, -1.0e-300),
        (f64::INFINITY, -f64::INFINITY),
        (f64::NEG_INFINITY, -f64::INFINITY),
        (f64::NAN, 1.0),
        (1.0, f64::NAN),
    ];
    for chunk in atan2_cases.chunks(S::Vf64::WIDTH) {
        let mut ys = vec![0.0f64; chunk.len()];
        let mut xs = vec![0.0f64; chunk.len()];
        for (idx, (y, x)) in chunk.iter().copied().enumerate() {
            ys[idx] = y;
            xs[idx] = x;
        }

        let yv = S::Vf64::load_from_slice(&ys);
        let xv = S::Vf64::load_from_slice(&xs);
        let out = yv.atan2_u35(xv);
        for lane in 0..chunk.len() {
            let y = ys[lane];
            let x = xs[lane];
            assert_f64_contract(
                "atan2_u35",
                y,
                out[lane],
                y.atan2(x),
                contracts::ATAN2_U35_F64_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("adversarial atan2_u35({y:?}, {x:?}) {e}"));
        }
    }

    let binary_cases = [
        (f64::MAX, f64::MIN_POSITIVE),
        (f64::MIN_POSITIVE, f64::MAX),
        (1.0e308, 1.0e-308),
        (1.0e-308, 1.0e308),
        (-1.0e308, 1.0e-308),
        (9.007_199_254_740_992e15, 1.25),
        (-9.007_199_254_740_992e15, 1.25),
        (9.007_199_254_740_993e15, 1.25),
        (-9.007_199_254_740_993e15, 1.25),
        (0.0, 3.0),
        (-0.0, 3.0),
        (f64::INFINITY, 2.0),
        (2.0, 0.0),
        (f64::NAN, 2.0),
        (2.0, f64::NAN),
    ];

    for chunk in binary_cases.chunks(S::Vf64::WIDTH) {
        let mut xs = vec![0.0f64; chunk.len()];
        let mut ys = vec![0.0f64; chunk.len()];
        for (idx, (x, y)) in chunk.iter().copied().enumerate() {
            xs[idx] = x;
            ys[idx] = y;
        }

        let xv = S::Vf64::load_from_slice(&xs);
        let yv = S::Vf64::load_from_slice(&ys);
        let h = xv.hypot_u35(yv);
        let r = xv.fmod(yv);

        for lane in 0..chunk.len() {
            let x = xs[lane];
            let y = ys[lane];
            assert_f64_contract(
                "hypot_u35",
                x,
                h[lane],
                x.hypot(y),
                contracts::HYPOT_U35_F64_MAX_ULP,
            )
            .unwrap_or_else(|e| panic!("adversarial hypot_u35({x:?}, {y:?}) {e}"));
            assert_f64_contract("fmod", x, r[lane], x % y, 0)
                .unwrap_or_else(|e| panic!("adversarial fmod({x:?}, {y:?}) {e}"));
        }
    }
}

fn run_f64_fmod_zero_and_guard_boundaries<S: Simd>() {
    let exact_zero_cases = [
        (6.0f64, 3.0f64),
        (6.0, -3.0),
        (-6.0, 3.0),
        (-6.0, -3.0),
        (0.0, 3.0),
        (-0.0, 3.0),
    ];
    for chunk in exact_zero_cases.chunks(S::Vf64::WIDTH) {
        let mut xs = vec![0.0f64; chunk.len()];
        let mut ys = vec![0.0f64; chunk.len()];
        for (idx, (x, y)) in chunk.iter().copied().enumerate() {
            xs[idx] = x;
            ys[idx] = y;
        }

        let xv = S::Vf64::load_from_slice(&xs);
        let yv = S::Vf64::load_from_slice(&ys);
        let out = xv.fmod(yv);
        for lane in 0..chunk.len() {
            let x = xs[lane];
            let y = ys[lane];
            assert_f64_contract("fmod", x, out[lane], x % y, 0)
                .unwrap_or_else(|e| panic!("zero-sign fmod({x:?}, {y:?}) {e}"));
        }
    }

    let q = 4_503_599_627_370_496.0f64;
    let guard_cases = [
        ((q - 1.0) * 2.0, 2.0),
        (q * 2.0, 2.0),
        ((q + 1.0) * 2.0, 2.0),
        (-((q - 1.0) * 2.0), 2.0),
        (-(q * 2.0), 2.0),
        (-((q + 1.0) * 2.0), 2.0),
        ((q - 1.0) * -2.0, -2.0),
        ((q + 1.0) * -2.0, -2.0),
    ];
    for chunk in guard_cases.chunks(S::Vf64::WIDTH) {
        let mut xs = vec![0.0f64; chunk.len()];
        let mut ys = vec![0.0f64; chunk.len()];
        for (idx, (x, y)) in chunk.iter().copied().enumerate() {
            xs[idx] = x;
            ys[idx] = y;
        }

        let xv = S::Vf64::load_from_slice(&xs);
        let yv = S::Vf64::load_from_slice(&ys);
        let out = xv.fmod(yv);
        for lane in 0..chunk.len() {
            let x = xs[lane];
            let y = ys[lane];
            assert_f64_contract("fmod", x, out[lane], x % y, 0)
                .unwrap_or_else(|e| panic!("guard-boundary fmod({x:?}, {y:?}) {e}"));
        }
    }
}

simd_math_targeted_all_backends!(
    f64_binary_misc_adversarial_lanes,
    run_f64_binary_misc_adversarial_lanes
);
simd_math_targeted_all_backends!(
    f64_fmod_zero_and_guard_boundaries,
    run_f64_fmod_zero_and_guard_boundaries
);
