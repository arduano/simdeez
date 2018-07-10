use super::*;
// -- Div

impl Div for F32x4 {
    type Output = F32x4;

    #[inline(always)]
    fn div(self, rhs: F32x4) -> F32x4 {
        F32x4(unsafe { _mm_div_ps(self.0, rhs.0) })
    }
}
impl Div for F32x8 {
    type Output = F32x8;

    #[inline(always)]
    fn div(self, rhs: F32x8) -> F32x8 {
        F32x8(unsafe { _mm256_div_ps(self.0, rhs.0) })
    }
}

impl Div for F64x2 {
    type Output = F64x2;

    #[inline(always)]
    fn div(self, rhs: F64x2) -> F64x2 {
        F64x2(unsafe { _mm_div_pd(self.0, rhs.0) })
    }
}
impl Div for F64x4 {
    type Output = F64x4;

    #[inline(always)]
    fn div(self, rhs: F64x4) -> F64x4 {
        F64x4(unsafe { _mm256_div_pd(self.0, rhs.0) })
    }
}
