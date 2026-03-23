use crate::math::scalar;
use crate::{Simd, SimdBaseIo, SimdBaseOps, SimdConsts, SimdFloat32, SimdInt32};

type SimdI32<V> = <<V as SimdConsts>::Engine as Simd>::Vi32;

const SIGN_MASK: i32 = 0x8000_0000u32 as i32;
const SINH_COSH_SMALL_ABS: f32 = 0.5;
const SINH_COSH_FAST_ABS_MAX: f32 = 40.0;
const TANH_SMALL_ABS: f32 = 0.625;
const TANH_FAST_ABS_MAX: f32 = 40.0;

// DECISION(2026-03-23): KEEP_SIMD_PORTABLE
// Function(s): f32 sinh_u35 / cosh_u35 / tanh_u35
// Why kept:
// - local benches show large wins over native scalar across runtime-selected and AVX2 paths
// - the family already centralizes exceptional-lane scalar patching cleanly
// Revisit when:
// - the exp/log backbone or fast-range cutovers change materially

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
fn apply_input_sign<V>(magnitude: V, input: V) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    let sign_bits = input.bitcast_i32() & SimdI32::<V>::set1(SIGN_MASK);
    (magnitude.bitcast_i32() | sign_bits).bitcast_f32()
}

#[inline(always)]
fn exp_u35<V>(input: V) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    super::portable::exp2_u35(input * V::set1(core::f32::consts::LOG2_E))
}

#[inline(always)]
fn sinh_small<V>(input: V, input_sq: V) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    let poly =
        ((V::set1(1.0 / 5040.0) * input_sq) + V::set1(1.0 / 120.0)) * input_sq + V::set1(1.0 / 6.0);
    input + (input * input_sq * poly)
}

#[inline(always)]
fn cosh_small<V>(input_sq: V) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    let poly = ((V::set1(1.0 / 720.0) * input_sq) + V::set1(1.0 / 24.0)) * input_sq + V::set1(0.5);
    V::set1(1.0) + (input_sq * poly)
}

#[inline(always)]
fn sinh_cosh_medium<V>(abs_input: V) -> (V, V)
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
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
fn sinh_cosh_masks<V>(input: V) -> (SimdI32<V>, V, V)
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    let abs_input = input.abs();
    let finite_mask = input.cmp_eq(input).bitcast_i32();
    let within_fast_range = abs_input
        .cmp_lte(V::set1(SINH_COSH_FAST_ABS_MAX))
        .bitcast_i32();

    (finite_mask & within_fast_range, abs_input, input * input)
}

#[inline(always)]
pub(super) fn sinh_u35<V>(input: V) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    let (fast_mask, abs_input, input_sq) = sinh_cosh_masks(input);
    let exceptional_mask = fast_mask.cmp_eq(SimdI32::<V>::zeroes());
    let small_mask = abs_input.cmp_lt(V::set1(SINH_COSH_SMALL_ABS));

    let fast_small = sinh_small(input, input_sq);
    let (sinh_medium, _) = sinh_cosh_medium(abs_input);
    let fast = small_mask.blendv(apply_input_sign(sinh_medium, input), fast_small);

    patch_exceptional_lanes(input, fast, exceptional_mask, scalar::sinh_u35_f32)
}

#[inline(always)]
pub(super) fn cosh_u35<V>(input: V) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    let (fast_mask, abs_input, input_sq) = sinh_cosh_masks(input);
    let exceptional_mask = fast_mask.cmp_eq(SimdI32::<V>::zeroes());
    let small_mask = abs_input.cmp_lt(V::set1(SINH_COSH_SMALL_ABS));

    let fast_small = cosh_small(input_sq);
    let (_, cosh_medium) = sinh_cosh_medium(abs_input);
    let fast = small_mask.blendv(cosh_medium, fast_small);

    patch_exceptional_lanes(input, fast, exceptional_mask, scalar::cosh_u35_f32)
}

#[inline(always)]
pub(super) fn tanh_u35<V>(input: V) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    let abs_input = input.abs();
    let finite_mask = input.cmp_eq(input).bitcast_i32();
    let within_fast_range = abs_input.cmp_lte(V::set1(TANH_FAST_ABS_MAX)).bitcast_i32();
    let exceptional_mask = (finite_mask & within_fast_range).cmp_eq(SimdI32::<V>::zeroes());
    let small_mask = abs_input.cmp_lt(V::set1(TANH_SMALL_ABS));

    let input_sq = input * input;
    let fast_small = sinh_small(input, input_sq) / cosh_small(input_sq);

    let exp_neg_2x = exp_u35(abs_input * V::set1(-2.0));
    let tanh_medium = (V::set1(1.0) - exp_neg_2x) / (V::set1(1.0) + exp_neg_2x);
    let fast = small_mask.blendv(apply_input_sign(tanh_medium, input), fast_small);

    patch_exceptional_lanes(input, fast, exceptional_mask, scalar::tanh_u35_f32)
}
