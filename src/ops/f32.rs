use super::*;

impl_op! {
    fn add<f32> {
        for Avx2(a: __m256, b: __m256) -> __m256 {
            _mm256_add_ps(a, b)
        }
        for Sse41(a: __m128, b: __m128) -> __m128 {
            _mm_add_ps(a, b)
        }
        for Sse2(a: __m128, b: __m128) -> __m128 {
            _mm_add_ps(a, b)
        }
        for Scalar(a: f32, b: f32) -> f32 {
            a + b
        }
        for Neon(a: float32x4_t, b: float32x4_t) -> float32x4_t {
            vaddq_f32(a, b)
        }
        for Wasm(a: v128, b: v128) -> v128 {
            f32x4_add(a, b)
        }
    }
}

impl_op! {
    fn sub<f32> {
        for Avx2(a: __m256, b: __m256) -> __m256 {
            _mm256_sub_ps(a, b)
        }
        for Sse41(a: __m128, b: __m128) -> __m128 {
            _mm_sub_ps(a, b)
        }
        for Sse2(a: __m128, b: __m128) -> __m128 {
            _mm_sub_ps(a, b)
        }
        for Scalar(a: f32, b: f32) -> f32 {
            a - b
        }
        for Neon(a: float32x4_t, b: float32x4_t) -> float32x4_t {
            vsubq_f32(a, b)
        }
        for Wasm(a: v128, b: v128) -> v128 {
            f32x4_sub(a, b)
        }
    }
}

impl_op! {
    fn mul<f32> {
        for Avx2(a: __m256, b: __m256) -> __m256 {
            _mm256_mul_ps(a, b)
        }
        for Sse41(a: __m128, b: __m128) -> __m128 {
            _mm_mul_ps(a, b)
        }
        for Sse2(a: __m128, b: __m128) -> __m128 {
            _mm_mul_ps(a, b)
        }
        for Scalar(a: f32, b: f32) -> f32 {
            a * b
        }
        for Neon(a: float32x4_t, b: float32x4_t) -> float32x4_t {
            vmulq_f32(a, b)
        }
        for Wasm(a: v128, b: v128) -> v128 {
            f32x4_mul(a, b)
        }
    }
}

impl_op! {
    fn div<f32> {
        for Avx2(a: __m256, b: __m256) -> __m256 {
            _mm256_div_ps(a, b)
        }
        for Sse41(a: __m128, b: __m128) -> __m128 {
            _mm_div_ps(a, b)
        }
        for Sse2(a: __m128, b: __m128) -> __m128 {
            _mm_div_ps(a, b)
        }
        for Scalar(a: f32, b: f32) -> f32 {
            a / b
        }
        for Neon(a: float32x4_t, b: float32x4_t) -> float32x4_t {
            vdivq_f32(a, b)
        }
        for Wasm(a: v128, b: v128) -> v128 {
            f32x4_div(a, b)
        }
    }
}

impl_op! {
    fn mul_add<f32> {
        for Avx2(a: __m256, b: __m256, c: __m256) -> __m256 {
            _mm256_fmadd_ps(a, b, c)
        }
        for Sse41(a: __m128, b: __m128, c: __m128) -> __m128 {
            _mm_add_ps(_mm_mul_ps(a, b), c)
        }
        for Sse2(a: __m128, b: __m128, c: __m128) -> __m128 {
            _mm_add_ps(_mm_mul_ps(a, b), c)
        }
        for Scalar(a: f32, b: f32, c: f32) -> f32 {
            a * b + c
        }
        for Neon(a: float32x4_t, b: float32x4_t, c: float32x4_t) -> float32x4_t {
            vfmaq_f32(c, a, b)
        }
        for Wasm(a: v128, b: v128, c: v128) -> v128 {
            f32x4_add(f32x4_mul(a, b), c)
        }
    }
}

