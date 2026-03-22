use crate::math::{map, scalar};
use crate::SimdFloat64;

#[inline(always)]
pub(crate) fn asinh_u35<V>(input: V) -> V
where
    V: SimdFloat64,
{
    map::unary_f64(input, scalar::asinh_u35_f64)
}

#[inline(always)]
pub(crate) fn acosh_u35<V>(input: V) -> V
where
    V: SimdFloat64,
{
    map::unary_f64(input, scalar::acosh_u35_f64)
}

#[inline(always)]
pub(crate) fn atanh_u35<V>(input: V) -> V
where
    V: SimdFloat64,
{
    map::unary_f64(input, scalar::atanh_u35_f64)
}
