use super::*;
// -- BitAndAssign
impl BitAndAssign for I32x1 {
    fn bitand_assign(&mut self, rhs: I32x1) {
        *self = I32x1(self.0 & rhs.0)
    }
}
impl BitAndAssign for I32x4 {
    fn bitand_assign(&mut self, rhs: I32x4) {
        *self = I32x4(unsafe { _mm_and_si128(self.0, rhs.0) })
    }
}
impl BitAndAssign for I32x4_41 {
    fn bitand_assign(&mut self, rhs: I32x4_41) {
        *self = I32x4_41(unsafe { _mm_and_si128(self.0, rhs.0) })
    }
}
impl BitAndAssign for I32x8 {
    fn bitand_assign(&mut self, rhs: I32x8) {
        *self = I32x8(unsafe { _mm256_and_si256(self.0, rhs.0) })
    }
}
impl BitAndAssign for F32x1 {
    fn bitand_assign(&mut self, rhs: F32x1) {
        unsafe {
            let self_i = mem::transmute::<&mut F32x1, &mut i32>(self);
            let rhs_i = mem::transmute::<F32x1, i32>(rhs);
            *self = mem::transmute::<i32, F32x1>(*self_i & rhs_i)
        }
    }
}
impl BitAndAssign for F32x4 {
    fn bitand_assign(&mut self, rhs: F32x4) {
        *self = F32x4(unsafe { _mm_and_ps(self.0, rhs.0) })
    }
}
impl BitAndAssign for F32x8 {
    fn bitand_assign(&mut self, rhs: F32x8) {
        *self = F32x8(unsafe { _mm256_and_ps(self.0, rhs.0) })
    }
}