impl_op! {
    fn mul_sub<f32> {
        for Avx2(a: __m256, b: __m256, c: __m256) -> __m256 {
            _mm256_fmsub_ps(a, b, c)
        }
        for Sse41(a: __m128, b: __m128, c: __m128) -> __m128 {
            _mm_sub_ps(_mm_mul_ps(a, b), c)
        }
        for Sse2(a: __m128, b: __m128, c: __m128) -> __m128 {
            _mm_sub_ps(_mm_mul_ps(a, b), c)
        }
        for Scalar(a: f32, b: f32, c: f32) -> f32 {
            a * b - c
        }
        for Neon(a: float32x4_t, b: float32x4_t, c: float32x4_t) -> float32x4_t {
            vnegq_f32(vfmsq_f32(c, a, b))
        }
        for Wasm(a: v128, b: v128, c: v128) -> v128 {
            f32x4_sub(f32x4_mul(a, b), c)
        }
    }
}

impl_op! {
    fn neg_mul_add<f32> {
        for Avx2(a: __m256, b: __m256, c: __m256) -> __m256 {
            _mm256_fnmadd_ps(a, b, c)
        }
        for Sse41(a: __m128, b: __m128, c: __m128) -> __m128 {
            _mm_sub_ps(c, _mm_mul_ps(a, b))
        }
        for Sse2(a: __m128, b: __m128, c: __m128) -> __m128 {
            _mm_sub_ps(c, _mm_mul_ps(a, b))
        }
        for Scalar(a: f32, b: f32, c: f32) -> f32 {
            c - a * b
        }
        for Neon(a: float32x4_t, b: float32x4_t, c: float32x4_t) -> float32x4_t {
            vfmsq_f32(c, a, b)
        }
        for Wasm(a: v128, b: v128, c: v128) -> v128 {
            f32x4_sub(c, f32x4_mul(a, b))
        }
    }
}

impl_op! {
    fn neg_mul_sub<f32> {
        for Avx2(a: __m256, b: __m256, c: __m256) -> __m256 {
            _mm256_fnmsub_ps(a, b, c)
        }
        for Sse41(a: __m128, b: __m128, c: __m128) -> __m128 {
            let mul = _mm_mul_ps(a, b);
            let neg = _mm_sub_ps(_mm_setzero_ps(), mul);
            _mm_sub_ps(neg, c)
        }
        for Sse2(a: __m128, b: __m128, c: __m128) -> __m128 {
            let mul = _mm_mul_ps(a, b);
            let neg = _mm_sub_ps(_mm_setzero_ps(), mul);
            _mm_sub_ps(neg, c)
        }
        for Scalar(a: f32, b: f32, c: f32) -> f32 {
            -a * b - c
        }
        for Neon(a: float32x4_t, b: float32x4_t, c: float32x4_t) -> float32x4_t {
            vnegq_f32(vfmaq_f32(c, a, b))
        }
        for Wasm(a: v128, b: v128, c: v128) -> v128 {
            f32x4_sub(f32x4_neg(f32x4_mul(a, b)), c)
        }
    }
}

impl_op! {
    fn sqrt<f32> {
        for Avx2(a: __m256) -> __m256 {
            _mm256_sqrt_ps(a)
        }
        for Sse41(a: __m128) -> __m128 {
            _mm_sqrt_ps(a)
        }
        for Sse2(a: __m128) -> __m128 {
            _mm_sqrt_ps(a)
        }
        for Scalar(a: f32) -> f32 {
            a.m_sqrt()
        }
        for Neon(a: float32x4_t) -> float32x4_t {
            vsqrtq_f32(a)
        }
        for Wasm(a: v128) -> v128 {
            f32x4_sqrt(a)
        }
    }
}

impl_op! {
    fn recip<f32> {
        for Avx2(a: __m256) -> __m256 {
            _mm256_rcp_ps(a)
        }
        for Sse41(a: __m128) -> __m128 {
            _mm_rcp_ps(a)
        }
        for Sse2(a: __m128) -> __m128 {
            _mm_rcp_ps(a)
        }
        for Scalar(a: f32) -> f32 {
            1.0 / a
        }
        for Neon(a: float32x4_t) -> float32x4_t {
            vrecpeq_f32(a)
        }
        for Wasm(a: v128) -> v128 {
            f32x4_div(f32x4_splat(1.0), a)
        }
    }
}

