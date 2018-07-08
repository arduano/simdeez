extern crate simdeez;

#[cfg(test)]
mod tests {
    use simdeez::avx2::*;
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

    #[inline(always)]
    unsafe fn maskload<S: Simd>() {
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
        let a = S::maskload_epi32(&someints[0], mask);
        let b = S::maskload_epi64(&somelongs[0], S::castepi32_epi64(mask));
        let c = S::maskload_ps(&somefloats[0], mask);
        let d = S::maskload_pd(&somedoubles[0], S::castepi32_epi64(mask));

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
        let a = S::maskload_epi32(&someints[0], mask);
        let b = S::maskload_epi64(&somelongs[0], S::castepi32_epi64(mask));
        let c = S::maskload_ps(&somefloats[0], mask);
        let d = S::maskload_pd(&somedoubles[0], S::castepi32_epi64(mask));

        for i in 0..S::VI32_WIDTH {
            assert_eq!(a[i], someints[i]);
            println!("c[{}]:{}  somfl[{}]:{}", i, c[i], i, somefloats[i]);
            assert_delta!(c[i], somefloats[i], 0.001);
        }

        for i in 0..S::VI64_WIDTH {
            assert_eq!(b[i], somelongs[i]);
            assert_delta!(d[i], somedoubles[i], 0.001);
        }
        //maskload only looks at the high bits so this should be all 0?
        let mask = S::set1_epi32(1);
        let a = S::maskload_epi32(&someints[0], mask);
        let b = S::maskload_epi64(&somelongs[0], S::castepi32_epi64(mask));
        let c = S::maskload_ps(&somefloats[0], mask);
        let d = S::maskload_pd(&somedoubles[0], S::castepi32_epi64(mask));
        for i in 0..S::VI32_WIDTH {
            assert_eq!(a[i], 0);
            assert_delta!(c[i], 0.0f32, 0.001);
        }

        for i in 0..S::VI64_WIDTH {
            assert_eq!(b[i], 0);
            assert_delta!(d[i], 0.0f64, 0.001);
        }
    }

    #[target_feature(enable = "sse2")]
    unsafe fn maskload_sse2() {
        maskload::<Sse2>()
    }
    #[target_feature(enable = "sse4.1")]
    unsafe fn maskload_sse41() {
        maskload::<Sse41>()
    }
    #[target_feature(enable = "avx2")]
    unsafe fn maskload_avx2() {
        maskload::<Avx2>()
    }

    #[test]
    fn maskload_test() {
        unsafe {
            maskload_avx2();
            maskload_sse2();
            maskload_sse41();
        }
    }

    #[inline(always)]
    unsafe fn minmax_ints<S: Simd>() -> (i32, i32, i32, i32) {
        let mut t1 = S::setzero_epi32();
        let mut t2 = S::setzero_epi32();
        for i in 0..S::VI32_WIDTH {
            let ias32 = i as i32;
            t1[i] = ias32;
            if i % 2 == 0 {
                t2[i] = ias32 * 10
            } else {
                t2[i] = -ias32;
            }
        }
        let a = S::min_epi32(t1, t2)[0];
        let b = S::min_epi32(t2, t1)[1];
        let c = S::max_epi32(t1, t2)[2];
        let d = S::max_epi32(t2, t1)[3];
        (a, b, c, d)
    }

    #[target_feature(enable = "sse2")]
    unsafe fn minmax_ints_sse2() -> (i32, i32, i32, i32) {
        minmax_ints::<Sse2>()
    }
    #[target_feature(enable = "sse4.1")]
    unsafe fn minmax_ints_sse41() -> (i32, i32, i32, i32) {
        minmax_ints::<Sse41>()
    }
    #[target_feature(enable = "avx2")]
    unsafe fn minmax_ints_avx2() -> (i32, i32, i32, i32) {
        minmax_ints::<Avx2>()
    }
    #[test]
    fn testminmax_ints_consistency() {
        unsafe {
            assert_eq!(minmax_ints_sse2(), minmax_ints_sse41());
            assert_eq!(minmax_ints_sse41(), minmax_ints_avx2());
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
