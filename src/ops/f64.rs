use super::*;

impl_op! {
    fn add<f64> {
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
        for Neon(a: float64x2_t, b: float64x2_t) -> float64x2_t {
            vaddq_f64(a, b)
        }
        for Wasm(a: v128, b: v128) -> v128 {
            f64x2_add(a, b)
        }
    }
}

impl_op! {
    fn sub<f64> {
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
        for Neon(a: float64x2_t, b: float64x2_t) -> float64x2_t {
            vsubq_f64(a, b)
        }
        for Wasm(a: v128, b: v128) -> v128 {
            f64x2_sub(a, b)
        }
    }
}

impl_op! {
    fn mul<f64> {
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
        for Neon(a: float64x2_t, b: float64x2_t) -> float64x2_t {
            vmulq_f64(a, b)
        }
        for Wasm(a: v128, b: v128) -> v128 {
            f64x2_mul(a, b)
        }
    }
}

impl_op! {
    fn div<f64> {
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
        for Neon(a: float64x2_t, b: float64x2_t) -> float64x2_t {
            vdivq_f64(a, b)
        }
        for Wasm(a: v128, b: v128) -> v128 {
            f64x2_div(a, b)
        }
    }
}

impl_op! {
    fn mul_add<f64> {
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
        for Neon(a: float64x2_t, b: float64x2_t, c: float64x2_t) -> float64x2_t {
            vmlaq_f64(c, a, b)
        }
        for Wasm(a: v128, b: v128, c: v128) -> v128 {
            f64x2_add(f64x2_mul(a, b), c)
        }
    }
}

impl_op! {
    fn mul_sub<f64> {
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
        for Neon(a: float64x2_t, b: float64x2_t, c: float64x2_t) -> float64x2_t {
            vnegq_f64(vfmsq_f64(c, a, b))
        }
        for Wasm(a: v128, b: v128, c: v128) -> v128 {
            f64x2_sub(f64x2_mul(a, b), c)
        }
    }
}

impl_op! {
    fn neg_mul_add<f64> {
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
        for Neon(a: float64x2_t, b: float64x2_t, c: float64x2_t) -> float64x2_t {
            vfmsq_f64(c, a, b)
        }
        for Wasm(a: v128, b: v128, c: v128) -> v128 {
            f64x2_sub(c, f64x2_mul(a, b))
        }
    }
}

impl_op! {
    fn neg_mul_sub<f64> {
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
        for Neon(a: float64x2_t, b: float64x2_t, c: float64x2_t) -> float64x2_t {
            vnegq_f64(vfmaq_f64(c, a, b))
        }
        for Wasm(a: v128, b: v128, c: v128) -> v128 {
            f64x2_sub(f64x2_neg(f64x2_mul(a, b)), c)
        }
    }
}

impl_op! {
    fn sqrt<f64> {
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
        for Neon(a: float64x2_t) -> float64x2_t {
            vsqrtq_f64(a)
        }
        for Wasm(a: v128) -> v128 {
            f64x2_sqrt(a)
        }
    }
}

impl_op! {
    fn rsqrt<f64> {
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
        for Neon(a: float64x2_t) -> float64x2_t {
            vrsqrteq_f64(a)
        }
        for Wasm(a: v128) -> v128 {
            f64x2_div(f64x2_splat(1.0), f64x2_sqrt(a))
        }
    }
}

impl_op! {
    fn min<f64> {
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
        for Neon(a: float64x2_t, b: float64x2_t) -> float64x2_t {
            vminq_f64(a, b)
        }
        for Wasm(a: v128, b: v128) -> v128 {
            f64x2_min(a, b)
        }
    }
}

impl_op! {
    fn max<f64> {
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
        for Neon(a: float64x2_t, b: float64x2_t) -> float64x2_t {
            vmaxq_f64(a, b)
        }
        for Wasm(a: v128, b: v128) -> v128 {
            f64x2_max(a, b)
        }
    }
}

impl_op! {
    fn abs<f64> {
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
        for Neon(a: float64x2_t) -> float64x2_t {
            vabsq_f64(a)
        }
        for Wasm(a: v128) -> v128 {
            f64x2_abs(a)
        }
    }
}

impl_op! {
    fn round<f64> {
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
        for Neon(a: float64x2_t) -> float64x2_t {
            vrndnq_f64(a)
        }
        for Wasm(a: v128) -> v128 {
            f64x2_nearest(a)
        }
    }
}

