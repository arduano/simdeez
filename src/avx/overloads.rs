use crate::sse2::{I16x8, I32x4, I64x2};
use super::*;


impl AddAssign for I16x16 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: I16x16) {
        unsafe {
            *self = m256i_run_on_halves_2_simd!(Add::add, self, rhs, I16x8, Self);
        }
    }
}

impl AddAssign for I32x8 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: I32x8) {
        unsafe {
            *self = m256i_run_on_halves_2_simd!(Add::add, self, rhs, I32x4, Self);
        }
    }
}

impl AddAssign for I64x4 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: I64x4) {
        unsafe {
            *self = m256i_run_on_halves_2_simd!(Add::add, self, rhs, I64x2, Self);
        }
    }
}
impl AddAssign for F32x8 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: F32x8) {
        *self = F32x8(unsafe { _mm256_add_ps(self.0, rhs.0) })
    }
}
impl AddAssign<F64x4> for F64x4 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: F64x4) {
        *self = F64x4(unsafe { _mm256_add_pd(self.0, rhs.0) })
    }
}

impl Add for I16x16 {
    type Output = I16x16;

    #[inline(always)]
    fn add(self, rhs: I16x16) -> I16x16 {
        unsafe {
            m256i_run_on_halves_2_simd!(Add::add, self, rhs, I16x8, I16x16)
        }
    }
}


impl Add for I32x8 {
    type Output = I32x8;

    #[inline(always)]
    fn add(self, rhs: I32x8) -> I32x8 {
        unsafe {
            m256i_run_on_halves_2_simd!(Add::add, self, rhs, I32x4, I32x8)
        }
    }
}


impl Add for I64x4 {
    type Output = I64x4;

    #[inline(always)]
    fn add(self, rhs: I64x4) -> I64x4 {
        unsafe {
            m256i_run_on_halves_2_simd!(Add::add, self, rhs, I64x2, Self)
        }
    }
}

impl Add for F32x8 {
    type Output = F32x8;

    #[inline(always)]
    fn add(self, rhs: F32x8) -> F32x8 {
        F32x8(unsafe { _mm256_add_ps(self.0, rhs.0) })
    }
}

impl Add for F64x4 {
    type Output = F64x4;

    #[inline(always)]
    fn add(self, rhs: F64x4) -> F64x4 {
        F64x4(unsafe { _mm256_add_pd(self.0, rhs.0) })
    }
}
impl BitAndAssign for I16x16 {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: I16x16) {
        unsafe {
            *self = m256i_run_on_halves_2_simd!(BitAnd::bitand, self, rhs, I16x8, Self);
        }
    }
}

impl BitAndAssign for I32x8 {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: I32x8) {
        unsafe {
            *self = m256i_run_on_halves_2_simd!(BitAnd::bitand, self, rhs, I32x4, Self);
        }
    }
}


impl BitAndAssign for I64x4 {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: I64x4) {
        unsafe {
            *self = m256i_run_on_halves_2_simd!(BitAnd::bitand, self, rhs, I64x2, Self);
        }
    }
}

impl BitAndAssign for F32x8 {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: F32x8) {
        *self = F32x8(unsafe { _mm256_and_ps(self.0, rhs.0) })
    }
}

impl BitAndAssign for F64x4 {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: F64x4) {
        *self = F64x4(unsafe { _mm256_and_pd(self.0, rhs.0) })
    }
}
impl BitAnd for I16x16 {
    type Output = I16x16;

    #[inline(always)]
    fn bitand(self, rhs: I16x16) -> I16x16 {
        unsafe {
            m256i_run_on_halves_2_simd!(BitAnd::bitand, self, rhs, I16x8, Self)
        }
    }
}

impl BitAnd for I32x8 {
    type Output = I32x8;

    #[inline(always)]
    fn bitand(self, rhs: I32x8) -> I32x8 {
        unsafe {
            m256i_run_on_halves_2_simd!(BitAnd::bitand, self, rhs, I32x4, Self)
        }
    }
}


impl BitAnd for I64x4 {
    type Output = I64x4;

