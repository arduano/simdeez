//! Portable SIMD math scaffolding for SLEEF-style transcendental families.
//!
//! Strategy C baseline: keep semantics in-tree and backend-agnostic by expressing
//! vector math over existing simdeez vector types.
//!
//! `f32` `log2_u35` / `exp2_u35` flow through a layered kernel stack:
//! portable SIMD kernels first, optional backend overrides where available,
//! and scalar-lane fallback for exceptional semantics.
//! `ln_u35` / `exp_u35` currently stay on deterministic scalar references.
//! `sin_u35` / `cos_u35` / `tan_u35` use portable SIMD range reduction with
//! centralized fallback for non-finite, very-large, and tan-pole-adjacent lanes.
//! Remaining historical SLEEF surface in this baseline pass is currently
//! lane-wise scalar mapped for correctness-first portability.

mod f32;
mod scalar;

use crate::{Simd, SimdFloat32, SimdFloat64};

/// Accuracy contracts for currently restored math families.
pub mod contracts {
    pub const LOG2_U35_F32_MAX_ULP: u32 = 35;
    pub const EXP2_U35_F32_MAX_ULP: u32 = 35;
    pub const LN_U35_F32_MAX_ULP: u32 = 1;
    pub const EXP_U35_F32_MAX_ULP: u32 = 1;
    pub const SIN_U35_F32_MAX_ULP: u32 = 35;
    pub const COS_U35_F32_MAX_ULP: u32 = 35;
    pub const TAN_U35_F32_MAX_ULP: u32 = 35;

    // Portable-baseline mappings currently use scalar reference implementations.
    pub const ASIN_U35_F32_MAX_ULP: u32 = 1;
    pub const ACOS_U35_F32_MAX_ULP: u32 = 1;
    pub const ATAN_U35_F32_MAX_ULP: u32 = 1;
    pub const ATAN2_U35_F32_MAX_ULP: u32 = 1;
    pub const SINH_U35_F32_MAX_ULP: u32 = 1;
    pub const COSH_U35_F32_MAX_ULP: u32 = 1;
    pub const TANH_U35_F32_MAX_ULP: u32 = 1;
    pub const ASINH_U35_F32_MAX_ULP: u32 = 1;
    pub const ACOSH_U35_F32_MAX_ULP: u32 = 1;
    pub const ATANH_U35_F32_MAX_ULP: u32 = 1;
    pub const HYPOT_U35_F32_MAX_ULP: u32 = 1;
    pub const LOG10_U35_F32_MAX_ULP: u32 = 1;

    pub const LOG2_U35_F64_MAX_ULP: u64 = 35;
    pub const EXP2_U35_F64_MAX_ULP: u64 = 35;
    pub const LN_U35_F64_MAX_ULP: u64 = 1;
    pub const EXP_U35_F64_MAX_ULP: u64 = 1;
    pub const SIN_U35_F64_MAX_ULP: u64 = 1;
    pub const COS_U35_F64_MAX_ULP: u64 = 1;
    pub const TAN_U35_F64_MAX_ULP: u64 = 1;

    pub const ASIN_U35_F64_MAX_ULP: u64 = 1;
    pub const ACOS_U35_F64_MAX_ULP: u64 = 1;
    pub const ATAN_U35_F64_MAX_ULP: u64 = 1;
    pub const ATAN2_U35_F64_MAX_ULP: u64 = 1;
    pub const SINH_U35_F64_MAX_ULP: u64 = 1;
    pub const COSH_U35_F64_MAX_ULP: u64 = 1;
    pub const TANH_U35_F64_MAX_ULP: u64 = 1;
    pub const ASINH_U35_F64_MAX_ULP: u64 = 1;
    pub const ACOSH_U35_F64_MAX_ULP: u64 = 1;
    pub const ATANH_U35_F64_MAX_ULP: u64 = 1;
    pub const HYPOT_U35_F64_MAX_ULP: u64 = 1;
    pub const LOG10_U35_F64_MAX_ULP: u64 = 1;
}

