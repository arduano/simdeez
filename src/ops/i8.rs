use super::*;

impl_op! {
    fn add<i8> {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            _mm256_add_epi8(a, b)
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            _mm_add_epi8(a, b)
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            _mm_add_epi8(a, b)
        }
        for Scalar(a: i8, b: i8) -> i8 {
            a.wrapping_add(b)
        }
        for Neon(a: int8x16_t, b: int8x16_t) -> int8x16_t {
            vaddq_s8(a, b)
        }
        for Wasm(a: v128, b: v128) -> v128 {
            i8x16_add(a, b)
        }
    }
}

impl_op! {
    fn sub<i8> {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            _mm256_sub_epi8(a, b)
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            _mm_sub_epi8(a, b)
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            _mm_sub_epi8(a, b)
        }
        for Scalar(a: i8, b: i8) -> i8 {
            a.wrapping_sub(b)
        }
        for Neon(a: int8x16_t, b: int8x16_t) -> int8x16_t {
            vsubq_s8(a, b)
        }
        for Wasm(a: v128, b: v128) -> v128 {
            i8x16_sub(a, b)
        }
    }
}

impl_op! {
    fn mul<i8> {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            let mut arr1 = core::mem::transmute::<__m256i, [i8; 32]>(a);
            let arr2 = core::mem::transmute::<__m256i, [i8; 32]>(b);
            for i in 0..32 {
                arr1[i] = arr1[i].wrapping_mul(arr2[i]);
            }
            core::mem::transmute::<_, _>(arr1)
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            let mut arr1 = core::mem::transmute::<__m128i, [i8; 16]>(a);
            let arr2 = core::mem::transmute::<__m128i, [i8; 16]>(b);
            for i in 0..16 {
                arr1[i] = arr1[i].wrapping_mul(arr2[i]);
            }
            core::mem::transmute::<_, _>(arr1)
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            let mut arr1 = core::mem::transmute::<__m128i, [i8; 16]>(a);
            let arr2 = core::mem::transmute::<__m128i, [i8; 16]>(b);
            for i in 0..16 {
                arr1[i] = arr1[i].wrapping_mul(arr2[i]);
            }
            core::mem::transmute::<_, _>(arr1)
        }
        for Scalar(a: i8, b: i8) -> i8 {
            a.wrapping_mul(b)
        }
        for Neon(a: int8x16_t, b: int8x16_t) -> int8x16_t {
            vmulq_s8(a, b)
        }
        for Wasm(a: v128, b: v128) -> v128 {
            let mut arr1 = core::mem::transmute::<_, [i8; 16]>(a);
            let arr2 = core::mem::transmute::<_, [i8; 16]>(b);
            for i in 0..16 {
                arr1[i] = arr1[i].wrapping_mul(arr2[i]);
            }
            core::mem::transmute::<_, _>(arr1)
        }
    }
}

impl_op! {
    fn min<i8> {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            _mm256_min_epi8(a, b)
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            _mm_min_epi8(a, b)
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            let mask = _mm_cmpgt_epi8(a, b);
            _mm_or_si128(_mm_and_si128(mask, b), _mm_andnot_si128(mask, a))
        }
        for Scalar(a: i8, b: i8) -> i8 {
            a.min(b)
        }
        for Neon(a: int8x16_t, b: int8x16_t) -> int8x16_t {
            vminq_s8(a, b)
        }
        for Wasm(a: v128, b: v128) -> v128 {
            i8x16_min(a, b)
        }
    }
}

impl_op! {
    fn max<i8> {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            _mm256_max_epi8(a, b)
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            _mm_max_epi8(a, b)
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            let mask = _mm_cmpgt_epi8(a, b);
            _mm_or_si128(_mm_and_si128(mask, a), _mm_andnot_si128(mask, b))
        }
        for Scalar(a: i8, b: i8) -> i8 {
            a.max(b)
        }
        for Neon(a: int8x16_t, b: int8x16_t) -> int8x16_t {
            vmaxq_s8(a, b)
        }
        for Wasm(a: v128, b: v128) -> v128 {
            i8x16_max(a, b)
        }
    }
}

