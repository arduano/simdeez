use super::*;
use crate::Simd;

pub struct Neon;
impl Simd for Neon {
    type Vi8 = I8x16_N;
    type Vi16 = I16x8_N;
    type Vi32 = I32x4_N;
    type Vf32 = F32x4_N;
    type Vf64 = F64x2_N;
    type Vi64 = I64x2_N;

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
