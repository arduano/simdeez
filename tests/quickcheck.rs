#[cfg(test)]
mod tests {
    use core::ops::*;

    use quickcheck::{TestResult, quickcheck};

    use simdeez::*;
    use simdeez::avx2::*;
    use simdeez::avx::*;
    use simdeez::scalar::*;
    use simdeez::sse2::*;
    use simdeez::sse41::*;

    macro_rules! gen_quickcheck_2_simd {
        ($fn_name:ident, $operation_scalar:expr, $operation_simd:expr, $ty:ty, $width:ident, $set_fn:ident) => {
            simd_runtime_generate!(
                fn $fn_name(a: $ty, b: $ty) -> bool {
                    let a_simd = S::$set_fn(a);
                    let b_simd = S::$set_fn(b);

                    let result_simd = $operation_simd(a_simd, b_simd);
                    let result_scalar = $operation_scalar(a, b);

                    for i in 0..S::$width {
                        if result_simd[i] != result_scalar {
                            println!(
                                "Values didn't match. Reference: {:?} Simdeez: {:?}",
                                result_scalar,
                                result_simd
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
                            println!("Testing scalar:");
                            ok &= [<$fn_name _scalar>](a, b);

                            if is_x86_feature_detected!("sse2"){
                                println!("\n Testing sse2:");
                                ok &= [<$fn_name _sse2>](a, b);
                            }
                            if is_x86_feature_detected!("sse4.1"){
                                println!("\n Testing sse41:");
                                ok &= [<$fn_name _sse41>](a, b);
                            }
                            if is_x86_feature_detected!("avx"){
                                println!("\n Testing avx:");
                                ok &= [<$fn_name _avx>](a, b);
                            }
                            if is_x86_feature_detected!("avx2"){
                                println!("\n Testing avx2:");
                                ok &= [<$fn_name _avx2>](a, b);
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

    gen_quickcheck_2_simd!(add_i64, Add::add, Add::add, i64, VI64_WIDTH, set1_epi64);
    gen_quickcheck_2_simd!(sub_i64, Sub::sub, Sub::sub, i64, VI64_WIDTH, set1_epi64);

    gen_quickcheck_2_simd!(bitand_i32, BitAnd::bitand, BitAnd::bitand, i32, VI32_WIDTH, set1_epi32);
    gen_quickcheck_2_simd!(bitor_i32, BitOr::bitor, BitOr::bitor, i32, VI32_WIDTH, set1_epi32);
    gen_quickcheck_2_simd!(bitxor_i32, BitXor::bitxor, BitXor::bitxor, i32, VI32_WIDTH, set1_epi32);

    gen_quickcheck_2_simd!(bitand_i64, BitAnd::bitand, BitAnd::bitand, i64, VI64_WIDTH, set1_epi64);
    gen_quickcheck_2_simd!(bitor_i64, BitOr::bitor, BitOr::bitor, i64, VI64_WIDTH, set1_epi64);
    gen_quickcheck_2_simd!(bitxor_i64, BitXor::bitxor, BitXor::bitxor, i64, VI64_WIDTH, set1_epi64);

    // Other

    gen_quickcheck_2_simd!(cmpeq_f32, (|a, b| if a == b { !0u32 } else { 0u32 } as f32 ), S::cmpeq_ps, f32, VF32_WIDTH, set1_ps);
    gen_quickcheck_2_simd!(cmpeq_f64, (|a, b| if a == b { !0u64 } else { 0u64 } as f64 ), S::cmpeq_pd, f64, VF64_WIDTH, set1_pd);

    gen_quickcheck_2_simd!(min_f32, f32::min, S::min_ps, f32, VF32_WIDTH, set1_ps);
    gen_quickcheck_2_simd!(max_f32, f32::max, S::max_ps, f32, VF32_WIDTH, set1_ps);

    gen_quickcheck_2_simd!(min_f64, f64::min, S::min_pd, f64, VF64_WIDTH, set1_pd);
    gen_quickcheck_2_simd!(max_f64, f64::max, S::max_pd, f64, VF64_WIDTH, set1_pd);

    gen_quickcheck_2_simd!(min_i32, Ord::min, S::min_epi32, i32, VI32_WIDTH, set1_epi32);
    gen_quickcheck_2_simd!(max_i32, Ord::max, S::max_epi32, i32, VI32_WIDTH, set1_epi32);
}
