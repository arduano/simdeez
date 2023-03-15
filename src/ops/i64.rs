use super::*;

impl_op! {
    fn add<i64> {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            _mm256_add_epi64(a, b)
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            _mm_add_epi64(a, b)
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            _mm_add_epi64(a, b)
        }
        for Scalar(a: i64, b: i64) -> i64 {
            a + b
        }
    }
}

impl_op! {
    fn sub<i64> {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            _mm256_sub_epi64(a, b)
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            _mm_sub_epi64(a, b)
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            _mm_sub_epi64(a, b)
        }
        for Scalar(a: i64, b: i64) -> i64 {
            a - b
        }
    }
}

impl_op! {
    fn mul<i64> {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            let a_arr = core::mem::transmute::<_, [i64; 4]>(a);
            let b_arr = core::mem::transmute::<_, [i64; 4]>(b);
            let c_arr = [
                a_arr[0] * b_arr[0],
                a_arr[1] * b_arr[1],
                a_arr[2] * b_arr[2],
                a_arr[3] * b_arr[3],
            ];
            core::mem::transmute::<_, __m256i>(c_arr)
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            let a_arr = core::mem::transmute::<_, [i64; 2]>(a);
            let b_arr = core::mem::transmute::<_, [i64; 2]>(b);
            let c_arr = [
                a_arr[0] * b_arr[0],
                a_arr[1] * b_arr[1],
            ];
            core::mem::transmute::<_, __m128i>(c_arr)
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            let a_arr = core::mem::transmute::<_, [i64; 2]>(a);
            let b_arr = core::mem::transmute::<_, [i64; 2]>(b);
            let c_arr = [
                a_arr[0] * b_arr[0],
                a_arr[1] * b_arr[1],
            ];
            core::mem::transmute::<_, __m128i>(c_arr)
        }
        for Scalar(a: i64, b: i64) -> i64 {
            a * b
        }
    }
}

impl_op! {
    fn min<i64> {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            let mask = _mm256_cmpgt_epi64(a, b);
            _mm256_or_si256(_mm256_and_si256(mask, b), _mm256_andnot_si256(mask, a))
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            let mask = _mm_cmpgt_epi64(a, b);
            _mm_or_si128(_mm_and_si128(mask, b), _mm_andnot_si128(mask, a))
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            let mask = _mm_cmpgt_epi64(a, b);
            _mm_or_si128(_mm_and_si128(mask, b), _mm_andnot_si128(mask, a))
        }
        for Scalar(a: i64, b: i64) -> i64 {
            a.min(b)
        }
    }
}

impl_op! {
    fn max<i64> {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            let mask = _mm256_cmpgt_epi64(a, b);
            _mm256_or_si256(_mm256_and_si256(mask, a), _mm256_andnot_si256(mask, b))
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            let mask = _mm_cmpgt_epi64(a, b);
            _mm_or_si128(_mm_and_si128(mask, a), _mm_andnot_si128(mask, b))
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            let mask = _mm_cmpgt_epi64(a, b);
            _mm_or_si128(_mm_and_si128(mask, a), _mm_andnot_si128(mask, b))
        }
        for Scalar(a: i64, b: i64) -> i64 {
            a.max(b)
        }
    }
}

impl_op! {
    fn abs<i64> {
        for Avx2(a: __m256i) -> __m256i {
            let mask = _mm256_cmpgt_epi64(_mm256_setzero_si256(), a);
            _mm256_sub_epi64(_mm256_xor_si256(a, mask), mask)
        }
        for Sse41(a: __m128i) -> __m128i {
            let mask = _mm_cmpgt_epi64(_mm_setzero_si128(), a);
            _mm_sub_epi64(_mm_xor_si128(a, mask), mask)
        }
        for Sse2(a: __m128i) -> __m128i {
            let mask = _mm_cmpgt_epi64(_mm_setzero_si128(), a);
            _mm_sub_epi64(_mm_xor_si128(a, mask), mask)
        }
        for Scalar(a: i64) -> i64 {
            a.abs()
        }
    }
}

