use super::*;

impl_op! {
    fn i32_add {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            _mm256_add_epi32(a, b)
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            _mm_add_epi32(a, b)
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            _mm_add_epi32(a, b)
        }
        for Scalar(a: i32, b: i32) -> i32 {
            a + b
        }
    }
}

impl_op! {
    fn i32_sub {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            _mm256_sub_epi32(a, b)
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            _mm_sub_epi32(a, b)
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            _mm_sub_epi32(a, b)
        }
        for Scalar(a: i32, b: i32) -> i32 {
            a - b
        }
    }
}

impl_op! {
    fn i32_mul {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            _mm256_mullo_epi32(a, b)
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            _mm_mullo_epi32(a, b)
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            let a_arr = core::mem::transmute::<_, [i32; 4]>(a);
            let b_arr = core::mem::transmute::<_, [i32; 4]>(b);
            let c_arr = [
                a_arr[0] * b_arr[0],
                a_arr[1] * b_arr[1],
                a_arr[2] * b_arr[2],
                a_arr[3] * b_arr[3],
            ];
            core::mem::transmute::<_, __m128i>(c_arr)
        }
        for Scalar(a: i32, b: i32) -> i32 {
            a * b
        }
    }
}

impl_op! {
    fn i32_min {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            _mm256_min_epi32(a, b)
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            _mm_min_epi32(a, b)
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            let mask = _mm_cmpgt_epi32(a, b);
            _mm_or_si128(_mm_and_si128(mask, b), _mm_andnot_si128(mask, a))
        }
        for Scalar(a: i32, b: i32) -> i32 {
            a.min(b)
        }
    }
}

impl_op! {
    fn i32_max {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            _mm256_max_epi32(a, b)
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            _mm_max_epi32(a, b)
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            let mask = _mm_cmpgt_epi32(a, b);
            _mm_or_si128(_mm_and_si128(mask, a), _mm_andnot_si128(mask, b))
        }
        for Scalar(a: i32, b: i32) -> i32 {
            a.max(b)
        }
    }
}

impl_op! {
    fn i32_abs {
        for Avx2(a: __m256i) -> __m256i {
            _mm256_abs_epi32(a)
        }
        for Sse41(a: __m128i) -> __m128i {
            _mm_abs_epi32(a)
        }
        for Sse2(a: __m128i) -> __m128i {
            let mask = _mm_cmpgt_epi32(_mm_setzero_si128(), a);
            _mm_sub_epi32(_mm_xor_si128(a, mask), mask)
        }
        for Scalar(a: i32) -> i32 {
            a.abs()
        }
    }
}

impl_op! {
    fn i32_eq {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            _mm256_cmpeq_epi32(a, b)
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            _mm_cmpeq_epi32(a, b)
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            _mm_cmpeq_epi32(a, b)
        }
        for Scalar(a: i32, b: i32) -> i32 {
            if a == b {
                u32::MAX as i32
            } else {
                0
            }
        }
    }
}

impl_op! {
    fn i32_neq {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            let eq = _mm256_cmpeq_epi32(a, b);
            _mm256_xor_si256(eq, _mm256_set1_epi32(u32::MAX as i32))
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            let eq = _mm_cmpeq_epi32(a, b);
            _mm_xor_si128(eq, _mm_set1_epi32(u32::MAX as i32))
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            let eq = _mm_cmpeq_epi32(a, b);
            _mm_xor_si128(eq, _mm_set1_epi32(u32::MAX as i32))
        }
        for Scalar(a: i32, b: i32) -> i32 {
            if a != b {
                u32::MAX as i32
            } else {
                0
            }
        }
    }
}

impl_op! {
    fn i32_lt {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            let gt = _mm256_cmpgt_epi32(a, b);
            let eq = _mm256_cmpeq_epi32(a, b);
            _mm256_andnot_si256(_mm256_or_si256(gt, eq), _mm256_set1_epi32(u32::MAX as i32))
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            let gt = _mm_cmpgt_epi32(a, b);
            let eq = _mm_cmpeq_epi32(a, b);
            _mm_andnot_si128(_mm_or_si128(gt, eq), _mm_set1_epi32(u32::MAX as i32))
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            let gt = _mm_cmpgt_epi32(a, b);
            let eq = _mm_cmpeq_epi32(a, b);
            _mm_andnot_si128(_mm_or_si128(gt, eq), _mm_set1_epi32(u32::MAX as i32))
        }
        for Scalar(a: i32, b: i32) -> i32 {
            if a < b {
                u32::MAX as i32
            } else {
                0
            }
        }
    }
}

