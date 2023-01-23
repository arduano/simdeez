/// QuickCheck tests
///
/// TODO: test shifts
#[cfg(test)]
mod tests {
    use core::ops::*;

    use quickcheck::{quickcheck, TestResult};

    use simdeez::avx2::*;
    use simdeez::scalar::*;
    use simdeez::sse2::*;
    use simdeez::sse41::*;

    trait SpecialEq {
        fn equivalent(a: Self, b: Self) -> bool;
    }

    impl SpecialEq for f64 {
        fn equivalent(a: Self, b: Self) -> bool {
            if (a.is_nan() && b.is_nan()) || (a.is_infinite() && b.is_infinite()) {
                a.to_bits() == b.to_bits()
            } else {
                (a - b).abs() < 0.01
            }
        }
    }
    impl SpecialEq for f32 {
        fn equivalent(a: Self, b: Self) -> bool {
            if (a.is_nan() && b.is_nan()) || (a.is_infinite() && b.is_infinite()) {
                a.to_bits() == b.to_bits()
            } else {
                (a - b).abs() < 0.01
            }
        }
    }
    impl SpecialEq for i32 {
        fn equivalent(a: Self, b: Self) -> bool {
            a == b
        }
    }
    impl SpecialEq for i64 {
        fn equivalent(a: Self, b: Self) -> bool {
            a == b
        }
    }

    // Double argument operations

    macro_rules! gen_quickcheck_2_simd {
        ($fn_name:ident, $operation_scalar:expr, $operation_simd:expr, $ty:ty, $width:ident, $set_fn:ident) => {
            simd_runtime_generate!(
                fn $fn_name(info: &str, a: $ty, b: $ty) -> bool {
                    let a_simd = S::$set_fn(a);
                    let b_simd = S::$set_fn(b);

                    let result_simd = $operation_simd(a_simd, b_simd);
                    let result_scalar = $operation_scalar(a, b);

                    for i in 0..S::$width {
                        if !SpecialEq::equivalent(result_simd[i], result_scalar) {
                            println!(
                                "Results didn't match ({}). Reference: {:?}; Simdeez: {:?}",
                                info, result_scalar, result_simd
                            );

                            return false;
                        }
                    }

                    true
                }
            );

            paste::item! {
                #[test]
                fn [<test_ $fn_name>](){
                    fn prop(data: ($ty, $ty)) -> TestResult {
                        let a = data.0;
                        let b = data.1;

                        let mut ok = true;

                        unsafe {
                            ok &= [<$fn_name _scalar>]("scalar", a, b);

                            if is_x86_feature_detected!("sse2"){
                                ok &= [<$fn_name _sse2>]("sse2", a, b);
                            }
                            if is_x86_feature_detected!("sse4.1"){
                                ok &= [<$fn_name _sse41>]("sse41", a, b);
                            }
                            if is_x86_feature_detected!("avx2"){
                                ok &= [<$fn_name _avx2>]("avx2", a, b);
                            }
                        }

                        TestResult::from_bool(ok)
                    }

                    quickcheck(prop as fn(($ty, $ty)) -> TestResult);
                }
            }
        };
    }

    // Overloads

    gen_quickcheck_2_simd!(add_f32, Add::add, Add::add, f32, VF32_WIDTH, set1_ps);
    gen_quickcheck_2_simd!(div_f32, Div::div, Div::div, f32, VF32_WIDTH, set1_ps);
    gen_quickcheck_2_simd!(mul_f32, Mul::mul, Mul::mul, f32, VF32_WIDTH, set1_ps);
    gen_quickcheck_2_simd!(sub_f32, Sub::sub, Sub::sub, f32, VF32_WIDTH, set1_ps);

    gen_quickcheck_2_simd!(add_f64, Add::add, Add::add, f64, VF64_WIDTH, set1_pd);
    gen_quickcheck_2_simd!(div_f64, Div::div, Div::div, f64, VF64_WIDTH, set1_pd);
    gen_quickcheck_2_simd!(mul_f64, Mul::mul, Mul::mul, f64, VF64_WIDTH, set1_pd);
    gen_quickcheck_2_simd!(sub_f64, Sub::sub, Sub::sub, f64, VF64_WIDTH, set1_pd);

