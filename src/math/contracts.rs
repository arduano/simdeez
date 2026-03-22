pub const LOG2_U35_F32_MAX_ULP: u32 = 35;
pub const EXP2_U35_F32_MAX_ULP: u32 = 35;
pub const LN_U35_F32_MAX_ULP: u32 = 1;
pub const EXP_U35_F32_MAX_ULP: u32 = 1;
pub const SIN_U35_F32_MAX_ULP: u32 = 35;
pub const COS_U35_F32_MAX_ULP: u32 = 35;
pub const TAN_U35_F32_MAX_ULP: u32 = 35;

// Portable inverse-trig kernels target the SLEEF-style u35 contract on f32.
pub const ASIN_U35_F32_MAX_ULP: u32 = 35;
pub const ACOS_U35_F32_MAX_ULP: u32 = 35;
pub const ATAN_U35_F32_MAX_ULP: u32 = 35;
pub const ATAN2_U35_F32_MAX_ULP: u32 = 1;
// Portable f32 hyperbolic kernels are now honest u35 implementations:
// bounded SIMD fast paths plus scalar-lane patching for exceptional inputs.
pub const SINH_U35_F32_MAX_ULP: u32 = 35;
pub const COSH_U35_F32_MAX_ULP: u32 = 35;
pub const TANH_U35_F32_MAX_ULP: u32 = 35;
// Portable f32 inverse-hyperbolic kernels target u35 accuracy.
pub const ASINH_U35_F32_MAX_ULP: u32 = 35;
pub const ACOSH_U35_F32_MAX_ULP: u32 = 35;
pub const ATANH_U35_F32_MAX_ULP: u32 = 35;
pub const HYPOT_U35_F32_MAX_ULP: u32 = 1;
// f32 log10 now rides the portable log2_u35 kernel and inherits its relaxed envelope.
pub const LOG10_U35_F32_MAX_ULP: u32 = 35;

pub const LOG2_U35_F64_MAX_ULP: u64 = 35;
pub const EXP2_U35_F64_MAX_ULP: u64 = 35;
pub const LN_U35_F64_MAX_ULP: u64 = 35;
pub const EXP_U35_F64_MAX_ULP: u64 = 35;
pub const SIN_U35_F64_MAX_ULP: u64 = 35;
pub const COS_U35_F64_MAX_ULP: u64 = 35;
pub const TAN_U35_F64_MAX_ULP: u64 = 35;

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
