use super::*;
// -- SubAssign

impl SubAssign for I32x4 {
    fn sub_assign(&mut self, rhs: I32x4) {
        *self = I32x4(unsafe { _mm_sub_epi32(self.0, rhs.0) })
    }
}
impl SubAssign for I32x4_41 {
    fn sub_assign(&mut self, rhs: I32x4_41) {
        *self = I32x4_41(unsafe { _mm_sub_epi32(self.0, rhs.0) })
    }
}
impl SubAssign for I32x8 {
    fn sub_assign(&mut self, rhs: I32x8) {
        *self = I32x8(unsafe { _mm256_sub_epi32(self.0, rhs.0) })
    }
}
impl SubAssign for I64x2 {
    fn sub_assign(&mut self, rhs: I64x2) {
        *self = I64x2(unsafe { _mm_sub_epi64(self.0, rhs.0) })
    }
}
impl SubAssign for I64x2_41 {
    fn sub_assign(&mut self, rhs: I64x2_41) {
        *self = I64x2_41(unsafe { _mm_sub_epi64(self.0, rhs.0) })
    }
}
impl SubAssign for I64x4 {
    fn sub_assign(&mut self, rhs: I64x4) {
        *self = I64x4(unsafe { _mm256_sub_epi64(self.0, rhs.0) })
    }
}
impl SubAssign for F32x4 {
    fn sub_assign(&mut self, rhs: F32x4) {
        *self = F32x4(unsafe { _mm_sub_ps(self.0, rhs.0) })
    }
}
impl SubAssign for F32x8 {
    fn sub_assign(&mut self, rhs: F32x8) {
        *self = F32x8(unsafe { _mm256_sub_ps(self.0, rhs.0) })
    }
}
impl SubAssign for F64x2 {
    fn sub_assign(&mut self, rhs: F64x2) {
        *self = F64x2(unsafe { _mm_sub_pd(self.0, rhs.0) })
    }
}
impl SubAssign for F64x4 {
    fn sub_assign(&mut self, rhs: F64x4) {
        *self = F64x4(unsafe { _mm256_sub_pd(self.0, rhs.0) })
    }
}
