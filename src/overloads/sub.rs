use super::*;
// -- Sub
impl Sub for I32x1 {
    type Output = I32x1;

    fn sub(self, rhs: I32x1) -> I32x1 {
        I32x1(self.0 - rhs.0)
    }
}
impl Sub for I32x4 {
    type Output = I32x4;

    fn sub(self, rhs: I32x4) -> I32x4 {
        I32x4(unsafe { _mm_sub_epi32(self.0, rhs.0) })
    }
}
impl Sub for I32x4_41 {
    type Output = I32x4_41;

    fn sub(self, rhs: I32x4_41) -> I32x4_41 {
        I32x4_41(unsafe { _mm_sub_epi32(self.0, rhs.0) })
    }
}
impl Sub for I32x8 {
    type Output = I32x8;

    fn sub(self, rhs: I32x8) -> I32x8 {
        I32x8(unsafe { _mm256_sub_epi32(self.0, rhs.0) })
    }
}
impl Sub for F32x1 {
    type Output = F32x1;

    fn sub(self, rhs: F32x1) -> F32x1 {
        F32x1(self.0 - rhs.0)
    }
}
impl Sub for F32x4 {
    type Output = F32x4;

    fn sub(self, rhs: F32x4) -> F32x4 {
        F32x4(unsafe { _mm_sub_ps(self.0, rhs.0) })
    }
}
impl Sub for F32x8 {
    type Output = F32x8;

    fn sub(self, rhs: F32x8) -> F32x8 {
        F32x8(unsafe { _mm256_sub_ps(self.0, rhs.0) })
    }
}
