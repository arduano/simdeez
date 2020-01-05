use super::*;

impl AddAssign for I16x8 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: I16x8) {
        *self = I16x8(unsafe { _mm_add_epi16(self.0, rhs.0) })
    }
}

impl AddAssign for I32x4 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: I32x4) {
        *self = I32x4(unsafe { _mm_add_epi32(self.0, rhs.0) })
    }
}

impl AddAssign for I64x2 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: I64x2) {
        *self = I64x2(unsafe { _mm_add_epi64(self.0, rhs.0) })
    }
}

impl AddAssign for F32x4 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: F32x4) {
        *self = F32x4(unsafe { _mm_add_ps(self.0, rhs.0) })
    }
}

impl AddAssign for F64x2 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: F64x2) {
        *self = F64x2(unsafe { _mm_add_pd(self.0, rhs.0) })
    }
}


impl Add for I16x8 {
    type Output = I16x8;

    #[inline(always)]
    fn add(self, rhs: I16x8) -> I16x8 {
        I16x8(unsafe { _mm_add_epi16(self.0, rhs.0) })
    }
}


impl Add for I32x4 {
    type Output = I32x4;

    #[inline(always)]
    fn add(self, rhs: I32x4) -> I32x4 {
        I32x4(unsafe { _mm_add_epi32(self.0, rhs.0) })
    }
}

impl Add for I64x2 {
    type Output = I64x2;

    #[inline(always)]
    fn add(self, rhs: I64x2) -> I64x2 {
        I64x2(unsafe { _mm_add_epi64(self.0, rhs.0) })
    }
}

impl Add for F32x4 {
    type Output = F32x4;

    #[inline(always)]
    fn add(self, rhs: F32x4) -> F32x4 {
        F32x4(unsafe { _mm_add_ps(self.0, rhs.0) })
    }
}

impl Add for F64x2 {
    type Output = F64x2;

    #[inline(always)]
    fn add(self, rhs: F64x2) -> F64x2 {
        F64x2(unsafe { _mm_add_pd(self.0, rhs.0) })
    }
}

impl BitAndAssign for I16x8 {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: I16x8) {
        *self = I16x8(unsafe { _mm_and_si128(self.0, rhs.0) })
    }
}

impl BitAndAssign for I32x4 {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: I32x4) {
        *self = I32x4(unsafe { _mm_and_si128(self.0, rhs.0) })
    }
}

impl BitAndAssign for I64x2 {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: I64x2) {
        *self = I64x2(unsafe { _mm_and_si128(self.0, rhs.0) })
    }
}

impl BitAndAssign for F32x4 {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: F32x4) {
        *self = F32x4(unsafe { _mm_and_ps(self.0, rhs.0) })
    }
}

impl BitAndAssign for F64x2 {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: F64x2) {
        *self = F64x2(unsafe { _mm_and_pd(self.0, rhs.0) })
    }
}

impl BitAnd for I16x8 {
    type Output = I16x8;

    #[inline(always)]
    fn bitand(self, rhs: I16x8) -> I16x8 {
        I16x8(unsafe { _mm_and_si128(self.0, rhs.0) })
    }
}
impl BitAnd for I32x4 {
    type Output = I32x4;

    #[inline(always)]
    fn bitand(self, rhs: I32x4) -> I32x4 {
        I32x4(unsafe { _mm_and_si128(self.0, rhs.0) })
    }
}

impl BitAnd for I64x2 {
    type Output = I64x2;

    #[inline(always)]
    fn bitand(self, rhs: I64x2) -> I64x2 {
        I64x2(unsafe { _mm_and_si128(self.0, rhs.0) })
    }
}

impl BitAnd for F32x4 {
    type Output = F32x4;

    #[inline(always)]
    fn bitand(self, rhs: F32x4) -> F32x4 {
        F32x4(unsafe { _mm_and_ps(self.0, rhs.0) })
    }
}

impl BitAnd for F64x2 {
    type Output = F64x2;

    #[inline(always)]
    fn bitand(self, rhs: F64x2) -> F64x2 {
        F64x2(unsafe { _mm_and_pd(self.0, rhs.0) })
    }
}

impl DivAssign for F32x4 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: F32x4) {
        *self = F32x4(unsafe { _mm_div_ps(self.0, rhs.0) })
    }
}

impl DivAssign for F64x2 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: F64x2) {
        *self = F64x2(unsafe { _mm_div_pd(self.0, rhs.0) })
    }
}


impl Div for F32x4 {
    type Output = F32x4;

