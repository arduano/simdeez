use crate::math::{map, scalar};
use crate::{Simd, SimdBaseIo, SimdBaseOps, SimdConsts, SimdFloat64, SimdInt, SimdInt64};

type SimdI64<V> = <<V as SimdConsts>::Engine as Simd>::Vi64;

const F64_EXPONENT_MASK: i64 = 0x7FF0_0000_0000_0000u64 as i64;
const F64_MANTISSA_MASK: i64 = 0x000F_FFFF_FFFF_FFFFu64 as i64;
const F64_LOG_NORM_MANTISSA: i64 = 0x3FE0_0000_0000_0000u64 as i64;
const F64_EXPONENT_BIAS_ADJUST: i64 = 1022;
const F64_EXP_LN2_HI: f64 = 6.931_471_803_691_238e-1;
const F64_EXP_LN2_LO: f64 = 1.908_214_929_270_587_7e-10;
// DECISION(2026-03-23): KEEP_SIMD_PORTABLE
// Function(s): f64 log2_u35 / exp2_u35 / ln_u35 / exp_u35
// Why kept:
// - the revived portable log/exp kernels now beat native scalar in local f64 core benches
// - exceptional lanes still patch back to scalar references without giving up the fast AVX2 path
// Revisit when:
// - non-x86 evidence disagrees sharply or a cheaper approximation family appears

// DECISION(2026-03-23): KEEP_SCALAR_REFERENCE
// Function(s): f64 sin_u35 / cos_u35 / tan_u35
// Why scalar:
// - the final retry of the old portable trig kernel failed the u35 contract around pi boundaries,
//   tan-pole neighborhoods, and moderate finite lanes before it could justify a speed keep
// - the refreshed scalar-reference recheck still leaves runtime-selected throughput behind native
//   scalar on this host (`sin`: about 17.03 ms vs 15.97 ms, `cos`: about 16.58 ms vs 15.75 ms,
//   `tan`: about 20.85 ms vs 20.19 ms)
// - native scalar still remains the honest default while family ownership stays localized here
// Revisit when:
// - a stronger range-reduction strategy or cheaper trig kernel appears

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
fn log2_exceptional_mask<V>(input: V) -> SimdI64<V>
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    let bits = input.bitcast_i64();
    let exponent_bits = bits & F64_EXPONENT_MASK;

    let non_positive = input
        .cmp_gt(V::zeroes())
        .bitcast_i64()
        .cmp_eq(SimdI64::<V>::zeroes());
    let subnormal_or_zero = exponent_bits.cmp_eq(SimdI64::<V>::zeroes());
    let inf_or_nan = exponent_bits.cmp_eq(SimdI64::<V>::set1(F64_EXPONENT_MASK));

    non_positive | subnormal_or_zero | inf_or_nan
}

#[inline(always)]
pub(crate) fn log2_u35<V>(input: V) -> V
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    let bits = input.bitcast_i64();
    let exponent_bits = bits & F64_EXPONENT_MASK;
    let mantissa_bits = bits & F64_MANTISSA_MASK;

    let exceptional_mask = log2_exceptional_mask(input);

    let exponent =
        (exponent_bits.shr(52) - SimdI64::<V>::set1(F64_EXPONENT_BIAS_ADJUST)).cast_f64();
    let normalized_mantissa = (mantissa_bits | F64_LOG_NORM_MANTISSA).bitcast_f64();

    let one = V::set1(1.0);
    let sqrt_half = V::set1(core::f64::consts::FRAC_1_SQRT_2);

    let adjust_mask = normalized_mantissa.cmp_lt(sqrt_half);
    let exponent = exponent - adjust_mask.blendv(V::zeroes(), one);
    let mantissa = adjust_mask.blendv(
        normalized_mantissa,
        normalized_mantissa + normalized_mantissa,
    );

    let t = (mantissa - one) / (mantissa + one);
    let t2 = t * t;

    let mut poly = V::set1(1.0 / 19.0);
    poly = (poly * t2) + V::set1(1.0 / 17.0);
    poly = (poly * t2) + V::set1(1.0 / 15.0);
    poly = (poly * t2) + V::set1(1.0 / 13.0);
    poly = (poly * t2) + V::set1(1.0 / 11.0);
    poly = (poly * t2) + V::set1(1.0 / 9.0);
    poly = (poly * t2) + V::set1(1.0 / 7.0);
    poly = (poly * t2) + V::set1(1.0 / 5.0);
    poly = (poly * t2) + V::set1(1.0 / 3.0);

    let log2_mantissa = V::set1(2.0 * core::f64::consts::LOG2_E) * t * ((poly * t2) + one);
    let fast = exponent + log2_mantissa;

    patch_exceptional_lanes(input, fast, exceptional_mask, scalar::log2_u35_f64)
}

