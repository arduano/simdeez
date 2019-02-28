/* EXPERIMENTAL 
use super::*;

impl Add<i16> for I16x1 {
    type Output = I16x1;

    #[inline(always)]
    fn add(self, rhs: i16) -> I16x1 {
        I16x1(self.0 + rhs)
    }
}

impl Add<i32> for I32x1 {
    type Output = I32x1;

    #[inline(always)]
    fn add(self, rhs: i32) -> I32x1 {
        I32x1(self.0 + rhs)
    }
}

impl Add<i64> for I64x1 {
    type Output = I64x1;

    #[inline(always)]
    fn add(self, rhs: i64) -> I64x1 {
        I64x1(self.0 + rhs)
    }
}

impl Add<f32> for F32x1 {
    type Output = F32x1;

    #[inline(always)]
    fn add(self, rhs:f32) -> F32x1 {
        F32x1(self.0 + rhs)
    }
}

impl Add<f64> for F64x1 {
    type Output = F64x1;

    #[inline(always)]
    fn add(self, rhs: f64) -> F64x1 {
        F64x1(self.0 + rhs)
    }
}

impl Add<i16> for I16x8 {
    type Output = I16x8;

    #[inline(always)]
    fn add(self, rhs: i16) -> I16x8 {
        I16x8(unsafe { _mm_add_epi16(self.0, _mm_set1_epi16(rhs)) })
    }
}

impl Add<i16> for I16x16 {
    type Output = I16x16;

    #[inline(always)]
    fn add(self, rhs: i16) -> I16x16 {
        I16x16(unsafe { _mm256_add_epi16(self.0, _mm256_set1_epi16(rhs)) })
    }
}

impl Add<i32> for I32x4 {
    type Output = I32x4;

    #[inline(always)]
    fn add(self, rhs: i32) -> I32x4 {
        I32x4(unsafe { _mm_add_epi32(self.0, _mm_set1_epi32(rhs)) })
    }
}



impl Add<i32> for I32x4_41 {
    type Output = I32x4_41;

    #[inline(always)]
    fn add(self, rhs: i32) -> I32x4_41 {
        I32x4_41(unsafe { _mm_add_epi32(self.0, _mm_set1_epi32(rhs)) })
    }
}
impl Add<i32> for I32x8 {
    type Output = I32x8;

    #[inline(always)]
    fn add(self, rhs: i32) -> I32x8 {
        I32x8(unsafe { _mm256_add_epi32(self.0, _mm256_set1_epi32(rhs)) })
    }
}
impl Add<i64> for I64x2 {
    type Output = I64x2;

    #[inline(always)]
    fn add(self, rhs: i64) -> I64x2 {
        I64x2(unsafe { _mm_add_epi64(self.0, _mm_set1_epi64x(rhs)) })
    }
}
impl Add<i64> for I64x2_41 {
    type Output = I64x2_41;

    #[inline(always)]
    fn add(self, rhs: i64) -> I64x2_41 {
        I64x2_41(unsafe { _mm_add_epi64(self.0, _mm_set1_epi64x(rhs)) })
    }
}
impl Add<i64> for I64x4 {
    type Output = I64x4;

    #[inline(always)]
    fn add(self, rhs: i64) -> I64x4 {
        I64x4(unsafe { _mm256_add_epi64(self.0, _mm256_set1_epi64x(rhs)) })
    }
}

impl Add<f32> for F32x4 {
    type Output = F32x4;

    #[inline(always)]
    fn add(self, rhs: f32) -> F32x4 {
        F32x4(unsafe { _mm_add_ps(self.0, _mm_set1_ps(rhs)) })
    }
}
impl Add<f32> for F32x8 {
    type Output = F32x8;

    #[inline(always)]
    fn add(self, rhs: f32) -> F32x8 {
        F32x8(unsafe { _mm256_add_ps(self.0, _mm256_set1_ps(rhs)) })
    }
}
impl Add<f64> for F64x2 {
    type Output = F64x2;

    #[inline(always)]
    fn add(self, rhs: f64) -> F64x2 {
        F64x2(unsafe { _mm_add_pd(self.0, _mm_set1_pd(rhs)) })
    }
}
impl Add<f64> for F64x4 {
    type Output = F64x4;

    #[inline(always)]
    fn add(self, rhs: f64) -> F64x4 {
        F64x4(unsafe { _mm256_add_pd(self.0, _mm256_set1_pd(rhs)) })
    }
}
*/