    #[inline(always)]
    fn div(self, rhs: F32x4) -> F32x4 {
        F32x4(unsafe { _mm_div_ps(self.0, rhs.0) })
    }
}

impl Div for F64x2 {
    type Output = F64x2;

    #[inline(always)]
    fn div(self, rhs: F64x2) -> F64x2 {
        F64x2(unsafe { _mm_div_pd(self.0, rhs.0) })
    }
}


impl IndexMut<usize> for I16x8 {
    #[inline(always)]
    fn index_mut(&mut self, i: usize) -> &mut i16 {
        debug_assert!(i < 8);
        let arr = unsafe { mem::transmute::<&mut I16x8, &mut [i16; 8]>(self) };
        &mut arr[i]
    }
}

impl IndexMut<usize> for I32x4 {
    #[inline(always)]
    fn index_mut(&mut self, i: usize) -> &mut i32 {
        debug_assert!(i < 4);
        let arr = unsafe { mem::transmute::<&mut I32x4, &mut [i32; 4]>(self) };
        &mut arr[i]
    }
}

impl IndexMut<usize> for I64x2 {
    #[inline(always)]
    fn index_mut(&mut self, i: usize) -> &mut i64 {
        debug_assert!(i < 2);
        let arr = unsafe { mem::transmute::<&mut I64x2, &mut [i64; 2]>(self) };
        &mut arr[i]
    }
}

impl IndexMut<usize> for F32x4 {
    #[inline(always)]
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        debug_assert!(i < 4);
        let arr = unsafe { mem::transmute::<&mut F32x4, &mut [f32; 4]>(self) };
        &mut arr[i]
    }
}

impl IndexMut<usize> for F64x2 {
    #[inline(always)]
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        debug_assert!(i < 4);
        let arr = unsafe { mem::transmute::<&mut F64x2, &mut [f64; 2]>(self) };
        &mut arr[i]
    }
}

impl Index<usize> for I16x8 {
    type Output = i16;

    #[inline(always)]
    fn index(&self, i: usize) -> &i16 {
        let arr = unsafe { mem::transmute::<&I16x8, &[i16; 8]>(self) };
        &arr[i]
    }
}

impl Index<usize> for I32x4 {
    type Output = i32;

    #[inline(always)]
    fn index(&self, i: usize) -> &i32 {
        debug_assert!(i < 4);
        let arr = unsafe { mem::transmute::<&I32x4, &[i32; 4]>(self) };
        &arr[i]
    }
}
impl Index<usize> for I64x2 {
    type Output = i64;

    #[inline(always)]
    fn index(&self, i: usize) -> &i64 {
        debug_assert!(i < 2);
        let arr = unsafe { mem::transmute::<&I64x2, &[i64; 2]>(self) };
        &arr[i]
    }
}

impl Index<usize> for F32x4 {
    type Output = f32;

    #[inline(always)]
    fn index(&self, i: usize) -> &f32 {
        debug_assert!(i < 4);
        let arr = unsafe { mem::transmute::<&F32x4, &[f32; 4]>(self) };
        &arr[i]
    }
}
impl Index<usize> for F64x2 {
    type Output = f64;

    #[inline(always)]
    fn index(&self, i: usize) -> &f64 {
        debug_assert!(i < 4);
        let arr = unsafe { mem::transmute::<&F64x2, &[f64; 2]>(self) };
        &arr[i]
    }
}

impl MulAssign for I16x8 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: I16x8) {
        *self = I16x8(unsafe { _mm_mullo_epi16(self.0, rhs.0) })
    }
}

impl MulAssign for I32x4 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: I32x4) {
        let tmp1 = unsafe {
            _mm_mul_epu32(self.0, rhs.0) /* mul 2,0*/
        };
        let tmp2 = unsafe {
            _mm_mul_epu32(_mm_srli_si128(self.0, 4), _mm_srli_si128(rhs.0, 4)) /* mul 3,1 */
        };
        *self = I32x4(unsafe {
            _mm_unpacklo_epi32(
                _mm_shuffle_epi32(tmp1, mm_shuffle!(0, 0, 2, 0) as i32),
                _mm_shuffle_epi32(tmp2, mm_shuffle!(0, 0, 2, 0) as i32),
            )
        }) /* shuffle results to [63..0] and pack */
    }
}

impl MulAssign for F32x4 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: F32x4) {
        *self = F32x4(unsafe { _mm_mul_ps(self.0, rhs.0) })
    }
}