    #[inline(always)]
    fn bitand(self, rhs: I64x4) -> I64x4 {
        unsafe {
            m256i_run_on_halves_2_simd!(BitAnd::bitand, self, rhs, I64x2, Self)
        }
    }
}
impl BitAnd for F32x8 {
    type Output = F32x8;

    #[inline(always)]
    fn bitand(self, rhs: F32x8) -> F32x8 {
        F32x8(unsafe { _mm256_and_ps(self.0, rhs.0) })
    }
}

impl BitAnd for F64x4 {
    type Output = F64x4;

    #[inline(always)]
    fn bitand(self, rhs: F64x4) -> F64x4 {
        F64x4(unsafe { _mm256_and_pd(self.0, rhs.0) })
    }
}

impl DivAssign for F32x8 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: F32x8) {
        *self = F32x8(unsafe { _mm256_div_ps(self.0, rhs.0) })
    }
}

impl DivAssign for F64x4 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: F64x4) {
        *self = F64x4(unsafe { _mm256_div_pd(self.0, rhs.0) })
    }
}
impl Div for F32x8 {
    type Output = F32x8;

    #[inline(always)]
    fn div(self, rhs: F32x8) -> F32x8 {
        F32x8(unsafe { _mm256_div_ps(self.0, rhs.0) })
    }
}

impl Div for F64x4 {
    type Output = F64x4;

    #[inline(always)]
    fn div(self, rhs: F64x4) -> F64x4 {
        F64x4(unsafe { _mm256_div_pd(self.0, rhs.0) })
    }
}

impl IndexMut<usize> for I16x16 {
    #[inline(always)]
    fn index_mut(&mut self, i: usize) -> &mut i16 {
        debug_assert!(i < 16);
        let arr = unsafe { mem::transmute::<&mut I16x16, &mut [i16; 16]>(self) };
        &mut arr[i]
    }
}


impl IndexMut<usize> for I32x8 {
    #[inline(always)]
    fn index_mut(&mut self, i: usize) -> &mut i32 {
        debug_assert!(i < 8);
        let arr = unsafe { mem::transmute::<&mut I32x8, &mut [i32; 8]>(self) };
        &mut arr[i]
    }
}


impl IndexMut<usize> for I64x4 {
    #[inline(always)]
    fn index_mut(&mut self, i: usize) -> &mut i64 {
        debug_assert!(i < 4);
        let arr = unsafe { mem::transmute::<&mut I64x4, &mut [i64; 4]>(self) };
        &mut arr[i]
    }
}

impl IndexMut<usize> for F32x8 {
    #[inline(always)]
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        debug_assert!(i < 8);
        let arr = unsafe { mem::transmute::<&mut F32x8, &mut [f32; 8]>(self) };
        &mut arr[i]
    }
}
impl IndexMut<usize> for F64x4 {
    #[inline(always)]
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        debug_assert!(i < 4);
        let arr = unsafe { mem::transmute::<&mut F64x4, &mut [f64; 4]>(self) };
        &mut arr[i]
    }
}


impl Index<usize> for I16x16 {
    type Output = i16;

    #[inline(always)]
    fn index(&self, i: usize) -> &i16 {
        let arr = unsafe { mem::transmute::<&I16x16, &[i16; 16]>(self) };
        &arr[i]
    }
}
impl Index<usize> for I32x8 {
    type Output = i32;

    #[inline(always)]
    fn index(&self, i: usize) -> &i32 {
        debug_assert!(i < 8);
        let arr = unsafe { mem::transmute::<&I32x8, &[i32; 8]>(self) };
        &arr[i]
    }
}

impl Index<usize> for I64x4 {
    type Output = i64;

    #[inline(always)]
    fn index(&self, i: usize) -> &i64 {
        debug_assert!(i < 4);
        let arr = unsafe { mem::transmute::<&I64x4, &[i64; 4]>(self) };
        &arr[i]
    }
}

impl Index<usize> for F32x8 {
    type Output = f32;