impl_op! {
    fn floor<f64> {
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
        for Neon(a: float64x2_t) -> float64x2_t {
            vrndmq_f64(a)
        }
        for Wasm(a: v128) -> v128 {
            f64x2_floor(a)
        }
    }
}

impl_op! {
    fn ceil<f64> {
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
        for Neon(a: float64x2_t) -> float64x2_t {
            vrndpq_f64(a)
        }
        for Wasm(a: v128) -> v128 {
            f64x2_ceil(a)
        }
    }
}

impl_op! {
    fn fast_round<f64> {
        for Avx2(a: __m256d) -> __m256d {
            Self::round(a)
        }
        for Sse41(a: __m128d) -> __m128d {
            Self::round(a)
        }
        for Sse2(a: __m128d) -> __m128d {
            Self::round(a)
        }
        for Scalar(a: f64) -> f64 {
            Self::round(a)
        }
        for Neon(a: float64x2_t) -> float64x2_t {
            Self::round(a)
        }
        for Wasm(a: v128) -> v128 {
            Self::round(a)
        }
    }
}

impl_op! {
    fn fast_floor<f64> {
        for Avx2(a: __m256d) -> __m256d {
            Self::floor(a)
        }
        for Sse41(a: __m128d) -> __m128d {
            Self::floor(a)
        }
        for Sse2(a: __m128d) -> __m128d {
            Self::floor(a)
        }
        for Scalar(a: f64) -> f64 {
            Self::floor(a)
        }
        for Neon(a: float64x2_t) -> float64x2_t {
            Self::floor(a)
        }
        for Wasm(a: v128) -> v128 {
            Self::floor(a)
        }
    }
}

impl_op! {
    fn fast_ceil<f64> {
        for Avx2(a: __m256d) -> __m256d {
            Self::ceil(a)
        }
        for Sse41(a: __m128d) -> __m128d {
            Self::ceil(a)
        }
        for Sse2(a: __m128d) -> __m128d {
            Self::ceil(a)
        }
        for Scalar(a: f64) -> f64 {
            Self::ceil(a)
        }
        for Neon(a: float64x2_t) -> float64x2_t {
            Self::ceil(a)
        }
        for Wasm(a: v128) -> v128 {
            Self::ceil(a)
        }
    }
}

impl_op! {
    fn eq<f64> {
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
        for Neon(a: float64x2_t, b: float64x2_t) -> float64x2_t {
            vreinterpretq_f64_u64(vceqq_f64(a, b))
        }
        for Wasm(a: v128, b: v128) -> v128 {
            f64x2_eq(a, b)
        }
    }
}

impl_op! {
    fn neq<f64> {
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
        for Neon(a: float64x2_t, b: float64x2_t) -> float64x2_t {
            vreinterpretq_f64_u32(vmvnq_u32(vreinterpretq_u32_u64(vceqq_f64(a, b))))
        }
        for Wasm(a: v128, b: v128) -> v128 {
            f64x2_ne(a, b)
        }
    }
}

impl_op! {
    fn lt<f64> {
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
        for Neon(a: float64x2_t, b: float64x2_t) -> float64x2_t {
            vreinterpretq_f64_u64(vcltq_f64(a, b))
        }
        for Wasm(a: v128, b: v128) -> v128 {
            f64x2_lt(a, b)
        }
    }
}

impl_op! {
    fn lte<f64> {
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
        for Neon(a: float64x2_t, b: float64x2_t) -> float64x2_t {
            vreinterpretq_f64_u64(vcleq_f64(a, b))
        }
        for Wasm(a: v128, b: v128) -> v128 {
            f64x2_le(a, b)
        }
    }
}

impl_op! {
    fn gt<f64> {
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
        for Neon(a: float64x2_t, b: float64x2_t) -> float64x2_t {
            vreinterpretq_f64_u64(vcgtq_f64(a, b))
        }
        for Wasm(a: v128, b: v128) -> v128 {
            f64x2_gt(a, b)
        }
    }
}

impl_op! {
    fn gte<f64> {
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
        for Neon(a: float64x2_t, b: float64x2_t) -> float64x2_t {
            vreinterpretq_f64_u64(vcgeq_f64(a, b))
        }
        for Wasm(a: v128, b: v128) -> v128 {
            f64x2_ge(a, b)
        }
    }
}