impl MulAssign for F64x2 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: F64x2) {
        *self = F64x2(unsafe { _mm_mul_pd(self.0, rhs.0) })
    }
}


impl Mul for I16x8 {
    type Output = I16x8;

    #[inline(always)]
    fn mul(self, rhs: I16x8) -> I16x8 {
        I16x8(unsafe { _mm_mullo_epi16(self.0, rhs.0) })
    }
}

impl Mul for I32x4 {
    type Output = I32x4;

    #[inline(always)]
    fn mul(self, rhs: I32x4) -> I32x4 {
        let tmp1 = unsafe {
            _mm_mul_epu32(self.0, rhs.0) /* mul 2,0*/
        };
        let tmp2 = unsafe {
            _mm_mul_epu32(_mm_srli_si128(self.0, 4), _mm_srli_si128(rhs.0, 4)) /* mul 3,1 */
        };
        I32x4(unsafe {
            _mm_unpacklo_epi32(
                _mm_shuffle_epi32(tmp1, mm_shuffle!(0, 0, 2, 0) as i32),
                _mm_shuffle_epi32(tmp2, mm_shuffle!(0, 0, 2, 0) as i32),
            )
        }) /* shuffle results to [63..0] and pack */
    }
}

impl Mul for F32x4 {
    type Output = F32x4;

    #[inline(always)]
    fn mul(self, rhs: F32x4) -> F32x4 {
        F32x4(unsafe { _mm_mul_ps(self.0, rhs.0) })
    }
}

impl Mul for F64x2 {
    type Output = F64x2;

    #[inline(always)]
    fn mul(self, rhs: F64x2) -> F64x2 {
        F64x2(unsafe { _mm_mul_pd(self.0, rhs.0) })
    }
}


impl Not for I16x8 {
    type Output = I16x8;

    #[inline(always)]
    fn not(self) -> I16x8 {
        unsafe { I16x8(_mm_xor_si128(self.0, _mm_set1_epi16(-1))) }
    }
}

impl Not for I32x4 {
    type Output = I32x4;

    #[inline(always)]
    fn not(self) -> I32x4 {
        unsafe { I32x4(_mm_xor_si128(self.0, _mm_set1_epi32(-1))) }
    }
}

impl Not for I64x2 {
    type Output = I64x2;

    #[inline(always)]
    fn not(self) -> I64x2 {
        unsafe { I64x2(_mm_xor_si128(self.0, _mm_set1_epi64x(-1))) }
    }
}

impl BitOrAssign for I16x8 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: I16x8) {
        *self = I16x8(unsafe { _mm_or_si128(self.0, rhs.0) })
    }
}
impl BitOrAssign for I32x4 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: I32x4) {
        *self = I32x4(unsafe { _mm_or_si128(self.0, rhs.0) })
    }
}
impl BitOrAssign for I64x2 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: I64x2) {
        *self = I64x2(unsafe { _mm_or_si128(self.0, rhs.0) })
    }
}
impl BitOrAssign for F32x4 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: F32x4) {
        *self = F32x4(unsafe { _mm_or_ps(self.0, rhs.0) })
    }
}
impl BitOrAssign for F64x2 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: F64x2) {
        *self = F64x2(unsafe { _mm_or_pd(self.0, rhs.0) })
    }
}

impl BitOr for I16x8 {
    type Output = I16x8;

    #[inline(always)]
    fn bitor(self, rhs: I16x8) -> I16x8 {
        I16x8(unsafe { _mm_or_si128(self.0, rhs.0) })
    }
}

impl BitOr for I32x4 {
    type Output = I32x4;

    #[inline(always)]
    fn bitor(self, rhs: I32x4) -> I32x4 {
        I32x4(unsafe { _mm_or_si128(self.0, rhs.0) })
    }
}

impl BitOr for I64x2 {
    type Output = I64x2;

    #[inline(always)]
    fn bitor(self, rhs: I64x2) -> I64x2 {
        I64x2(unsafe { _mm_or_si128(self.0, rhs.0) })
    }
}

impl BitOr for F32x4 {
    type Output = F32x4;

    #[inline(always)]
    fn bitor(self, rhs: F32x4) -> F32x4 {
        F32x4(unsafe { _mm_or_ps(self.0, rhs.0) })
    }
}

impl BitOr for F64x2 {
    type Output = F64x2;

    #[inline(always)]
    fn bitor(self, rhs: F64x2) -> F64x2 {
        F64x2(unsafe { _mm_or_pd(self.0, rhs.0) })
    }
}