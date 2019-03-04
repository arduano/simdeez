extern crate simdeez;

#[cfg(test)]
mod tests {
    use simdeez::avx2::*;
    use simdeez::scalar::*;
    use simdeez::sse2::*;
    use simdeez::sse41::*;
    use simdeez::*;
    use std::f32::*;
    use std::f64::*;
    use std::*;

    // Macro for checking if f32/f64 are equal to within a delta
    macro_rules! assert_delta {
        ($x:expr, $y:expr, $d:expr) => {
            if $x.is_nan() && $y.is_nan() {
            } else if $x.is_nan() {
                assert!(false);
            } else if $y.is_nan() {
                assert!(false);
            } else if $x.is_infinite() && $y.is_infinite() {
            } else if $x.is_infinite() {
                assert!(false);
            } else if $y.is_infinite() {
                assert!(false);
            } else {
                assert!(($x - $y).abs() < $d);
            }
        };
    }


    simd_runtime_generate!(
        fn set1(floats:&Vec<f32>, ints:&Vec<i32>) {
            for i in 0 .. ints.len() {
                let a = S::set1_epi32(ints[i]);
                for j in 0..S::VI32_WIDTH {
                    assert_eq!(a[j],ints[i]);
                }
            }
            for i in 0 .. floats.len() {
                let b = S::set1_ps(floats[i]);
                for j in 0..S::VF32_WIDTH {
                    assert_delta!(b[j],floats[i],0.001);
                }
            }
        });
    #[test]
    fn set1_test() {
        let ints = &vec![-1,1,0,10,-10,i32::max_value(),i32::min_value()];
        let floats = &vec![-1.0,1.0,0.0,-0.0,10.0,-10.0,f32::MIN,f32::MAX,f32::MIN_POSITIVE,f32::NAN,f32::NEG_INFINITY,f32::INFINITY];
        unsafe {
        set1_sse2(floats,ints);
        set1_sse41(floats,ints);
        set1_avx2(floats,ints);
        set1_scalar(floats,ints);
        }
    }
     simd_runtime_generate!(
        fn sub(floats:&Vec<f32>, ints:&Vec<i32>) {
            for i in 0 .. ints.len() {
                let a = S::sub_epi32(S::set1_epi32(ints[i]),S::set1_epi32(ints[i]));
                for j in 0..S::VI32_WIDTH {
                    assert_eq!(a[j],ints[i]-ints[i]);
                }
            }
            for i in 0 .. floats.len() {
                let b = S::sub_ps(S::set1_ps(floats[i]),S::set1_ps(floats[i]));
                for j in 0..S::VF32_WIDTH {
                    assert_delta!(b[j],floats[i]-floats[i],0.001);
                }
            }
        });
    #[test]
    fn sub_test() {
        let ints = &vec![-1,1,0,10,-10,i32::max_value(),i32::min_value()];
        let floats = &vec![-1.0,1.0,0.0,-0.0,10.0,-10.0,f32::MIN,f32::MAX,f32::MIN_POSITIVE,f32::NAN,f32::NEG_INFINITY,f32::INFINITY];
        unsafe {
        sub_sse2(floats,ints);
        sub_sse41(floats,ints);
        sub_avx2(floats,ints);
        sub_scalar(floats,ints);
        }
    }     
 simd_compiletime_generate!(
        fn cvt(floats:&Vec<f32>, ints:&Vec<i32>) {
            for i in 0 .. ints.len() {
                let a = S::cvtepi32_ps(S::set1_epi32(ints[i]));
                for j in 0..S::VI32_WIDTH {
                    assert_delta!(a[j],ints[i] as f32,0.001);
                }
            }
            for i in 0 .. floats.len() {
                let b = S::cvtps_epi32(S::set1_ps(floats[i]));
                for j in 0..S::VF32_WIDTH {
                  println!("i:{}",i);
                    let x = floats[i];
                    let rounded =  (x + 0.5).floor();
                    assert_eq!(b[j],rounded as i32);

                }
            }
        });
    #[test]
    fn cvt_test() {
        let ints = &vec![-1,1,0,10,-10,i32::max_value(),i32::min_value()];
        let floats = &vec![1.5,-1.5,-0.5,0.5,-0.999,0.999,0.0001,-0.0001,-1.0,1.0,0.0,-0.0,10.0,-10.0,f32::MIN,f32::MAX,f32::MIN_POSITIVE,f32::NAN,f32::NEG_INFINITY,f32::INFINITY];
        unsafe {
            cvt_compiletime(floats,ints);
//        cvt_sse2(floats,ints);
//        cvt_sse41(floats,ints);
//        cvt_avx2(floats,ints);
//        cvt_scalar(floats,ints);

        }
    }     
    simd_runtime_generate!(
    fn blendv() {
        //i32
        let a = S::set1_epi32(1);
        let b = S::set1_epi32(2);
        let cmp = S::cmplt_epi32(a, b);
        let r = S::blendv_epi32(a, b, cmp);
        //r should be all 2
        for i in 0..S::VI32_WIDTH {
            assert_eq!(r[i], 2);
        }

        let a = S::set1_epi32(2);
        let b = S::set1_epi32(1);
        let cmp = S::cmplt_epi32(a, b);
        let r = S::blendv_epi32(a, b, cmp);
        //r should be all 2
        for i in 0..S::VI32_WIDTH {
            assert_eq!(r[i], 2);
        }

        //i64
        let a = S::set1_epi64(1);
        let b = S::set1_epi64(2);
        let cmp = S::cmplt_epi64(a, b);
        let r = S::blendv_epi64(a, b, cmp);
        //r should be all 2
        for i in 0..S::VI64_WIDTH {
            assert_eq!(r[i], 2);
        }

        let a = S::set1_epi64(2);
        let b = S::set1_epi64(1);
        let cmp = S::cmplt_epi64(a, b);
        let r = S::blendv_epi64(a, b, cmp);
        //r should be all 2
        for i in 0..S::VI64_WIDTH {
            assert_eq!(r[i], 2);
        }

        //f32
        let a = S::set1_ps(1.0);
        let b = S::set1_ps(2.0);
        let cmp = S::cmplt_ps(a, b);
        let r = S::blendv_ps(a, b, cmp);
        //r should be all 2
        for i in 0..S::VI32_WIDTH {
            assert_eq!(r[i], 2.0);
        }

        let a = S::set1_ps(2.0);
        let b = S::set1_ps(1.0);
        let cmp = S::cmplt_ps(a, b);
        let r = S::blendv_ps(a, b, cmp);
        //r should be all 2
        for i in 0..S::VI32_WIDTH {
            assert_eq!(r[i], 2.0);
        }
        let a = S::set1_ps(1.0);
        let b = S::set1_ps(9.0);
        let cmp = S::set1_epi32(-1);
        let r = S::blendv_ps(a, b, S::castepi32_ps(cmp));
        //r should be all 9

        for i in 0..S::VI32_WIDTH {
            assert_eq!(r[i], 9.0);
        }

        //f64
        let a = S::set1_pd(1.0);
        let b = S::set1_pd(2.0);
        let cmp = S::cmplt_pd(a, b);
        let r = S::blendv_pd(a, b, cmp);
        //r should be all 2
        for i in 0..S::VI64_WIDTH {
            assert_eq!(r[i], 2.0);
        }

        let a = S::set1_pd(2.0);
        let b = S::set1_pd(1.0);
        let cmp = S::cmplt_pd(a, b);
        let r = S::blendv_pd(a, b, cmp);
        //r should be all 2
        for i in 0..S::VI64_WIDTH {
            assert_eq!(r[i], 2.0);
        }
    });

