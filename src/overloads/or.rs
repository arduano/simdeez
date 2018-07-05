use super::*;
// -- Bitwise Or

impl BitOr for I32x4 {
    type Output = I32x4;

    fn bitor(self, rhs: I32x4) -> I32x4 {
        I32x4(unsafe { _mm_or_si128(self.0, rhs.0) })
    }
}
impl BitOr for I32x4_41 {
    type Output = I32x4_41;

    fn bitor(self, rhs: I32x4_41) -> I32x4_41 {
        I32x4_41(unsafe { _mm_or_si128(self.0, rhs.0) })
    }
}
impl BitOr for I32x8 {
    type Output = I32x8;

    fn bitor(self, rhs: I32x8) -> I32x8 {
        I32x8(unsafe { _mm256_or_si256(self.0, rhs.0) })
    }
}
impl BitOr for I64x2 {
    type Output = I64x2;

    fn bitor(self, rhs: I64x2) -> I64x2 {
        I64x2(unsafe { _mm_or_si128(self.0, rhs.0) })
    }
}
impl BitOr for I64x2_41 {
    type Output = I64x2_41;

    fn bitor(self, rhs: I64x2_41) -> I64x2_41 {
        I64x2_41(unsafe { _mm_or_si128(self.0, rhs.0) })
    }
}
impl BitOr for I64x4 {
    type Output = I64x4;

    fn bitor(self, rhs: I64x4) -> I64x4 {
        I64x4(unsafe { _mm256_or_si256(self.0, rhs.0) })
    }
}
impl BitOr for F32x4 {
    type Output = F32x4;

    fn bitor(self, rhs: F32x4) -> F32x4 {
        F32x4(unsafe { _mm_or_ps(self.0, rhs.0) })
    }
}
impl BitOr for F32x8 {
    type Output = F32x8;

    fn bitor(self, rhs: F32x8) -> F32x8 {
        F32x8(unsafe { _mm256_or_ps(self.0, rhs.0) })
    }
}

impl BitOr for F64x2 {
    type Output = F64x2;

    fn bitor(self, rhs: F64x2) -> F64x2 {
        F64x2(unsafe { _mm_or_pd(self.0, rhs.0) })
    }
}
impl BitOr for F64x4 {
    type Output = F64x4;

    fn bitor(self, rhs: F64x4) -> F64x4 {
        F64x4(unsafe { _mm256_or_pd(self.0, rhs.0) })
    }
}
