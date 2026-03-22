use crate::math::{map, scalar};
use crate::{SimdFloat32, SimdFloat64};

pub trait SimdMathF32Hyperbolic: SimdFloat32 {
    #[inline(always)]
    fn sinh_u35(self) -> Self {
        map::unary_f32(self, scalar::sinh_u35_f32)
    }

    #[inline(always)]
    fn cosh_u35(self) -> Self {
        map::unary_f32(self, scalar::cosh_u35_f32)
    }

    #[inline(always)]
    fn tanh_u35(self) -> Self {
        map::unary_f32(self, scalar::tanh_u35_f32)
    }
}

impl<T: SimdFloat32> SimdMathF32Hyperbolic for T {}

pub trait SimdMathF64Hyperbolic: SimdFloat64 {
    #[inline(always)]
    fn sinh_u35(self) -> Self {
        map::unary_f64(self, scalar::sinh_u35_f64)
    }

    #[inline(always)]
    fn cosh_u35(self) -> Self {
        map::unary_f64(self, scalar::cosh_u35_f64)
    }

    #[inline(always)]
    fn tanh_u35(self) -> Self {
        map::unary_f64(self, scalar::tanh_u35_f64)
    }
}

impl<T: SimdFloat64> SimdMathF64Hyperbolic for T {}
