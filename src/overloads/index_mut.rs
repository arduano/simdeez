use super::*;

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

impl IndexMut<usize> for I16x16 {
    #[inline(always)]
    fn index_mut(&mut self, i: usize) -> &mut i16 {
        debug_assert!(i < 16);
        let arr = unsafe { mem::transmute::<&mut I16x16, &mut [i16; 16]>(self) };
        &mut arr[i]
    }
}

