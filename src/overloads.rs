#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
use std::mem;
use std::ops::*;

// Newtypes for i32 vectors
// We have to do this to allow for overloading of
// __m128 etc
#[derive(Copy, Debug, Clone)]
pub struct I32x1(pub i32);
#[derive(Copy, Debug, Clone)]
pub struct I32x4(pub __m128i);
#[derive(Copy, Debug, Clone)]
pub struct I32x4_41(pub __m128i);
#[derive(Copy, Debug, Clone)]
pub struct I32x8(pub __m256i);

// Newtypes for i32 vectors
// We have to do this to allow for overloading of
// __m128i etc
#[derive(Copy, Debug, Clone)]
pub struct F32x1(pub f32);
#[derive(Copy, Debug, Clone)]
pub struct F32x4(pub __m128);
#[derive(Copy, Debug, Clone)]
pub struct F32x8(pub __m256);

// -- IndexMut
impl IndexMut<usize> for I32x1 {
    fn index_mut(&mut self, i: usize) -> &mut i32 {
        debug_assert!(i == 0);
        let arr = unsafe { mem::transmute::<&mut I32x1, &mut [i32; 1]>(self) };
        &mut arr[i]
    }
}
impl IndexMut<usize> for F32x1 {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        debug_assert!(i == 0);
        let arr = unsafe { mem::transmute::<&mut F32x1, &mut [f32; 1]>(self) };
        &mut arr[i]
    }
}
impl IndexMut<usize> for I32x4 {
    fn index_mut(&mut self, i: usize) -> &mut i32 {
        debug_assert!(i < 4);
        let arr = unsafe { mem::transmute::<&mut I32x4, &mut [i32; 4]>(self) };
        &mut arr[i]
    }
}
impl IndexMut<usize> for I32x4_41 {
    fn index_mut(&mut self, i: usize) -> &mut i32 {
        debug_assert!(i < 4);
        let arr = unsafe { mem::transmute::<&mut I32x4_41, &mut [i32; 4]>(self) };
        &mut arr[i]
    }
}
impl IndexMut<usize> for I32x8 {
    fn index_mut(&mut self, i: usize) -> &mut i32 {
        debug_assert!(i < 8);
        let arr = unsafe { mem::transmute::<&mut I32x8, &mut [i32; 8]>(self) };
        &mut arr[i]
    }
}
impl IndexMut<usize> for F32x4 {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        debug_assert!(i < 4);
        let arr = unsafe { mem::transmute::<&mut F32x4, &mut [f32; 4]>(self) };
        &mut arr[i]
    }
}
impl IndexMut<usize> for F32x8 {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        debug_assert!(i < 8);
        let arr = unsafe { mem::transmute::<&mut F32x8, &mut [f32; 8]>(self) };
        &mut arr[i]
    }
}
// -- Index
impl Index<usize> for I32x1 {
    type Output = i32;

    fn index(&self, i: usize) -> &i32 {
        debug_assert!(i == 0);
        let arr = unsafe { mem::transmute::<&I32x1, &[i32; 1]>(self) };
        &arr[i]
    }
}
impl Index<usize> for F32x1 {
    type Output = f32;

    fn index(&self, i: usize) -> &f32 {
        debug_assert!(i == 0);
        let arr = unsafe { mem::transmute::<&F32x1, &[f32; 1]>(self) };
        &arr[i]
    }
}
impl Index<usize> for I32x4 {
    type Output = i32;

    fn index(&self, i: usize) -> &i32 {
        debug_assert!(i < 4);
        let arr = unsafe { mem::transmute::<&I32x4, &[i32; 4]>(self) };
        &arr[i]
    }
}
impl Index<usize> for I32x4_41 {
    type Output = i32;

    fn index(&self, i: usize) -> &i32 {
        debug_assert!(i < 4);
        let arr = unsafe { mem::transmute::<&I32x4_41, &[i32; 4]>(self) };
        &arr[i]
    }
}
impl Index<usize> for I32x8 {
    type Output = i32;

    fn index(&self, i: usize) -> &i32 {
        debug_assert!(i < 8);
        let arr = unsafe { mem::transmute::<&I32x8, &[i32; 8]>(self) };
        &arr[i]
    }
}
impl Index<usize> for F32x4 {
    type Output = f32;

