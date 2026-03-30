use crate::libm_ext::FloatExt;

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
