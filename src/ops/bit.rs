use super::*;

impl_op! {
    fn bit_and<binary> {
        for Avx2(a: __m256, b: __m256) -> __m256 {
            _mm256_and_ps(a, b)
        }
        for Sse41(a: __m128, b: __m128) -> __m128 {
            _mm_and_ps(a, b)
        }
        for Sse2(a: __m128, b: __m128) -> __m128 {
            _mm_and_ps(a, b)
        }
        for Scalar(a: u64, b: u64) -> u64 {
            a & b
        }
        for Neon(a: int8x16_t, b: int8x16_t) -> int8x16_t {
            vandq_s8(a, b)
        }
        for Wasm(a: v128, b: v128) -> v128 {
            v128_and(a, b)
        }
    }
}

impl_op! {
    fn bit_or<binary> {
        for Avx2(a: __m256, b: __m256) -> __m256 {
            _mm256_or_ps(a, b)
        }
        for Sse41(a: __m128, b: __m128) -> __m128 {
            _mm_or_ps(a, b)
        }
        for Sse2(a: __m128, b: __m128) -> __m128 {
            _mm_or_ps(a, b)
        }
        for Scalar(a: u64, b: u64) -> u64 {
            a | b
        }
        for Neon(a: int8x16_t, b: int8x16_t) -> int8x16_t {
            vorrq_s8(a, b)
        }
        for Wasm(a: v128, b: v128) -> v128 {
            v128_or(a, b)
        }
    }
}

impl_op! {
    fn bit_xor<binary> {
        for Avx2(a: __m256, b: __m256) -> __m256 {
            _mm256_xor_ps(a, b)
        }
        for Sse41(a: __m128, b: __m128) -> __m128 {
            _mm_xor_ps(a, b)
        }
        for Sse2(a: __m128, b: __m128) -> __m128 {
            _mm_xor_ps(a, b)
        }
        for Scalar(a: u64, b: u64) -> u64 {
            a ^ b
        }
        for Neon(a: int8x16_t, b: int8x16_t) -> int8x16_t {
            veorq_s8(a, b)
        }
        for Wasm(a: v128, b: v128) -> v128 {
            v128_xor(a, b)
        }
    }
}

impl_op! {
    fn bit_not<binary> {
        for Avx2(a: __m256) -> __m256 {
            let all1 = _mm256_set1_epi32(-1);
            _mm256_castsi256_ps(_mm256_xor_si256(_mm256_castps_si256(a), all1))
        }
        for Sse41(a: __m128) -> __m128 {
            let all1 = _mm_set1_epi32(-1);
            _mm_castsi128_ps(_mm_xor_si128(_mm_castps_si128(a), all1))
        }
        for Sse2(a: __m128) -> __m128 {
            let all1 = _mm_set1_epi32(-1);
            _mm_castsi128_ps(_mm_xor_si128(_mm_castps_si128(a), all1))
        }
        for Scalar(a: u64) -> u64 {
            !a
        }
        for Neon(a: int8x16_t) -> int8x16_t {
            vreinterpretq_s8_u8(vmvnq_u8(vreinterpretq_u8_s8(a)))
        }
        for Wasm(a: v128) -> v128 {
            v128_not(a)
        }
    }
}

impl_op! {
    fn bit_andnot<binary> {
        for Avx2(a: __m256, b: __m256) -> __m256 {
            _mm256_andnot_ps(a, b)
        }
        for Sse41(a: __m128, b: __m128) -> __m128 {
            _mm_andnot_ps(a, b)
        }
        for Sse2(a: __m128, b: __m128) -> __m128 {
            _mm_andnot_ps(a, b)
        }
        for Scalar(a: u64, b: u64) -> u64 {
            !a & b
        }
        for Neon(a: int8x16_t, b: int8x16_t) -> int8x16_t {
            vandq_s8(vreinterpretq_s8_u8(vmvnq_u8(vreinterpretq_u8_s8(a))), b)
        }
        for Wasm(a: v128, b: v128) -> v128 {
            v128_andnot(b, a)
        }
    }
}