impl_op! {
    fn abs<i8> {
        for Avx2(a: __m256i) -> __m256i {
            _mm256_abs_epi8(a)
        }
        for Sse41(a: __m128i) -> __m128i {
            _mm_abs_epi8(a)
        }
        for Sse2(a: __m128i) -> __m128i {
            let mask = _mm_cmpgt_epi8(_mm_setzero_si128(), a);
            _mm_sub_epi8(_mm_xor_si128(a, mask), mask)
        }
        for Scalar(a: i8) -> i8 {
            a.abs()
        }
        for Neon(a: int8x16_t) -> int8x16_t {
            vabsq_s8(a)
        }
        for Wasm(a: v128) -> v128 {
            i8x16_abs(a)
        }
    }
}

impl_op! {
    fn eq<i8> {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            _mm256_cmpeq_epi8(a, b)
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            _mm_cmpeq_epi8(a, b)
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            _mm_cmpeq_epi8(a, b)
        }
        for Scalar(a: i8, b: i8) -> i8 {
            if a == b {
                u32::MAX as i8
            } else {
                0
            }
        }
        for Neon(a: int8x16_t, b: int8x16_t) -> int8x16_t {
            vreinterpretq_s8_u8(vceqq_s8(a, b))
        }
        for Wasm(a: v128, b: v128) -> v128 {
            i8x16_eq(a, b)
        }
    }
}

impl_op! {
    fn neq<i8> {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            let eq = _mm256_cmpeq_epi8(a, b);
            _mm256_xor_si256(eq, _mm256_set1_epi8(u32::MAX as i8))
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            let eq = _mm_cmpeq_epi8(a, b);
            _mm_xor_si128(eq, _mm_set1_epi8(u32::MAX as i8))
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            let eq = _mm_cmpeq_epi8(a, b);
            _mm_xor_si128(eq, _mm_set1_epi8(u32::MAX as i8))
        }
        for Scalar(a: i8, b: i8) -> i8 {
            if a != b {
                u32::MAX as i8
            } else {
                0
            }
        }
        for Neon(a: int8x16_t, b: int8x16_t) -> int8x16_t {
            vreinterpretq_s8_u8(vmvnq_u8(vceqq_s8(a, b)))
        }
        for Wasm(a: v128, b: v128) -> v128 {
            i8x16_ne(a, b)
        }
    }
}

impl_op! {
    fn lt<i8> {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            let gt = _mm256_cmpgt_epi8(a, b);
            let eq = _mm256_cmpeq_epi8(a, b);
            _mm256_andnot_si256(_mm256_or_si256(gt, eq), _mm256_set1_epi8(u32::MAX as i8))
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            let gt = _mm_cmpgt_epi8(a, b);
            let eq = _mm_cmpeq_epi8(a, b);
            _mm_andnot_si128(_mm_or_si128(gt, eq), _mm_set1_epi8(u32::MAX as i8))
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            let gt = _mm_cmpgt_epi8(a, b);
            let eq = _mm_cmpeq_epi8(a, b);
            _mm_andnot_si128(_mm_or_si128(gt, eq), _mm_set1_epi8(u32::MAX as i8))
        }
        for Scalar(a: i8, b: i8) -> i8 {
            if a < b {
                u32::MAX as i8
            } else {
                0
            }
        }
        for Neon(a: int8x16_t, b: int8x16_t) -> int8x16_t {
            vreinterpretq_s8_u8(vcltq_s8(a, b))
        }
        for Wasm(a: v128, b: v128) -> v128 {
            i8x16_lt(a, b)
        }
    }
}

impl_op! {
    fn lte<i8> {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            let gt = _mm256_cmpgt_epi8(a, b);
            _mm256_xor_si256(gt, _mm256_set1_epi8(u32::MAX as i8))
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            let gt = _mm_cmpgt_epi8(a, b);
            _mm_xor_si128(gt, _mm_set1_epi8(u32::MAX as i8))
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            let gt = _mm_cmpgt_epi8(a, b);
            _mm_xor_si128(gt, _mm_set1_epi8(u32::MAX as i8))
        }
        for Scalar(a: i8, b: i8) -> i8 {
            if a <= b {
                u32::MAX as i8
            } else {
                0
            }
        }
        for Neon(a: int8x16_t, b: int8x16_t) -> int8x16_t {
            vreinterpretq_s8_u8(vcleq_s8(a, b))
        }
        for Wasm(a: v128, b: v128) -> v128 {
            i8x16_le(a, b)
        }
    }
}