    gen_quickcheck_2_simd!(add_i32, Add::add, Add::add, i32, VI32_WIDTH, set1_epi32);
    gen_quickcheck_2_simd!(mul_i32, Mul::mul, Mul::mul, i32, VI32_WIDTH, set1_epi32);
    gen_quickcheck_2_simd!(sub_i32, Sub::sub, Sub::sub, i32, VI32_WIDTH, set1_epi32);

    gen_quickcheck_2_simd!(
        bitand_i32,
        BitAnd::bitand,
        BitAnd::bitand,
        i32,
        VI32_WIDTH,
        set1_epi32
    );
    gen_quickcheck_2_simd!(
        bitor_i32,
        BitOr::bitor,
        BitOr::bitor,
        i32,
        VI32_WIDTH,
        set1_epi32
    );
    gen_quickcheck_2_simd!(
        bitxor_i32,
        BitXor::bitxor,
        BitXor::bitxor,
        i32,
        VI32_WIDTH,
        set1_epi32
    );

    // Equality/ordering/min/max

    gen_quickcheck_2_simd!(
        cmpeq_f32,
        (|a, b| f32::from_bits(if a == b { !0u32 } else { 0u32 })),
        S::cmpeq_ps,
        f32,
        VF32_WIDTH,
        set1_ps
    );
    gen_quickcheck_2_simd!(
        cmpneq_f32,
        (|a, b| f32::from_bits(if a != b { !0u32 } else { 0u32 })),
        S::cmpneq_ps,
        f32,
        VF32_WIDTH,
        set1_ps
    );
    gen_quickcheck_2_simd!(
        cmpge_f32,
        (|a, b| f32::from_bits(if a >= b { !0u32 } else { 0u32 })),
        S::cmpge_ps,
        f32,
        VF32_WIDTH,
        set1_ps
    );
    gen_quickcheck_2_simd!(
        cmpgt_f32,
        (|a, b| f32::from_bits(if a > b { !0u32 } else { 0u32 })),
        S::cmpgt_ps,
        f32,
        VF32_WIDTH,
        set1_ps
    );
    gen_quickcheck_2_simd!(
        cmple_f32,
        (|a, b| f32::from_bits(if a <= b { !0u32 } else { 0u32 })),
        S::cmple_ps,
        f32,
        VF32_WIDTH,
        set1_ps
    );
    gen_quickcheck_2_simd!(
        cmplt_f32,
        (|a, b| f32::from_bits(if a < b { !0u32 } else { 0u32 })),
        S::cmplt_ps,
        f32,
        VF32_WIDTH,
        set1_ps
    );

    gen_quickcheck_2_simd!(
        cmpeq_f64,
        (|a, b| f64::from_bits(if a == b { !0u64 } else { 0u64 })),
        S::cmpeq_pd,
        f64,
        VF64_WIDTH,
        set1_pd
    );
    gen_quickcheck_2_simd!(
        cmpneq_f64,
        (|a, b| f64::from_bits(if a != b { !0u64 } else { 0u64 })),
        S::cmpneq_pd,
        f64,
        VF64_WIDTH,
        set1_pd
    );
    gen_quickcheck_2_simd!(
        cmpge_f64,
        (|a, b| f64::from_bits(if a >= b { !0u64 } else { 0u64 })),
        S::cmpge_pd,
        f64,
        VF64_WIDTH,
        set1_pd
    );
    gen_quickcheck_2_simd!(
        cmpgt_f64,
        (|a, b| f64::from_bits(if a > b { !0u64 } else { 0u64 })),
        S::cmpgt_pd,
        f64,
        VF64_WIDTH,
        set1_pd
    );
    gen_quickcheck_2_simd!(
        cmple_f64,
        (|a, b| f64::from_bits(if a <= b { !0u64 } else { 0u64 })),
        S::cmple_pd,
        f64,
        VF64_WIDTH,
        set1_pd
    );
    gen_quickcheck_2_simd!(
        cmplt_f64,
        (|a, b| f64::from_bits(if a < b { !0u64 } else { 0u64 })),
        S::cmplt_pd,
        f64,
        VF64_WIDTH,
        set1_pd
    );

