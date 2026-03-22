use crate::libm_ext::FloatExt;

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
