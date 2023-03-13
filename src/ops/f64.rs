use super::*;

impl_op! {
    fn f64_add {
        for Avx2(a: __m256d, b: __m256d) -> __m256d {
            _mm256_add_pd(a, b)
        }
        for Sse41(a: __m128d, b: __m128d) -> __m128d {
            _mm_add_pd(a, b)
        }
        for Sse2(a: __m128d, b: __m128d) -> __m128d {
            _mm_add_pd(a, b)
        }
        for Scalar(a: f64, b: f64) -> f64 {
            a + b
        }
    }
}

impl_op! {
    fn f64_sub {
        for Avx2(a: __m256d, b: __m256d) -> __m256d {
            _mm256_sub_pd(a, b)
        }
        for Sse41(a: __m128d, b: __m128d) -> __m128d {
            _mm_sub_pd(a, b)
        }
        for Sse2(a: __m128d, b: __m128d) -> __m128d {
            _mm_sub_pd(a, b)
        }
        for Scalar(a: f64, b: f64) -> f64 {
            a - b
        }
    }
}

impl_op! {
    fn f64_mul {
        for Avx2(a: __m256d, b: __m256d) -> __m256d {
            _mm256_mul_pd(a, b)
        }
        for Sse41(a: __m128d, b: __m128d) -> __m128d {
            _mm_mul_pd(a, b)
        }
        for Sse2(a: __m128d, b: __m128d) -> __m128d {
            _mm_mul_pd(a, b)
        }
        for Scalar(a: f64, b: f64) -> f64 {
            a * b
        }
    }
}

impl_op! {
    fn f64_div {
        for Avx2(a: __m256d, b: __m256d) -> __m256d {
            _mm256_div_pd(a, b)
        }
        for Sse41(a: __m128d, b: __m128d) -> __m128d {
            _mm_div_pd(a, b)
        }
        for Sse2(a: __m128d, b: __m128d) -> __m128d {
            _mm_div_pd(a, b)
        }
        for Scalar(a: f64, b: f64) -> f64 {
            a / b
        }
    }
}

impl_op! {
    fn f64_mul_add {
        for Avx2(a: __m256d, b: __m256d, c: __m256d) -> __m256d {
            _mm256_fmadd_pd(a, b, c)
        }
        for Sse41(a: __m128d, b: __m128d, c: __m128d) -> __m128d {
            _mm_add_pd(_mm_mul_pd(a, b), c)
        }
        for Sse2(a: __m128d, b: __m128d, c: __m128d) -> __m128d {
            _mm_add_pd(_mm_mul_pd(a, b), c)
        }
        for Scalar(a: f64, b: f64, c: f64) -> f64 {
            a * b + c
        }
    }
}

impl_op! {
    fn f64_mul_sub {
        for Avx2(a: __m256d, b: __m256d, c: __m256d) -> __m256d {
            _mm256_fmsub_pd(a, b, c)
        }
        for Sse41(a: __m128d, b: __m128d, c: __m128d) -> __m128d {
            _mm_sub_pd(_mm_mul_pd(a, b), c)
        }
        for Sse2(a: __m128d, b: __m128d, c: __m128d) -> __m128d {
            _mm_sub_pd(_mm_mul_pd(a, b), c)
        }
        for Scalar(a: f64, b: f64, c: f64) -> f64 {
            a * b - c
        }
    }
}

impl_op! {
    fn f64_neg_mul_add {
        for Avx2(a: __m256d, b: __m256d, c: __m256d) -> __m256d {
            _mm256_fnmadd_pd(a, b, c)
        }
        for Sse41(a: __m128d, b: __m128d, c: __m128d) -> __m128d {
            _mm_sub_pd(c, _mm_mul_pd(a, b))
        }
        for Sse2(a: __m128d, b: __m128d, c: __m128d) -> __m128d {
            _mm_sub_pd(c, _mm_mul_pd(a, b))
        }
        for Scalar(a: f64, b: f64, c: f64) -> f64 {
            c - a * b
        }
    }
}

