extern crate simdeez;
#[cfg(test)]
mod tests {
    use simdeez::avx2::*;
    use simdeez::sse2::*;
    use simdeez::sse41::*;
    use simdeez::*;

    // If using runtime feature detection, you will want to be sure this inlines
    // so you can leverage target_feature attributes
    #[inline(always)]
    unsafe fn distance<S: Simd>(x1: &[f32], y1: &[f32], x2: &[f32], y2: &[f32]) -> Vec<f32> {
        let mut result: Vec<f32> = Vec::with_capacity(x1.len());
        result.set_len(x1.len()); // for efficiency

        // Operations have to be done in terms of the vector width
        // so that it will work with any size vector.
        // the width of a vector type is provided as a constant
        // so the compiler is free to optimize it more.
        let mut i = 0;
        //S::VF32_WIDTH is a constant, 4 when using SSE, 8 when using AVX2, etc
        while i < x1.len() {
            //load data from your vec into a SIMD value
            let xv1 = S::loadu_ps(&x1[i]);
            let yv1 = S::loadu_ps(&y1[i]);
            let xv2 = S::loadu_ps(&x2[i]);
            let yv2 = S::loadu_ps(&y2[i]);

            // Use the usual intrinsic syntax if you prefer
            let mut xdiff = S::sub_ps(xv1, xv2);
            // Or use operater overloading if you like
            let mut ydiff = yv1 - yv2;
            xdiff *= xdiff;
            ydiff *= ydiff;
            let distance = S::sqrt_ps(xdiff + ydiff);
            // Store the SIMD value into the result vec
            S::storeu_ps(&mut result[i], distance);
            // Increment i by the vector width
            i += S::VF32_WIDTH
        }
        result
    }

    //Call distance as an SSE2 function
    #[target_feature(enable = "sse2")]
    unsafe fn distance_sse2(x1: &[f32], y1: &[f32], x2: &[f32], y2: &[f32]) -> Vec<f32> {
        distance::<Sse2>(x1, y1, x2, y2)
    }
    //Call distance as an SSE41 function
    #[target_feature(enable = "sse4.1")]
    unsafe fn distance_sse41(x1: &[f32], y1: &[f32], x2: &[f32], y2: &[f32]) -> Vec<f32> {
        distance::<Sse41>(x1, y1, x2, y2)
    }
    //Call distance as an AVX2 function
    #[target_feature(enable = "avx2")]
    unsafe fn distance_avx2(x1: &[f32], y1: &[f32], x2: &[f32], y2: &[f32]) -> Vec<f32> {
        distance::<Avx2>(x1, y1, x2, y2)
    }

    #[test]
    fn distance_test() {
        unsafe {
            let x1 = [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
            let y1 = [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
            let x2 = [8.0f32, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];
            let y2 = [8.0f32, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];

            let distances_sse2 = distance_sse2(&x1, &y1, &x2, &y2);
            println!("sse2 dist:{:?}", distances_sse2);
            let distances_sse41 = distance_sse41(&x1, &y1, &x2, &y2);
            println!("sse41 dist:{:?}", distances_sse41);
            let distances_avx2 = distance_avx2(&x1, &y1, &x2, &y2);
            println!("avx2 dist:{:?}", distances_avx2);
            for i in 0..8 {
                assert_eq!(distances_sse2[i], distances_sse41[i]);
                assert_eq!(distances_sse41[i], distances_avx2[i]);
            }
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

    // If using runtime feature detection, you will want to be sure this inlines
    #[inline(always)]
    unsafe fn sample<S: Simd>() -> i32 {
        let a = S::set1_epi32(3);
        let b = S::set1_epi32(-1);
        let c = a + b; //2
                       // let d = a + 2; // 4
        c[S::VF32_WIDTH - 1]
    }

    // Make an sse2 version of sample
    #[target_feature(enable = "sse2")]
    unsafe fn sample_sse2() -> i32 {
        sample::<Sse2>()
    }

    // Make an avx2 version of sample
    #[target_feature(enable = "avx2")]
    unsafe fn sample_avx2() -> i32 {
        sample::<Avx2>()
    }
    #[target_feature(enable = "sse4.1")]
    unsafe fn sample_sse41() -> i32 {
        sample::<Sse41>()
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
    fn consistency() {
        unsafe {
            assert_eq!(sample_sse2(), sample_sse41());
            assert_eq!(sample_sse41(), sample_avx2());
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
    #[test]
    fn overloadi32() {
        unsafe {
            assert_eq!(sample_sse2(), 2);
            assert_eq!(sample_sse41(), 2);
            assert_eq!(sample_avx2(), 2);
        }
    }
}
