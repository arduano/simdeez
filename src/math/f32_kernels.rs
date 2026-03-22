use crate::math::scalar;
use crate::{Simd, SimdBaseIo, SimdBaseOps, SimdConsts, SimdFloat32, SimdInt, SimdInt32};

type SimdI32<V> = <<V as SimdConsts>::Engine as Simd>::Vi32;

const F32_EXPONENT_MASK: i32 = 0x7F80_0000u32 as i32;
const F32_MANTISSA_MASK: i32 = 0x007F_FFFF;
const F32_LOG_NORM_MANTISSA: i32 = 0x3F00_0000;
const F32_EXPONENT_BIAS_ADJUST: i32 = 126;

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
fn patch_exceptional_lanes<V>(
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

        V::load_from_array(output_lanes)
    }
}

#[inline(always)]
pub(crate) fn log2_u35<V>(input: V) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    let bits = input.bitcast_i32();
    let exponent_bits = bits & F32_EXPONENT_MASK;
    let mantissa_bits = bits & F32_MANTISSA_MASK;

    let non_positive = input
        .cmp_gt(V::zeroes())
        .bitcast_i32()
        .cmp_eq(SimdI32::<V>::zeroes());
    let subnormal_or_zero = exponent_bits.cmp_eq(SimdI32::<V>::zeroes());
    let inf_or_nan = exponent_bits.cmp_eq(SimdI32::<V>::set1(F32_EXPONENT_MASK));
    let exceptional_mask = non_positive | subnormal_or_zero | inf_or_nan;

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
pub(crate) fn exp2_u35<V>(input: V) -> V
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
