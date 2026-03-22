use crate::math::scalar;
use crate::{Simd, SimdBaseIo, SimdBaseOps, SimdConsts, SimdFloat32, SimdInt, SimdInt32};

pub(super) type SimdI32<V> = <<V as SimdConsts>::Engine as Simd>::Vi32;

pub(super) const F32_EXPONENT_MASK: i32 = 0x7F80_0000u32 as i32;
pub(super) const F32_MANTISSA_MASK: i32 = 0x007F_FFFF;
pub(super) const F32_LOG_NORM_MANTISSA: i32 = 0x3F00_0000;
pub(super) const F32_EXPONENT_BIAS_ADJUST: i32 = 126;

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
pub(super) fn log2_exceptional_mask<V>(input: V) -> SimdI32<V>
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    let bits = input.bitcast_i32();
    let exponent_bits = bits & F32_EXPONENT_MASK;

    let non_positive = input
        .cmp_gt(V::zeroes())
        .bitcast_i32()
        .cmp_eq(SimdI32::<V>::zeroes());
    let subnormal_or_zero = exponent_bits.cmp_eq(SimdI32::<V>::zeroes());
    let inf_or_nan = exponent_bits.cmp_eq(SimdI32::<V>::set1(F32_EXPONENT_MASK));

    non_positive | subnormal_or_zero | inf_or_nan
}

#[inline(always)]
pub(super) fn patch_exceptional_lanes<V>(
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
pub(super) fn log2_u35<V>(input: V) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    let bits = input.bitcast_i32();
    let exponent_bits = bits & F32_EXPONENT_MASK;
    let mantissa_bits = bits & F32_MANTISSA_MASK;

    let exceptional_mask = log2_exceptional_mask(input);

    let exponent = (exponent_bits.shr(23) - F32_EXPONENT_BIAS_ADJUST).cast_f32();
    let normalized_mantissa = (mantissa_bits | F32_LOG_NORM_MANTISSA).bitcast_f32();

    let one = V::set1(1.0);
    let half = V::set1(0.5);
    let sqrt_half = V::set1(core::f32::consts::FRAC_1_SQRT_2);

    let adjust_mask = normalized_mantissa.cmp_lt(sqrt_half);
    let exponent = exponent - adjust_mask.blendv(V::zeroes(), one);
    let reduced = adjust_mask.blendv(
        normalized_mantissa - one,
        (normalized_mantissa + normalized_mantissa) - one,
    );

    let reduced_sq = reduced * reduced;

    let mut poly = V::set1(7.037_683_6e-2);
    poly = (poly * reduced) + V::set1(-1.151_461e-1);
    poly = (poly * reduced) + V::set1(1.167_699_9e-1);
    poly = (poly * reduced) + V::set1(-1.242_014_1e-1);
    poly = (poly * reduced) + V::set1(1.424_932_3e-1);
    poly = (poly * reduced) + V::set1(-1.666_805_8e-1);
    poly = (poly * reduced) + V::set1(2.000_071_5e-1);
    poly = (poly * reduced) + V::set1(-2.499_999_4e-1);
    poly = (poly * reduced) + V::set1(3.333_333e-1);

    poly *= reduced;
    poly *= reduced_sq;
    poly += exponent * V::set1(-2.121_944_4e-4);
    poly -= half * reduced_sq;

    let ln_x = reduced + poly + (exponent * V::set1(0.693_359_4));
    let fast = ln_x * V::set1(core::f32::consts::LOG2_E);

    patch_exceptional_lanes(input, fast, exceptional_mask, scalar::log2_u35_f32)
}

