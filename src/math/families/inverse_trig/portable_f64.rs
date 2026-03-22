use crate::math::scalar;
use crate::{Simd, SimdBaseIo, SimdBaseOps, SimdConsts, SimdFloat64, SimdInt64};

type SimdI64<V> = <<V as SimdConsts>::Engine as Simd>::Vi64;

const F64_EXPONENT_MASK: i64 = 0x7FF0_0000_0000_0000u64 as i64;
const F64_SIGN_MASK: i64 = i64::MIN;

const ASIN_SQRT_FALLBACK_BOUND_BITS: u64 = 0x3FEF_F000_0000_0000;

const FRAC_PI_2: f64 = core::f64::consts::FRAC_PI_2;
const FRAC_PI_4: f64 = core::f64::consts::FRAC_PI_4;
const PI: f64 = core::f64::consts::PI;

const TAN_PI_8: f64 = 0.414_213_562_373_095_03;
const TAN_3PI_8: f64 = 2.414_213_562_373_095;

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
fn restore_sign<V>(magnitude: V, sign_bits: SimdI64<V>) -> V
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    (magnitude.bitcast_i64() ^ sign_bits).bitcast_f64()
}

#[inline(always)]
fn finite_mask<V>(input: V) -> SimdI64<V>
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    let exponent_bits = input.bitcast_i64() & SimdI64::<V>::set1(F64_EXPONENT_MASK);
    exponent_bits.cmp_neq(SimdI64::<V>::set1(F64_EXPONENT_MASK))
}

#[inline(always)]
fn atan_poly<V>(z: V) -> V
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    let p0 = V::set1(-8.750_608_600_031_904e-1);
    let p1 = V::set1(-1.615_753_718_733_365_2e1);
    let p2 = V::set1(-7.500_855_792_314_705e1);
    let p3 = V::set1(-1.228_866_684_490_136_2e2);
    let p4 = V::set1(-6.485_021_904_942_025e1);

    let q0 = V::set1(2.485_846_490_142_306_4e1);
    let q1 = V::set1(1.650_270_098_316_988_6e2);
    let q2 = V::set1(4.328_810_604_912_903e2);
    let q3 = V::set1(4.853_903_996_359_137e2);
    let q4 = V::set1(1.945_506_571_482_614e2);

    let numerator = (((((p0 * z) + p1) * z) + p2) * z + p3) * z + p4;
    let denominator = (((((z + q0) * z) + q1) * z + q2) * z + q3) * z + q4;
    numerator / denominator
}

#[inline(always)]
fn atan_reduced<V>(reduced: V) -> V
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    let z = reduced * reduced;
    reduced + reduced * z * atan_poly(z)
}

#[inline(always)]
fn atan_fast<V>(input: V) -> V
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    let sign_bits = input.bitcast_i64() & SimdI64::<V>::set1(F64_SIGN_MASK);
    let abs_input = input.abs();

    let large_mask = abs_input.cmp_gt(V::set1(TAN_3PI_8)).bitcast_i64();
    let medium_base = abs_input.cmp_gt(V::set1(TAN_PI_8)).bitcast_i64();
    let medium_mask = medium_base & large_mask.cmp_eq(SimdI64::<V>::zeroes());
    let large_mask_f = large_mask.bitcast_f64();
    let medium_mask_f = medium_mask.bitcast_f64();

    let mut reduced = abs_input;
    reduced = medium_mask_f.blendv(
        reduced,
        (abs_input - V::set1(1.0)) / (abs_input + V::set1(1.0)),
    );
    reduced = large_mask_f.blendv(reduced, -V::set1(1.0) / abs_input);

    let mut offset = V::zeroes();
    offset = medium_mask_f.blendv(offset, V::set1(FRAC_PI_4));
    offset = large_mask_f.blendv(offset, V::set1(FRAC_PI_2));

    let magnitude = offset + atan_reduced(reduced);
    restore_sign(magnitude, sign_bits)
}

#[inline(always)]
fn asin_exceptional_mask<V>(input: V) -> SimdI64<V>
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    let finite = finite_mask(input);
    let abs_input = input.abs();
    let out_of_domain = abs_input.cmp_gt(V::set1(1.0)).bitcast_i64();
    let near_edge = abs_input
        .bitcast_i64()
        .cmp_gt(SimdI64::<V>::set1(ASIN_SQRT_FALLBACK_BOUND_BITS as i64));
    finite.cmp_eq(SimdI64::<V>::zeroes()) | out_of_domain | near_edge
}

#[inline(always)]
fn acos_exceptional_mask<V>(input: V) -> SimdI64<V>
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    asin_exceptional_mask(input)
}

#[inline(always)]
fn asin_acos_core<V>(input: V) -> (V, V)
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    let one = V::set1(1.0);
    let half = V::set1(0.5);
    let abs_input = input.abs();
    let sign_bits = input.bitcast_i64() & SimdI64::<V>::set1(F64_SIGN_MASK);

    let small_mask = abs_input.cmp_lt(half).bitcast_i64();
    let small_mask_f = small_mask.bitcast_f64();

    let small_t = abs_input / (one - abs_input * abs_input).sqrt();
    let small_asin_mag = atan_fast(small_t);

    let large_t = ((one - abs_input) / (one + abs_input)).sqrt();
    let large_angle = V::set1(2.0) * atan_fast(large_t);
    let large_asin_mag = V::set1(FRAC_PI_2) - large_angle;

    let asin_mag = small_mask_f.blendv(large_asin_mag, small_asin_mag);
    let asin_out = restore_sign(asin_mag, sign_bits);

    let small_acos = V::set1(FRAC_PI_2) - asin_out;
    let large_acos_pos = large_angle;
    let large_acos_neg = V::set1(PI) - large_angle;
    let negative_mask = input.cmp_lt(V::zeroes()).bitcast_i64().bitcast_f64();
    let large_acos = negative_mask.blendv(large_acos_pos, large_acos_neg);
    let acos_out = small_mask_f.blendv(large_acos, small_acos);

    (asin_out, acos_out)
}

#[inline(always)]
pub(crate) fn asin_u35<V>(input: V) -> V
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    let exceptional_mask = asin_exceptional_mask(input);
    let (fast, _) = asin_acos_core(input);
    patch_exceptional_lanes(input, fast, exceptional_mask, scalar::asin_u35_f64)
}

#[inline(always)]
pub(crate) fn acos_u35<V>(input: V) -> V
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    let exceptional_mask = acos_exceptional_mask(input);
    let (_, fast) = asin_acos_core(input);
    patch_exceptional_lanes(input, fast, exceptional_mask, scalar::acos_u35_f64)
}

#[inline(always)]
pub(crate) fn atan_u35<V>(input: V) -> V
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    let fast = atan_fast(input);
    let exceptional_mask = finite_mask(input).cmp_eq(SimdI64::<V>::zeroes());
    patch_exceptional_lanes(input, fast, exceptional_mask, scalar::atan_u35_f64)
}
