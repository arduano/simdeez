use crate::math::{map, scalar};
use crate::SimdFloat64;

// DECISION(2026-03-23): KEEP_SCALAR_REFERENCE
// Function(s): f64 sinh_u35 / cosh_u35 / tanh_u35
// Why scalar:
// - local benches do not justify a portable SIMD default for this family on the current host
// - keeping the family split still preserves test and ownership structure for a later retry
// Revisit when:
// - a cheaper f64 exp/log backbone or a dedicated hyperbolic kernel lands

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