impl_op! {
    fn gt<i8> {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            _mm256_cmpgt_epi8(a, b)
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            _mm_cmpgt_epi8(a, b)
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            _mm_cmpgt_epi8(a, b)
        }
        for Scalar(a: i8, b: i8) -> i8 {
            if a > b {
                u32::MAX as i8
            } else {
                0
            }
        }
        for Neon(a: int8x16_t, b: int8x16_t) -> int8x16_t {
            vreinterpretq_s8_u8(vcgtq_s8(a, b))
        }
        for Wasm(a: v128, b: v128) -> v128 {
            i8x16_gt(a, b)
        }
    }
}

impl_op! {
    fn gte<i8> {
        for Avx2(a: __m256i, b: __m256i) -> __m256i {
            let gt = _mm256_cmpgt_epi8(a, b);
            let eq = _mm256_cmpeq_epi8(a, b);
            _mm256_or_si256(gt, eq)
        }
        for Sse41(a: __m128i, b: __m128i) -> __m128i {
            let gt = _mm_cmpgt_epi8(a, b);
            let eq = _mm_cmpeq_epi8(a, b);
            _mm_or_si128(gt, eq)
        }
        for Sse2(a: __m128i, b: __m128i) -> __m128i {
            let gt = _mm_cmpgt_epi8(a, b);
            let eq = _mm_cmpeq_epi8(a, b);
            _mm_or_si128(gt, eq)
        }
        for Scalar(a: i8, b: i8) -> i8 {
            if a >= b {
                u32::MAX as i8
            } else {
                0
            }
        }
        for Neon(a: int8x16_t, b: int8x16_t) -> int8x16_t {
            vreinterpretq_s8_u8(vcgeq_s8(a, b))
        }
        for Wasm(a: v128, b: v128) -> v128 {
            i8x16_ge(a, b)
        }
    }
}

impl_op! {
    fn blendv<i8> {
        for Avx2(a: __m256i, b: __m256i, mask: __m256i) -> __m256i {
            _mm256_blendv_epi8(a, b, mask)
        }
        for Sse41(a: __m128i, b: __m128i, mask: __m128i) -> __m128i {
            _mm_blendv_epi8(a, b, mask)
        }
        for Sse2(a: __m128i, b: __m128i, mask: __m128i) -> __m128i {
            _mm_or_si128(_mm_and_si128(mask, b), _mm_andnot_si128(mask, a))
        }
        for Scalar(a: i8, b: i8, mask: i8) -> i8 {
            if mask == 0 {
                a
            } else {
                b
            }
        }
        for Neon(a: int8x16_t, b: int8x16_t, mask: int8x16_t) -> int8x16_t {
            vbslq_s8(vreinterpretq_u8_s8(mask), b, a)
        }
        for Wasm(a: v128, b: v128, mask: v128) -> v128 {
            v128_or(v128_and(mask, b), v128_andnot(a, mask))
        }
    }
}

impl_op! {
    fn shl<i8> {
        for Avx2(a: __m256i, rhs: i32) -> __m256i {
            // Do 16 bit shifts, then mask out the bits that are shifted in.
            let rhs2 = _mm_cvtsi32_si128(rhs);
            let shifted_i16 = _mm256_sll_epi16(a, rhs2);

            let mask = 0x00FFu16 >> (8 - rhs) << 8;
            let mask = _mm256_set1_epi16(mask as i16);
            _mm256_andnot_si256(mask, shifted_i16)
        }
        for Sse41(a: __m128i, rhs: i32) -> __m128i {
            Ops::<Sse2, i8>::shl(a, rhs)
        }
        for Sse2(a: __m128i, rhs: i32) -> __m128i {
            // Do 16 bit shifts, then mask out the bits that are shifted in.
            let rhs2 = _mm_cvtsi32_si128(rhs);
            let shifted_i16 = _mm_sll_epi16(a, rhs2);

            let mask = 0x00FFu16 >> (8 - rhs) << 8;
            let mask = _mm_set1_epi16(mask as i16);
            _mm_andnot_si128(mask, shifted_i16)
        }
        for Scalar(a: i8, rhs: i32) -> i8 {
            a << rhs
        }
        for Neon(a: int8x16_t, rhs: i32) -> int8x16_t {
            let rhs = Self::set1(rhs as i8);
            vshlq_s8(a, rhs)
        }
        for Wasm(a: v128, b: i32) -> v128 {
            i8x16_shl(a, b as u32)
        }
    }
}

