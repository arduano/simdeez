mod portable_f32;

use crate::math::{f64, map, scalar};
use crate::{Simd, SimdFloat32, SimdFloat64};

// DECISION(2026-03-23): KEEP_SCALAR_REFERENCE
// Function(s): f32 fmod
// Why scalar:
// - local benches still favor native scalar and there is no convincing portable SIMD default yet
// - the public trait entry point stays stable while the honest implementation remains scalar-reference
// Revisit when:
// - quotient-range handling becomes cheap enough for a worthwhile portable kernel

pub trait SimdMathF32BinaryMisc: SimdFloat32 {
    #[inline(always)]
    fn log10_u35(self) -> Self
    where
        Self::Engine: Simd<Vf32 = Self>,
    {
        portable_f32::log10_u35(self)
    }

    #[inline(always)]
    fn atan2_u35(self, x: Self) -> Self
    where
        Self::Engine: Simd<Vf32 = Self>,
    {
        portable_f32::atan2_u35(self, x)
    }

    #[inline(always)]
    fn hypot_u35(self, y: Self) -> Self
    where
        Self::Engine: Simd<Vf32 = Self>,
    {
        portable_f32::hypot_u35(self, y)
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
    fn log10_u35(self) -> Self
    where
        Self::Engine: Simd<Vf64 = Self>,
    {
        f64::log10_u35(self)
    }

    #[inline(always)]
    fn atan2_u35(self, x: Self) -> Self
    where
        Self::Engine: Simd<Vf64 = Self>,
    {
        f64::atan2_u35(self, x)
    }

    #[inline(always)]
    fn hypot_u35(self, y: Self) -> Self
    where
        Self::Engine: Simd<Vf64 = Self>,
    {
        f64::hypot_u35(self, y)
    }

    /// Floating-point remainder with C/libm `fmod` semantics (sign follows dividend).
    #[inline(always)]
    fn fmod(self, y: Self) -> Self
    where
        Self::Engine: Simd<Vf64 = Self>,
    {
        f64::fmod(self, y)
    }
}

impl<T: SimdFloat64> SimdMathF64BinaryMisc for T {}
