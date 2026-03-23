//! Hand-optimized AVX2/FMA overrides for f32 math kernels.
//!
//! Keep these overrides semantically aligned with `portable` kernels and always
//! reuse scalar exceptional-lane patching to preserve special-case contracts.

use crate::math::f32::portable;
use crate::math::scalar;
use crate::{Simd, SimdFloat32};

// DECISION(2026-03-23): KEEP_SIMD_OVERRIDE
// Function(s): f32 log2_u35 AVX2 override
// Why kept:
// - the AVX2 path is the fastest local benchmark variant for the restored log2 kernel
// - exceptional semantics still route through the shared portable scalar patching
// Revisit when:
// - the portable fallback catches up or semantic divergence appears

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

#[inline(always)]
pub(super) unsafe fn log2_u35<V>(input: V) -> V
where
    V: SimdFloat32,
    V::Engine: Simd<Vf32 = V>,
{
    debug_assert!(
        core::any::TypeId::of::<V::Engine>()
            == core::any::TypeId::of::<crate::engines::avx2::Avx2>()
    );

    let exceptional_mask = portable::log2_exceptional_mask(input);
    let x = input.try_transmute_avx2();
    let fast = log2_u35_avx2_impl(x);
    let fast = V::try_transmute_from_avx2(fast);

    portable::patch_exceptional_lanes(input, fast, exceptional_mask, scalar::log2_u35_f32)
}

#[target_feature(enable = "avx2", enable = "fma")]
unsafe fn log2_u35_avx2_impl(x: __m256) -> __m256 {
    let exponent_bits = _mm256_and_si256(
        _mm256_castps_si256(x),
        _mm256_set1_epi32(portable::F32_EXPONENT_MASK),
    );
    let mantissa_bits = _mm256_and_si256(
        _mm256_castps_si256(x),
        _mm256_set1_epi32(portable::F32_MANTISSA_MASK),
    );

    let exponent = _mm256_cvtepi32_ps(_mm256_sub_epi32(
        _mm256_srli_epi32(exponent_bits, 23),
        _mm256_set1_epi32(portable::F32_EXPONENT_BIAS_ADJUST),
    ));

    let normalized_mantissa = _mm256_castsi256_ps(_mm256_or_si256(
        mantissa_bits,
        _mm256_set1_epi32(portable::F32_LOG_NORM_MANTISSA),
    ));

    let one = _mm256_set1_ps(1.0);
    let adjust_mask = _mm256_cmp_ps(
        normalized_mantissa,
        _mm256_set1_ps(core::f32::consts::FRAC_1_SQRT_2),
        _CMP_LT_OQ,
    );

    let exponent = _mm256_sub_ps(exponent, _mm256_and_ps(adjust_mask, one));

    let reduced = _mm256_blendv_ps(
        _mm256_sub_ps(normalized_mantissa, one),
        _mm256_sub_ps(_mm256_add_ps(normalized_mantissa, normalized_mantissa), one),
        adjust_mask,
    );

    let reduced_sq = _mm256_mul_ps(reduced, reduced);

    let mut poly = _mm256_set1_ps(7.037_683_6e-2);
    poly = _mm256_fmadd_ps(poly, reduced, _mm256_set1_ps(-1.151_461e-1));
    poly = _mm256_fmadd_ps(poly, reduced, _mm256_set1_ps(1.167_699_9e-1));
    poly = _mm256_fmadd_ps(poly, reduced, _mm256_set1_ps(-1.242_014_1e-1));
    poly = _mm256_fmadd_ps(poly, reduced, _mm256_set1_ps(1.424_932_3e-1));
    poly = _mm256_fmadd_ps(poly, reduced, _mm256_set1_ps(-1.666_805_8e-1));
    poly = _mm256_fmadd_ps(poly, reduced, _mm256_set1_ps(2.000_071_5e-1));
    poly = _mm256_fmadd_ps(poly, reduced, _mm256_set1_ps(-2.499_999_4e-1));
    poly = _mm256_fmadd_ps(poly, reduced, _mm256_set1_ps(3.333_333e-1));

    poly = _mm256_mul_ps(poly, reduced);
    poly = _mm256_mul_ps(poly, reduced_sq);
    poly = _mm256_fmadd_ps(exponent, _mm256_set1_ps(-2.121_944_4e-4), poly);
    poly = _mm256_fnmadd_ps(_mm256_set1_ps(0.5), reduced_sq, poly);

    let ln_x = _mm256_fmadd_ps(
        exponent,
        _mm256_set1_ps(0.693_359_4),
        _mm256_add_ps(reduced, poly),
    );
    _mm256_mul_ps(ln_x, _mm256_set1_ps(core::f32::consts::LOG2_E))
}
