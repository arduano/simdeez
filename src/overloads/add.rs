use super::*;

impl Add for I32x4 {
    type Output = I32x4;

    fn add(self, rhs: I32x4) -> I32x4 {
        I32x4(unsafe { _mm_add_epi32(self.0, rhs.0) })
    }
}
impl Add for I32x4_41 {
    type Output = I32x4_41;

    fn add(self, rhs: I32x4_41) -> I32x4_41 {
        I32x4_41(unsafe { _mm_add_epi32(self.0, rhs.0) })
    }
}
impl Add for I32x8 {
    type Output = I32x8;

    fn add(self, rhs: I32x8) -> I32x8 {
        I32x8(unsafe { _mm256_add_epi32(self.0, rhs.0) })
    }
}
impl Add for I64x2 {
    type Output = I64x2;

    fn add(self, rhs: I64x2) -> I64x2 {
        I64x2(unsafe { _mm_add_epi64(self.0, rhs.0) })
    }
}
impl Add for I64x2_41 {
    type Output = I64x2_41;

    fn add(self, rhs: I64x2_41) -> I64x2_41 {
        I64x2_41(unsafe { _mm_add_epi64(self.0, rhs.0) })
    }
}
impl Add for I64x4 {
    type Output = I64x4;

    fn add(self, rhs: I64x4) -> I64x4 {
        I64x4(unsafe { _mm256_add_epi64(self.0, rhs.0) })
    }
}

impl Add for F32x4 {
    type Output = F32x4;

    fn add(self, rhs: F32x4) -> F32x4 {
        F32x4(unsafe { _mm_add_ps(self.0, rhs.0) })
    }
}
impl Add for F32x8 {
    type Output = F32x8;

    fn add(self, rhs: F32x8) -> F32x8 {
        F32x8(unsafe { _mm256_add_ps(self.0, rhs.0) })
    }
}
impl Add for F64x2 {
    type Output = F64x2;

    fn add(self, rhs: F64x2) -> F64x2 {
        F64x2(unsafe { _mm_add_pd(self.0, rhs.0) })
    }
}
impl Add for F64x4 {
    type Output = F64x4;

    fn add(self, rhs: F64x4) -> F64x4 {
        F64x4(unsafe { _mm256_add_pd(self.0, rhs.0) })
    }
}
