use crate::math::scalar;
use crate::{Simd, SimdBaseIo, SimdBaseOps, SimdConsts, SimdFloat64};

type SimdI64<V> = <<V as SimdConsts>::Engine as Simd>::Vi64;

const SINH_COSH_SMALL_ABS: f64 = 0.125;
const SINH_COSH_FAST_ABS_MAX: f64 = 0.125;
const TANH_SMALL_ABS: f64 = 0.0;
const TANH_FAST_ABS_MAX: f64 = 0.0;

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
fn exp_u35<V>(input: V) -> V
where
    V: SimdFloat64,
{
    // Temporary family-local bridge: use scalar exp lane mapping here while
    // avoiding scalar lane mapping for the final hyperbolic functions.
    unsafe {
        let mut lanes = input.as_array();
        for lane in 0..V::WIDTH {
            lanes[lane] = scalar::exp_u35_f64(lanes[lane]);
        }
        V::load_from_ptr_unaligned(&lanes as *const V::ArrayRepresentation as *const f64)
    }
}

#[inline(always)]
fn sinh_small<V>(input: V, input_sq: V) -> V
where
    V: SimdFloat64,
{
    let poly = ((((V::set1(1.0 / 39916800.0) * input_sq) + V::set1(1.0 / 362880.0)) * input_sq
        + V::set1(1.0 / 5040.0))
        * input_sq
        + V::set1(1.0 / 120.0))
        * input_sq
        + V::set1(1.0 / 6.0);

    input + (input * input_sq * poly)
}

#[inline(always)]
fn cosh_small<V>(input_sq: V) -> V
where
    V: SimdFloat64,
{
    let poly = (((V::set1(1.0 / 40320.0) * input_sq) + V::set1(1.0 / 720.0)) * input_sq
        + V::set1(1.0 / 24.0))
        * input_sq
        + V::set1(0.5);

    V::set1(1.0) + (input_sq * poly)
}

#[inline(always)]
fn sinh_cosh_medium<V>(abs_input: V) -> (V, V)
where
    V: SimdFloat64,
{
    let exp_abs = exp_u35(abs_input);
    let exp_neg_abs = V::set1(1.0) / exp_abs;
    let half = V::set1(0.5);

    (
        (exp_abs - exp_neg_abs) * half,
        (exp_abs + exp_neg_abs) * half,
    )
}

#[inline(always)]
fn sinh_cosh_masks<V>(input: V) -> (SimdI64<V>, V, V)
where
    V: SimdFloat64,
{
    let abs_input = input.abs();
    let finite_mask = input.cmp_eq(input).bitcast_i64();
    let within_fast_range = abs_input
        .cmp_lte(V::set1(SINH_COSH_FAST_ABS_MAX))
        .bitcast_i64();

    (finite_mask & within_fast_range, abs_input, input * input)
}

#[inline(always)]
pub(crate) fn sinh_u35<V>(input: V) -> V
where
    V: SimdFloat64,
{
    let (fast_mask, abs_input, input_sq) = sinh_cosh_masks(input);
    let exceptional_mask = fast_mask.cmp_eq(SimdI64::<V>::zeroes());
    let small_mask = abs_input.cmp_lt(V::set1(SINH_COSH_SMALL_ABS));

    let fast_small = sinh_small(input, input_sq);
    let exp_input = exp_u35(input);
    let exp_neg_input = V::set1(1.0) / exp_input;
    let sinh_medium = (exp_input - exp_neg_input) * V::set1(0.5);
    let fast = small_mask.blendv(sinh_medium, fast_small);
    let zero_mask = input.cmp_eq(V::set1(0.0));
    let fast = zero_mask.blendv(fast, input);

    patch_exceptional_lanes(input, fast, exceptional_mask, scalar::sinh_u35_f64)
}

#[inline(always)]
pub(crate) fn cosh_u35<V>(input: V) -> V
where
    V: SimdFloat64,
{
    let (fast_mask, abs_input, input_sq) = sinh_cosh_masks(input);
    let exceptional_mask = fast_mask.cmp_eq(SimdI64::<V>::zeroes());
    let small_mask = abs_input.cmp_lt(V::set1(SINH_COSH_SMALL_ABS));

    let fast_small = cosh_small(input_sq);
    let (_, cosh_medium) = sinh_cosh_medium(abs_input);
    let fast = small_mask.blendv(cosh_medium, fast_small);

    patch_exceptional_lanes(input, fast, exceptional_mask, scalar::cosh_u35_f64)
}

#[inline(always)]
pub(crate) fn tanh_u35<V>(input: V) -> V
where
    V: SimdFloat64,
{
    let abs_input = input.abs();
    let finite_mask = input.cmp_eq(input).bitcast_i64();
    let within_fast_range = abs_input.cmp_lte(V::set1(TANH_FAST_ABS_MAX)).bitcast_i64();
    let exceptional_mask = (finite_mask & within_fast_range).cmp_eq(SimdI64::<V>::zeroes());
    let small_mask = abs_input.cmp_lt(V::set1(TANH_SMALL_ABS));

    let input_sq = input * input;
    let fast_small = sinh_small(input, input_sq) / cosh_small(input_sq);

    let exp_input = exp_u35(input);
    let exp_neg_input = V::set1(1.0) / exp_input;
    let tanh_medium = (exp_input - exp_neg_input) / (exp_input + exp_neg_input);
    let fast = small_mask.blendv(tanh_medium, fast_small);
    let zero_mask = input.cmp_eq(V::set1(0.0));
    let fast = zero_mask.blendv(fast, input);

    patch_exceptional_lanes(input, fast, exceptional_mask, scalar::tanh_u35_f64)
}