impl_op! {
    fn shr<i8> {
        for Avx2(a: __m256i, rhs: i32) -> __m256i {
            // Do 16 bit shifts, then mask out the bits that are shifted in.
            let rhs2 = _mm_cvtsi32_si128(rhs);
            let shifted_i16 = _mm256_srl_epi16(a, rhs2);

            let mask = 0xFF00u16 << (8 - rhs) >> 8;
            let mask = _mm256_set1_epi16(mask as i16);
            _mm256_andnot_si256(mask, shifted_i16)
        }
        for Sse41(a: __m128i, rhs: i32) -> __m128i {
            Ops::<Sse2, i8>::shr(a, rhs)
        }
        for Sse2(a: __m128i, rhs: i32) -> __m128i {
            // Do 16 bit shifts, then mask out the bits that are shifted in.
            let rhs2 = _mm_cvtsi32_si128(rhs);
            let shifted_i16 = _mm_srl_epi16(a, rhs2);

            let mask = 0xFF00u16 << (8 - rhs) >> 8;
            let mask = _mm_set1_epi16(mask as i16);
            _mm_andnot_si128(mask, shifted_i16)
        }
        for Scalar(a: i8, rhs: i32) -> i8 {
            ((a as u8) >> rhs) as i8
        }
        for Neon(a: int8x16_t, rhs: i32) -> int8x16_t {
            let rhs = Self::set1(-rhs as i8);
            vreinterpretq_s8_u8(vshlq_u8(vreinterpretq_u8_s8(a), rhs))
        }
        for Wasm(a: v128, b: i32) -> v128 {
            u8x16_shr(a, b as u32)
        }
    }
}

impl_imm8_op! {
    fn shl_const<i8, const BY: i32> {
        for Avx2(a: __m256i) -> __m256i {
            Self::shl(a, BY)
        }
        for Sse41(a: __m128i) -> __m128i {
            Self::shl(a, BY)
        }
        for Sse2(a: __m128i) -> __m128i {
            Self::shl(a, BY)
        }
        for Scalar(a: i8) -> i8 {
            a << BY
        }
        for Neon(a: int8x16_t) -> int8x16_t {
            vshlq_n_s8(a, BY)
        }
        for Wasm(a: v128) -> v128 {
            Self::shl(a, BY)
        }
    }
}

impl_imm8_op! {
    fn shr_const<i8, const BY: i32> {
        for Avx2(a: __m256i) -> __m256i {
            Self::shr(a, BY)
        }
        for Sse41(a: __m128i) -> __m128i {
            Self::shr(a, BY)
        }
        for Sse2(a: __m128i) -> __m128i {
            Self::shr(a, BY)
        }
        for Scalar(a: i8) -> i8 {
            ((a as u8) >> BY) as i8
        }
        for Neon(a: int8x16_t) -> int8x16_t {
            vreinterpretq_s8_u8(vshrq_n_u8(vreinterpretq_u8_s8(a), BY))
        }
        for Wasm(a: v128) -> v128 {
            Self::shr(a, BY)
        }
    }
}

impl_op! {
    fn extend_i16<i8> {
        for Avx2(val: __m256i) -> (__m256i, __m256i) {
            let a = _mm256_cvtepi8_epi16(_mm256_extracti128_si256(val, 0));
            let b = _mm256_cvtepi8_epi16(_mm256_extracti128_si256(val, 1));
            (a, b)
        }
        for Sse41(val: __m128i) -> (__m128i, __m128i) {
            let a = _mm_cvtepi8_epi16(val);
            let b = _mm_cvtepi8_epi16(_mm_shuffle_epi32(val, 0b_01_00_11_10));
            (a, b)
        }
        for Sse2(val: __m128i) -> (__m128i, __m128i) {
            let arr = core::mem::transmute::<__m128i, [i8; 16]>(val);
            let a = [
                arr[0] as i16,
                arr[1] as i16,
                arr[2] as i16,
                arr[3] as i16,
                arr[4] as i16,
                arr[5] as i16,
                arr[6] as i16,
                arr[7] as i16,
            ];
            let b = [
                arr[8] as i16,
                arr[9] as i16,
                arr[10] as i16,
                arr[11] as i16,
                arr[12] as i16,
                arr[13] as i16,
                arr[14] as i16,
                arr[15] as i16,
            ];
            (core::mem::transmute::<[i16; 8], __m128i>(a), core::mem::transmute::<[i16; 8], __m128i>(b))
        }
        for Scalar(val: i8) -> (i16, i16) {
            (val as i16, 0)
        }
        for Neon(val: int8x16_t) -> (int16x8_t, int16x8_t) {
            let a = vmovl_s8(vget_low_s8(val));
            let b = vmovl_s8(vget_high_s8(val));
            (a, b)
        }
        for Wasm(val: v128) -> (v128, v128) {
            let a = i16x8_extend_low_i8x16(val);
            let b = i16x8_extend_high_i8x16(val);
            (a, b)
        }
    }
}

