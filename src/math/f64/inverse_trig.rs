use crate::math::{map, scalar};
use crate::SimdFloat64;

#[inline(always)]
pub(crate) fn asin_u35<V>(input: V) -> V
where
    V: SimdFloat64,
{
    map::unary_f64(input, scalar::asin_u35_f64)
}

#[inline(always)]
pub(crate) fn acos_u35<V>(input: V) -> V
where
    V: SimdFloat64,
{
    map::unary_f64(input, scalar::acos_u35_f64)
}

#[inline(always)]
pub(crate) fn atan_u35<V>(input: V) -> V
where
    V: SimdFloat64,
{
    map::unary_f64(input, scalar::atan_u35_f64)
}
