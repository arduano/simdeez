use super::*;

// -- MulAssign

impl MulAssign for I32x4 {
    fn mul_assign(&mut self, rhs: I32x4) {
        *self = I32x4(unsafe { _mm_mul_epi32(self.0, rhs.0) })
    }
}
impl MulAssign for I32x4_41 {
    fn mul_assign(&mut self, rhs: I32x4_41) {
        *self = I32x4_41(unsafe { _mm_mul_epi32(self.0, rhs.0) })
    }
}
impl MulAssign for I32x8 {
    fn mul_assign(&mut self, rhs: I32x8) {
        *self = I32x8(unsafe { _mm256_mul_epi32(self.0, rhs.0) })
    }
}

impl MulAssign for F32x4 {
    fn mul_assign(&mut self, rhs: F32x4) {
        *self = F32x4(unsafe { _mm_mul_ps(self.0, rhs.0) })
    }
}
impl MulAssign for F32x8 {
    fn mul_assign(&mut self, rhs: F32x8) {
        *self = F32x8(unsafe { _mm256_mul_ps(self.0, rhs.0) })
    }
}

impl MulAssign for F64x2 {
    fn mul_assign(&mut self, rhs: F64x2) {
        *self = F64x2(unsafe { _mm_mul_pd(self.0, rhs.0) })
    }
}
impl MulAssign for F64x4 {
    fn mul_assign(&mut self, rhs: F64x4) {
        *self = F64x4(unsafe { _mm256_mul_pd(self.0, rhs.0) })
    }
}