    #[test]
    fn blendv_test() {
        unsafe {
            blendv_sse41();
            blendv_avx2();
            blendv_sse2();
            blendv_scalar();
        }
    }

    simd_runtime_generate!(fn maskload() {
        let someints = [1i32, 0, -1, i32::MAX, i32::MIN, 100, -100, 1000];
        let somefloats = [
            0.0f32,
            -0.0,
            f32::MAX,
            f32::MIN,
            1.0 / 0.0,
            -100000.0,
            10000.0,
            0.000001,
        ];
        let somelongs = [1i64, 0, -1, i64::MAX, i64::MIN, 100, -100, 1000];
        let somedoubles = [
            0.0f64,
            -0.0,
            f64::MAX,
            f64::MIN,
            1.0 / 0.0,
            -100000.0,
            10000.0,
            0.000001,
        ];

        //Mask is all 0 so nothing should get loaded, only 0s
        let mask = S::setzero_epi32();
        let mask64 = S::setzero_epi64();
        let a = S::maskload_epi32(&someints[0], mask);
        let b = S::maskload_epi64(&somelongs[0], mask64);
        let c = S::maskload_ps(&somefloats[0], mask);
        let d = S::maskload_pd(&somedoubles[0], mask64);

        for i in 0..S::VI32_WIDTH {
            assert_eq!(a[i], 0);
            assert_delta!(c[i], 0.0f32, 0.001);
        }

        for i in 0..S::VI64_WIDTH {
            assert_eq!(b[i], 0);
            assert_delta!(d[i], 0.0f64, 0.001);
        }
        //All things should get loaded
        let mask = S::set1_epi32(-1);
        let mask64 = S::set1_epi64(-1);
        let a = S::maskload_epi32(&someints[0], mask);
        let b = S::maskload_epi64(&somelongs[0], mask64);
        let c = S::maskload_ps(&somefloats[0], mask);
        let d = S::maskload_pd(&somedoubles[0], mask64);

        for i in 0..S::VI32_WIDTH {
            assert_eq!(a[i], someints[i]);
            assert_delta!(c[i], somefloats[i], 0.001);
        }

        for i in 0..S::VI64_WIDTH {
            assert_eq!(b[i], somelongs[i]);
            assert_delta!(d[i], somedoubles[i], 0.001);
        }
    });

  
    #[test]
    fn maskload_test() {
        unsafe {
            maskload_avx2();
            maskload_sse2();
            maskload_sse41();
            maskload_scalar();
        }
    }

