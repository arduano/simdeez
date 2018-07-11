use super::*;

impl Shl<i32> for I32x4 {
    type Output = I32x4;

    #[inline(always)]
    fn shl(self, rhs: i32) -> I32x4 {
        macro_rules! call {
            ($rhs:expr) => {
                unsafe { I32x4(_mm_slli_epi32(self.0, $rhs)) }
            };
        }
        constify_imm8!(rhs, call)
    }
}
impl Shl<i32> for I32x4_41 {
    type Output = I32x4_41;

    #[inline(always)]
    fn shl(self, rhs: i32) -> I32x4_41 {
        macro_rules! call {
            ($rhs:expr) => {
                unsafe { I32x4_41(_mm_slli_epi32(self.0, $rhs)) }
            };
        }
        constify_imm8!(rhs, call)
    }
}
impl Shl<i32> for I32x8 {
    type Output = I32x8;

    #[inline(always)]
    fn shl(self, rhs: i32) -> I32x8 {
        macro_rules! call {
            ($rhs:expr) => {
                unsafe { I32x8(_mm256_slli_epi32(self.0, $rhs)) }
            };
        }
        constify_imm8!(rhs, call)
    }
}