#[inline(always)]
fn map_unary_f32<V: SimdFloat32>(input: V, f: impl Fn(f32) -> f32) -> V {
    unsafe {
        let mut lanes = input.as_array();
        for i in 0..V::WIDTH {
            lanes[i] = f(lanes[i]);
        }
        V::load_from_ptr_unaligned(&lanes as *const V::ArrayRepresentation as *const f32)
    }
}

#[inline(always)]
fn map_unary_f64<V: SimdFloat64>(input: V, f: impl Fn(f64) -> f64) -> V {
    unsafe {
        let mut lanes = input.as_array();
        for i in 0..V::WIDTH {
            lanes[i] = f(lanes[i]);
        }
        V::load_from_ptr_unaligned(&lanes as *const V::ArrayRepresentation as *const f64)
    }
}

#[inline(always)]
fn map_binary_f32<V: SimdFloat32>(lhs: V, rhs: V, f: impl Fn(f32, f32) -> f32) -> V {
    unsafe {
        let lhs_lanes = lhs.as_array();
        let rhs_lanes = rhs.as_array();
        let mut out_lanes = lhs_lanes.clone();
        for i in 0..V::WIDTH {
            out_lanes[i] = f(lhs_lanes[i], rhs_lanes[i]);
        }
        V::load_from_ptr_unaligned(&out_lanes as *const V::ArrayRepresentation as *const f32)
    }
}

#[inline(always)]
fn map_binary_f64<V: SimdFloat64>(lhs: V, rhs: V, f: impl Fn(f64, f64) -> f64) -> V {
    unsafe {
        let lhs_lanes = lhs.as_array();
        let rhs_lanes = rhs.as_array();
        let mut out_lanes = lhs_lanes.clone();
        for i in 0..V::WIDTH {
            out_lanes[i] = f(lhs_lanes[i], rhs_lanes[i]);
        }
        V::load_from_ptr_unaligned(&out_lanes as *const V::ArrayRepresentation as *const f64)
    }
}

pub trait SimdMathF32: SimdFloat32 {
    #[inline(always)]
    fn log2_u35(self) -> Self
    where
        Self::Engine: Simd<Vf32 = Self>,
    {
        f32::log2_u35(self)
    }

    #[inline(always)]
    fn exp2_u35(self) -> Self
    where
        Self::Engine: Simd<Vf32 = Self>,
    {
        f32::exp2_u35(self)
    }

    #[inline(always)]
    fn ln_u35(self) -> Self {
        map_unary_f32(self, scalar::ln_u35_f32)
    }

    #[inline(always)]
    fn exp_u35(self) -> Self {
        map_unary_f32(self, scalar::exp_u35_f32)
    }

    #[inline(always)]
    fn sin_u35(self) -> Self
    where
        Self::Engine: Simd<Vf32 = Self>,
    {
        f32::sin_u35(self)
    }

    #[inline(always)]
    fn cos_u35(self) -> Self
    where
        Self::Engine: Simd<Vf32 = Self>,
    {
        f32::cos_u35(self)
    }

    #[inline(always)]
    fn tan_u35(self) -> Self
    where
        Self::Engine: Simd<Vf32 = Self>,
    {
        f32::tan_u35(self)
    }

    #[inline(always)]
    fn asin_u35(self) -> Self {
        map_unary_f32(self, scalar::asin_u35_f32)
    }

    #[inline(always)]
    fn acos_u35(self) -> Self {
        map_unary_f32(self, scalar::acos_u35_f32)
    }

    #[inline(always)]
    fn atan_u35(self) -> Self {
        map_unary_f32(self, scalar::atan_u35_f32)
    }

    #[inline(always)]
    fn sinh_u35(self) -> Self {
        map_unary_f32(self, scalar::sinh_u35_f32)
    }

    #[inline(always)]
    fn cosh_u35(self) -> Self {
        map_unary_f32(self, scalar::cosh_u35_f32)
    }

    #[inline(always)]
    fn tanh_u35(self) -> Self {
        map_unary_f32(self, scalar::tanh_u35_f32)
    }

    #[inline(always)]
    fn asinh_u35(self) -> Self {
        map_unary_f32(self, scalar::asinh_u35_f32)
    }