impl_op! {
    fn rsqrt<f32> {
        for Avx2(a: __m256) -> __m256 {
            _mm256_rsqrt_ps(a)
        }
        for Sse41(a: __m128) -> __m128 {
            _mm_rsqrt_ps(a)
        }
        for Sse2(a: __m128) -> __m128 {
            _mm_rsqrt_ps(a)
        }
        for Scalar(a: f32) -> f32 {
            1.0 / a.m_sqrt()
        }
        for Neon(a: float32x4_t) -> float32x4_t {
            vrsqrteq_f32(a)
        }
        for Wasm(a: v128) -> v128 {
            f32x4_div(f32x4_splat(1.0), f32x4_sqrt(a))
        }
    }
}

impl_op! {
    fn min<f32> {
        for Avx2(a: __m256, b: __m256) -> __m256 {
            _mm256_min_ps(a, b)
        }
        for Sse41(a: __m128, b: __m128) -> __m128 {
            _mm_min_ps(a, b)
        }
        for Sse2(a: __m128, b: __m128) -> __m128 {
            _mm_min_ps(a, b)
        }
        for Scalar(a: f32, b: f32) -> f32 {
            a.min(b)
        }
        for Neon(a: float32x4_t, b: float32x4_t) -> float32x4_t {
            vminq_f32(a, b)
        }
        for Wasm(a: v128, b: v128) -> v128 {
            f32x4_min(a, b)
        }
    }
}

impl_op! {
    fn max<f32> {
        for Avx2(a: __m256, b: __m256) -> __m256 {
            _mm256_max_ps(a, b)
        }
        for Sse41(a: __m128, b: __m128) -> __m128 {
            _mm_max_ps(a, b)
        }
        for Sse2(a: __m128, b: __m128) -> __m128 {
            _mm_max_ps(a, b)
        }
        for Scalar(a: f32, b: f32) -> f32 {
            a.max(b)
        }
        for Neon(a: float32x4_t, b: float32x4_t) -> float32x4_t {
            vmaxq_f32(a, b)
        }
        for Wasm(a: v128, b: v128) -> v128 {
            f32x4_max(a, b)
        }
    }
}

impl_op! {
    fn abs<f32> {
        for Avx2(a: __m256) -> __m256 {
            _mm256_andnot_ps(_mm256_set1_ps(-0.0), a)
        }
        for Sse41(a: __m128) -> __m128 {
            _mm_andnot_ps(_mm_set1_ps(-0.0), a)
        }
        for Sse2(a: __m128) -> __m128 {
            _mm_andnot_ps(_mm_set1_ps(-0.0), a)
        }
        for Scalar(a: f32) -> f32 {
            a.m_abs()
        }
        for Neon(a: float32x4_t) -> float32x4_t {
            vabsq_f32(a)
        }
        for Wasm(a: v128) -> v128 {
            f32x4_abs(a)
        }
    }
}

impl_op! {
    fn round<f32> {
        for Avx2(a: __m256) -> __m256 {
            _mm256_round_ps(a, _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC)
        }
        for Sse41(a: __m128) -> __m128 {
            _mm_round_ps(a, _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC)
        }
        for Sse2(a: __m128) -> __m128 {
            let sign_mask = _mm_set1_ps(-0.0);
            let magic = _mm_castsi128_ps(_mm_set1_epi32(0x4B000000));
            let sign = _mm_and_ps(a, sign_mask);
            let signed_magic = _mm_or_ps(magic, sign);
            let b = _mm_add_ps(a, signed_magic);
            _mm_sub_ps(b, signed_magic)
        }
        for Scalar(a: f32) -> f32 {
            a.m_round()
        }
        for Neon(a: float32x4_t) -> float32x4_t {
            vrndaq_f32(a)
        }
        for Wasm(a: v128) -> v128 {
            f32x4_nearest(a)
        }
    }
}

