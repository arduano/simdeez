use super::*;
// -- AddAssign
impl AddAssign for I16x1 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: I16x1) {
        *self = I16x1(self.0 + rhs.0);
    }
}

impl AddAssign for I32x1 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: I32x1) {
        *self = I32x1(self.0 + rhs.0);
    }
}

impl AddAssign for I64x1 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: I64x1) {
        *self = I64x1(self.0 + rhs.0);
    }
}

impl AddAssign for F32x1 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: F32x1) {
        *self = F32x1(self.0 + rhs.0);
    }
}

impl AddAssign for F64x1 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: F64x1) {
        *self = F64x1(self.0 + rhs.0);
    }
}

impl Add for I16x1 {
    type Output = I16x1;

    #[inline(always)]
    fn add(self, rhs: I16x1) -> I16x1 {
        I16x1(self.0 + rhs.0)
    }
}

impl Add for I32x1 {
    type Output = I32x1;

    #[inline(always)]
    fn add(self, rhs: I32x1) -> I32x1 {
        I32x1(self.0 + rhs.0)
    }
}

impl Add for I64x1 {
    type Output = I64x1;

    #[inline(always)]
    fn add(self, rhs: I64x1) -> I64x1 {
        I64x1(self.0 + rhs.0)
    }
}

impl Add for F32x1 {
    type Output = F32x1;

    #[inline(always)]
    fn add(self, rhs: F32x1) -> F32x1 {
        F32x1(self.0 + rhs.0)
    }
}

impl Add for F64x1 {
    type Output = F64x1;

    #[inline(always)]
    fn add(self, rhs: F64x1) -> F64x1 {
        F64x1(self.0 + rhs.0)
    }
}
// -- BitAndAssign

impl BitAndAssign for I16x1 {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: I16x1) {
        *self = I16x1(self.0 & rhs.0);
    }
}

impl BitAndAssign for I32x1 {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: I32x1) {
        *self = I32x1(self.0 & rhs.0);
    }
}

impl BitAndAssign for I64x1 {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: I64x1) {
        *self = I64x1(self.0 & rhs.0);
    }
}

impl BitAndAssign for F32x1 {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: F32x1) {
        let lbits = self.0.to_bits();
        let rbits = rhs.0.to_bits();
        let result = lbits & rbits;
        *self = F32x1(f32::from_bits(result));
    }
}

impl BitAndAssign for F64x1 {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: F64x1) {
        let lbits = self.0.to_bits();
        let rbits = rhs.0.to_bits();
        let result = lbits & rbits;
        *self = F64x1(f64::from_bits(result));
    }
}

// -- Bitwise And

impl BitAnd for I16x1 {
    type Output = I16x1;

    #[inline(always)]
    fn bitand(self, rhs: I16x1) -> I16x1 {
        I16x1(self.0 & rhs.0)
    }
}

impl BitAnd for I32x1 {
    type Output = I32x1;

    #[inline(always)]
    fn bitand(self, rhs: I32x1) -> I32x1 {
        I32x1(self.0 & rhs.0)
    }
}

impl BitAnd for I64x1 {
    type Output = I64x1;

    #[inline(always)]
    fn bitand(self, rhs: I64x1) -> I64x1 {
        I64x1(self.0 & rhs.0)
    }
}

impl BitAnd for F32x1 {
    type Output = F32x1;

    #[inline(always)]
    fn bitand(self, rhs: F32x1) -> F32x1 {
        let lbits = self.0.to_bits();
        let rbits = rhs.0.to_bits();
        let result = lbits & rbits;
        F32x1(f32::from_bits(result))
    }
}

impl BitAnd for F64x1 {
    type Output = F64x1;

    #[inline(always)]
    fn bitand(self, rhs: F64x1) -> F64x1 {
        let lbits = self.0.to_bits();
        let rbits = rhs.0.to_bits();
        let result = lbits & rbits;
        F64x1(f64::from_bits(result))
    }
}

impl DivAssign for F32x1 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: F32x1) {
        *self = F32x1(self.0 / rhs.0);
    }
}

impl DivAssign for F64x1 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: F64x1) {
        *self = F64x1(self.0 / rhs.0);
    }
}

impl Div for F32x1 {
    type Output = F32x1;

    #[inline(always)]
    fn div(self, rhs: F32x1) -> F32x1 {
        F32x1(self.0 / rhs.0)
    }
}

impl Div for F64x1 {
    type Output = F64x1;

    #[inline(always)]
    fn div(self, rhs: F64x1) -> F64x1 {
        F64x1(self.0 / rhs.0)
    }
}

impl IndexMut<usize> for I16x1 {
    #[inline(always)]
    fn index_mut(&mut self, _i: usize) -> &mut i16 {
        &mut self.0
    }
}