impl_op! {
    fn unsigned_extend_i16<i8> {
        for Avx2(val: __m256i) -> (__m256i, __m256i) {
            let a = _mm256_cvtepu8_epi16(_mm256_extracti128_si256(val, 0));
            let b = _mm256_cvtepu8_epi16(_mm256_extracti128_si256(val, 1));
            (a, b)
        }
        for Sse41(val: __m128i) -> (__m128i, __m128i) {
            let a = _mm_cvtepu8_epi16(val);
            let b = _mm_cvtepu8_epi16(_mm_shuffle_epi32(val, 0b_01_00_11_10));
            (a, b)
        }
        for Sse2(val: __m128i) -> (__m128i, __m128i) {
            let arr = core::mem::transmute::<__m128i, [i8; 16]>(val);
            let a = [
                arr[0] as u8 as u16 as i16,
                arr[1] as u8 as u16 as i16,
                arr[2] as u8 as u16 as i16,
                arr[3] as u8 as u16 as i16,
                arr[4] as u8 as u16 as i16,
                arr[5] as u8 as u16 as i16,
                arr[6] as u8 as u16 as i16,
                arr[7] as u8 as u16 as i16,
            ];
            let b = [
                arr[8] as u8 as u16 as i16,
                arr[9] as u8 as u16 as i16,
                arr[10] as u8 as u16 as i16,
                arr[11] as u8 as u16 as i16,
                arr[12] as u8 as u16 as i16,
                arr[13] as u8 as u16 as i16,
                arr[14] as u8 as u16 as i16,
                arr[15] as u8 as u16 as i16,
            ];
            (core::mem::transmute::<[i16; 8], __m128i>(a), core::mem::transmute::<[i16; 8], __m128i>(b))
        }
        for Scalar(val: i8) -> (i16, i16) {
            (val as u8 as u16 as i16, 0)
        }
        for Neon(val: int8x16_t) -> (int16x8_t, int16x8_t) {
            let a = vreinterpretq_s16_u16(vmovl_u8(vreinterpret_u8_s8(vget_low_s8(val))));
            let b = vreinterpretq_s16_u16(vmovl_u8(vreinterpret_u8_s8(vget_high_s8(val))));
            (a, b)
        }
        for Wasm(val: v128) -> (v128, v128) {
            let a = i16x8_extend_low_u8x16(val);
            let b = i16x8_extend_high_u8x16(val);
            (a, b)
        }
    }
}

impl_op! {
    fn get_mask<i8> {
        for Avx2(val: __m256i) -> u32 {
            _mm256_movemask_epi8(val) as u32
        }
        for Sse41(val: __m128i) -> u32 {
            _mm_movemask_epi8(val) as u32
        }
        for Sse2(val: __m128i) -> u32 {
            _mm_movemask_epi8(val) as u32
        }
        for Scalar(val: i8) -> u32 {
            ((val as u8) & 0x1) as u32
        }
        for Neon(val: int8x16_t) -> u32 {
            let val = vreinterpretq_u8_s8(val);

            // Algorithm copied from here: https://stackoverflow.com/a/68694558
            let uc_shift: [i8; 16] = [-7,-6,-5,-4,-3,-2,-1,0,-7,-6,-5,-4,-3,-2,-1,0];
            let vshift: int8x16_t = std::mem::transmute(uc_shift);
            let vmask = vandq_u8(val, vdupq_n_u8(0x80));

            let vmask = vshlq_u8(vmask, vshift);

            let mut out: u16 = 0;
            out |= vaddv_u8(vget_low_u8(vmask)) as u16;
            out |= (vaddv_u8(vget_high_u8(vmask)) as u16) << 8;

            out as u32
        }
        for Wasm(val: v128) -> u32 {
            i8x16_bitmask(val) as u32
        }
    }
}