impl_op! {
    fn f64_neg_mul_sub {
        for Avx2(a: __m256d, b: __m256d, c: __m256d) -> __m256d {
            _mm256_fnmsub_pd(a, b, c)
        }
        for Sse41(a: __m128d, b: __m128d, c: __m128d) -> __m128d {
            let mul = _mm_mul_pd(a, b);
            let neg = _mm_sub_pd(_mm_setzero_pd(), mul);
            _mm_sub_pd(neg, c)
        }
        for Sse2(a: __m128d, b: __m128d, c: __m128d) -> __m128d {
            let mul = _mm_mul_pd(a, b);
            let neg = _mm_sub_pd(_mm_setzero_pd(), mul);
            _mm_sub_pd(neg, c)
        }
        for Scalar(a: f64, b: f64, c: f64) -> f64 {
            -a * b - c
        }
    }
}

impl_op! {
    fn f64_sqrt {
        for Avx2(a: __m256d) -> __m256d {
            _mm256_sqrt_pd(a)
        }
        for Sse41(a: __m128d) -> __m128d {
            _mm_sqrt_pd(a)
        }
        for Sse2(a: __m128d) -> __m128d {
            _mm_sqrt_pd(a)
        }
        for Scalar(a: f64) -> f64 {
            a.m_sqrt()
        }
    }
}

impl_op! {
    fn f64_rsqrt {
        for Avx2(a: __m256d) -> __m256d {
            let one = _mm256_set1_pd(1.0);
            _mm256_div_pd(one, _mm256_sqrt_pd(a))
        }
        for Sse41(a: __m128d) -> __m128d {
            let one = _mm_set1_pd(1.0);
            _mm_div_pd(one, _mm_sqrt_pd(a))
        }
        for Sse2(a: __m128d) -> __m128d {
            let one = _mm_set1_pd(1.0);
            _mm_div_pd(one, _mm_sqrt_pd(a))
        }
        for Scalar(a: f64) -> f64 {
            1.0 / a.m_sqrt()
        }
    }
}

impl_op! {
    fn f64_min {
        for Avx2(a: __m256d, b: __m256d) -> __m256d {
            _mm256_min_pd(a, b)
        }
        for Sse41(a: __m128d, b: __m128d) -> __m128d {
            _mm_min_pd(a, b)
        }
        for Sse2(a: __m128d, b: __m128d) -> __m128d {
            _mm_min_pd(a, b)
        }
        for Scalar(a: f64, b: f64) -> f64 {
            a.min(b)
        }
    }
}

impl_op! {
    fn f64_max {
        for Avx2(a: __m256d, b: __m256d) -> __m256d {
            _mm256_max_pd(a, b)
        }
        for Sse41(a: __m128d, b: __m128d) -> __m128d {
            _mm_max_pd(a, b)
        }
        for Sse2(a: __m128d, b: __m128d) -> __m128d {
            _mm_max_pd(a, b)
        }
        for Scalar(a: f64, b: f64) -> f64 {
            a.max(b)
        }
    }
}

impl_op! {
    fn f64_abs {
        for Avx2(a: __m256d) -> __m256d {
            _mm256_andnot_pd(_mm256_set1_pd(-0.0), a)
        }
        for Sse41(a: __m128d) -> __m128d {
            _mm_andnot_pd(_mm_set1_pd(-0.0), a)
        }
        for Sse2(a: __m128d) -> __m128d {
            _mm_andnot_pd(_mm_set1_pd(-0.0), a)
        }
        for Scalar(a: f64) -> f64 {
            a.m_abs()
        }
    }
}

impl_op! {
    fn f64_round {
        for Avx2(a: __m256d) -> __m256d {
            _mm256_round_pd(a, _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC)
        }
        for Sse41(a: __m128d) -> __m128d {
            _mm_round_pd(a, _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC)
        }
        for Sse2(a: __m128d) -> __m128d {
            let sign_mask = _mm_set1_pd(-0.0);
            let magic = _mm_castsi128_pd(_mm_set_epi32(
                0x43300000, 0, 0x43300000, 0,
            ));
            let sign = _mm_and_pd(a, sign_mask);
            let signedmagic = _mm_or_pd(magic, sign);
            let b = _mm_add_pd(a, signedmagic);
            _mm_sub_pd(b, signedmagic)
        }
        for Scalar(a: f64) -> f64 {
            a.m_round()
        }
    }
}

