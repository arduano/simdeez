use crate::math::{f64, map, scalar};
use crate::{Simd, SimdBaseIo, SimdBaseOps, SimdConsts, SimdFloat64, SimdInt64};

type SimdI64<V> = <<V as SimdConsts>::Engine as Simd>::Vi64;

const SIGN_MASK: i64 = i64::MIN;
const SINH_COSH_SCALAR_PATCH_ABS: f64 = 1.0;
const SINH_COSH_FAST_ABS_MAX: f64 = 20.0;
const TANH_SCALAR_PATCH_ABS: f64 = 1.0;
const TANH_FAST_ABS_MAX: f64 = 20.0;

// DECISION(2026-03-23): KEEP_MIXED
// Function(s): f64 sinh_u35 / cosh_u35 / tanh_u35
// Why kept:
// - local runtime-selected benches show clear wins for sinh_u35 and tanh_u35 after restoring
//   scalar-lane patching for the strict 1-ULP near-zero region
// - cosh_u35 still loses to native scalar on this host, so it stays scalar-reference
// Revisit when:
// - cosh_u35 gets a better kernel or non-x86 evidence shifts the keep/revert balance

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
fn apply_input_sign<V>(magnitude: V, input: V) -> V
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    let sign_bits = input.bitcast_i64() & SimdI64::<V>::set1(SIGN_MASK);
    (magnitude.bitcast_i64() | sign_bits).bitcast_f64()
}

#[inline(always)]
fn sinh_cosh_medium<V>(abs_input: V) -> (V, V)
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    let exp_abs = f64::exp_u35(abs_input);
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
    V::Engine: Simd<Vf64 = V>,
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
    V::Engine: Simd<Vf64 = V>,
{
    let (fast_mask, abs_input, _) = sinh_cosh_masks(input);
    let small_scalar_mask = abs_input
        .cmp_lt(V::set1(SINH_COSH_SCALAR_PATCH_ABS))
        .bitcast_i64();
    let exceptional_mask = fast_mask.cmp_eq(SimdI64::<V>::zeroes()) | small_scalar_mask;

    let (sinh_medium, _) = sinh_cosh_medium(abs_input);
    let fast = apply_input_sign(sinh_medium, input);

    patch_exceptional_lanes(input, fast, exceptional_mask, scalar::sinh_u35_f64)
}

#[inline(always)]
pub(crate) fn cosh_u35<V>(input: V) -> V
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    map::unary_f64(input, scalar::cosh_u35_f64)
}

#[inline(always)]
pub(crate) fn tanh_u35<V>(input: V) -> V
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    let abs_input = input.abs();
    let finite_mask = input.cmp_eq(input).bitcast_i64();
    let within_fast_range = abs_input.cmp_lte(V::set1(TANH_FAST_ABS_MAX)).bitcast_i64();
    let small_scalar_mask = abs_input
        .cmp_lt(V::set1(TANH_SCALAR_PATCH_ABS))
        .bitcast_i64();
    let exceptional_mask =
        (finite_mask & within_fast_range).cmp_eq(SimdI64::<V>::zeroes()) | small_scalar_mask;

    let exp_neg_2x = f64::exp_u35(abs_input * V::set1(-2.0));
    let tanh_medium = (V::set1(1.0) - exp_neg_2x) / (V::set1(1.0) + exp_neg_2x);
    let fast = apply_input_sign(tanh_medium, input);

    patch_exceptional_lanes(input, fast, exceptional_mask, scalar::tanh_u35_f64)
}
