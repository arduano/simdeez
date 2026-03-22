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
pub fn log2_u35_f64(x: f64) -> f64 {
    x.m_log2()
}

#[inline(always)]
pub fn exp2_u35_f64(x: f64) -> f64 {
    x.m_exp2()
}
