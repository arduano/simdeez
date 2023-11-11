use super::*;
use crate::Simd;

pub struct Wasm;
impl Simd for Wasm {
    type Vi8 = I8x16Wasm;
    type Vi16 = I16x8Wasm;
    type Vi32 = I32x4Wasm;
    type Vf32 = F32x4Wasm;
    type Vf64 = F64x2Wasm;
    type Vi64 = I64x2Wasm;

    #[inline]
    fn invoke<R>(f: impl FnOnce() -> R) -> R {
        #[inline]
        #[target_feature(enable = "simd128")]
        unsafe fn inner<R>(f: impl FnOnce() -> R) -> R {
            f()
        }

        unsafe { inner(f) }
    }
}