impl IndexMut<usize> for I32x1 {
    #[inline(always)]
    fn index_mut(&mut self, _i: usize) -> &mut i32 {
        &mut self.0
    }
}

impl IndexMut<usize> for I64x1 {
    #[inline(always)]
    fn index_mut(&mut self, _i: usize) -> &mut i64 {
        &mut self.0
    }
}

impl IndexMut<usize> for F32x1 {
    #[inline(always)]
    fn index_mut(&mut self, _i: usize) -> &mut f32 {
        &mut self.0
    }
}

impl IndexMut<usize> for F64x1 {
    #[inline(always)]
    fn index_mut(&mut self, _i: usize) -> &mut f64 {
        &mut self.0
    }
}
impl Index<usize> for I16x1 {
    type Output = i16;

    #[inline(always)]
    fn index(&self, _i: usize) -> &i16 {
        &self.0
    }
}

impl Index<usize> for I32x1 {
    type Output = i32;

    #[inline(always)]
    fn index(&self, _i: usize) -> &i32 {
        &self.0
    }
}

impl Index<usize> for I64x1 {
    type Output = i64;

    #[inline(always)]
    fn index(&self, _i: usize) -> &i64 {
        &self.0
    }
}

impl Index<usize> for F32x1 {
    type Output = f32;

    #[inline(always)]
    fn index(&self, _i: usize) -> &f32 {
        &self.0
    }
}

impl Index<usize> for F64x1 {
    type Output = f64;

    #[inline(always)]
    fn index(&self, _i: usize) -> &f64 {
        &self.0
    }
}

impl MulAssign for I16x1 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: I16x1) {
        *self = I16x1(self.0.wrapping_mul(rhs.0));
    }
}

impl MulAssign for I32x1 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: I32x1) {
        *self = I32x1(self.0.wrapping_mul(rhs.0));
    }
}

impl MulAssign for I64x1 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: I64x1) {
        *self = I64x1(self.0.wrapping_mul(rhs.0));
    }
}

impl MulAssign for F32x1 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: F32x1) {
        *self = F32x1(self.0 * rhs.0);
    }
}

impl MulAssign for F64x1 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: F64x1) {
        *self = F64x1(self.0 * rhs.0);
    }
}

impl Mul for I16x1 {
    type Output = I16x1;

    #[inline(always)]
    fn mul(self, rhs: I16x1) -> I16x1 {
        I16x1(self.0.wrapping_mul(rhs.0))
    }
}

impl Mul for I32x1 {
    type Output = I32x1;

    #[inline(always)]
    fn mul(self, rhs: I32x1) -> I32x1 {
        I32x1(self.0.wrapping_mul(rhs.0))
    }
}

impl Mul for I64x1 {
    type Output = I64x1;

    #[inline(always)]
    fn mul(self, rhs: I64x1) -> I64x1 {
        I64x1(self.0.wrapping_mul(rhs.0))
    }
}

impl Mul for F32x1 {
    type Output = F32x1;

    #[inline(always)]
    fn mul(self, rhs: F32x1) -> F32x1 {
        F32x1(self.0 * rhs.0)
    }
}

impl Mul for F64x1 {
    type Output = F64x1;

    #[inline(always)]
    fn mul(self, rhs: F64x1) -> F64x1 {
        F64x1(self.0 * rhs.0)
    }
}

impl Not for I16x1 {
    type Output = I16x1;

    #[inline(always)]
    fn not(self) -> I16x1 {
        I16x1(!(self.0))
    }
}

impl Not for I32x1 {
    type Output = I32x1;

    #[inline(always)]
    fn not(self) -> I32x1 {
        I32x1(!(self.0))
    }
}

impl Not for I64x1 {
    type Output = I64x1;

    #[inline(always)]
    fn not(self) -> I64x1 {
        I64x1(!(self.0))
    }
}

impl Not for F32x1 {
    type Output = F32x1;

    #[inline(always)]
    fn not(self) -> F32x1 {
        let bits = !(self.0.to_bits());
        F32x1(f32::from_bits(bits))
    }
}

impl Not for F64x1 {
    type Output = F64x1;

    #[inline(always)]
    fn not(self) -> F64x1 {
        let bits = !(self.0.to_bits());
        F64x1(f64::from_bits(bits))
    }
}

impl BitOrAssign for I16x1 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: I16x1) {
        *self = I16x1(self.0 | rhs.0);
    }
}

impl BitOrAssign for I32x1 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: I32x1) {
        *self = I32x1(self.0 | rhs.0);
    }
}

impl BitOrAssign for I64x1 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: I64x1) {
        *self = I64x1(self.0 | rhs.0);
    }
}

