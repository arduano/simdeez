use super::*;

// -- MulAssign
impl MulAssign for I16x1 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: I16x1) {
        *self = I16x1(self.0.wrapping_mul(rhs.0));
    }
}

impl MulAssign for I32x1 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: I32x1) {
        *self = I32x1(self.0.wrapping_mul(rhs.0));
    }
}

impl MulAssign for I64x1 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: I64x1) {
        *self = I64x1(self.0.wrapping_mul(rhs.0));
    }
}

impl MulAssign for F32x1 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: F32x1) {
        *self = F32x1(self.0 * rhs.0);
    }
}

impl MulAssign for F64x1 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: F64x1) {
        *self = F64x1(self.0 * rhs.0);
    }
}

impl MulAssign for I16x16 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: I16x16) {
        *self = I16x16(unsafe { _mm256_mullo_epi16(self.0, rhs.0) })
    }
}



