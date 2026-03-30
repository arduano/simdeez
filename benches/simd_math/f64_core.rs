use crate::shared::{self, BenchTargetsF64, INPUT_LEN};
use criterion::Criterion;
use simdeez::math::SimdMathF64Core;
use simdeez::scalar::Scalar;
use simdeez::{prelude::*, simd_unsafe_generate_all};

#[inline(never)]
fn scalar_log2_sum(input: &[f64]) -> f64 {
    input.iter().copied().map(f64::log2).sum()
}
#[inline(never)]
fn scalar_exp2_sum(input: &[f64]) -> f64 {
    input.iter().copied().map(f64::exp2).sum()
}
#[inline(never)]
fn scalar_ln_sum(input: &[f64]) -> f64 {
    input.iter().copied().map(f64::ln).sum()
}
#[inline(never)]
fn scalar_exp_sum(input: &[f64]) -> f64 {
    input.iter().copied().map(f64::exp).sum()
}
#[inline(never)]
fn scalar_sin_sum(input: &[f64]) -> f64 {
    input.iter().copied().map(f64::sin).sum()
}
#[inline(never)]
fn scalar_cos_sum(input: &[f64]) -> f64 {
    input.iter().copied().map(f64::cos).sum()
}
#[inline(never)]
fn scalar_tan_sum(input: &[f64]) -> f64 {
    input.iter().copied().map(f64::tan).sum()
}

simd_unsafe_generate_all!(
    fn simdeez_log2_sum(input: &[f64]) -> f64 {
        shared::simdeez_sum_impl_f64::<S>(input, |v| v.log2_u35())
    }
);
simd_unsafe_generate_all!(
    fn simdeez_exp2_sum(input: &[f64]) -> f64 {
        shared::simdeez_sum_impl_f64::<S>(input, |v| v.exp2_u35())
    }
);
simd_unsafe_generate_all!(
    fn simdeez_ln_sum(input: &[f64]) -> f64 {
        shared::simdeez_sum_impl_f64::<S>(input, |v| v.ln_u35())
    }
);
simd_unsafe_generate_all!(
    fn simdeez_exp_sum(input: &[f64]) -> f64 {
        shared::simdeez_sum_impl_f64::<S>(input, |v| v.exp_u35())
    }
);
simd_unsafe_generate_all!(
    fn simdeez_sin_sum(input: &[f64]) -> f64 {
        shared::simdeez_sum_impl_f64::<S>(input, |v| v.sin_u35())
    }
);
simd_unsafe_generate_all!(
    fn simdeez_cos_sum(input: &[f64]) -> f64 {
        shared::simdeez_sum_impl_f64::<S>(input, |v| v.cos_u35())
    }
);
simd_unsafe_generate_all!(
    fn simdeez_tan_sum(input: &[f64]) -> f64 {
        shared::simdeez_sum_impl_f64::<S>(input, |v| v.tan_u35())
    }
);

#[inline(never)]
fn forced_scalar_log2_sum(input: &[f64]) -> f64 {
    shared::force_scalar_sum_f64(input, |v: <Scalar as Simd>::Vf64| v.log2_u35())
}

#[inline(never)]
fn forced_scalar_exp2_sum(input: &[f64]) -> f64 {
    shared::force_scalar_sum_f64(input, |v: <Scalar as Simd>::Vf64| v.exp2_u35())
}

#[inline(never)]
fn forced_scalar_ln_sum(input: &[f64]) -> f64 {
    shared::force_scalar_sum_f64(input, |v: <Scalar as Simd>::Vf64| v.ln_u35())
}

#[inline(never)]
fn forced_scalar_exp_sum(input: &[f64]) -> f64 {
    shared::force_scalar_sum_f64(input, |v: <Scalar as Simd>::Vf64| v.exp_u35())
}

#[inline(never)]
fn forced_scalar_sin_sum(input: &[f64]) -> f64 {
    shared::force_scalar_sum_f64(input, |v: <Scalar as Simd>::Vf64| v.sin_u35())
}

#[inline(never)]
fn forced_scalar_cos_sum(input: &[f64]) -> f64 {
    shared::force_scalar_sum_f64(input, |v: <Scalar as Simd>::Vf64| v.cos_u35())
}

