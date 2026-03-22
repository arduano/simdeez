use crate::math::{map, scalar};
use crate::SimdFloat64;

#[inline(always)]
pub(crate) fn sinh_u35<V>(input: V) -> V
where
    V: SimdFloat64,
{
    map::unary_f64(input, scalar::sinh_u35_f64)
}

#[inline(always)]
pub(crate) fn cosh_u35<V>(input: V) -> V
where
    V: SimdFloat64,
{
    map::unary_f64(input, scalar::cosh_u35_f64)
}

#[inline(always)]
pub(crate) fn tanh_u35<V>(input: V) -> V
where
    V: SimdFloat64,
{
    map::unary_f64(input, scalar::tanh_u35_f64)
}