impl_op! {
    fn eq<i64> {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            _mm256_cmpeq_epi64(a, b)
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            _mm_cmpeq_epi64(a, b)
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            _mm_cmpeq_epi64(a, b)
        }
        for Scalar(a: i64, b: i64) -> i64 {
            if a == b {
                u64::MAX as i64
            } else {
                0
            }
        }
    }
}

impl_op! {
    fn neq<i64> {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            let eq = _mm256_cmpeq_epi64(a, b);
            _mm256_xor_si256(eq, _mm256_set1_epi64x(u64::MAX as i64))
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            let eq = _mm_cmpeq_epi64(a, b);
            _mm_xor_si128(eq, _mm_set1_epi64x(u64::MAX as i64))
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            let eq = _mm_cmpeq_epi64(a, b);
            _mm_xor_si128(eq, _mm_set1_epi64x(u64::MAX as i64))
        }
        for Scalar(a: i64, b: i64) -> i64 {
            if a != b {
                u64::MAX as i64
            } else {
                0
            }
        }
    }
}

impl_op! {
    fn lt<i64> {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            let gt = _mm256_cmpgt_epi64(a, b);
            let eq = _mm256_cmpeq_epi64(a, b);
            _mm256_andnot_si256(_mm256_or_si256(gt, eq), _mm256_set1_epi64x(u64::MAX as i64))
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            let gt = _mm_cmpgt_epi64(a, b);
            let eq = _mm_cmpeq_epi64(a, b);
            _mm_andnot_si128(_mm_or_si128(gt, eq), _mm_set1_epi64x(u64::MAX as i64))
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            let gt = _mm_cmpgt_epi64(a, b);
            let eq = _mm_cmpeq_epi64(a, b);
            _mm_andnot_si128(_mm_or_si128(gt, eq), _mm_set1_epi64x(u64::MAX as i64))
        }
        for Scalar(a: i64, b: i64) -> i64 {
            if a < b {
                u64::MAX as i64
            } else {
                0
            }
        }
    }
}

impl_op! {
    fn lte<i64> {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            let gt = _mm256_cmpgt_epi64(a, b);
            _mm256_xor_si256(gt, _mm256_set1_epi64x(u64::MAX as i64))
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            let gt = _mm_cmpgt_epi64(a, b);
            _mm_xor_si128(gt, _mm_set1_epi64x(u64::MAX as i64))
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            let gt = _mm_cmpgt_epi64(a, b);
            _mm_xor_si128(gt, _mm_set1_epi64x(u64::MAX as i64))
        }
        for Scalar(a: i64, b: i64) -> i64 {
            if a <= b {
                u64::MAX as i64
            } else {
                0
            }
        }
    }
}

impl_op! {
    fn gt<i64> {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            _mm256_cmpgt_epi64(a, b)
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            _mm_cmpgt_epi64(a, b)
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            _mm_cmpgt_epi64(a, b)
        }
        for Scalar(a: i64, b: i64) -> i64 {
            if a > b {
                u64::MAX as i64
            } else {
                0
            }
        }
    }
}

impl_op! {
    fn gte<i64> {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            let gt = _mm256_cmpgt_epi64(a, b);
            let eq = _mm256_cmpeq_epi64(a, b);
            _mm256_or_si256(gt, eq)
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            let gt = _mm_cmpgt_epi64(a, b);
            let eq = _mm_cmpeq_epi64(a, b);
            _mm_or_si128(gt, eq)
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            let gt = _mm_cmpgt_epi64(a, b);
            let eq = _mm_cmpeq_epi64(a, b);
            _mm_or_si128(gt, eq)
        }
        for Scalar(a: i64, b: i64) -> i64 {
            if a >= b {
                u64::MAX as i64
            } else {
                0
            }
        }
    }
}

impl_op! {
    fn blendv<i64> {
        for Avx2(a: __m256i, b: __m256i, mask: __m256i) -> __m256i {
            _mm256_blendv_epi8(a, b, mask)
        }
        for Sse41(a: __m128i, b: __m128i, mask: __m128i) -> __m128i {
            _mm_blendv_epi8(a, b, mask)
        }
        for Sse2(a: __m128i, b: __m128i, mask: __m128i) -> __m128i {
            _mm_or_si128(_mm_and_si128(mask, b), _mm_andnot_si128(mask, a))
        }
        for Scalar(a: i64, b: i64, mask: i64) -> i64 {
            if mask == 0 {
                a
            } else {
                b
            }
        }
    }
}

