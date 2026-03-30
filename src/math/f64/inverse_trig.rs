use crate::math::families::inverse_trig::portable_f64;
use crate::{Simd, SimdFloat64};

#[inline(always)]
pub(crate) fn asin_u35<V>(input: V) -> V
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    portable_f64::asin_u35(input)
}

#[inline(always)]
pub(crate) fn acos_u35<V>(input: V) -> V
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    portable_f64::acos_u35(input)
}

#[inline(always)]
pub(crate) fn atan_u35<V>(input: V) -> V
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    portable_f64::atan_u35(input)
}
