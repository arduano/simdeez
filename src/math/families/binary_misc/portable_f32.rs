use crate::math::{f32, scalar};
use crate::{Simd, SimdBaseIo, SimdBaseOps, SimdFloat32, SimdInt32};

// DECISION(2026-03-23): KEEP_SIMD_PORTABLE
// Function(s): f32 log10_u35 / atan2_u35 / hypot_u35
// Why kept:
// - local benches show each of these kernels materially ahead of native scalar
// - targeted edge tests already cover domain, signed-zero, and scale-stress behavior
// Revisit when:
// - the shared log2/atan primitives or exceptional-lane rules change materially

type SimdI32<V> = <<V as crate::SimdConsts>::Engine as Simd>::Vi32;

#[inline(always)]
fn any_lane_nonzero<V>(mask: SimdI32<V>) -> bool
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
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
    exceptional_mask: SimdI32<V>,
    scalar_fallback: fn(f32) -> f32,
) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
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

        V::load_from_ptr_unaligned(&output_lanes as *const V::ArrayRepresentation as *const f32)
    }
}

#[inline(always)]
fn patch_binary_exceptional_lanes<V>(
    lhs: V,
    rhs: V,
    output: V,
    exceptional_mask: SimdI32<V>,
    scalar_fallback: fn(f32, f32) -> f32,
) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
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

        V::load_from_ptr_unaligned(&output_lanes as *const V::ArrayRepresentation as *const f32)
    }
}

#[inline(always)]
pub(super) fn log10_u35<V>(input: V) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    let positive_finite =
        input.cmp_gt(V::zeroes()).bitcast_i32() & input.cmp_eq(input).bitcast_i32();
    let fast = f32::log2_u35(input) * V::set1(core::f32::consts::LOG10_2);
    patch_unary_exceptional_lanes(
        input,
        fast,
        positive_finite.cmp_eq(SimdI32::<V>::zeroes()),
        scalar::log10_u35_f32,
    )
}

#[inline(always)]
fn atan_u35_poly<V>(z: V) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    let z2 = z * z;

    let mut poly = V::set1(0.002_866_225_7);
    poly = (poly * z2) + V::set1(-0.016_165_737);
    poly = (poly * z2) + V::set1(0.042_909_615);
    poly = (poly * z2) + V::set1(-0.075_289_64);
    poly = (poly * z2) + V::set1(0.106_562_64);
    poly = (poly * z2) + V::set1(-0.142_089);
    poly = (poly * z2) + V::set1(0.199_935_51);
    poly = (poly * z2) + V::set1(-0.333_331_47);
    poly = (poly * z2) + V::set1(1.0);

    z * poly
}

#[inline(always)]
pub(super) fn atan2_u35<V>(y: V, x: V) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    let abs_y = y.abs();
    let abs_x = x.abs();

    let x_is_zero = abs_x.cmp_eq(V::zeroes()).bitcast_i32();
    let y_is_zero = abs_y.cmp_eq(V::zeroes()).bitcast_i32();
    let x_finite = x.cmp_eq(x).bitcast_i32() & abs_x.cmp_neq(V::set1(f32::INFINITY)).bitcast_i32();
    let y_finite = y.cmp_eq(y).bitcast_i32() & abs_y.cmp_neq(V::set1(f32::INFINITY)).bitcast_i32();

    let fast_mask = x_finite & y_finite & x_is_zero.cmp_eq(SimdI32::<V>::zeroes());
    let exceptional_mask = fast_mask.cmp_eq(SimdI32::<V>::zeroes());

    let use_inv = abs_y.cmp_gt(abs_x);
    let z = use_inv.blendv(abs_y / abs_x, abs_x / abs_y);
    let z_atan = atan_u35_poly(z);

    let pi_over_2 = V::set1(core::f32::consts::FRAC_PI_2);
    let pi = V::set1(core::f32::consts::PI);
    let y_sign = y.bitcast_i32() & SimdI32::<V>::set1(i32::MIN);
    let x_negative = x.cmp_lt(V::zeroes()).bitcast_i32();

    let base = use_inv.blendv(z_atan, pi_over_2 - z_atan);
    let with_x_quadrant = x_negative.bitcast_f32().blendv(base, pi - base);
    let signed = (y_sign ^ with_x_quadrant.bitcast_i32()).bitcast_f32();

    let both_zero = x_is_zero & y_is_zero;
    let y_neg_zero = y.bitcast_i32().cmp_eq(SimdI32::<V>::set1(i32::MIN));
    let x_neg = x.cmp_lt(V::zeroes()).bitcast_i32();
    let signed_zero_result = y_neg_zero.bitcast_f32().blendv(V::zeroes(), V::set1(-0.0));
    let zero_quadrant = x_neg
        .bitcast_f32()
        .blendv(signed_zero_result, y_neg_zero.bitcast_f32().blendv(pi, -pi));
    let fast = both_zero.bitcast_f32().blendv(signed, zero_quadrant);

    patch_binary_exceptional_lanes(y, x, fast, exceptional_mask, scalar::atan2_u35_f32)
}

#[inline(always)]
pub(super) fn hypot_u35<V>(x: V, y: V) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    let abs_x = x.abs();
    let abs_y = y.abs();
    let max_xy = abs_x.cmp_gt(abs_y).blendv(abs_y, abs_x);
    let min_xy = abs_x.cmp_gt(abs_y).blendv(abs_x, abs_y);

    let finite_mask = x.cmp_eq(x).bitcast_i32()
        & y.cmp_eq(y).bitcast_i32()
        & abs_x.cmp_neq(V::set1(f32::INFINITY)).bitcast_i32()
        & abs_y.cmp_neq(V::set1(f32::INFINITY)).bitcast_i32();
    let max_nonzero = max_xy.cmp_neq(V::zeroes()).bitcast_i32();
    let fast_mask = finite_mask & max_nonzero;
    let exceptional_mask = fast_mask.cmp_eq(SimdI32::<V>::zeroes());

    let ratio = min_xy / max_xy;
    let fast = max_xy * (V::set1(1.0) + ratio * ratio).sqrt();

    patch_binary_exceptional_lanes(x, y, fast, exceptional_mask, scalar::hypot_u35_f32)
}
