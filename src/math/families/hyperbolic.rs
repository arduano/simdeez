use crate::math::{f32, f64};
use crate::{Simd, SimdFloat32, SimdFloat64};

pub trait SimdMathF32Hyperbolic: SimdFloat32 {
    #[inline(always)]
    fn sinh_u35(self) -> Self
    where
        Self::Engine: Simd<Vf32 = Self>,
    {
        f32::sinh_u35(self)
    }

    #[inline(always)]
    fn cosh_u35(self) -> Self
    where
        Self::Engine: Simd<Vf32 = Self>,
    {
        f32::cosh_u35(self)
    }

    #[inline(always)]
    fn tanh_u35(self) -> Self
    where
        Self::Engine: Simd<Vf32 = Self>,
    {
        f32::tanh_u35(self)
    }
}

impl<T: SimdFloat32> SimdMathF32Hyperbolic for T {}

pub trait SimdMathF64Hyperbolic: SimdFloat64 {
    #[inline(always)]
    fn sinh_u35(self) -> Self {
        f64::sinh_u35(self)
    }

    #[inline(always)]
    fn cosh_u35(self) -> Self {
        f64::cosh_u35(self)
    }

    #[inline(always)]
    fn tanh_u35(self) -> Self {
        f64::tanh_u35(self)
    }
}

impl<T: SimdFloat64> SimdMathF64Hyperbolic for T {}