#[inline(always)]
pub(super) fn exp2_u35<V>(input: V) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    let finite_mask = input.cmp_eq(input).bitcast_i32();
    let in_lower_bound = input.cmp_gte(V::set1(-126.0)).bitcast_i32();
    let in_upper_bound = input.cmp_lte(V::set1(126.0)).bitcast_i32();
    let fast_mask = finite_mask & in_lower_bound & in_upper_bound;
    let exceptional_mask = fast_mask.cmp_eq(SimdI32::<V>::zeroes());

    let integral = input.floor().cast_i32();
    let fractional = input - integral.cast_f32();
    let reduced = fractional * V::set1(core::f32::consts::LN_2);

    let mut poly = V::set1(1.987_569_1e-4);
    poly = (poly * reduced) + V::set1(1.398_2e-3);
    poly = (poly * reduced) + V::set1(8.333_452e-3);
    poly = (poly * reduced) + V::set1(4.166_579_6e-2);
    poly = (poly * reduced) + V::set1(1.666_666_5e-1);
    poly = (poly * reduced) + V::set1(5e-1);

    let reduced_sq = reduced * reduced;
    let exp_reduced = (poly * reduced_sq) + reduced + V::set1(1.0);

    let exp_bits = (integral + 127).shl(23);
    let scale = exp_bits.bitcast_f32();
    let fast = exp_reduced * scale;

    patch_exceptional_lanes(input, fast, exceptional_mask, scalar::exp2_u35_f32)
}

#[inline(always)]
fn trig_exceptional_mask<V>(input: V) -> SimdI32<V>
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    let finite_mask = input.cmp_eq(input).bitcast_i32();
    let within_fast_range = input.abs().cmp_lte(V::set1(8192.0)).bitcast_i32();
    let non_zero_mask = input.cmp_neq(V::zeroes()).bitcast_i32();
    let fast_mask = finite_mask & within_fast_range & non_zero_mask;
    fast_mask.cmp_eq(SimdI32::<V>::zeroes())
}

#[inline(always)]
fn sin_cos_fast<V>(input: V) -> (V, V)
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    let two_over_pi = V::set1(core::f32::consts::FRAC_2_PI);
    let n = (input * two_over_pi).round().cast_i32();

    let n_f = n.cast_f32();
    let r = ((input - n_f * V::set1(1.570_312_5)) - n_f * V::set1(4.837_513e-4))
        - n_f * V::set1(7.549_789_4e-8);
    let r2 = r * r;

    let sin_poly = (((V::set1(-2.388_985_9e-8) * r2 + V::set1(2.752_556_2e-6)) * r2
        + V::set1(-1.984_127e-4))
        * r2
        + V::set1(8.333_331e-3))
        * r2
        + V::set1(-1.666_666_7e-1);
    let sin_r = ((sin_poly * r2) * r) + r;

    let cos_poly = (((V::set1(-2.605_161_5e-7) * r2 + V::set1(2.476_049_5e-5)) * r2
        + V::set1(-1.388_837_8e-3))
        * r2
        + V::set1(4.166_664_6e-2))
        * r2
        + V::set1(-5e-1);
    let cos_r = (cos_poly * r2) + V::set1(1.0);

    let q = n & SimdI32::<V>::set1(3);
    let q0 = q.cmp_eq(SimdI32::<V>::zeroes()).bitcast_f32();
    let q1 = q.cmp_eq(SimdI32::<V>::set1(1)).bitcast_f32();
    let q2 = q.cmp_eq(SimdI32::<V>::set1(2)).bitcast_f32();

    let mut sin_out = q0.blendv(V::zeroes(), sin_r);
    sin_out = q1.blendv(sin_out, cos_r);
    sin_out = q2.blendv(sin_out, -sin_r);
    sin_out = (q0 | q1 | q2).cmp_eq(V::zeroes()).blendv(sin_out, -cos_r);

    let mut cos_out = q0.blendv(V::zeroes(), cos_r);
    cos_out = q1.blendv(cos_out, -sin_r);
    cos_out = q2.blendv(cos_out, -cos_r);
    cos_out = (q0 | q1 | q2).cmp_eq(V::zeroes()).blendv(cos_out, sin_r);

    (sin_out, cos_out)
}