impl_op! {
    fn floor<f32> {
        for Avx2(a: __m256) -> __m256 {
            _mm256_round_ps(a, _MM_FROUND_TO_NEG_INF | _MM_FROUND_NO_EXC)
        }
        for Sse41(a: __m128) -> __m128 {
            _mm_round_ps(a, _MM_FROUND_TO_NEG_INF | _MM_FROUND_NO_EXC)
        }
        for Sse2(a: __m128) -> __m128 {
            let nums_arr = core::mem::transmute::<__m128, [f32; 4]>(a);
            let ceil = [
                nums_arr[0].m_floor(),
                nums_arr[1].m_floor(),
                nums_arr[2].m_floor(),
                nums_arr[3].m_floor(),
            ];
            core::mem::transmute::<[f32; 4], __m128>(ceil)
        }
        for Scalar(a: f32) -> f32 {
            a.m_floor()
        }
        for Neon(a: float32x4_t) -> float32x4_t {
            vrndmq_f32(a)
        }
        for Wasm(a: v128) -> v128 {
            f32x4_floor(a)
        }
    }
}

impl_op! {
    fn ceil<f32> {
        for Avx2(a: __m256) -> __m256 {
            _mm256_round_ps(a, _MM_FROUND_TO_POS_INF | _MM_FROUND_NO_EXC)
        }
        for Sse41(a: __m128) -> __m128 {
            _mm_round_ps(a, _MM_FROUND_TO_POS_INF | _MM_FROUND_NO_EXC)
        }
        for Sse2(a: __m128) -> __m128 {
            let nums_arr = core::mem::transmute::<__m128, [f32; 4]>(a);
            let ceil = [
                nums_arr[0].m_ceil(),
                nums_arr[1].m_ceil(),
                nums_arr[2].m_ceil(),
                nums_arr[3].m_ceil(),
            ];
            core::mem::transmute::<[f32; 4], __m128>(ceil)
        }
        for Scalar(a: f32) -> f32 {
            a.m_ceil()
        }
        for Neon(a: float32x4_t) -> float32x4_t {
            vrndpq_f32(a)
        }
        for Wasm(a: v128) -> v128 {
            f32x4_ceil(a)
        }
    }
}

impl_op! {
    fn fast_round<f32> {
        for Avx2(a: __m256) -> __m256 {
            Self::round(a)
        }
        for Sse41(a: __m128) -> __m128 {
            Self::round(a)
        }
        for Sse2(a: __m128) -> __m128 {
            Self::round(a)
        }
        for Scalar(a: f32) -> f32 {
            Self::round(a)
        }
        for Neon(a: float32x4_t) -> float32x4_t {
            Self::round(a)
        }
        for Wasm(a: v128) -> v128 {
            Self::round(a)
        }
    }
}

impl_op! {
    fn fast_floor<f32> {
        for Avx2(a: __m256) -> __m256 {
            Self::floor(a)
        }
        for Sse41(a: __m128) -> __m128 {
            Self::floor(a)
        }
        for Sse2(a: __m128) -> __m128 {
            Self::floor(a)
        }
        for Scalar(a: f32) -> f32 {
            Self::floor(a)
        }
        for Neon(a: float32x4_t) -> float32x4_t {
            Self::floor(a)
        }
        for Wasm(a: v128) -> v128 {
            Self::floor(a)
        }
    }
}

impl_op! {
    fn fast_ceil<f32> {
        for Avx2(a: __m256) -> __m256 {
            Self::ceil(a)
        }
        for Sse41(a: __m128) -> __m128 {
            Self::ceil(a)
        }
        for Sse2(a: __m128) -> __m128 {
            Self::ceil(a)
        }
        for Scalar(a: f32) -> f32 {
            Self::ceil(a)
        }
        for Neon(a: float32x4_t) -> float32x4_t {
            Self::ceil(a)
        }
        for Wasm(a: v128) -> v128 {
            Self::ceil(a)
        }
    }
}

impl_op! {
    fn eq<f32> {
        for Avx2(a: __m256, b: __m256) -> __m256 {
            _mm256_cmp_ps(a, b, _CMP_EQ_OQ)
        }
        for Sse41(a: __m128, b: __m128) -> __m128 {
            _mm_cmpeq_ps(a, b)
        }
        for Sse2(a: __m128, b: __m128) -> __m128 {
            _mm_cmpeq_ps(a, b)
        }
        for Scalar(a: f32, b: f32) -> f32 {
            if a == b {
                f32::from_bits(u32::MAX)
            } else {
                0.0
            }
        }
        for Neon(a: float32x4_t, b: float32x4_t) -> float32x4_t {
            vreinterpretq_f32_u32(vceqq_f32(a, b))
        }
        for Wasm(a: v128, b: v128) -> v128 {
            f32x4_eq(a, b)
        }
    }
}

