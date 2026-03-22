//! Portable SIMD math scaffolding for SLEEF-style transcendental families.
//!
//! Strategy C baseline: keep semantics in-tree and backend-agnostic by expressing
//! vector math over existing simdeez vector types. This first milestone restores
//! working `log2`/`exp2` families (plus composed `ln`/`exp`) with explicit contract
//! constants and deterministic per-lane scalar oracle behavior.
//!
//! Follow-up work can replace scalar-lane kernels with tighter fully vectorized
//! approximations while preserving these contracts.

mod scalar;

use crate::{SimdFloat32, SimdFloat64};

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
/// Current implementation uses deterministic lane-wise scalar kernels while
/// preserving SIMD API shape and contracts.
pub trait SimdMathF32: SimdFloat32 {
    /// `log2(x)` with target `u35`-tier contract.
    ///
    /// Special-case semantics match IEEE scalar `log2`:
    /// - `x == +0.0` or `x == -0.0` => `-inf`
    /// - `x < 0.0` => `NaN`
    /// - `x == +inf` => `+inf`
    /// - `x == NaN` => `NaN`
    #[inline(always)]
    fn log2_u35(self) -> Self {
        map_unary_f32(self, scalar::log2_u35_f32)
    }

    /// `exp2(x)` with target `u35`-tier contract.
    ///
    /// Special-case semantics match IEEE scalar `exp2`:
    /// - `x == +inf` => `+inf`
    /// - `x == -inf` => `+0.0`
    /// - `x == NaN` => `NaN`
    /// - finite overflow/underflow follow scalar backend behavior.
    #[inline(always)]
    fn exp2_u35(self) -> Self {
        map_unary_f32(self, scalar::exp2_u35_f32)
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
