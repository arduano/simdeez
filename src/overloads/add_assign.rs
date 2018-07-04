use super::*;
// -- AddAssign

impl AddAssign for I32x4 {
    fn add_assign(&mut self, rhs: I32x4) {
        *self = I32x4(unsafe { _mm_add_epi32(self.0, rhs.0) })
    }
}
impl AddAssign for I32x4_41 {
    fn add_assign(&mut self, rhs: I32x4_41) {
        *self = I32x4_41(unsafe { _mm_add_epi32(self.0, rhs.0) })
    }
}
impl AddAssign for I32x8 {
    fn add_assign(&mut self, rhs: I32x8) {
        *self = I32x8(unsafe { _mm256_add_epi32(self.0, rhs.0) })
    }
}
impl AddAssign for F32x4 {
    fn add_assign(&mut self, rhs: F32x4) {
        *self = F32x4(unsafe { _mm_add_ps(self.0, rhs.0) })
    }
}
impl AddAssign<F32x8> for F32x8 {
    fn add_assign(&mut self, rhs: F32x8) {
        *self = F32x8(unsafe { _mm256_add_ps(self.0, rhs.0) })
    }
}
impl AddAssign for F64x2 {
    fn add_assign(&mut self, rhs: F64x2) {
        *self = F64x2(unsafe { _mm_add_pd(self.0, rhs.0) })
    }
}
impl AddAssign<F64x4> for F64x4 {
    fn add_assign(&mut self, rhs: F64x4) {
        *self = F64x4(unsafe { _mm256_add_pd(self.0, rhs.0) })
    }
}
