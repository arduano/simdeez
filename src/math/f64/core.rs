use crate::math::{map, scalar};
use crate::{Simd, SimdBaseIo, SimdBaseOps, SimdConsts, SimdFloat64, SimdInt64};

type SimdI64<V> = <<V as SimdConsts>::Engine as Simd>::Vi64;

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
pub(crate) fn log2_u35<V>(input: V) -> V
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    map::unary_f64(input, scalar::log2_u35_f64)
}

#[inline(always)]
pub(crate) fn exp2_u35<V>(input: V) -> V
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    map::unary_f64(input, scalar::exp2_u35_f64)
}

#[inline(always)]
pub(crate) fn ln_u35<V>(input: V) -> V
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    map::unary_f64(input, scalar::ln_u35_f64)
}

#[inline(always)]
pub(crate) fn exp_u35<V>(input: V) -> V
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    map::unary_f64(input, scalar::exp_u35_f64)
}

#[inline(always)]
fn trig_exceptional_mask<V>(input: V) -> SimdI64<V>
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    let finite_mask = input.cmp_eq(input).bitcast_i64();
    let within_fast_range = input
        .abs()
        .cmp_lte(V::set1(core::f64::consts::FRAC_PI_4))
        .bitcast_i64();
    let non_zero = input.cmp_neq(V::zeroes()).bitcast_i64();
    (finite_mask & within_fast_range & non_zero).cmp_eq(SimdI64::<V>::zeroes())
}

#[inline(always)]
fn sin_cos_fast<V>(input: V) -> (V, V)
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    let two_over_pi = V::set1(core::f64::consts::FRAC_2_PI);
    let n = (input * two_over_pi).round().cast_i64();

    let n_f = n.cast_f64();
    let r = ((input - n_f * V::set1(core::f64::consts::FRAC_PI_2))
        - n_f * V::set1(6.123_233_995_736_766e-17))
        - n_f * V::set1(-2.022_266_248_795_951e-21);
    let r2 = r * r;

    let mut sin_poly = V::set1(1.589_690_995_211_55e-10);
    sin_poly = (sin_poly * r2) + V::set1(-2.505_076_025_340_686_3e-8);
    sin_poly = (sin_poly * r2) + V::set1(2.755_731_370_707_006_8e-6);
    sin_poly = (sin_poly * r2) + V::set1(-1.984_126_982_985_795e-4);
    sin_poly = (sin_poly * r2) + V::set1(8.333_333_333_322_49e-3);
    sin_poly = (sin_poly * r2) + V::set1(-1.666_666_666_666_632_4e-1);
    let sin_r = ((sin_poly * r2) * r) + r;

    let mut cos_poly = V::set1(-1.135_964_755_778_819_5e-11);
    cos_poly = (cos_poly * r2) + V::set1(2.087_572_321_298_175e-9);
    cos_poly = (cos_poly * r2) + V::set1(-2.755_731_435_139_066_3e-7);
    cos_poly = (cos_poly * r2) + V::set1(2.480_158_728_947_673e-5);
    cos_poly = (cos_poly * r2) + V::set1(-1.388_888_888_887_305_6e-3);
    cos_poly = (cos_poly * r2) + V::set1(4.166_666_666_666_659e-2);
    let cos_r = (cos_poly * r2 * r2) - (V::set1(0.5) * r2) + V::set1(1.0);

    let q = n & SimdI64::<V>::set1(3);
    let q0 = q.cmp_eq(SimdI64::<V>::zeroes()).bitcast_f64();
    let q1 = q.cmp_eq(SimdI64::<V>::set1(1)).bitcast_f64();
    let q2 = q.cmp_eq(SimdI64::<V>::set1(2)).bitcast_f64();

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
pub(crate) fn sin_u35<V>(input: V) -> V
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    let exceptional_mask = trig_exceptional_mask(input);
    let (sin_fast, _) = sin_cos_fast(input);
    patch_exceptional_lanes(input, sin_fast, exceptional_mask, scalar::sin_u35_f64)
}

#[inline(always)]
pub(crate) fn cos_u35<V>(input: V) -> V
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    let exceptional_mask = trig_exceptional_mask(input);
    let (_, cos_fast) = sin_cos_fast(input);
    patch_exceptional_lanes(input, cos_fast, exceptional_mask, scalar::cos_u35_f64)
}

#[inline(always)]
pub(crate) fn tan_u35<V>(input: V) -> V
where
    V: SimdFloat64,
    V::Engine: Simd<Vf64 = V>,
{
    let base_exceptional = trig_exceptional_mask(input);
    let (sin_fast, cos_fast) = sin_cos_fast(input);
    let dangerous = cos_fast.abs().cmp_lt(V::set1(1.0e-12)).bitcast_i64();
    let exceptional_mask = base_exceptional | dangerous;
    let fast = sin_fast / cos_fast;
    patch_exceptional_lanes(input, fast, exceptional_mask, scalar::tan_u35_f64)
}