impl_op! {
    fn shl<i64> {
        for Avx2(a: __m256i, rhs: i32) -> __m256i {
            _mm256_sll_epi64(a, _mm_cvtsi32_si128(rhs))
        }
        for Sse41(a: __m128i, b: i32) -> __m128i {
            _mm_sll_epi64(a, _mm_cvtsi32_si128(b))
        }
        for Sse2(a: __m128i, b: i32) -> __m128i {
            _mm_sll_epi64(a, _mm_cvtsi32_si128(b))
        }
        for Scalar(a: i64, b: i32) -> i64 {
            a << b
        }
    }
}

impl_op! {
    fn shr<i64> {
        for Avx2(a: __m256i, rhs: i32) -> __m256i {
            _mm256_srl_epi64(a, _mm_cvtsi32_si128(rhs))
        }
        for Sse41(a: __m128i, rhs: i32) -> __m128i {
            _mm_srl_epi64(a, _mm_cvtsi32_si128(rhs))
        }
        for Sse2(a: __m128i, rhs: i32) -> __m128i {
            _mm_srl_epi64(a, _mm_cvtsi32_si128(rhs))
        }
        for Scalar(a: i64, rhs: i32) -> i64 {
            ((a as u64) >> rhs) as i64
        }
    }
}

impl_imm8_op! {
    fn shl_const<i64, const BY: i32> {
        for Avx2(a: __m256i) -> __m256i {
            _mm256_slli_epi64(a, BY)
        }
        for Sse41(a: __m128i) -> __m128i {
            _mm_slli_epi64(a, BY)
        }
        for Sse2(a: __m128i) -> __m128i {
            _mm_slli_epi64(a, BY)
        }
        for Scalar(a: i64) -> i64 {
            a << BY
        }
    }
}

impl_imm8_op! {
    fn shr_const<i64, const BY: i32> {
        for Avx2(a: __m256i) -> __m256i {
            _mm256_srli_epi64(a, BY)
        }
        for Sse41(a: __m128i) -> __m128i {
            _mm_srli_epi64(a, BY)
        }
        for Sse2(a: __m128i) -> __m128i {
            _mm_srli_epi64(a, BY)
        }
        for Scalar(a: i64) -> i64 {
            ((a as u64) >> BY) as i64
        }
    }
}

impl_op! {
    fn cast_f64<i64> {
        for Avx2(a: __m256i) -> __m256d {
            let arr = core::mem::transmute::<_, [i64; 4]>(a);
            let result = [
                arr[0] as f64,
                arr[1] as f64,
                arr[2] as f64,
                arr[3] as f64,
            ];
            core::mem::transmute::<_, __m256d>(result)
        }
        for Sse41(a: __m128i) -> __m128d {
            let arr = core::mem::transmute::<_, [i64; 2]>(a);
            let result = [
                arr[0] as f64,
                arr[1] as f64,
            ];
            core::mem::transmute::<_, __m128d>(result)
        }
        for Sse2(a: __m128i) -> __m128d {
            let arr = core::mem::transmute::<_, [i64; 2]>(a);
            let result = [
                arr[0] as f64,
                arr[1] as f64,
            ];
            core::mem::transmute::<_, __m128d>(result)
        }
        for Scalar(a: i64) -> f64 {
            a as f64
        }
    }
}

impl_op! {
    fn bitcast_f64<i64> {
        for Avx2(a: __m256i) -> __m256d {
            _mm256_castsi256_pd(a)
        }
        for Sse41(a: __m128i) -> __m128d {
            _mm_castsi128_pd(a)
        }
        for Sse2(a: __m128i) -> __m128d {
            _mm_castsi128_pd(a)
        }
        for Scalar(a: i64) -> f64 {
            f64::from_bits(a as u64)
        }
    }
}

