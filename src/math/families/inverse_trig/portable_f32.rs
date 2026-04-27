use crate::math::scalar;
use crate::{Simd, SimdBaseIo, SimdBaseOps, SimdConsts, SimdFloat32, SimdInt32};

// DECISION(2026-03-23): KEEP_SIMD_PORTABLE
// Function(s): f32 asin_u35 / acos_u35 / atan_u35
// Why kept:
// - these remain some of the strongest portable SIMD wins in local benchmarks
// - targeted near-edge and symmetry tests match the current reduction thresholds
// Revisit when:
// - the approximation family or fallback boundaries move materially

type SimdI32<V> = <<V as SimdConsts>::Engine as Simd>::Vi32;

const F32_EXPONENT_MASK: i32 = 0x7F80_0000u32 as i32;
const F32_SIGN_MASK: i32 = i32::MIN;

const ASIN_SQRT_FALLBACK_BOUND_BITS: u32 = 0x3F7F_F000;

const FRAC_PI_2_HI: f32 = f32::from_bits(0x3FC9_0F80);
const FRAC_PI_2_LO: f32 = core::f32::consts::FRAC_PI_2 - FRAC_PI_2_HI;
const FRAC_PI_4_HI: f32 = f32::from_bits(0x3F49_0F80);
const FRAC_PI_4_LO: f32 = core::f32::consts::FRAC_PI_4 - FRAC_PI_4_HI;
const PI_HI: f32 = f32::from_bits(0x4049_0F80);
const PI_LO: f32 = core::f32::consts::PI - PI_HI;

const TAN_PI_8: f32 = 0.414_213_57;
const TAN_3PI_8: f32 = 2.414_213_7;

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

        V::load_from_ptr_unaligned(&output_lanes as *const V::ArrayRepresentation as *const f32)
    }
}

#[inline(always)]
fn restore_sign<V>(magnitude: V, sign_bits: SimdI32<V>) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    (magnitude.bitcast_i32() ^ sign_bits).bitcast_f32()
}

#[inline(always)]
fn finite_mask<V>(input: V) -> SimdI32<V>
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    let exponent_bits = input.bitcast_i32() & F32_EXPONENT_MASK;
    exponent_bits.cmp_neq(SimdI32::<V>::set1(F32_EXPONENT_MASK))
}

#[inline(always)]
fn asin_poly<V>(z: V) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    let mut poly = V::set1(4.216_319_8e-2);
    poly = (poly * z) + V::set1(2.418_131e-2);
    poly = (poly * z) + V::set1(4.547_003e-2);
    poly = (poly * z) + V::set1(7.495_300_5e-2);
    (poly * z) + V::set1(1.666_675_2e-1)
}

#[inline(always)]
fn atan_poly<V>(z: V) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    let mut poly = V::set1(8.053_745e-2);
    poly = (poly * z) + V::set1(-1.387_768_5e-1);
    poly = (poly * z) + V::set1(1.997_771_1e-1);
    (poly * z) + V::set1(-3.333_295e-1)
}

#[inline(always)]
fn asin_exceptional_mask<V>(input: V) -> SimdI32<V>
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    let finite = finite_mask(input);
    let abs_input = input.abs();
    let out_of_domain = abs_input.cmp_gt(V::set1(1.0)).bitcast_i32();
    let near_edge = abs_input
        .bitcast_i32()
        .cmp_gt(SimdI32::<V>::set1(ASIN_SQRT_FALLBACK_BOUND_BITS as i32));
    finite.cmp_eq(SimdI32::<V>::zeroes()) | out_of_domain | near_edge
}

#[inline(always)]
fn acos_exceptional_mask<V>(input: V) -> SimdI32<V>
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    let finite = finite_mask(input);
    let abs_input = input.abs();
    let out_of_domain = abs_input.cmp_gt(V::set1(1.0)).bitcast_i32();
    let near_one = input
        .bitcast_i32()
        .cmp_gt(SimdI32::<V>::set1(ASIN_SQRT_FALLBACK_BOUND_BITS as i32));
    finite.cmp_eq(SimdI32::<V>::zeroes()) | out_of_domain | near_one
}

