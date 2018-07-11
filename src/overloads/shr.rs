use super::*;

impl Shr<i32> for I32x4 {
    type Output = I32x4;

    #[inline(always)]
    fn shr(self, rhs: i32) -> I32x4 {
        macro_rules! call {
            ($rhs:expr) => {
                unsafe { I32x4(_mm_srai_epi32(self.0, $rhs)) }
            };
        }
        constify_imm8!(rhs, call)
    }
}
impl Shr<i32> for I32x4_41 {
    type Output = I32x4_41;

    #[inline(always)]
    fn shr(self, rhs: i32) -> I32x4_41 {
        macro_rules! call {
            ($rhs:expr) => {
                unsafe { I32x4_41(_mm_srai_epi32(self.0, $rhs)) }
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
