use crate::math::{f64, map, scalar};
use crate::{Simd, SimdBaseIo, SimdBaseOps, SimdConsts, SimdFloat64};

// DECISION(2026-03-23): KEEP_MIXED
// Function(s): f64 asinh_u35
// Why kept:
// - local benches show the current hybrid path materially ahead of native scalar
// - the fast path still uses an explicit scalar-lane ln step to preserve the stricter 1-ULP contract
// Revisit when:
// - asinh gets its own cheaper core or can safely absorb the relaxed portable ln_u35 error budget

// DECISION(2026-03-23): KEEP_MIXED
// Function(s): f64 acosh_u35 / atanh_u35
// Why kept:
// - acosh_u35 now passes the strict contract and beats native scalar on local runtime-selected benches
// - atanh_u35's retry never held the strict 1-ULP contract without collapsing the fast band too far,
//   so it remains scalar-reference on this host
// Revisit when:
// - atanh_u35 gets a tighter portable kernel or cleaner cross-host evidence appears

type SimdI64<V> = <<V as SimdConsts>::Engine as Simd>::Vi64;

#[inline(always)]
fn any_lane_nonzero<V>(mask: SimdI64<V>) -> bool
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    unsafe {
        let lanes = mask.as_array();
        for lane in 0..V::WIDTH {
            if lanes[lane] != 0 {
                return true;
            }
        }
    }

    false
}

#[inline(always)]
fn patch_exceptional_lanes<V>(
    input: V,
    output: V,
    exceptional_mask: SimdI64<V>,
    scalar_fallback: fn(f64) -> f64,
) -> V
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    if !any_lane_nonzero::<V>(exceptional_mask) {
        return output;
    }

    unsafe {
        let input_lanes = input.as_array();
        let mask_lanes = exceptional_mask.as_array();
        let mut output_lanes = output.as_array();

        for lane in 0..V::WIDTH {
            if mask_lanes[lane] != 0 {
                output_lanes[lane] = scalar_fallback(input_lanes[lane]);
            }
        }

        V::load_from_ptr_unaligned(&output_lanes as *const V::ArrayRepresentation as *const f64)
    }
}

#[inline(always)]
pub(crate) fn asinh_u35<V>(input: V) -> V
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    let finite_mask = input.cmp_eq(input).bitcast_i64();
    let abs_x = input.abs();
    let tiny_mask = abs_x.cmp_lt(V::set1(1.0)).bitcast_i64();
    let large_mask = abs_x.cmp_gt(V::set1(1.0e150)).bitcast_i64();
    let zero_mask = input.cmp_eq(V::zeroes()).bitcast_i64();
    let exceptional_mask =
        finite_mask.cmp_eq(SimdI64::<V>::zeroes()) | tiny_mask | large_mask | zero_mask;

    let radicand = (abs_x * abs_x) + V::set1(1.0);
    let magnitude = map::unary_f64(abs_x + radicand.sqrt(), scalar::ln_u35_f64);
    let negative_mask = input.cmp_lt(V::zeroes());
    let fast = negative_mask.blendv(magnitude, -magnitude);

    patch_exceptional_lanes(input, fast, exceptional_mask, scalar::asinh_u35_f64)
}

#[inline(always)]
pub(crate) fn acosh_u35<V>(input: V) -> V
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    let finite_mask = input.cmp_eq(input).bitcast_i64();
    let in_domain_mask = input.cmp_gte(V::set1(1.0)).bitcast_i64();
    let away_from_one_mask = input.cmp_gte(V::set1(1.5)).bitcast_i64();
    let fast_mask = finite_mask & in_domain_mask & away_from_one_mask;
    let exceptional_mask = fast_mask.cmp_eq(SimdI64::<V>::zeroes());

    let root_term = ((input - V::set1(1.0)).sqrt()) * ((input + V::set1(1.0)).sqrt());
    let fast = f64::ln_u35(input + root_term);

    patch_exceptional_lanes(input, fast, exceptional_mask, scalar::acosh_u35_f64)
}

#[inline(always)]
pub(crate) fn atanh_u35<V>(input: V) -> V
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    map::unary_f64(input, scalar::atanh_u35_f64)
}