    fn index(&self, i: usize) -> &f32 {
        debug_assert!(i < 4);
        let arr = unsafe { mem::transmute::<&F32x4, &[f32; 4]>(self) };
        &arr[i]
    }
}
impl Index<usize> for F32x8 {
    type Output = f32;

    fn index(&self, i: usize) -> &f32 {
        debug_assert!(i < 8);
        let arr = unsafe { mem::transmute::<&F32x8, &[f32; 8]>(self) };
        &arr[i]
    }
}
// -- Add
impl Add for I32x1 {
    type Output = I32x1;

    fn add(self, rhs: I32x1) -> I32x1 {
        I32x1(self.0 + rhs.0)
    }
}
impl Add for I32x4 {
    type Output = I32x4;

    fn add(self, rhs: I32x4) -> I32x4 {
        I32x4(unsafe { _mm_add_epi32(self.0, rhs.0) })
    }
}
impl Add for I32x4_41 {
    type Output = I32x4_41;

    fn add(self, rhs: I32x4_41) -> I32x4_41 {
        I32x4_41(unsafe { _mm_add_epi32(self.0, rhs.0) })
    }
}
impl Add for I32x8 {
    type Output = I32x8;

    fn add(self, rhs: I32x8) -> I32x8 {
        I32x8(unsafe { _mm256_add_epi32(self.0, rhs.0) })
    }
}
impl Add for F32x1 {
    type Output = F32x1;

    fn add(self, rhs: F32x1) -> F32x1 {
        F32x1(self.0 + rhs.0)
    }
}
impl Add for F32x4 {
    type Output = F32x4;

    fn add(self, rhs: F32x4) -> F32x4 {
        F32x4(unsafe { _mm_add_ps(self.0, rhs.0) })
    }
}
impl Add for F32x8 {
    type Output = F32x8;

    fn add(self, rhs: F32x8) -> F32x8 {
        F32x8(unsafe { _mm256_add_ps(self.0, rhs.0) })
    }
}
// -- AddAssign
impl AddAssign for I32x1 {
    fn add_assign(&mut self, rhs: I32x1) {
        *self = I32x1(self.0 + rhs.0)
    }
}
impl AddAssign for I32x4 {
    fn add_assign(&mut self, rhs: I32x4) {
        *self = I32x4(unsafe { _mm_add_epi32(self.0, rhs.0) })
    }
}
impl AddAssign for I32x4_41 {
    fn add_assign(&mut self, rhs: I32x4_41) {
        *self = I32x4_41(unsafe { _mm_add_epi32(self.0, rhs.0) })
    }
}
impl AddAssign for I32x8 {
    fn add_assign(&mut self, rhs: I32x8) {
        *self = I32x8(unsafe { _mm256_add_epi32(self.0, rhs.0) })
    }
}
impl AddAssign for F32x1 {
    fn add_assign(&mut self, rhs: F32x1) {
        *self = F32x1(self.0 + rhs.0)
    }
}
impl AddAssign for F32x4 {
    fn add_assign(&mut self, rhs: F32x4) {
        *self = F32x4(unsafe { _mm_add_ps(self.0, rhs.0) })
    }
}
impl AddAssign for F32x8 {
    fn add_assign(&mut self, rhs: F32x8) {
        *self = F32x8(unsafe { _mm256_add_ps(self.0, rhs.0) })
    }
}
// -- Sub
impl Sub for I32x1 {
    type Output = I32x1;

    fn sub(self, rhs: I32x1) -> I32x1 {
        I32x1(self.0 - rhs.0)
    }
}
impl Sub for I32x4 {
    type Output = I32x4;

    fn sub(self, rhs: I32x4) -> I32x4 {
        I32x4(unsafe { _mm_sub_epi32(self.0, rhs.0) })
    }
}
impl Sub for I32x4_41 {
    type Output = I32x4_41;

    fn sub(self, rhs: I32x4_41) -> I32x4_41 {
        I32x4_41(unsafe { _mm_sub_epi32(self.0, rhs.0) })
    }
}
impl Sub for I32x8 {
    type Output = I32x8;