    #[inline(always)]
    fn index(&self, i: usize) -> &f32 {
        debug_assert!(i < 8);
        let arr = unsafe { mem::transmute::<&F32x8, &[f32; 8]>(self) };
        &arr[i]
    }
}

impl Index<usize> for F64x4 {
    type Output = f64;

    #[inline(always)]
    fn index(&self, i: usize) -> &f64 {
        debug_assert!(i < 4);
        let arr = unsafe { mem::transmute::<&F64x4, &[f64; 4]>(self) };
        &arr[i]
    }
}


impl MulAssign for I16x16 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: I16x16) {
        unsafe {
            *self = m256i_run_on_halves_2_simd!(Mul::mul, self, rhs, I16x8, Self);
        }
    }
}

impl MulAssign for I32x8 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: I32x8) {
        unsafe {
            *self = m256i_run_on_halves_2_simd!(Mul::mul, self, rhs, I32x4, Self);
        }
    }
}

impl MulAssign for F32x8 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: F32x8) {
        *self = F32x8(unsafe { _mm256_mul_ps(self.0, rhs.0) })
    }
}

impl MulAssign for F64x4 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: F64x4) {
        *self = F64x4(unsafe { _mm256_mul_pd(self.0, rhs.0) })
    }
}
impl Mul for I16x16 {
    type Output = I16x16;

    #[inline(always)]
    fn mul(self, rhs: I16x16) -> I16x16 {
        unsafe {
            m256i_run_on_halves_2_simd!(Mul::mul, self, rhs, I16x8, Self)
        }
    }
}

impl Mul for I32x8 {
    type Output = I32x8;
    #[inline(always)]
    fn mul(self, rhs: I32x8) -> I32x8 {
        unsafe {
            m256i_run_on_halves_2_simd!(Mul::mul, self, rhs, I32x4, Self)
        }
    }
}

impl Mul for F32x8 {
    type Output = F32x8;
    #[inline(always)]
    fn mul(self, rhs: F32x8) -> F32x8 {
        F32x8(unsafe { _mm256_mul_ps(self.0, rhs.0) })
    }
}

impl Mul for F64x4 {
    type Output = F64x4;
    #[inline(always)]
    fn mul(self, rhs: F64x4) -> F64x4 {
        F64x4(unsafe { _mm256_mul_pd(self.0, rhs.0) })
    }
}
impl Not for I16x16 {
    type Output = I16x16;

    #[inline(always)]
    fn not(self) -> I16x16 {
        unsafe {
            m256i_run_on_halves_1_simd!(Not::not, self, I16x8, Self)
        }
    }
}


impl Not for I32x8 {
    type Output = I32x8;

    #[inline(always)]
    fn not(self) -> I32x8 {
        unsafe {
            m256i_run_on_halves_1_simd!(Not::not, self, I32x4, Self)
        }
    }
}


impl Not for I64x4 {
    type Output = I64x4;

    #[inline(always)]
    fn not(self) -> I64x4 {
        unsafe {
            m256i_run_on_halves_1_simd!(Not::not, self, I64x2, Self)
        }
    }
}
impl BitOrAssign for I16x16 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: I16x16) {
        unsafe {
            *self = m256i_run_on_halves_2_simd!(BitOr::bitor, self, rhs, I16x8, Self);
        }
    }
}
impl BitOrAssign for I32x8 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: I32x8) {
        unsafe {
            *self = m256i_run_on_halves_2_simd!(BitOr::bitor, self, rhs, I32x4, Self);
        }
    }
}


impl BitOrAssign for I64x4 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: I64x4) {
        unsafe {
            *self = m256i_run_on_halves_2_simd!(BitOr::bitor, self, rhs, I64x2, Self);
        }
    }
}

impl BitOrAssign for F32x8 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: F32x8) {
        *self = F32x8(unsafe { _mm256_or_ps(self.0, rhs.0) })
    }
}

impl BitOrAssign for F64x4 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: F64x4) {
        *self = F64x4(unsafe { _mm256_or_pd(self.0, rhs.0) })
    }
}

impl BitOr for I16x16 {
    type Output = I16x16;