#[inline(always)]
fn asin_acos_core<V>(input: V) -> (V, V)
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    let one = V::set1(1.0);
    let half = V::set1(0.5);
    let abs_input = input.abs();
    let sign_bits = input.bitcast_i32() & SimdI32::<V>::set1(F32_SIGN_MASK);

    let large_mask = abs_input.cmp_gt(half).bitcast_i32();
    let large_mask_f = large_mask.bitcast_f32();

    let small_z = abs_input * abs_input;
    let small_mag = abs_input + (abs_input * small_z) * asin_poly(small_z);

    let large_z = (one - abs_input) * half;
    let large_s = large_z.sqrt();
    let large_term = (large_s + (large_s * large_z) * asin_poly(large_z))
        + (large_s * large_z) * asin_poly(large_z);
    let large_mag = (V::set1(FRAC_PI_2_HI) - (large_s + large_term)) + V::set1(FRAC_PI_2_LO);

    let asin_mag = large_mask_f.blendv(small_mag, large_mag);
    let asin_out = restore_sign(asin_mag, sign_bits);

    let small_acos = (V::set1(FRAC_PI_2_HI) - asin_out) + V::set1(FRAC_PI_2_LO);
    let large_acos_pos = large_s + large_term;
    let large_acos_neg = (V::set1(PI_HI) - large_acos_pos) + V::set1(PI_LO);

    let negative_mask = input.cmp_lt(V::zeroes()).bitcast_i32().bitcast_f32();
    let large_acos = negative_mask.blendv(large_acos_pos, large_acos_neg);
    let acos_out = large_mask_f.blendv(small_acos, large_acos);

    (asin_out, acos_out)
}

#[inline(always)]
pub(super) fn asin_u35<V>(input: V) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    let exceptional_mask = asin_exceptional_mask(input);
    let (fast, _) = asin_acos_core(input);
    patch_exceptional_lanes(input, fast, exceptional_mask, scalar::asin_u35_f32)
}

#[inline(always)]
pub(super) fn acos_u35<V>(input: V) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    let exceptional_mask = acos_exceptional_mask(input);
    let (_, fast) = asin_acos_core(input);
    patch_exceptional_lanes(input, fast, exceptional_mask, scalar::acos_u35_f32)
}

#[inline(always)]
pub(super) fn atan_u35<V>(input: V) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    let sign_bits = input.bitcast_i32() & SimdI32::<V>::set1(F32_SIGN_MASK);
    let abs_input = input.abs();

    let large_mask = abs_input.cmp_gt(V::set1(TAN_3PI_8)).bitcast_i32();
    let medium_base = abs_input.cmp_gt(V::set1(TAN_PI_8)).bitcast_i32();
    let medium_mask = medium_base & large_mask.cmp_eq(SimdI32::<V>::zeroes());
    let large_mask_f = large_mask.bitcast_f32();
    let medium_mask_f = medium_mask.bitcast_f32();

    let mut reduced = abs_input;
    reduced = medium_mask_f.blendv(
        reduced,
        (abs_input - V::set1(1.0)) / (abs_input + V::set1(1.0)),
    );
    reduced = large_mask_f.blendv(reduced, -V::set1(1.0) / abs_input);

    let z = reduced * reduced;
    let atan_reduced = reduced + (reduced * z) * atan_poly(z);

    let mut offset = V::zeroes();
    offset = medium_mask_f.blendv(offset, V::set1(FRAC_PI_4_HI + FRAC_PI_4_LO));
    offset = large_mask_f.blendv(offset, V::set1(FRAC_PI_2_HI + FRAC_PI_2_LO));

    let magnitude = offset + atan_reduced;
    let fast = restore_sign(magnitude, sign_bits);
    let exceptional_mask = finite_mask(input).cmp_eq(SimdI32::<V>::zeroes());

    patch_exceptional_lanes(input, fast, exceptional_mask, scalar::atan_u35_f32)
}