#[inline(always)]
pub(crate) fn exp2_u35<V>(input: V) -> V
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    let finite_mask = input.cmp_eq(input).bitcast_i64();
    let in_lower_bound = input.cmp_gte(V::set1(-1022.0)).bitcast_i64();
    let in_upper_bound = input.cmp_lte(V::set1(1023.0)).bitcast_i64();
    let fast_mask = finite_mask & in_lower_bound & in_upper_bound;
    let exceptional_mask = fast_mask.cmp_eq(SimdI64::<V>::zeroes());

    let integral = input.round().cast_i64();
    let fractional = input - integral.cast_f64();
    let reduced = fractional * V::set1(core::f64::consts::LN_2);

    let mut poly = V::set1(1.0 / 479_001_600.0);
    poly = (poly * reduced) + V::set1(1.0 / 39_916_800.0);
    poly = (poly * reduced) + V::set1(1.0 / 3_628_800.0);
    poly = (poly * reduced) + V::set1(1.0 / 362_880.0);
    poly = (poly * reduced) + V::set1(1.0 / 40_320.0);
    poly = (poly * reduced) + V::set1(1.0 / 5_040.0);
    poly = (poly * reduced) + V::set1(1.0 / 720.0);
    poly = (poly * reduced) + V::set1(1.0 / 120.0);
    poly = (poly * reduced) + V::set1(1.0 / 24.0);
    poly = (poly * reduced) + V::set1(1.0 / 6.0);
    poly = (poly * reduced) + V::set1(0.5);

    let exp_reduced = (poly * reduced * reduced) + reduced + V::set1(1.0);
    let scale_bits = (integral + SimdI64::<V>::set1(1023)).shl(52);
    let scale = scale_bits.bitcast_f64();
    let fast = exp_reduced * scale;

    patch_exceptional_lanes(input, fast, exceptional_mask, scalar::exp2_u35_f64)
}

#[inline(always)]
pub(crate) fn ln_u35<V>(input: V) -> V
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    let bits = input.bitcast_i64();
    let exponent_bits = bits & F64_EXPONENT_MASK;
    let mantissa_bits = bits & F64_MANTISSA_MASK;

    let exceptional_mask = log2_exceptional_mask(input);

    let exponent =
        (exponent_bits.shr(52) - SimdI64::<V>::set1(F64_EXPONENT_BIAS_ADJUST)).cast_f64();
    let normalized_mantissa = (mantissa_bits | F64_LOG_NORM_MANTISSA).bitcast_f64();

    let one = V::set1(1.0);
    let sqrt_half = V::set1(core::f64::consts::FRAC_1_SQRT_2);

    let adjust_mask = normalized_mantissa.cmp_lt(sqrt_half);
    let exponent = exponent - adjust_mask.blendv(V::zeroes(), one);
    let mantissa = adjust_mask.blendv(
        normalized_mantissa,
        normalized_mantissa + normalized_mantissa,
    );

    let t = (mantissa - one) / (mantissa + one);
    let t2 = t * t;

    let mut poly = V::set1(1.0 / 19.0);
    poly = (poly * t2) + V::set1(1.0 / 17.0);
    poly = (poly * t2) + V::set1(1.0 / 15.0);
    poly = (poly * t2) + V::set1(1.0 / 13.0);
    poly = (poly * t2) + V::set1(1.0 / 11.0);
    poly = (poly * t2) + V::set1(1.0 / 9.0);
    poly = (poly * t2) + V::set1(1.0 / 7.0);
    poly = (poly * t2) + V::set1(1.0 / 5.0);
    poly = (poly * t2) + V::set1(1.0 / 3.0);

    let ln_mantissa = V::set1(2.0) * t * ((poly * t2) + one);
    let fast = exponent * V::set1(core::f64::consts::LN_2) + ln_mantissa;

    patch_exceptional_lanes(input, fast, exceptional_mask, scalar::ln_u35_f64)
}

#[inline(always)]
pub(crate) fn exp_u35<V>(input: V) -> V
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    let finite_mask = input.cmp_eq(input).bitcast_i64();
    let in_lower_bound = input.cmp_gte(V::set1(-708.0)).bitcast_i64();
    let in_upper_bound = input.cmp_lte(V::set1(709.0)).bitcast_i64();
    let fast_mask = finite_mask & in_lower_bound & in_upper_bound;
    let exceptional_mask = fast_mask.cmp_eq(SimdI64::<V>::zeroes());

    let scaled = input * V::set1(core::f64::consts::LOG2_E);
    let integral = scaled.round().cast_i64();
    let integral_f = integral.cast_f64();
    let reduced =
        (input - integral_f * V::set1(F64_EXP_LN2_HI)) - integral_f * V::set1(F64_EXP_LN2_LO);

    let mut poly = V::set1(1.0 / 479_001_600.0);
    poly = (poly * reduced) + V::set1(1.0 / 39_916_800.0);
    poly = (poly * reduced) + V::set1(1.0 / 3_628_800.0);
    poly = (poly * reduced) + V::set1(1.0 / 362_880.0);
    poly = (poly * reduced) + V::set1(1.0 / 40_320.0);
    poly = (poly * reduced) + V::set1(1.0 / 5_040.0);
    poly = (poly * reduced) + V::set1(1.0 / 720.0);
    poly = (poly * reduced) + V::set1(1.0 / 120.0);
    poly = (poly * reduced) + V::set1(1.0 / 24.0);
    poly = (poly * reduced) + V::set1(1.0 / 6.0);
    poly = (poly * reduced) + V::set1(0.5);

    let exp_reduced = (poly * reduced * reduced) + reduced + V::set1(1.0);
    let scale_bits = (integral + SimdI64::<V>::set1(1023)).shl(52);
    let scale = scale_bits.bitcast_f64();
    let fast = exp_reduced * scale;

    patch_exceptional_lanes(input, fast, exceptional_mask, scalar::exp_u35_f64)
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
