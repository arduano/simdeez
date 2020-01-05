use super::*;
// -- BitXorAssign

impl BitXorAssign for I16x1 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: I16x1) {
        *self = I16x1(self.0 ^ rhs.0);
    }
}

impl BitXorAssign for I32x1 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: I32x1) {
        *self = I32x1(self.0 ^ rhs.0);
    }
}

impl BitXorAssign for I64x1 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: I64x1) {
        *self = I64x1(self.0 ^ rhs.0);
    }
}

impl BitXorAssign for F32x1 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: F32x1) {
        let lbits = self.0.to_bits();
        let rbits = rhs.0.to_bits();
        let result = lbits ^ rbits;
        *self = F32x1(f32::from_bits(result));
    }
}

impl BitXorAssign for F64x1 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: F64x1) {
        let lbits = self.0.to_bits();
        let rbits = rhs.0.to_bits();
        let result = lbits ^ rbits;
        *self = F64x1(f64::from_bits(result));
    }
}

impl BitXorAssign for I16x16 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: I16x16) {
        *self = I16x16(unsafe { _mm256_xor_si256(self.0, rhs.0) })
    }
}


