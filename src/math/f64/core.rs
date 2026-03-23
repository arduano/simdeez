use crate::math::{map, scalar};
use crate::{Simd, SimdFloat64};

// DECISION(2026-03-23): KEEP_SCALAR_REFERENCE
// Function(s): f64 log2_u35 / exp2_u35 / ln_u35 / exp_u35
// Why scalar:
// - local benches keep putting runtime-selected behavior at or below native scalar
// - family structure stays useful, but the current default is still scalar-reference
// Revisit when:
// - a genuinely worthwhile f64 log/exp SIMD kernel exists

// DECISION(2026-03-23): KEEP_SCALAR_REFERENCE
// Function(s): f64 sin_u35 / cos_u35 / tan_u35
// Why scalar:
// - the previous portable trig fast path still lagged native scalar on this host
// - the implementation was reverted to scalar-reference while preserving family ownership
// Revisit when:
// - a stronger range-reduction strategy or cheaper trig kernel appears

#[inline(always)]
pub(crate) fn log2_u35<V>(input: V) -> V
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    map::unary_f64(input, scalar::log2_u35_f64)
}

#[inline(always)]
pub(crate) fn exp2_u35<V>(input: V) -> V
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    map::unary_f64(input, scalar::exp2_u35_f64)
}

#[inline(always)]
pub(crate) fn ln_u35<V>(input: V) -> V
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
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
