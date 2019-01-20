use super::*;
// -- SubAssign
impl SubAssign for I16x1 {
       #[inline(always)]
    fn sub_assign(&mut self, rhs: I16x1) {
        *self = I16x1(self.0-rhs.0);
    }
}

impl SubAssign for I32x1 {
       #[inline(always)]
    fn sub_assign(&mut self, rhs: I32x1) {
        *self = I32x1(self.0-rhs.0);
    }
}

impl SubAssign for I64x1 {
       #[inline(always)]
    fn sub_assign(&mut self, rhs: I64x1) {
        *self = I64x1(self.0-rhs.0);
    }
}

impl SubAssign for F32x1 {
       #[inline(always)]
    fn sub_assign(&mut self, rhs: F32x1) {
        *self = F32x1(self.0-rhs.0);
    }
}

impl SubAssign for F64x1 {
       #[inline(always)]
    fn sub_assign(&mut self, rhs: F64x1) {
        *self = F64x1(self.0-rhs.0);
    }
}
impl SubAssign for I16x8 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: I16x8) {
        *self = I16x8(unsafe { _mm_sub_epi16(self.0, rhs.0) })
    }
}
impl SubAssign for I16x16 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: I16x16) {
        *self = I16x16(unsafe { _mm256_sub_epi16(self.0, rhs.0) })
    }
}
impl SubAssign for I32x4 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: I32x4) {
        *self = I32x4(unsafe { _mm_sub_epi32(self.0, rhs.0) })
    }
}
impl SubAssign for I32x4_41 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: I32x4_41) {
        *self = I32x4_41(unsafe { _mm_sub_epi32(self.0, rhs.0) })
    }
}
impl SubAssign for I32x8 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: I32x8) {
        *self = I32x8(unsafe { _mm256_sub_epi32(self.0, rhs.0) })
    }
}
impl SubAssign for I64x2 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: I64x2) {
        *self = I64x2(unsafe { _mm_sub_epi64(self.0, rhs.0) })
    }
}
impl SubAssign for I64x2_41 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: I64x2_41) {
        *self = I64x2_41(unsafe { _mm_sub_epi64(self.0, rhs.0) })
    }
}
impl SubAssign for I64x4 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: I64x4) {
        *self = I64x4(unsafe { _mm256_sub_epi64(self.0, rhs.0) })
    }
}
impl SubAssign for F32x4 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: F32x4) {
        *self = F32x4(unsafe { _mm_sub_ps(self.0, rhs.0) })
    }
}
impl SubAssign for F32x8 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: F32x8) {
        *self = F32x8(unsafe { _mm256_sub_ps(self.0, rhs.0) })
    }
}
impl SubAssign for F64x2 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: F64x2) {
        *self = F64x2(unsafe { _mm_sub_pd(self.0, rhs.0) })
    }
}
impl SubAssign for F64x4 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: F64x4) {
        *self = F64x4(unsafe { _mm256_sub_pd(self.0, rhs.0) })
    }
}
