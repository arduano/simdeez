use crate::math::{f32, f64};
use crate::{Simd, SimdFloat32, SimdFloat64};

pub trait SimdMathF32InverseHyperbolic: SimdFloat32 {
    #[inline(always)]
    fn asinh_u35(self) -> Self
    where
        Self::Engine: Simd<Vf32 = Self>,
    {
        f32::asinh_u35(self)
    }

    #[inline(always)]
    fn acosh_u35(self) -> Self
    where
        Self::Engine: Simd<Vf32 = Self>,
    {
        f32::acosh_u35(self)
    }

    #[inline(always)]
    fn atanh_u35(self) -> Self
    where
        Self::Engine: Simd<Vf32 = Self>,
    {
        f32::atanh_u35(self)
    }
}

impl<T: SimdFloat32> SimdMathF32InverseHyperbolic for T {}

pub trait SimdMathF64InverseHyperbolic: SimdFloat64 {
    #[inline(always)]
    fn asinh_u35(self) -> Self {
        f64::asinh_u35(self)
    }

    #[inline(always)]
    fn acosh_u35(self) -> Self {
        f64::acosh_u35(self)
    }

    #[inline(always)]
    fn atanh_u35(self) -> Self {
        f64::atanh_u35(self)
    }
}

impl<T: SimdFloat64> SimdMathF64InverseHyperbolic for T {}