impl_op! {
    fn i32_lte {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            let gt = _mm256_cmpgt_epi32(a, b);
            _mm256_xor_si256(gt, _mm256_set1_epi32(u32::MAX as i32))
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            let gt = _mm_cmpgt_epi32(a, b);
            _mm_xor_si128(gt, _mm_set1_epi32(u32::MAX as i32))
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            let gt = _mm_cmpgt_epi32(a, b);
            _mm_xor_si128(gt, _mm_set1_epi32(u32::MAX as i32))
        }
        for Scalar(a: i32, b: i32) -> i32 {
            if a <= b {
                u32::MAX as i32
            } else {
                0
            }
        }
    }
}

impl_op! {
    fn i32_gt {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            _mm256_cmpgt_epi32(a, b)
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            _mm_cmpgt_epi32(a, b)
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            _mm_cmpgt_epi32(a, b)
        }
        for Scalar(a: i32, b: i32) -> i32 {
            if a > b {
                u32::MAX as i32
            } else {
                0
            }
        }
    }
}

impl_op! {
    fn i32_gte {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            let gt = _mm256_cmpgt_epi32(a, b);
            let eq = _mm256_cmpeq_epi32(a, b);
            _mm256_or_si256(gt, eq)
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            let gt = _mm_cmpgt_epi32(a, b);
            let eq = _mm_cmpeq_epi32(a, b);
            _mm_or_si128(gt, eq)
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            let gt = _mm_cmpgt_epi32(a, b);
            let eq = _mm_cmpeq_epi32(a, b);
            _mm_or_si128(gt, eq)
        }
        for Scalar(a: i32, b: i32) -> i32 {
            if a >= b {
                u32::MAX as i32
            } else {
                0
            }
        }
    }
}

impl_op! {
    fn i32_blendv {
        for Avx2(a: __m256i, b: __m256i, mask: __m256i) -> __m256i {
            _mm256_blendv_epi8(a, b, mask)
        }
        for Sse41(a: __m128i, b: __m128i, mask: __m128i) -> __m128i {
            _mm_blendv_epi8(a, b, mask)
        }
        for Sse2(a: __m128i, b: __m128i, mask: __m128i) -> __m128i {
            _mm_or_si128(_mm_and_si128(mask, b), _mm_andnot_si128(mask, a))
        }
        for Scalar(a: i32, b: i32, mask: i32) -> i32 {
            if mask == 0 {
                a
            } else {
                b
            }
        }
    }
}

impl_op! {
    fn i32_shl {
        for Avx2(a: __m256i, rhs: i32) -> __m256i {
            _mm256_sll_epi32(a, _mm_cvtsi32_si128(rhs))
        }
        for Sse41(a: __m128i, b: i32) -> __m128i {
            _mm_sll_epi32(a, _mm_cvtsi32_si128(b))
        }
        for Sse2(a: __m128i, b: i32) -> __m128i {
            _mm_sll_epi32(a, _mm_cvtsi32_si128(b))
        }
        for Scalar(a: i32, b: i32) -> i32 {
            a << b
        }
    }
}

impl_op! {
    fn i32_shr {
        for Avx2(a: __m256i, rhs: i32) -> __m256i {
            _mm256_srl_epi32(a, _mm_cvtsi32_si128(rhs))
        }
        for Sse41(a: __m128i, b: i32) -> __m128i {
            _mm_srl_epi32(a, _mm_cvtsi32_si128(b))
        }
        for Sse2(a: __m128i, b: i32) -> __m128i {
            _mm_srl_epi32(a, _mm_cvtsi32_si128(b))
        }
        for Scalar(a: i32, b: i32) -> i32 {
            a >> b
        }
    }
}

impl_op! {
    fn i32_cast_f32 {
        for Avx2(a: __m256i) -> __m256 {
            _mm256_cvtepi32_ps(a)
        }
        for Sse41(a: __m128i) -> __m128 {
            _mm_cvtepi32_ps(a)
        }
        for Sse2(a: __m128i) -> __m128 {
            _mm_cvtepi32_ps(a)
        }
        for Scalar(a: i32) -> f32 {
            a as f32
        }
    }
}

