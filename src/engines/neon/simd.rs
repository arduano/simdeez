use super::*;
use crate::Simd;

pub struct Neon;
impl Simd for Neon {
    type Vi8 = I8x16Neon;
    type Vi16 = I16x8Neon;
    type Vi32 = I32x4Neon;
    type Vf32 = F32x4Neon;
    type Vf64 = F64x2Neon;
    type Vi64 = I64x2Neon;

    #[inline]
    fn invoke<R>(f: impl FnOnce() -> R) -> R {
        #[inline]
        #[target_feature(enable = "neon")]
        unsafe fn inner<R>(f: impl FnOnce() -> R) -> R {
            f()
        }

        unsafe { inner(f) }
    }
}
