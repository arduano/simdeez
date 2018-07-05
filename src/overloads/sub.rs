use super::*;
// -- Sub

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
impl Sub for I64x2 {
    type Output = I64x2;

    fn sub(self, rhs: I64x2) -> I64x2 {
        I64x2(unsafe { _mm_sub_epi64(self.0, rhs.0) })
    }
}
impl Sub for I64x2_41 {
    type Output = I64x2_41;

    fn sub(self, rhs: I64x2_41) -> I64x2_41 {
        I64x2_41(unsafe { _mm_sub_epi64(self.0, rhs.0) })
    }
}
impl Sub for I64x4 {
    type Output = I64x4;

    fn sub(self, rhs: I64x4) -> I64x4 {
        I64x4(unsafe { _mm256_sub_epi64(self.0, rhs.0) })
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

impl Sub for F64x2 {
    type Output = F64x2;

    fn sub(self, rhs: F64x2) -> F64x2 {
        F64x2(unsafe { _mm_sub_pd(self.0, rhs.0) })
    }
}
impl Sub for F64x4 {
    type Output = F64x4;

    fn sub(self, rhs: F64x4) -> F64x4 {
        F64x4(unsafe { _mm256_sub_pd(self.0, rhs.0) })
    }
}
