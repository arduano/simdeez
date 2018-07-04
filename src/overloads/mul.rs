use super::*;
// -- Mul

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

impl Mul for F64x2 {
    type Output = F64x2;

    fn mul(self, rhs: F64x2) -> F64x2 {
        F64x2(unsafe { _mm_mul_pd(self.0, rhs.0) })
    }
}
impl Mul for F64x4 {
    type Output = F64x4;
    fn mul(self, rhs: F64x4) -> F64x4 {
        F64x4(unsafe { _mm256_mul_pd(self.0, rhs.0) })
    }
}
