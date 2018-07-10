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
    unsafe fn blendv<S:Simd>() {
     
        //i32
        let a = S::set1_epi32(1);
        let b=  S::set1_epi32(2);
        let cmp = S::cmplt_epi32(a,b);
        let r = S::blendv_epi32(a,b,cmp);
        //r should be all 2
        for i in 0 .. S::VI32_WIDTH {
            assert_eq!(r[i],2);
        }

        let a = S::set1_epi32(2);
        let b=  S::set1_epi32(1);
        let cmp = S::cmplt_epi32(a,b);
        let r = S::blendv_epi32(a,b,cmp);
        //r should be all 2
        for i in 0 .. S::VI32_WIDTH {
            assert_eq!(r[i],2);
        }

        //i64
        let a = S::set1_epi64(1);
        let b=  S::set1_epi64(2);
        let cmp = S::cmplt_epi64(a,b);
        let r = S::blendv_epi64(a,b,cmp);
        //r should be all 2
        for i in 0 .. S::VI64_WIDTH {
            assert_eq!(r[i],2);
        }

        let a = S::set1_epi64(2);
        let b=  S::set1_epi64(1);
        let cmp = S::cmplt_epi64(a,b);
        let r = S::blendv_epi64(a,b,cmp);
        //r should be all 2
        for i in 0 .. S::VI64_WIDTH {
            assert_eq!(r[i],2);
        }

     
         //f32
        let a = S::set1_ps(1.0);
        let b=  S::set1_ps(2.0);
        let cmp = S::cmplt_ps(a,b);
        let r = S::blendv_ps(a,b,cmp);
        //r should be all 2
        for i in 0 .. S::VI32_WIDTH {
            assert_eq!(r[i],2.0);
        }

        let a = S::set1_ps(2.0);
        let b=  S::set1_ps(1.0);
        let cmp = S::cmplt_ps(a,b);
        let r = S::blendv_ps(a,b,cmp);
        //r should be all 2
        for i in 0 .. S::VI32_WIDTH {
            assert_eq!(r[i],2.0);
        }
        let a = S::set1_ps(1.0);
        let b=  S::set1_ps(9.0);
        let cmp = S::set1_epi32(-1);
        let r = S::blendv_ps(a,b,S::castepi32_ps(cmp));
        //r should be all 9
        
         for i in 0 .. S::VI32_WIDTH {
            assert_eq!(r[i],9.0);
        }


        //f64
        let a = S::set1_pd(1.0);
        let b=  S::set1_pd(2.0);
        let cmp = S::cmplt_pd(a,b);
        let r = S::blendv_pd(a,b,cmp);
        //r should be all 2
        for i in 0 .. S::VI64_WIDTH {
            assert_eq!(r[i],2.0);
        }

        let a = S::set1_pd(2.0);
        let b=  S::set1_pd(1.0);
        let cmp = S::cmplt_pd(a,b);
        let r = S::blendv_pd(a,b,cmp);
        //r should be all 2
        for i in 0 .. S::VI64_WIDTH {
            assert_eq!(r[i],2.0);
        }

        


    }
      #[target_feature(enable="sse2")]
    unsafe fn blendv_sse2() {
        blendv::<Sse2>()
    }
   #[target_feature(enable="sse4.1")]
    unsafe fn blendv_sse41() {
        blendv::<Sse41>()
    }
   #[target_feature(enable="avx2")]
    unsafe fn blendv_avx2() {
        blendv::<Avx2>()
    }
    #[test]
    fn blendv_test() {
        unsafe {
            blendv_sse41();
            blendv_avx2();
            blendv_sse2();
        }
    }
    #[inline(always)]
    unsafe fn cmpeq<S: Simd>() {
        let the_true = -1i32;
        let the_false = 0i32;
        let ai32 = [i32::MAX, i32::MIN, 0, -1, 1, 100, -100, 10];
        let bi32 = [i32::MAX, i32::MAX, 0, 1, -1, 100, -100, 10];

        let ri32 = S::cmpeq_epi32(S::loadu_epi32(&ai32[0]), S::loadu_epi32(&bi32[0]));
        assert_eq!(ri32[0], the_true);
        assert_eq!(ri32[1], the_false);
        assert_eq!(ri32[2], the_true);
        assert_eq!(ri32[3], the_false);

        let the_true = -1i64;
        let the_false = 0i64;
        let ai64 = [i64::MAX, i64::MIN, 0, -1];
        let bi64 = [i64::MAX, i64::MAX, 0, 1];
        let ri64 = S::cmpeq_epi64(S::loadu_epi64(&ai64[0]), S::loadu_epi64(&bi64[0]));
        assert_eq!(ri64[0], the_true);
        assert_eq!(ri64[1], the_false);

        let the_true = -1i32;
        let the_false = 0i32;
        let af32 = [f32::MAX, f32::MIN, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0];
        let bf32 = [f32::MAX, f32::MAX, -0.0, 1.0, 0.0, 0.0, 0.0, 0.0];

        let rf32 = S::cmpeq_ps(S::loadu_ps(&af32[0]), S::loadu_ps(&bf32[0]));
        assert_eq!(mem::transmute::<f32, i32>(rf32[0]), the_true);
        assert_eq!(mem::transmute::<f32, i32>(rf32[1]), the_false);
        assert_eq!(mem::transmute::<f32, i32>(rf32[2]), the_true);
        assert_eq!(mem::transmute::<f32, i32>(rf32[3]), the_false);

        let the_true = -1i64;
        let the_false = 0i64;
        let af64 = [f64::MAX, f64::MIN, 0.0, -1.0];
        let bf64 = [f64::MAX, f64::MAX, -0.0, 1.0];

        let rf64 = S::cmpeq_pd(S::loadu_pd(&af64[0]), S::loadu_pd(&bf64[0]));
        assert_eq!(mem::transmute::<f64, i64>(rf64[0]), the_true);
        assert_eq!(mem::transmute::<f64, i64>(rf64[1]), the_false);
        let rf64 = S::cmpeq_pd(S::loadu_pd(&af64[2]), S::loadu_pd(&bf64[2]));
        assert_eq!(mem::transmute::<f64, i64>(rf64[0]), the_true);
        assert_eq!(mem::transmute::<f64, i64>(rf64[1]), the_false);
    }
    #[target_feature(enable = "sse2")]
    unsafe fn cmpeq_sse2() {
        cmpeq::<Sse2>();
    }
    #[target_feature(enable = "sse4.1")]
    unsafe fn cmpeq_sse41() {
        cmpeq::<Sse41>();
    }
    #[target_feature(enable = "avx2")]
    unsafe fn cmpeq_avx2() {
        cmpeq::<Avx2>();
    }
    #[test]
    fn cmpeq_test() {
        unsafe {
            cmpeq_sse2();
            cmpeq_sse41();
            cmpeq_avx2();
        }
    }

    #[inline(always)]
    unsafe fn cmplt<S: Simd>() {
        let the_true = -1i32;
        let the_false = 0i32;
        let ai32 = [i32::MAX, i32::MIN, 0, 1, 1, 100, -100, 10];
        let bi32 = [i32::MAX, i32::MAX, 0, -1, -1, 100, -100, 10];

        let ri32 = S::cmplt_epi32(S::loadu_epi32(&ai32[0]), S::loadu_epi32(&bi32[0]));
        assert_eq!(ri32[0], the_false);
        assert_eq!(ri32[1], the_true);
        assert_eq!(ri32[2], the_false);
        assert_eq!(ri32[3], the_false);

        let the_true = -1i64;
        let the_false = 0i64;
        let ai64 = [i64::MAX, i64::MIN, 0, -1];
        let bi64 = [i64::MAX, i64::MAX, 0, 1];
        let ri64 = S::cmplt_epi64(S::loadu_epi64(&ai64[0]), S::loadu_epi64(&bi64[0]));
        assert_eq!(ri64[0], the_false);
        assert_eq!(ri64[1], the_true);

        let the_true = -1i32;
        let the_false = 0i32;
        let af32 = [f32::MAX, f32::MIN, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0];
        let bf32 = [f32::MAX, f32::MAX, -0.0, 1.0, 0.0, 0.0, 0.0, 0.0];

        let rf32 = S::cmplt_ps(S::loadu_ps(&af32[0]), S::loadu_ps(&bf32[0]));
        assert_eq!(mem::transmute::<f32, i32>(rf32[0]), the_false);
        assert_eq!(mem::transmute::<f32, i32>(rf32[1]), the_true);
        assert_eq!(mem::transmute::<f32, i32>(rf32[2]), the_false);
        assert_eq!(mem::transmute::<f32, i32>(rf32[3]), the_true);

        let the_true = -1i64;
        let the_false = 0i64;
        let af64 = [f64::MAX, f64::MIN, 0.0, -1.0];
        let bf64 = [f64::MAX, f64::MAX, -0.0, 1.0];

        let rf64 = S::cmplt_pd(S::loadu_pd(&af64[0]), S::loadu_pd(&bf64[0]));
        assert_eq!(mem::transmute::<f64, i64>(rf64[0]), the_false);
        assert_eq!(mem::transmute::<f64, i64>(rf64[1]), the_true);
        let rf64 = S::cmplt_pd(S::loadu_pd(&af64[2]), S::loadu_pd(&bf64[2]));
        assert_eq!(mem::transmute::<f64, i64>(rf64[0]), the_false);
        assert_eq!(mem::transmute::<f64, i64>(rf64[1]), the_true);
    }
    #[target_feature(enable = "sse2")]
    unsafe fn cmplt_sse2() {
        cmplt::<Sse2>();
    }
    #[target_feature(enable = "sse4.1")]
    unsafe fn cmplt_sse41() {
        cmplt::<Sse41>();
    }
    #[target_feature(enable = "avx2")]
    unsafe fn cmplt_avx2() {
        cmplt::<Avx2>();
    }
    #[test]
    fn cmplt_test() {
        unsafe {
            cmplt_sse2();
            cmplt_sse41();
            cmplt_avx2();
        }
    }
    #[inline(always)]
    unsafe fn cmple<S: Simd>() {
        let the_true = -1i32;
        let the_false = 0i32;
        let ai32 = [i32::MAX, i32::MIN, 0, 1, 1, 100, -100, 10];
        let bi32 = [i32::MAX, i32::MAX, 0, -1, -1, 100, -100, 10];

        let ri32 = S::cmple_epi32(S::loadu_epi32(&ai32[0]), S::loadu_epi32(&bi32[0]));
        assert_eq!(ri32[0], the_true);
        assert_eq!(ri32[1], the_true);
        assert_eq!(ri32[2], the_true);
        assert_eq!(ri32[3], the_false);

        let the_true = -1i64;
        let the_false = 0i64;
        let ai64 = [i64::MAX, i64::MIN, 2, -1];
        let bi64 = [i64::MAX, i64::MAX, 0, 1];
        let ri64 = S::cmple_epi64(S::loadu_epi64(&ai64[0]), S::loadu_epi64(&bi64[0]));
        assert_eq!(ri64[0], the_true);
        assert_eq!(ri64[1], the_true);
        let ri64 = S::cmple_epi64(S::loadu_epi64(&ai64[2]), S::loadu_epi64(&bi64[2]));
        assert_eq!(ri64[0], the_false);
        assert_eq!(ri64[1], the_true);

        let the_true = -1i32;
        let the_false = 0i32;
        let af32 = [f32::MAX, 2.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0];
        let bf32 = [f32::MAX, 1.0, -0.0, 1.0, 0.0, 0.0, 0.0, 0.0];

        let rf32 = S::cmple_ps(S::loadu_ps(&af32[0]), S::loadu_ps(&bf32[0]));
        assert_eq!(mem::transmute::<f32, i32>(rf32[0]), the_true);
        assert_eq!(mem::transmute::<f32, i32>(rf32[1]), the_false);
        assert_eq!(mem::transmute::<f32, i32>(rf32[2]), the_true);
        assert_eq!(mem::transmute::<f32, i32>(rf32[3]), the_true);

        let the_true = -1i64;
        let the_false = 0i64;
        let af64 = [f64::MAX, f64::MIN, 0.0, 2.0];
        let bf64 = [f64::MAX, f64::MAX, -0.0, 1.0];

        let rf64 = S::cmple_pd(S::loadu_pd(&af64[0]), S::loadu_pd(&bf64[0]));
        assert_eq!(mem::transmute::<f64, i64>(rf64[0]), the_true);
        assert_eq!(mem::transmute::<f64, i64>(rf64[1]), the_true);
        let rf64 = S::cmple_pd(S::loadu_pd(&af64[2]), S::loadu_pd(&bf64[2]));
        assert_eq!(mem::transmute::<f64, i64>(rf64[0]), the_true);
        assert_eq!(mem::transmute::<f64, i64>(rf64[1]), the_false);
    }
    #[target_feature(enable = "sse2")]
    unsafe fn cmple_sse2() {
        cmple::<Sse2>();
    }
    #[target_feature(enable = "sse4.1")]
    unsafe fn cmple_sse41() {
        cmple::<Sse41>();
    }
    #[target_feature(enable = "avx2")]
    unsafe fn cmple_avx2() {
        cmple::<Avx2>();
    }
    #[test]
    fn cmple_test() {
        unsafe {
            cmple_sse2();
            cmple_sse41();
            cmple_avx2();
        }
    }

    #[inline(always)]
    unsafe fn cmpgt<S: Simd>() {
        let the_true = -1i32;
        let the_false = 0i32;
        let ai32 = [i32::MAX, i32::MIN, 0, 1, 1, 100, -100, 10];
        let bi32 = [i32::MAX, i32::MAX, 0, -1, -1, 100, -100, 10];

        let ri32 = S::cmpgt_epi32(S::loadu_epi32(&ai32[0]), S::loadu_epi32(&bi32[0]));
        assert_eq!(ri32[0], the_false);
        assert_eq!(ri32[1], the_false);
        assert_eq!(ri32[2], the_false);
        assert_eq!(ri32[3], the_true);

        let the_true = -1i64;
        let the_false = 0i64;
        let ai64 = [i64::MAX, i64::MIN, 2, -1];
        let bi64 = [i64::MAX, i64::MAX, 0, 1];
        let ri64 = S::cmpgt_epi64(S::loadu_epi64(&ai64[0]), S::loadu_epi64(&bi64[0]));
        assert_eq!(ri64[0], the_false);
        assert_eq!(ri64[1], the_false);
        let ri64 = S::cmpgt_epi64(S::loadu_epi64(&ai64[2]), S::loadu_epi64(&bi64[2]));
        assert_eq!(ri64[0], the_true);
        assert_eq!(ri64[1], the_false);

        let the_true = -1i32;
        let the_false = 0i32;
        let af32 = [f32::MAX, 2.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0];
        let bf32 = [f32::MAX, 1.0, -0.0, 1.0, 0.0, 0.0, 0.0, 0.0];

        let rf32 = S::cmpgt_ps(S::loadu_ps(&af32[0]), S::loadu_ps(&bf32[0]));
        assert_eq!(mem::transmute::<f32, i32>(rf32[0]), the_false);
        assert_eq!(mem::transmute::<f32, i32>(rf32[1]), the_true);
        assert_eq!(mem::transmute::<f32, i32>(rf32[2]), the_false);
        assert_eq!(mem::transmute::<f32, i32>(rf32[3]), the_false);

        let the_true = -1i64;
        let the_false = 0i64;
        let af64 = [f64::MAX, f64::MIN, 0.0, 2.0];
        let bf64 = [f64::MAX, f64::MAX, -0.0, 1.0];

        let rf64 = S::cmpgt_pd(S::loadu_pd(&af64[0]), S::loadu_pd(&bf64[0]));
        assert_eq!(mem::transmute::<f64, i64>(rf64[0]), the_false);
        assert_eq!(mem::transmute::<f64, i64>(rf64[1]), the_false);
        let rf64 = S::cmpgt_pd(S::loadu_pd(&af64[2]), S::loadu_pd(&bf64[2]));
        assert_eq!(mem::transmute::<f64, i64>(rf64[0]), the_false);
        assert_eq!(mem::transmute::<f64, i64>(rf64[1]), the_true);
    }
    #[target_feature(enable = "sse2")]
    unsafe fn cmpgt_sse2() {
        cmpgt::<Sse2>();
    }
    #[target_feature(enable = "sse4.1")]
    unsafe fn cmpgt_sse41() {
        cmpgt::<Sse41>();
    }
    #[target_feature(enable = "avx2")]
    unsafe fn cmpgt_avx2() {
        cmpgt::<Avx2>();
    }
    #[test]
    fn cmpgt_test() {
        unsafe {
            cmpgt_sse2();
            cmpgt_sse41();
            cmpgt_avx2();
        }
    }

    #[inline(always)]
    unsafe fn cmpge<S: Simd>() {
        let the_true = -1i32;
        let the_false = 0i32;
        let ai32 = [i32::MAX, i32::MIN, 0, 1, 1, 100, -100, 10];
        let bi32 = [i32::MAX, i32::MAX, 0, -1, -1, 100, -100, 10];

        let ri32 = S::cmpge_epi32(S::loadu_epi32(&ai32[0]), S::loadu_epi32(&bi32[0]));
        assert_eq!(ri32[0], the_true);
        assert_eq!(ri32[1], the_false);
        assert_eq!(ri32[2], the_true);
        assert_eq!(ri32[3], the_true);

        let the_true = -1i64;
        let the_false = 0i64;
        let ai64 = [i64::MAX, i64::MIN, 2, -1];
        let bi64 = [i64::MAX, i64::MAX, 0, 1];
        let ri64 = S::cmpge_epi64(S::loadu_epi64(&ai64[0]), S::loadu_epi64(&bi64[0]));
        assert_eq!(ri64[0], the_true);
        assert_eq!(ri64[1], the_false);
        let ri64 = S::cmpge_epi64(S::loadu_epi64(&ai64[2]), S::loadu_epi64(&bi64[2]));
        assert_eq!(ri64[0], the_true);
        assert_eq!(ri64[1], the_false);

        let the_true = -1i32;
        let the_false = 0i32;
        let af32 = [f32::MAX, 2.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0];
        let bf32 = [f32::MAX, 1.0, -0.0, 1.0, 0.0, 0.0, 0.0, 0.0];

        let rf32 = S::cmpge_ps(S::loadu_ps(&af32[0]), S::loadu_ps(&bf32[0]));
        assert_eq!(mem::transmute::<f32, i32>(rf32[0]), the_true);
        assert_eq!(mem::transmute::<f32, i32>(rf32[1]), the_true);
        assert_eq!(mem::transmute::<f32, i32>(rf32[2]), the_true);
        assert_eq!(mem::transmute::<f32, i32>(rf32[3]), the_false);
       let the_true = -1i64;
        let the_false = 0i64;
        let af64 = [f64::MAX, f64::MIN, 0.0, 2.0];
        let bf64 = [f64::MAX, f64::MAX, -0.0, 1.0];

        let rf64 = S::cmpge_pd(S::loadu_pd(&af64[0]), S::loadu_pd(&bf64[0]));
        assert_eq!(mem::transmute::<f64, i64>(rf64[0]), the_true);
        assert_eq!(mem::transmute::<f64, i64>(rf64[1]), the_false);
        let rf64 = S::cmpge_pd(S::loadu_pd(&af64[2]), S::loadu_pd(&bf64[2]));
        assert_eq!(mem::transmute::<f64, i64>(rf64[0]), the_true);
        assert_eq!(mem::transmute::<f64, i64>(rf64[1]), the_true);
 }
    #[target_feature(enable = "sse2")]
    unsafe fn cmpge_sse2() {
        cmpge::<Sse2>();
    }
    #[target_feature(enable = "sse4.1")]
    unsafe fn cmpge_sse41() {
        cmpge::<Sse41>();
    }
    #[target_feature(enable = "avx2")]
    unsafe fn cmpge_avx2() {
        cmpge::<Avx2>();
    }
    #[test]
    fn cmpge_test() {
        unsafe {
            cmpge_sse2();
            cmpge_sse41();
            cmpge_avx2();
        }
    }

    #[inline(always)]
    unsafe fn cmpneq<S: Simd>() {
        let the_true = -1i32;
        let the_false = 0i32;
        let ai32 = [i32::MAX, i32::MIN, 0, 1, 1, 100, -100, 10];
        let bi32 = [i32::MAX, i32::MAX, 0, -1, -1, 100, -100, 10];

        let ri32 = S::cmpneq_epi32(S::loadu_epi32(&ai32[0]), S::loadu_epi32(&bi32[0]));
        assert_eq!(ri32[0], the_false);
        assert_eq!(ri32[1], the_true);
        assert_eq!(ri32[2], the_false);
        assert_eq!(ri32[3], the_true);

        let the_true = -1i64;
        let the_false = 0i64;
        let ai64 = [i64::MAX, i64::MIN, 2, -1];
        let bi64 = [i64::MAX, i64::MAX, 0, 1];
        let ri64 = S::cmpneq_epi64(S::loadu_epi64(&ai64[0]), S::loadu_epi64(&bi64[0]));
        assert_eq!(ri64[0], the_false);
        assert_eq!(ri64[1], the_true);
        let ri64 = S::cmpneq_epi64(S::loadu_epi64(&ai64[2]), S::loadu_epi64(&bi64[2]));
        assert_eq!(ri64[0], the_true);
        assert_eq!(ri64[1], the_true);

        let the_true = -1i32;
        let the_false = 0i32;
        let af32 = [f32::MAX, 2.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0];
        let bf32 = [f32::MAX, 1.0, -0.0, 1.0, 0.0, 0.0, 0.0, 0.0];

        let rf32 = S::cmpneq_ps(S::loadu_ps(&af32[0]), S::loadu_ps(&bf32[0]));
        assert_eq!(mem::transmute::<f32, i32>(rf32[0]), the_false);
        assert_eq!(mem::transmute::<f32, i32>(rf32[1]), the_true);
        assert_eq!(mem::transmute::<f32, i32>(rf32[2]), the_false);
        assert_eq!(mem::transmute::<f32, i32>(rf32[3]), the_true);
       let the_true = -1i64;
        let the_false = 0i64;
        let af64 = [f64::MAX, f64::MIN, 0.0, 2.0];
        let bf64 = [f64::MAX, f64::MAX, -0.0, 1.0];

        let rf64 = S::cmpneq_pd(S::loadu_pd(&af64[0]), S::loadu_pd(&bf64[0]));
        assert_eq!(mem::transmute::<f64, i64>(rf64[0]), the_false);
        assert_eq!(mem::transmute::<f64, i64>(rf64[1]), the_true);
        let rf64 = S::cmpneq_pd(S::loadu_pd(&af64[2]), S::loadu_pd(&bf64[2]));
        assert_eq!(mem::transmute::<f64, i64>(rf64[0]), the_false);
        assert_eq!(mem::transmute::<f64, i64>(rf64[1]), the_true);
    }
    #[target_feature(enable = "sse2")]
    unsafe fn cmpneq_sse2() {
        cmpneq::<Sse2>();
    }
    #[target_feature(enable = "sse4.1")]
    unsafe fn cmpneq_sse41() {
        cmpneq::<Sse41>();
    }
    #[target_feature(enable = "avx2")]
    unsafe fn cmpneq_avx2() {
        cmpneq::<Avx2>();
    }
    #[test]
    fn cmpneq_test() {
        unsafe {
            cmpneq_sse2();
            cmpneq_sse41();
            cmpneq_avx2();
        }
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
            assert_delta!(c[i], somefloats[i], 0.001);
        }

        for i in 0..S::VI64_WIDTH {
            assert_eq!(b[i], somelongs[i]);
            assert_delta!(d[i], somedoubles[i], 0.001);
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
    unsafe fn maskstore<S: Simd>() {
        let mut someints = [0i32,1,2,3,4,5,6,7];
        let mut somefloats = [0.0f32,1.0,2.0,3.0,4.0,5.0,6.0,7.0];
        let mut somelongs = [0i64,1,2,3,4,5,6,7];        
        let mut somedoubles =[0.0f64,1.0,2.0,3.0,4.0,5.0,6.0,7.0];
 
        let i32data = S::set1_epi32(9);
        let f32data = S::set1_ps(9.0);
        let i64data = S::set1_epi64(9);
        let f64data = S::set1_pd(9.0);

        //Mask is all 0 so nothing should get stored                    
        let mask = S::setzero_epi32();
        S::maskstore_epi32(&mut someints[0], mask,i32data);
        S::maskstore_epi64(&mut somelongs[0], S::castepi32_epi64(mask),i64data);
        S::maskstore_ps(&mut somefloats[0], mask, f32data);
        S::maskstore_pd(&mut somedoubles[0], S::castepi32_epi64(mask),f64data);

        for i in 0..S::VI32_WIDTH {
            assert_eq!(someints[i], i as i32);
            assert_delta!(somefloats[i], i as f32, 0.001);
        }

        for i in 0..S::VI64_WIDTH {
            assert_eq!(somelongs[i], i as i64);
            //assert_delta!(somedoubles[i], i as f64, 0.001);
            assert_eq!(somedoubles[i],i as f64);
        }

        
        //All things should get stores
        let mask = S::set1_epi32(-1);
        let mask64 = S::castepi32_epi64(mask);

   
        S::maskstore_epi32(&mut someints[0], mask,i32data);
        S::maskstore_epi64(&mut somelongs[0],mask64,i64data);
        S::maskstore_ps(&mut somefloats[0], mask,f32data);
        S::maskstore_pd(&mut somedoubles[0], mask64,f64data);

        for i in 0..S::VI32_WIDTH {
            assert_eq!(someints[i], 9);
            assert_delta!(somefloats[i],9.0f32,0.001); 
        }

        for i in 0..S::VI64_WIDTH {
            assert_eq!(somelongs[i], 9);
//            assert_delta!(somedoubles[i], 9.0f64, 0.001);
              println!("i:{}",i);
            assert_eq!(somedoubles[i],9.0);
        }
/*
       //maskstore only looks at the high bits so this should be all 0?
        let mask = S::set1_epi32(1);
        let a = S::maskstore_epi32(&someints[0], mask);
        let b = S::maskstore_epi64(&somelongs[0], S::castepi32_epi64(mask));
        let c = S::maskstore_ps(&somefloats[0], mask);
        let d = S::maskstore_pd(&somedoubles[0], S::castepi32_epi64(mask));
        for i in 0..S::VI32_WIDTH {
            assert_eq!(a[i], 0);
            assert_delta!(c[i], 0.0f32, 0.001);
        }

        for i in 0..S::VI64_WIDTH {
            assert_eq!(b[i], 0);
            assert_delta!(d[i], 0.0f64, 0.001);
        }
        */
    }

    #[target_feature(enable = "sse2")]
    unsafe fn maskstore_sse2() {
        maskstore::<Sse2>()
    }
    #[target_feature(enable = "sse4.1")]
    unsafe fn maskstore_sse41() {
        maskstore::<Sse41>()
    }
    #[target_feature(enable = "avx2")]
    unsafe fn maskstore_avx2() {
        maskstore::<Avx2>()
    }

    #[test]
    fn maskstore_test() {
        unsafe {
            
            maskstore_avx2();
            maskstore_sse2();
          maskstore_sse41();
           
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
