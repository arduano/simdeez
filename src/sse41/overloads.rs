use super::*;

impl AddAssign for I32x4_41 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: I32x4_41) {
        *self = I32x4_41(unsafe { _mm_add_epi32(self.0, rhs.0) })
    }
}
impl AddAssign for I64x2_41 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: I64x2_41) {
        *self = I64x2_41(unsafe { _mm_add_epi64(self.0, rhs.0) })
    }
}

impl Add for I32x4_41 {
    type Output = I32x4_41;

    #[inline(always)]
    fn add(self, rhs: I32x4_41) -> I32x4_41 {
        I32x4_41(unsafe { _mm_add_epi32(self.0, rhs.0) })
    }
}
impl Add for I64x2_41 {
    type Output = I64x2_41;

    #[inline(always)]
    fn add(self, rhs: I64x2_41) -> I64x2_41 {
        I64x2_41(unsafe { _mm_add_epi64(self.0, rhs.0) })
    }
}

impl BitAndAssign for I32x4_41 {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: I32x4_41) {
        *self = I32x4_41(unsafe { _mm_and_si128(self.0, rhs.0) })
    }
}

impl BitAndAssign for I64x2_41 {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: I64x2_41) {
        *self = I64x2_41(unsafe { _mm_and_si128(self.0, rhs.0) })
    }
}

impl BitAnd for I32x4_41 {
    type Output = I32x4_41;

    #[inline(always)]
    fn bitand(self, rhs: I32x4_41) -> I32x4_41 {
        I32x4_41(unsafe { _mm_and_si128(self.0, rhs.0) })
    }
}
impl BitAnd for I64x2_41 {
    type Output = I64x2_41;

    #[inline(always)]
    fn bitand(self, rhs: I64x2_41) -> I64x2_41 {
        I64x2_41(unsafe { _mm_and_si128(self.0, rhs.0) })
    }
}

impl IndexMut<usize> for I32x4_41 {
    #[inline(always)]
    fn index_mut(&mut self, i: usize) -> &mut i32 {
        debug_assert!(i < 4);
        let arr = unsafe { mem::transmute::<&mut I32x4_41, &mut [i32; 4]>(self) };
        &mut arr[i]
    }
}

impl IndexMut<usize> for I64x2_41 {
    #[inline(always)]
    fn index_mut(&mut self, i: usize) -> &mut i64 {
        debug_assert!(i < 2);
        let arr = unsafe { mem::transmute::<&mut I64x2_41, &mut [i64; 2]>(self) };
        &mut arr[i]
    }
}

impl Index<usize> for I32x4_41 {
    type Output = i32;

    #[inline(always)]
    fn index(&self, i: usize) -> &i32 {
        debug_assert!(i < 4);
        let arr = unsafe { mem::transmute::<&I32x4_41, &[i32; 4]>(self) };
        &arr[i]
    }
}
impl Index<usize> for I64x2_41 {
    type Output = i64;

    #[inline(always)]
    fn index(&self, i: usize) -> &i64 {
        debug_assert!(i < 2);
        let arr = unsafe { mem::transmute::<&I64x2_41, &[i64; 2]>(self) };
        &arr[i]
    }
}

impl MulAssign for I32x4_41 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: I32x4_41) {
        *self = I32x4_41(unsafe { _mm_mullo_epi32(self.0, rhs.0) })
    }
}

impl Mul for I32x4_41 {
    type Output = I32x4_41;

    #[inline(always)]
    fn mul(self, rhs: I32x4_41) -> I32x4_41 {
        I32x4_41(unsafe { _mm_mullo_epi32(self.0, rhs.0) })
    }
}

impl Not for I32x4_41 {
    type Output = I32x4_41;

    #[inline(always)]
    fn not(self) -> I32x4_41 {
        unsafe { I32x4_41(_mm_xor_si128(self.0, _mm_set1_epi32(-1))) }
    }
}
impl Not for I64x2_41 {
    type Output = I64x2_41;

    #[inline(always)]
    fn not(self) -> I64x2_41 {
        unsafe { I64x2_41(_mm_xor_si128(self.0, _mm_set1_epi64x(-1))) }
    }
}

impl BitOrAssign for I32x4_41 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: I32x4_41) {
        *self = I32x4_41(unsafe { _mm_or_si128(self.0, rhs.0) })
    }
}

impl BitOrAssign for I64x2_41 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: I64x2_41) {
        *self = I64x2_41(unsafe { _mm_or_si128(self.0, rhs.0) })
    }
}

impl BitOr for I32x4_41 {
    type Output = I32x4_41;

    #[inline(always)]
    fn bitor(self, rhs: I32x4_41) -> I32x4_41 {
        I32x4_41(unsafe { _mm_or_si128(self.0, rhs.0) })
    }
}

impl BitOr for I64x2_41 {
    type Output = I64x2_41;

    #[inline(always)]
    fn bitor(self, rhs: I64x2_41) -> I64x2_41 {
        I64x2_41(unsafe { _mm_or_si128(self.0, rhs.0) })
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

impl ShrAssign<i32> for I32x4_41 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: i32) {
        macro_rules! call {
            ($rhs:expr) => {
                *self = unsafe { I32x4_41(_mm_srai_epi32(self.0, $rhs)) }
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

impl SubAssign for I32x4_41 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: I32x4_41) {
        *self = I32x4_41(unsafe { _mm_sub_epi32(self.0, rhs.0) })
    }
}

impl SubAssign for I64x2_41 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: I64x2_41) {
        *self = I64x2_41(unsafe { _mm_sub_epi64(self.0, rhs.0) })
    }
}

impl Sub for I32x4_41 {
    type Output = I32x4_41;

    #[inline(always)]
    fn sub(self, rhs: I32x4_41) -> I32x4_41 {
        I32x4_41(unsafe { _mm_sub_epi32(self.0, rhs.0) })
    }
}

impl Sub for I64x2_41 {
    type Output = I64x2_41;

    #[inline(always)]
    fn sub(self, rhs: I64x2_41) -> I64x2_41 {
        I64x2_41(unsafe { _mm_sub_epi64(self.0, rhs.0) })
    }
}

impl BitXorAssign for I32x4_41 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: I32x4_41) {
        *self = I32x4_41(unsafe { _mm_xor_si128(self.0, rhs.0) })
    }
}

impl BitXorAssign for I64x2_41 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: I64x2_41) {
        *self = I64x2_41(unsafe { _mm_xor_si128(self.0, rhs.0) })
    }
}

impl BitXor for I32x4_41 {
    type Output = I32x4_41;

    #[inline(always)]
    fn bitxor(self, rhs: I32x4_41) -> I32x4_41 {
        I32x4_41(unsafe { _mm_xor_si128(self.0, rhs.0) })
    }
}

impl BitXor for I64x2_41 {
    type Output = I64x2_41;

    #[inline(always)]
    fn bitxor(self, rhs: I64x2_41) -> I64x2_41 {
        I64x2_41(unsafe { _mm_xor_si128(self.0, rhs.0) })
    }
}
