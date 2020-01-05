use super::*;

impl Not for I16x1 {
    type Output = I16x1;

    #[inline(always)]
    fn not(self) -> I16x1 {
        I16x1(!(self.0))
    }
}

impl Not for I32x1 {
    type Output = I32x1;

    #[inline(always)]
    fn not(self) -> I32x1 {
        I32x1(!(self.0))
    }
}

impl Not for I64x1 {
    type Output = I64x1;

    #[inline(always)]
    fn not(self) -> I64x1 {
        I64x1(!(self.0))
    }
}

impl Not for F32x1 {
    type Output = F32x1;

    #[inline(always)]
    fn not(self) -> F32x1 {
        let bits = !(self.0.to_bits());
        F32x1(f32::from_bits(bits))
    }
}

impl Not for F64x1 {
    type Output = F64x1;

    #[inline(always)]
    fn not(self) -> F64x1 {
        let bits = !(self.0.to_bits());
        F64x1(f64::from_bits(bits))
    }
}

impl Not for I16x16 {
    type Output = I16x16;

    #[inline(always)]
    fn not(self) -> I16x16 {
        unsafe { I16x16(_mm256_xor_si256(self.0, _mm256_set1_epi16(-1))) }
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
