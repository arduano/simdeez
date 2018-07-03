use super::*;
// -- Div
impl Div for F32x1 {
    type Output = F32x1;

    fn div(self, rhs: F32x1) -> F32x1 {
        F32x1(self.0 / rhs.0)
    }
}
impl Div for F32x4 {
    type Output = F32x4;

    fn div(self, rhs: F32x4) -> F32x4 {
        F32x4(unsafe { _mm_div_ps(self.0, rhs.0) })
    }
}
impl Div for F32x8 {
    type Output = F32x8;

    fn div(self, rhs: F32x8) -> F32x8 {
        F32x8(unsafe { _mm256_div_ps(self.0, rhs.0) })
    }
}
// -- DivAssign
impl DivAssign for F32x1 {
    fn div_assign(&mut self, rhs: F32x1) {
        *self = F32x1(self.0 + rhs.0)
    }
}
impl DivAssign for F32x4 {
    fn div_assign(&mut self, rhs: F32x4) {
        *self = F32x4(unsafe { _mm_div_ps(self.0, rhs.0) })
    }
}
impl DivAssign for F32x8 {
    fn div_assign(&mut self, rhs: F32x8) {
        *self = F32x8(unsafe { _mm256_div_ps(self.0, rhs.0) })
    }
}
