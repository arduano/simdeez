use crate::math::{f64, scalar};
use crate::{Simd, SimdBaseIo, SimdBaseOps, SimdConsts, SimdFloat64, SimdInt64};

type SimdI64<V> = <<V as SimdConsts>::Engine as Simd>::Vi64;

const F64_SIGN_MASK: i64 = i64::MIN;
const FAST_FMOD_MAX_QUOTIENT: f64 = 4_503_599_627_370_496.0;

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
fn patch_unary_exceptional_lanes<V>(
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
fn patch_binary_exceptional_lanes<V>(
    lhs: V,
    rhs: V,
    output: V,
    exceptional_mask: SimdI64<V>,
    scalar_fallback: fn(f64, f64) -> f64,
) -> V
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    if !any_lane_nonzero::<V>(exceptional_mask) {
        return output;
    }

    unsafe {
        let lhs_lanes = lhs.as_array();
        let rhs_lanes = rhs.as_array();
        let mask_lanes = exceptional_mask.as_array();
        let mut output_lanes = output.as_array();

        for lane in 0..V::WIDTH {
            if mask_lanes[lane] != 0 {
                output_lanes[lane] = scalar_fallback(lhs_lanes[lane], rhs_lanes[lane]);
            }
        }

        V::load_from_ptr_unaligned(&output_lanes as *const V::ArrayRepresentation as *const f64)
    }
}

#[inline(always)]
fn finite_mask<V>(input: V) -> SimdI64<V>
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    input.cmp_eq(input).bitcast_i64() & input.abs().cmp_neq(V::set1(f64::INFINITY)).bitcast_i64()
}

#[inline(always)]
pub(crate) fn log10_u35<V>(input: V) -> V
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    let positive_finite = input.cmp_gt(V::zeroes()).bitcast_i64() & finite_mask(input);
    let fast = f64::log2_u35(input) * V::set1(core::f64::consts::LOG10_2);
    patch_unary_exceptional_lanes(
        input,
        fast,
        positive_finite.cmp_eq(SimdI64::<V>::zeroes()),
        scalar::log10_u35_f64,
    )
}

#[inline(always)]
pub(crate) fn atan2_u35<V>(y: V, x: V) -> V
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    let abs_y = y.abs();
    let abs_x = x.abs();

    let x_is_zero = abs_x.cmp_eq(V::zeroes()).bitcast_i64();
    let fast_mask = finite_mask(x) & finite_mask(y) & x_is_zero.cmp_eq(SimdI64::<V>::zeroes());
    let exceptional_mask = fast_mask.cmp_eq(SimdI64::<V>::zeroes());

    let large_mask = abs_y.cmp_gt(abs_x).bitcast_i64();
    let reduced_base = large_mask
        .bitcast_f64()
        .blendv(abs_y / abs_x, abs_x / abs_y);

    let z_atan = f64::atan_u35(reduced_base);

    let pi_over_2 = V::set1(core::f64::consts::FRAC_PI_2);
    let pi = V::set1(core::f64::consts::PI);
    let y_sign = y.bitcast_i64() & SimdI64::<V>::set1(F64_SIGN_MASK);
    let x_negative = x.cmp_lt(V::zeroes()).bitcast_i64();

    let base = large_mask.bitcast_f64().blendv(z_atan, pi_over_2 - z_atan);
    let with_x_quadrant = x_negative.bitcast_f64().blendv(base, pi - base);
    let fast = (y_sign ^ with_x_quadrant.bitcast_i64()).bitcast_f64();

    patch_binary_exceptional_lanes(y, x, fast, exceptional_mask, scalar::atan2_u35_f64)
}

#[inline(always)]
pub(crate) fn hypot_u35<V>(x: V, y: V) -> V
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    let abs_x = x.abs();
    let abs_y = y.abs();
    let x_gt_y = abs_x.cmp_gt(abs_y);
    let max_xy = x_gt_y.blendv(abs_y, abs_x);
    let min_xy = x_gt_y.blendv(abs_x, abs_y);

    let fast_mask = finite_mask(x) & finite_mask(y) & max_xy.cmp_neq(V::zeroes()).bitcast_i64();
    let exceptional_mask = fast_mask.cmp_eq(SimdI64::<V>::zeroes());

    let ratio = min_xy / max_xy;
    let fast = max_xy * (V::set1(1.0) + ratio * ratio).sqrt();

    patch_binary_exceptional_lanes(x, y, fast, exceptional_mask, scalar::hypot_u35_f64)
}

#[inline(always)]
pub(crate) fn fmod<V>(x: V, y: V) -> V
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    let quotient = x / y;
    let trunc = quotient
        .cmp_lt(V::zeroes())
        .blendv(quotient.floor(), quotient.ceil());
    let fast = x - trunc * y;
    let zero_mask = fast.cmp_eq(V::zeroes()).bitcast_i64();
    let signed_zero = (x.bitcast_i64() & SimdI64::<V>::set1(F64_SIGN_MASK)).bitcast_f64();
    let fast = zero_mask.bitcast_f64().blendv(fast, signed_zero);

    let finite_inputs = finite_mask(x) & finite_mask(y);
    let y_nonzero = y.cmp_neq(V::zeroes()).bitcast_i64();
    let x_not_inf = x.abs().cmp_neq(V::set1(f64::INFINITY)).bitcast_i64();
    let small_quotient = quotient
        .abs()
        .cmp_lt(V::set1(FAST_FMOD_MAX_QUOTIENT))
        .bitcast_i64();
    let fast_mask = finite_inputs & y_nonzero & x_not_inf & small_quotient;
    let exceptional_mask = fast_mask.cmp_eq(SimdI64::<V>::zeroes());

    patch_binary_exceptional_lanes(x, y, fast, exceptional_mask, scalar::fmod_f64)
}