    #[inline(always)]
    fn bitor(self, rhs: I16x16) -> I16x16 {
        unsafe {
            m256i_run_on_halves_2_simd!(BitOr::bitor, self, rhs, I16x8, Self)
        }
    }
}


impl BitOr for I32x8 {
    type Output = I32x8;

    #[inline(always)]
    fn bitor(self, rhs: I32x8) -> I32x8 {
        unsafe {
            m256i_run_on_halves_2_simd!(BitOr::bitor, self, rhs, I32x4, Self)
        }
    }
}


impl BitOr for I64x4 {
    type Output = I64x4;

    #[inline(always)]
    fn bitor(self, rhs: I64x4) -> I64x4 {
        unsafe {
            m256i_run_on_halves_2_simd!(BitOr::bitor, self, rhs, I64x2, Self)
        }
    }
}

impl BitOr for F32x8 {
    type Output = F32x8;

    #[inline(always)]
    fn bitor(self, rhs: F32x8) -> F32x8 {
        F32x8(unsafe { _mm256_or_ps(self.0, rhs.0) })
    }
}

impl BitOr for F64x4 {
    type Output = F64x4;

    #[inline(always)]
    fn bitor(self, rhs: F64x4) -> F64x4 {
        F64x4(unsafe { _mm256_or_pd(self.0, rhs.0) })
    }
}
impl ShlAssign<i32> for I16x16 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: i32) {
        unsafe {
            *self = m256i_run_on_halves_1_simd!(Shl::shl, self, rhs, I16x8, Self);
        }
    }
}


impl ShlAssign<i32> for I32x8 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: i32) {
        unsafe {
            *self = m256i_run_on_halves_1_simd!(Shl::shl, self, rhs, I32x4, Self);
        }
    }
}
impl Shl<i32> for I16x16 {
    type Output = I16x16;

    #[inline(always)]
    fn shl(self, rhs: i32) -> I16x16 {
        unsafe {
            m256i_run_on_halves_1_simd!(Shl::shl, self, rhs, I16x8, Self)
        }
    }
}


impl Shl<i32> for I32x8 {
    type Output = I32x8;

    #[inline(always)]
    fn shl(self, rhs: i32) -> I32x8 {
        unsafe {
            m256i_run_on_halves_1_simd!(Shl::shl, self, rhs, I32x4, Self)
        }
    }
}

impl ShrAssign<i32> for I16x16 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: i32) {
        unsafe {
            *self = m256i_run_on_halves_1_simd!(Shr::shr, self, rhs, I16x8, Self);
        }
    }
}



impl ShrAssign<i32> for I32x8 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: i32) {
        unsafe {
            *self = m256i_run_on_halves_1_simd!(Shr::shr, self, rhs, I32x4, Self);
        }
    }
}
impl Shr<i32> for I16x16 {
    type Output = I16x16;

    #[inline(always)]
    fn shr(self, rhs: i32) -> I16x16 {
        unsafe {
            m256i_run_on_halves_1_simd!(Shr::shr, self, rhs, I16x8, Self)
        }
    }
}

impl Shr<i32> for I32x8 {
    type Output = I32x8;

    #[inline(always)]
    fn shr(self, rhs: i32) -> I32x8 {
        unsafe {
            m256i_run_on_halves_1_simd!(Shr::shr, self, rhs, I32x4, Self)
        }
    }
}
impl SubAssign for I16x16 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: I16x16) {
        unsafe {
            *self = m256i_run_on_halves_2_simd!(Sub::sub, self, rhs, I16x8, Self);
        }
    }
}
impl SubAssign for I32x8 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: I32x8) {
        unsafe {
            *self = m256i_run_on_halves_2_simd!(Sub::sub, self, rhs, I32x4, Self);
        }
    }
}


impl SubAssign for I64x4 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: I64x4) {
        unsafe {
            *self = m256i_run_on_halves_2_simd!(Sub::sub, self, rhs, I64x2, Self);
        }
    }
}

