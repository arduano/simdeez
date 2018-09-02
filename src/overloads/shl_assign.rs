use super::*;

impl ShlAssign<i32> for I16x8 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: i32) {
        macro_rules! call {
            ($rhs:expr) => {
                *self = unsafe { I16x8(_mm_slli_epi16(self.0, $rhs)) }
            };
        }
        constify_imm8!(rhs, call)
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
impl ShlAssign<i32> for I32x4 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: i32) {
        macro_rules! call {
            ($rhs:expr) => {
                *self = unsafe { I32x4(_mm_slli_epi32(self.0, $rhs)) }
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