impl_op! {
    fn f64_floor {
        for Avx2(a: __m256d) -> __m256d {
            _mm256_round_pd(a, _MM_FROUND_TO_NEG_INF | _MM_FROUND_NO_EXC)
        }
        for Sse41(a: __m128d) -> __m128d {
            _mm_round_pd(a, _MM_FROUND_TO_NEG_INF | _MM_FROUND_NO_EXC)
        }
        for Sse2(a: __m128d) -> __m128d {
            let nums_arr = core::mem::transmute::<__m128d, [f64; 2]>(a);
            let ceil = [
                nums_arr[0].m_floor(),
                nums_arr[1].m_floor(),
            ];
            core::mem::transmute::<[f64; 2], __m128d>(ceil)
        }
        for Scalar(a: f64) -> f64 {
            a.m_floor()
        }
    }
}

impl_op! {
    fn f64_ceil {
        for Avx2(a: __m256d) -> __m256d {
            _mm256_round_pd(a, _MM_FROUND_TO_POS_INF | _MM_FROUND_NO_EXC)
        }
        for Sse41(a: __m128d) -> __m128d {
            _mm_round_pd(a, _MM_FROUND_TO_POS_INF | _MM_FROUND_NO_EXC)
        }
        for Sse2(a: __m128d) -> __m128d {
            let nums_arr = core::mem::transmute::<__m128d, [f64; 2]>(a);
            let ceil = [
                nums_arr[0].m_ceil(),
                nums_arr[1].m_ceil(),
            ];
            core::mem::transmute::<[f64; 2], __m128d>(ceil)
        }
        for Scalar(a: f64) -> f64 {
            a.m_ceil()
        }
    }
}

impl_op! {
    fn f64_fast_round {
        for Avx2(a: __m256d) -> __m256d {
            Self::f64_round(a)
        }
        for Sse41(a: __m128d) -> __m128d {
            Self::f64_round(a)
        }
        for Sse2(a: __m128d) -> __m128d {
            Self::f64_round(a)
        }
        for Scalar(a: f64) -> f64 {
            Self::f64_round(a)
        }
    }
}

impl_op! {
    fn f64_fast_floor {
        for Avx2(a: __m256d) -> __m256d {
            Self::f64_floor(a)
        }
        for Sse41(a: __m128d) -> __m128d {
            Self::f64_floor(a)
        }
        for Sse2(a: __m128d) -> __m128d {
            Self::f64_floor(a)
        }
        for Scalar(a: f64) -> f64 {
            Self::f64_floor(a)
        }
    }
}

impl_op! {
    fn f64_fast_ceil {
        for Avx2(a: __m256d) -> __m256d {
            Self::f64_ceil(a)
        }
        for Sse41(a: __m128d) -> __m128d {
            Self::f64_ceil(a)
        }
        for Sse2(a: __m128d) -> __m128d {
            Self::f64_ceil(a)
        }
        for Scalar(a: f64) -> f64 {
            Self::f64_ceil(a)
        }
    }
}

impl_op! {
    fn f64_eq {
        for Avx2(a: __m256d, b: __m256d) -> __m256d {
            _mm256_cmp_pd(a, b, _CMP_EQ_OQ)
        }
        for Sse41(a: __m128d, b: __m128d) -> __m128d {
            _mm_cmpeq_pd(a, b)
        }
        for Sse2(a: __m128d, b: __m128d) -> __m128d {
            _mm_cmpeq_pd(a, b)
        }
        for Scalar(a: f64, b: f64) -> f64 {
            if a == b {
                f64::from_bits(u64::MAX)
            } else {
                0.0
            }
        }
    }
}

impl_op! {
    fn f64_neq {
        for Avx2(a: __m256d, b: __m256d) -> __m256d {
            _mm256_cmp_pd(a, b, _CMP_NEQ_OQ)
        }
        for Sse41(a: __m128d, b: __m128d) -> __m128d {
            _mm_cmpneq_pd(a, b)
        }
        for Sse2(a: __m128d, b: __m128d) -> __m128d {
            _mm_cmpneq_pd(a, b)
        }
        for Scalar(a: f64, b: f64) -> f64 {
            if a != b {
                f64::from_bits(u64::MAX)
            } else {
                0.0
            }
        }
    }
}