    simd_runtime_generate!( fn maskstore() {
        let mut someints = [0i32, 1, 2, 3, 4, 5, 6, 7];
        let mut somefloats = [0.0f32, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
        let mut somelongs = [0i64, 1, 2, 3, 4, 5, 6, 7];
        let mut somedoubles = [0.0f64, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];

        let i32data = S::set1_epi32(9);
        let f32data = S::set1_ps(9.0);
        let i64data = S::set1_epi64(9);
        let f64data = S::set1_pd(9.0);

        //Mask is all 0 so nothing should get stored
        let mask = S::setzero_epi32();
        let mask64 = S::setzero_epi64();
        S::maskstore_epi32(&mut someints[0], mask, i32data);
        S::maskstore_epi64(&mut somelongs[0], mask64, i64data);
        S::maskstore_ps(&mut somefloats[0], mask, f32data);
        S::maskstore_pd(&mut somedoubles[0], mask64, f64data);

        for i in 0..S::VI32_WIDTH {
            assert_eq!(someints[i], i as i32);
            assert_delta!(somefloats[i], i as f32, 0.001);
        }

        for i in 0..S::VI64_WIDTH {
            assert_eq!(somelongs[i], i as i64);
            //assert_delta!(somedoubles[i], i as f64, 0.001);
            assert_eq!(somedoubles[i], i as f64);
        }

        //All things should get stores
        let mask = S::set1_epi32(-1);
        let mask64 = S::set1_epi64(-1);

        S::maskstore_epi32(&mut someints[0], mask, i32data);
        S::maskstore_epi64(&mut somelongs[0], mask64, i64data);
        S::maskstore_ps(&mut somefloats[0], mask, f32data);
        S::maskstore_pd(&mut somedoubles[0], mask64, f64data);

        for i in 0..S::VI32_WIDTH {
            assert_eq!(someints[i], 9);
            assert_delta!(somefloats[i], 9.0f32, 0.001);
        }

        for i in 0..S::VI64_WIDTH {
            assert_eq!(somelongs[i], 9);
            //            assert_delta!(somedoubles[i], 9.0f64, 0.001);
            println!("i:{}", i);
            assert_eq!(somedoubles[i], 9.0);
        }
    });

    #[test]
    fn maskstore_test() {
        unsafe {
            maskstore_avx2();
            maskstore_sse2();
            maskstore_sse41();
            maskstore_scalar();
        }
    }

    #[inline(always)]
    unsafe fn setlanetest<S: Simd>() -> f32 {
        let mut a = S::set1_ps(1.0);
        a[0] = 5.0;
        a[0]
    }
    unsafe fn setlanetest_avx2() -> f32 {
        setlanetest::<Avx2>()
    }

    #[inline(always)]
    unsafe fn gathertest_simd<S: Simd>() -> f32 {
        let a = [4.0, 3.0, 2.0, 1.0];
        let iarr = [0, 1, 2, 3];

        let index = S::loadu_epi32(&iarr[0]);
        let result = S::i32gather_ps(&a, index);
        result[0]
    }
    unsafe fn gathertest_sse2() -> f32 {
        gathertest_simd::<Sse2>()
    }

    #[inline(always)]
    unsafe fn overload_test<S: Simd>() -> i32 {
        let a = S::set1_epi32(3);
        let b = S::set1_epi32(2);
        let c = a + b; // 5
        let d = c * b; // 10
        let mut e = d - a; // 7
        e *= b; // 14
        let mut result = S::set1_epi32(9);
        result[0] = e[0];
        result[0]
    }
    unsafe fn overload_test_sse2() -> i32 {
        overload_test::<Sse2>()
    }

    #[test]
    fn overloads() {
        unsafe {
            assert_eq!(overload_test_sse2(), 14);
        }
    }
    #[inline(always)]
    unsafe fn overload_float_test<S: Simd>() -> f32 {
        let a = S::set1_ps(3.0);
        let b = S::set1_ps(2.0);
        let c = a + b; // 5
        let d = c * b; // 10
        let e = d - a; // 7
        let e = e / b; // 3.5
        let e = e * S::set1_ps(2.0); //7
        e[0]
    }
    unsafe fn overload_float_test_sse2() -> f32 {
        overload_float_test::<Sse2>()
    }

    #[test]
    fn overloads_float() {
        unsafe {
            assert_eq!(overload_float_test_sse2(), 7.0);
        }
    }
    #[test]
    fn setlane() {
        unsafe {
            assert_eq!(setlanetest_avx2(), 5.0);
        }
    }
    #[test]
    fn gathertest() {
        unsafe {
            assert_eq!(gathertest_sse2(), 4.0);
        }
    }
}