    gen_quickcheck_2_simd!(
        cmpeq_i32,
        (|a, b| if a == b { !0i32 } else { 0i32 }),
        S::cmpeq_epi32,
        i32,
        VI32_WIDTH,
        set1_epi32
    );
    gen_quickcheck_2_simd!(
        cmpneq_i32,
        (|a, b| if a != b { !0i32 } else { 0i32 }),
        S::cmpneq_epi32,
        i32,
        VI32_WIDTH,
        set1_epi32
    );
    gen_quickcheck_2_simd!(
        cmpge_i32,
        (|a, b| if a >= b { !0i32 } else { 0i32 }),
        S::cmpge_epi32,
        i32,
        VI32_WIDTH,
        set1_epi32
    );
    gen_quickcheck_2_simd!(
        cmpgt_i32,
        (|a, b| if a > b { !0i32 } else { 0i32 }),
        S::cmpgt_epi32,
        i32,
        VI32_WIDTH,
        set1_epi32
    );
    gen_quickcheck_2_simd!(
        cmple_i32,
        (|a, b| if a <= b { !0i32 } else { 0i32 }),
        S::cmple_epi32,
        i32,
        VI32_WIDTH,
        set1_epi32
    );
    gen_quickcheck_2_simd!(
        cmplt_i32,
        (|a, b| if a < b { !0i32 } else { 0i32 }),
        S::cmplt_epi32,
        i32,
        VI32_WIDTH,
        set1_epi32
    );

    gen_quickcheck_2_simd!(min_f32, f32::min, S::min_ps, f32, VF32_WIDTH, set1_ps);
    gen_quickcheck_2_simd!(max_f32, f32::max, S::max_ps, f32, VF32_WIDTH, set1_ps);

    gen_quickcheck_2_simd!(min_f64, f64::min, S::min_pd, f64, VF64_WIDTH, set1_pd);
    gen_quickcheck_2_simd!(max_f64, f64::max, S::max_pd, f64, VF64_WIDTH, set1_pd);

    gen_quickcheck_2_simd!(min_i32, Ord::min, S::min_epi32, i32, VI32_WIDTH, set1_epi32);
    gen_quickcheck_2_simd!(max_i32, Ord::max, S::max_epi32, i32, VI32_WIDTH, set1_epi32);

    // Bitwise

    gen_quickcheck_2_simd!(
        andnot_i32,
        (|a: i32, b: i32| !a & b),
        S::andnot_epi32,
        i32,
        VI32_WIDTH,
        set1_epi32
    );

    gen_quickcheck_2_simd!(
        andnot_f32,
        (|a: f32, b: f32| f32::from_bits((!a.to_bits()) & b.to_bits())),
        S::andnot_ps,
        f32,
        VF32_WIDTH,
        set1_ps
    );
    gen_quickcheck_2_simd!(
        andnot_f64,
        (|a: f64, b: f64| f64::from_bits((!a.to_bits()) & b.to_bits())),
        S::andnot_pd,
        f64,
        VF64_WIDTH,
        set1_pd
    );

    // Triple argument operations (e.g., fused multiply-add)

