use super::*;
// -- Sub
impl Sub for I16x1 {
    type Output = I16x1;

    #[inline(always)]
    fn sub(self, rhs: I16x1) -> I16x1 {
        I16x1(self.0 - rhs.0)
    }
}

impl Sub for I32x1 {
    type Output = I32x1;

    #[inline(always)]
    fn sub(self, rhs: I32x1) -> I32x1 {
        I32x1(self.0 - rhs.0)
    }
}

impl Sub for I64x1 {
    type Output = I64x1;

    #[inline(always)]
    fn sub(self, rhs: I64x1) -> I64x1 {
        I64x1(self.0 - rhs.0)
    }
}

impl Sub for F32x1 {
    type Output = F32x1;

    #[inline(always)]
    fn sub(self, rhs: F32x1) -> F32x1 {
        F32x1(self.0 - rhs.0)
    }
}

impl Sub for F64x1 {
    type Output = F64x1;

    #[inline(always)]
    fn sub(self, rhs: F64x1) -> F64x1 {
        F64x1(self.0 - rhs.0)
    }
}
impl Sub for I16x8 {
    type Output = I16x8;

    #[inline(always)]
    fn sub(self, rhs: I16x8) -> I16x8 {
        I16x8(unsafe { _mm_sub_epi16(self.0, rhs.0) })
    }
}
impl Sub for I16x16 {
    type Output = I16x16;

    #[inline(always)]
    fn sub(self, rhs: I16x16) -> I16x16 {
        I16x16(unsafe { _mm256_sub_epi16(self.0, rhs.0) })
    }
}
impl Sub for I32x4 {
    type Output = I32x4;

    #[inline(always)]
    fn sub(self, rhs: I32x4) -> I32x4 {
        I32x4(unsafe { _mm_sub_epi32(self.0, rhs.0) })
    }
}
impl Sub for I32x4_41 {
    type Output = I32x4_41;

    #[inline(always)]
    fn sub(self, rhs: I32x4_41) -> I32x4_41 {
        I32x4_41(unsafe { _mm_sub_epi32(self.0, rhs.0) })
    }
}
impl Sub for I32x8 {
    type Output = I32x8;

    #[inline(always)]
    fn sub(self, rhs: I32x8) -> I32x8 {
        I32x8(unsafe { _mm256_sub_epi32(self.0, rhs.0) })
    }
}
impl Sub for I64x2 {
    type Output = I64x2;

    #[inline(always)]
    fn sub(self, rhs: I64x2) -> I64x2 {
        I64x2(unsafe { _mm_sub_epi64(self.0, rhs.0) })
    }
}
impl Sub for I64x2_41 {
    type Output = I64x2_41;

    #[inline(always)]
    fn sub(self, rhs: I64x2_41) -> I64x2_41 {
        I64x2_41(unsafe { _mm_sub_epi64(self.0, rhs.0) })
    }
}
impl Sub for I64x4 {
    type Output = I64x4;

    #[inline(always)]
    fn sub(self, rhs: I64x4) -> I64x4 {
        I64x4(unsafe { _mm256_sub_epi64(self.0, rhs.0) })
    }
}
impl Sub for F32x4 {
    type Output = F32x4;

    #[inline(always)]
    fn sub(self, rhs: F32x4) -> F32x4 {
        F32x4(unsafe { _mm_sub_ps(self.0, rhs.0) })
    }
}
impl Sub for F32x8 {
    type Output = F32x8;

    #[inline(always)]
    fn sub(self, rhs: F32x8) -> F32x8 {
        F32x8(unsafe { _mm256_sub_ps(self.0, rhs.0) })
    }
}

impl Sub for F64x2 {
    type Output = F64x2;

    #[inline(always)]
    fn sub(self, rhs: F64x2) -> F64x2 {
        F64x2(unsafe { _mm_sub_pd(self.0, rhs.0) })
    }
}
impl Sub for F64x4 {
    type Output = F64x4;

    #[inline(always)]
    fn sub(self, rhs: F64x4) -> F64x4 {
        F64x4(unsafe { _mm256_sub_pd(self.0, rhs.0) })
    }
}
