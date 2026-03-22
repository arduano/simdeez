use crate::math::{map, scalar};
use crate::SimdFloat64;

#[inline(always)]
pub(crate) fn log10_u35<V>(input: V) -> V
where
    V: SimdFloat64,
{
    map::unary_f64(input, scalar::log10_u35_f64)
}

#[inline(always)]
pub(crate) fn atan2_u35<V>(y: V, x: V) -> V
where
    V: SimdFloat64,
{
    map::binary_f64(y, x, scalar::atan2_u35_f64)
}

#[inline(always)]
pub(crate) fn hypot_u35<V>(x: V, y: V) -> V
where
    V: SimdFloat64,
{
    map::binary_f64(x, y, scalar::hypot_u35_f64)
}

#[inline(always)]
pub(crate) fn fmod<V>(x: V, y: V) -> V
where
    V: SimdFloat64,
{
    map::binary_f64(x, y, scalar::fmod_f64)
}