impl_op! {
    fn zeroes<i8> {
        for Avx2() -> __m256i {
            _mm256_setzero_si256()
        }
        for Sse41() -> __m128i {
            _mm_setzero_si128()
        }
        for Sse2() -> __m128i {
            _mm_setzero_si128()
        }
        for Scalar() -> i8 {
            0
        }
        for Neon() -> int8x16_t {
            vdupq_n_s8(0)
        }
        for Wasm() -> v128 {
            i8x16_splat(0)
        }
    }
}

impl_op! {
    fn set1<i8> {
        for Avx2(val: i8) -> __m256i {
            _mm256_set1_epi8(val)
        }
        for Sse41(val: i8) -> __m128i {
            _mm_set1_epi8(val)
        }
        for Sse2(val: i8) -> __m128i {
            _mm_set1_epi8(val)
        }
        for Scalar(val: i8) -> i8 {
            val
        }
        for Neon(val: i8) -> int8x16_t {
            vdupq_n_s8(val)
        }
        for Wasm(val: i8) -> v128 {
            i8x16_splat(val)
        }
    }
}

impl_op! {
    fn load_unaligned<i8> {
        for Avx2(ptr: *const i8) -> __m256i {
            _mm256_loadu_si256(ptr as *const __m256i)
        }
        for Sse41(ptr: *const i8) -> __m128i {
            _mm_loadu_si128(ptr as *const __m128i)
        }
        for Sse2(ptr: *const i8) -> __m128i {
            _mm_loadu_si128(ptr as *const __m128i)
        }
        for Scalar(ptr: *const i8) -> i8 {
            unsafe { *ptr }
        }
        for Neon(ptr: *const i8) -> int8x16_t {
            vld1q_s8(ptr)
        }
        for Wasm(ptr: *const i8) -> v128 {
            *(ptr as *const v128)
        }
    }
}

impl_op! {
    fn load_aligned<i8> {
        for Avx2(ptr: *const i8) -> __m256i {
            _mm256_load_si256(ptr as *const __m256i)
        }
        for Sse41(ptr: *const i8) -> __m128i {
            _mm_load_si128(ptr as *const __m128i)
        }
        for Sse2(ptr: *const i8) -> __m128i {
            _mm_load_si128(ptr as *const __m128i)
        }
        for Scalar(ptr: *const i8) -> i8 {
            unsafe { *ptr }
        }
        for Neon(ptr: *const i8) -> int8x16_t {
            vld1q_s8(ptr)
        }
        for Wasm(ptr: *const i8) -> v128 {
            *(ptr as *const v128)
        }
    }
}

impl_op! {
    fn store_unaligned<i8> {
        for Avx2(ptr: *mut i8, a: __m256i) {
            _mm256_storeu_si256(ptr as *mut __m256i, a)
        }
        for Sse41(ptr: *mut i8, a: __m128i) {
            _mm_storeu_si128(ptr as *mut __m128i, a)
        }
        for Sse2(ptr: *mut i8, a: __m128i) {
            _mm_storeu_si128(ptr as *mut __m128i, a)
        }
        for Scalar(ptr: *mut i8, a: i8) {
            unsafe { *ptr = a }
        }
        for Neon(ptr: *mut i8, a: int8x16_t) {
            vst1q_s8(ptr, a)
        }
        for Wasm(ptr: *mut i8, a: v128) {
            *(ptr as *mut v128) = a;
        }
    }
}

impl_op! {
    fn store_aligned<i8> {
        for Avx2(ptr: *mut i8, a: __m256i) {
            _mm256_store_si256(ptr as *mut __m256i, a)
        }
        for Sse41(ptr: *mut i8, a: __m128i) {
            _mm_store_si128(ptr as *mut __m128i, a)
        }
        for Sse2(ptr: *mut i8, a: __m128i) {
            _mm_store_si128(ptr as *mut __m128i, a)
        }
        for Scalar(ptr: *mut i8, a: i8) {
            unsafe { *ptr = a }
        }
        for Neon(ptr: *mut i8, a: int8x16_t) {
            vst1q_s8(ptr, a)
        }
        for Wasm(ptr: *mut i8, a: v128) {
            *(ptr as *mut v128) = a;
        }
    }
}
