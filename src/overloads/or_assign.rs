use super::*;
// -- BitOrAssign

impl BitOrAssign for I16x8 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: I16x8) {
        *self = I16x8(unsafe { _mm_or_si128(self.0, rhs.0) })
    }
}
impl BitOrAssign for I16x16 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: I16x16) {
        *self = I16x16(unsafe { _mm256_or_si256(self.0, rhs.0) })
    }
}
impl BitOrAssign for I32x4 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: I32x4) {
        *self = I32x4(unsafe { _mm_or_si128(self.0, rhs.0) })
    }
}
impl BitOrAssign for I32x4_41 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: I32x4_41) {
        *self = I32x4_41(unsafe { _mm_or_si128(self.0, rhs.0) })
    }
}
impl BitOrAssign for I32x8 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: I32x8) {
        *self = I32x8(unsafe { _mm256_or_si256(self.0, rhs.0) })
    }
}
impl BitOrAssign for I64x2 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: I64x2) {
        *self = I64x2(unsafe { _mm_or_si128(self.0, rhs.0) })
    }
}
impl BitOrAssign for I64x2_41 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: I64x2_41) {
        *self = I64x2_41(unsafe { _mm_or_si128(self.0, rhs.0) })
    }
}
impl BitOrAssign for I64x4 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: I64x4) {
        *self = I64x4(unsafe { _mm256_or_si256(self.0, rhs.0) })
    }
}
impl BitOrAssign for F32x4 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: F32x4) {
        *self = F32x4(unsafe { _mm_or_ps(self.0, rhs.0) })
    }
}
impl BitOrAssign for F32x8 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: F32x8) {
        *self = F32x8(unsafe { _mm256_or_ps(self.0, rhs.0) })
    }
}

impl BitOrAssign for F64x2 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: F64x2) {
        *self = F64x2(unsafe { _mm_or_pd(self.0, rhs.0) })
    }
}
impl BitOrAssign for F64x4 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: F64x4) {
        *self = F64x4(unsafe { _mm256_or_pd(self.0, rhs.0) })
    }
}