    macro_rules! gen_quickcheck_3_simd {
        ($fn_name:ident, $operation_scalar:expr, $operation_simd:expr, $ty:ty, $width:ident, $set_fn:ident) => {
            simd_runtime_generate!(
                fn $fn_name(info: &str, a: $ty, b: $ty, c: $ty) -> bool {
                    let a_simd = S::$set_fn(a);
                    let b_simd = S::$set_fn(b);
                    let c_simd = S::$set_fn(b);

                    let result_simd = $operation_simd(a_simd, b_simd, c_simd);
                    let result_scalar = $operation_scalar(a, b, c);

                    for i in 0..S::$width {
                        if !SpecialEq::equivalent(result_simd[i], result_scalar) {
                            println!(
                                "Results didn't match ({}). Reference: {:?}; Simdeez: {:?}",
                                info, result_scalar, result_simd
                            );

                            return false;
                        }
                    }

                    true
                }
            );

            paste::item! {
                #[test]
                fn [<test_ $fn_name>](){
                    fn prop(data: ($ty, $ty, $ty)) -> TestResult {
                        let a = data.0;
                        let b = data.1;
                        let c = data.1;

                        let mut ok = true;

                        unsafe {
                            ok &= [<$fn_name _scalar>]("scalar", a, b, c);

                            if is_x86_feature_detected!("sse2"){
                                ok &= [<$fn_name _sse2>]("sse2", a, b, c);
                            }
                            if is_x86_feature_detected!("sse4.1"){
                                ok &= [<$fn_name _sse41>]("sse41", a, b, c);
                            }
                            if is_x86_feature_detected!("avx2"){
                                ok &= [<$fn_name _avx2>]("avx2", a, b, c);
                            }
                        }

                        TestResult::from_bool(ok)
                    }

                    quickcheck(prop as fn(($ty, $ty, $ty)) -> TestResult);
                }
            }
        };
    }

    gen_quickcheck_3_simd!(
        fmadd_f32,
        (|a, b, c| a * b + c),
        S::fmadd_ps,
        f32,
        VF32_WIDTH,
        set1_ps
    );
    gen_quickcheck_3_simd!(
        fmadd_f64,
        (|a, b, c| a * b + c),
        S::fmadd_pd,
        f64,
        VF64_WIDTH,
        set1_pd
    );

    gen_quickcheck_3_simd!(
        fnmadd_f32,
        (|a, b, c| -1.0f32 * a * b + c),
        S::fnmadd_ps,
        f32,
        VF32_WIDTH,
        set1_ps
    );
    gen_quickcheck_3_simd!(
        fnmadd_f64,
        (|a, b, c| -1.0f64 * a * b + c),
        S::fnmadd_pd,
        f64,
        VF64_WIDTH,
        set1_pd
    );

    gen_quickcheck_3_simd!(
        fmsub_f32,
        (|a, b, c| a * b - c),
        S::fmsub_ps,
        f32,
        VF32_WIDTH,
        set1_ps
    );
    gen_quickcheck_3_simd!(
        fmsub_f64,
        (|a, b, c| a * b - c),
        S::fmsub_pd,
        f64,
        VF64_WIDTH,
        set1_pd
    );

    gen_quickcheck_3_simd!(
        fnmsub_f32,
        (|a, b, c| -1.0f32 * a * b - c),
        S::fnmsub_ps,
        f32,
        VF32_WIDTH,
        set1_ps
    );
    gen_quickcheck_3_simd!(
        fnmsub_f64,
        (|a, b, c| -1.0f64 * a * b - c),
        S::fnmsub_pd,
        f64,
        VF64_WIDTH,
        set1_pd
    );

    // Single argument operations

    macro_rules! gen_quickcheck_1_simd {
        ($fn_name:ident, $operation_scalar:expr, $operation_simd:expr, $ty:ty, $width:ident, $set_fn:ident, $discard_value_lambda:expr) => {
            simd_runtime_generate!(
                fn $fn_name(info: &str, a: $ty) -> bool {
                    let result_simd = $operation_simd(S::$set_fn(a));
                    let result_scalar = $operation_scalar(a);

                    for i in 0..S::$width {
                        if !SpecialEq::equivalent(result_simd[i], result_scalar) {
                            println!(
                                "Results didn't match ({}). Reference: {:?}; Simdeez: {:?}",
                                info, result_scalar, result_simd
                            );

                            return false;
                        }
                    }

                    true
                }
            );

            paste::item! {
                #[test]
                fn [<test_ $fn_name>](){
                    fn prop(a: $ty) -> TestResult {
                        if ($discard_value_lambda)(a){
                            return TestResult::discard();
                        }

                        let mut ok = true;

                        unsafe {
                            ok &= [<$fn_name _scalar>]("scalar", a);

                            if is_x86_feature_detected!("sse2"){
                                ok &= [<$fn_name _sse2>]("sse2", a);
                            }
                            if is_x86_feature_detected!("sse4.1"){
                                ok &= [<$fn_name _sse41>]("sse41", a);
                            }
                            if is_x86_feature_detected!("avx2"){
                                ok &= [<$fn_name _avx2>]("avx2", a);
                            }
                        }

                        TestResult::from_bool(ok)
                    }

                    quickcheck(prop as fn($ty) -> TestResult);
                }
            }
        };
    }