impl BitOrAssign for F32x1 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: F32x1) {
        let lbits = self.0.to_bits();
        let rbits = rhs.0.to_bits();
        let result = lbits | rbits;
        *self = F32x1(f32::from_bits(result));
    }
}

impl BitOrAssign for F64x1 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: F64x1) {
        let lbits = self.0.to_bits();
        let rbits = rhs.0.to_bits();
        let result = lbits | rbits;
        *self = F64x1(f64::from_bits(result));
    }
}

impl BitOr for I16x1 {
    type Output = I16x1;
    #[inline(always)]
    fn bitor(self, rhs: I16x1) -> I16x1 {
        I16x1(self.0 | rhs.0)
    }
}

impl BitOr for I32x1 {
    type Output = I32x1;

    #[inline(always)]
    fn bitor(self, rhs: I32x1) -> I32x1 {
        I32x1(self.0 | rhs.0)
    }
}

impl BitOr for I64x1 {
    type Output = I64x1;

    #[inline(always)]
    fn bitor(self, rhs: I64x1) -> I64x1 {
        I64x1(self.0 | rhs.0)
    }
}

impl BitOr for F32x1 {
    type Output = F32x1;

    #[inline(always)]
    fn bitor(self, rhs: F32x1) -> F32x1 {
        let lbits = self.0.to_bits();
        let rbits = rhs.0.to_bits();
        let result = lbits | rbits;
        F32x1(f32::from_bits(result))
    }
}

impl BitOr for F64x1 {
    type Output = F64x1;

    #[inline(always)]
    fn bitor(self, rhs: F64x1) -> F64x1 {
        let lbits = self.0.to_bits();
        let rbits = rhs.0.to_bits();
        let result = lbits | rbits;
        F64x1(f64::from_bits(result))
    }
}

impl ShlAssign<i32> for I16x1 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: i32) {
        *self = I16x1(self.0 << rhs);
    }
}

impl ShlAssign<i32> for I32x1 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: i32) {
        *self = I32x1(self.0 << rhs);
    }
}

impl ShlAssign<i32> for I64x1 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: i32) {
        *self = I64x1(self.0 << rhs);
    }
}

impl ShlAssign<i32> for F32x1 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: i32) {
        let bits = self.0.to_bits() << rhs;
        *self = F32x1(f32::from_bits(bits));
    }
}

impl ShlAssign<i32> for F64x1 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: i32) {
        let bits = self.0.to_bits() << rhs;
        *self = F64x1(f64::from_bits(bits));
    }
}

impl Shl<i32> for I16x1 {
    type Output = I16x1;

    #[inline(always)]
    fn shl(self, rhs: i32) -> I16x1 {
        I16x1(self.0 << rhs)
    }
}

impl Shl<i32> for I32x1 {
    type Output = I32x1;

    #[inline(always)]
    fn shl(self, rhs: i32) -> I32x1 {
        I32x1(self.0 << rhs)
    }
}

impl Shl<i32> for I64x1 {
    type Output = I64x1;

    #[inline(always)]
    fn shl(self, rhs: i32) -> I64x1 {
        I64x1(self.0 << rhs)
    }
}

impl Shl<i32> for F32x1 {
    type Output = F32x1;

    #[inline(always)]
    fn shl(self, rhs: i32) -> F32x1 {
        let bits = self.0.to_bits() << rhs;
        F32x1(f32::from_bits(bits))
    }
}

impl Shl<i32> for F64x1 {
    type Output = F64x1;

    #[inline(always)]
    fn shl(self, rhs: i32) -> F64x1 {
        let bits = self.0.to_bits() << rhs;
        F64x1(f64::from_bits(bits))
    }
}

impl ShrAssign<i32> for I16x1 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: i32) {
        *self = I16x1(self.0 >> rhs);
    }
}

impl ShrAssign<i32> for I32x1 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: i32) {
        *self = I32x1(self.0 >> rhs);
    }
}

impl ShrAssign<i32> for I64x1 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: i32) {
        *self = I64x1(self.0 >> rhs);
    }
}

impl ShrAssign<i32> for F32x1 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: i32) {
        let bits = self.0.to_bits() >> rhs;
        *self = F32x1(f32::from_bits(bits));
    }
}

impl ShrAssign<i32> for F64x1 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: i32) {
        let bits = self.0.to_bits() >> rhs;
        *self = F64x1(f64::from_bits(bits));
    }
}

impl Shr<i32> for I16x1 {
    type Output = I16x1;

    #[inline(always)]
    fn shr(self, rhs: i32) -> I16x1 {
        I16x1(self.0 >> rhs)
    }
}

impl Shr<i32> for I32x1 {
    type Output = I32x1;

