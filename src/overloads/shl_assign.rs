use super::*;

impl ShlAssign<i32> for I16x1 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: i32) {
        *self = I16x1(self.0 << rhs);
    }
}

impl ShlAssign<i32> for I32x1 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: i32) {
        *self = I32x1(self.0 << rhs);
    }
}

impl ShlAssign<i32> for I64x1 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: i32) {
        *self = I64x1(self.0 << rhs);
    }
}

impl ShlAssign<i32> for F32x1 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: i32) {
        let bits = self.0.to_bits() << rhs;
        *self = F32x1(f32::from_bits(bits));
    }
}

impl ShlAssign<i32> for F64x1 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: i32) {
        let bits = self.0.to_bits() << rhs;
        *self = F64x1(f64::from_bits(bits));
    }
}

impl ShlAssign<i32> for I16x16 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: i32) {
        macro_rules! call {
            ($rhs:expr) => {
                *self = unsafe { I16x16(_mm256_slli_epi16(self.0, $rhs)) }
            };
        }
        constify_imm8!(rhs, call)
    }
}

impl ShlAssign<i32> for I32x4_41 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: i32) {
        macro_rules! call {
            ($rhs:expr) => {
                *self = unsafe { I32x4_41(_mm_slli_epi32(self.0, $rhs)) }
            };
        }
        constify_imm8!(rhs, call)
    }
}
impl ShlAssign<i32> for I32x8 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: i32) {
        macro_rules! call {
            ($rhs:expr) => {
                *self = unsafe { I32x8(_mm256_slli_epi32(self.0, $rhs)) }
            };
        }
        constify_imm8!(rhs, call)
    }
}
