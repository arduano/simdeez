use super::*;

impl Shr<i32> for I16x1 {
    type Output = I16x1;

    #[inline(always)]
    fn shr(self, rhs: i32) -> I16x1 {
        I16x1(self.0 >> rhs)
    }
}

impl Shr<i32> for I32x1 {
    type Output = I32x1;

    #[inline(always)]
    fn shr(self, rhs: i32) -> I32x1 {
        I32x1(self.0 >> rhs)
    }
}

impl Shr<i32> for I64x1 {
    type Output = I64x1;

    #[inline(always)]
    fn shr(self, rhs: i32) -> I64x1 {
        I64x1(self.0 >> rhs)
    }
}

impl Shr<i32> for F32x1 {
    type Output = F32x1;

    #[inline(always)]
    fn shr(self, rhs: i32) -> F32x1 {
        let bits = self.0.to_bits() >> rhs;
        F32x1(f32::from_bits(bits))
    }
}

impl Shr<i32> for F64x1 {
    type Output = F64x1;

    #[inline(always)]
    fn shr(self, rhs: i32) -> F64x1 {
        let bits = self.0.to_bits() >> rhs;
        F64x1(f64::from_bits(bits))
    }
}

impl Shr<i32> for I16x16 {
    type Output = I16x16;

    #[inline(always)]
    fn shr(self, rhs: i32) -> I16x16 {
        macro_rules! call {
            ($rhs:expr) => {
                unsafe { I16x16(_mm256_srai_epi16(self.0, $rhs)) }
            };
        }
        constify_imm8!(rhs, call)
    }
}


impl Shr<i32> for I32x8 {
    type Output = I32x8;

    #[inline(always)]
    fn shr(self, rhs: i32) -> I32x8 {
        macro_rules! call {
            ($rhs:expr) => {
                unsafe { I32x8(_mm256_srai_epi32(self.0, $rhs)) }
            };
        }
        constify_imm8!(rhs, call)
    }
}
