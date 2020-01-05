use super::*;
// -- AddAssign
impl AddAssign for I16x1 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: I16x1) {
        *self = I16x1(self.0 + rhs.0);
    }
}

impl AddAssign for I32x1 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: I32x1) {
        *self = I32x1(self.0 + rhs.0);
    }
}

impl AddAssign for I64x1 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: I64x1) {
        *self = I64x1(self.0 + rhs.0);
    }
}

impl AddAssign for F32x1 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: F32x1) {
        *self = F32x1(self.0 + rhs.0);
    }
}

impl AddAssign for F64x1 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: F64x1) {
        *self = F64x1(self.0 + rhs.0);
    }
}

impl AddAssign for I16x16 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: I16x16) {
        *self = I16x16(unsafe { _mm256_add_epi16(self.0, rhs.0) })
    }
}


impl AddAssign for I32x8 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: I32x8) {
        *self = I32x8(unsafe { _mm256_add_epi32(self.0, rhs.0) })
    }
}

impl AddAssign for I64x4 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: I64x4) {
        *self = I64x4(unsafe { _mm256_add_epi64(self.0, rhs.0) })
    }
}
impl AddAssign for F32x8 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: F32x8) {
        *self = F32x8(unsafe { _mm256_add_ps(self.0, rhs.0) })
    }
}
impl AddAssign<F64x4> for F64x4 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: F64x4) {
        *self = F64x4(unsafe { _mm256_add_pd(self.0, rhs.0) })
    }
}
