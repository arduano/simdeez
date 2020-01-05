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