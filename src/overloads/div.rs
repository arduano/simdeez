use super::*;
// -- Div

impl Div for F32x1 {
    type Output = F32x1;

    #[inline(always)]
    fn div(self, rhs: F32x1) -> F32x1 {
        F32x1(self.0 / rhs.0)
    }
}

impl Div for F64x1 {
    type Output = F64x1;

    #[inline(always)]
    fn div(self, rhs: F64x1) -> F64x1 {
        F64x1(self.0 / rhs.0)
    }
}

