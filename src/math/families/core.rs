use crate::math::{f32, f64, map, scalar};
use crate::{Simd, SimdFloat32, SimdFloat64};

// DECISION(2026-03-23): KEEP_SCALAR_REFERENCE
// Function(s): f32 ln_u35 / exp_u35
// Why scalar:
// - the final combined-wave recheck still keeps runtime-selected ln_u35 and exp_u35 below native scalar
//   on this host (`ln`: about 2.72 ms vs 2.46 ms, `exp`: about 2.34 ms vs 2.11 ms)
// - these contracts are stricter than the relaxed portable f32 log2_u35 / exp2_u35 pieces they would
//   naturally compose from, so there is no cheap honest rescue today
// Revisit when:
// - a better shared f32 log/exp kernel exists

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
    fn log2_u35(self) -> Self
    where
        Self::Engine: Simd<Vf64 = Self>,
    {
        f64::log2_u35(self)
    }

    #[inline(always)]
    fn exp2_u35(self) -> Self
    where
        Self::Engine: Simd<Vf64 = Self>,
    {
        f64::exp2_u35(self)
    }

    #[inline(always)]
    fn ln_u35(self) -> Self
    where
        Self::Engine: Simd<Vf64 = Self>,
    {
        f64::ln_u35(self)
    }

    #[inline(always)]
    fn exp_u35(self) -> Self
    where
        Self::Engine: Simd<Vf64 = Self>,
    {
        f64::exp_u35(self)
    }

    #[inline(always)]
    fn sin_u35(self) -> Self
    where
        Self::Engine: Simd<Vf64 = Self>,
    {
        f64::sin_u35(self)
    }

    #[inline(always)]
    fn cos_u35(self) -> Self
    where
        Self::Engine: Simd<Vf64 = Self>,
    {
        f64::cos_u35(self)
    }

    #[inline(always)]
    fn tan_u35(self) -> Self
    where
        Self::Engine: Simd<Vf64 = Self>,
    {
        f64::tan_u35(self)
    }
}

impl<T: SimdFloat64> SimdMathF64Core for T {}
