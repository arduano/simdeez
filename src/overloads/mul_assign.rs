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

impl MulAssign for I16x8 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: I16x8) {
        *self = I16x8(unsafe { _mm_mullo_epi16(self.0, rhs.0) })
    }
}

impl MulAssign for I16x16 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: I16x16) {
        *self = I16x16(unsafe { _mm256_mullo_epi16(self.0, rhs.0) })
    }
}

impl MulAssign for I32x4 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: I32x4) {
        *self = I32x4(unsafe { _mm_mul_epi32(self.0, rhs.0) })
    }
}
impl MulAssign for I32x4_41 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: I32x4_41) {
        *self = I32x4_41(unsafe { _mm_mul_epi32(self.0, rhs.0) })
    }
}
impl MulAssign for I32x8 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: I32x8) {
        *self = I32x8(unsafe { _mm256_mul_epi32(self.0, rhs.0) })
    }
}

impl MulAssign for F32x4 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: F32x4) {
        *self = F32x4(unsafe { _mm_mul_ps(self.0, rhs.0) })
    }
}
impl MulAssign for F32x8 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: F32x8) {
        *self = F32x8(unsafe { _mm256_mul_ps(self.0, rhs.0) })
    }
}

impl MulAssign for F64x2 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: F64x2) {
        *self = F64x2(unsafe { _mm_mul_pd(self.0, rhs.0) })
    }
}
impl MulAssign for F64x4 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: F64x4) {
        *self = F64x4(unsafe { _mm256_mul_pd(self.0, rhs.0) })
    }
}
