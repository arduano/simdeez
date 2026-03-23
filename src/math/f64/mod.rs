//! f64 SIMD math dispatch layering:
//! - family-local modules own the public internal routing points for each math family.
//! - current decisions are intentionally mixed:
//!   scalar-reference for core/trig and hyperbolic defaults,
//!   portable SIMD for inverse trig and several binary-misc kernels,
//!   and hybrid paths where scalar sub-ops still underpin the fast path.
//! - follow-up optimization work can still replace one family module at a time.

mod binary_misc;
mod core;
mod hyperbolic;
mod inverse_hyperbolic;
mod inverse_trig;

pub(crate) use binary_misc::{atan2_u35, fmod, hypot_u35, log10_u35};
pub(crate) use core::{cos_u35, exp2_u35, exp_u35, ln_u35, log2_u35, sin_u35, tan_u35};
pub(crate) use hyperbolic::{cosh_u35, sinh_u35, tanh_u35};
pub(crate) use inverse_hyperbolic::{acosh_u35, asinh_u35, atanh_u35};
pub(crate) use inverse_trig::{acos_u35, asin_u35, atan_u35};
