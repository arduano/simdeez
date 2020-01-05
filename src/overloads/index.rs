use super::*;
//TODO perf comparison of this method vs a load intrinsic

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

impl Index<usize> for I16x16 {
    type Output = i16;

    #[inline(always)]
    fn index(&self, i: usize) -> &i16 {
        let arr = unsafe { mem::transmute::<&I16x16, &[i16; 16]>(self) };
        &arr[i]
    }
}
