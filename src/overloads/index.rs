use super::*;
//TODO perf comparison of this method vs a load intrinsic
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
impl Index<usize> for I64x2 {
    type Output = i64;

    fn index(&self, i: usize) -> &i64 {
        debug_assert!(i < 2);
        let arr = unsafe { mem::transmute::<&I64x2, &[i64; 2]>(self) };
        &arr[i]
    }
}
impl Index<usize> for I64x2_41 {
    type Output = i64;

    fn index(&self, i: usize) -> &i64 {
        debug_assert!(i < 2);
        let arr = unsafe { mem::transmute::<&I64x2_41, &[i64; 2]>(self) };
        &arr[i]
    }
}
impl Index<usize> for I64x4 {
    type Output = i64;

    fn index(&self, i: usize) -> &i64 {
        debug_assert!(i < 4);
        let arr = unsafe { mem::transmute::<&I64x4, &[i64; 4]>(self) };
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

impl Index<usize> for F64x2 {
    type Output = f64;

    fn index(&self, i: usize) -> &f64 {
        debug_assert!(i < 4);
        let arr = unsafe { mem::transmute::<&F64x2, &[f64; 2]>(self) };
        &arr[i]
    }
}
impl Index<usize> for F64x4 {
    type Output = f64;

    fn index(&self, i: usize) -> &f64 {
        debug_assert!(i < 4);
        let arr = unsafe { mem::transmute::<&F64x4, &[f64; 4]>(self) };
        &arr[i]
    }
}
