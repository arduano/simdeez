use crate::libm_ext::FloatExt;

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