impl_op! {
    fn neq<f32> {
        for Avx2(a: __m256, b: __m256) -> __m256 {
            _mm256_cmp_ps(a, b, _CMP_NEQ_OQ)
        }
        for Sse41(a: __m128, b: __m128) -> __m128 {
            _mm_cmpneq_ps(a, b)
        }
        for Sse2(a: __m128, b: __m128) -> __m128 {
            _mm_cmpneq_ps(a, b)
        }
        for Scalar(a: f32, b: f32) -> f32 {
            if a != b {
                f32::from_bits(u32::MAX)
            } else {
                0.0
            }
        }
        for Neon(a: float32x4_t, b: float32x4_t) -> float32x4_t {
            vreinterpretq_f32_u32(vmvnq_u32(vceqq_f32(a, b)))
        }
        for Wasm(a: v128, b: v128) -> v128 {
            f32x4_ne(a, b)
        }
    }
}

impl_op! {
    fn lt<f32> {
        for Avx2(a: __m256, b: __m256) -> __m256 {
            _mm256_cmp_ps(a, b, _CMP_LT_OQ)
        }
        for Sse41(a: __m128, b: __m128) -> __m128 {
            _mm_cmplt_ps(a, b)
        }
        for Sse2(a: __m128, b: __m128) -> __m128 {
            _mm_cmplt_ps(a, b)
        }
        for Scalar(a: f32, b: f32) -> f32 {
            if a < b {
                f32::from_bits(u32::MAX)
            } else {
                0.0
            }
        }
        for Neon(a: float32x4_t, b: float32x4_t) -> float32x4_t {
            vreinterpretq_f32_u32(vcltq_f32(a, b))
        }
        for Wasm(a: v128, b: v128) -> v128 {
            f32x4_lt(a, b)
        }
    }
}

impl_op! {
    fn lte<f32> {
        for Avx2(a: __m256, b: __m256) -> __m256 {
            _mm256_cmp_ps(a, b, _CMP_LE_OQ)
        }
        for Sse41(a: __m128, b: __m128) -> __m128 {
            _mm_cmple_ps(a, b)
        }
        for Sse2(a: __m128, b: __m128) -> __m128 {
            _mm_cmple_ps(a, b)
        }
        for Scalar(a: f32, b: f32) -> f32 {
            if a <= b {
                f32::from_bits(u32::MAX)
            } else {
                0.0
            }
        }
        for Neon(a: float32x4_t, b: float32x4_t) -> float32x4_t {
            vreinterpretq_f32_u32(vcleq_f32(a, b))
        }
        for Wasm(a: v128, b: v128) -> v128 {
            f32x4_le(a, b)
        }
    }
}

impl_op! {
    fn gt<f32> {
        for Avx2(a: __m256, b: __m256) -> __m256 {
            _mm256_cmp_ps(a, b, _CMP_GT_OQ)
        }
        for Sse41(a: __m128, b: __m128) -> __m128 {
            _mm_cmpgt_ps(a, b)
        }
        for Sse2(a: __m128, b: __m128) -> __m128 {
            _mm_cmpgt_ps(a, b)
        }
        for Scalar(a: f32, b: f32) -> f32 {
            if a > b {
                f32::from_bits(u32::MAX)
            } else {
                0.0
            }
        }
        for Neon(a: float32x4_t, b: float32x4_t) -> float32x4_t {
            vreinterpretq_f32_u32(vcgtq_f32(a, b))
        }
        for Wasm(a: v128, b: v128) -> v128 {
            f32x4_gt(a, b)
        }
    }
}

impl_op! {
    fn gte<f32> {
        for Avx2(a: __m256, b: __m256) -> __m256 {
            _mm256_cmp_ps(a, b, _CMP_GE_OQ)
        }
        for Sse41(a: __m128, b: __m128) -> __m128 {
            _mm_cmpge_ps(a, b)
        }
        for Sse2(a: __m128, b: __m128) -> __m128 {
            _mm_cmpge_ps(a, b)
        }
        for Scalar(a: f32, b: f32) -> f32 {
            if a >= b {
                f32::from_bits(u32::MAX)
            } else {
                0.0
            }
        }
        for Neon(a: float32x4_t, b: float32x4_t) -> float32x4_t {
            vreinterpretq_f32_u32(vcgeq_f32(a, b))
        }
        for Wasm(a: v128, b: v128) -> v128 {
            f32x4_ge(a, b)
        }
    }
}

