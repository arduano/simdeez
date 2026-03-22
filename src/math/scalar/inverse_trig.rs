use crate::libm_ext::FloatExt;

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
