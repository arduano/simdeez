extern crate simdeez;

#[cfg(test)]
mod tests {
    use simdeez::avx::*;
    use simdeez::avx2::*;
    use simdeez::scalar::*;
    use simdeez::*;
    use simdeez::sse2::*;
    use simdeez::sse41::*;
    use std::*;

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
    //Call distance as an AVX2 function
    #[target_feature(enable = "avx")]
    unsafe fn distance_avx(x1: &[f32], y1: &[f32], x2: &[f32], y2: &[f32]) -> Vec<f32> {
        distance::<Avx>(x1, y1, x2, y2)
    }
    //Call distance as scalar
    unsafe fn distance_scalar(x1: &[f32], y1: &[f32], x2: &[f32], y2: &[f32]) -> Vec<f32> {
        distance::<Scalar>(x1, y1, x2, y2)
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
            let distances_avx = distance_avx(&x1, &y1, &x2, &y2);
            println!("avx dist:{:?}", distances_avx);
            let distance_scalar = distance_scalar(&x1, &y1, &x2, &y2);
            for i in 0..8 {
                assert_eq!(distances_sse2[i], distances_sse41[i]);
                assert_eq!(distances_sse41[i], distances_avx[i]);
                assert_eq!(distances_avx[i], distances_avx2[i]);
                assert_eq!(distances_sse2[i], distance_scalar[i]);
            }
        }
    }

    // If using runtime feature detection, you will want to be sure this inlines
    #[inline(always)]
    unsafe fn sample<S: Simd>() -> i32 {
        let a = S::set1_epi32(3);
        let b = S::set1_epi32(-1);
        let c = S::add_epi32(a, b); //2

        c[S::VI32_WIDTH - 1]
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
    // Make an avx version of sample
    #[target_feature(enable = "avx")]
    unsafe fn sample_avx() -> i32 {
        sample::<Avx>()
    }
    #[target_feature(enable = "sse4.1")]
    unsafe fn sample_sse41() -> i32 {
        sample::<Sse41>()
    }
    unsafe fn sample_scalar() -> i32 {
        sample::<Scalar>()
    }
    #[test]
    fn consistency() {
        unsafe {
            assert_eq!(sample_sse2(), sample_sse41());
            assert_eq!(sample_sse41(), sample_avx());
            assert_eq!(sample_avx(), sample_avx2());
            assert_eq!(sample_scalar(), sample_avx2());
        }
    }
}