impl_op! {
    fn blendv<f32> {
        for Avx2(a: __m256, b: __m256, mask: __m256) -> __m256 {
            _mm256_blendv_ps(a, b, mask)
        }
        for Sse41(a: __m128, b: __m128, mask: __m128) -> __m128 {
            _mm_blendv_ps(a, b, mask)
        }
        for Sse2(a: __m128, b: __m128, mask: __m128) -> __m128 {
            _mm_or_ps(_mm_and_ps(mask, b), _mm_andnot_ps(mask, a))
        }
        for Scalar(a: f32, b: f32, mask: f32) -> f32 {
            if mask.to_bits() == 0 {
                a
            } else {
                b
            }
        }
        for Neon(a: float32x4_t, b: float32x4_t, mask: float32x4_t) -> float32x4_t {
            vbslq_f32(vreinterpretq_u32_f32(mask), b, a)
        }
        for Wasm(a: v128, b: v128, mask: v128) -> v128 {
            v128_or(v128_and(mask, b), v128_andnot(a, mask))
        }
    }
}

impl_op! {
    fn horizontal_add<f32> {
        for Avx2(a: __m256) -> f32 {
            let a = _mm256_hadd_ps(a, a);
            let b = _mm256_hadd_ps(a, a);

            let first = _mm_cvtss_f32(_mm256_extractf128_ps(b, 0));
            let second = _mm_cvtss_f32(_mm256_extractf128_ps(b, 1));

            first + second
        }
        for Sse41(a: __m128) -> f32 {
            let a = _mm_hadd_ps(a, a);
            let b = _mm_hadd_ps(a, a);

            _mm_cvtss_f32(b)
        }
        for Sse2(a: __m128) -> f32 {
            let t1 = _mm_movehl_ps(a, a);
            let t2 = _mm_add_ps(a, t1);
            let t3 = _mm_shuffle_ps(t2, t2, 1);
            _mm_cvtss_f32(t2) + _mm_cvtss_f32(t3)
        }
        for Scalar(a: f32) -> f32 {
            a
        }
        for Neon(a: float32x4_t) -> f32 {
            let a = vpaddq_f32(a, a);
            let a = vpaddq_f32(a, a);
            vgetq_lane_f32(a, 0)
        }
        for Wasm(a: v128) -> f32 {
            let l0 = f32x4_extract_lane::<0>(a);
            let l1 = f32x4_extract_lane::<1>(a);
            let l2 = f32x4_extract_lane::<2>(a);
            let l3 = f32x4_extract_lane::<3>(a);
            l0 + l1 + l2 + l3
        }
    }
}

impl_op! {
    fn cast_i32<f32> {
        for Avx2(a: __m256) -> __m256i {
            _mm256_cvtps_epi32(a)
        }
        for Sse41(a: __m128) -> __m128i {
            _mm_cvtps_epi32(a)
        }
        for Sse2(a: __m128) -> __m128i {
            _mm_cvtps_epi32(a)
        }
        for Scalar(a: f32) -> i32 {
            a.m_round() as i32
        }
        for Neon(a: float32x4_t) -> int32x4_t {
            // Because other intrinsics round instead of flooring, we round here first.
            let a = vrndnq_f32(a);
            vcvtq_s32_f32(a)
        }
        for Wasm(a: v128) -> v128 {
            let a = f32x4_nearest(a);
            i32x4_trunc_sat_f32x4(a)
        }
    }
}

impl_op! {
    fn bitcast_i32<f32> {
        for Avx2(a: __m256) -> __m256i {
            _mm256_castps_si256(a)
        }
        for Sse41(a: __m128) -> __m128i {
            _mm_castps_si128(a)
        }
        for Sse2(a: __m128) -> __m128i {
            _mm_castps_si128(a)
        }
        for Scalar(a: f32) -> i32 {
            a.to_bits() as i32
        }
        for Neon(a: float32x4_t) -> int32x4_t {
            vreinterpretq_s32_f32(a)
        }
        for Wasm(a: v128) -> v128 {
            a
        }
    }
}