    #[inline(always)]
    fn shr(self, rhs: i32) -> I32x1 {
        I32x1(self.0 >> rhs)
    }
}

impl Shr<i32> for I64x1 {
    type Output = I64x1;

    #[inline(always)]
    fn shr(self, rhs: i32) -> I64x1 {
        I64x1(self.0 >> rhs)
    }
}

impl Shr<i32> for F32x1 {
    type Output = F32x1;

    #[inline(always)]
    fn shr(self, rhs: i32) -> F32x1 {
        let bits = self.0.to_bits() >> rhs;
        F32x1(f32::from_bits(bits))
    }
}

impl Shr<i32> for F64x1 {
    type Output = F64x1;

    #[inline(always)]
    fn shr(self, rhs: i32) -> F64x1 {
        let bits = self.0.to_bits() >> rhs;
        F64x1(f64::from_bits(bits))
    }
}

impl SubAssign for I16x1 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: I16x1) {
        *self = I16x1(self.0 - rhs.0);
    }
}

impl SubAssign for I32x1 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: I32x1) {
        *self = I32x1(self.0 - rhs.0);
    }
}

impl SubAssign for I64x1 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: I64x1) {
        *self = I64x1(self.0 - rhs.0);
    }
}

impl SubAssign for F32x1 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: F32x1) {
        *self = F32x1(self.0 - rhs.0);
    }
}

impl SubAssign for F64x1 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: F64x1) {
        *self = F64x1(self.0 - rhs.0);
    }
}

impl Sub for I16x1 {
    type Output = I16x1;

    #[inline(always)]
    fn sub(self, rhs: I16x1) -> I16x1 {
        I16x1(self.0 - rhs.0)
    }
}

impl Sub for I32x1 {
    type Output = I32x1;

    #[inline(always)]
    fn sub(self, rhs: I32x1) -> I32x1 {
        I32x1(self.0 - rhs.0)
    }
}

impl Sub for I64x1 {
    type Output = I64x1;

    #[inline(always)]
    fn sub(self, rhs: I64x1) -> I64x1 {
        I64x1(self.0 - rhs.0)
    }
}

impl Sub for F32x1 {
    type Output = F32x1;

    #[inline(always)]
    fn sub(self, rhs: F32x1) -> F32x1 {
        F32x1(self.0 - rhs.0)
    }
}

impl Sub for F64x1 {
    type Output = F64x1;

    #[inline(always)]
    fn sub(self, rhs: F64x1) -> F64x1 {
        F64x1(self.0 - rhs.0)
    }
}

impl BitXorAssign for I16x1 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: I16x1) {
        *self = I16x1(self.0 ^ rhs.0);
    }
}

impl BitXorAssign for I32x1 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: I32x1) {
        *self = I32x1(self.0 ^ rhs.0);
    }
}

impl BitXorAssign for I64x1 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: I64x1) {
        *self = I64x1(self.0 ^ rhs.0);
    }
}

impl BitXorAssign for F32x1 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: F32x1) {
        let lbits = self.0.to_bits();
        let rbits = rhs.0.to_bits();
        let result = lbits ^ rbits;
        *self = F32x1(f32::from_bits(result));
    }
}

impl BitXorAssign for F64x1 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: F64x1) {
        let lbits = self.0.to_bits();
        let rbits = rhs.0.to_bits();
        let result = lbits ^ rbits;
        *self = F64x1(f64::from_bits(result));
    }
}

impl BitXor for I16x1 {
    type Output = I16x1;

    #[inline(always)]
    fn bitxor(self, rhs: I16x1) -> I16x1 {
        I16x1(self.0 ^ rhs.0)
    }
}

impl BitXor for I32x1 {
    type Output = I32x1;

    #[inline(always)]
    fn bitxor(self, rhs: I32x1) -> I32x1 {
        I32x1(self.0 ^ rhs.0)
    }
}

impl BitXor for I64x1 {
    type Output = I64x1;

    #[inline(always)]
    fn bitxor(self, rhs: I64x1) -> I64x1 {
        I64x1(self.0 ^ rhs.0)
    }
}

impl BitXor for F32x1 {
    type Output = F32x1;

    #[inline(always)]
    fn bitxor(self, rhs: F32x1) -> F32x1 {
        let lbits = self.0.to_bits();
        let rbits = rhs.0.to_bits();
        let result = lbits ^ rbits;
        F32x1(f32::from_bits(result))
    }
}

impl BitXor for F64x1 {
    type Output = F64x1;

    #[inline(always)]
    fn bitxor(self, rhs: F64x1) -> F64x1 {
        let lbits = self.0.to_bits();
        let rbits = rhs.0.to_bits();
        let result = lbits ^ rbits;
        F64x1(f64::from_bits(result))
    }
}