    gen_quickcheck_1_simd!(
        abs_f32,
        f32::abs,
        S::abs_ps,
        f32,
        VF32_WIDTH,
        set1_ps,
        |_| false
    );
    gen_quickcheck_1_simd!(
        abs_f64,
        f64::abs,
        S::abs_pd,
        f64,
        VF64_WIDTH,
        set1_pd,
        |_| false
    );

    gen_quickcheck_1_simd!(
        sqrt_f32,
        f32::sqrt,
        S::sqrt_ps,
        f32,
        VF32_WIDTH,
        set1_ps,
        |a: f32| a.is_sign_negative()
    );
    gen_quickcheck_1_simd!(
        sqrt_f64,
        f64::sqrt,
        S::sqrt_pd,
        f64,
        VF64_WIDTH,
        set1_pd,
        |a: f64| a.is_sign_negative()
    );

    gen_quickcheck_1_simd!(
        floor_f32,
        f32::floor,
        S::floor_ps,
        f32,
        VF32_WIDTH,
        set1_ps,
        |_| false
    );
    gen_quickcheck_1_simd!(
        floor_f64,
        f64::floor,
        S::floor_pd,
        f64,
        VF64_WIDTH,
        set1_pd,
        |_| false
    );

    gen_quickcheck_1_simd!(
        ceil_f32,
        f32::ceil,
        S::ceil_ps,
        f32,
        VF32_WIDTH,
        set1_ps,
        |_| false
    );
    gen_quickcheck_1_simd!(
        ceil_f64,
        f64::ceil,
        S::ceil_pd,
        f64,
        VF64_WIDTH,
        set1_pd,
        |_| false
    );

    gen_quickcheck_1_simd!(
        round_f32,
        f32::round,
        S::round_ps,
        f32,
        VF32_WIDTH,
        set1_ps,
        |_| false
    );
    gen_quickcheck_1_simd!(
        round_f64,
        f64::round,
        S::round_pd,
        f64,
        VF64_WIDTH,
        set1_pd,
        |_| false
    );

    gen_quickcheck_1_simd!(
        fast_floor_f32,
        f32::floor,
        S::fast_floor_ps,
        f32,
        VF64_WIDTH,
        set1_ps,
        |_| false
    );
    gen_quickcheck_1_simd!(
        fast_ceil_f32,
        f32::ceil,
        S::fast_ceil_ps,
        f32,
        VF32_WIDTH,
        set1_ps,
        |_| false
    );
    gen_quickcheck_1_simd!(
        fast_round_f32,
        f32::round,
        S::fast_round_ps,
        f32,
        VF32_WIDTH,
        set1_ps,
        |_| false
    );

    // These would require accepting more imprecision not to fail
    // gen_quickcheck_1_simd!(rcp_f32, (|a| 1.0f32 / a), S::rcp_ps, f32, VF32_WIDTH, set1_ps, |_| false);
    // gen_quickcheck_1_simd!(rsqrt_f32, (|a: f32| 1.0 / a.sqrt()), S::rsqrt_ps, f32, VF32_WIDTH, set1_ps, |a: f32| a.is_sign_negative());
    // gen_quickcheck_1_simd!(rsqrt_f64, (|a: f64| 1.0/ a.sqrt()), S::rsqrt_pd, f64, VF64_WIDTH, set1_pd, |a: f64| a.is_sign_negative());
}
