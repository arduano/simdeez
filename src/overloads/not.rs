use super::*;

impl Not for I32x4 {
    type Output = I32x4;

    #[inline(always)]
    fn not(self) -> I32x4 {
        unsafe { I32x4(_mm_xor_si128(self.0, _mm_set1_epi32(-1))) }
    }
}
impl Not for I32x4_41 {
    type Output = I32x4_41;

    #[inline(always)]
    fn not(self) -> I32x4_41 {
        unsafe { I32x4_41(_mm_xor_si128(self.0, _mm_set1_epi32(-1))) }
    }
}
impl Not for I32x8 {
    type Output = I32x8;

    #[inline(always)]
    fn not(self) -> I32x8 {
        unsafe { I32x8(_mm256_xor_si256(self.0, _mm256_set1_epi32(-1))) }
    }
}
impl Not for I64x2 {
    type Output = I64x2;

    #[inline(always)]
    fn not(self) -> I64x2 {
        unsafe { I64x2(_mm_xor_si128(self.0, _mm_set1_epi64x(-1))) }
    }
}
impl Not for I64x2_41 {
    type Output = I64x2_41;

    #[inline(always)]
    fn not(self) -> I64x2_41 {
        unsafe { I64x2_41(_mm_xor_si128(self.0, _mm_set1_epi64x(-1))) }
    }
}
impl Not for I64x4 {
    type Output = I64x4;

    #[inline(always)]
    fn not(self) -> I64x4 {
        unsafe { I64x4(_mm256_xor_si256(self.0, _mm256_set1_epi64x(-1))) }
    }
}