impl SubAssign for F32x8 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: F32x8) {
        *self = F32x8(unsafe { _mm256_sub_ps(self.0, rhs.0) })
    }
}

impl SubAssign for F64x4 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: F64x4) {
        *self = F64x4(unsafe { _mm256_sub_pd(self.0, rhs.0) })
    }
}
impl Sub for I16x16 {
    type Output = I16x16;

    #[inline(always)]
    fn sub(self, rhs: I16x16) -> I16x16 {
        unsafe {
            m256i_run_on_halves_2_simd!(Sub::sub, self, rhs, I16x8, Self)
        }
    }
}
impl Sub for I32x8 {
    type Output = I32x8;

    #[inline(always)]
    fn sub(self, rhs: I32x8) -> I32x8 {
        unsafe {
            m256i_run_on_halves_2_simd!(Sub::sub, self, rhs, I32x4, Self)
        }
    }
}
impl Sub for I64x4 {
    type Output = I64x4;

    #[inline(always)]
    fn sub(self, rhs: I64x4) -> I64x4 {
        unsafe {
            m256i_run_on_halves_2_simd!(Sub::sub, self, rhs, I64x2, Self)
        }
    }
}

impl Sub for F32x8 {
    type Output = F32x8;

    #[inline(always)]
    fn sub(self, rhs: F32x8) -> F32x8 {
        F32x8(unsafe { _mm256_sub_ps(self.0, rhs.0) })
    }
}

impl Sub for F64x4 {
    type Output = F64x4;

    #[inline(always)]
    fn sub(self, rhs: F64x4) -> F64x4 {
        F64x4(unsafe { _mm256_sub_pd(self.0, rhs.0) })
    }
}

impl BitXorAssign for I16x16 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: I16x16) {
        unsafe {
            *self = m256i_run_on_halves_2_simd!(BitXor::bitxor, self, rhs, I16x8, Self);
        }
    }
}

impl BitXorAssign for I32x8 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: I32x8) {
        unsafe {
            *self = m256i_run_on_halves_2_simd!(BitXor::bitxor, self, rhs, I32x4, Self);
        }
    }
}


impl BitXorAssign for I64x4 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: I64x4) {
        unsafe {
            *self = m256i_run_on_halves_2_simd!(BitXor::bitxor, self, rhs, I64x2, Self);
        }
    }
}

impl BitXorAssign for F32x8 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: F32x8) {
        *self = F32x8(unsafe { _mm256_xor_ps(self.0, rhs.0) })
    }
}

impl BitXorAssign for F64x4 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: F64x4) {
        *self = F64x4(unsafe { _mm256_xor_pd(self.0, rhs.0) })
    }
}
impl BitXor for I16x16 {
    type Output = I16x16;

    #[inline(always)]
    fn bitxor(self, rhs: I16x16) -> I16x16 {
        unsafe {
            m256i_run_on_halves_2_simd!(BitXor::bitxor, self, rhs, I16x8, Self)
        }
    }
}

impl BitXor for I32x8 {
    type Output = I32x8;

    #[inline(always)]
    fn bitxor(self, rhs: I32x8) -> I32x8 {
        unsafe {
            m256i_run_on_halves_2_simd!(BitXor::bitxor, self, rhs, I32x4, Self)
        }
    }
}


impl BitXor for I64x4 {
    type Output = I64x4;

    #[inline(always)]
    fn bitxor(self, rhs: I64x4) -> I64x4 {
        unsafe {
            m256i_run_on_halves_2_simd!(BitXor::bitxor, self, rhs, I64x2, Self)
        }
    }
}

impl BitXor for F32x8 {
    type Output = F32x8;

    #[inline(always)]
    fn bitxor(self, rhs: F32x8) -> F32x8 {
        F32x8(unsafe { _mm256_xor_ps(self.0, rhs.0) })
    }
}

impl BitXor for F64x4 {
    type Output = F64x4;

    #[inline(always)]
    fn bitxor(self, rhs: F64x4) -> F64x4 {
        F64x4(unsafe { _mm256_xor_pd(self.0, rhs.0) })
    }
}
