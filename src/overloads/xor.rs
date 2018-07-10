use super::*;
// -- Bitwise XOr

impl BitXor for I32x4 {
    type Output = I32x4;

    #[inline(always)]
    fn bitxor(self, rhs: I32x4) -> I32x4 {
        I32x4(unsafe { _mm_xor_si128(self.0, rhs.0) })
    }
}
impl BitXor for I32x4_41 {
    type Output = I32x4_41;

    #[inline(always)]
    fn bitxor(self, rhs: I32x4_41) -> I32x4_41 {
        I32x4_41(unsafe { _mm_xor_si128(self.0, rhs.0) })
    }
}
impl BitXor for I32x8 {
    type Output = I32x8;

    #[inline(always)]
    fn bitxor(self, rhs: I32x8) -> I32x8 {
        I32x8(unsafe { _mm256_xor_si256(self.0, rhs.0) })
    }
}
impl BitXor for I64x2 {
    type Output = I64x2;

    #[inline(always)]
    fn bitxor(self, rhs: I64x2) -> I64x2 {
        I64x2(unsafe { _mm_xor_si128(self.0, rhs.0) })
    }
}
impl BitXor for I64x2_41 {
    type Output = I64x2_41;

    #[inline(always)]
    fn bitxor(self, rhs: I64x2_41) -> I64x2_41 {
        I64x2_41(unsafe { _mm_xor_si128(self.0, rhs.0) })
    }
}
impl BitXor for I64x4 {
    type Output = I64x4;

    #[inline(always)]
    fn bitxor(self, rhs: I64x4) -> I64x4 {
        I64x4(unsafe { _mm256_xor_si256(self.0, rhs.0) })
    }
}
impl BitXor for F32x4 {
    type Output = F32x4;

    #[inline(always)]
    fn bitxor(self, rhs: F32x4) -> F32x4 {
        F32x4(unsafe { _mm_xor_ps(self.0, rhs.0) })
    }
}
impl BitXor for F32x8 {
    type Output = F32x8;

    #[inline(always)]
    fn bitxor(self, rhs: F32x8) -> F32x8 {
        F32x8(unsafe { _mm256_xor_ps(self.0, rhs.0) })
    }
}

impl BitXor for F64x2 {
    type Output = F64x2;

    #[inline(always)]
    fn bitxor(self, rhs: F64x2) -> F64x2 {
        F64x2(unsafe { _mm_xor_pd(self.0, rhs.0) })
    }
}
impl BitXor for F64x4 {
    type Output = F64x4;

    #[inline(always)]
    fn bitxor(self, rhs: F64x4) -> F64x4 {
        F64x4(unsafe { _mm256_xor_pd(self.0, rhs.0) })
    }
}