    #[inline(always)]
    fn acosh_u35(self) -> Self {
        map_unary_f32(self, scalar::acosh_u35_f32)
    }

    #[inline(always)]
    fn atanh_u35(self) -> Self {
        map_unary_f32(self, scalar::atanh_u35_f32)
    }

    #[inline(always)]
    fn log10_u35(self) -> Self {
        map_unary_f32(self, scalar::log10_u35_f32)
    }

    #[inline(always)]
    fn atan2_u35(self, x: Self) -> Self {
        map_binary_f32(self, x, scalar::atan2_u35_f32)
    }

    #[inline(always)]
    fn hypot_u35(self, y: Self) -> Self {
        map_binary_f32(self, y, scalar::hypot_u35_f32)
    }

    /// Floating-point remainder with C/libm `fmod` semantics (sign follows dividend).
    #[inline(always)]
    fn fmod(self, y: Self) -> Self {
        map_binary_f32(self, y, scalar::fmod_f32)
    }
}

impl<T: SimdFloat32> SimdMathF32 for T {}

pub trait SimdMathF64: SimdFloat64 {
    #[inline(always)]
    fn log2_u35(self) -> Self {
        map_unary_f64(self, scalar::log2_u35_f64)
    }

    #[inline(always)]
    fn exp2_u35(self) -> Self {
        map_unary_f64(self, scalar::exp2_u35_f64)
    }

    #[inline(always)]
    fn ln_u35(self) -> Self {
        map_unary_f64(self, scalar::ln_u35_f64)
    }

    #[inline(always)]
    fn exp_u35(self) -> Self {
        map_unary_f64(self, scalar::exp_u35_f64)
    }

    #[inline(always)]
    fn sin_u35(self) -> Self {
        map_unary_f64(self, scalar::sin_u35_f64)
    }

    #[inline(always)]
    fn cos_u35(self) -> Self {
        map_unary_f64(self, scalar::cos_u35_f64)
    }

    #[inline(always)]
    fn tan_u35(self) -> Self {
        map_unary_f64(self, scalar::tan_u35_f64)
    }

    #[inline(always)]
    fn asin_u35(self) -> Self {
        map_unary_f64(self, scalar::asin_u35_f64)
    }

    #[inline(always)]
    fn acos_u35(self) -> Self {
        map_unary_f64(self, scalar::acos_u35_f64)
    }

    #[inline(always)]
    fn atan_u35(self) -> Self {
        map_unary_f64(self, scalar::atan_u35_f64)
    }

    #[inline(always)]
    fn sinh_u35(self) -> Self {
        map_unary_f64(self, scalar::sinh_u35_f64)
    }

    #[inline(always)]
    fn cosh_u35(self) -> Self {
        map_unary_f64(self, scalar::cosh_u35_f64)
    }

    #[inline(always)]
    fn tanh_u35(self) -> Self {
        map_unary_f64(self, scalar::tanh_u35_f64)
    }

    #[inline(always)]
    fn asinh_u35(self) -> Self {
        map_unary_f64(self, scalar::asinh_u35_f64)
    }

    #[inline(always)]
    fn acosh_u35(self) -> Self {
        map_unary_f64(self, scalar::acosh_u35_f64)
    }

    #[inline(always)]
    fn atanh_u35(self) -> Self {
        map_unary_f64(self, scalar::atanh_u35_f64)
    }

    #[inline(always)]
    fn log10_u35(self) -> Self {
        map_unary_f64(self, scalar::log10_u35_f64)
    }

    #[inline(always)]
    fn atan2_u35(self, x: Self) -> Self {
        map_binary_f64(self, x, scalar::atan2_u35_f64)
    }

    #[inline(always)]
    fn hypot_u35(self, y: Self) -> Self {
        map_binary_f64(self, y, scalar::hypot_u35_f64)
    }

    /// Floating-point remainder with C/libm `fmod` semantics (sign follows dividend).
    #[inline(always)]
    fn fmod(self, y: Self) -> Self {
        map_binary_f64(self, y, scalar::fmod_f64)
    }
}

impl<T: SimdFloat64> SimdMathF64 for T {}
