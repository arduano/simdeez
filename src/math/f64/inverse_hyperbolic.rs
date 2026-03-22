use crate::math::{f64, scalar};
use crate::{Simd, SimdBaseIo, SimdBaseOps, SimdConsts, SimdFloat64};

type SimdI64<V> = <<V as SimdConsts>::Engine as Simd>::Vi64;

#[inline(always)]
fn any_lane_nonzero<V>(mask: SimdI64<V>) -> bool
where
    V: SimdFloat64,
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
{
    let finite_mask = input.cmp_eq(input).bitcast_i64();
    let abs_x = input.abs();
    let tiny_mask = abs_x.cmp_lt(V::set1(1.0)).bitcast_i64();
    let large_mask = abs_x.cmp_gt(V::set1(1.0e150)).bitcast_i64();
    let zero_mask = input.cmp_eq(V::zeroes()).bitcast_i64();
    let exceptional_mask =
        finite_mask.cmp_eq(SimdI64::<V>::zeroes()) | tiny_mask | large_mask | zero_mask;

    let radicand = (abs_x * abs_x) + V::set1(1.0);
    let magnitude = f64::ln_u35(abs_x + radicand.sqrt());
    let negative_mask = input.cmp_lt(V::zeroes());
    let fast = negative_mask.blendv(magnitude, -magnitude);

    patch_exceptional_lanes(input, fast, exceptional_mask, scalar::asinh_u35_f64)
}

#[inline(always)]
pub(crate) fn acosh_u35<V>(input: V) -> V
where
    V: SimdFloat64,
{
    let finite_mask = input.cmp_eq(input).bitcast_i64();
    let in_domain_mask = input.cmp_gte(V::set1(1.0)).bitcast_i64();
    let fast_mask = finite_mask & in_domain_mask;
    let exceptional_mask = fast_mask.cmp_eq(SimdI64::<V>::zeroes());

    let root_term = ((input - V::set1(1.0)).sqrt()) * ((input + V::set1(1.0)).sqrt());
    let fast = f64::ln_u35(input + root_term);

    patch_exceptional_lanes(input, fast, exceptional_mask, scalar::acosh_u35_f64)
}

#[inline(always)]
pub(crate) fn atanh_u35<V>(input: V) -> V
where
    V: SimdFloat64,
{
    let finite_mask = input.cmp_eq(input).bitcast_i64();
    let abs_x = input.abs();
    let strict_domain_mask = abs_x.cmp_lt(V::set1(1.0)).bitcast_i64();
    let non_zero_mask = input.cmp_neq(V::zeroes()).bitcast_i64();
    let stable_range_mask = abs_x.cmp_lte(V::set1(0.99)).bitcast_i64();
    let away_from_zero_mask = abs_x.cmp_gte(V::set1(0.9)).bitcast_i64();
    let fast_mask =
        finite_mask & strict_domain_mask & non_zero_mask & stable_range_mask & away_from_zero_mask;
    let exceptional_mask = fast_mask.cmp_eq(SimdI64::<V>::zeroes());

    let one = V::set1(1.0);
    let ratio = (one + input) / (one - input);
    let fast = f64::ln_u35(ratio) * V::set1(0.5);

    patch_exceptional_lanes(input, fast, exceptional_mask, scalar::atanh_u35_f64)
}