impl_op! {
    fn horizontal_add<i64> {
        for Avx2(val: __m256i) -> i64 {
            let a = val;
            let b = _mm256_permute4x64_epi64(a, 0b00_01_10_11); // Shuffle [0, 1, 2, 3]
            let c = _mm256_add_epi64(a, b);
            let val1 = _mm256_extract_epi64(c, 0);
            let val2 = _mm256_extract_epi64(c, 1);
            val1.wrapping_add(val2)
        }
        for Sse41(val: __m128i) -> i64 {
            let first = _mm_cvtsi128_si64(val);
            let second = _mm_cvtsi128_si64(_mm_shuffle_epi32(val, 0b_01_00_11_10));
            first.wrapping_add(second)
        }
        for Sse2(val: __m128i) -> i64 {
            let first = _mm_cvtsi128_si64(val);
            let second = _mm_cvtsi128_si64(_mm_shuffle_epi32(val, 0b_01_00_11_10));
            first.wrapping_add(second)
        }
        for Scalar(val: i64) -> i64 {
            val
        }
    }
}

impl_op! {
    fn zeroes<i64> {
        for Avx2() -> __m256i {
            _mm256_setzero_si256()
        }
        for Sse41() -> __m128i {
            _mm_setzero_si128()
        }
        for Sse2() -> __m128i {
            _mm_setzero_si128()
        }
        for Scalar() -> i64 {
            0
        }
    }
}

impl_op! {
    fn set1<i64> {
        for Avx2(val: i64) -> __m256i {
            _mm256_set1_epi64x(val)
        }
        for Sse41(val: i64) -> __m128i {
            _mm_set1_epi64x(val)
        }
        for Sse2(val: i64) -> __m128i {
            _mm_set1_epi64x(val)
        }
        for Scalar(val: i64) -> i64 {
            val
        }
    }
}

impl_op! {
    fn load_unaligned<i64> {
        for Avx2(ptr: *const i64) -> __m256i {
            _mm256_loadu_si256(ptr as *const __m256i)
        }
        for Sse41(ptr: *const i64) -> __m128i {
            _mm_loadu_si128(ptr as *const __m128i)
        }
        for Sse2(ptr: *const i64) -> __m128i {
            _mm_loadu_si128(ptr as *const __m128i)
        }
        for Scalar(ptr: *const i64) -> i64 {
            unsafe { *ptr }
        }
    }
}

impl_op! {
    fn load_aligned<i64> {
        for Avx2(ptr: *const i64) -> __m256i {
            _mm256_load_si256(ptr as *const __m256i)
        }
        for Sse41(ptr: *const i64) -> __m128i {
            _mm_load_si128(ptr as *const __m128i)
        }
        for Sse2(ptr: *const i64) -> __m128i {
            _mm_load_si128(ptr as *const __m128i)
        }
        for Scalar(ptr: *const i64) -> i64 {
            unsafe { *ptr }
        }
    }
}

impl_op! {
    fn store_unaligned<i64> {
        for Avx2(ptr: *mut i64, a: __m256i) {
            _mm256_storeu_si256(ptr as *mut __m256i, a)
        }
        for Sse41(ptr: *mut i64, a: __m128i) {
            _mm_storeu_si128(ptr as *mut __m128i, a)
        }
        for Sse2(ptr: *mut i64, a: __m128i) {
            _mm_storeu_si128(ptr as *mut __m128i, a)
        }
        for Scalar(ptr: *mut i64, a: i64) {
            unsafe { *ptr = a }
        }
    }
}

impl_op! {
    fn store_aligned<i64> {
        for Avx2(ptr: *mut i64, a: __m256i) {
            _mm256_store_si256(ptr as *mut __m256i, a)
        }
        for Sse41(ptr: *mut i64, a: __m128i) {
            _mm_store_si128(ptr as *mut __m128i, a)
        }
        for Sse2(ptr: *mut i64, a: __m128i) {
            _mm_store_si128(ptr as *mut __m128i, a)
        }
        for Scalar(ptr: *mut i64, a: i64) {
            unsafe { *ptr = a }
        }
    }
}