impl_op! {
    fn i32_bitcast_f32 {
        for Avx2(a: __m256i) -> __m256 {
            _mm256_castsi256_ps(a)
        }
        for Sse41(a: __m128i) -> __m128 {
            _mm_castsi128_ps(a)
        }
        for Sse2(a: __m128i) -> __m128 {
            _mm_castsi128_ps(a)
        }
        for Scalar(a: i32) -> f32 {
            f32::from_bits(a as u32)
        }
    }
}

impl_op! {
    fn i32_extend_i64 {
        for Avx2(val: __m256i) -> (__m256i, __m256i) {
            let a = _mm256_cvtepi32_epi64(_mm256_extracti128_si256(val, 0));
            let b = _mm256_cvtepi32_epi64(_mm256_extracti128_si256(val, 1));
            (a, b)
        }
        for Sse41(val: __m128i) -> (__m128i, __m128i) {
            let a = _mm_cvtepi32_epi64(val);
            let b = _mm_cvtepi32_epi64(_mm_shuffle_epi32(val, 0b11_10_11_10));
            (a, b)
        }
        for Sse2(val: __m128i) -> (__m128i, __m128i) {
            let arr = core::mem::transmute::<_, [i32; 4]>(val);
            let a = [arr[0] as i64, arr[1] as i64];
            let b = [arr[2] as i64, arr[3] as i64];
            (core::mem::transmute(a), core::mem::transmute(b))
        }
        for Scalar(val: i32) -> (i64, i64) {
            (val as i64, 0)
        }
    }
}

impl_op! {
    fn i32_zeroes {
        for Avx2() -> __m256i {
            _mm256_setzero_si256()
        }
        for Sse41() -> __m128i {
            _mm_setzero_si128()
        }
        for Sse2() -> __m128i {
            _mm_setzero_si128()
        }
        for Scalar() -> i32 {
            0
        }
    }
}

impl_op! {
    fn i32_set1 {
        for Avx2(val: i32) -> __m256i {
            _mm256_set1_epi32(val)
        }
        for Sse41(val: i32) -> __m128i {
            _mm_set1_epi32(val)
        }
        for Sse2(val: i32) -> __m128i {
            _mm_set1_epi32(val)
        }
        for Scalar(val: i32) -> i32 {
            val
        }
    }
}

impl_op! {
    fn i32_load_unaligned {
        for Avx2(ptr: *const i32) -> __m256i {
            _mm256_loadu_si256(ptr as *const __m256i)
        }
        for Sse41(ptr: *const i32) -> __m128i {
            _mm_loadu_si128(ptr as *const __m128i)
        }
        for Sse2(ptr: *const i32) -> __m128i {
            _mm_loadu_si128(ptr as *const __m128i)
        }
        for Scalar(ptr: *const i32) -> i32 {
            unsafe { *ptr }
        }
    }
}

impl_op! {
    fn i32_load_aligned {
        for Avx2(ptr: *const i32) -> __m256i {
            _mm256_load_si256(ptr as *const __m256i)
        }
        for Sse41(ptr: *const i32) -> __m128i {
            _mm_load_si128(ptr as *const __m128i)
        }
        for Sse2(ptr: *const i32) -> __m128i {
            _mm_load_si128(ptr as *const __m128i)
        }
        for Scalar(ptr: *const i32) -> i32 {
            unsafe { *ptr }
        }
    }
}

impl_op! {
    fn i32_store_unaligned {
        for Avx2(ptr: *mut i32, a: __m256i) {
            _mm256_storeu_si256(ptr as *mut __m256i, a)
        }
        for Sse41(ptr: *mut i32, a: __m128i) {
            _mm_storeu_si128(ptr as *mut __m128i, a)
        }
        for Sse2(ptr: *mut i32, a: __m128i) {
            _mm_storeu_si128(ptr as *mut __m128i, a)
        }
        for Scalar(ptr: *mut i32, a: i32) {
            unsafe { *ptr = a }
        }
    }
}

impl_op! {
    fn i32_store_aligned {
        for Avx2(ptr: *mut i32, a: __m256i) {
            _mm256_store_si256(ptr as *mut __m256i, a)
        }
        for Sse41(ptr: *mut i32, a: __m128i) {
            _mm_store_si128(ptr as *mut __m128i, a)
        }
        for Sse2(ptr: *mut i32, a: __m128i) {
            _mm_store_si128(ptr as *mut __m128i, a)
        }
        for Scalar(ptr: *mut i32, a: i32) {
            unsafe { *ptr = a }
        }
    }
}
