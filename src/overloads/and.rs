use super::*;
// -- Bitwise And

impl BitAnd for I32x4 {
    type Output = I32x4;

    fn bitand(self, rhs: I32x4) -> I32x4 {
        I32x4(unsafe { _mm_and_si128(self.0, rhs.0) })
    }
}
impl BitAnd for I32x4_41 {
    type Output = I32x4_41;

    fn bitand(self, rhs: I32x4_41) -> I32x4_41 {
        I32x4_41(unsafe { _mm_and_si128(self.0, rhs.0) })
    }
}
impl BitAnd for I32x8 {
    type Output = I32x8;

    fn bitand(self, rhs: I32x8) -> I32x8 {
        I32x8(unsafe { _mm256_and_si256(self.0, rhs.0) })
    }
}

impl BitAnd for F32x4 {
    type Output = F32x4;

    fn bitand(self, rhs: F32x4) -> F32x4 {
        F32x4(unsafe { _mm_and_ps(self.0, rhs.0) })
    }
}
impl BitAnd for F32x8 {
    type Output = F32x8;

    fn bitand(self, rhs: F32x8) -> F32x8 {
        F32x8(unsafe { _mm256_and_ps(self.0, rhs.0) })
    }
}

impl BitAnd for F64x2 {
    type Output = F64x2;

    fn bitand(self, rhs: F64x2) -> F64x2 {
        F64x2(unsafe { _mm_and_pd(self.0, rhs.0) })
    }
}
impl BitAnd for F64x4 {
    type Output = F64x4;

    fn bitand(self, rhs: F64x4) -> F64x4 {
        F64x4(unsafe { _mm256_and_pd(self.0, rhs.0) })
    }
}
