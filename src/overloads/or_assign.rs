use super::*;
// -- BitOrAssign
impl BitOrAssign for I32x1 {
    fn bitor_assign(&mut self, rhs: I32x1) {
        *self = I32x1(self.0 | rhs.0)
    }
}
impl BitOrAssign for I32x4 {
    fn bitor_assign(&mut self, rhs: I32x4) {
        *self = I32x4(unsafe { _mm_or_si128(self.0, rhs.0) })
    }
}
impl BitOrAssign for I32x4_41 {
    fn bitor_assign(&mut self, rhs: I32x4_41) {
        *self = I32x4_41(unsafe { _mm_or_si128(self.0, rhs.0) })
    }
}
impl BitOrAssign for I32x8 {
    fn bitor_assign(&mut self, rhs: I32x8) {
        *self = I32x8(unsafe { _mm256_or_si256(self.0, rhs.0) })
    }
}
impl BitOrAssign for F32x1 {
    fn bitor_assign(&mut self, rhs: F32x1) {
        unsafe {
            let self_i = mem::transmute::<&mut F32x1, &mut i32>(self);
            let rhs_i = mem::transmute::<F32x1, i32>(rhs);
            *self = mem::transmute::<i32, F32x1>(*self_i | rhs_i)
        }
    }
}
impl BitOrAssign for F32x4 {
    fn bitor_assign(&mut self, rhs: F32x4) {
        *self = F32x4(unsafe { _mm_or_ps(self.0, rhs.0) })
    }
}
impl BitOrAssign for F32x8 {
    fn bitor_assign(&mut self, rhs: F32x8) {
        *self = F32x8(unsafe { _mm256_or_ps(self.0, rhs.0) })
    }
}