    fn sub(self, rhs: I32x8) -> I32x8 {
        I32x8(unsafe { _mm256_sub_epi32(self.0, rhs.0) })
    }
}
impl Sub for F32x1 {
    type Output = F32x1;

    fn sub(self, rhs: F32x1) -> F32x1 {
        F32x1(self.0 - rhs.0)
    }
}
impl Sub for F32x4 {
    type Output = F32x4;

    fn sub(self, rhs: F32x4) -> F32x4 {
        F32x4(unsafe { _mm_sub_ps(self.0, rhs.0) })
    }
}
impl Sub for F32x8 {
    type Output = F32x8;

    fn sub(self, rhs: F32x8) -> F32x8 {
        F32x8(unsafe { _mm256_sub_ps(self.0, rhs.0) })
    }
}
// -- SubAssign
impl SubAssign for I32x1 {
    fn sub_assign(&mut self, rhs: I32x1) {
        *self = I32x1(self.0 - rhs.0)
    }
}
impl SubAssign for I32x4 {
    fn sub_assign(&mut self, rhs: I32x4) {
        *self = I32x4(unsafe { _mm_sub_epi32(self.0, rhs.0) })
    }
}
impl SubAssign for I32x4_41 {
    fn sub_assign(&mut self, rhs: I32x4_41) {
        *self = I32x4_41(unsafe { _mm_sub_epi32(self.0, rhs.0) })
    }
}
impl SubAssign for I32x8 {
    fn sub_assign(&mut self, rhs: I32x8) {
        *self = I32x8(unsafe { _mm256_sub_epi32(self.0, rhs.0) })
    }
}
impl SubAssign for F32x1 {
    fn sub_assign(&mut self, rhs: F32x1) {
        *self = F32x1(self.0 - rhs.0)
    }
}
impl SubAssign for F32x4 {
    fn sub_assign(&mut self, rhs: F32x4) {
        *self = F32x4(unsafe { _mm_sub_ps(self.0, rhs.0) })
    }
}
impl SubAssign for F32x8 {
    fn sub_assign(&mut self, rhs: F32x8) {
        *self = F32x8(unsafe { _mm256_sub_ps(self.0, rhs.0) })
    }
}
// -- Mul
impl Mul for I32x1 {
    type Output = I32x1;

    fn mul(self, rhs: I32x1) -> I32x1 {
        I32x1(self.0 * rhs.0)
    }
}
impl Mul for I32x4 {
    type Output = I32x4;

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
impl Mul for I32x4_41 {
    type Output = I32x4_41;

    fn mul(self, rhs: I32x4_41) -> I32x4_41 {
        I32x4_41(unsafe { _mm_mullo_epi32(self.0, rhs.0) })
    }
}
impl Mul for I32x8 {
    type Output = I32x8;
    fn mul(self, rhs: I32x8) -> I32x8 {
        I32x8(unsafe { _mm256_mul_epi32(self.0, rhs.0) })
    }
}
impl Mul for F32x1 {
    type Output = F32x1;

    fn mul(self, rhs: F32x1) -> F32x1 {
        F32x1(self.0 * rhs.0)
    }
}
impl Mul for F32x4 {
    type Output = F32x4;

