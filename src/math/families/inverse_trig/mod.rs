mod portable_f32;

use crate::math::f64;
use crate::{Simd, SimdFloat32, SimdFloat64};

pub trait SimdMathF32InverseTrig: SimdFloat32 {
    #[inline(always)]
    fn asin_u35(self) -> Self
    where
        Self::Engine: Simd<Vf32 = Self>,
    {
        portable_f32::asin_u35(self)
    }

    #[inline(always)]
    fn acos_u35(self) -> Self
    where
        Self::Engine: Simd<Vf32 = Self>,
    {
        portable_f32::acos_u35(self)
    }

    #[inline(always)]
    fn atan_u35(self) -> Self
    where
        Self::Engine: Simd<Vf32 = Self>,
    {
        portable_f32::atan_u35(self)
    }
}

impl<T: SimdFloat32> SimdMathF32InverseTrig for T {}

pub trait SimdMathF64InverseTrig: SimdFloat64 {
    #[inline(always)]
    fn asin_u35(self) -> Self {
        f64::asin_u35(self)
    }

    #[inline(always)]
    fn acos_u35(self) -> Self {
        f64::acos_u35(self)
    }

    #[inline(always)]
    fn atan_u35(self) -> Self {
        f64::atan_u35(self)
    }
}

impl<T: SimdFloat64> SimdMathF64InverseTrig for T {}