#[inline(always)]
pub(super) fn sin_u35<V>(input: V) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    let exceptional_mask = trig_exceptional_mask(input);
    let (sin_fast, _) = sin_cos_fast(input);
    patch_exceptional_lanes(input, sin_fast, exceptional_mask, scalar::sin_u35_f32)
}

#[inline(always)]
pub(super) fn cos_u35<V>(input: V) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    let exceptional_mask = trig_exceptional_mask(input);
    let (_, cos_fast) = sin_cos_fast(input);
    patch_exceptional_lanes(input, cos_fast, exceptional_mask, scalar::cos_u35_f32)
}

#[inline(always)]
pub(super) fn tan_u35<V>(input: V) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    let base_exceptional = trig_exceptional_mask(input);
    let (sin_fast, cos_fast) = sin_cos_fast(input);
    let dangerous = cos_fast.abs().cmp_lt(V::set1(1.0e-4)).bitcast_i32();
    let exceptional_mask = base_exceptional | dangerous;
    let fast = sin_fast / cos_fast;
    patch_exceptional_lanes(input, fast, exceptional_mask, scalar::tan_u35_f32)
}

#[inline(always)]
pub(super) fn asinh_u35<V>(input: V) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    let finite_mask = input.cmp_eq(input).bitcast_i32();
    let abs_x = input.abs();
    let tiny_mask = abs_x.cmp_lt(V::set1(0.05)).bitcast_i32();
    let large_mask = abs_x.cmp_gt(V::set1(1.0e19)).bitcast_i32();
    let zero_mask = input.cmp_eq(V::zeroes()).bitcast_i32();
    let exceptional_mask =
        finite_mask.cmp_eq(SimdI32::<V>::zeroes()) | tiny_mask | large_mask | zero_mask;

    let radicand = (abs_x * abs_x) + V::set1(1.0);
    let magnitude = log2_u35(abs_x + radicand.sqrt()) * V::set1(core::f32::consts::LN_2);
    let negative_mask = input.cmp_lt(V::zeroes());
    let fast = negative_mask.blendv(magnitude, -magnitude);

    patch_exceptional_lanes(input, fast, exceptional_mask, scalar::asinh_u35_f32)
}

#[inline(always)]
pub(super) fn acosh_u35<V>(input: V) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    let finite_mask = input.cmp_eq(input).bitcast_i32();
    let in_domain_mask = input.cmp_gte(V::set1(1.0)).bitcast_i32();
    let fast_mask = finite_mask & in_domain_mask;
    let exceptional_mask = fast_mask.cmp_eq(SimdI32::<V>::zeroes());

    let root_term = ((input - V::set1(1.0)).sqrt()) * ((input + V::set1(1.0)).sqrt());
    let fast = log2_u35(input + root_term) * V::set1(core::f32::consts::LN_2);

    patch_exceptional_lanes(input, fast, exceptional_mask, scalar::acosh_u35_f32)
}

#[inline(always)]
pub(super) fn atanh_u35<V>(input: V) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    let finite_mask = input.cmp_eq(input).bitcast_i32();
    let abs_x = input.abs();
    let strict_domain_mask = abs_x.cmp_lt(V::set1(1.0)).bitcast_i32();
    let non_zero_mask = input.cmp_neq(V::zeroes()).bitcast_i32();
    let stable_range_mask = abs_x.cmp_lte(V::set1(0.75)).bitcast_i32();
    let away_from_zero_mask = abs_x.cmp_gte(V::set1(0.05)).bitcast_i32();
    let fast_mask =
        finite_mask & strict_domain_mask & non_zero_mask & stable_range_mask & away_from_zero_mask;
    let exceptional_mask = fast_mask.cmp_eq(SimdI32::<V>::zeroes());

    let one = V::set1(1.0);
    let ratio = (one + input) / (one - input);
    let fast = log2_u35(ratio) * V::set1(0.5 * core::f32::consts::LN_2);

    patch_exceptional_lanes(input, fast, exceptional_mask, scalar::atanh_u35_f32)
}
