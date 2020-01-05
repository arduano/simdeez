use super::*;
// -- BitOrAssign

impl BitOrAssign for I16x1 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: I16x1) {
        *self = I16x1(self.0 | rhs.0);
    }
}

impl BitOrAssign for I32x1 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: I32x1) {
        *self = I32x1(self.0 | rhs.0);
    }
}

impl BitOrAssign for I64x1 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: I64x1) {
        *self = I64x1(self.0 | rhs.0);
    }
}

impl BitOrAssign for F32x1 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: F32x1) {
        let lbits = self.0.to_bits();
        let rbits = rhs.0.to_bits();
        let result = lbits | rbits;
        *self = F32x1(f32::from_bits(result));
    }
}

impl BitOrAssign for F64x1 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: F64x1) {
        let lbits = self.0.to_bits();
        let rbits = rhs.0.to_bits();
        let result = lbits | rbits;
        *self = F64x1(f64::from_bits(result));
    }
}

impl BitOrAssign for I16x16 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: I16x16) {
        *self = I16x16(unsafe { _mm256_or_si256(self.0, rhs.0) })
    }
}


impl BitOrAssign for I32x8 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: I32x8) {
        *self = I32x8(unsafe { _mm256_or_si256(self.0, rhs.0) })
    }
}


impl BitOrAssign for I64x4 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: I64x4) {
        *self = I64x4(unsafe { _mm256_or_si256(self.0, rhs.0) })
    }
}

impl BitOrAssign for F32x8 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: F32x8) {
        *self = F32x8(unsafe { _mm256_or_ps(self.0, rhs.0) })
    }
}

impl BitOrAssign for F64x4 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: F64x4) {
        *self = F64x4(unsafe { _mm256_or_pd(self.0, rhs.0) })
    }
}