    fn mul(self, rhs: F32x4) -> F32x4 {
        F32x4(unsafe { _mm_mul_ps(self.0, rhs.0) })
    }
}
impl Mul for F32x8 {
    type Output = F32x8;
    fn mul(self, rhs: F32x8) -> F32x8 {
        F32x8(unsafe { _mm256_mul_ps(self.0, rhs.0) })
    }
}
// -- MulAssign
impl MulAssign for I32x1 {
    fn mul_assign(&mut self, rhs: I32x1) {
        *self = I32x1(self.0 * rhs.0)
    }
}
impl MulAssign for I32x4 {
    fn mul_assign(&mut self, rhs: I32x4) {
        *self = I32x4(unsafe { _mm_mul_epi32(self.0, rhs.0) })
    }
}
impl MulAssign for I32x4_41 {
    fn mul_assign(&mut self, rhs: I32x4_41) {
        *self = I32x4_41(unsafe { _mm_mul_epi32(self.0, rhs.0) })
    }
}
impl MulAssign for I32x8 {
    fn mul_assign(&mut self, rhs: I32x8) {
        *self = I32x8(unsafe { _mm256_mul_epi32(self.0, rhs.0) })
    }
}
impl MulAssign for F32x1 {
    fn mul_assign(&mut self, rhs: F32x1) {
        *self = F32x1(self.0 * rhs.0)
    }
}
impl MulAssign for F32x4 {
    fn mul_assign(&mut self, rhs: F32x4) {
        *self = F32x4(unsafe { _mm_mul_ps(self.0, rhs.0) })
    }
}
impl MulAssign for F32x8 {
    fn mul_assign(&mut self, rhs: F32x8) {
        *self = F32x8(unsafe { _mm256_mul_ps(self.0, rhs.0) })
    }
}
// -- Div
impl Div for F32x1 {
    type Output = F32x1;

    fn div(self, rhs: F32x1) -> F32x1 {
        F32x1(self.0 / rhs.0)
    }
}
impl Div for F32x4 {
    type Output = F32x4;

    fn div(self, rhs: F32x4) -> F32x4 {
        F32x4(unsafe { _mm_div_ps(self.0, rhs.0) })
    }
}
impl Div for F32x8 {
    type Output = F32x8;

    fn div(self, rhs: F32x8) -> F32x8 {
        F32x8(unsafe { _mm256_div_ps(self.0, rhs.0) })
    }
}
// -- DivAssign
impl DivAssign for F32x1 {
    fn div_assign(&mut self, rhs: F32x1) {
        *self = F32x1(self.0 + rhs.0)
    }
}
impl DivAssign for F32x4 {
    fn div_assign(&mut self, rhs: F32x4) {
        *self = F32x4(unsafe { _mm_div_ps(self.0, rhs.0) })
    }
}
impl DivAssign for F32x8 {
    fn div_assign(&mut self, rhs: F32x8) {
        *self = F32x8(unsafe { _mm256_div_ps(self.0, rhs.0) })
    }
}
// -- Bitwise And
impl BitAnd for I32x1 {
    type Output = I32x1;

    fn bitand(self, rhs: I32x1) -> I32x1 {
        I32x1(self.0 & rhs.0)
    }
}
impl BitAnd for I32x4 {
    type Output = I32x4;

    fn bitand(self, rhs: I32x4) -> I32x4 {
        I32x4(unsafe { _mm_and_si128(self.0, rhs.0) })
    }
}
impl BitAnd for I32x4_41 {
    type Output = I32x4_41;

    fn bitand(self, rhs: I32x4_41) -> I32x4_41 {
        I32x4_41(unsafe { _mm_and_si128(self.0, rhs.0) })
    }
}
impl BitAnd for I32x8 {
    type Output = I32x8;

    fn bitand(self, rhs: I32x8) -> I32x8 {
        I32x8(unsafe { _mm256_and_si256(self.0, rhs.0) })
    }
}
impl BitAnd for F32x1 {
    type Output = F32x1;

    fn bitand(self, rhs: F32x1) -> F32x1 {
        unsafe {
            let self_i = mem::transmute::<F32x1, i32>(self);
            let rhs_i = mem::transmute::<F32x1, i32>(rhs);
            mem::transmute::<i32, F32x1>(self_i & rhs_i)
        }
    }
}
impl BitAnd for F32x4 {
    type Output = F32x4;

    fn bitand(self, rhs: F32x4) -> F32x4 {
        F32x4(unsafe { _mm_and_ps(self.0, rhs.0) })
    }
}
impl BitAnd for F32x8 {
    type Output = F32x8;

