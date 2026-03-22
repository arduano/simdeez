//! Portable SIMD math scaffolding for SLEEF-style transcendental families.
//!
//! Strategy C baseline: keep semantics in-tree and backend-agnostic by expressing
//! vector math over existing simdeez vector types.
//!
//! `f32` `log2_u35` / `exp2_u35` are implemented with native SIMD reduction and
//! polynomial kernels, with scalar-lane fallback only for exceptional lanes.
//! `ln_u35` / `exp_u35` currently stay on deterministic scalar references.

mod f32_kernels;
mod scalar;

use crate::{Simd, SimdFloat32, SimdFloat64};

/// Accuracy contracts for currently restored math families.
pub mod contracts {
    /// Maximum ULP error target for the f32 `log2_u35` kernel family.
    pub const LOG2_U35_F32_MAX_ULP: u32 = 35;

    /// Maximum ULP error target for the f32 `exp2_u35` kernel family.
    pub const EXP2_U35_F32_MAX_ULP: u32 = 35;

    /// Maximum ULP error target for the f64 `log2_u35` kernel family.
    pub const LOG2_U35_F64_MAX_ULP: u64 = 35;

    /// Maximum ULP error target for the f64 `exp2_u35` kernel family.
    pub const EXP2_U35_F64_MAX_ULP: u64 = 35;

    /// Maximum ULP error target for f32 `ln_u35`.
    pub const LN_U35_F32_MAX_ULP: u32 = 1;

    /// Maximum ULP error target for f32 `exp_u35`.
    pub const EXP_U35_F32_MAX_ULP: u32 = 1;

    /// Maximum ULP error target for f64 `ln_u35`.
    pub const LN_U35_F64_MAX_ULP: u64 = 1;

    /// Maximum ULP error target for f64 `exp_u35`.
    pub const EXP_U35_F64_MAX_ULP: u64 = 1;
}

#[inline(always)]
fn map_unary_f32<V: SimdFloat32>(input: V, f: impl Fn(f32) -> f32) -> V {
    unsafe {
        let mut lanes = input.as_array();
        for i in 0..V::WIDTH {
            lanes[i] = f(lanes[i]);
        }
        V::load_from_array(lanes)
    }
}

#[inline(always)]
fn map_unary_f64<V: SimdFloat64>(input: V, f: impl Fn(f64) -> f64) -> V {
    unsafe {
        let mut lanes = input.as_array();
        for i in 0..V::WIDTH {
            lanes[i] = f(lanes[i]);
        }
        V::load_from_array(lanes)
    }
}

/// SIMD math extension trait for `f32` vector types.
///
/// `log2_u35`/`exp2_u35` use SIMD-native reduction/polynomial kernels.
/// `ln_u35`/`exp_u35` currently use deterministic lane-wise scalar references.
pub trait SimdMathF32: SimdFloat32 {
    /// `log2(x)` with target `u35`-tier contract.
    ///
    /// Uses a SIMD-native mantissa/exponent reduction + polynomial kernel for
    /// positive normal inputs, with scalar fallback for exceptional lanes.
    #[inline(always)]
    fn log2_u35(self) -> Self
    where
        Self::Engine: Simd<Vf32 = Self>,
    {
        f32_kernels::log2_u35(self)
    }

    /// `exp2(x)` with target `u35`-tier contract.
    ///
    /// Uses a SIMD-native floor/reduction + polynomial kernel in the finite
    /// in-range domain, with scalar fallback for exceptional lanes.
    #[inline(always)]
    fn exp2_u35(self) -> Self
    where
        Self::Engine: Simd<Vf32 = Self>,
    {
        f32_kernels::exp2_u35(self)
    }

    /// `ln(x)` with target `u35`-tier contract.
    #[inline(always)]
    fn ln_u35(self) -> Self {
        map_unary_f32(self, scalar::ln_u35_f32)
    }

    /// `exp(x)` with target `u35`-tier contract.
    #[inline(always)]
    fn exp_u35(self) -> Self {
        map_unary_f32(self, scalar::exp_u35_f32)
    }
}

impl<T: SimdFloat32> SimdMathF32 for T {}

/// SIMD math extension trait for `f64` vector types.
pub trait SimdMathF64: SimdFloat64 {
    /// `log2(x)` with target `u35`-tier contract.
    #[inline(always)]
    fn log2_u35(self) -> Self {
        map_unary_f64(self, scalar::log2_u35_f64)
    }

    /// `exp2(x)` with target `u35`-tier contract.
    #[inline(always)]
    fn exp2_u35(self) -> Self {
        map_unary_f64(self, scalar::exp2_u35_f64)
    }

    /// `ln(x)` with target `u35`-tier contract.
    #[inline(always)]
    fn ln_u35(self) -> Self {
        map_unary_f64(self, scalar::ln_u35_f64)
    }

    /// `exp(x)` with target `u35`-tier contract.
    #[inline(always)]
    fn exp_u35(self) -> Self {
        map_unary_f64(self, scalar::exp_u35_f64)
    }
}

impl<T: SimdFloat64> SimdMathF64 for T {}