#[inline(never)]
fn forced_scalar_tan_sum(input: &[f64]) -> f64 {
    shared::force_scalar_sum_f64(input, |v: <Scalar as Simd>::Vf64| v.tan_u35())
}

pub fn register(c: &mut Criterion) {
    let log_inputs = shared::make_positive_log_inputs_f64(INPUT_LEN, 0xF640_1001);
    let exp2_inputs = shared::make_exp2_inputs_f64(INPUT_LEN, 0xF640_1002);
    let exp_inputs = shared::make_exp_inputs_f64(INPUT_LEN, 0xF640_1003);
    let trig_inputs = shared::make_trig_inputs_f64(INPUT_LEN, 0xF640_1004);
    let tan_inputs = shared::make_tan_inputs_f64(INPUT_LEN, 0xF640_1005);

    shared::bench_variants_f64(
        c,
        "simd_math/f64/log2_u35",
        &log_inputs,
        BenchTargetsF64 {
            scalar_native: scalar_log2_sum,
            simdeez_runtime: simdeez_log2_sum,
            simdeez_scalar: forced_scalar_log2_sum,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse2: simdeez_log2_sum_sse2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse41: simdeez_log2_sum_sse41,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx2: simdeez_log2_sum_avx2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx512: simdeez_log2_sum_avx512,
        },
    );
    shared::bench_variants_f64(
        c,
        "simd_math/f64/exp2_u35",
        &exp2_inputs,
        BenchTargetsF64 {
            scalar_native: scalar_exp2_sum,
            simdeez_runtime: simdeez_exp2_sum,
            simdeez_scalar: forced_scalar_exp2_sum,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse2: simdeez_exp2_sum_sse2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse41: simdeez_exp2_sum_sse41,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx2: simdeez_exp2_sum_avx2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx512: simdeez_exp2_sum_avx512,
        },
    );
    shared::bench_variants_f64(
        c,
        "simd_math/f64/ln_u35",
        &log_inputs,
        BenchTargetsF64 {
            scalar_native: scalar_ln_sum,
            simdeez_runtime: simdeez_ln_sum,
            simdeez_scalar: forced_scalar_ln_sum,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse2: simdeez_ln_sum_sse2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse41: simdeez_ln_sum_sse41,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx2: simdeez_ln_sum_avx2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx512: simdeez_ln_sum_avx512,
        },
    );
    shared::bench_variants_f64(
        c,
        "simd_math/f64/exp_u35",
        &exp_inputs,
        BenchTargetsF64 {
            scalar_native: scalar_exp_sum,
            simdeez_runtime: simdeez_exp_sum,
            simdeez_scalar: forced_scalar_exp_sum,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse2: simdeez_exp_sum_sse2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse41: simdeez_exp_sum_sse41,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx2: simdeez_exp_sum_avx2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx512: simdeez_exp_sum_avx512,
        },
    );
    shared::bench_variants_f64(
        c,
        "simd_math/f64/sin_u35",
        &trig_inputs,
        BenchTargetsF64 {
            scalar_native: scalar_sin_sum,
            simdeez_runtime: simdeez_sin_sum,
            simdeez_scalar: forced_scalar_sin_sum,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse2: simdeez_sin_sum_sse2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse41: simdeez_sin_sum_sse41,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx2: simdeez_sin_sum_avx2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx512: simdeez_sin_sum_avx512,
        },
    );
    shared::bench_variants_f64(
        c,
        "simd_math/f64/cos_u35",
        &trig_inputs,
        BenchTargetsF64 {
            scalar_native: scalar_cos_sum,
            simdeez_runtime: simdeez_cos_sum,
            simdeez_scalar: forced_scalar_cos_sum,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse2: simdeez_cos_sum_sse2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse41: simdeez_cos_sum_sse41,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx2: simdeez_cos_sum_avx2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx512: simdeez_cos_sum_avx512,
        },
    );
    shared::bench_variants_f64(
        c,
        "simd_math/f64/tan_u35",
        &tan_inputs,
        BenchTargetsF64 {
            scalar_native: scalar_tan_sum,
            simdeez_runtime: simdeez_tan_sum,
            simdeez_scalar: forced_scalar_tan_sum,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse2: simdeez_tan_sum_sse2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_sse41: simdeez_tan_sum_sse41,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx2: simdeez_tan_sum_avx2,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            simdeez_avx512: simdeez_tan_sum_avx512,
        },
    );
}
