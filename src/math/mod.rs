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
//! `sinh_u35` / `cosh_u35` / `tanh_u35` now use family-local portable SIMD
//! kernels with centralized scalar patching for exceptional lanes.
//! The stabilized `f64` map is intentionally mixed:
//! portable SIMD for the revived core log/exp family, inverse trig, and several binary-misc kernels,
//! scalar-reference for the current losing trig and hyperbolic families,
//! and hybrid keep decisions where SIMD structure still relies on scalar sub-ops.
//!
//! Structure notes:
//! - `families/` owns public extension traits grouped by math family.
//! - `scalar/` owns scalar fallback helpers using the same family boundaries.
//! - `f64/` mirrors the family split so future rescue-or-revert work can stay localized.
//! - `contracts.rs` and `map.rs` stay stable so follow-up optimization PRs can
//!   target a single family file with minimal overlap.
//!
//! Decision vocabulary used by the math audit ledger:
//! - `KEEP_SIMD_PORTABLE`: portable SIMD stays enabled by default.
//! - `KEEP_SIMD_OVERRIDE`: portable SIMD stays enabled and a backend override stays justified.
//! - `KEEP_SCALAR_REFERENCE`: the honest default remains lane-wise scalar reference.
//! - `KEEP_MIXED`: keep a hybrid path that combines vector structure with scalar sub-ops or patching.
//! - `RESEARCH_NEEDED`: current evidence is not strong enough for a cleaner keep/revert call.

pub mod contracts;
mod f32;
mod f64;
mod families;
mod map;
mod scalar;

pub use families::{
    SimdMathF32, SimdMathF32BinaryMisc, SimdMathF32Core, SimdMathF32Hyperbolic,
    SimdMathF32InverseHyperbolic, SimdMathF32InverseTrig, SimdMathF64, SimdMathF64BinaryMisc,
    SimdMathF64Core, SimdMathF64Hyperbolic, SimdMathF64InverseHyperbolic, SimdMathF64InverseTrig,
};