impl_op! {
    fn f64_lt {
        for Avx2(a: __m256d, b: __m256d) -> __m256d {
            _mm256_cmp_pd(a, b, _CMP_LT_OQ)
        }
        for Sse41(a: __m128d, b: __m128d) -> __m128d {
            _mm_cmplt_pd(a, b)
        }
        for Sse2(a: __m128d, b: __m128d) -> __m128d {
            _mm_cmplt_pd(a, b)
        }
        for Scalar(a: f64, b: f64) -> f64 {
            if a < b {
                f64::from_bits(u64::MAX)
            } else {
                0.0
            }
        }
    }
}

impl_op! {
    fn f64_lte {
        for Avx2(a: __m256d, b: __m256d) -> __m256d {
            _mm256_cmp_pd(a, b, _CMP_LE_OQ)
        }
        for Sse41(a: __m128d, b: __m128d) -> __m128d {
            _mm_cmple_pd(a, b)
        }
        for Sse2(a: __m128d, b: __m128d) -> __m128d {
            _mm_cmple_pd(a, b)
        }
        for Scalar(a: f64, b: f64) -> f64 {
            if a <= b {
                f64::from_bits(u64::MAX)
            } else {
                0.0
            }
        }
    }
}

impl_op! {
    fn f64_gt {
        for Avx2(a: __m256d, b: __m256d) -> __m256d {
            _mm256_cmp_pd(a, b, _CMP_GT_OQ)
        }
        for Sse41(a: __m128d, b: __m128d) -> __m128d {
            _mm_cmpgt_pd(a, b)
        }
        for Sse2(a: __m128d, b: __m128d) -> __m128d {
            _mm_cmpgt_pd(a, b)
        }
        for Scalar(a: f64, b: f64) -> f64 {
            if a > b {
                f64::from_bits(u64::MAX)
            } else {
                0.0
            }
        }
    }
}

impl_op! {
    fn f64_gte {
        for Avx2(a: __m256d, b: __m256d) -> __m256d {
            _mm256_cmp_pd(a, b, _CMP_GE_OQ)
        }
        for Sse41(a: __m128d, b: __m128d) -> __m128d {
            _mm_cmpge_pd(a, b)
        }
        for Sse2(a: __m128d, b: __m128d) -> __m128d {
            _mm_cmpge_pd(a, b)
        }
        for Scalar(a: f64, b: f64) -> f64 {
            if a >= b {
                f64::from_bits(u64::MAX)
            } else {
                0.0
            }
        }
    }
}

impl_op! {
    fn f64_blendv {
        for Avx2(a: __m256d, b: __m256d, mask: __m256d) -> __m256d {
            _mm256_blendv_pd(a, b, mask)
        }
        for Sse41(a: __m128d, b: __m128d, mask: __m128d) -> __m128d {
            _mm_blendv_pd(a, b, mask)
        }
        for Sse2(a: __m128d, b: __m128d, mask: __m128d) -> __m128d {
            _mm_or_pd(_mm_and_pd(mask, b), _mm_andnot_pd(mask, a))
        }
        for Scalar(a: f64, b: f64, mask: f64) -> f64 {
            if mask.to_bits() == 0 {
                a
            } else {
                b
            }
        }
    }
}

impl_op! {
    fn f64_horizontal_add {
        for Avx2(a: __m256d) -> f64 {
            let a = _mm256_hadd_pd(a, a);
            let b = _mm256_hadd_pd(a, a);

            let first = _mm_cvtsd_f64(_mm256_extractf128_pd(b, 0));
            let second = _mm_cvtsd_f64(_mm256_extractf128_pd(b, 1));

            first + second
        }
        for Sse41(a: __m128d) -> f64 {
            let a = _mm_hadd_pd(a, a);

            let first = _mm_cvtsd_f64(a);
            let second = _mm_cvtsd_f64(_mm_shuffle_pd(a, a, 1));

            first + second
        }
        for Sse2(a: __m128d) -> f64 {
            let a = _mm_add_pd(a, _mm_shuffle_pd(a, a, 1));

            let first = _mm_cvtsd_f64(a);
            let second = _mm_cvtsd_f64(_mm_shuffle_pd(a, a, 1));

            first + second
        }
        for Scalar(a: f64) -> f64 {
            a
        }
    }
}

