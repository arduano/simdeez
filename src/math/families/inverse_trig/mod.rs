mod portable_f32;

use crate::math::{map, scalar};
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
        map::unary_f64(self, scalar::asin_u35_f64)
    }

    #[inline(always)]
    fn acos_u35(self) -> Self {
        map::unary_f64(self, scalar::acos_u35_f64)
    }

    #[inline(always)]
    fn atan_u35(self) -> Self {
        map::unary_f64(self, scalar::atan_u35_f64)
    }
}

impl<T: SimdFloat64> SimdMathF64InverseTrig for T {}