impl_op! {
    fn zeroes<f32> {
        for Avx2() -> __m256 {
            _mm256_setzero_ps()
        }
        for Sse41() -> __m128 {
            _mm_setzero_ps()
        }
        for Sse2() -> __m128 {
            _mm_setzero_ps()
        }
        for Scalar() -> f32 {
            0.0
        }
        for Neon() -> float32x4_t {
            vdupq_n_f32(0.0)
        }
        for Wasm() -> v128 {
            f32x4_splat(0.0)
        }
    }
}

impl_op! {
    fn set1<f32> {
        for Avx2(val: f32) -> __m256 {
            _mm256_set1_ps(val)
        }
        for Sse41(val: f32) -> __m128 {
            _mm_set1_ps(val)
        }
        for Sse2(val: f32) -> __m128 {
            _mm_set1_ps(val)
        }
        for Scalar(val: f32) -> f32 {
            val
        }
        for Neon(val: f32) -> float32x4_t {
            vdupq_n_f32(val)
        }
        for Wasm(val: f32) -> v128 {
            f32x4_splat(val)
        }
    }
}

impl_op! {
    fn load_unaligned<f32> {
        for Avx2(ptr: *const f32) -> __m256 {
            _mm256_loadu_ps(ptr)
        }
        for Sse41(ptr: *const f32) -> __m128 {
            _mm_loadu_ps(ptr)
        }
        for Sse2(ptr: *const f32) -> __m128 {
            _mm_loadu_ps(ptr)
        }
        for Scalar(ptr: *const f32) -> f32 {
            unsafe { *ptr }
        }
        for Neon(ptr: *const f32) -> float32x4_t {
            vld1q_f32(ptr)
        }
        for Wasm(ptr: *const f32) -> v128 {
            *(ptr as *const v128)
        }
    }
}

impl_op! {
    fn load_aligned<f32> {
        for Avx2(ptr: *const f32) -> __m256 {
            _mm256_load_ps(ptr)
        }
        for Sse41(ptr: *const f32) -> __m128 {
            _mm_load_ps(ptr)
        }
        for Sse2(ptr: *const f32) -> __m128 {
            _mm_load_ps(ptr)
        }
        for Scalar(ptr: *const f32) -> f32 {
            unsafe { *ptr }
        }
        for Neon(ptr: *const f32) -> float32x4_t {
            vld1q_f32(ptr)
        }
        for Wasm(ptr: *const f32) -> v128 {
            *(ptr as *const v128)
        }
    }
}

impl_op! {
    fn store_unaligned<f32> {
        for Avx2(ptr: *mut f32, a: __m256) {
            _mm256_storeu_ps(ptr, a)
        }
        for Sse41(ptr: *mut f32, a: __m128) {
            _mm_storeu_ps(ptr, a)
        }
        for Sse2(ptr: *mut f32, a: __m128) {
            _mm_storeu_ps(ptr, a)
        }
        for Scalar(ptr: *mut f32, a: f32) {
            unsafe { *ptr = a }
        }
        for Neon(ptr: *mut f32, a: float32x4_t) {
            vst1q_f32(ptr, a)
        }
        for Wasm(ptr: *mut f32, a: v128) {
            *(ptr as *mut v128) = a;
        }
    }
}

impl_op! {
    fn store_aligned<f32> {
        for Avx2(ptr: *mut f32, a: __m256) {
            _mm256_store_ps(ptr, a)
        }
        for Sse41(ptr: *mut f32, a: __m128) {
            _mm_store_ps(ptr, a)
        }
        for Sse2(ptr: *mut f32, a: __m128) {
            _mm_store_ps(ptr, a)
        }
        for Scalar(ptr: *mut f32, a: f32) {
            unsafe { *ptr = a }
        }
        for Neon(ptr: *mut f32, a: float32x4_t) {
            vst1q_f32(ptr, a)
        }
        for Wasm(ptr: *mut f32, a: v128) {
            *(ptr as *mut v128) = a;
        }
    }
}
