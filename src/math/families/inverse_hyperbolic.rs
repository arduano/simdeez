use crate::math::{map, scalar};
use crate::{SimdFloat32, SimdFloat64};

pub trait SimdMathF32InverseHyperbolic: SimdFloat32 {
    #[inline(always)]
    fn asinh_u35(self) -> Self {
        map::unary_f32(self, scalar::asinh_u35_f32)
    }

    #[inline(always)]
    fn acosh_u35(self) -> Self {
        map::unary_f32(self, scalar::acosh_u35_f32)
    }

    #[inline(always)]
    fn atanh_u35(self) -> Self {
        map::unary_f32(self, scalar::atanh_u35_f32)
    }
}

impl<T: SimdFloat32> SimdMathF32InverseHyperbolic for T {}

pub trait SimdMathF64InverseHyperbolic: SimdFloat64 {
    #[inline(always)]
    fn asinh_u35(self) -> Self {
        map::unary_f64(self, scalar::asinh_u35_f64)
    }

    #[inline(always)]
    fn acosh_u35(self) -> Self {
        map::unary_f64(self, scalar::acosh_u35_f64)
    }

    #[inline(always)]
    fn atanh_u35(self) -> Self {
        map::unary_f64(self, scalar::atanh_u35_f64)
    }
}

impl<T: SimdFloat64> SimdMathF64InverseHyperbolic for T {}
