use super::*;

impl_op! {
    fn f32_add {
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
    }
}

impl_op! {
    fn f32_sub {
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
    }
}

impl_op! {
    fn f32_mul {
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
    }
}

impl_op! {
    fn f32_div {
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
    }
}

impl_op! {
    fn f32_mul_add {
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
    }
}

impl_op! {
    fn f32_mul_sub {
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
    }
}

impl_op! {
    fn f32_neg_mul_add {
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
    }
}

impl_op! {
    fn f32_neg_mul_sub {
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
    }
}

impl_op! {
    fn f32_sqrt {
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
    }
}

impl_op! {
    fn f32_recip {
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
    }
}

impl_op! {
    fn f32_rsqrt {
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
    }
}

impl_op! {
    fn f32_min {
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
    }
}

impl_op! {
    fn f32_max {
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
    }
}

impl_op! {
    fn f32_abs {
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
    }
}

impl_op! {
    fn f32_round {
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
    }
}

impl_op! {
    fn f32_floor {
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
    }
}

impl_op! {
    fn f32_ceil {
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
    }
}

impl_op! {
    fn f32_fast_round {
        for Avx2(a: __m256) -> __m256 {
            Self::f32_round(a)
        }
        for Sse41(a: __m128) -> __m128 {
            Self::f32_round(a)
        }
        for Sse2(a: __m128) -> __m128 {
            Self::f32_round(a)
        }
        for Scalar(a: f32) -> f32 {
            Self::f32_round(a)
        }
    }
}

impl_op! {
    fn f32_fast_floor {
        for Avx2(a: __m256) -> __m256 {
            Self::f32_floor(a)
        }
        for Sse41(a: __m128) -> __m128 {
            Self::f32_floor(a)
        }
        for Sse2(a: __m128) -> __m128 {
            Self::f32_floor(a)
        }
        for Scalar(a: f32) -> f32 {
            Self::f32_floor(a)
        }
    }
}

impl_op! {
    fn f32_fast_ceil {
        for Avx2(a: __m256) -> __m256 {
            Self::f32_ceil(a)
        }
        for Sse41(a: __m128) -> __m128 {
            Self::f32_ceil(a)
        }
        for Sse2(a: __m128) -> __m128 {
            Self::f32_ceil(a)
        }
        for Scalar(a: f32) -> f32 {
            Self::f32_ceil(a)
        }
    }
}

impl_op! {
    fn f32_eq {
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
    }
}

impl_op! {
    fn f32_neq {
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
    }
}

impl_op! {
    fn f32_lt {
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
    }
}

impl_op! {
    fn f32_lte {
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
    }
}

impl_op! {
    fn f32_gt {
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
    }
}

impl_op! {
    fn f32_gte {
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
    }
}

impl_op! {
    fn f32_blendv {
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
    }
}

impl_op! {
    fn f32_horizontal_add {
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
    }
}

impl_op! {
    fn f32_cast_i32 {
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
            a as i32
        }
    }
}

impl_op! {
    fn f32_bitcast_i32 {
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
    }
}

impl_op! {
    fn f32_zeroes {
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
    }
}

impl_op! {
    fn f32_set1 {
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
    }
}

impl_op! {
    fn f32_load_unaligned {
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
    }
}

impl_op! {
    fn f32_load_aligned {
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
    }
}

impl_op! {
    fn f32_store_unaligned {
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
    }
}

impl_op! {
    fn f32_store_aligned {
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
    }
}
