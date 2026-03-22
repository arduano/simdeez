use crate::math::{map, scalar};
use crate::{SimdFloat32, SimdFloat64};

pub trait SimdMathF32BinaryMisc: SimdFloat32 {
    #[inline(always)]
    fn log10_u35(self) -> Self {
        map::unary_f32(self, scalar::log10_u35_f32)
    }

    #[inline(always)]
    fn atan2_u35(self, x: Self) -> Self {
        map::binary_f32(self, x, scalar::atan2_u35_f32)
    }

    #[inline(always)]
    fn hypot_u35(self, y: Self) -> Self {
        map::binary_f32(self, y, scalar::hypot_u35_f32)
    }

    /// Floating-point remainder with C/libm `fmod` semantics (sign follows dividend).
    #[inline(always)]
    fn fmod(self, y: Self) -> Self {
        map::binary_f32(self, y, scalar::fmod_f32)
    }
}

impl<T: SimdFloat32> SimdMathF32BinaryMisc for T {}

pub trait SimdMathF64BinaryMisc: SimdFloat64 {
    #[inline(always)]
    fn log10_u35(self) -> Self {
        map::unary_f64(self, scalar::log10_u35_f64)
    }

    #[inline(always)]
    fn atan2_u35(self, x: Self) -> Self {
        map::binary_f64(self, x, scalar::atan2_u35_f64)
    }

    #[inline(always)]
    fn hypot_u35(self, y: Self) -> Self {
        map::binary_f64(self, y, scalar::hypot_u35_f64)
    }

    /// Floating-point remainder with C/libm `fmod` semantics (sign follows dividend).
    #[inline(always)]
    fn fmod(self, y: Self) -> Self {
        map::binary_f64(self, y, scalar::fmod_f64)
    }
}

impl<T: SimdFloat64> SimdMathF64BinaryMisc for T {}
