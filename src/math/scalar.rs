use crate::libm_ext::FloatExt;

#[inline(always)]
pub fn log2_u35_f32(x: f32) -> f32 {
    x.m_log2()
}

#[inline(always)]
pub fn exp2_u35_f32(x: f32) -> f32 {
    x.m_exp2()
}

#[inline(always)]
pub fn ln_u35_f32(x: f32) -> f32 {
    x.m_ln()
}

#[inline(always)]
pub fn exp_u35_f32(x: f32) -> f32 {
    x.m_exp()
}

#[inline(always)]
pub fn sin_u35_f32(x: f32) -> f32 {
    x.m_sin()
}

#[inline(always)]
pub fn cos_u35_f32(x: f32) -> f32 {
    x.m_cos()
}

#[inline(always)]
pub fn tan_u35_f32(x: f32) -> f32 {
    x.m_tan()
}

#[inline(always)]
pub fn asin_u35_f32(x: f32) -> f32 {
    x.m_asin()
}

#[inline(always)]
pub fn acos_u35_f32(x: f32) -> f32 {
    x.m_acos()
}

#[inline(always)]
pub fn atan_u35_f32(x: f32) -> f32 {
    x.m_atan()
}

#[inline(always)]
pub fn sinh_u35_f32(x: f32) -> f32 {
    x.m_sinh()
}

#[inline(always)]
pub fn cosh_u35_f32(x: f32) -> f32 {
    x.m_cosh()
}

#[inline(always)]
pub fn tanh_u35_f32(x: f32) -> f32 {
    x.m_tanh()
}

#[inline(always)]
pub fn asinh_u35_f32(x: f32) -> f32 {
    x.m_asinh()
}

#[inline(always)]
pub fn acosh_u35_f32(x: f32) -> f32 {
    x.m_acosh()
}

#[inline(always)]
pub fn atanh_u35_f32(x: f32) -> f32 {
    x.m_atanh()
}

#[inline(always)]
pub fn log10_u35_f32(x: f32) -> f32 {
    x.m_log10()
}

#[inline(always)]
pub fn atan2_u35_f32(y: f32, x: f32) -> f32 {
    y.m_atan2(x)
}

#[inline(always)]
pub fn hypot_u35_f32(x: f32, y: f32) -> f32 {
    x.m_hypot(y)
}

#[inline(always)]
pub fn fmod_f32(x: f32, y: f32) -> f32 {
    x % y
}

#[inline(always)]
pub fn log2_u35_f64(x: f64) -> f64 {
    x.m_log2()
}

#[inline(always)]
pub fn exp2_u35_f64(x: f64) -> f64 {
    x.m_exp2()
}

#[inline(always)]
pub fn ln_u35_f64(x: f64) -> f64 {
    x.m_ln()
}

#[inline(always)]
pub fn exp_u35_f64(x: f64) -> f64 {
    x.m_exp()
}

#[inline(always)]
pub fn sin_u35_f64(x: f64) -> f64 {
    x.m_sin()
}

#[inline(always)]
pub fn cos_u35_f64(x: f64) -> f64 {
    x.m_cos()
}

#[inline(always)]
pub fn tan_u35_f64(x: f64) -> f64 {
    x.m_tan()
}

#[inline(always)]
pub fn asin_u35_f64(x: f64) -> f64 {
    x.m_asin()
}

#[inline(always)]
pub fn acos_u35_f64(x: f64) -> f64 {
    x.m_acos()
}

#[inline(always)]
pub fn atan_u35_f64(x: f64) -> f64 {
    x.m_atan()
}

#[inline(always)]
pub fn sinh_u35_f64(x: f64) -> f64 {
    x.m_sinh()
}

#[inline(always)]
pub fn cosh_u35_f64(x: f64) -> f64 {
    x.m_cosh()
}

#[inline(always)]
pub fn tanh_u35_f64(x: f64) -> f64 {
    x.m_tanh()
}

#[inline(always)]
pub fn asinh_u35_f64(x: f64) -> f64 {
    x.m_asinh()
}

#[inline(always)]
pub fn acosh_u35_f64(x: f64) -> f64 {
    x.m_acosh()
}

#[inline(always)]
pub fn atanh_u35_f64(x: f64) -> f64 {
    x.m_atanh()
}

#[inline(always)]
pub fn log10_u35_f64(x: f64) -> f64 {
    x.m_log10()
}

#[inline(always)]
pub fn atan2_u35_f64(y: f64, x: f64) -> f64 {
    y.m_atan2(x)
}

#[inline(always)]
pub fn hypot_u35_f64(x: f64, y: f64) -> f64 {
    x.m_hypot(y)
}

#[inline(always)]
pub fn fmod_f64(x: f64, y: f64) -> f64 {
    x % y
}
