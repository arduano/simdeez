use super::*;
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
impl IndexMut<usize> for F64x1 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        debug_assert!(i == 0);
        let arr = unsafe { mem::transmute::<&mut F64x1, &mut [f64; 1]>(self) };
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
impl IndexMut<usize> for F64x2 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        debug_assert!(i < 4);
        let arr = unsafe { mem::transmute::<&mut F64x2, &mut [f64; 2]>(self) };
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
impl IndexMut<usize> for F64x4 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        debug_assert!(i < 4);
        let arr = unsafe { mem::transmute::<&mut F64x4, &mut [f64; 4]>(self) };
        &mut arr[i]
    }
}