use super::*;
// -- DivAssign

impl DivAssign for F32x1 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: F32x1) {
        *self = F32x1(self.0 / rhs.0);
    }
}

impl DivAssign for F64x1 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: F64x1) {
        *self = F64x1(self.0 / rhs.0);
    }
}