impl_op! {
    fn f64_cast_i64 {
        for Avx2(a: __m256d) -> __m256i {
            let nums_arr = core::mem::transmute::<_, [f64; 4]>(a);
            let ceil = [
                nums_arr[0].m_round() as i64,
                nums_arr[1].m_round() as i64,
                nums_arr[2].m_round() as i64,
                nums_arr[3].m_round() as i64,
            ];
            core::mem::transmute::<_, __m256i>(ceil)
        }
        for Sse41(a: __m128d) -> __m128i {
            let nums_arr = core::mem::transmute::<_, [f64; 2]>(a);
            let ceil = [
                nums_arr[0].m_round() as i64,
                nums_arr[1].m_round() as i64,
            ];
            core::mem::transmute::<_, __m128i>(ceil)
        }
        for Sse2(a: __m128d) -> __m128i {
            let nums_arr = core::mem::transmute::<_, [f64; 2]>(a);
            let ceil = [
                nums_arr[0].m_round() as i64,
                nums_arr[1].m_round() as i64,
            ];
            core::mem::transmute::<_, __m128i>(ceil)
        }
        for Scalar(a: f64) -> i64 {
            a.m_round() as i64
        }
    }
}

impl_op! {
    fn f64_bitcast_i64 {
        for Avx2(a: __m256d) -> __m256i {
            _mm256_castpd_si256(a)
        }
        for Sse41(a: __m128d) -> __m128i {
            _mm_castpd_si128(a)
        }
        for Sse2(a: __m128d) -> __m128i {
            _mm_castpd_si128(a)
        }
        for Scalar(a: f64) -> i64 {
            a.to_bits() as i64
        }
    }
}

impl_op! {
    fn f64_zeroes {
        for Avx2() -> __m256d {
            _mm256_setzero_pd()
        }
        for Sse41() -> __m128d {
            _mm_setzero_pd()
        }
        for Sse2() -> __m128d {
            _mm_setzero_pd()
        }
        for Scalar() -> f64 {
            0.0
        }
    }
}

impl_op! {
    fn f64_set1 {
        for Avx2(val: f64) -> __m256d {
            _mm256_set1_pd(val)
        }
        for Sse41(val: f64) -> __m128d {
            _mm_set1_pd(val)
        }
        for Sse2(val: f64) -> __m128d {
            _mm_set1_pd(val)
        }
        for Scalar(val: f64) -> f64 {
            val
        }
    }
}

impl_op! {
    fn f64_load_unaligned {
        for Avx2(ptr: *const f64) -> __m256d {
            _mm256_loadu_pd(ptr)
        }
        for Sse41(ptr: *const f64) -> __m128d {
            _mm_loadu_pd(ptr)
        }
        for Sse2(ptr: *const f64) -> __m128d {
            _mm_loadu_pd(ptr)
        }
        for Scalar(ptr: *const f64) -> f64 {
            unsafe { *ptr }
        }
    }
}

impl_op! {
    fn f64_load_aligned {
        for Avx2(ptr: *const f64) -> __m256d {
            _mm256_load_pd(ptr)
        }
        for Sse41(ptr: *const f64) -> __m128d {
            _mm_load_pd(ptr)
        }
        for Sse2(ptr: *const f64) -> __m128d {
            _mm_load_pd(ptr)
        }
        for Scalar(ptr: *const f64) -> f64 {
            unsafe { *ptr }
        }
    }
}

impl_op! {
    fn f64_store_unaligned {
        for Avx2(ptr: *mut f64, a: __m256d) {
            _mm256_storeu_pd(ptr, a)
        }
        for Sse41(ptr: *mut f64, a: __m128d) {
            _mm_storeu_pd(ptr, a)
        }
        for Sse2(ptr: *mut f64, a: __m128d) {
            _mm_storeu_pd(ptr, a)
        }
        for Scalar(ptr: *mut f64, a: f64) {
            unsafe { *ptr = a }
        }
    }
}

impl_op! {
    fn f64_store_aligned {
        for Avx2(ptr: *mut f64, a: __m256d) {
            _mm256_store_pd(ptr, a)
        }
        for Sse41(ptr: *mut f64, a: __m128d) {
            _mm_store_pd(ptr, a)
        }
        for Sse2(ptr: *mut f64, a: __m128d) {
            _mm_store_pd(ptr, a)
        }
        for Scalar(ptr: *mut f64, a: f64) {
            unsafe { *ptr = a }
        }
    }
}
