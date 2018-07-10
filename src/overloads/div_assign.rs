use super::*;
// -- DivAssign

impl DivAssign for F32x4 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: F32x4) {
        *self = F32x4(unsafe { _mm_div_ps(self.0, rhs.0) })
    }
}
impl DivAssign for F32x8 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: F32x8) {
        *self = F32x8(unsafe { _mm256_div_ps(self.0, rhs.0) })
    }
}

impl DivAssign for F64x2 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: F64x2) {
        *self = F64x2(unsafe { _mm_div_pd(self.0, rhs.0) })
    }
}
impl DivAssign for F64x4 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: F64x4) {
        *self = F64x4(unsafe { _mm256_div_pd(self.0, rhs.0) })
    }
}
