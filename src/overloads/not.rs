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