    fn bitand(self, rhs: F32x8) -> F32x8 {
        F32x8(unsafe { _mm256_and_ps(self.0, rhs.0) })
    }
}
// -- BitAndAssign
impl BitAndAssign for I32x1 {
    fn bitand_assign(&mut self, rhs: I32x1) {
        *self = I32x1(self.0 & rhs.0)
    }
}
impl BitAndAssign for I32x4 {
    fn bitand_assign(&mut self, rhs: I32x4) {
        *self = I32x4(unsafe { _mm_and_si128(self.0, rhs.0) })
    }
}
impl BitAndAssign for I32x4_41 {
    fn bitand_assign(&mut self, rhs: I32x4_41) {
        *self = I32x4_41(unsafe { _mm_and_si128(self.0, rhs.0) })
    }
}
impl BitAndAssign for I32x8 {
    fn bitand_assign(&mut self, rhs: I32x8) {
        *self = I32x8(unsafe { _mm256_and_si256(self.0, rhs.0) })
    }
}
impl BitAndAssign for F32x1 {
    fn bitand_assign(&mut self, rhs: F32x1) {
        unsafe {
            let self_i = mem::transmute::<&mut F32x1, &mut i32>(self);
            let rhs_i = mem::transmute::<F32x1, i32>(rhs);
            *self = mem::transmute::<i32, F32x1>(*self_i & rhs_i)
        }
    }
}
impl BitAndAssign for F32x4 {
    fn bitand_assign(&mut self, rhs: F32x4) {
        *self = F32x4(unsafe { _mm_and_ps(self.0, rhs.0) })
    }
}
impl BitAndAssign for F32x8 {
    fn bitand_assign(&mut self, rhs: F32x8) {
        *self = F32x8(unsafe { _mm256_and_ps(self.0, rhs.0) })
    }
}
// -- Bitwise Or
impl BitOr for I32x1 {
    type Output = I32x1;

    fn bitor(self, rhs: I32x1) -> I32x1 {
        I32x1(self.0 | rhs.0)
    }
}
impl BitOr for I32x4 {
    type Output = I32x4;

    fn bitor(self, rhs: I32x4) -> I32x4 {
        I32x4(unsafe { _mm_or_si128(self.0, rhs.0) })
    }
}
impl BitOr for I32x4_41 {
    type Output = I32x4_41;

    fn bitor(self, rhs: I32x4_41) -> I32x4_41 {
        I32x4_41(unsafe { _mm_or_si128(self.0, rhs.0) })
    }
}
impl BitOr for I32x8 {
    type Output = I32x8;

    fn bitor(self, rhs: I32x8) -> I32x8 {
        I32x8(unsafe { _mm256_or_si256(self.0, rhs.0) })
    }
}
impl BitOr for F32x1 {
    type Output = F32x1;

    fn bitor(self, rhs: F32x1) -> F32x1 {
        unsafe {
            let self_i = mem::transmute::<F32x1, i32>(self);
            let rhs_i = mem::transmute::<F32x1, i32>(rhs);
            mem::transmute::<i32, F32x1>(self_i & rhs_i)
        }
    }
}
impl BitOr for F32x4 {
    type Output = F32x4;

    fn bitor(self, rhs: F32x4) -> F32x4 {
        F32x4(unsafe { _mm_or_ps(self.0, rhs.0) })
    }
}
impl BitOr for F32x8 {
    type Output = F32x8;

    fn bitor(self, rhs: F32x8) -> F32x8 {
        F32x8(unsafe { _mm256_or_ps(self.0, rhs.0) })
    }
}
// -- BitOrAssign
impl BitOrAssign for I32x1 {
    fn bitor_assign(&mut self, rhs: I32x1) {
        *self = I32x1(self.0 | rhs.0)
    }
}
impl BitOrAssign for I32x4 {
    fn bitor_assign(&mut self, rhs: I32x4) {
        *self = I32x4(unsafe { _mm_or_si128(self.0, rhs.0) })
    }
}
impl BitOrAssign for I32x4_41 {
    fn bitor_assign(&mut self, rhs: I32x4_41) {
        *self = I32x4_41(unsafe { _mm_or_si128(self.0, rhs.0) })
    }
}
impl BitOrAssign for I32x8 {
    fn bitor_assign(&mut self, rhs: I32x8) {
        *self = I32x8(unsafe { _mm256_or_si256(self.0, rhs.0) })
    }
}
impl BitOrAssign for F32x1 {
    fn bitor_assign(&mut self, rhs: F32x1) {
        unsafe {
            let self_i = mem::transmute::<&mut F32x1, &mut i32>(self);
            let rhs_i = mem::transmute::<F32x1, i32>(rhs);
            *self = mem::transmute::<i32, F32x1>(*self_i | rhs_i)
        }
    }
}
impl BitOrAssign for F32x4 {
    fn bitor_assign(&mut self, rhs: F32x4) {
        *self = F32x4(unsafe { _mm_or_ps(self.0, rhs.0) })
    }
}
impl BitOrAssign for F32x8 {
    fn bitor_assign(&mut self, rhs: F32x8) {
        *self = F32x8(unsafe { _mm256_or_ps(self.0, rhs.0) })
    }
}
// -- Bitwise XOr
impl BitXor for I32x1 {
    type Output = I32x1;