impl_op! {
    fn blendv<f64> {
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
        for Neon(a: float64x2_t, b: float64x2_t, mask: float64x2_t) -> float64x2_t {
            vbslq_f64(vreinterpretq_u64_f64(mask), b, a)
        }
        for Wasm(a: v128, b: v128, mask: v128) -> v128 {
            v128_or(v128_and(mask, b), v128_andnot(a, mask))
        }
    }
}

impl_op! {
    fn horizontal_add<f64> {
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
        for Neon(a: float64x2_t) -> f64 {
            let a = vpaddq_f64(a, a);
            vgetq_lane_f64(a, 0) + vgetq_lane_f64(a, 1)
        }
        for Wasm(a: v128) -> f64 {
            let l0 = f64x2_extract_lane::<0>(a);
            let l1 = f64x2_extract_lane::<1>(a);
            l0 + l1
        }
    }
}

impl_op! {
    fn cast_i64<f64> {
        for Avx2(a: __m256d) -> __m256i {
            let nums_arr = core::mem::transmute::<__m256d, [f64; 4]>(a);
            let ceil = [
                nums_arr[0].m_round() as i64,
                nums_arr[1].m_round() as i64,
                nums_arr[2].m_round() as i64,
                nums_arr[3].m_round() as i64,
            ];
            core::mem::transmute::<_, __m256i>(ceil)
        }
        for Sse41(a: __m128d) -> __m128i {
            let nums_arr = core::mem::transmute::<__m128d, [f64; 2]>(a);
            let ceil = [
                nums_arr[0].m_round() as i64,
                nums_arr[1].m_round() as i64,
            ];
            core::mem::transmute::<_, __m128i>(ceil)
        }
        for Sse2(a: __m128d) -> __m128i {
            let nums_arr = core::mem::transmute::<__m128d, [f64; 2]>(a);
            let ceil = [
                nums_arr[0].m_round() as i64,
                nums_arr[1].m_round() as i64,
            ];
            core::mem::transmute::<_, __m128i>(ceil)
        }
        for Scalar(a: f64) -> i64 {
            a.m_round() as i64
        }
        for Neon(a: float64x2_t) -> int64x2_t {
            let nums_arr = core::mem::transmute::<float64x2_t, [f64; 2]>(a);
            let ceil = [
                nums_arr[0].m_round() as i64,
                nums_arr[1].m_round() as i64,
            ];
            core::mem::transmute::<_, int64x2_t>(ceil)
        }
        for Wasm(a: v128) -> v128 {
            let nums_arr = core::mem::transmute::<_, [f64; 2]>(a);
            let ceil = [
                nums_arr[0].m_round() as i64,
                nums_arr[1].m_round() as i64,
            ];
            core::mem::transmute::<_, v128>(ceil)
        }
    }
}

impl_op! {
    fn bitcast_i64<f64> {
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
        for Neon(a: float64x2_t) -> int64x2_t {
            vreinterpretq_s64_f64(a)
        }
        for Wasm(a: v128) -> v128 {
            a
        }
    }
}

impl_op! {
    fn zeroes<f64> {
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
        for Neon() -> float64x2_t {
            vdupq_n_f64(0.0)
        }
        for Wasm() -> v128 {
            f64x2_splat(0.0)
        }
    }
}

impl_op! {
    fn set1<f64> {
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
        for Neon(val: f64) -> float64x2_t {
            vdupq_n_f64(val)
        }
        for Wasm(val: f64) -> v128 {
            f64x2_splat(val)
        }
    }
}

impl_op! {
    fn load_unaligned<f64> {
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
        for Neon(ptr: *const f64) -> float64x2_t {
            vld1q_f64(ptr)
        }
        for Wasm(ptr: *const f64) -> v128 {
            *(ptr as *const v128)
        }
    }
}

impl_op! {
    fn load_aligned<f64> {
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
        for Neon(ptr: *const f64) -> float64x2_t {
            vld1q_f64(ptr)
        }
        for Wasm(ptr: *const f64) -> v128 {
            *(ptr as *const v128)
        }
    }
}

impl_op! {
    fn store_unaligned<f64> {
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
        for Neon(ptr: *mut f64, a: float64x2_t) {
            vst1q_f64(ptr, a)
        }
        for Wasm(ptr: *mut f64, a: v128) {
            *(ptr as *mut v128) = a;
        }
    }
}

impl_op! {
    fn store_aligned<f64> {
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
        for Neon(ptr: *mut f64, a: float64x2_t) {
            vst1q_f64(ptr, a)
        }
        for Wasm(ptr: *mut f64, a: v128) {
            *(ptr as *mut v128) = a;
        }
    }
}
