use crate::math::{f32, map, scalar};
use crate::{Simd, SimdFloat32, SimdFloat64};

pub trait SimdMathF32Core: SimdFloat32 {
    #[inline(always)]
    fn log2_u35(self) -> Self
    where
        Self::Engine: Simd<Vf32 = Self>,
    {
        f32::log2_u35(self)
    }

    #[inline(always)]
    fn exp2_u35(self) -> Self
    where
        Self::Engine: Simd<Vf32 = Self>,
    {
        f32::exp2_u35(self)
    }

    #[inline(always)]
    fn ln_u35(self) -> Self {
        map::unary_f32(self, scalar::ln_u35_f32)
    }

    #[inline(always)]
    fn exp_u35(self) -> Self {
        map::unary_f32(self, scalar::exp_u35_f32)
    }

    #[inline(always)]
    fn sin_u35(self) -> Self
    where
        Self::Engine: Simd<Vf32 = Self>,
    {
        f32::sin_u35(self)
    }

    #[inline(always)]
    fn cos_u35(self) -> Self
    where
        Self::Engine: Simd<Vf32 = Self>,
    {
        f32::cos_u35(self)
    }

    #[inline(always)]
    fn tan_u35(self) -> Self
    where
        Self::Engine: Simd<Vf32 = Self>,
    {
        f32::tan_u35(self)
    }
}

impl<T: SimdFloat32> SimdMathF32Core for T {}

pub trait SimdMathF64Core: SimdFloat64 {
    #[inline(always)]
    fn log2_u35(self) -> Self {
        map::unary_f64(self, scalar::log2_u35_f64)
    }

    #[inline(always)]
    fn exp2_u35(self) -> Self {
        map::unary_f64(self, scalar::exp2_u35_f64)
    }

    #[inline(always)]
    fn ln_u35(self) -> Self {
        map::unary_f64(self, scalar::ln_u35_f64)
    }

    #[inline(always)]
    fn exp_u35(self) -> Self {
        map::unary_f64(self, scalar::exp_u35_f64)
    }

    #[inline(always)]
    fn sin_u35(self) -> Self {
        map::unary_f64(self, scalar::sin_u35_f64)
    }

    #[inline(always)]
    fn cos_u35(self) -> Self {
        map::unary_f64(self, scalar::cos_u35_f64)
    }

    #[inline(always)]
    fn tan_u35(self) -> Self {
        map::unary_f64(self, scalar::tan_u35_f64)
    }
}

impl<T: SimdFloat64> SimdMathF64Core for T {}