    fn bitxor(self, rhs: I32x1) -> I32x1 {
        I32x1(self.0 ^ rhs.0)
    }
}
impl BitXor for I32x4 {
    type Output = I32x4;

    fn bitxor(self, rhs: I32x4) -> I32x4 {
        I32x4(unsafe { _mm_xor_si128(self.0, rhs.0) })
    }
}
impl BitXor for I32x4_41 {
    type Output = I32x4_41;

    fn bitxor(self, rhs: I32x4_41) -> I32x4_41 {
        I32x4_41(unsafe { _mm_xor_si128(self.0, rhs.0) })
    }
}
impl BitXor for I32x8 {
    type Output = I32x8;

    fn bitxor(self, rhs: I32x8) -> I32x8 {
        I32x8(unsafe { _mm256_xor_si256(self.0, rhs.0) })
    }
}
impl BitXor for F32x1 {
    type Output = F32x1;

    fn bitxor(self, rhs: F32x1) -> F32x1 {
        unsafe {
            let self_i = mem::transmute::<F32x1, i32>(self);
            let rhs_i = mem::transmute::<F32x1, i32>(rhs);
            mem::transmute::<i32, F32x1>(self_i & rhs_i)
        }
    }
}
impl BitXor for F32x4 {
    type Output = F32x4;

    fn bitxor(self, rhs: F32x4) -> F32x4 {
        F32x4(unsafe { _mm_xor_ps(self.0, rhs.0) })
    }
}
impl BitXor for F32x8 {
    type Output = F32x8;

    fn bitxor(self, rhs: F32x8) -> F32x8 {
        F32x8(unsafe { _mm256_xor_ps(self.0, rhs.0) })
    }
}
// -- BitXorAssign
impl BitXorAssign for I32x1 {
    fn bitxor_assign(&mut self, rhs: I32x1) {
        *self = I32x1(self.0 ^ rhs.0)
    }
}
impl BitXorAssign for I32x4 {
    fn bitxor_assign(&mut self, rhs: I32x4) {
        *self = I32x4(unsafe { _mm_xor_si128(self.0, rhs.0) })
    }
}
impl BitXorAssign for I32x4_41 {
    fn bitxor_assign(&mut self, rhs: I32x4_41) {
        *self = I32x4_41(unsafe { _mm_xor_si128(self.0, rhs.0) })
    }
}
impl BitXorAssign for I32x8 {
    fn bitxor_assign(&mut self, rhs: I32x8) {
        *self = I32x8(unsafe { _mm256_xor_si256(self.0, rhs.0) })
    }
}
impl BitXorAssign for F32x1 {
    fn bitxor_assign(&mut self, rhs: F32x1) {
        unsafe {
            let self_i = mem::transmute::<&mut F32x1, &mut i32>(self);
            let rhs_i = mem::transmute::<F32x1, i32>(rhs);
            *self = mem::transmute::<i32, F32x1>(*self_i ^ rhs_i)
        }
    }
}
impl BitXorAssign for F32x4 {
    fn bitxor_assign(&mut self, rhs: F32x4) {
        *self = F32x4(unsafe { _mm_xor_ps(self.0, rhs.0) })
    }
}
impl BitXorAssign for F32x8 {
    fn bitxor_assign(&mut self, rhs: F32x8) {
        *self = F32x8(unsafe { _mm256_xor_ps(self.0, rhs.0) })
    }
}
