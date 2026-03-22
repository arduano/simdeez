use crate::math::{map, scalar};
use crate::SimdFloat64;

#[inline(always)]
pub(crate) fn log2_u35<V>(input: V) -> V
where
    V: SimdFloat64,
{
    map::unary_f64(input, scalar::log2_u35_f64)
}

#[inline(always)]
pub(crate) fn exp2_u35<V>(input: V) -> V
where
    V: SimdFloat64,
{
    map::unary_f64(input, scalar::exp2_u35_f64)
}

#[inline(always)]
pub(crate) fn ln_u35<V>(input: V) -> V
where
    V: SimdFloat64,
{
    map::unary_f64(input, scalar::ln_u35_f64)
}

#[inline(always)]
pub(crate) fn exp_u35<V>(input: V) -> V
where
    V: SimdFloat64,
{
    map::unary_f64(input, scalar::exp_u35_f64)
}

#[inline(always)]
pub(crate) fn sin_u35<V>(input: V) -> V
where
    V: SimdFloat64,
{
    map::unary_f64(input, scalar::sin_u35_f64)
}

#[inline(always)]
pub(crate) fn cos_u35<V>(input: V) -> V
where
    V: SimdFloat64,
{
    map::unary_f64(input, scalar::cos_u35_f64)
}

#[inline(always)]
pub(crate) fn tan_u35<V>(input: V) -> V
where
    V: SimdFloat64,
{
    map::unary_f64(input, scalar::tan_u35_f64)
}
